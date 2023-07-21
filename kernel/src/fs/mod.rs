mod devfs;
pub mod fat32;
mod fd_table;
mod file;
pub mod file_system;
mod hash_key;
pub mod inode;
// pub mod inode_fat32_tmp;
// pub mod fat32_tmp;
pub mod ffi;
mod page_cache;
pub mod pipe;
mod procfs;
pub mod socket;
mod testfs;

use alloc::string::ToString;
use alloc::sync::Arc;
pub use fat32::FAT32FileSystem;
pub use fd_table::FdTable;
pub use fd_table::MAX_FD;
pub use file::File;
pub use file::SeekFrom;
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
pub use page_cache::PageCache;

use crate::driver::BLOCK_DEVICE;
use crate::fs::inode::FAST_PATH_CACHE;
use crate::mm::MapPermission;
use crate::stack_trace;
use crate::sync::mutex::SpinNoIrqLock;
use crate::utils::error::GeneralRet;
use crate::utils::error::SyscallErr;
use crate::utils::path;

use self::ffi::StatFlags;
use self::file_system::FsDevice;
use self::inode::INODE_CACHE;

type Mutex<T> = SpinNoIrqLock<T>;

pub fn init() {
    INODE_CACHE.init();
    FAST_PATH_CACHE.init();

    FILE_SYSTEM_MANAGER
        .mount(
            "/",
            "/dev/mmcblk0",
            file_system::FsDevice::BlockDevice(BLOCK_DEVICE.lock().as_ref().unwrap().clone()),
            FileSystemType::VFAT,
            StatFlags::ST_NOSUID,
        )
        .expect("rootfs init fail!");
    // FILE_SYSTEM_MANAGER.mount("/", "/dev/vda2", FsDevice::None, FileSystemType::VFAT, StatFlags::ST_NOSUID);

    let root_inode = FILE_SYSTEM_MANAGER.root_inode();

    root_inode.load_children();

    let dev_dir = root_inode
        .mkdir(Arc::clone(&root_inode), "dev", InodeMode::FileDIR)
        .expect("mkdir /dev fail!");

    let key = HashKey::new(root_inode.metadata().ino, "dev".to_string());
    INODE_CACHE.insert(key, dev_dir);

    let proc_dir = root_inode
        .mkdir(Arc::clone(&root_inode), "proc", InodeMode::FileDIR)
        .expect("mkdir /proc fail!");

    let key = HashKey::new(root_inode.metadata().ino, "proc".to_string());
    INODE_CACHE.insert(key, proc_dir);

    let tmp_dir = root_inode
        .mkdir(Arc::clone(&root_inode), "tmp", InodeMode::FileDIR)
        .expect("mkdir /tmp fail!");

    let key = HashKey::new(root_inode.metadata().ino, "tmp".to_string());
    INODE_CACHE.insert(key, tmp_dir);

    let etc_dir = root_inode
        .mkdir(Arc::clone(&root_inode), "etc", InodeMode::FileDIR)
        .expect("mkdir /etc fail!");
    let key = HashKey::new(root_inode.metadata().ino, "etc".to_string());
    INODE_CACHE.insert(key, etc_dir.clone());

    // for build in command
    let sleep = root_inode
        .mknod(Arc::clone(&root_inode), "sleep", InodeMode::FileREG, None)
        .expect("mknod /sleep fail!");
    let key = HashKey::new(root_inode.metadata().ino, "sleep".to_string());
    INODE_CACHE.insert(key, sleep.clone());

    let ls = root_inode
        .mknod(Arc::clone(&root_inode), "ls", InodeMode::FileREG, None)
        .expect("mknod /ls fail!");
    let key = HashKey::new(root_inode.metadata().ino, "ls".to_string());
    INODE_CACHE.insert(key, ls.clone());

    let musl_dl_path = etc_dir
        .mknod(
            Arc::clone(&etc_dir),
            "ld-musl-riscv64-sf.path",
            InodeMode::FileREG,
            None,
        )
        .expect("mknod /etc/ld-musl-riscv64-sf.path fail!");
    let file = musl_dl_path
        .open(musl_dl_path.clone(), OpenFlags::RDWR)
        .unwrap();
    file.sync_write("/".as_bytes()).unwrap();
    // block_on(musl_dl_path.write(0, "/".as_bytes())).unwrap();
    log::debug!("[fs::init] etc dir ino {}", etc_dir.metadata().ino);
    let key = HashKey::new(
        etc_dir.metadata().ino,
        "ld-musl-riscv64-sf.path".to_string(),
    );
    INODE_CACHE.insert(key, musl_dl_path);

    FILE_SYSTEM_MANAGER
        .mount(
            "/dev",
            "udev",
            FsDevice::None,
            FileSystemType::DevTmpFS,
            StatFlags::ST_NOSUID,
        )
        .expect("devfs init fail!");

    FILE_SYSTEM_MANAGER
        .mount(
            "/proc",
            "proc",
            FsDevice::None,
            FileSystemType::Proc,
            StatFlags::ST_NOSUID,
        )
        .expect("procfs init fail!");

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
        const FD_CLOEXEC = 1;
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

/// Try not to use this when you have dirfd
pub fn resolve_path(path: &str, flags: OpenFlags) -> GeneralRet<Arc<dyn Inode>> {
    debug!("[resolve_path]: path: {}, flags: {:?}", path, flags);
    let inode = <dyn Inode>::lookup_from_root(path)?;
    // inode
    if inode.is_some() {
        return Ok(inode.unwrap());
    }
    if flags.contains(OpenFlags::CREATE) {
        let parent_path = path::get_parent_dir(path).unwrap();
        let parent = <dyn Inode>::lookup_from_root(&parent_path)?;
        let child_name = path::get_name(path);
        if let Some(parent) = parent {
            debug!("create file {}", path);
            let res = {
                if flags.contains(OpenFlags::DIRECTORY) {
                    parent
                        .mkdir(parent.clone(), child_name, InodeMode::FileDIR)
                        .unwrap()
                } else {
                    // TODO dev id
                    parent
                        .mknod(parent.clone(), child_name, InodeMode::FileREG, None)
                        .unwrap()
                }
            };
            let key = HashKey::new(parent.metadata().ino, child_name.to_string());
            INODE_CACHE.insert(key, res.clone());
            res.create_page_cache_if_needed();
            Ok(res)
        } else {
            warn!("parent dir {} doesn't exist", parent_path);
            return Err(SyscallErr::ENOENT);
        }
    } else {
        return Err(SyscallErr::ENOENT);
    }
}

/// You should try using this when you have dirfd and path(*const u8), do not use resolve_path.
pub fn resolve_path_with_dirfd(
    dirfd: isize,
    path: *const u8,
    flags: OpenFlags,
) -> GeneralRet<Arc<dyn Inode>> {
    let res = path::path_to_inode(dirfd, path);
    let inode = res.0?;
    stack_trace!();
    let path = res.1.unwrap();
    if inode.is_some() {
        return Ok(inode.unwrap());
    }
    if flags.contains(OpenFlags::CREATE) {
        let parent = res.2;
        let parent = match parent {
            Some(parent) => parent,
            None => {
                let parent_path = path::get_parent_dir(&path).unwrap();
                <dyn Inode>::lookup_from_root(&parent_path)
                    .ok()
                    .unwrap()
                    .unwrap()
            }
        };
        let child_name = path::get_name(&path);
        debug!("create file {}", path);
        let res = {
            if flags.contains(OpenFlags::DIRECTORY) {
                parent
                    .mkdir(parent.clone(), child_name, InodeMode::FileDIR)
                    .unwrap()
            } else {
                // TODO dev id
                parent
                    .mknod(parent.clone(), child_name, InodeMode::FileREG, None)
                    .unwrap()
            }
        };
        let key = HashKey::new(parent.metadata().ino, child_name.to_string());
        INODE_CACHE.insert(key, res.clone());
        res.create_page_cache_if_needed();
        Ok(res)
    } else {
        warn!("parent dir {} doesn't exist", path);
        return Err(SyscallErr::ENOENT);
    }
}

pub fn list_rootfs() {
    FILE_SYSTEM_MANAGER.root_inode().load_children();
    for sb in FILE_SYSTEM_MANAGER
        .root_inode()
        .metadata()
        .inner
        .lock()
        .children
        .iter()
    {
        println!("-- {}", sb.0);
    }
}
