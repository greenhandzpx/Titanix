use alloc::{collections::VecDeque, sync::Arc};

use crate::{process::thread::Thread, processor::current_task};

use super::mutex::SpinNoIrqLock;

type Mutex<T> = SpinNoIrqLock<T>;

/// Conditional variable
pub struct CondVar {
    /// Wait queue of threads
    pub wait_queue: Mutex<VecDeque<Arc<Thread>>>,
}

impl CondVar {
    /// Create a new condvar
    pub fn new() -> Self {
        Self {
            wait_queue: Mutex::new(VecDeque::new()),
        }
    }
    /// Wait in this condvar
    pub fn wait_without_mutex(&self) {
        self.wait_queue.lock().push_back(current_task().clone());
    }
    /// Wait in this condvar
    pub fn wait(&self) {
        todo!()
    }
    /// Signal any one of the threads in the wait queue
    pub fn signal(&self) {
        if let Some(thread) = self.wait_queue.lock().pop_front() {
            thread.wake_up();
        }
    }
}
