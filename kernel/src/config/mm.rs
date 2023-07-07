/// boot
pub const HART_START_ADDR: usize = 0x80200000;

pub const USER_STACK_SIZE: usize = 4096 * 8;

pub const _KERNEL_STACK_SIZE: usize = 4096 * 2;

// pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;
pub const KERNEL_HEAP_SIZE: usize = 0x200_0000;
// pub const KERNEL_HEAP_SIZE: usize = 0x400_0000;

pub const PAGE_CACHE_LEVEL_NUM: usize = 3;

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const PAGE_TABLE_LEVEL_NUM: usize = 3;

/// When directly map: vpn = ppn + kernel direct offset
// pub const KERNEL_DIRECT_OFFSET: usize = 0x7f_00000;
pub const KERNEL_DIRECT_OFFSET: usize = 0xffff_ffc0_0000_0;
// pub const KERNEL_DIRECT_OFFSET: usize = 0x0;

pub const USER_SPACE_SIZE: usize = 0x30_0000_0000;

/// Mmap area toppest address
pub const MMAP_TOP: usize = USER_SPACE_SIZE;

/// Dynamic linked interpreter address range in user space
pub const DL_INTERP_OFFSET: usize = 0x20_0000_0000;
