//! Trap handling functionality
mod context;
/// Kernel trap handler
pub mod kernel_trap;
/// User trap handler
pub mod user_trap;

use crate::processor::{current_process, current_trap_cx};
use core::arch::global_asm;
use log::{debug, error, warn};
use riscv::register::{mtvec::TrapMode, scause, sie, stval, stvec};

global_asm!(include_str!("trap.S"));

extern "C" {
    fn __trap_from_user();
    fn __trap_from_kernel();
}

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(__trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        // stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
        stvec::write(__trap_from_user as usize, TrapMode::Direct);
    }
}
/// enable timer interrupt in sie CSR
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
/// Unimplement: traps/interrupts/exceptions from kernel mode
pub fn trap_from_kernel() {
    // #[cfg(feature = "kernel_timer_interrupt")]
    let scause = scause::read();
    match scause.cause() {
        // Trap::Interrupt(Interrupt::SupervisorTimer) => {
        //     set_next_trigger();
        //     handle_timeout_events();
        //     process::yield_now().await;
        // }
        _ => {
            error!(
                "[kernel] {:?}(scause:{}) in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it. pid: {}",
                scause::read().cause(),
                scause::read().bits(),
                stval::read(),
                current_trap_cx().sepc,
                current_process().pid()
            );
            panic!(
                "a trap {:?} from kernel! stval {:#x}",
                scause::read().cause(),
                stval::read()
            );
        }
    }
}

pub use context::TrapContext;
pub use context::UserContext;
