use alloc::{boxed::Box, sync::Arc};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    processor::{
        self,
        context::{EnvContext, KernelTaskContext, TaskContext},
    },
    utils::debug::stack_tracker::StackTracker,
};
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
    task_ctx: LocalContext,
    task_future: F,
}

impl<F: Future + Send + 'static> UserTaskFuture<F> {
    #[inline]
    pub fn new(thread: Arc<Thread>, future: F) -> Self {
        let mut task_ctx = TaskContext {
            thread: thread.clone(),
            page_table: thread.process.inner.lock().memory_set.page_table.clone(),
            env: EnvContext::new(),
        };
        task_ctx.env.stack_tracker = Some(StackTracker::new());
        let task_ctx = Box::new(task_ctx);
        let local_ctx = LocalContext::TaskContext(task_ctx);
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
        // hart.push_task(&mut self.task_ctx);

        // // TODO: refactor ?
        // let ret = Poll::Pending;
        // if this.task_ctx.task_ctx().thread.runnable() {
        // }

        // run the `threadloop`
        // SAFETY:
        // the task future(i.e. threadloop) won't be moved.
        // One way to avoid unsafe is to wrap the task_future in
        // a Mutex<Pin<Box<>>>>, which requires locking for every polling
        let ret = unsafe { Pin::new_unchecked(&mut this.task_future).poll(cx) };
        hart.pop_task(&mut this.task_ctx);

        // TODO change back old thread ctx
        // TODO clear tlb
        ret
    }
}

pub struct KernelTaskFuture<F: Future<Output = ()> + Send + 'static> {
    // always_local: AlwaysLocal,
    task_ctx: KernelTaskContext,
    task: F,
}

impl<F: Future<Output = ()> + Send + 'static> KernelTaskFuture<F> {
    pub fn new(task: F) -> Self {
        Self {
            task_ctx: KernelTaskContext {},
            // always_local: AlwaysLocal::new(),
            task,
        }
    }
}

impl<F: Future<Output = ()> + Send + 'static> Future for KernelTaskFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
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
        hart.push_kernel_task(&mut this.task_ctx);
        ret
        // todo!("Finish kernel task switch");
    }
}
