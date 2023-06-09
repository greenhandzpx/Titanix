/*
 * @Author: greenhandzpx 893522573@qq.com
 * @Date: 2023-01-28 12:24:08
 * @LastEditors: greenhandzpx 893522573@qq.com
 * @LastEditTime: 2023-02-22 10:03:39
 * @FilePath: /oscomp-kernel/os/src/mm/mod.rs
 * @Description:
 *
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 */
//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.
mod address;
mod frame_allocator;
pub mod heap_allocator;
///
pub mod memory_set;
mod page;
mod page_cache;
mod page_table;
mod radix_tree;
mod recycle_allocator;
///
pub mod user_check;

// use address::StepByOne;
pub use address::VPNRange;
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
use log::info;
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page::Page;
pub use page_cache::page_cache_test;
pub use page_cache::PageCache;
pub use page_table::PageTable;
pub use page_table::PageTableEntry;
pub use recycle_allocator::RecycleAllocator;


/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    memory_set::init_kernel_space();
    unsafe {
        KERNEL_SPACE
            .as_ref()
            .expect("KERNEL SPACE not init yet")
            .activate();
    }
    info!("KERNEL SPACE init finished");
    // KERNEL_SPACE.exclusive_access().activate();
}
