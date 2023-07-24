use crate::config::mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS};

pub const CLOCK_FREQ: usize = 1000000;
// pub const MEMORY_END: usize = 0x81000000;
pub const MEMORY_END: usize = (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS) + 0x88000000;

pub const MMIO: &[(usize, usize)] = &[
    (0x00000000, 0x1000),     // Debug
    (0x00001000, 0x1000),     // Rom
    (0x00004000, 0x1000),     // Test Status
    (0x00006000, 0x1000),     // Chip Select
    (0x00010000, 0x8000),     // Rom
    (0x01000000, 0x2000),     // S7 DTIM (8KiB)
    (0x01700000, 0x1000),     // S7 Hart 0 Bus Error Unit
    (0x01701000, 0x1000),     // U74 Hart 1 Bus Error Unit
    (0x01702000, 0x1000),     // U74 Hart 2 Bus Error Unit
    (0x01703000, 0x1000),     // U74 Hart 3 Bus Error Unit
    (0x01704000, 0x1000),     // U74 Hart 4 Bus Error Unit
    (0x02000000, 0x10000),    // CLINT
    (0x02010000, 0x1000),     // L2 Cache Controller
    (0x02020000, 0x1000),     // MSI
    (0x03000000, 0x100000),   // DMA
    (0x08000000, 0x200000),   // L2 Cache Controller
    (0x09000000, 0x200000),   // Rom
    (0x0A000000, 0x2000000),  // Rom
    (0x0C000000, 0x4000000),  // PLIC
    (0x10000000, 0x1000),     // PRCI
    (0x10010000, 0x1000),     // UART 0
    (0x10011000, 0x1000),     // UART 1
    (0x10020000, 0x1000),     // PWM 0
    (0x10021000, 0x1000),     // PWM 1
    (0x10030000, 0x1000),     // I2C 0
    (0x10031000, 0x1000),     // I2C 1
    (0x10040000, 0x1000),     // QSPI 0
    (0x10041000, 0x1000),     // QSPI 1
    (0x10050000, 0x1000),     // QSPI 2
    (0x10060000, 0x1000),     // GPIO
    (0x10070000, 0x1000),     // OTP
    (0x10080000, 0x1000),     // Pin Control
    (0x10090000, 0x2000),     // Ethernet
    (0x100A0000, 0x1000),     // GEMGXL MGMT
    (0x100B0000, 0x4000),     // Memory Controller
    (0x100B8000, 0x1000),     // Physical Filter
    (0x100C0000, 0x1000),     // DDR MGMT
    (0x100D0000, 0x1000),     // PCIE MGMT
    (0x100E0000, 0x1000),     // Order Ogler
    (0x14000000, 0x4000000),  // Error Device 0
    (0x18000000, 0x8000000),  // Error Device 1
    (0x20000000, 0x10000000), // SPI 0
    (0x30000000, 0x10000000), // SPI 1
    (0x60000000, 0x20000000), // PCIe
];
