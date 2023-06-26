use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use super::{current_time_duration, Timer, TIMER_QUEUE};

pub enum TimeoutTaskOutput<T> {
    Timeout,
    Ok(T),
}

pub struct TimeoutTaskFuture<F: Future + Send + 'static> {
    expired_time: Duration,
    task_future: F,
}

impl<F: Future + Send + 'static> TimeoutTaskFuture<F> {
    pub fn new(duration: Duration, task_future: F) -> Self {
        Self {
            expired_time: current_time_duration() + duration,
            task_future,
        }
    }
}

impl<F: Future + Send + 'static> Future for TimeoutTaskFuture<F> {
    type Output = TimeoutTaskOutput<F::Output>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        let ret = unsafe { Pin::new_unchecked(&mut this.task_future).poll(cx) };
        if ret.is_pending() {
            if current_time_duration() >= this.expired_time {
                Poll::Ready(TimeoutTaskOutput::Timeout)
            } else {
                // TODO: avoid adding to timer list repeatly
                let timer = Timer {
                    expired_time: this.expired_time,
                    waker: Some(cx.waker().clone()),
                };
                TIMER_QUEUE.add_timer(timer);
                Poll::Pending
            }
        } else {
            Poll::Ready(TimeoutTaskOutput::Ok(ret.ready()?))
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
    TimeoutTaskFuture::new(duration, IdleFuture {}).await;
}
