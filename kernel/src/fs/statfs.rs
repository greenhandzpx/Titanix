use alloc::string::{String, ToString};

use super::fat32::SECTOR_SIZE;

pub const STATFS_SIZE: usize = core::mem::size_of::<Statfs>();
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
