use core::{
    marker::PhantomData,
    mem::{transmute_copy, ManuallyDrop},
    ops::Deref,
    sync::atomic::{self, AtomicU64, Ordering},
};

use alloc::{boxed::Box, vec::Vec};

use crate::{
    processor::{preempt_disable, preempt_enable},
    sync::mutex::SpinNoIrqLock,
};

// pub fn rcu_report_qs() {

// }

// pub fn rcu_read_lock() {
//     preempt_disable();
// }

// pub fn rcu_read_unlock() {
//     preempt_enable();
// }

/// Give it a lifetime parameter to ensure it only lives as long as
/// the RCU box reference.
pub struct RcuReadGuard<'a, T> {
    _mark: PhantomData<&'a RcuBox<T>>,
    value: ManuallyDrop<RcuBox<T>>,
}

impl<T> !Send for RcuReadGuard<'_, T> {}
impl<T> !Sync for RcuReadGuard<'_, T> {}
impl<T> Deref for RcuReadGuard<'_, T> {
    type Target = Box<T>;
    fn deref(&self) -> &Self::Target {
        &self.value.deref().object
    }
}

impl<T> RcuReadGuard<'_, T> {
    pub fn new(value: ManuallyDrop<RcuBox<T>>) -> Self {
        preempt_disable();
        RcuReadGuard {
            _mark: PhantomData,
            value,
        }
    }
}

impl<T> Drop for RcuReadGuard<'_, T> {
    fn drop(&mut self) {
        preempt_enable();
    }
}

pub struct RcuBox<T> {
    object: Box<T>,
}

impl<T> Deref for RcuBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T> Drop for RcuBox<T> {
    fn drop(&mut self) {
        log::info!("start to drop me..");
    }
}

impl<T> RcuBox<T> {
    pub fn new(object: T) -> Self {
        RcuBox {
            object: Box::new(object),
        }
    }

    // #[inline(always)]
    // fn rcu_assert() {
    //     use core::mem::{align_of, size_of};
    //     assert_eq!(size_of::<Self>(), align_of::<Self>());
    //     assert!(size_of::<Self>() <= size_of::<usize>());
    //     assert!(size_of::<u32>() <= size_of::<usize>());
    // }

    pub fn rcu_read(&self) -> RcuReadGuard<T> {
        let value = unsafe { core::mem::ManuallyDrop::new(core::ptr::read_volatile(self)) };
        // log::info!("1111");
        // Ensure that the later reads won't be reordered before
        // the pointer fetching.
        atomic::fence(atomic::Ordering::Acquire);
        RcuReadGuard::new(value)
    }

    pub unsafe fn rcu_write(&self, src: Self) {
        // Ensure that the previous writes won't be reordered before
        // the pointer storing.
        atomic::fence(atomic::Ordering::Release);
        log::info!("111");
        let old = core::ptr::replace(self as *const _ as *mut _, src);
        log::info!("222");
        old.rcu_defer_reclaim();
        log::info!("333");
    }

    fn rcu_defer_reclaim(self) {
        // if !core::mem::needs_drop::<T>() {
        //     return;
        // }
        log::info!("444");
        unsafe { RCU_MANAGER.add(self.rcu_transmute()) }
        log::info!("555");
    }

    unsafe fn to_raw_ptr(self) -> usize {
        let ptr = transmute_copy::<Self, usize>(&self);
        // We will defer dropping the object.
        core::mem::forget(self);
        ptr
    }

    unsafe fn from_raw_ptr(ptr: usize) -> Self {
        transmute_copy::<usize, Self>(&ptr)
    }

    fn gen_drop_dtor() -> unsafe fn(usize) {
        |a| unsafe { core::mem::drop(Self::from_raw_ptr(a)) }
    }

    unsafe fn rcu_transmute(self) -> RcuDeferredDrop {
        RcuDeferredDrop {
            ptr: self.to_raw_ptr(),
            dtor: Self::gen_drop_dtor(),
        }
    }
}

struct RcuDeferredDrop {
    ptr: usize,
    dtor: unsafe fn(usize),
}

impl Drop for RcuDeferredDrop {
    fn drop(&mut self) {
        panic!("Should not invoke this drop method!")
    }
}

impl RcuDeferredDrop {
    #[inline(always)]
    pub unsafe fn rcu_drop(self) {
        (self.dtor)(self.ptr);
        core::mem::forget(self);
    }
}

struct CPReclaimedQueues {
    current: Vec<RcuDeferredDrop>,
    pending: Vec<RcuDeferredDrop>,
}

struct RcuMananger {
    qs_bitmap: AtomicU64,
    cp_queues: SpinNoIrqLock<CPReclaimedQueues>,
}

impl RcuMananger {
    fn add(&self, handler: RcuDeferredDrop) {
        self.cp_queues.lock().pending.push(handler);
        log::info!(
            "RCU: add a handler, queue len {}",
            self.cp_queues.lock().pending.len()
        );
    }

    pub fn critical_start(&self, cpuid: usize) {
        let pending_mask = 1 << cpuid;
        self.qs_bitmap.fetch_or(pending_mask, Ordering::Relaxed);
    }

    pub fn critical_end(&self, cpuid: usize) {
        let pending_mask = 1 << cpuid;
        let current_mask = 1 << (32 + cpuid);
        let both_mask = pending_mask | current_mask;

        let mut all_leave;
        let mut has_work;
        let mut old_bm = self.qs_bitmap.load(Ordering::Relaxed);

        loop {
            let mut new_bm = old_bm & !both_mask;
            all_leave = (new_bm >> 32) == 0;

            let cp = unsafe { self.cp_queues.unsafe_get() };
            has_work = !cp.current.is_empty() || !cp.pending.is_empty();

            if all_leave {
                // Now we're the last one to leave.
                // Let pending masks become current masks,
                // i.e. start a new grace period.
                new_bm <<= 32;
                if has_work {
                    // In order that no other cpus are doing reclaiming
                    // work at the same time.
                    new_bm |= current_mask;
                }
            }

            match self.qs_bitmap.compare_exchange(
                old_bm,
                new_bm,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(v) => old_bm = v,
            };
        }
        if !all_leave {
            return;
        }
        if !has_work {
            return;
        }
        log::info!("RCU: reclaiming work...");
        // log::info!("RCU: reclaiming work...");
        let mut cp = self.cp_queues.lock();
        let mut old_current = Vec::new();
        unsafe {
            core::ptr::swap(&mut cp.current, &mut old_current);
            core::ptr::swap(&mut cp.pending, &mut cp.current);
        }

        // Release bitmap lock and queues lock
        drop(cp);
        self.qs_bitmap.fetch_add(!current_mask, Ordering::Relaxed);

        for handler in old_current.drain(..) {
            unsafe {
                handler.rcu_drop();
            }
        }
        log::info!("RCU: reclaiming work done");
    }
}

static RCU_MANAGER: RcuMananger = RcuMananger {
    qs_bitmap: AtomicU64::new(0),
    cp_queues: SpinNoIrqLock::new(CPReclaimedQueues {
        current: Vec::new(),
        pending: Vec::new(),
    }),
};

pub fn critical_start(cpuid: usize) {
    RCU_MANAGER.critical_start(cpuid);
}

pub fn critical_end(cpuid: usize) {
    RCU_MANAGER.critical_end(cpuid);
}

// pub fn synchronize_rcu() {
//     RCU_MANAGER.synchronize_rcu();
// }
