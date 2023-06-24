use core::sync::atomic::AtomicUsize;

use alloc::{string::ToString, sync::Arc};
use log::{debug, info};

use crate::{
    fs::{posix::StatFlags, FileSystemType, InodeState, FILE_SYSTEM_MANAGER},
    utils::{error::GeneralRet, path},
};

use self::mounts::MountsInode;

use super::{file_system::FileSystemMeta, inode::InodeMeta, FileSystem, Inode, InodeMode};

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
        _dev_id: usize,
    ) -> GeneralRet<()> {
        debug!("[ProcRootInode mknod] mknod: {}", pathname);
        for proc in PROC_NAME {
            if proc.0 == pathname {
                let creator = proc.2;
                let inode = creator(this.clone(), pathname);
                this.metadata()
                    .inner
                    .lock()
                    .children
                    .insert(path::get_name(pathname).to_string(), inode);
            }
        }
        Ok(())
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
); 1] = [("/proc/mounts", InodeMode::FileREG, |parent, path| {
    Arc::new(MountsInode::new(parent, path))
})];

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
        mount_point: &str,
    ) -> GeneralRet<Arc<dyn Inode>> {
        let mut root_inode = ProcRootInode::new();
        root_inode.init(parent.clone(), mount_point, InodeMode::FileDIR, 0)?;
        let res = Arc::new(root_inode);
        let name = path::get_name(mount_point);
        let parent = parent.expect("No pareny");
        parent
            .metadata()
            .inner
            .lock()
            .children
            .insert(name.to_string(), res.clone());
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

    for (proc_name, inode_mode, _) in PROC_NAME {
        proc_root_inode.mknod(
            proc_root_inode.clone(),
            proc_name,
            inode_mode,
            proc_fs
                .id_allocator
                .fetch_add(1, core::sync::atomic::Ordering::AcqRel),
        )?;
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
