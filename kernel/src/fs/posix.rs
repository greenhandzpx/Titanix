use core::mem::size_of;

use alloc::string::{String, ToString};
use easy_fs::BLOCK_SZ;

use super::fat32::SECTOR_SIZE;
use crate::timer::{current_time_duration, TimeSpec};

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

pub const STATFS_SIZE: usize = size_of::<Statfs>();
pub struct Fsid_t {
    val: [i32; 2],
}

pub struct Statfs {
    /// Type of filesystem.
    f_type: i64,
    /// Optimal transfer block size.
    f_bsize: i64,
    /// Total data blocks in filesystem.
    f_blocks: u64,
    /// Free blocks in filesystem.
    f_bfree: u64,
    /// Free blocks available to unprivileged user.
    f_bavail: u64,
    /// Total inodes in filesystem.
    f_files: u64,
    /// Free inodes in filesystem.
    f_ffree: u64,
    /// Filesystem ID.
    f_fsid: Fsid_t,
    /// Maximum length of filenames.
    f_namelen: i64,
    /// Fragment size (since Linux 2.6).
    f_frsize: i64,
    /// Mount flags of filesystem (since Linux 2.6.36).
    f_flags: i64,
    /// Padding bytes rese=rved for future use.
    f_spare: [i64; 4],
}

impl Statfs {
    pub fn new() -> Self {
        Statfs {
            f_type: Magic::ExfatSuperMagic as i64,
            f_bsize: SECTOR_SIZE as i64,
            f_blocks: 1 << 27,
            f_bfree: 1 << 26,
            f_bavail: 1 << 20,
            f_files: 1 << 10,
            f_ffree: 1 << 9,
            f_fsid: Fsid_t { val: [0; 2] },
            f_namelen: 1 << 8,
            f_frsize: 1 << 9,
            f_flags: StatFlags::bits(&StatFlags::ST_NOSUID) as i64,
            f_spare: [0; 4],
        }
    }
}

bitflags! {
    pub struct StatFlags: u32 {
        /// This filesystem is mounted read-only.
        const ST_RDONLY = 1;
        /// The set-user-ID and set-group-ID bits are ignored by exec(3) for executable files on this filesystem.
        const ST_NOSUID = 1 << 1;
        /// Disallow access to device special files on this filesystem.
        const ST_NODEV = 1 << 2;
        /// Execution of programs is disallowed on this filesystem.
        const ST_NOEXEC = 1 << 3;
        /// Writes are synched to the filesystem immediately (see the description of O_SYNC in open(2)).
        const ST_SYNCHRONOUS = 1 << 4;
        const ST_VAILD = 1 << 5;
        /// Mandatory locking is permitted on the filesystem.
        const ST_MANDLOCK = 1 << 6;
        /// Do not update access times.
        const ST_NOATIME = 1 << 10;
        /// Do not update directory access times.
        const ST_NODIRATIME = 1 << 11;
        /// Update atime relative to mtime/ctime.
        const ST_RELATIME = 1 << 12;
        /// Symbolic links are not followed when resolving paths; see mount(2).
        const ST_NOSYMFOLLOW = 1 << 13;
    }
}

impl StatFlags {
    pub fn to_string(&self) -> String {
        let mut res = "".to_string();
        if self.contains(StatFlags::ST_RDONLY) {
            res += "ro";
        } else {
            res += "rw";
        }
        if self.contains(StatFlags::ST_NOSUID) {
            res += ",nosuid";
        }
        if self.contains(StatFlags::ST_NODEV) {
            res += ",nodev";
        }
        if self.contains(StatFlags::ST_NOEXEC) {
            res += ",noexec";
        }
        if self.contains(StatFlags::ST_RELATIME) {
            res += ",relatime";
        }
        res
    }
}

pub enum Magic {
    ExfatSuperMagic = 0x2011BAB0,
}

pub const SYSINFO_SIZE: usize = size_of::<Sysinfo>();

const _F_SIZE: usize = 20 - 2 * size_of::<u64>() - size_of::<u32>();
#[repr(C)]
pub struct Sysinfo {
    /// Seconds since boot
    pub uptime: i64,
    /// 1, 5, and 15 minute load averages
    pub loads: [u64; 3],
    /// Total usable main memory size
    pub totalram: u64,
    /// Available memory size
    pub freeram: u64,
    /// Amount of shared memory
    pub sharedram: u64,
    /// Memory used by buffers
    pub bufferram: u64,
    /// Total swap space size
    pub totalswap: u64,
    /// swap space still available
    pub freeswap: u64,
    /// Number of current processes
    pub procs: u16,
    /// Explicit padding for m68k
    pub pad: u16,
    /// Total high memory size
    pub totalhigh: u64,
    /// Available high memory size
    pub freehigh: u64,
    /// Memory unit size in bytes
    pub mem_uint: u32,
    /// Padding: libc5 uses this..
    pub _f: [u8; _F_SIZE],
}

impl Sysinfo {
    pub fn collect() -> Self {
        // Self {
        //     uptime: current_time_duration().as_secs() as i64,
        //     loads: (),
        //     totalram: (),
        //     freeram: (),
        //     sharedram: (),
        //     bufferram: (),
        //     totalswap: (),
        //     freeswap: (),
        //     procs: (),
        //     pad: (),
        //     totalhigh: (),
        //     freehigh: (),
        //     mem_uint: (),
        //     _f: [0; _F_SIZE],
        // }
        todo!()
    }
}
