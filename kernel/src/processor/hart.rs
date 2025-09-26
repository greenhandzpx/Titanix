use core::arch::asm;

use alloc::{boxed::Box, sync::Arc};
use riscv::register::sstatus::{self, FS};

use crate::{
    config::{mm::PAGE_SIZE, processor::HART_NUM},
    mm::{PageTable, KERNEL_SPACE},
    process::thread::Thread,
    processor::{env, local_irq_disable, local_irq_enable},
    stack_trace,
    utils::cell::SyncUnsafeCell,
};

use super::{
    ctx::{EnvContext, LocalContext},
    current_trap_cx,
};

/// Local context in one hart, either Idle or Something(about one thread)

/// The processor has several `Hart`s
pub struct Hart {
    hart_id: usize,
    local_ctx: Option<Box<LocalContext>>,
    /// Every hart has its own kernel stack
    kstack_bottom: usize,
}

impl Hart {
    pub fn env(&self) -> &EnvContext {
        self.local_ctx.as_ref().unwrap().env()
    }

    pub fn env_mut(&mut self) -> &mut EnvContext {
        self.local_ctx.as_mut().unwrap().env_mut()
    }

    #[allow(unused)]
    pub fn local_ctx(&self) -> &LocalContext {
        &self.local_ctx.as_ref().unwrap()
    }

    pub fn local_ctx_mut(&mut self) -> &mut LocalContext {
        self.local_ctx.as_mut().unwrap()
    }

    pub fn current_task(&self) -> &Arc<Thread> {
        // TODO: add debug assert to ensure now the hart must have a task
        // assert_ne!(self.local_ctx.task_ctx())
        stack_trace!();
        &self.local_ctx.as_ref().unwrap().task_ctx().thread
    }

    pub fn ktid(&self) -> usize {
        self.local_ctx.as_ref().unwrap().ktid()
    }

    pub fn is_idle(&self) -> bool {
        self.local_ctx.is_none() || self.local_ctx.as_ref().unwrap().is_idle()
        // self.local_ctx.is_none() |
    }

    pub fn is_absolutely_idle(&self) -> bool {
        self.local_ctx.is_none()
        // self.local_ctx.is_none() |
    }

    pub fn is_kthread(&self) -> bool {
        self.local_ctx.as_ref().unwrap().is_idle()
    }

    pub fn change_page_table(&mut self, page_table: Arc<SyncUnsafeCell<PageTable>>) {
        stack_trace!();
        let task_ctx = self.local_ctx.as_mut().unwrap().task_ctx_mut();
        task_ctx.page_table = page_table;
    }
}

impl Hart {
    pub const fn new() -> Self {
        // TODO: modify kstack_bottom init val
        Hart {
            hart_id: 0,
            // spare_env_ctx: EnvContext::new(),
            local_ctx: None,
            kstack_bottom: 0,
        }
    }

    pub fn init_local_ctx(&mut self) {
        self.local_ctx = Some(Box::new(LocalContext::new(None, None)));
    }

    pub fn set_hart_id(&mut self, hart_id: usize) {
        self.hart_id = hart_id;
    }

    pub fn hart_id(&self) -> usize {
        self.hart_id
    }

    pub fn set_stack(&mut self, kstack: usize) {
        self.kstack_bottom = kstack;
    }

    /// Change thread(task) context,
    /// Now only change page table temporarily
    pub fn push_task(&mut self, task: &mut Box<LocalContext>) {
        local_irq_disable();

        let new_env = task.env();
        let old_env = self.env();

        let enable_irq = EnvContext::env_change(new_env, old_env);
        if self.is_idle()
            || task.task_ctx().thread.process.pid() != self.current_task().process.pid()
        {
            // stack_trace!();
            assert!(!task.is_idle());
            // Only flush tlb when switching process
            unsafe {
                (*task.task_ctx().page_table.get()).activate();
            }
        }
        if !task.is_idle() {
            unsafe {
                (*task.task_ctx().thread.inner.get())
                    .time_info
                    .when_entering()
            }
        }
        core::mem::swap(self.local_ctx_mut(), task);

        if enable_irq {
            local_irq_enable();
        }
    }

