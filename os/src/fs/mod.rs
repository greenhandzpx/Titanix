mod devfs;
pub mod dirent;
pub mod fat32;
mod fd_table;
mod file;
mod file_system;
mod hash_name;
pub mod inode;
pub mod stat;
pub mod utsname;
// pub mod inode_fat32_tmp;
pub mod fat32_tmp;
pub mod pipe;
mod procfs;
mod stdio;
mod testfs;
mod uio;

// pub use dentry::Dentry;
pub use dirent::Dirent;
pub use dirent::DIRENT_SIZE;
pub use fat32::FAT32FileSystem;
pub use fd_table::FdTable;
pub use file::File;
pub use file_system::FileSystem;
pub use file_system::FileSystemType;
pub use file_system::FILE_SYSTEM_MANAGER;
pub use inode::Inode;
pub use inode::InodeMode;
pub use inode::InodeState;
pub use stdio::Stdin;
pub use stdio::Stdout;
pub use uio::*;
pub use utsname::UtsName;
pub use utsname::UTSNAME_SIZE;

use crate::mm::MapPermission;
use crate::sync::mutex::SpinNoIrqLock;

type Mutex<T> = SpinNoIrqLock<T>;

pub fn init() {
    fat32_tmp::init().expect("fat32 init fail");
    // // first mount root fs
    // testfs::init().expect("testfs init fail");
    // todo!();
    devfs::init().expect("devfs init fail");
}

bitflags! {
    /// Open file flags
    pub struct OpenFlags: u32 {
        const APPEND = 1 << 10;
        const ASYNC = 1 << 13;
        const DIRECT = 1 << 14;
        const DSYNC = 1 << 12;
        const EXCL = 1 << 7;
        const NOATIME = 1 << 18;
        const NOCTTY = 1 << 8;
        const NOFOLLOW = 1 << 17;
        const PATH = 1 << 21;
        /// TODO: need to find 1 << 15
        const TEMP = 1 << 15;
        /// Read only
        const RDONLY = 0;
        /// Write only
        const WRONLY = 1 << 0;
        /// Read & Write
        const RDWR = 1 << 1;
        /// Allow create
        const CREATE = 1 << 6;
        /// Clear file and return an empty one
        const TRUNC = 1 << 9;
        /// Directory
        const DIRECTORY = 1 << 16;
        /// Enable the close-on-exec flag for the new file descriptor
        const CLOEXEC = 1 << 19;
        /// When possible, the file is opened in nonblocking mode
        const NONBLOCK = 1 << 11;
    }

    /// stat flags
    pub struct StatFlags: u32 {
        const AT_EMPTY_PATH = 1 << 0;
        const AT_NO_AUTOMOUNT = 1 << 11;
        const AT_SYMLINK_NOFOLLOW = 1 << 8;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

impl From<MapPermission> for OpenFlags {
    fn from(perm: MapPermission) -> Self {
        let mut res = OpenFlags::from_bits(0).unwrap();
        if perm.contains(MapPermission::R) && perm.contains(MapPermission::W) {
            res |= OpenFlags::RDWR;
        } else if perm.contains(MapPermission::R) {
            res |= OpenFlags::RDONLY;
        } else if perm.contains(MapPermission::W) {
            res |= OpenFlags::WRONLY;
        }
        res
    }
}
