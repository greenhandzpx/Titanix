use alloc::sync::Arc;
use log::warn;
use riscv::register::{sie, sstatus};

use crate::{
    mm::PageTable,
    process::thread::Thread,
    utils::{cell::SyncUnsafeCell, debug::stack_tracker::StackTracker},
};

pub struct LocalContext {
    /// If no user task now(i.e. kernel thread is running), then None
    user_task_ctx: Option<UserTaskContext>,
    env: EnvContext,
}

impl LocalContext {
    pub fn new(user_task_ctx: Option<UserTaskContext>, env: Option<EnvContext>) -> Self {
        let env = match env {
            Some(env) => env,
            None => EnvContext::new(),
        };
        Self { user_task_ctx, env }
    }

    pub fn task_ctx_mut(&mut self) -> &mut UserTaskContext {
        match self.user_task_ctx.as_mut() {
            Some(user_ctx) => user_ctx,
            None => panic!("Idle LocalContext"),
        }
    }

    pub fn task_ctx(&self) -> &UserTaskContext {
        match self.user_task_ctx.as_ref() {
            Some(user_ctx) => user_ctx,
            None => panic!("Idle LocalContext"),
        }
    }

    pub fn env_mut(&mut self) -> &mut EnvContext {
        &mut self.env
    }

    pub fn env(&self) -> &EnvContext {
        &self.env
    }

    /// Whether there is no user task now(i.e. kernel thread is running)
    pub fn is_idle(&self) -> bool {
        self.user_task_ctx.is_none()
    }
}

pub struct UserTaskContext {
    pub thread: Arc<Thread>,
    /// Although we can get pagetable from the thread's process's memory space,
    /// it needs lock, which reduces performance.
    pub page_table: Arc<SyncUnsafeCell<PageTable>>,
}

/// Store some permission flags
pub struct EnvContext {
    /// Supervisor interrupt enable
    sie: usize,
    /// Permit supervisor user memory access
    sum: usize,
    /// Stack tracker
    pub stack_tracker: StackTracker,
}

impl EnvContext {
    pub fn new() -> Self {
        let sie = usize::MAX;
        // if sie > 0 {
        //     EnvContext::enable_sie();
        // }
        Self {
            sie,
            sum: 0,
            stack_tracker: StackTracker::new(),
        }
    }

    pub fn sie_dec(&mut self) {
        if self.sie == usize::MAX {
            unsafe {
                sstatus::clear_sum();
            }
        }
        self.sie -= 1;
    }

    pub fn sie_inc(&mut self) {
        if self.sum == 0 {
            unsafe {
                sstatus::set_sum();
            }
        }
        self.sie += 1;
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


    pub fn sie(&self) -> bool {
        self.sie > 0
    }

    pub fn env_change(new: &Self, old: &Self) -> bool {
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
            //         EnvContext::enable_sie();
            //     } else {
            //         sstatus::clear_sie();
            //     }
            // }
        }
        return new.sie > 0;
    }

}
