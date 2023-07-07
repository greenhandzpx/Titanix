use core::sync::atomic::AtomicUsize;

use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use log::{debug, info};

use crate::{
    fs::{
        hash_key::HashKey, inode::INODE_CACHE, posix::StatFlags, FileSystemType, InodeState,
        FILE_SYSTEM_MANAGER,
    },
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
        name: &str,
        _mode: InodeMode,
        _dev_id: usize,
    ) -> GeneralRet<Arc<dyn Inode>> {
        debug!("[ProcRootInode mknod] mknod: {}", name);
        let mut index = 0;
        for (i, proc) in PROC_NAME.into_iter().enumerate() {
            if proc.0 == name {
                index = i;
            }
        }
        let creator = PROC_NAME[index].2;
        let inode = creator(this.clone(), PROC_NAME[index].0.to_string());
        this.metadata()
            .inner
            .lock()
            .children
            .insert(name.to_string(), inode.clone());
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
    fn(parent: Arc<dyn Inode>, name: String) -> Arc<dyn Inode>,
); 2] = [
    ("mounts", InodeMode::FileREG, |parent, name| {
        Arc::new(MountsInode::new(parent, name))
    }),
    ("meminfo", InodeMode::FileREG, |parent, name| {
        Arc::new(MeminfoInode::new(parent, name))
    }),
];

pub struct ProcFs {
    metadata: Option<FileSystemMeta>,
    id_allocator: AtomicUsize,
}

impl ProcFs {
    pub fn new() -> Self {
        Self {
            metadata: None,
            id_allocator: AtomicUsize::new(0),
        }
    }
}

impl FileSystem for ProcFs {
    fn create_root(
        &self,
        parent: Option<Arc<dyn Inode>>,
        name: String,
    ) -> GeneralRet<Arc<dyn Inode>> {
        let mut root_inode = ProcRootInode::new();
        root_inode.init(parent.clone(), name.clone(), InodeMode::FileDIR, 0)?;
        let res = Arc::new(root_inode);
        let parent = parent.expect("No pareny");
        parent
            .metadata()
            .inner
            .lock()
            .children
            .insert(name, res.clone());
        Ok(res)
    }

    fn set_metadata(&mut self, metadata: FileSystemMeta) {
        self.metadata = Some(metadata);
    }

    fn metadata(&self) -> FileSystemMeta {
        self.metadata.as_ref().unwrap().clone()
    }
}

pub fn init() -> GeneralRet<()> {
    info!("start to init procfs...");
    let mut proc_fs = ProcFs::new();
    proc_fs.init(
        "proc".to_string(),
        "/proc",
        FileSystemType::VFAT,
        StatFlags::ST_NOSUID | StatFlags::ST_NODEV | StatFlags::ST_NOEXEC,
    )?;
    let proc_fs = Arc::new(proc_fs);
    let proc_root_inode = proc_fs.metadata().root_inode.as_ref().cloned().unwrap();

    let mut cache_lock = INODE_CACHE.lock();
    let parent_ino = proc_root_inode.metadata().ino;

    for (proc_name, inode_mode, _) in PROC_NAME {
        let child = proc_root_inode.mknod(
            proc_root_inode.clone(),
            proc_name,
            inode_mode,
            proc_fs
                .id_allocator
                .fetch_add(1, core::sync::atomic::Ordering::AcqRel),
        )?;
        let key = HashKey::new(parent_ino, proc_name.to_string());
        cache_lock.insert(key, child);
        debug!("[procfs] insert {} finished", proc_name);
    }

    FILE_SYSTEM_MANAGER
        .fs_mgr
        .lock()
        .insert("/proc".to_string(), proc_fs);
    info!("[procfs] init procfs success");

    proc_root_inode.metadata().inner.lock().state = InodeState::Synced;

    Ok(())
}
