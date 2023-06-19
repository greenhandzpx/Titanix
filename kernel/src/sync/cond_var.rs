use core::{task::Waker, future::Future};

use alloc::{collections::VecDeque, sync::Arc};

use crate::{process::thread::Thread, processor::current_task, utils::async_tools};

use super::mutex::SpinNoIrqLock;

type Mutex<T> = SpinNoIrqLock<T>;

/// Conditional variable
pub struct CondVar {
    /// Wait queue of threads
    pub wait_queue: Mutex<VecDeque<Waker>>,
}

impl CondVar {
    /// Create a new condvar
    pub fn new() -> Self {
        Self {
            wait_queue: Mutex::new(VecDeque::new()),
        }
    }
    /// Wait in this condvar
    pub async fn wait_without_mutex(&self) {
        self.wait_queue.lock().push_back(async_tools::take_waker().await);
    }
    /// Wait in this condvar
    pub fn wait(&self) {
        todo!()
    }
    /// Signal any one of the threads in the wait queue
    pub fn signal(&self) {
        if let Some(waker) = self.wait_queue.lock().pop_front() {
            waker.wake();
        }
    }
}

// struct CondVarFuture {
//     predicate:  
// }

// impl Future for CondVarFuture {

//     type Output = ();
//     fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        
//     }
// }