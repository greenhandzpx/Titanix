use core::cell::SyncUnsafeCell;

use alloc::sync::Arc;

use crate::{
    mm::{PageTable, KERNEL_SPACE},
    process::thread::Thread, stack_trace,
};

use super::context::{EnvContext, KernelTaskContext, LocalContext};

/// Local context in one hart, either Idle or Something(about one thread)

/// The processor has several `Hart`s
pub struct Hart {
    hart_id: usize,
    /// Spare env ctx when in need(e.g. kernel thread or idle thread)
    spare_env_ctx: EnvContext,
    local_ctx: LocalContext,
    /// Every hart has its own kernel stack
    kstack_bottom: usize,
}

impl Hart {
    pub fn env(&mut self) -> &mut EnvContext {
        self.local_ctx.env(&mut self.spare_env_ctx)
    }

    pub fn local_ctx(&mut self) -> &mut LocalContext {
        &mut self.local_ctx
    }

    pub fn current_task(&self) -> &Arc<Thread> {
        // TODO: add debug assert to ensure now the hart must have a task
        // assert_ne!(self.local_ctx.task_ctx())
        stack_trace!();
        &self.local_ctx.task_ctx().thread
    }

    pub fn is_idle(&self) -> bool {
        match self.local_ctx {
            LocalContext::Idle => true,
            _ => false,
        }
    }
    pub fn change_page_table(&mut self, page_table: Arc<SyncUnsafeCell<PageTable>>) {
        stack_trace!();
        let task_ctx = self.local_ctx.task_ctx_mut();
        task_ctx.page_table = page_table;
    }
}

impl Hart {
    pub const fn new() -> Self {
        // TODO: modify kstack_bottom init val
        Hart {
            hart_id: 0,
            spare_env_ctx: EnvContext::new(),
            local_ctx: LocalContext::Idle,
            kstack_bottom: 0,
        }
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
    pub fn push_task(&mut self, task: &mut LocalContext) {
        // println!("push user task");
        // let dummy = self.local_ctx.as_mut();
        stack_trace!();
        let new_env = task.env(&mut self.spare_env_ctx);
        let old_env = self.local_ctx.env(&mut self.spare_env_ctx);
        EnvContext::env_change(new_env, old_env);
        if self.local_ctx.is_idle()
            || task.task_ctx().thread.process.pid()
                != self.local_ctx.task_ctx().thread.process.pid()
        {
            // Only flush tlb when switching process
            unsafe {
                (*task.task_ctx().page_table.get()).activate();
            }
        }
        core::mem::swap(&mut self.local_ctx, task);
    }

    pub fn pop_task(&mut self, task: &mut LocalContext) {
        let new_env = task.env(&mut self.spare_env_ctx);
        let old_env = self.local_ctx.env(&mut self.spare_env_ctx);
        EnvContext::env_change(new_env, old_env);
        unsafe {
            KERNEL_SPACE
                .as_ref()
                .expect("KERNEL SPACE not init yet")
                .activate();
        }
        // task.task_ctx().page_table.activate();
        core::mem::swap(&mut self.local_ctx, task);
    }

    pub fn push_kernel_task(&mut self, task: &mut KernelTaskContext) {
        // unsafe {
        //     (*task.task_ctx().page_table.get()).activate();
        // }
        // todo!()
        // TODO: save sie state?
        // core::mem::swap(&mut self.local_ctx, task);
    }

    pub fn pop_kernel_task(&mut self, task: &mut KernelTaskContext) {
        // let dummy = self.local_ctx.as_mut();
        // unsafe {
        //     (*task.task_ctx().page_table.get()).activate();
        // }
        // todo!()
        // TODO: recover sie state?
        // core::mem::swap(&mut self.local_ctx, task);
    }
}
