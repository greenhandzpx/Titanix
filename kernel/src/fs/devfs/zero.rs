use crate::{
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, Mutex, OpenFlags,
    },
    processor::SumGuard,
    sync::mutex::SleepLock,
    utils::error::{AsyscallRet, GeneralRet, SyscallRet},
};
use alloc::{boxed::Box, sync::Arc};
use log::debug;

pub struct ZeroInode {
    metadata: InodeMeta,
    // dev_fs: SyncUnsafeCell<Option<Arc<DevFs>>>,
}

impl ZeroInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        let metadata = InodeMeta::new(Some(parent), path, crate::fs::InodeMode::FileCHR, 0, None);
        Self { metadata }
    }
}

impl Inode for ZeroInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(ZeroFile {
            meta: FileMeta {
                inner: Mutex::new(FileMetaInner {
                    flags,
                    inode: Some(this),
                    mode: self.metadata.mode,
                    pos: 0,
                    dirent_index: 0,
                }),
                prw_lock: SleepLock::new(()),
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
            Ok(buf.len())
        })
    }
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        debug!("[write] /dev/zero");
        Box::pin(async move { Ok(buf.len()) })
    }

    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        debug!("[sync_read] /dev/zero");
        let _sum_guard = SumGuard::new();
        buf.fill(0);
        debug!("[sync_read] /dev/zero: fill 0");
        Ok(buf.len())
    }

    fn sync_write(&self, buf: &[u8]) -> SyscallRet {
        debug!("[sync_write] /dev/zero");
        Ok(buf.len())
    }
    fn flags(&self) -> OpenFlags {
        self.meta.inner.lock().flags
    }
}
