use alloc::sync::Arc;
use log::debug;

use crate::{
    process::thread::exit::handle_exit,
    processor::current_task,
    trap::{self, TrapContext},
    utils::async_tools,
};

use super::Thread;

pub async fn threadloop(thread: Arc<Thread>) {
    thread.set_waker(async_tools::take_waker().await);
    debug!(
        "into thread loop, sepc {:#x}, trap cx addr {:#x}",
        current_task().trap_context_ref().sepc,
        current_task().trap_context_ref() as *const TrapContext as usize
    );
    loop {
        trap::user_trap::trap_return();

        // next time when user traps into kernel, it will come back here
        trap::user_trap::trap_handler().await;

        if thread.is_zombie() {
            debug!("thread {} terminated", current_task().tid());
            break;
        }
    }

    // When the process becomes zombie, all of its threads should exit too
    handle_exit(&thread);
}
