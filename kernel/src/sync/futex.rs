use core::{
    future::Future,
    intrinsics::atomic_load_acquire,
    pin::Pin,
    task::{Context, Poll, Waker}, cell::UnsafeCell,
};

use alloc::collections::{BTreeMap, VecDeque};

use crate::{mm::VirtAddr, processor::current_process};

/// Futex queue that stores: uaddr -> waiters
pub struct FutexQueue(pub BTreeMap<VirtAddr, VecDeque<FutexWaiter>>);

impl FutexQueue {

    /// Construct a futex queue
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Wait 
    pub fn add_waiter(&mut self, addr: VirtAddr, waker: Waker) {
        let waiter = FutexWaiter::new(waker);
        if let Some(queue) = self.0.get_mut(&addr) {
            queue.push_back(waiter);
        } else {
            let mut queue = VecDeque::new();
            queue.push_back(waiter);
            self.0.insert(addr, queue);
        }
    }

    /// Wake
    pub fn wake(&mut self, addr: VirtAddr, nval: usize) -> usize {
        if let Some(waiters) = self.0.get_mut(&addr) {
            for i in 0..nval {
                if waiters.is_empty() {
                    return i;
                }
                let waiter = waiters.pop_front().unwrap();
                waiter.wake();
            }
            nval
        } else {
            0
        }
    }
}

pub struct FutexWaiter(Waker);

impl FutexWaiter {
    fn new(waker: Waker) -> Self {
        Self(waker)
    }
    fn wake(self) {
        self.0.wake();
    }
}

/// Futex future for waiters
pub struct FutexFuture {
    addr: VirtAddr,
    expected_val: u32,
    has_added_waiter: UnsafeCell<bool>,
}

impl FutexFuture {
    /// Construct a futex future
    pub fn new(addr: VirtAddr, expected_val: u32) -> Self {
        Self {
            addr,
            expected_val,
            has_added_waiter: UnsafeCell::new(false),
        }
    }
}

impl Future for FutexFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Add waiter before we try to load the value.
        // Because If the waker wakes up us after we load the value and
        // before we add the waiter, then we will sleep forever.
        if !unsafe { *self.has_added_waiter.get() } {
            current_process().inner_handler(|proc| {
                proc.futex_queue.add_waiter(self.addr, cx.waker().clone());
            });
            unsafe { *self.has_added_waiter.get() = true; }
        }
        if unsafe { atomic_load_acquire(self.addr.0 as *const u32) } != self.expected_val {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
