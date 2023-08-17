use alloc::{borrow::ToOwned, boxed::Box, sync::Arc};
use log::debug;

use crate::{
    fs::{
        fat32::SECTOR_SIZE,
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, InodeMode, Mutex, OpenFlags,
    },
    processor::SumGuard,
    sync::mutex::SleepLock,
    utils::error::{AsyscallRet, GeneralRet, SyscallErr},
};

pub struct FilesystemsInode {
    metadata: InodeMeta,
}
impl FilesystemsInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        Self {
            metadata: InodeMeta::new(Some(parent), path, InodeMode::FileREG, SECTOR_SIZE, None),
        }
    }
}

impl Inode for FilesystemsInode {
    fn open(&self, this: Arc<dyn Inode>) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(FilesystemsFile {
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
}

pub struct FilesystemsFile {
    meta: FileMeta,
}

impl File for FilesystemsFile {
    fn read<'a>(&'a self, buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        debug!("[FilesystemsFile] read");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let info =
                "nodev\ttmpfs\n".to_owned() + "nodev\tproc\n" + "nodev\tdevtmpfs\n" + "\tvfat\n";
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
        debug!("[FilesystemsFile] cannot write");
        Box::pin(async move { Err(SyscallErr::EACCES) })
    }

    fn metadata(&self) -> &FileMeta {
        &self.meta
    }
}
