use crate::sync::mutex::SpinNoIrqLock;
use alloc::collections::VecDeque;
use async_task::{Runnable, Task};
use core::future::Future;
use lazy_static::*;

struct TaskQueue {
    queue: SpinNoIrqLock<VecDeque<Runnable>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            queue: SpinNoIrqLock::new(VecDeque::new()),
        }
    }
    pub fn init(&self) {
        // *self.queue.lock() = Some(VecDeque::new());
    }
    pub fn push_task(&self, runnable: Runnable) {
        // self.queue.lock().as_mut().unwrap().push_back(runnable);
        self.queue.lock().push_back(runnable);
    }
    pub fn fetch_task(&self) -> Option<Runnable> {
        // println!("fetch a task inside");
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

/// 将一个任务加入执行器
pub fn spawn<F>(future: F) -> (Runnable, Task<F::Output>)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    async_task::spawn(future, |runnable| {
        TASK_QUEUE.push_task(runnable);
    })
}

/// 返回执行了多少个future
pub fn run_until_idle() -> usize {
    let mut n = 0;
    loop {
        if let Some(task) = TASK_QUEUE.fetch_task() {
            // println!("fetch a task");
            task.run();
            n += 1;
        } else {
            break;
        }
    }
    n
}
