use crate::sync::mutex::SpinNoIrqLock;
use alloc::collections::VecDeque;
use async_task::{Runnable, Task};
use core::future::Future;
use lazy_static::*;
use log::{debug, info};

struct TaskQueue {
    queue: SpinNoIrqLock<VecDeque<Runnable>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            queue: SpinNoIrqLock::new(VecDeque::new()),
        }
    }
    // pub fn init(&self) {
    //     *self.queue.lock() = Some(VecDeque::new());
    // }
    pub fn push_task(&self, runnable: Runnable) {
        // self.queue.lock().as_mut().unwrap().push_back(runnable);
        self.queue.lock().push_back(runnable);
    }
    pub fn fetch_task(&self) -> Option<Runnable> {
        // debug!("fetch a task inside");
        self.queue.lock().pop_front()
    }
}

lazy_static! {
    static ref TASK_QUEUE: TaskQueue = TaskQueue::new();
}
// static TASK_QUEUE: TaskQueue = TaskQueue::new();

// pub fn init() {
//     TASK_QUEUE.init();
// }

/// Add a task into task queue
pub fn spawn<F>(future: F) -> (Runnable, Task<F::Output>)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    async_task::spawn(future, |runnable| {
        TASK_QUEUE.push_task(runnable);
    })
}

/// Return the number of the tasks executed
#[allow(unused)]
pub fn run_until_idle() -> usize {
    let mut n = 0;
    loop {
        if let Some(task) = TASK_QUEUE.fetch_task() {
            // info!("fetch a task");
            task.run();
            n += 1;
        } else {
            break;
        }
    }
    n
}

pub fn run_forever() -> ! {
    loop {
        if let Some(task) = TASK_QUEUE.fetch_task() {
            // info!("fetch a task");
            task.run();
        // } else {
            // debug!("no task");
        }
    }
}
