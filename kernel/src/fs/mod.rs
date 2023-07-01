mod devfs;
pub mod fat32;
mod fd_table;
mod file;
pub mod file_system;
mod hash_key;
pub mod inode;
// pub mod inode_fat32_tmp;
// pub mod fat32_tmp;
pub mod pipe;
pub mod posix;
mod procfs;
mod testfs;

use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
pub use fat32::FAT32FileSystem;
pub use fd_table::FdTable;
pub use file::File;
pub use file_system::FileSystem;
pub use file_system::FileSystemType;
pub use file_system::FILE_SYSTEM_MANAGER;
pub use hash_key::HashKey;
pub use inode::Inode;
pub use inode::InodeMode;
pub use inode::InodeState;
use log::debug;
use log::info;
use log::warn;

use crate::driver::BLOCK_DEVICE;
use crate::mm::MapPermission;
use crate::processor::current_process;
use crate::stack_trace;
use crate::sync::mutex::SpinNoIrqLock;
use crate::timer::posix::current_time_spec;
use crate::utils::error::SyscallErr;
use crate::utils::error::SyscallRet;
use crate::utils::path;

use self::file_system::FsDevice;
use self::inode::INODE_CACHE;
use self::posix::StatFlags;

type Mutex<T> = SpinNoIrqLock<T>;

