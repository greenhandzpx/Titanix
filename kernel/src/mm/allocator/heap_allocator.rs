//! The global allocator
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

use crate::{config::mm::KERNEL_HEAP_SIZE, sync::mutex::SpinNoIrqLock};
use buddy_system_allocator::{Heap, LockedHeap};
use log::{debug, error, info};

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: GlobalHeap = GlobalHeap::empty();

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    error!("heap alloc err!!");
    panic!("Heap allocation error, layout = {:?}", layout);
}

struct GlobalHeap(SpinNoIrqLock<Heap>);

impl GlobalHeap {
    const fn empty() -> Self {
        Self(SpinNoIrqLock::new(Heap::empty()))
    }
}

unsafe impl GlobalAlloc for GlobalHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(NonNull::new_unchecked(ptr), layout)
    }
}

/// heap space ([u8; KERNEL_HEAP_SIZE])
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
/// initiate heap allocator
pub fn init_heap() {
    unsafe {
        let start = HEAP_SPACE.as_ptr() as usize;
        HEAP_ALLOCATOR.0.lock().init(start, KERNEL_HEAP_SIZE);
        debug!(
            "[kernel] heap start {:#x}, end {:#x}",
            start as usize,
            start + KERNEL_HEAP_SIZE
        );
    }
}

///
#[allow(unused)]
pub fn heap_test() {
    info!("heap_test start...");
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    let max_len = (KERNEL_HEAP_SIZE - 10000) / core::mem::size_of::<usize>();
    for i in 0..500 {
        v.push(i);
    }
    for (i, val) in v.iter().take(500).enumerate() {
        assert_eq!(*val, i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    info!("heap_test passed!");
}