    pub fn pop_task(&mut self, task: &mut Box<LocalContext>) {
        local_irq_disable();

        let new_env = task.env();
        let old_env = self.env();
        let enable_irq = EnvContext::env_change(new_env, old_env);

        // Save float regs
        current_trap_cx().user_fx.yield_task();

        if task.has_user_ctx() {
            unsafe {
                (*task.task_ctx().page_table.get()).activate();
            }
        } else {
            unsafe {
                KERNEL_SPACE
                    .as_ref()
                    .expect("KERNEL SPACE not init yet")
                    .activate();
            }
        }
        // task.task_ctx().page_table.activate();
        core::mem::swap(self.local_ctx_mut(), task);

        if task.env().irq_is_disabled() {
            log::warn!("IRQ disabled while scheduling!");
        }
        if !task.is_idle() {
            unsafe {
                (*task.task_ctx().thread.inner.get())
                    .time_info
                    .when_leaving()
            }
        }
        if enable_irq {
            local_irq_enable();
        }
    }

    pub fn push_kernel_task(&mut self, task: &mut Box<LocalContext>) {
        local_irq_disable();

        let new_env = task.env();
        let old_env = self.env();
        let enable_irq = EnvContext::env_change(new_env, old_env);
        core::mem::swap(self.local_ctx_mut(), task);

        if enable_irq {
            local_irq_enable();
        }
    }

    pub fn pop_kernel_task(&mut self, task: &mut Box<LocalContext>) {
        local_irq_disable();

        let new_env = task.env();
        let old_env = self.env();
        let enable_irq = EnvContext::env_change(new_env, old_env);
        core::mem::swap(self.local_ctx_mut(), task);

        if task.env().irq_is_disabled() {
            log::warn!("IRQ disabled while scheduling!");
        }
        if enable_irq {
            local_irq_enable();
        }
    }

    /// Provide a new temporary hart ctx for preempted task,
    /// which will be replaced when fetching a task from task queue.
    pub fn enter_preempt_switch(&mut self) -> Self {
        self.env_mut().preempt_record();

        let mut new = Self::new();
        new.hart_id = self.hart_id;
        let env_ctx = EnvContext::new();
        let local_ctx = Box::new(LocalContext::new(None, Some(env_ctx)));
        new.local_ctx = Some(local_ctx);
        core::mem::swap(&mut new, self);

        new
    }

    pub fn leave_preempt_switch(&mut self, old_hart: &mut Hart) {
        core::mem::swap(self, old_hart);

        unsafe { self.env_mut().preempt_resume() };
    }
}

const HART_EACH: Hart = Hart::new();
pub static mut HARTS: [Hart; HART_NUM] = [HART_EACH; HART_NUM];

unsafe fn get_hart_by_id(hart_id: usize) -> &'static mut Hart {
    &mut HARTS[hart_id]
}

/// Set the cpu hart control block according to `hard_id`
pub unsafe fn set_local_hart(hart_id: usize) {
    let hart = get_hart_by_id(hart_id);
    hart.set_hart_id(hart_id);
    let hart_addr = hart as *const _ as usize;
    asm!("mv tp, {}", in(reg) hart_addr);
}

pub fn set_hart_stack() {
    let h = local_hart();
    let sp: usize;
    unsafe {
        asm!("mv {}, sp", out(reg) sp);
    }
    println!("[kernel][hart{}] set_hart_stack: sp {:#x}", h.hart_id, sp);
    h.set_stack((sp & !(PAGE_SIZE - 1)) + PAGE_SIZE);
}

static mut HARTS_DISABLE_PREEMPT: [usize; HART_NUM] = [0; HART_NUM];

pub fn local_hart_preemptible() -> bool {
    unsafe { HARTS_DISABLE_PREEMPT[local_hart().hart_id()] == 0 }
}

/// Don't use this
pub fn set_local_hart_preemptible(preemptible: bool) {
    unsafe {
        if preemptible {
            if HARTS_DISABLE_PREEMPT[local_hart().hart_id()] > 0 {
                HARTS_DISABLE_PREEMPT[local_hart().hart_id()] -= 1;
            }
        } else {
            HARTS_DISABLE_PREEMPT[local_hart().hart_id()] += 1;
        }
    }
}

/// Get the current local hart
pub fn local_hart() -> &'static mut Hart {
    unsafe {
        let tp: usize;
        asm!("mv {}, tp", out(reg) tp);
        &mut *(tp as *mut Hart)
    }
}

pub fn init(hart_id: usize) {
    println!("start to init hart {}...", hart_id);
    unsafe {
        set_local_hart(hart_id);
    }
    set_hart_stack();
    unsafe {
        // sstatus::set_fs(FS::Clean);
        sstatus::set_fs(FS::Initial);
    }
    println!("init hart {} finished", hart_id);
}
