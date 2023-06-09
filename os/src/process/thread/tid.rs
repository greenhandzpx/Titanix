use log::{debug, warn};

use crate::{
    mm::user_check::UserCheck,
    processor::{current_process, SumGuard},
    syscall::futex_wake,
};

/// Thread id
pub struct TidHandle(pub usize);

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
