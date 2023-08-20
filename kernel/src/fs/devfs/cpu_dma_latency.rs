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
    utils::error::{AsyscallRet, GeneralRet, SyscallRet},
};
use alloc::{boxed::Box, sync::Arc};
use log::debug;

pub struct LatencyInode {
    metadata: InodeMeta,
    // dev_fs: SyncUnsafeCell<Option<Arc<DevFs>>>,
}

impl LatencyInode {
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

impl Inode for LatencyInode {
    fn open(&self, this: Arc<dyn Inode>) -> GeneralRet<Arc<dyn File>> {
        stack_trace!();
        Ok(Arc::new(LatencyFile {
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

pub struct LatencyFile {
    meta: FileMeta,
}

// #[async_trait]
impl File for LatencyFile {
    fn metadata(&self) -> &FileMeta {
        stack_trace!();
        &self.meta
    }
    fn read<'a>(&'a self, buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        stack_trace!();
        debug!("[read] /dev/cpu_dma_latency");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            buf.fill(0);
            debug!("/dev/cpu_dma_latency: fill 0");
            Ok(buf.len())
        })
    }
    fn write<'a>(&'a self, buf: &'a [u8], _flags: OpenFlags) -> AsyscallRet {
        stack_trace!();
        debug!("[write] /dev/cpu_dma_latency");
        Box::pin(async move { Ok(buf.len()) })
    }

    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        stack_trace!();
        debug!("[sync_read] /dev/cpu_dma_latency");
        let _sum_guard = SumGuard::new();
        buf.fill(0);
        debug!("[sync_read] /dev/cpu_dma_latency: fill 0");
        Ok(buf.len())
    }

    fn sync_write(&self, buf: &[u8]) -> SyscallRet {
        stack_trace!();
        debug!("[sync_write] /dev/cpu_dma_latency");
        Ok(buf.len())
    }
}
