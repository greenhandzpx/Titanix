use crate::{
    fs::{
        fat32::SECTOR_SIZE,
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, Mutex, OpenFlags,
    },
    processor::SumGuard,
    stack_trace,
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
        stack_trace!();
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
        stack_trace!();
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
        stack_trace!();
        self.metadata = meta;
    }
    fn metadata(&self) -> &InodeMeta {
        stack_trace!();
        &self.metadata
    }
    fn load_children_from_disk(&self, _this: Arc<dyn Inode>) {
        stack_trace!();
        panic!("Unsupported operation load_children")
    }
    fn delete_child(&self, _child_name: &str) {
        stack_trace!();
        panic!("Unsupported operation delete")
    }
    fn child_removeable(&self) -> GeneralRet<()> {
        stack_trace!();
        Err(crate::utils::error::SyscallErr::EPERM)
    }
}

pub struct UrandomFile {
    meta: FileMeta,
}

// #[async_trait]
impl File for UrandomFile {
    fn metadata(&self) -> &FileMeta {
        stack_trace!();
        &self.meta
    }
    fn read<'a>(&'a self, buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        stack_trace!();
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
        stack_trace!();
        todo!()
    }

    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        stack_trace!();
        debug!("[sync_read] /dev/urandom");
        let _sum_guard = SumGuard::new();
        unsafe {
            RNG.fill_bytes(buf);
        }
        Ok(buf.len())
    }

    fn sync_write(&self, _buf: &[u8]) -> SyscallRet {
        stack_trace!();
        todo!()
    }
}
