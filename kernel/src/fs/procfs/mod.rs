use core::sync::atomic::AtomicUsize;

use alloc::{string::ToString, sync::Arc, vec::Vec};
use log::debug;

use crate::{
    fs::{ffi::StatFlags, hash_key::HashKey, inode::INODE_CACHE, FileSystemType},
    utils::{error::GeneralRet, path},
};

use self::{meminfo::MeminfoInode, mounts::MountsInode};

use super::{file_system::FileSystemMeta, inode::InodeMeta, FileSystem, Inode, InodeMode};

mod meminfo;
mod mounts;
pub struct ProcRootInode {
    metadata: Option<InodeMeta>,
}

impl Inode for ProcRootInode {
    fn mknod(
        &self,
        this: Arc<dyn Inode>,
        pathname: &str,
        _mode: InodeMode,
        _dev_id: Option<usize>,
    ) -> GeneralRet<Arc<dyn Inode>> {
        debug!("[ProcRootInode mknod] mknod: {}", pathname);
        let mut index = 0;
        for (i, proc) in PROC_NAME.into_iter().enumerate() {
            if proc.0 == pathname {
                index = i;
            }
        }
        let creator = PROC_NAME[index].2;
        let inode = creator(this.clone(), pathname);
        this.metadata()
            .inner
            .lock()
            .children
            .insert(path::get_name(pathname).to_string(), inode.clone());
        Ok(inode)
    }
    fn metadata(&self) -> &InodeMeta {
        &self.metadata.as_ref().unwrap()
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = Some(meta);
    }

    fn load_children_from_disk(&self, this: Arc<dyn Inode>) {
        debug!("[ProcRootInode::load_children_from_disk]: there is nothing we should do.");
    }

    fn delete_child(&self, child_name: &str) {
        todo!()
    }
}

impl ProcRootInode {
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

const PROC_NAME: [(
    &str,
    InodeMode,
    fn(parent: Arc<dyn Inode>, path: &str) -> Arc<dyn Inode>,
); 2] = [
    ("/proc/mounts", InodeMode::FileREG, |parent, path| {
        Arc::new(MountsInode::new(parent, path))
    }),
    ("/proc/meminfo", InodeMode::FileREG, |parent, path| {
        Arc::new(MeminfoInode::new(parent, path))
    }),
];

pub struct ProcFs {
    metadata: FileSystemMeta,
    id_allocator: AtomicUsize,
}

impl ProcFs {
    pub fn new(
        mount_point: &str,
        dev_name: &str,
        fstype: FileSystemType,
        flags: StatFlags,
        fa_inode: Option<Arc<dyn Inode>>,
        covered_inode: Option<Arc<dyn Inode>>,
    ) -> GeneralRet<Self> {
        let mut raw_root_inode = ProcRootInode::new();
        raw_root_inode.init(Option::clone(&fa_inode), mount_point, InodeMode::FileDIR, 0)?;
        let root_inode = Arc::new(raw_root_inode);

        let id_allocator = AtomicUsize::new(0);

        let mut cache_lock = INODE_CACHE.lock();
        let parent_ino = root_inode.metadata().ino;
        for (proc_name, inode_mode, _) in PROC_NAME {
            let child = root_inode.mknod(
                root_inode.clone(),
                proc_name,
                inode_mode,
                Some(id_allocator.fetch_add(1, core::sync::atomic::Ordering::AcqRel)),
            )?;
            let child_name = child.metadata().name.clone();
            let key = HashKey::new(parent_ino, child_name);
            cache_lock.insert(key, child);
            debug!("insert {} finished", proc_name);
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
            id_allocator,
        })
    }
}

impl FileSystem for ProcFs {
    fn metadata(&self) -> &FileSystemMeta {
        &self.metadata
    }

    // fn sync_fs(&self) {}
}
