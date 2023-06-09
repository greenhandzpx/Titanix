use core::sync::atomic::AtomicUsize;

use alloc::boxed::Box;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::{Arc, Weak},
};
use log::{debug, info};

use crate::utils::error::AsyscallRet;
use crate::utils::path::Path;
use crate::{
    driver::block::{BlockDevice, BlockDeviceImpl},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet},
};

use self::{block_device::BlockDeviceInode, zero::ZeroInode};

use super::file::FileMetaInner;
use super::OpenFlags;
use super::{
    file::FileMeta,
    // dentry::DentryMeta,
    file_system::{FileSystem, FileSystemMeta, FILE_SYSTEM_MANAGER},
    inode::{InodeDevice, InodeMeta, InodeMode},
    File,
    Inode,
};

mod block_device;
mod null;
mod zero;

type Mutex<T> = SpinNoIrqLock<T>;

/// i.e. /dev
pub struct DevRootInode {
    metadata: Option<InodeMeta>,
    dev_mgr: Arc<DevManager>,
}

impl Inode for DevRootInode {
    /// Look up for target like 'sda' 'null' etc
    // fn lookup(&self, target_name: &str) -> Option<Arc<dyn Inode>> {
    //     let dev_mgr = self.dev_mgr.0.lock();
    //     let dev_wrapper = dev_mgr.get(target_name);
    //     match dev_wrapper {
    //         Some(dev_wrapper) => Some(dev_wrapper.inode.clone()),
    //         None => None,
    //     }
    // }

    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn super::File>> {
        Ok(Arc::new(DevRootDir {
            meta: FileMeta {
                path: self.metadata().path.clone(),
                flags,
                inner: Mutex::new(FileMetaInner {
                    inode: Some(this),
                    pos: 0,
                    dirent_index: 0,
                }),
            },
        }))
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = Some(meta);
    }

    fn metadata(&self) -> &InodeMeta {
        &self.metadata.as_ref().unwrap()
    }

    /// Load children like 'sda' 'null' etc
    fn load_children_from_disk(&self, this: Arc<dyn Inode>) {
        let mut meta = self.metadata().inner.lock();
        let dev_mgr = self.dev_mgr.dev_map.lock();
        for dev in dev_mgr.iter() {
            meta.children.insert(dev.0.clone(), dev.1.inode.clone());
        }
    }

    /// Delete inode in disk
    fn delete_child(&self, child_name: &str) {}
}

impl DevRootInode {
    pub fn new(dev_mgr: Arc<DevManager>) -> Self {
        Self {
            metadata: None,
            dev_mgr,
        }
    }
}

/// i.e. /dev dir
pub struct DevRootDir {
    meta: FileMeta,
}

// #[async_trait]
impl File for DevRootDir {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn metadata(&self) -> &FileMeta {
        &self.meta
    }
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        todo!("how to read dir file?")
        // buf.fill(0);
        // Ok(buf.len() as isize)
    }
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        todo!("how to write dir file?")
    }
}

pub struct DevFs {
    metadata: Option<FileSystemMeta>,
    dev_mgr: Arc<DevManager>,
}

pub struct DevManager {
    pub dev_map: Mutex<BTreeMap<String, DevWrapper>>,
    pub id_allocator: AtomicUsize,
}

pub struct DevWrapper {
    pub dev_id: usize,
    inode: Arc<dyn Inode>,
    dev: Option<Arc<dyn BlockDevice>>,
}

impl DevFs {
    pub fn new() -> Self {
        Self {
            metadata: None,
            dev_mgr: Arc::new(DevManager {
                dev_map: Mutex::new(BTreeMap::new()),
                id_allocator: AtomicUsize::new(1),
            }),
        }
    }
}

impl FileSystem for DevFs {
    /// i.e. parent: /    mount_point: /dev
    fn create_root(
        &self,
        parent: Option<Arc<dyn Inode>>,
        mount_point: &str,
    ) -> GeneralRet<Arc<dyn Inode>> {
        let mut root_inode = DevRootInode::new(self.dev_mgr.clone());
        root_inode.init(parent.clone(), mount_point, InodeMode::FileDIR, 0)?;
        let res = Arc::new(root_inode);
        // TODO: should we add a flag to indicate that this dentry(i.e dev) is no need to be flushed
        // to disk
        parent
            .expect("Need a parent")
            .metadata()
            .inner
            .lock()
            .children
            .insert(Path::get_name(mount_point).to_string(), res.clone());
        Ok(res)
    }
    fn set_metadata(&mut self, metadata: super::file_system::FileSystemMeta) {
        self.metadata = Some(metadata);
    }
    fn metadata(&self) -> FileSystemMeta {
        self.metadata.as_ref().unwrap().clone()
    }
}

pub fn init() -> GeneralRet<()> {
    info!("start to init devfs...");

    let mut dev_fs = DevFs::new();

    dev_fs.init("/dev", crate::fs::FileSystemType::VFAT)?;
    // dev_fs.init("/")?;

    let dev_fs = Arc::new(dev_fs);

    dev_fs.dev_mgr.dev_map.lock().insert(
        "vda2".to_string(),
        DevWrapper {
            dev_id: dev_fs
                .dev_mgr
                .id_allocator
                .fetch_add(1, core::sync::atomic::Ordering::AcqRel),
            inode: Arc::new(BlockDeviceInode::new()),
            // dev: Some(Arc::new(BlockDeviceImpl::new())),
            dev: None,
        },
    );
    debug!("insert vda2");
    dev_fs.dev_mgr.dev_map.lock().insert(
        "zero".to_string(),
        DevWrapper {
            dev_id: dev_fs
                .dev_mgr
                .id_allocator
                .fetch_add(1, core::sync::atomic::Ordering::AcqRel),
            inode: Arc::new(ZeroInode::new()),
            dev: None,
        },
    );
    debug!("insert zero");

    FILE_SYSTEM_MANAGER
        .fs_mgr
        .lock()
        .insert("/dev".to_string(), dev_fs);
    info!("init devfs success");

    Ok(())
}
