/// boot
pub const HART_START_ADDR: usize = 0x80200000;

pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
// pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;
pub const KERNEL_HEAP_SIZE: usize = 0x600_0000;

pub const PAGE_CACHE_LEVEL_NUM: usize = 3;

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const PAGE_TABLE_LEVEL_NUM: usize = 3;
/// When directly map: vpn = ppn + kernel direct offset
// pub const KERNEL_DIRECT_OFFSET: usize = 0x7f_00000;
pub const KERNEL_DIRECT_OFFSET: usize = 0x0;

pub const USER_SPACE_SIZE: usize = 0x80000000;

/// Mmap area toppest address
pub const MMAP_TOP: usize = USER_SPACE_SIZE;
