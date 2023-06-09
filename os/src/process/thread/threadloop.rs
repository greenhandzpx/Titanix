/*
 * @Author: greenhandzpx 893522573@qq.com
 * @Date: 2023-01-28 14:42:18
 * @LastEditors: greenhandzpx 893522573@qq.com
 * @LastEditTime: 2023-02-24 22:49:47
 * @FilePath: /oscomp-kernel/os/src/process/thread/threadloop.rs
 * @Description:
 *
 * Copyright (c) 2023 by greenhandzpx 893522573@qq.com, All Rights Reserved.
 */

use alloc::sync::Arc;
use log::{debug, info};

use crate::{
    process::thread::exit::handle_exit,
    processor::{current_process, current_task},
    stack_trace,
    trap::{self, TrapContext},
};

use super::Thread;

pub async fn threadloop(thread: Arc<Thread>) {
    debug!(
        "into thread loop, sepc {:#x}, trap cx addr {:#x}",
        current_task().trap_context_ref().sepc,
        current_task().trap_context_ref() as *const TrapContext as usize
    );
    loop {
        // println!("pid {} back to threadloop", current_process().get_pid());
        // return to user mode
        let trap_context = unsafe {
            // TODO: figure out why we can't use in this way
            // &mut (*thread.inner.get()).trap_context
            let p = &mut *thread.inner.get();
            &mut p.trap_context
        };
        // let trap_context = thread.task.inner_exclusive_access().get_trap_cx();
        // // TODO: not sure whther `exclusive access` leads to deadlock
        trap::trap_return(trap_context);
        // println!("trap from user");
        // next time when user traps into kernel, it will come back here
        trap::trap_handler().await;

        stack_trace!();

        if thread.is_zombie() {
            debug!("thread {} terminated", current_task().tid());
            break;
        }
        // if unsafe {
        //     // TODO: not sure what memory order should be used
        //     (*thread.inner.get()).terminated.load(Ordering::Relaxed)
        // } {
        //     debug!("thread {} terminated", current_task().tid());
        //     break;
        // }

        // // TODO: find a more elegant way to know whether the process
        // // has become zombie. The following way will lock the process
        // // inner and it may become a bottleneck
        // if  current_process().is_zombie() {
        //     break;
        // }
    }

    // When the process becomes zombie, all of its threads should exit too
    handle_exit(&thread);
}
