use crate::{
    mm::user_check::UserCheck, process::INITPROC, processor::current_process, signal::Signal,
    stack_trace,
};
use alloc::{sync::Arc, vec::Vec};
use log::{debug, error, info, warn};

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

    // The reason why we clear tid addr here is that in the destruction function of TidAddr, we will lock the process inner.
    let inner = unsafe { &mut (*thread.inner.get()) };
    debug!("clear tid address");
    inner.tid_addr.take();
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
    // 3. send signal to parent process
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
    let parent_prcess = {
        if let Some(parent_process) = process_inner.parent.as_ref() {
            parent_process.upgrade().unwrap()
        } else {
            panic!("initproc will die");
        }
    };
    // In order to avoid dead lock
    drop(process_inner);
    debug!("Send SIGCHILD to parent {}", parent_prcess.pid());
    parent_prcess.inner_handler(|proc| proc.pending_sigs.send_signal(Signal::SIGCHLD))
    // todo!("Handle thread exit")
}

/// Exit and terminate all threads of the current process.
/// Note that the caller cannot hold the process inner's lock
pub fn exit_and_terminate_all_threads(exit_code: i8) {
    debug!("exit and terminate all threads, exit code {}", exit_code);
    let threads = current_process().inner_handler(|proc| {
        let mut threads: Vec<Arc<Thread>> = Vec::new();
        proc.exit_code = exit_code;
        proc.is_zombie = true;
        // current_process().set_zombie();
        for thread in proc.threads.iter_mut() {
            threads.push(thread.upgrade().unwrap());
            // unsafe { (*thread.as_ptr()).terminate() }
        }
        threads
        // proc.threads.clear();
    });
    for thread in threads {
        thread.terminate();
    }
}

/// Terminate all threads of the current process except main thread(i.e. idx = 0).
/// Note that the caller cannot hold the process inner's lock
pub fn terminate_all_threads_except_main() {
    let threads = current_process().inner_handler(|proc| {
        let mut threads: Vec<Arc<Thread>> = Vec::new();
        for (i, thread) in proc.threads.iter_mut().enumerate() {
            if i == 0 {
                continue;
            }
            threads.push(thread.upgrade().unwrap());
            // unsafe { (*thread.as_ptr()).terminate() }
        }
        threads
    });
    for thread in threads {
        thread.terminate();
    }
}

/// Terminate the given thread
/// Note that the caller cannot hold the process inner's lock
pub fn terminate_given_thread(tid: usize, exit_code: i8) {
    if let Some(thread) = current_process().inner_handler(|proc| {
        proc.exit_code = exit_code;
        // let mut idx: Option<usize> = None;
        for (_, thread) in proc.threads.iter_mut().enumerate() {
            // let t = unsafe { &mut *(thread.as_ptr() as *mut Thread) };
            if let Some(t) = thread.upgrade() {
                if t.tid() == tid {
                    return Some(t);
                    // // idx = Some(i);
                    // break;
                }
            } else {
                panic!(
                    "Weak thread pointer is invalid in process {}",
                    current_process().pid()
                );
                // return None;
            }
        }
        return None;
        // if let Some(idx) = idx {
        //     proc.threads.remove(idx);
        // }
    }) {
        debug!("terminate given tid {}", tid);
        thread.terminate();
    }
}