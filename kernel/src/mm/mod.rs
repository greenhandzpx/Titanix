//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_space to control its virtual memory.
mod address;
mod frame_allocator;
pub mod heap_allocator;
///
pub mod memory_space;
mod page;
// mod page_cache;
mod page_table;
mod recycle_allocator;
mod shm;
pub use shm::SHARED_MEMORY_MANAGER;
///
pub mod user_check;

// use address::StepByOne;
pub use address::VPNRange;
pub use address::{KernelAddr, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
use log::info;
pub use memory_space::remap_test;
pub use memory_space::{MapPermission, MemorySpace, KERNEL_SPACE};
pub use page::{Page, PageBuilder};
// pub use page_cache::page_cache_test;
// pub use page_cache::PageCache;
pub use page_table::PageTable;
pub use page_table::PageTableEntry;
pub use recycle_allocator::RecycleAllocator;

use crate::processor::hart::HARTS;

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    unsafe {
        for hart in HARTS.iter_mut() {
            hart.init_local_ctx();
        }
    }
    frame_allocator::init_frame_allocator();
    memory_space::init_kernel_space();
    unsafe {
        KERNEL_SPACE
            .as_ref()
            .expect("KERNEL SPACE not init yet")
            .activate();
    }
    info!("KERNEL SPACE init finished");
    // KERNEL_SPACE.exclusive_access().activate();
}
