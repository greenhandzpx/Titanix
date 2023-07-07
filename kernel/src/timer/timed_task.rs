use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use super::{current_time_duration, Timer, TIMER_QUEUE};

pub struct TimedTaskFuture<F: Fn() -> bool> {
    interval: Duration,
    next_expired_time: Duration,
    // /// First trigger timeout
    // first_trigger_timeout: Option<Duration>,
    /// Callback: return false if the timer should be over
    callback: F,
}

impl<F: Fn() -> bool> TimedTaskFuture<F> {
    pub fn new(interval: Duration, callback: F, next_expired_time: Duration) -> Self {
        Self {
            interval,
            next_expired_time,
            callback,
            // first_trigger_timeout,
        }
    }
}

impl<F: Fn() -> bool> Future for TimedTaskFuture<F> {
    // type Output = TimeoutTaskOutput<F::Output>;
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_ts = current_time_duration();
        if current_ts >= self.next_expired_time {
            if !(self.callback)() {
                return Poll::Ready(());
            }
            let this = unsafe { self.get_unchecked_mut() };
            this.next_expired_time = current_ts + this.interval;
            let timer = Timer {
                expired_time: this.next_expired_time,
                waker: Some(cx.waker().clone()),
            };
            TIMER_QUEUE.add_timer(timer);
        }

        // If single core
        #[cfg(not(feature = "kernel_interrupt"))]
        cx.waker().wake_by_ref();

        Poll::Pending
    }
}
