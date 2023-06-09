//! The global allocator
use crate::config::mm::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;
use log::{debug, info, error};

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    error!("heap alloc err!!");
    panic!("Heap allocation error, layout = {:?}", layout);
}
/// heap space ([u8; KERNEL_HEAP_SIZE])
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
/// initiate heap allocator
pub fn init_heap() {
    unsafe {
        let start = HEAP_SPACE.as_ptr() as usize;
        HEAP_ALLOCATOR.lock().init(start, KERNEL_HEAP_SIZE / 2);
        // HEAP_ALLOCATOR
        //     .lock()
        //     .add_to_heap(start + KERNEL_HEAP_SIZE / 2, start + KERNEL_HEAP_SIZE);
        debug!(
            "kernel heap start {:#x}, mid {:#x}, end {:#x}",
            start as usize,
            start + KERNEL_HEAP_SIZE / 2,
            start + KERNEL_HEAP_SIZE
        );
    }
}

// pub fn init_heap() {
//     unsafe {
//         HEAP_ALLOCATOR
//             .lock()
//             .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
//     }
// }

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
