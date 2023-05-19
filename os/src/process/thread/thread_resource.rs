use alloc::boxed::Box;
use log::{debug, info};

use crate::{
    config::{mm::PAGE_SIZE, mm::USER_STACK_SIZE},
    mm::{memory_set::UStackPageFaultHandler, MapPermission, VirtAddr},
};

use super::Thread;

/// Get ustack bottom by the given tid
fn get_ustack_by_tid(ustack_bottom: usize, tid: usize) -> usize {
    ustack_bottom + tid * (USER_STACK_SIZE + PAGE_SIZE)
}

impl Thread {
    ///
    pub fn ustack_top(&self) -> usize {
        let inner = unsafe { &mut (*self.inner.get()) };
        get_ustack_by_tid(inner.ustack_base, self.tid.0) + USER_STACK_SIZE
    }
}

impl Thread {
    ///
    pub fn alloc_ustack(&self) {
        let inner = unsafe { &mut (*self.inner.get()) };
        let ustack_bottom = get_ustack_by_tid(inner.ustack_base, self.tid.0);
        debug!(
            "ustack: {:#x}, {:#x}",
            ustack_bottom,
            ustack_bottom + USER_STACK_SIZE
        );
        // self.process.inner.lock().memory_set.insert_framed_area(
        //     ustack_bottom.into(),
        //     (ustack_bottom + USER_STACK_SIZE).into(),
        //     MapPermission::R | MapPermission::W | MapPermission::U,
        // );

        self.process.inner_handler(move |proc| {
            if proc
                .memory_set
                .find_vm_area_by_vpn(VirtAddr::from(ustack_bottom).floor())
                .is_some()
            {
                debug!("ustack {:#x} has been added to memory set", ustack_bottom);
                return;
            }
            proc.memory_set.insert_framed_area_lazily(
                ustack_bottom.into(),
                (ustack_bottom + USER_STACK_SIZE).into(),
                MapPermission::R | MapPermission::W | MapPermission::U,
                Some(Box::new(UStackPageFaultHandler {})),
            );
            proc.memory_set.activate();
        });
        // self.process.inner_handler(move |proc| {
        //     if proc
        //         .memory_set
        //         .find_vm_area_by_vpn(VirtAddr::from(ustack_bottom).floor())
        //         .is_some()
        //     {
        //         debug!("ustack {:#x} has been added to memory set", ustack_bottom);
        //         return;
        //     }
        //     proc.memory_set.insert_framed_area(
        //         ustack_bottom.into(),
        //         (ustack_bottom + USER_STACK_SIZE).into(),
        //         MapPermission::R | MapPermission::W | MapPermission::U,
        //     );
        // });

        // self.process
        //     .inner
        //     .lock()
        //     .memory_set
        //     .insert_framed_area_lazily(
        //         ustack_bottom.into(),
        //         (ustack_bottom + USER_STACK_SIZE).into(),
        //         MapPermission::R | MapPermission::W | MapPermission::U,
        //         Some(Box::new(UStackPageFaultHandler {})),
        //     );
    }

    ///
    pub fn alloc_tid(&mut self) {
        self.tid = self.process.alloc_tid();
    }

    ///
    pub fn dealloc_ustack(&self) {
        debug!("dealloc ustack");
        let inner = unsafe { &mut (*self.inner.get()) };
        let ustack_bottom = get_ustack_by_tid(inner.ustack_base, self.tid.0);
        self.process
            .inner
            .lock()
            .memory_set
            .remove_area_with_start_vpn(ustack_bottom.into());
    }

    ///
    pub fn dealloc_tid(&self) {
        self.process.dealloc_tid(self.tid.0);
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        debug!("drop a thread(tid {})", self.tid.0);
        // if !self.user_specified_stack {
        self.dealloc_ustack();
        // }
        self.dealloc_tid();
    }
}
