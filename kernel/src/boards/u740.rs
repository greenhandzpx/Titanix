use crate::config::mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS};

pub const CLOCK_FREQ: usize = 1000000;
// pub const MEMORY_END: usize = 0x81000000;
pub const MEMORY_END: usize = (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS) + 0x88000000;

pub const MMIO: &[(usize, usize)] = &[
    (0x0010001000, 0x1000), // VIRT_TEST/RTC  in virt machine
];
