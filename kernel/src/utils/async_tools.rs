use core::{
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll, Waker},
};

use alloc::{boxed::Box, sync::Arc, task::Wake};
use log::trace;

/// Take the waker of the current future
#[inline(always)]
pub async fn take_waker() -> Waker {
    TakeWakerFuture.await
}

struct TakeWakerFuture;

impl Future for TakeWakerFuture {
    type Output = Waker;
    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(cx.waker().clone())
    }
}

/// A wrapper for a data structure that be sent between threads
pub struct SendWrapper<T>(pub T);

impl<T> SendWrapper<T> {
    pub fn new(data: T) -> Self {
        SendWrapper(data)
    }
}

unsafe impl<T> Send for SendWrapper<T> {}
unsafe impl<T> Sync for SendWrapper<T> {}

impl<T: Deref> Deref for SendWrapper<T> {
    type Target = T::Target;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: DerefMut> DerefMut for SendWrapper<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

/// A waker that wakes up the current thread when called.
struct BlockWaker;

impl Wake for BlockWaker {
    fn wake(self: Arc<Self>) {
        trace!("block waker wakes");
    }
}

/// Run a future to completion on the current thread.
/// Note that since this function is used in kernel mode,
/// we won't switch thread when the inner future pending.
/// Instead, we just poll the inner future again and again.
pub fn block_on<T>(fut: impl Future<Output = T>) -> T {
    // Pin the future so it can be polled.
    let mut fut = Box::pin(fut);

    let waker = Arc::new(BlockWaker).into();
    let mut cx = Context::from_waker(&waker);

    // Run the future to completion.
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(res) => return res,
            Poll::Pending => continue,
        }
    }
}

pub enum SelectOutput<T1, T2> {
    Output1(T1),
    Output2(T2),
}

/// Select two futures at a time.
/// Note that future1 has a higher level than future2
pub struct Select2Futures<T1, T2, F1, F2>
where
    F1: Future<Output = T1>,
    F2: Future<Output = T2>,
{
    future1: F1,
    future2: F2,
}

impl<T1, T2, F1, F2> Select2Futures<T1, T2, F1, F2>
where
    F1: Future<Output = T1>,
    F2: Future<Output = T2>,
{
    pub fn new(future1: F1, future2: F2) -> Self {
        Self { future1, future2 }
    }
}

impl<T1, T2, F1, F2> Future for Select2Futures<T1, T2, F1, F2>
where
    F1: Future<Output = T1>,
    F2: Future<Output = T2>,
{
    type Output = SelectOutput<T1, T2>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        let ret = unsafe { Pin::new_unchecked(&mut this.future1).poll(cx) };
        if ret.is_ready() {
            return Poll::Ready(SelectOutput::Output1(ret.ready()?));
        }
        let ret = unsafe { Pin::new_unchecked(&mut this.future2).poll(cx) };
        if ret.is_ready() {
            return Poll::Ready(SelectOutput::Output2(ret.ready()?));
        }
        Poll::Pending
    }
}
