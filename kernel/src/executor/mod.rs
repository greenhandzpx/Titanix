use crate::sync::mutex::SpinNoIrqLock;
use alloc::collections::VecDeque;
use async_task::{Runnable, ScheduleInfo, Task, WithInfo};
use core::future::Future;

struct TaskQueue {
    queue: SpinNoIrqLock<Option<VecDeque<Runnable>>>,
}

impl TaskQueue {
    pub const fn new() -> Self {
        Self {
            queue: SpinNoIrqLock::new(None),
        }
    }
    pub fn init(&self) {
        *self.queue.lock() = Some(VecDeque::new());
    }
    pub fn push(&self, runnable: Runnable) {
        self.queue.lock().as_mut().unwrap().push_back(runnable);
    }
    pub fn push_preempt(&self, runnable: Runnable) {
        self.queue.lock().as_mut().unwrap().push_front(runnable);
    }
    pub fn fetch(&self) -> Option<Runnable> {
        self.queue.lock().as_mut().unwrap().pop_front()
    }
}

static TASK_QUEUE: TaskQueue = TaskQueue::new();

pub fn init() {
    TASK_QUEUE.init();
}

/// Add a task into task queue
pub fn spawn<F>(future: F) -> (Runnable, Task<F::Output>)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let schedule = move |runnable: Runnable, info: ScheduleInfo| {
        // TASK_QUEUE.push(runnable);
        if info.woken_while_running {
            // i.e `yield_now()`
            TASK_QUEUE.push(runnable);
        } else {
            // i.e. woken up by some signal
            TASK_QUEUE.push_preempt(runnable);
        }
    };
    async_task::spawn(future, WithInfo(schedule))
}

/// Return the number of the tasks executed
#[allow(unused)]
pub fn run_until_idle() -> usize {
    let mut n = 0;
    loop {
        if let Some(task) = TASK_QUEUE.fetch() {
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
        if let Some(task) = TASK_QUEUE.fetch() {
            // info!("fetch a task");
            task.run();
        }
    }
}
