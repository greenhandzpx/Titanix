use crate::{process::INITPROC, processor::current_process, stack_trace};
use alloc::sync::Arc;
use log::{debug, info};

use super::Thread;

/// Things the thread need to do when it terminated
pub fn handle_exit(thread: &Arc<Thread>) {
    stack_trace!();
    debug!("thread {} handle exit", thread.tid());
    if thread.process.pid() == 0 {
        panic!("initproc die!!!");
    }
    // Thread resource(i.e. tid, ustack) will be
    // released when the thread is destructed automatically

    // We should visit the process inner exclusively
    // since different harts may arrive here at the
    // same time
    let mut process_inner = thread.process.inner.lock();


    let mut idx: Option<usize> = None;
    for (i, t) in process_inner.threads.iter().enumerate() {
        // // # SAFETY:
        // // it is impossibe for the case like one thread is dead
        // // but hasn't been removed from its process's `threads` member
        // // since one thread must be removed from `threads` first and then
        // // die
        // if unsafe { (*t.as_ptr()).tid.0 == thread.tid.0 } {
        //     idx = Some(i);
        //     break;
        // }
        // TODO: not sure whether it is safe to unwrap here
        if t.upgrade().unwrap().tid.0 == thread.tid.0 {
            idx = Some(i);
            break;
        }
    }
    if let Some(idx) = idx {
        process_inner.threads.remove(idx);
    } else {
        panic!("Cannot find the thread in its process")
    }

    if process_inner.thread_count() > 0 {
        // this thread isn't the final thread
        debug!(
            "thread {} terminated, process thread cnt {}",
            thread.tid(),
            process_inner.thread_count()
        );
        return;
    }

    // Final exited thread should:
    // 1. mark the process as zombie
    // 2. handle the process's children migration
    debug!(
        "final thread {} terminated, process become zombie",
        thread.tid()
    );
    process_inner.is_zombie = true;
    for child in process_inner.children.iter() {
        unsafe {
            child.inner.lock().parent = Some(Arc::downgrade(INITPROC.as_ref().unwrap()));
            INITPROC
                .as_ref()
                .unwrap()
                .inner
                .lock()
                .children
                .push(child.clone());
        }
    }
    // TODO: Maybe we don't need to clear here?()
    process_inner.children.clear();
    // todo!("Handle thread exit")
}

/// Exit and terminate all threads of the current process.
/// Note that the caller cannot hold the process inner's lock
pub fn exit_and_terminate_all_threads(exit_code: i8) {
    current_process().inner_handler(|proc| {
        proc.exit_code = exit_code;
        proc.is_zombie = true;
        for thread in proc.threads.iter_mut() {
            unsafe { (*thread.as_ptr()).terminate() }
        }
        // proc.threads.clear();
    });
}

/// Terminate all threads of the current process except main thread(i.e. idx = 0).
/// Note that the caller cannot hold the process inner's lock
pub fn terminate_all_threads_except_main() {
    current_process().inner_handler(|proc| {
        for (i, thread) in proc.threads.iter_mut().enumerate() {
            if i == 0 {
                continue;
            }
            unsafe { (*thread.as_ptr()).terminate() }
        }
        // let len = proc.threads.len();
        // for _ in 0..len - 1 {
        //     proc.threads.pop();
        // }
    });
}

/// Terminate the given thread
/// Note that the caller cannot hold the process inner's lock
pub fn terminate_given_thread(tid: usize, exit_code: i8) {
    current_process().inner_handler(|proc| {
        proc.exit_code = exit_code;
        // let mut idx: Option<usize> = None;
        for (_, thread) in proc.threads.iter_mut().enumerate() {
            let t = unsafe { &mut *(thread.as_ptr() as *mut Thread) };
            if t.tid() == tid {
                // info!("terminate given tid {}", tid);
                t.terminate();
                // idx = Some(i);
                break;
            }
        }
        // if let Some(idx) = idx {
        //     proc.threads.remove(idx);
        // }
    });
}
