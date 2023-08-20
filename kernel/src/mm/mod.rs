//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_space to control its virtual memory.
mod address;

mod buf;

mod allocator;
///
pub mod memory_space;
mod page;
// mod page_cache;
mod page_table;
mod shm;
pub use shm::SHARED_MEMORY_MANAGER;
///
pub mod user_check;

// use address::StepByOne;
pub use address::VPNRange;
pub use address::{
    KernelAddr, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum, VA_WIDTH_SV39,
};
pub use allocator::frame_allocator::{
    frame_alloc, frame_alloc_contig, frame_dealloc, FrameTracker,
};
use log::info;
pub use memory_space::remap_test;
pub use memory_space::{MapPermission, MemorySpace, KERNEL_SPACE};
pub use page::{Page, PageBuilder};
// pub use page_cache::page_cache_test;
// pub use page_cache::PageCache;
pub use allocator::recycle_allocator::RecycleAllocator;
pub use page_table::PageTable;
pub use page_table::PageTableEntry;

use crate::processor::hart::HARTS;
use crate::stack_trace;

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
        stack_trace!();
    allocator::heap_allocator::init_heap();
    allocator::heap_allocator::heap_test();
    unsafe {
        for hart in HARTS.iter_mut() {
            hart.init_local_ctx();
        }
    }
    allocator::frame_allocator::init_frame_allocator();
    memory_space::init_kernel_space();
    info!("KERNEL SPACE init finish1");
    unsafe {
        KERNEL_SPACE
            .as_ref()
            .expect("KERNEL SPACE not init yet")
            .activate();
    }
    info!("KERNEL SPACE init finish2");
}
