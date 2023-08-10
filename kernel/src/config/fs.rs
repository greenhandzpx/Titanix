use super::mm::PAGE_SIZE;

/// data structure
// pub const RADIX_TREE_MAP_SHIFT: usize = 6;
/// page cache: level num = 3, shift = 4, i.e. there are 2^12 pages(i.e. 16M)
pub const RADIX_TREE_MAP_SHIFT: usize = 4;

// pub const FILE_PAGE_SIZE: usize = 0x1000;

/// max num file descriptors
pub const MAX_FD_NUM: usize = 1024;

// pub const PIPE_BUF_CAPACITY: usize = PAGE_SIZE;
pub const PIPE_BUF_CAPACITY: usize = 16 * PAGE_SIZE;
// pub const PIPE_BUF_CAPACITY: usize = 4 * PAGE_SIZE;
