use core::sync::atomic::AtomicUsize;

use alloc::{string::ToString, sync::Arc};
use log::{debug, info};

use crate::utils::{error::GeneralRet, path};

use self::mounts::MountsInode;

use super::{
    file_system::FileSystemMeta, inode::InodeMeta, FileSystem, Inode, InodeMode,
    FILE_SYSTEM_MANAGER,
};

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

    fn load_children_from_disk(&self, this: alloc::sync::Arc<dyn Inode>) {
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
        parent
            .expect("No parent")
            .metadata()
            .inner
            .lock()
            .children
            .insert(path::get_name(mount_point).to_string(), res.clone());
        Ok(res)
    }

    fn set_metadata(&mut self, metadata: FileSystemMeta) {
        self.metadata = Some(metadata);
    }

    fn metadata(&self) -> FileSystemMeta {
        self.metadata.as_ref().unwrap().clone()
    }
}

pub fn init() -> GeneralRet<isize> {
    info!("start to init procfs...");
    let mut proc_fs = ProcFs::new();

    // let root_fs = FILE_SYSTEM_MANAGER
    //     .fs_mgr
    //     .lock()
    //     .get("/")
    //     .cloned()
    //     .expect("No root fs is mounted");

    // let mut root_inode = root_fs.metadata().root_inode.clone().unwrap();
    // root_inode.mkdir(root_inode.clone(), "proc", InodeMode::FileDIR)?;
    // let inner = root_inode.metadata().inner.lock();
    // let proc = inner.children.get("proc").unwrap();
    // proc.mknod(proc.clone(), "mounts", InodeMode::FileREG, 0)?;
    // let inner = proc.metadata().inner.lock();
    // let mounts = inner.children.get("mounts").unwrap();
    // let proc_str = "proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0\n";
    // let dev_str = "udev /dev devtmpfs rw,nosuid,relatime 0 0\n";
    // mounts.write(0, proc_str.as_bytes());
    // let len = proc_str.len();
    // mounts.write(len, dev_str.as_bytes());
    Ok(0)
}
