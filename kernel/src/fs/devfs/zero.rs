use crate::{
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, Mutex, OpenFlags,
    },
    processor::SumGuard,
    utils::error::{AsyscallRet, GeneralRet, SyscallRet},
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
};
use log::debug;

pub struct ZeroInode {
    metadata: InodeMeta,
    // dev_fs: SyncUnsafeCell<Option<Arc<DevFs>>>,
}

impl ZeroInode {
    pub fn new(parent: Arc<dyn Inode>, name: String) -> Self {
        let metadata = InodeMeta::new(Some(parent), name, crate::fs::InodeMode::FileCHR, 0, None);
        Self { metadata }
    }
}

impl Inode for ZeroInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(ZeroFile {
            meta: FileMeta {
                // path: self.metadata().path.clone(),
                inner: Mutex::new(FileMetaInner {
                    flags,
                    inode: Some(this),
                    pos: 0,
                    dirent_index: 0,
                }),
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
        panic!("Unsupported operation load_children")
    }
    fn delete_child(&self, _child_name: &str) {
        panic!("Unsupported operation delete")
    }
}

pub struct ZeroFile {
    meta: FileMeta,
}

// #[async_trait]
impl File for ZeroFile {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    fn metadata(&self) -> &FileMeta {
        &self.meta
    }
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        debug!("[read] /dev/zero");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            buf.fill(0);
            debug!("/dev/zero: fill 0");
            Ok(buf.len() as isize)
        })
    }
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        debug!("[write] /dev/zero");
        Box::pin(async move { Ok(buf.len() as isize) })
    }

    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        debug!("[sync_read] /dev/zero");
        let _sum_guard = SumGuard::new();
        buf.fill(0);
        debug!("[sync_read] /dev/zero: fill 0");
        Ok(buf.len() as isize)
    }

    fn sync_write(&self, buf: &[u8]) -> SyscallRet {
        debug!("[sync_write] /dev/zero");
        Ok(buf.len() as isize)
    }
}
