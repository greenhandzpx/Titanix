mod devfs;
pub mod fat32;
mod fd_table;
pub mod ffi;
mod file;
pub mod file_system;
mod hash_key;
pub mod inode;
mod page_cache;
pub mod pipe;
mod procfs;
pub mod socket;
pub mod tmpfs;

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
use crate::loader::get_app_data_by_name;
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

fn create_mem_file(parent_inode: &Arc<dyn Inode>, name: &str) {
    let inode = parent_inode
        .mknod_v(name, InodeMode::FileREG, None)
        .unwrap();
    let file = inode.open(inode.clone(), OpenFlags::RDWR).unwrap();
    file.sync_write(get_app_data_by_name(name).unwrap())
        .unwrap();
}

pub fn init() {
    INODE_CACHE.init();
    FAST_PATH_CACHE.init();

    // First we mount root fs
    #[cfg(feature = "tmpfs")]
    FILE_SYSTEM_MANAGER
        .mount(
            "/",
            // TODO: not sure
            "/dev/tmp",
            file_system::FsDevice::None,
            FileSystemType::TmpFS,
            StatFlags::ST_NOSUID,
        )
        .expect("rootfs init fail!");

    #[cfg(not(feature = "tmpfs"))]
    FILE_SYSTEM_MANAGER
        .mount(
            "/",
            "/dev/mmcblk0",
            file_system::FsDevice::BlockDevice(BLOCK_DEVICE.lock().as_ref().unwrap().clone()),
            FileSystemType::VFAT,
            StatFlags::ST_NOSUID,
        )
        .expect("rootfs init fail!");

    #[cfg(feature = "preliminary")]
    FILE_SYSTEM_MANAGER.mount(
        "/",
        "/dev/vda2",
        FsDevice::None,
        FileSystemType::VFAT,
        StatFlags::ST_NOSUID,
    );

    let root_inode = FILE_SYSTEM_MANAGER.root_inode();

    root_inode.load_children();

    #[cfg(feature = "tmpfs")]
    let mem_apps = [
        "time-test",
        "busybox_testcode.sh",
        "busybox_cmd.txt",
        "busybox",
        "runtestcases",
        "shell",
        "lmbench_all",
        "lmbench_testcode.sh",
        "runtest.exe",
        "entry-static.exe",
        "run-static.sh",
    ];
    #[cfg(not(feature = "tmpfs"))]
    let mem_apps = ["busybox", "runtestcases", "shell"];
    for app in mem_apps {
        create_mem_file(&root_inode, app);
    }

    // For builtin commands
    let builtin_cmds = ["sleep", "ls"];
    for cmd in builtin_cmds {
        root_inode.mknod_v(cmd, InodeMode::FileREG, None).unwrap();
    }

    // Create some necessary dirs
    let dirs = ["dev", "proc", "tmp"];
    for dir in dirs {
        root_inode.mkdir_v(dir, InodeMode::FileDIR).unwrap();
    }

    let var_dir = root_inode.mkdir_v("var", InodeMode::FileDIR).unwrap();
    var_dir
        .mkdir_v("tmp", InodeMode::FileDIR)
        .expect("mkdir /var/tmp fail!");

    let etc_dir = root_inode.mkdir_v("etc", InodeMode::FileDIR).unwrap();
    let musl_dl_path = etc_dir
        .mknod_v("ld-musl-riscv64-sf.path", InodeMode::FileREG, None)
        .expect("mknod /etc/ld-musl-riscv64-sf.path fail!");
    let file = musl_dl_path
        .open(musl_dl_path.clone(), OpenFlags::RDWR)
        .unwrap();
    file.sync_write("/".as_bytes()).unwrap();

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

    FILE_SYSTEM_MANAGER
        .mount(
            "/tmp",
            "tmp",
            FsDevice::None,
            FileSystemType::TmpFS,
            StatFlags::ST_NOSUID,
        )
        .expect("tmpfs init fail!");

    FILE_SYSTEM_MANAGER
        .mount(
            "/var/tmp",
            "var_tmp",
            FsDevice::None,
            FileSystemType::TmpFS,
            StatFlags::ST_NOSUID,
        )
        .expect("tmpfs init fail!");

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

/// Resolve path at dirfd(except that `path` is absolute path)
pub fn resolve_path(dirfd: isize, path: &str, flags: OpenFlags) -> GeneralRet<Arc<dyn Inode>> {
    let res = path::path_to_inode(dirfd, Some(path));
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
                parent.mkdir_v(child_name, InodeMode::FileDIR).unwrap()
            } else {
                // TODO dev id
                parent
                    .mknod_v(child_name, InodeMode::FileREG, None)
                    .unwrap()
            }
        };
        Ok(res)
    } else {
        warn!("parent dir {} doesn't exist", path);
        return Err(SyscallErr::ENOENT);
    }
}

pub fn list_rootfs() {
    stack_trace!();
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
