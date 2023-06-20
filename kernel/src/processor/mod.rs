use crate::{
    process::{thread::Thread, Process},
    trap::TrapContext,
};

pub use self::hart::{local_hart, HARTS};

use self::context::EnvContext;
pub mod context;
pub mod env;
pub mod hart;

use alloc::sync::Arc;
pub use env::SumGuard;

/// We store the local hart's addr in `tp` reg, instead of the hart id,

pub fn local_env() -> &'static mut EnvContext {
    local_hart().env_mut()
}

// TODO: figure out whether to use ref or not
pub fn current_task() -> &'static Arc<Thread> {
    // TODO: add assert to ensure the hart has a task now
    &local_hart().current_task()
}

pub fn current_process() -> &'static Arc<Process> {
    &current_task().process
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    local_hart().current_task().trap_context_mut()
}

pub fn hart_idle_now() -> bool {
    local_hart().is_idle()
}

// pub fn init() {
//     unsafe {
//         for hart in HARTS.iter_mut() {
//             hart.init();
//         }
//     }
// }
