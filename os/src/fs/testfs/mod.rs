use core::cell::{SyncUnsafeCell, UnsafeCell};

use alloc::boxed::Box;
use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use log::{debug, info, warn};

use crate::mm::memory_set::VmArea;
use crate::utils::error::AsyscallRet;
use crate::{
    fs::file_system::FILE_SYSTEM_MANAGER,
    utils::error::{GeneralRet, SyscallErr, SyscallRet},
};

use super::file::{FileMeta, FileMetaInner};
use super::Mutex;
use super::{
    file_system::{FileSystem, FileSystemMeta},
    inode::{InodeMeta, INODE_CACHE},
    File, Inode, InodeMode, OpenFlags,
};

pub struct TestRootInode {
    metadata: Option<InodeMeta>,
}

impl Inode for TestRootInode {
    fn open(
        &self,
        this: Arc<dyn Inode>,
        flags: OpenFlags,
    ) -> GeneralRet<alloc::sync::Arc<dyn super::File>> {
        let file = TestRootFile {
            metadata: Some(FileMeta {
                path: this.metadata().path.clone(),
                flags,
                inner: Mutex::new(FileMetaInner {
                    inode: Some(this.clone()),
                    pos: 0,
                    dirent_index: 0,
                }),
            }),
        };
        Ok(Arc::new(file))
    }
    fn mkdir(&self, this: Arc<dyn Inode>, pathname: &str, mode: InodeMode) -> GeneralRet<()> {
        debug!("testfs mkdir: {}", pathname);
        let mut new_inode = TestRootInode { metadata: None };
        new_inode.init(Some(this.clone()), pathname, mode, 0)?;
        let key = new_inode.metadata().inner.lock().hash_name.name_hash as usize;
        let new_inode = Arc::new(new_inode);
        INODE_CACHE.lock().insert(key, new_inode.clone());
        this.metadata()
            .inner
            .lock()
            .children
            .insert(new_inode.metadata.as_ref().unwrap().name.clone(), new_inode);
        Ok(())
    }

    fn metadata(&self) -> &super::inode::InodeMeta {
        &self.metadata.as_ref().unwrap()
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = Some(meta);
    }
    fn load_children_from_disk(&self, this: Arc<dyn Inode>) {
        debug!("try to load children from test root fs");
        // todo!()
    }
    fn delete_child(&self, child_name: &str) {
        debug!("try to delete inode in disk");
        // todo!()
    }
}

pub struct TestRootFile {
    metadata: Option<FileMeta>,
}

// #[async_trait]
impl File for TestRootFile {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        debug!("try to read from test root file");
        Box::pin(async move { Ok(0) })
    }
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        debug!("try to write to test root file");
        Box::pin(async move { Ok(0) })
    }
    fn metadata(&self) -> &FileMeta {
        self.metadata.as_ref().unwrap()
    }
}

pub struct TestFs {
    metadata: SyncUnsafeCell<Option<FileSystemMeta>>,
}

impl TestFs {
    pub fn new() -> Self {
        Self {
            metadata: SyncUnsafeCell::new(None),
        }
    }
}

impl FileSystem for TestFs {
    fn create_root(
        &self,
        parent: Option<alloc::sync::Arc<dyn Inode>>,
        mount_point: &str,
    ) -> crate::utils::error::GeneralRet<alloc::sync::Arc<dyn Inode>> {
        let mut root_inode = TestRootInode { metadata: None };
        root_inode.init(parent, mount_point, super::InodeMode::FileDIR, 0)?;
        Ok(Arc::new(root_inode))
    }

    fn metadata(&self) -> super::file_system::FileSystemMeta {
        unsafe { (*self.metadata.get()).as_ref().unwrap().clone() }
        // self.metadata.as_ref().unwrap().clone()
    }

    fn set_metadata(&mut self, metadata: super::file_system::FileSystemMeta) {
        // self.metadata = Some(metadata);
        self.metadata = SyncUnsafeCell::new(Some(metadata));
    }

    fn set_metadata_ref(&self, metadata: FileSystemMeta) {
        unsafe { *self.metadata.get() = Some(metadata) }
    }
}

pub fn init() -> GeneralRet<()> {
    info!("start to init testfs...");

    let mut test_fs = TestFs {
        metadata: SyncUnsafeCell::new(None),
    };
    test_fs.init("/", crate::fs::FileSystemType::VFAT)?;

    FILE_SYSTEM_MANAGER
        .fs_mgr
        .lock()
        .insert("/".to_string(), Arc::new(test_fs));
    info!("init testfs success");

    Ok(())
}
