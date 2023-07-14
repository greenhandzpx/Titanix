use core::{
    cell::UnsafeCell,
    future::Future,
    intrinsics::atomic_load_acquire,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use alloc::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{
    mm::{user_check::UserCheck, VirtAddr},
    processor::{current_process, current_task, SumGuard},
    stack_trace,
    utils::error::{GeneralRet, SyscallRet},
};

/// Futex queue that stores: uaddr -> waiters
pub struct FutexQueue(pub BTreeMap<VirtAddr, VecDeque<FutexWaiter>>);

impl FutexQueue {
    /// Construct a futex queue
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Wait
    pub fn add_waiter(&mut self, addr: VirtAddr, tid: usize, waker: Waker) {
        let waiter = FutexWaiter::new(tid, waker);
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

    /// Wake up one waiter.
    /// Returns the waiter's tid
    pub fn wake_one(&mut self, addr: VirtAddr) -> Option<usize> {
        if let Some(waiters) = self.0.get_mut(&addr) {
            if let Some(waiter) = waiters.pop_front() {
                let tid = waiter.tid;
                waiter.wake();
                Some(tid)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct FutexWaiter {
    tid: usize,
    waker: Waker,
}

impl FutexWaiter {
    fn new(tid: usize, waker: Waker) -> Self {
        Self { tid, waker }
    }
    fn wake(self) {
        self.waker.wake();
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
                proc.futex_queue
                    .add_waiter(self.addr, current_task().tid(), cx.waker().clone());
            });
            unsafe {
                *self.has_added_waiter.get() = true;
            }
        }
        if unsafe { atomic_load_acquire(self.addr.0 as *const u32) } != self.expected_val {
            unsafe {
                (*current_task().inner.get())
                    .owned_futexes
                    .0
                    .insert(self.addr);
            }
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// Wake up
pub fn futex_wake(uaddr: usize, val: u32) -> SyscallRet {
    stack_trace!();
    UserCheck::new().check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
    unsafe {
        (*current_task().inner.get())
            .owned_futexes
            .0
            .remove(&crate::mm::VirtAddr(uaddr));
    }
    let cnt =
        current_process().inner_handler(|proc| proc.futex_queue.wake(uaddr.into(), val as usize));
    return Ok(cnt as isize);
}

/// Wake up one waiter.
/// Return the waiter's tid
pub fn futex_wake_one(uaddr: usize) -> GeneralRet<Option<usize>> {
    stack_trace!();
    UserCheck::new().check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
    unsafe {
        (*current_task().inner.get())
            .owned_futexes
            .0
            .remove(&crate::mm::VirtAddr(uaddr));
    }
    Ok(current_process().inner_handler(|proc| proc.futex_queue.wake_one(uaddr.into())))
}

pub const FUTEX_OWNER_DIED: u32 = 1 << 30;
///
pub struct OwnedFutexes(pub BTreeSet<VirtAddr>);

impl OwnedFutexes {
    ///
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    ///
    pub fn owner_died(&mut self) {
        let _sum_guard = SumGuard::new();
        while let Some(addr) = self.0.pop_first() {
            if let Some(tid) = futex_wake_one(addr.0).ok() {
                if let Some(tid) = tid {
                    unsafe {
                        *(addr.0 as *mut u32) = tid as u32;
                        *(addr.0 as *mut u32) |= FUTEX_OWNER_DIED;
                    }
                } else {
                    unsafe {
                        *(addr.0 as *mut u32) |= FUTEX_OWNER_DIED;
                    }
                }
                log::debug!("[owner_died] futex word {:#x}", unsafe {
                    *(addr.0 as *const u32)
                });
            } else {
                log::warn!("[handle_exit] futex wake err?!");
            }
        }
    }
}
