use alloc::{boxed::Box, sync::Arc};
use riscv::register::sstatus;

use crate::{mm::PageTable, process::thread::Thread, utils::{debug::stack_tracker::StackTracker, cell::SyncUnsafeCell}};

pub enum LocalContext {
    /// There is no user task now(i.e. kernel thread is running)
    Idle,
    /// User task is running
    TaskContext(Box<TaskContext>),
}

impl LocalContext {
    pub fn task_ctx_mut(&mut self) -> &mut TaskContext {
        match self {
            LocalContext::TaskContext(task_ctx) => task_ctx,
            LocalContext::Idle => panic!("Idle LocalContext"),
        }
    }

    pub fn task_ctx(&self) -> &TaskContext {
        match self {
            LocalContext::TaskContext(task_ctx) => task_ctx,
            LocalContext::Idle => panic!("Idle LocalContext"),
        }
    }

    pub fn env(&mut self, spare_env: *mut EnvContext) -> &mut EnvContext {
        match self {
            // SAFETY:
            // spare_env is the local hart's member, which lives forever
            Self::Idle => unsafe { &mut *spare_env },
            Self::TaskContext(task) => &mut task.env,
        }
    }

    /// Whether there is no user task now(i.e. kernel thread is running)
    pub fn is_idle(&self) -> bool {
        match self {
            LocalContext::Idle => true,
            _ => false,
        }
    }
}

pub struct TaskContext {
    pub thread: Arc<Thread>,
    pub page_table: Arc<SyncUnsafeCell<PageTable>>,
    pub env: EnvContext,
}

/// Store some permission flags
pub struct EnvContext {
    sie: usize,
    sum: usize,
    // TODO: add more members
    pub stack_tracker: Option<StackTracker>,
}

impl EnvContext {
    pub const fn new() -> Self {
        Self {
            sie: 0,
            sum: 0,
            stack_tracker: None,
        }
    }

    pub fn sum_inc(&mut self) {
        if self.sum == 0 {
            unsafe {
                sstatus::set_sum();
            }
        }
        self.sum += 1
    }

    pub fn sum_dec(&mut self) {
        if self.sum == 1 {
            unsafe {
                sstatus::clear_sum();
            }
        }
        self.sum -= 1
    }

    pub fn env_change(new: &mut Self, old: &mut Self) {
        unsafe {
            if (new.sum > 0) != (old.sum > 0) {
                if new.sum > 0 {
                    sstatus::set_sum();
                } else {
                    sstatus::clear_sum();
                }
            }
            // if (new.sie > 0) != (old.sie > 0) {
            //     if new.sie > 0 {
            //         sstatus::set_sie();
            //     } else {
            //         sstatus::clear_sie();
            //     }
            // }
            // TODO: what if just clear sie in the period of trap
            sstatus::clear_sie();
        }
    }
}
pub struct KernelTaskContext {}
