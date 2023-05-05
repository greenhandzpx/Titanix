use core::arch::asm;

use crate::{
    config::{mm::PAGE_SIZE, processor::HART_NUM},
    process::{thread::Thread, Process},
    processor::hart::Hart,
    trap::TrapContext,
    utils::debug::stack_tracker::StackTracker,
};

use self::context::EnvContext;
pub mod context;
pub mod env;
pub mod hart;

use alloc::sync::Arc;
pub use env::SumGuard;

/// We store the local hart's addr in `tp` reg, instead of the hart id,

const HART_EACH: Hart = Hart::new();
pub static mut HARTS: [Hart; HART_NUM] = [HART_EACH; HART_NUM];

unsafe fn get_hart_by_id(hart_id: usize) -> &'static Hart {
    &HARTS[hart_id]
}

/// Set the cpu hart control block according to `hard_id`
pub unsafe fn set_local_hart(hart_id: usize) {
    let hart = get_hart_by_id(hart_id);
    let hart_addr = hart as *const _ as usize;
    asm!("mv tp, {}", in(reg) hart_addr);
}

pub fn set_hart_stack() {
    let h = local_hart();
    let sp: usize;
    unsafe {
        asm!("mv {}, sp", out(reg) sp);
    }
    h.set_stack((sp & !(PAGE_SIZE - 1)) + PAGE_SIZE);
}

/// Get the current local hart
pub fn local_hart() -> &'static mut hart::Hart {
    unsafe {
        let tp: usize;
        asm!("mv {}, tp", out(reg) tp);
        &mut *(tp as *mut Hart)
    }
}

pub fn local_env() -> &'static mut EnvContext {
    local_hart().env()
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

pub fn init() {
    unsafe {
        for hart in HARTS.iter_mut() {
            hart.env().stack_tracker = Some(StackTracker::new());
        }
    }
}
