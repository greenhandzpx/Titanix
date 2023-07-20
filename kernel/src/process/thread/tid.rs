//!Implementation of [`TidAllocator`]
use crate::mm::user_check::UserCheck;
use crate::mm::RecycleAllocator;
use crate::processor::SumGuard;
use crate::stack_trace;
use crate::sync::mutex::SpinNoIrqLock;
use crate::{config::process::INITPROC_PID, sync::futex_wake};
use log::{debug, warn};

static TID_ALLOCATOR: SpinNoIrqLock<RecycleAllocator> =
    SpinNoIrqLock::new(RecycleAllocator::new(INITPROC_PID));
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
    /// Set tid address
    pub set_tid_address: Option<usize>,
    /// Clear tid address
    pub clear_tid_address: Option<usize>,
}

impl TidAddress {
    ///
    pub fn new() -> Self {
        Self {
            set_tid_address: None,
            clear_tid_address: None,
        }
    }

    ///
    pub fn thread_died(&self) {
        stack_trace!();
        if let Some(clear_tid_address) = self.clear_tid_address {
            log::info!("Drop tid address {:#x}", clear_tid_address);
            if UserCheck::new()
                .check_writable_slice(clear_tid_address as *mut u8, core::mem::size_of::<usize>())
                .is_ok()
            {
                let _sum_guard = SumGuard::new();
                unsafe {
                    *(clear_tid_address as *mut usize) = 0;
                }
            }
            if futex_wake(clear_tid_address, 1).is_err() {
                warn!("futex wake failed when thread died");
            }
        }
    }
}

// impl Drop for TidAddress {
//     fn drop(&mut self) {
//     }
// }
