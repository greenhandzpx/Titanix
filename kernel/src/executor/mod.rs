use crate::{processor::local_hart, sync::mutex::SpinNoIrqLock};
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
        // log::error!("push lock before");
        let mut lock = self.queue.lock();
        // log::error!("push before, queue len {}", lock.as_mut().unwrap().len());
        lock.as_mut().unwrap().push_back(runnable);
        // self.queue.lock().as_mut().unwrap().push_back(runnable);
        // log::error!("push after");
    }
    pub fn push_preempt(&self, runnable: Runnable) {
        self.queue.lock().as_mut().unwrap().push_front(runnable);
    }
    pub fn fetch(&self) -> Option<Runnable> {
        self.queue.lock().as_mut().unwrap().pop_front()
    }

    fn is_empty(&self) -> bool {
        self.queue.lock().as_ref().unwrap().is_empty()
    }
}

static TASK_QUEUE: TaskQueue = TaskQueue::new();
static TIME_CONSUMING_TASK_QUEUE: TaskQueue = TaskQueue::new();

pub fn init() {
    TASK_QUEUE.init();
    TIME_CONSUMING_TASK_QUEUE.init();
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
            // log::error!("yield now");
            TASK_QUEUE.push(runnable);
        } else {
            // i.e. woken up by some signal
            TASK_QUEUE.push_preempt(runnable);
        }
    };
    async_task::spawn(future, WithInfo(schedule))
}

/// Add a task into task queue
pub fn spawn_time_consuming<F>(future: F) -> (Runnable, Task<F::Output>)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let schedule = move |runnable: Runnable, info: ScheduleInfo| {
        // TASK_QUEUE.push(runnable);
        if info.woken_while_running {
            // i.e `yield_now()`
            // log::error!("yield now");
            TIME_CONSUMING_TASK_QUEUE.push(runnable);
        } else {
            // i.e. woken up by some signal
            TIME_CONSUMING_TASK_QUEUE.push_preempt(runnable);
        }
    };
    async_task::spawn(future, WithInfo(schedule))
}

/// Return the number of the tasks executed
pub fn run_until_idle() -> usize {
    let mut n = 0;
    loop {
        if let Some(task) = TASK_QUEUE.fetch() {
            // log::info!("fetch a task");
            task.run();

            n += 1;
        } else if let Some(task) = TIME_CONSUMING_TASK_QUEUE.fetch() {
            task.run();

            n += 1;
        } else {
            #[cfg(feature = "kernel_preempt")]
            panic!("There must be at least the idle task thread");

            #[cfg(not(feature = "kernel_preempt"))]
            // log::info!("No more task");
            break;
        }
    }
    n
}

/// Only run tasks that are NOT time-consuming
pub fn run_one_task() {
    if let Some(task) = TASK_QUEUE.fetch() {
        // log::info!("fetch a task");
        task.run();
    }
}

/// Only take care of NOT time-consuming tasks
pub fn has_task() -> bool {
    !TASK_QUEUE.is_empty()
}

#[allow(unused)]
pub fn run_forever() -> ! {
    let mut cnt = 0;
    loop {
        if let Some(task) = TASK_QUEUE.fetch() {
            println!("{} fetch a task", local_hart().hart_id());
            task.run();
        } else {
            cnt += 1;
            if cnt == 1000000 {
                println!("{} no more task", local_hart().hart_id());
                cnt = 0;
            }
        }
    }
}