pub fn init() {
    FILE_SYSTEM_MANAGER.mount(
        "/",
        "/dev/mmcblk0",
        file_system::FsDevice::BlockDevice(Arc::clone(&BLOCK_DEVICE)),
        FileSystemType::VFAT,
        StatFlags::ST_NOSUID
    ).expect("rootfs init fail!");
    // FILE_SYSTEM_MANAGER.mount("/", "/dev/vda2", FsDevice::None, FileSystemType::VFAT, StatFlags::ST_NOSUID);

    let root_inode = FILE_SYSTEM_MANAGER.root_inode();

    <dyn Inode>::load_children(Arc::clone(&root_inode));
    
    let dev_dir = root_inode.mkdir(
        Arc::clone(&root_inode),
        "/dev",
        InodeMode::FileDIR
    ).expect("mkdir /dev fail!");

    let key = HashKey::new(
        root_inode.metadata().ino,
        "dev".to_string()
    );
    INODE_CACHE.lock().insert(key, dev_dir);

    let proc_dir = root_inode.mkdir(
        Arc::clone(&root_inode),
        "/proc",
        InodeMode::FileDIR
    ).expect("mkdir /proc fail!");

    let key = HashKey::new(
        root_inode.metadata().ino,
        "proc".to_string()
    );
    INODE_CACHE.lock().insert(key, proc_dir);

    FILE_SYSTEM_MANAGER.mount(
        "/dev",
        "udev",
        FsDevice::None,
        FileSystemType::DevTmpFS,
        StatFlags::ST_NOSUID
    ).expect("devfs init fail!");

    FILE_SYSTEM_MANAGER.mount(
        "/proc",
        "proc",
        FsDevice::None,
        FileSystemType::Proc,
        StatFlags::ST_NOSUID
    ).expect("procfs init fail!");

    list_rootfs();
}
pub const AT_FDCWD: isize = -100;

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

    /// fcntl flags
    pub struct FcntlFlags: u32 {
        const AT_EMPTY_PATH = 1 << 0;
        const AT_NO_AUTOMOUNT = 1 << 11;
        const AT_SYMLINK_NOFOLLOW = 1 << 8;
        const AT_EACCESS = 1 << 9;
    }

    /// renameat flag
    pub struct Renameat2Flags: u32 {
        /// Go back to renameat
        const RENAME_NONE = 0;
        /// Atomically exchange oldpath and newpath.
        const RENAME_EXCHANGE = 1 << 1;
        /// Don't overwrite newpath of the rename. Return an error if newpath already exists.
        const RENAME_NOREPLACE = 1 << 0;
        /// This operation makes sense only for overlay/union filesystem implementations.
        const RENAME_WHITEOUT = 1 << 2;
    }

    /// faccessat flag
    pub struct FaccessatFlags: u32 {
        const F_OK = 0;
        const R_OK = 1 << 2;
        const W_OK = 1 << 1;
        const X_OK = 1 << 0;
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

#[allow(unused)]
pub fn print_dir_tree() {
    info!("------------ dir tree: ------------");
    let parent = Arc::clone(&FILE_SYSTEM_MANAGER.root_inode());
    print_dir_recursively(parent, 1);
}

fn print_dir_recursively(inode: Arc<dyn Inode>, level: usize) {
    let children = inode.metadata().inner.lock().children.clone();
    for child in children {
        for _ in 0..level {
            print!("-");
        }
        println!("{}", child.0);
        print_dir_recursively(child.1, level + 1);
    }
}

pub fn resolve_path(name: &str, flags: OpenFlags) -> Option<Arc<dyn Inode>> {
    debug!("[resolve_path]: name {}, flags {:?}", name, flags);
    let inode = <dyn Inode>::lookup_from_root_tmp(name);
    // inode
    if flags.contains(OpenFlags::CREATE) {
        if inode.is_some() {
            return inode;
        }
        let parent_path = path::get_parent_dir(name).unwrap();
        let parent = <dyn Inode>::lookup_from_root_tmp(&parent_path);
        let child_name = path::get_name(name);
        if let Some(parent) = parent {
            debug!("create file {}", name);
            if flags.contains(OpenFlags::DIRECTORY) {
                parent
                    .mkdir(parent.clone(), child_name, InodeMode::FileDIR)
                    .unwrap();
            } else {
                // TODO dev id
                parent
                    .mknod(parent.clone(), child_name, InodeMode::FileREG, None)
                    .unwrap();
            }
            let res = <dyn Inode>::lookup_from_root_tmp(name);
            if let Some(inode) = res.as_ref() {
                <dyn Inode>::create_page_cache_if_needed(inode.clone());
            }
            res
        } else {
            warn!("parent dir {} doesn't exist", parent_path);
            return None;
        }
    } else {
        inode
    }
}

/// We should give the absolute path (Or None) to open_file function.
pub fn open_file(absolute_path: Option<String>, flags: u32) -> SyscallRet {
    stack_trace!();
    debug!(
        "[open_file] absolute path: {:?}, flags: {}",
        absolute_path, flags
    );
    let flags = OpenFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    match absolute_path {
        Some(absolute_path) => {
            debug!("[open_file] file name {}", absolute_path);
            if let Some(inode) = resolve_path(&absolute_path, flags) {
                stack_trace!();
                let mut inner_lock = inode.metadata().inner.lock();
                inner_lock.st_atim = current_time_spec();
                match inner_lock.state {
                    InodeState::Synced => {
                        inner_lock.state = InodeState::DirtyInode;
                    }
                    _ => {}
                }
                debug!(
                    "[open_file] inode ino: {}, name: {}",
                    inode.metadata().ino,
                    inode.metadata().name
                );
                // TODO: add to fs's dirty list
                let fd = current_process().inner_handler(|proc| {
                    let fd = proc.fd_table.alloc_fd();
                    let file = inode.open(inode.clone(), flags)?;

                    proc.fd_table.put(fd, file);
                    Ok(fd)
                })?;
                debug!("[open_file] find fd: {}", fd);
                Ok(fd as isize)
            } else {
                debug!("file {} doesn't exist", absolute_path);
                Err(SyscallErr::ENOENT)
            }
        }
        None => {
            debug!("cannot find the file, absolute_path is none");
            Err(SyscallErr::ENOENT)
        }
    }
}

pub fn list_rootfs() {
    <dyn Inode>::load_children(FILE_SYSTEM_MANAGER.root_inode());
    for sb in FILE_SYSTEM_MANAGER.root_inode().metadata().inner.lock().children.iter() {
        println!("-- {}", sb.0);
    }
}
