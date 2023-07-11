use core::sync::atomic::AtomicUsize;

use alloc::vec::Vec;
use alloc::{string::ToString, sync::Arc};
use log::debug;

use self::null::NullInode;
use self::rtc::RtcInode;
use self::{tty::TtyInode, zero::ZeroInode};
use crate::fs::ffi::StatFlags;
use crate::fs::hash_key::HashKey;
use crate::fs::inode::{FAST_PATH, INODE_CACHE};
use crate::utils::error::GeneralRet;

use super::inode::FAST_PATH_CACHE;
use super::testfs::TestRootInode;
use super::FileSystemType;
use super::{
    file_system::{FileSystem, FileSystemMeta},
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
        _mode: InodeMode,
        dev_id: Option<usize>,
    ) -> GeneralRet<Arc<dyn Inode>> {
        debug!("[DevRootInode::mknod]: mknod: {}", name);
        //        debug_assert!(dev_id.unwrap() < DEV_NAMES.len());
        let creator = DEV_NAMES[dev_id.unwrap()].2;
        let inode = creator(this.clone(), DEV_NAMES[dev_id.unwrap()].0);
        this.metadata()
            .inner
            .lock()
            .children
            .insert(inode.metadata().name.clone(), inode.clone());
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

/// TODO: resolve dtb instead of constant list
const DEV_NAMES: [(
    &str,
    InodeMode,
    fn(parent: Arc<dyn Inode>, path: &str) -> Arc<dyn Inode>,
); 5] = [
    ("/dev/vda2", InodeMode::FileBLK, |parent, path| {
        Arc::new(TestRootInode::new(parent, path))
    }),
    ("/dev/zero", InodeMode::FileCHR, |parent, path| {
        Arc::new(ZeroInode::new(parent, path))
    }),
    ("/dev/null", InodeMode::FileCHR, |parent, path| {
        Arc::new(NullInode::new(parent, path))
    }),
    ("/dev/tty", InodeMode::FileCHR, |parent, path| {
        Arc::new(TtyInode::new(parent, path))
    }),
    ("/dev/rtc", InodeMode::FileCHR, |parent, path| {
        Arc::new(RtcInode::new(parent, path))
    }),
];

pub struct DevFs {
    metadata: FileSystemMeta,
    // _id_allocator: AtomicUsize,
    // dev_mgr: Arc<DevManager>,
}

impl DevFs {
    pub fn new(
        mount_point: &str,
        dev_name: &str,
        fstype: FileSystemType,
        flags: StatFlags,
        fa_inode: Option<Arc<dyn Inode>>,
        covered_inode: Option<Arc<dyn Inode>>,
    ) -> GeneralRet<Self> {
        let mut raw_root_inode = DevRootInode::new();
        raw_root_inode.init(Option::clone(&fa_inode), mount_point, InodeMode::FileDIR, 0)?;
        let root_inode = Arc::new(raw_root_inode);

        let id_allocator = AtomicUsize::new(0);

        let mut cache_lock = INODE_CACHE.lock();
        let mut fast_path = FAST_PATH_CACHE.lock();
        let parent_ino = root_inode.metadata().ino;
        for (dev_name2, inode_mode, _) in DEV_NAMES {
            let child = root_inode.mknod(
                root_inode.clone(),
                dev_name2,
                inode_mode,
                Some(id_allocator.fetch_add(1, core::sync::atomic::Ordering::AcqRel)),
            )?;
            let child_name = child.metadata().name.clone();
            let key = HashKey::new(parent_ino, child_name);
            cache_lock.insert(key, child.clone());
            if FAST_PATH.contains(&dev_name2) {
                debug!("inster {} into fast path cache", dev_name2);
                fast_path.insert(dev_name2.to_string(), child);
            }
            debug!("insert {} finished", dev_name2);
        }

        Ok(Self {
            metadata: FileSystemMeta {
                dev_name: dev_name.to_string(),
                mount_point: mount_point.to_string(),
                fstype,
                flags,
                root_inode,
                fa_inode,
                covered_inode,
                s_dirty: Vec::new(),
            },
            // id_allocator: id_allocator,
        })
    }
}

impl FileSystem for DevFs {
    fn metadata(&self) -> &FileSystemMeta {
        &self.metadata
    }

    // fn sync_fs(&self) {}
}
