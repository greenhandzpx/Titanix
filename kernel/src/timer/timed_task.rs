use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use super::{current_time_duration, Timer, TIMER_QUEUE};

pub struct TimedTaskFuture<F: Fn() -> bool> {
    interval: Duration,
    /// callback: return false if the timer should be over
    callback: F,
}

impl<F: Fn() -> bool> TimedTaskFuture<F> {
    pub fn new(interval: Duration, callback: F) -> Self {
        Self { interval, callback }
    }
}

impl<F: Fn() -> bool> Future for TimedTaskFuture<F> {
    // type Output = TimeoutTaskOutput<F::Output>;
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !(self.callback)() {
            return Poll::Ready(());
        }
        let current_ts = current_time_duration();
        let timer = Timer {
            expired_time: self.interval + current_ts,
            waker: Some(cx.waker().clone()),
        };
        TIMER_QUEUE.add_timer(timer);
        Poll::Pending
    }
}
