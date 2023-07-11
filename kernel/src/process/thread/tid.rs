//!Implementation of [`TidAllocator`]
use crate::config::process::INITPROC_PID;
use crate::mm::user_check::UserCheck;
use crate::mm::RecycleAllocator;
use crate::processor::SumGuard;
use crate::sync::mutex::SpinNoIrqLock;
use crate::syscall::futex_wake;
use lazy_static::*;
use log::{debug, warn};

lazy_static! {
    static ref TID_ALLOCATOR: SpinNoIrqLock<RecycleAllocator> =
        SpinNoIrqLock::new(RecycleAllocator::new(INITPROC_PID));
}
///Bind pid lifetime to `TidHandle`
pub struct TidHandle(pub usize);

impl Drop for TidHandle {
    fn drop(&mut self) {
        debug!("drop pid {}", self.0);
        // println!("\u{1B}[33m drop pid {} \u{1B}[0m", self.0);
        TID_ALLOCATOR.lock().dealloc(self.0);
    }
}
///Allocate a pid from PID_ALLOCATOR
pub fn tid_alloc() -> TidHandle {
    TidHandle(TID_ALLOCATOR.lock().alloc())
}

/// Tid address which may be set by `set_tid_address` syscall
pub struct TidAddress {
    /// Address
    pub addr: usize,
}

impl Drop for TidAddress {
    fn drop(&mut self) {
        debug!("Drop tid address {:#x}", self.addr);
        if UserCheck::new()
            .check_writable_slice(self.addr as *mut u8, core::mem::size_of::<usize>())
            .is_ok()
        {
            let _sum_guard = SumGuard::new();
            unsafe {
                *(self.addr as *mut usize) = 0;
            }
        }
        if futex_wake(self.addr, 1).is_err() {
            warn!("futex wake failed when thread died");
        }
    }
}
