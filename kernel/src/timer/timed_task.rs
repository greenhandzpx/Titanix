use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use super::{current_time_duration, Timer, TIMER_QUEUE};

pub struct TimedTaskFuture<F: Fn() -> bool> {
    interval: Duration,
    /// First trigger timeout
    first_trigger_timeout: Option<Duration>,
    /// Callback: return false if the timer should be over
    callback: F,
}

impl<F: Fn() -> bool> TimedTaskFuture<F> {
    pub fn new(interval: Duration, callback: F, first_trigger_timeout: Option<Duration>) -> Self {
        Self {
            interval,
            callback,
            first_trigger_timeout,
        }
    }
}

impl<F: Fn() -> bool> Future for TimedTaskFuture<F> {
    // type Output = TimeoutTaskOutput<F::Output>;
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_ts = current_time_duration();
        let expired_time = if let Some(timeout) = self.first_trigger_timeout && !timeout.is_zero() {
            let ret = current_ts + timeout;
            // TODO: is it safe ?
            unsafe { self.get_unchecked_mut() }.first_trigger_timeout = None;
            ret
        } else {
            if !(self.callback)() {
                return Poll::Ready(());
            }
            current_ts + self.interval
        };
        let timer = Timer {
            expired_time,
            waker: Some(cx.waker().clone()),
        };
        TIMER_QUEUE.add_timer(timer);

        // If single core
        cx.waker().wake_by_ref();

        Poll::Pending
    }
}
