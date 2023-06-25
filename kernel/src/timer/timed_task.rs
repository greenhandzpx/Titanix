use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
    time::Duration,
};

use alloc::collections::LinkedList;

use crate::sync::mutex::SpinNoIrqLock;

use super::current_time_duration;
use lazy_static::*;

pub fn handle_timeout_events() {
    // debug!("[handle_timeout_events]: start..., sepc {:#x}", sepc::read());
    let mut timers = TIMER_LIST.timers.lock();
    let current_time = current_time_duration();
    let mut timeout_cnt = 0;
    for timer in timers.iter_mut() {
        if current_time >= timer.expired_time {
            timer.waker.take().unwrap().wake();
            timeout_cnt += 1;
        }
    }
    for _ in 0..timeout_cnt {
        timers.pop_front();
    }
    // debug!("[handle_timeout_events]: finish, timeout cnt {}", timeout_cnt);
}

struct TimerList {
    timers: SpinNoIrqLock<LinkedList<Timer>>,
}

lazy_static! {
    static ref TIMER_LIST: TimerList = TimerList {
        timers: SpinNoIrqLock::new(LinkedList::new())
    };
}

struct Timer {
    expired_time: Duration,
    waker: Option<Waker>,
    // waker: SyncUnsafeCell<Option<Waker>>,
}

pub enum TimedTaskOutput<T> {
    Timeout,
    Ok(T),
}

impl<T> TimedTaskOutput<T> {
    pub fn timeout(&self) -> bool {
        match self {
            Self::Timeout => true,
            _ => false,
        }
    }

}

pub struct TimedTaskFuture<F: Future + Send + 'static> {
    expired_time: Duration,
    task_future: F,
}

impl<F: Future + Send + 'static> TimedTaskFuture<F> {
    pub fn new(duration: Duration, task_future: F) -> Self {
        Self {
            expired_time: current_time_duration() + duration,
            task_future,
        }
    }
}

impl<F: Future + Send + 'static> Future for TimedTaskFuture<F> {
    type Output = TimedTaskOutput<F::Output>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        let ret = unsafe { Pin::new_unchecked(&mut this.task_future).poll(cx) };
        if ret.is_pending() {
            if current_time_duration() >= this.expired_time {
                Poll::Ready(TimedTaskOutput::Timeout)
            } else {
                // TODO: avoid adding to timer list repeatly
                let timer = Timer {
                    expired_time: this.expired_time,
                    waker: Some(cx.waker().clone()),
                };
                TIMER_LIST.timers.lock().push_back(timer);
                Poll::Pending
            }
        } else {
            Poll::Ready(TimedTaskOutput::Ok(ret.ready()?))
        }
    }
}

struct IdleFuture;

impl Future for IdleFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

#[allow(unused)]
pub async fn ksleep(duration: Duration) {
    TimedTaskFuture::new(duration, IdleFuture {}).await;
}
