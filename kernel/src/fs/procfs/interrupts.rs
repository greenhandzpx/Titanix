use alloc::{boxed::Box, sync::Arc};
use log::debug;

use crate::{
    fs::{
        fat32::SECTOR_SIZE,
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, InodeMode, Mutex, OpenFlags, FILE_SYSTEM_MANAGER,
    },
    irq_count::IRQ_COUNTER,
    processor::SumGuard,
    sync::mutex::SleepLock,
    utils::error::{AsyscallRet, GeneralRet, SyscallErr},
};

pub struct InterruptsInode {
    metadata: InodeMeta,
}
impl InterruptsInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        Self {
            metadata: InodeMeta::new(Some(parent), path, InodeMode::FileREG, SECTOR_SIZE, None),
        }
    }
}

impl Inode for InterruptsInode {
    fn open(&self, this: Arc<dyn Inode>) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(InterruptsFile {
            meta: FileMeta {
                inner: Mutex::new(FileMetaInner {
                    inode: Some(this),
                    mode: InodeMode::FileREG,
                    pos: 0,
                    dirent_index: 0,
                    file: None,
                }),
                prw_lock: SleepLock::new(()),
            },
        }))
    }
    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = meta;
    }

    fn load_children_from_disk(&self, _this: Arc<dyn Inode>) {
        panic!("Unsupported operation")
    }

    fn delete_child(&self, _child_name: &str) {
        panic!("Unsupported operation")
    }
    fn child_removeable(&self) -> GeneralRet<()> {
        Err(crate::utils::error::SyscallErr::EPERM)
    }
}

pub struct InterruptsFile {
    meta: FileMeta,
}

impl File for InterruptsFile {
    fn read<'a>(&'a self, buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        debug!("[InterruptsFile] read");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let info = IRQ_COUNTER.dump();
            let len = info.len();
            let mut inner = self.metadata().inner.lock();
            if inner.pos == len {
                debug!("[MountFile] already read");
                Ok(0)
            } else {
                buf[..len].copy_from_slice(info.as_bytes());
                inner.pos = len;
                Ok(len)
            }
        })
    }

    fn write<'a>(&'a self, _buf: &'a [u8], _flags: OpenFlags) -> AsyscallRet {
        debug!("[InterruptsFile] cannot write");
        Box::pin(async move { Err(SyscallErr::EACCES) })
    }

    fn metadata(&self) -> &FileMeta {
        &self.meta
    }
}
