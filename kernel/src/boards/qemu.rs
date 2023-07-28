#![allow(dead_code)]
use crate::{
    config::mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS},
    mm::MapPermission,
};

pub const CLOCK_FREQ: usize = 10000000;
pub const MEMORY_END: usize = (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS) + 0x88000000;

pub const PERMISSION_RW: MapPermission = MapPermission::union(MapPermission::R, MapPermission::W);

pub const MMIO: &[(usize, usize, MapPermission)] = &[
    (0x10000000, 0x1000, PERMISSION_RW),   // UART
    (0x10001000, 0x1000, PERMISSION_RW),   // VIRTIO
    (0x02000000, 0x10000, PERMISSION_RW),  // CLINT
    (0x0C000000, 0x400000, PERMISSION_RW), // PLIC
];
