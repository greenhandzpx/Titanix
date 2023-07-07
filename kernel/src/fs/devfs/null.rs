use crate::{
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, Mutex, OpenFlags,
    },
    utils::error::{AsyscallRet, GeneralRet, SyscallRet},
};
use alloc::{boxed::Box, sync::Arc};
use log::debug;

pub struct NullInode {
    metadata: InodeMeta,
    // dev_fs: SyncUnsafeCell<Option<Arc<DevFs>>>,
}

impl NullInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        let metadata = InodeMeta::new(Some(parent), path, crate::fs::InodeMode::FileCHR, 0, None);
        Self { metadata }
    }
}

impl Inode for NullInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(NullFile {
            meta: FileMeta {
                inner: Mutex::new(FileMetaInner {
                    flags,
                    inode: Some(this),
                    pos: 0,
                    dirent_index: 0,
                }),
                // path: self.metadata().path.clone(),
            },
        }))
    }
    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = meta;
    }
    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }
    fn load_children_from_disk(&self, _this: Arc<dyn Inode>) {
        panic!("Unsupported operation")
    }
    fn delete_child(&self, _child_name: &str) {
        panic!("Unsupported operation delete")
    }
}

pub struct NullFile {
    meta: FileMeta,
}

// #[async_trait]
impl File for NullFile {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    fn metadata(&self) -> &FileMeta {
        &self.meta
    }
    fn read<'a>(&'a self, _buf: &'a mut [u8]) -> AsyscallRet {
        debug!("[read] /dev/null");
        Box::pin(async move { Ok(0) })
    }
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        debug!("[write] /dev/null");
        Box::pin(async move { Ok(buf.len() as isize) })
    }
    fn sync_read(&self, _buf: &mut [u8]) -> SyscallRet {
        debug!("[sync_read] /dev/null");
        Ok(0)
    }
    fn sync_write(&self, buf: &[u8]) -> SyscallRet {
        debug!("[sync_write] /dev/null");
        Ok(buf.len() as isize)
    }
}
