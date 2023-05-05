use core::mem::size_of;

use easy_fs::BLOCK_SZ;

pub const KSTAT_SIZE: usize = size_of::<KSTAT>() as usize;

#[repr(C)]
pub struct KSTAT {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    _pad: u64,
    pub st_size: u64,
    pub st_blsize: u32,
    _pad2: u32,
    pub st_blocks: u64,
    pub st_atime_sec: i64,
    pub st_atime_nsec: i64,
    pub st_mtime_sec: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime_sec: i64,
    pub st_ctime_nsec: i64,
    _unused: [u32; 2],
}

impl KSTAT {
    pub fn new() -> Self {
        KSTAT {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 1,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            _pad: 0,
            st_size: 0,
            st_blsize: BLOCK_SZ as u32,
            _pad2: 0,
            st_blocks: 0,
            st_atime_sec: 0,
            st_atime_nsec: 0,
            st_mtime_sec: 0,
            st_mtime_nsec: 0,
            st_ctime_sec: 0,
            st_ctime_nsec: 0,
            _unused: [0; 2],
        }
    }
}
