use alloc::{boxed::Box, sync::Arc};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use log::trace;

use crate::processor::{self, context::UserTaskContext};
// use crate::process::context::TaskContext;
use crate::processor::context::LocalContext;

use super::Thread;

pub struct YieldFuture(pub bool);

impl Future for YieldFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.0 {
            return Poll::Ready(());
        }
        self.0 = true;
        // Wake up this future, which means putting this thread into
        // the tail of the task queue
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

/// The outermost future, i.e. the future that wraps
/// one thread's task future(doing some env context changes e.g.
/// pagetable switching)
pub struct UserTaskFuture<F: Future + Send + 'static> {
    task_ctx: Box<LocalContext>,
    task_future: F,
}

impl<F: Future + Send + 'static> UserTaskFuture<F> {
    #[inline]
    pub fn new(thread: Arc<Thread>, future: F) -> Self {
        let task_ctx = UserTaskContext {
            thread: thread.clone(),
            page_table: thread.process.inner.lock().memory_space.page_table.clone(),
        };
        // task_ctx.env.stack_tracker = Some(StackTracker::new());
        let local_ctx = Box::new(LocalContext::new(Some(task_ctx), None));
        Self {
            task_ctx: local_ctx,
            task_future: future,
        }
    }
}

impl<F: Future + Send + 'static> Future for UserTaskFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // There are 2 cases that are safe:
        // 1. the outermost future itself is unpin
        // 2. the outermost future isn't unpin but we make sure that it won't be moved
        // SAFETY: although getting the mut ref of a pin type is unsafe,
        // we only need to change the task_ctx, which is ok
        let this = unsafe { self.get_unchecked_mut() };
        // let this = self.get_mut();
        let hart = processor::local_hart();
        hart.push_task(&mut this.task_ctx);

        // debug!("[poll]: run task");

        // run the `threadloop`
        // SAFETY:
        // the task future(i.e. threadloop) won't be moved.
        // One way to avoid unsafe is to wrap the task_future in
        // a Mutex<Pin<Box<>>>>, which requires locking for every polling
        let ret = unsafe { Pin::new_unchecked(&mut this.task_future).poll(cx) };
        hart.pop_task(&mut this.task_ctx);

        ret
    }
}

pub struct KernelTaskFuture<F: Future<Output = ()> + Send + 'static> {
    // always_local: AlwaysLocal,
    task_ctx: Box<LocalContext>,
    task: F,
}

impl<F: Future<Output = ()> + Send + 'static> KernelTaskFuture<F> {
    pub fn new(task: F) -> Self {
        Self {
            task_ctx: Box::new(LocalContext::new(None, None)),
            // always_local: AlwaysLocal::new(),
            task,
        }
    }
}

impl<F: Future<Output = ()> + Send + 'static> Future for KernelTaskFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        trace!("switch to kernel task");
        let this = unsafe { self.get_unchecked_mut() };

        let hart = processor::local_hart();
        hart.push_kernel_task(&mut this.task_ctx);
        let ret = unsafe {
            // let local = local::hart_local();
            // let this = self.get_unchecked_mut();
            // local.switch_kernel_task(&mut this.always_local);
            Pin::new_unchecked(&mut this.task).poll(cx)
            // local.switch_kernel_task(&mut this.always_local);
        };
        hart.pop_kernel_task(&mut this.task_ctx);
        ret
        // todo!("Finish kernel task switch");
    }
}
