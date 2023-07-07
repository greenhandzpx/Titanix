use core::sync::atomic::AtomicUsize;

use alloc::string::String;
use alloc::{string::ToString, sync::Arc};
use log::{debug, info};

use crate::fs::hash_key::HashKey;
use crate::fs::inode::INODE_CACHE;
use crate::fs::posix::StatFlags;
use crate::utils::error::GeneralRet;
use crate::utils::path;

use self::null::NullInode;
use self::rtc::RtcInode;
use self::{tty::TtyInode, zero::ZeroInode};

use super::testfs::TestRootInode;
use super::{
    // dentry::DentryMeta,
    file_system::{FileSystem, FileSystemMeta, FILE_SYSTEM_MANAGER},
    inode::{InodeMeta, InodeMode},
    Inode,
};

mod block_device;
mod null;
mod rtc;
mod tty;
mod zero;

/// i.e. /dev
pub struct DevRootInode {
    metadata: Option<InodeMeta>,
}

impl Inode for DevRootInode {
    fn mknod(
        &self,
        this: Arc<dyn Inode>,
        name: &str,
        mode: InodeMode,
        dev_id: usize,
    ) -> GeneralRet<Arc<dyn Inode>> {
        debug!("[DevRootInode::mknod]: mknod: {}", name);
        debug_assert!(dev_id < DEV_NAMES.len());
        let creator = DEV_NAMES[dev_id].2;
        let inode = creator(this.clone(), name.to_string());
        this.metadata()
            .inner
            .lock()
            .children
            .insert(name.to_string(), inode.clone());
        Ok(inode)
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = Some(meta);
    }

    fn metadata(&self) -> &InodeMeta {
        &self.metadata.as_ref().unwrap()
    }

    /// Load children like 'sda' 'null' etc
    fn load_children_from_disk(&self, _this: Arc<dyn Inode>) {
        debug!("[DevRootInode::load_children_from_disk]: there is nothing we should do.");
    }

    /// Delete inode in disk
    fn delete_child(&self, _child_name: &str) {
        todo!()
    }
}

impl DevRootInode {
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

pub struct DevFs {
    metadata: Option<FileSystemMeta>,
    id_allocator: AtomicUsize,
    // dev_mgr: Arc<DevManager>,
}

impl DevFs {
    pub fn new() -> Self {
        Self {
            metadata: None,
            id_allocator: AtomicUsize::new(0),
        }
    }
}

impl FileSystem for DevFs {
    /// i.e. parent: /    mount_point: /dev
    fn create_root(
        &self,
        parent: Option<Arc<dyn Inode>>,
        name: String,
    ) -> GeneralRet<Arc<dyn Inode>> {
        let mut root_inode = DevRootInode::new();
        root_inode.init(parent.clone(), name.clone(), InodeMode::FileDIR, 0)?;
        let res = Arc::new(root_inode);
        // TODO: should we add a flag to indicate that this dentry(i.e dev) is no need to be flushed
        // to disk
        parent
            .expect("Need a parent")
            .metadata()
            .inner
            .lock()
            .children
            .insert(name, res.clone());
        Ok(res)
    }
    fn set_metadata(&mut self, metadata: super::file_system::FileSystemMeta) {
        self.metadata = Some(metadata);
    }
    fn metadata(&self) -> FileSystemMeta {
        self.metadata.as_ref().unwrap().clone()
    }
}

const DEV_NAMES: [(
    &str,
    InodeMode,
    fn(parent: Arc<dyn Inode>, name: String) -> Arc<dyn Inode>,
); 5] = [
    ("vda2", InodeMode::FileBLK, |parent, name| {
        Arc::new(TestRootInode::new(parent, name))
    }),
    ("zero", InodeMode::FileCHR, |parent, name| {
        Arc::new(ZeroInode::new(parent, name))
    }),
    ("null", InodeMode::FileCHR, |parent, name| {
        Arc::new(NullInode::new(parent, name))
    }),
    ("tty", InodeMode::FileCHR, |parent, name| {
        Arc::new(TtyInode::new(parent, name))
    }),
    ("rtc", InodeMode::FileCHR, |parent, name| {
        Arc::new(RtcInode::new(parent, name))
    }),
];

pub fn init() -> GeneralRet<()> {
    info!("start to init devfs...");

    let mut dev_fs = DevFs::new();

    dev_fs.init(
        "udev".to_string(),
        "/dev",
        crate::fs::FileSystemType::VFAT,
        StatFlags::ST_NOSUID,
    )?;
    // dev_fs.init("/")?;

    let dev_fs = Arc::new(dev_fs);

    let dev_root_inode = dev_fs.metadata().root_inode.as_ref().cloned().unwrap();

    let mut cache_lock = INODE_CACHE.lock();
    let parent_ino = dev_root_inode.metadata().ino;
    for (dev_name, inode_mode, _) in DEV_NAMES {
        let child = dev_root_inode.mknod(
            dev_root_inode.clone(),
            dev_name,
            inode_mode,
            dev_fs
                .id_allocator
                .fetch_add(1, core::sync::atomic::Ordering::AcqRel),
        )?;
        let key = HashKey::new(parent_ino, dev_name.to_string());
        cache_lock.insert(key, child);
        debug!("insert {} finished", dev_name);
    }

    FILE_SYSTEM_MANAGER
        .fs_mgr
        .lock()
        .insert("/dev".to_string(), dev_fs);
    info!("init devfs success");

    Ok(())
}
