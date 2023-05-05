/// data structure
// pub const RADIX_TREE_MAP_SHIFT: usize = 6;
/// page cache: level num = 3, shift = 4, i.e. there are 2^12 pages(i.e. 16M)
pub const RADIX_TREE_MAP_SHIFT: usize = 4;

pub const FILE_PAGE_SIZE: usize = 0x1000;

/// max num file descriptors
pub const RLIMIT_NOFILE: usize = 128;
