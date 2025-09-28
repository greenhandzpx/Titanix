use alloc::sync::Arc;
use log::debug;

use crate::{
    process::thread::exit::handle_exit,
    processor::{current_task, local_irq_disable, local_irq_enable},
    stack_trace,
    trap::{self, TrapContext},
    utils::async_utils,
};

use super::Thread;

pub async fn threadloop(thread: Arc<Thread>) {
    thread.set_waker(async_utils::take_waker().await);
    debug!(
        "into thread loop, sepc {:#x}, trap cx addr {:#x}",
        current_task().trap_context_ref().sepc,
        current_task().trap_context_ref() as *const TrapContext as usize
    );
    loop {
        trap::user_trap::trap_return();

        // next time when user traps into kernel, it will come back here
        trap::user_trap::trap_handler().await;

        // let sstatus = riscv::register::sstatus::read();
        // log::info!("[threadloop] sie {}", sstatus.sie());
        if thread.is_zombie() {
            debug!("thread {} terminated", current_task().tid());
            break;
        }
    }

    // When the process becomes zombie, all of its threads should exit too
    handle_exit(&thread);
}
