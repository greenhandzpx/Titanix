use crate::{
    process::{thread::Thread, Process},
    trap::TrapContext,
};

pub use self::hart::{local_hart, HARTS};

use self::ctx::EnvContext;
pub mod ctx;
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
    local_hart().is_absolutely_idle()
}

pub fn hart_is_kthread_now() -> bool {
    local_hart().is_kthread()
}

// pub fn init() {
//     unsafe {
//         for hart in HARTS.iter_mut() {
//             hart.init();
//         }
//     }
// }

pub fn local_irq_disable() {
    #[cfg(feature = "kernel_interrupt")]
    unsafe {
        riscv::register::sstatus::clear_sie()
    }
}

pub fn local_irq_enable() {
    #[cfg(feature = "kernel_interrupt")]
    unsafe {
        riscv::register::sstatus::set_sie();
    }
}

pub fn local_irq_is_enabled() -> bool {
    #[cfg(feature = "kernel_interrupt")]
    return riscv::register::sstatus::read().sie();

    #[cfg(not(feature = "kernel_interrupt"))]
    false
}

// pub fn close_interrupt() {
//     #[cfg(feature = "kernel_interrupt")]
//     unsafe {
//         riscv::register::sstatus::clear_sie()
//     }
// }
// pub fn open_interrupt() {
//     // info!("open interrupt");
//     #[cfg(feature = "kernel_interrupt")]
//     unsafe {
//         riscv::register::sstatus::set_sie();
//     }
// }
