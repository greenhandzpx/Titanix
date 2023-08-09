use crate::{
    fs::{
        fat32::SECTOR_SIZE,
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, Mutex, OpenFlags,
    },
    processor::SumGuard,
    sync::mutex::SleepLock,
    utils::{
        error::{AsyscallRet, GeneralRet, SyscallRet},
        random::RNG,
    },
};
use alloc::{boxed::Box, sync::Arc};
use log::debug;
use rand_core::RngCore;

pub struct UrandomInode {
    metadata: InodeMeta,
    // dev_fs: SyncUnsafeCell<Option<Arc<DevFs>>>,
}

impl UrandomInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        let metadata = InodeMeta::new(
            Some(parent),
            path,
            crate::fs::InodeMode::FileCHR,
            SECTOR_SIZE,
            None,
        );
        Self { metadata }
    }
}

impl Inode for UrandomInode {
    fn open(&self, this: Arc<dyn Inode>) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(UrandomFile {
            meta: FileMeta {
                inner: Mutex::new(FileMetaInner {
                    inode: Some(this),
                    mode: self.metadata.mode,
                    pos: 0,
                    dirent_index: 0,
                    file: None,
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

pub struct UrandomFile {
    meta: FileMeta,
}

// #[async_trait]
impl File for UrandomFile {
    fn metadata(&self) -> &FileMeta {
        &self.meta
    }
    fn read<'a>(&'a self, buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        debug!("[read] /dev/urandom");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            unsafe {
                RNG.fill_bytes(buf);
            }
            Ok(buf.len())
        })
    }
    fn write<'a>(&'a self, _buf: &'a [u8], _flags: OpenFlags) -> AsyscallRet {
        todo!()
    }

    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        debug!("[sync_read] /dev/urandom");
        let _sum_guard = SumGuard::new();
        unsafe {
            RNG.fill_bytes(buf);
        }
        Ok(buf.len())
    }

    fn sync_write(&self, _buf: &[u8]) -> SyscallRet {
        todo!()
    }
}
