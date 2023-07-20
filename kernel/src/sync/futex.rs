use core::{
    cell::UnsafeCell,
    future::Future,
    intrinsics::atomic_load_acquire,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use alloc::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::Arc,
};

use crate::{
    mm::{user_check::UserCheck, VirtAddr},
    processor::{current_process, current_task, SumGuard},
    stack_trace,
    utils::{
        cell::SyncUnsafeCell,
        error::{GeneralRet, SyscallRet},
    },
};

/// Futex queue that stores: uaddr -> waiters(tid -> waiter)
pub struct FutexQueue(pub BTreeMap<VirtAddr, BTreeMap<usize, FutexWaiter>>);

impl FutexQueue {
    /// Construct a futex queue
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Wait
    pub fn emplace_waiter(
        &mut self,
        addr: Arc<SyncUnsafeCell<VirtAddr>>,
        tid: usize,
        waker: Waker,
    ) {
        let waiter = FutexWaiter::new(tid, addr, waker);
        self.add_waiter(waiter);
    }

    ///
    fn add_waiter(&mut self, waiter: FutexWaiter) {
        let addr = waiter.addr.get_unchecked_mut().clone();
        if let Some(queue) = self.0.get_mut(&addr) {
            queue.insert(waiter.tid, waiter);
        } else {
            let mut queue = BTreeMap::new();
            queue.insert(waiter.tid, waiter);
            self.0.insert(addr, queue);
        }
    }

    fn remove_waiter(&mut self, addr: VirtAddr, tid: usize) {
        if let Some(queue) = self.0.get_mut(&addr) {
            queue.remove(&tid);
        }
    }

    /// Wake up `nval` waiters.
    pub fn wake(&mut self, addr: VirtAddr, nval: usize) -> usize {
        // if let Some(waiters) = self.0.get_mut(&addr) {
        //     for (i, waiter) in waiters.iter().enumerate() {
        //         if i == nval {
        //             return nval;
        //         }
        //         waiter.1.wake_by_ref();
        //     }
        //     waiters.len()
        // } else {
        //     0
        // }
        if let Some(waiters) = self.0.get_mut(&addr) {
            for i in 0..nval {
                if waiters.is_empty() {
                    return i;
                }
                let waiter = waiters.pop_first().unwrap();
                log::info!("[FutexQueue::wake] wake up {}", waiter.0);
                waiter.1.wake();
            }
            nval
        } else {
            0
        }
    }

    /// Wake up at most nval_wake `old_addr`'s waiters.
    /// Migrate at most nval_rq `old_addr`'s waiters to `new_addr`.
    /// Return the number of threads waking up and
    ///  migrating to the new queue
    pub fn requeue_waiters(
        &mut self,
        old_addr: VirtAddr,
        new_addr: VirtAddr,
        nval_wake: usize,
        nval_rq: usize,
    ) -> usize {
        if old_addr.0 == new_addr.0 {
            return 0;
        }
        let ret = self.wake(old_addr, nval_wake);
        for i in 0..nval_rq {
            if let Some(old_queue) = self.0.get_mut(&old_addr) {
                if let Some((_, waiter)) = old_queue.pop_first() {
                    *waiter.addr.get_unchecked_mut() = new_addr;
                    self.add_waiter(waiter);
                }
            } else {
                return ret + i;
            }
        }
        ret + nval_rq
    }

    /// Wake up one waiter.
    /// Returns the waiter's tid.
    pub fn wake_one(&mut self, addr: VirtAddr) -> Option<usize> {
        // if let Some(waiters) = self.0.get_mut(&addr) {
        //     for (_, waiter) in waiters.iter() {
        //         let tid = waiter.tid;
        //         waiter.wake_by_ref();
        //         return Some(tid);
        //     }
        //     None
        // } else {
        //     None
        // }
        if let Some(waiters) = self.0.get_mut(&addr) {
            log::info!(
                "[FutexQueue::wake_one] addr {:#x} waiters len {}",
                addr.0,
                waiters.len()
            );
            if let Some((tid, waiter)) = waiters.pop_first() {
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
    addr: Arc<SyncUnsafeCell<VirtAddr>>,
    waker: Waker,
}

impl FutexWaiter {
    fn new(tid: usize, addr: Arc<SyncUnsafeCell<VirtAddr>>, waker: Waker) -> Self {
        Self { tid, addr, waker }
    }
    fn wake(self) {
        self.waker.wake();
    }
    #[allow(unused)]
    fn wake_by_ref(&self) {
        self.waker.wake_by_ref();
    }
}

/// Futex future for waiters
pub struct FutexFuture {
    /// The reason why we use Arc here is that we may need to change the
    /// addr this thread is waiting for(i.e. futex queue) through another thread.
    /// The reason why we use UnsafeCell is that since we will lock the process
    /// every time we poll, there is no need locking again.
    addr: Arc<SyncUnsafeCell<VirtAddr>>,
    expected_val: u32,
    has_added_waiter: UnsafeCell<bool>,
}

impl FutexFuture {
    /// Construct a futex future
    pub fn new(addr: VirtAddr, expected_val: u32) -> Self {
        Self {
            addr: Arc::new(SyncUnsafeCell::new(addr)),
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
        current_process().inner_handler(|proc| {
            let addr = unsafe { *self.addr.get() };
            if !unsafe { *self.has_added_waiter.get() } {
                proc.futex_queue.emplace_waiter(
                    self.addr.clone(),
                    current_task().tid(),
                    cx.waker().clone(),
                );
                unsafe {
                    *self.has_added_waiter.get() = true;
                }

                // Check the value in case that someone change that value before waking us up.
                if unsafe { atomic_load_acquire(addr.0 as *const u32) } != self.expected_val {
                    return Poll::Ready(());
                } else {
                    return Poll::Pending;
                }
            }
            log::info!(
                "[FutexFuture::poll] wake up, addr {:#x}, val {:#x}",
                addr.0,
                unsafe { atomic_load_acquire(addr.0 as *const u32) }
            );
            proc.futex_queue.remove_waiter(addr, current_task().tid());
            // TODO: change thread's owned futexes when requeue
            // unsafe {
            //     (*current_task().inner.get()).owned_futexes.0.insert(addr);
            // }
            Poll::Ready(())
            // // let addr_locked = self.addr.lock();
            // let addr = unsafe { *self.addr.get() };
            // if unsafe { atomic_load_acquire(addr.0 as *const u32) } != self.expected_val {
            //     proc.futex_queue.remove_waiter(addr, current_task().tid());
            //     // TODO: change thread's owned futexes when requeue
            //     unsafe {
            //         (*current_task().inner.get()).owned_futexes.0.insert(addr);
            //     }
            //     Poll::Ready(())
            // } else {
            //     Poll::Pending
            // }
        })
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
    // if UserCheck::new()
    //     .check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())
    //     .is_err()
    // {
    //     log::warn!("[futex_wake_one] invalid addr {:#x}", uaddr);
    //     return Ok(None);
    // }
    UserCheck::new().check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
    unsafe {
        (*current_task().inner.get())
            .owned_futexes
            .0
            .remove(&crate::mm::VirtAddr(uaddr));
    }
    Ok(current_process().inner_handler(|proc| proc.futex_queue.wake_one(uaddr.into())))
}

// pub fn futex_

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
        stack_trace!();
        log::info!(
            "[OwnedFutexes::owner_died] owned futexes len {}",
            self.0.len()
        );
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
                log::info!("[owner_died] futex word val {:#x}", unsafe {
                    *(addr.0 as *const u32)
                });
            } else {
                log::warn!("[handle_exit] futex wake err?!");
            }
        }
    }
}
