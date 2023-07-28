#![allow(dead_code)]
use crate::{
    config::mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS},
    mm::MapPermission,
};

pub const CLOCK_FREQ: usize = 1000000;
pub const MEMORY_END: usize = (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS) + 0x480000000;

pub const PERMISSION_NONE: MapPermission = MapPermission::empty();
pub const PERMISSION_RW: MapPermission = MapPermission::union(MapPermission::R, MapPermission::W);
pub const PERMISSION_RX: MapPermission = MapPermission::union(MapPermission::R, MapPermission::X);
pub const PERMISSION_RWX: MapPermission = MapPermission::union(PERMISSION_RW, MapPermission::X);

pub const MMIO: &[(usize, usize, MapPermission)] = &[
    (0x00000000, 0x1000, PERMISSION_NONE),   // Debug
    (0x00001000, 0x1000, PERMISSION_RX),     // Rom
    (0x00004000, 0x1000, PERMISSION_RW),     // Test Status
    (0x00006000, 0x1000, PERMISSION_RW),     // Chip Select
    (0x00010000, 0x8000, PERMISSION_RX),     // Rom
    (0x01000000, 0x2000, PERMISSION_RWX),    // S7 DTIM (8KiB)
    (0x01700000, 0x1000, PERMISSION_RW),     // S7 Hart 0 Bus Error Unit
    (0x01701000, 0x1000, PERMISSION_RW),     // U74 Hart 1 Bus Error Unit
    (0x01702000, 0x1000, PERMISSION_RW),     // U74 Hart 2 Bus Error Unit
    (0x01703000, 0x1000, PERMISSION_RW),     // U74 Hart 3 Bus Error Unit
    (0x01704000, 0x1000, PERMISSION_RW),     // U74 Hart 4 Bus Error Unit
    (0x02000000, 0x10000, PERMISSION_RW),    // CLINT
    (0x02010000, 0x1000, PERMISSION_RW),     // L2 Cache Controller
    (0x02020000, 0x1000, PERMISSION_RW),     // MSI
    (0x03000000, 0x100000, PERMISSION_RW),   // DMA
    (0x08000000, 0x200000, PERMISSION_RWX),  // L2 Cache Controller
    (0x09000000, 0x200000, PERMISSION_RWX),  // Rom
    (0x0A000000, 0x2000000, PERMISSION_RWX), // Rom
    (0x0C000000, 0x4000000, PERMISSION_RW),  // PLIC
    (0x10000000, 0x1000, PERMISSION_RW),     // PRCI
    (0x10010000, 0x1000, PERMISSION_RW),     // UART 0
    (0x10011000, 0x1000, PERMISSION_RW),     // UART 1
    (0x10020000, 0x1000, PERMISSION_RW),     // PWM 0
    (0x10021000, 0x1000, PERMISSION_RW),     // PWM 1
    (0x10030000, 0x1000, PERMISSION_RW),     // I2C 0
    (0x10031000, 0x1000, PERMISSION_RW),     // I2C 1
    (0x10040000, 0x1000, PERMISSION_RW),     // QSPI 0
    (0x10041000, 0x1000, PERMISSION_RW),     // QSPI 1
    (0x10050000, 0x1000, PERMISSION_RW),     // QSPI 2
    (0x10060000, 0x1000, PERMISSION_RW),     // GPIO
    (0x10070000, 0x1000, PERMISSION_RW),     // OTP
    (0x10080000, 0x1000, PERMISSION_RW),     // Pin Control
    (0x10090000, 0x2000, PERMISSION_RW),     // Ethernet
    (0x100A0000, 0x1000, PERMISSION_RW),     // GEMGXL MGMT
    (0x100B0000, 0x4000, PERMISSION_RW),     // Memory Controller
    (0x100B8000, 0x1000, PERMISSION_RW),     // Physical Filter
    (0x100C0000, 0x1000, PERMISSION_RW),     // DDR MGMT
    (0x100D0000, 0x1000, PERMISSION_RW),     // PCIE MGMT
    (0x100E0000, 0x1000, PERMISSION_RW),     // Order Ogler
    (0x14000000, 0x4000000, PERMISSION_RWX), // Error Device 0
    (0x18000000, 0x8000000, PERMISSION_RWX), // Error Device 1
    (0x20000000, 0x10000000, PERMISSION_RX), // SPI 0
    (0x30000000, 0x10000000, PERMISSION_RX), // SPI 1
    (0x60000000, 0x20000000, PERMISSION_RW), // PCIe
];
