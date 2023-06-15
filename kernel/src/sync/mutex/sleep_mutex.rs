use core::{cell::{UnsafeCell, SyncUnsafeCell}, task::{Waker, Context, Poll}, ops::{Deref, DerefMut}, pin::Pin, future::Future, sync::atomic::{AtomicBool, Ordering}};


use crate::utils::async_tools;

use super::{MutexSupport, spin_mutex::SpinMutex};
use alloc::sync::Arc;
use intrusive_collections::{LinkedList, LinkedListLink, intrusive_adapter};


intrusive_adapter!(
    SMQueueAdapter = Arc<GrantInfo>: GrantInfo { link: LinkedListLink }
);

struct MutexInner {
    locked: bool, 
    // queue: 
    queue: LinkedList<SMQueueAdapter>,
}


/// SleepMutex can step over `await`
pub struct SleepMutex<T: ?Sized, S: MutexSupport> {
    lock: SpinMutex<MutexInner, S>, // push at prev, release at next
    data: UnsafeCell<T>,            // actual data
}


impl<'a, T, S: MutexSupport> SleepMutex<T, S> {
    /// Construct a SleepMutex
    pub fn new(user_data: T) -> Self {
        SleepMutex {
            lock: SpinMutex::new(MutexInner { locked: false, queue: LinkedList::new(SMQueueAdapter::new())}),
            // _marker: PhantomData,
            data: UnsafeCell::new(user_data),
            // debug_cnt: UnsafeCell::new(0),
        }
    }
}

impl<T: ?Sized + Send, S: MutexSupport> SleepMutex<T, S> {
    /// Lock
    #[inline]
    pub async fn lock(&self) -> impl DerefMut<Target = T> + Send + Sync + '_ {
        let future = &mut SleepMutexFuture::new(self);
        unsafe { Pin::new_unchecked(future).init().await.await }
    }
}


struct GrantInfo {
    inner: SyncUnsafeCell<(AtomicBool, Option<Waker>)>,
    // granted: bool,
    // waker: Option<Waker>,
    link: LinkedListLink,
}

struct SleepMutexFuture<'a, T: ?Sized, S: MutexSupport> {
    mutex: &'a SleepMutex<T, S>,
    grant: Arc<GrantInfo>,
}

impl<'a, T: ?Sized, S: MutexSupport> SleepMutexFuture<'a, T, S> {
    #[inline(always)]
    fn new(mutex: &'a SleepMutex<T, S>) -> Self {
        SleepMutexFuture {
            mutex,
            grant: Arc::new(GrantInfo {
                inner: SyncUnsafeCell::new((AtomicBool::new(false), None)),
                // granted: false,
                // waker: None,
                link: LinkedListLink::new(),
            }),
        }
    }

    async fn init(self: Pin<&mut Self>) -> Pin<&mut SleepMutexFuture<'a, T, S>> {
        let this = unsafe { self.get_unchecked_mut() };
        let mut inner = unsafe { &mut *this.mutex.lock.sent_lock() };
        if !inner.locked {
            // The sleep lock is not yet locked, just granted.
            inner.locked = true;
            unsafe {&mut *this.grant.inner.get()}.0.store(true, Ordering::Release);
        } else {
            unsafe {&mut *this.grant.inner.get()}.1 = Some(async_tools::take_waker().await);
            inner.queue.push_back(this.grant.clone());
        }
        unsafe { Pin::new_unchecked(this) }
    }

}

impl<'a, T: ?Sized, S: MutexSupport> Future for SleepMutexFuture<'a, T, S> {
    type Output = SleepMutexGuard<'a, T, S>;
    #[inline(always)]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let granted = unsafe { & *self.grant.inner.get() }.0.load(Ordering::Acquire);
        match granted {
            false => Poll::Pending,
            true => Poll::Ready(SleepMutexGuard { mutex: self.mutex }),
        }
    }
}

struct SleepMutexGuard<'a, T: ?Sized, S: MutexSupport> {
    mutex: &'a SleepMutex<T, S>,
}

unsafe impl<'a, T: ?Sized + Send, S: MutexSupport> Send for SleepMutexGuard<'a, T, S> {}
unsafe impl<'a, T: ?Sized + Send, S: MutexSupport> Sync for SleepMutexGuard<'a, T, S> {}

impl<'a, T: ?Sized, S: MutexSupport> Deref for SleepMutexGuard<'a, T, S> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T: ?Sized, S: MutexSupport> DerefMut for SleepMutexGuard<'a, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T: ?Sized, S: MutexSupport> Drop for SleepMutexGuard<'a, T, S> {
    #[inline]
    fn drop(&mut self) {
        let mut inner = self.mutex.lock.lock();
        debug_assert!(inner.locked);
        let waiter = match inner.queue.pop_front() {
            None => {
                // The wait queue is empty
                inner.locked = false; 
                return;
            }
            Some(waiter) => waiter,
        };
        drop(inner);
        // Waker should be fetched before we make the grant_inner.0 true
        // since it will be invalid after that.
        let grant_inner = unsafe { 
            &mut *waiter.inner.get()
        };
        let waker = grant_inner.1.take().unwrap();
        grant_inner.0.store(true, Ordering::Release);
        waker.wake();
    }
}

