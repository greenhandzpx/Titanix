use alloc::{boxed::Box, sync::Arc};
use log::{debug, info, warn};

use crate::{
    config::{mm::PAGE_SIZE, mm::USER_STACK_SIZE},
    mm::{memory_space::UStackPageFaultHandler, user_check::UserCheck, MapPermission, VirtAddr},
    processor::SumGuard,
    syscall::futex_wake,
};

use super::Thread;

/// Get ustack bottom by the given tid
fn get_ustack_by_tid(ustack_base: usize, tid: usize) -> usize {
    ustack_base + tid * (USER_STACK_SIZE + PAGE_SIZE)
}

impl Thread {
    ///
    pub fn ustack_top(&self) -> usize {
        let inner = unsafe { &mut (*self.inner.get()) };
        get_ustack_by_tid(inner.ustack_base, self.tid.0) + USER_STACK_SIZE
    }
}

impl Thread {
    /// Note that this method must be called by the process that owns this thread
    pub fn alloc_ustack(&self) {
        let inner = unsafe { &mut (*self.inner.get()) };
        let ustack_bottom = get_ustack_by_tid(inner.ustack_base, self.tid.0);
        info!(
            "ustack: {:#x}, {:#x}",
            ustack_bottom,
            ustack_bottom + USER_STACK_SIZE
        );
        // self.process.inner.lock().memory_space.insert_framed_area(
        //     ustack_bottom.into(),
        //     (ustack_bottom + USER_STACK_SIZE).into(),
        //     MapPermission::R | MapPermission::W | MapPermission::U,
        // );

        self.process.inner_handler(move |proc| {
            if proc
                .memory_space
                .find_vm_area_by_vpn(VirtAddr::from(ustack_bottom).floor())
                .is_some()
            {
                debug!("ustack {:#x} has been added to memory set", ustack_bottom);
                return;
            }
            proc.memory_space.insert_framed_area_lazily(
                ustack_bottom.into(),
                (ustack_bottom + USER_STACK_SIZE).into(),
                MapPermission::R | MapPermission::W | MapPermission::U,
                Some(Arc::new(UStackPageFaultHandler {})),
            );
            proc.memory_space.activate();
        });
        // self.process.inner_handler(move |proc| {
        //     if proc
        //         .memory_space
        //         .find_vm_area_by_vpn(VirtAddr::from(ustack_bottom).floor())
        //         .is_some()
        //     {
        //         debug!("ustack {:#x} has been added to memory set", ustack_bottom);
        //         return;
        //     }
        //     proc.memory_space.insert_framed_area(
        //         ustack_bottom.into(),
        //         (ustack_bottom + USER_STACK_SIZE).into(),
        //         MapPermission::R | MapPermission::W | MapPermission::U,
        //     );
        // });

        // self.process
        //     .inner
        //     .lock()
        //     .memory_space
        //     .insert_framed_area_lazily(
        //         ustack_bottom.into(),
        //         (ustack_bottom + USER_STACK_SIZE).into(),
        //         MapPermission::R | MapPermission::W | MapPermission::U,
        //         Some(Box::new(UStackPageFaultHandler {})),
        //     );
    }

    ///
    pub fn dealloc_ustack(&self) {
        debug!("dealloc ustack");
        let inner = unsafe { &mut (*self.inner.get()) };
        let ustack_bottom = get_ustack_by_tid(inner.ustack_base, self.tid.0);
        self.process
            .inner
            .lock()
            .memory_space
            .remove_area_with_start_vpn(ustack_bottom.into());
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        debug!("drop a thread(tid {})", self.tid.0);
        // if !self.user_specified_stack {
        self.dealloc_ustack();
        // }
    }
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
