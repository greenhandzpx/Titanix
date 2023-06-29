use core::cell::SyncUnsafeCell;

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::{string::ToString, sync::Arc};
use log::{debug, info};

use crate::fs::file_system::FsDevice;
use crate::fs::posix::StatFlags;
use crate::utils::error::AsyscallRet;
use crate::{fs::file_system::FILE_SYSTEM_MANAGER, utils::error::GeneralRet};

use super::file::{FileMeta, FileMetaInner};
use super::{Mutex, FileSystemType};
use super::{
    file_system::{FileSystem, FileSystemMeta},
    inode::InodeMeta,
    File, Inode, InodeMode, OpenFlags,
};

pub struct TestRootInode {
    pub metadata: Option<InodeMeta>,
}

impl TestRootInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        let metadata = InodeMeta::new(Some(parent), path, crate::fs::InodeMode::FileBLK, 0, None);
        Self {
            metadata: Some(metadata),
        }
    }
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
                inner: Mutex::new(FileMetaInner {
                    flags,
                    inode: Some(this.clone()),
                    pos: 0,
                    dirent_index: 0,
                }),
            }),
        };
        Ok(Arc::new(file))
    }
    fn mkdir(
        &self,
        this: Arc<dyn Inode>,
        pathname: &str,
        mode: InodeMode,
    ) -> GeneralRet<Arc<dyn Inode>> {
        todo!()
        // debug!("testfs mkdir: {}", pathname);
        // let mut new_inode = TestRootInode { metadata: None };
        // new_inode.init(Some(this.clone()), pathname, mode, 0)?;
        // // let key = new_inode.metadata().inner.lock().hash_name.name_hash as usize;
        // let new_inode = Arc::new(new_inode);
        // // INODE_CACHE.lock().insert(key, new_inode.clone());
        // this.metadata()
        //     .inner
        //     .lock()
        //     .children
        //     .insert(new_inode.metadata.as_ref().unwrap().name.clone(), new_inode);
        // Ok(())
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

    fn sync(&self) {
        debug!("try to sync inode to disk");
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
    metadata: FileSystemMeta,
}

impl TestFs {
    pub fn new(
        mount_point: &str,
        dev_name: &str,
        fstype: FileSystemType,
        flags: StatFlags,
        fa_inode: Option<Arc<dyn Inode>>,
        covered_inode: Option<Arc<dyn Inode>>,
    ) -> GeneralRet<Self> {
        let mut root_inode = TestRootInode { metadata: None };
        root_inode.init(Option::clone(&fa_inode), mount_point, super::InodeMode::FileDIR, 0)?;
        
        Ok(Self {
            metadata: FileSystemMeta {
                dev_name: dev_name.to_string(),
                mount_point: mount_point.to_string(),
                fstype,
                flags,
                root_inode: Arc::new(root_inode),
                fa_inode,
                covered_inode,
                s_dirty: Vec::new(),
            }
        })
    }
}

impl FileSystem for TestFs {
    fn sync_fs(&self) {
        self.metadata().root_inode.sync();
    }

    fn metadata(&self) -> &FileSystemMeta {
        &self.metadata
    }
}
