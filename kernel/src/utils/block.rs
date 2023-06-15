use core::{future::Future, task::{Context, Poll}};

use alloc::{task::Wake, sync::Arc, boxed::Box};
use log::trace;

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