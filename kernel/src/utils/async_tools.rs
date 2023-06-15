use core::{task::{Waker, Context, Poll}, future::Future, pin::Pin, ops::{Deref, DerefMut}};


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
pub struct SendWrapper<T> (pub T);

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