use crate::{
    config::process::INITPROC_PID,
    process::PROCESS_MANAGER,
    processor::{current_process, current_trap_cx},
    signal::SIGCHLD,
    stack_trace,
};
use alloc::{sync::Arc, vec::Vec};
use log::{debug, info};

use super::Thread;

/// Things the thread need to do when it terminated
pub fn handle_exit(thread: &Arc<Thread>) {
    stack_trace!();
    info!("thread {} handle exit", thread.tid());
    if thread.process.pid() == INITPROC_PID {
        panic!("initproc die!!!, sepc {:#x}", current_trap_cx().sepc);
    }

    PROCESS_MANAGER.remove(thread.tid());
    // Thread resource(i.e. tid, ustack) will be
    // released when the thread is destructed automatically

    // The reason why we clear tid addr here is that in the destruction function of TidAddr, we will lock the process inner.
    let inner = unsafe { &mut (*thread.inner.get()) };
    debug!("clear tid address");
    inner.tid_addr.thread_died();

    // We should visit the process inner exclusively
    // since different harts may arrive here at the
    // same time
    let mut process_inner = thread.process.inner.lock();

    process_inner.threads.remove(&thread.tid());

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
    info!(
        "final thread {} terminated, process become zombie",
        thread.tid()
    );
    process_inner.is_zombie = true;

    let init_proc = PROCESS_MANAGER.init_proc();
    for child in process_inner.children.iter() {
        child.inner.lock().parent = Some(Arc::downgrade(&init_proc));
        init_proc.inner.lock().children.push(child.clone());
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

    stack_trace!();
    debug!("Send SIGCHILD to parent {}", parent_prcess.pid());
    // parent_prcess.mailbox.send_event(Event::CHILD_EXIT);
    parent_prcess.recv_signal(SIGCHLD).unwrap();
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
        for (_, thread) in proc.threads.iter_mut() {
            threads.push(thread.upgrade().unwrap());
        }
        threads
        // proc.threads.clear();
    });
    for thread in threads {
        // thread.recv_signal(SIGKILL);
        thread.terminate();
    }
}

/// Terminate all threads of the current process except main thread(i.e. idx = 0).
/// Note that the caller cannot hold the process inner's lock
pub fn terminate_all_threads_except_main() {
    let threads = current_process().inner_handler(|proc| {
        let mut threads: Vec<Arc<Thread>> = Vec::new();
        for (i, (_, thread)) in proc.threads.iter_mut().enumerate() {
            if i == 0 {
                continue;
            }
            threads.push(thread.upgrade().unwrap());
        }
        threads
    });
    for thread in threads {
        // thread.recv_signal(SIGKILL);
        thread.terminate();
    }
}

/// Terminate the given thread
/// Note that the caller cannot hold the process inner's lock
pub fn terminate_given_thread(tid: usize, exit_code: i8) {
    if let Some(thread) = current_process().inner_handler(|proc| {
        proc.exit_code = exit_code;
        // let mut idx: Option<usize> = None;
        for (_, (_, thread)) in proc.threads.iter_mut().enumerate() {
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
    }) {
        debug!("terminate given tid {}", tid);
        // thread.recv_signal(SIGKILL);
        thread.terminate();
    }
}
