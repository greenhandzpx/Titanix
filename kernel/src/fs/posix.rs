use core::mem::size_of;

use easy_fs::BLOCK_SZ;

use crate::timer::TimeSpec;

pub const STAT_SIZE: usize = size_of::<STAT>() as usize;

#[repr(C)]
pub struct STAT {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub st_size: u64,
    pub st_blksize: u32,
    pub st_blocks: u64,
    pub st_atim: TimeSpec,
    pub st_mtim: TimeSpec,
    pub st_ctim: TimeSpec,
}

impl STAT {
    pub fn new() -> Self {
        STAT {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 1,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: BLOCK_SZ as u32,
            st_blocks: 0,
            st_atim: TimeSpec::new(),
            st_mtim: TimeSpec::new(),
            st_ctim: TimeSpec::new(),
        }
    }
}

#[repr(C)]
pub struct Iovec {
    /// user space buf starting address
    pub iov_base: usize,
    /// number of bytes to transfer
    pub iov_len: usize,
}

unsafe impl Send for Iovec {}
unsafe impl Sync for Iovec {}


use crate::utils::string::str_to_array_65;

// const SYSNAME: &str = "Titanix";
// const NODENAME: &str = "Titanix";
// const RELEASE: &str = "Titanix 1.0.0";
// const VERSION: &str = "1.0.0";
const SYSNAME: &str = "Linux";
const NODENAME: &str = "Linux";
const RELEASE: &str = "5.19.0-42-generic";
// const VERSION: &str = "6.4.0";
const VERSION: &str = "#43~22.04.1-Ubuntu SMP PREEMPT_DYNAMIC Fri Apr 21 16:51:08 UTC 2";
const MACHINE: &str = "RISC-V SiFive Freedom U740 SoC";
const DOMAINNAME: &str = "titanix.org";

pub const UTSNAME_SIZE: usize = size_of::<UtsName>() as usize;

#[repr(C)]
pub struct UtsName {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}

impl UtsName {
    pub fn get_utsname() -> Self {
        UtsName {
            sysname: str_to_array_65(SYSNAME),
            nodename: str_to_array_65(NODENAME),
            release: str_to_array_65(RELEASE),
            version: str_to_array_65(VERSION),
            machine: str_to_array_65(MACHINE),
            domainname: str_to_array_65(DOMAINNAME),
        }
    }
}

