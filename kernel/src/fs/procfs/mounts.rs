use alloc::{boxed::Box, string::ToString, sync::Arc, vec::Vec};
use log::debug;

use crate::{
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, InodeMode, Mutex, OpenFlags, FILE_SYSTEM_MANAGER,
    },
    processor::SumGuard,
    utils::error::{AsyscallRet, GeneralRet, SyscallErr, SyscallRet},
};

pub struct MountsInode {
    metadata: InodeMeta,
}
impl MountsInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        Self {
            metadata: InodeMeta::new(Some(parent), path, InodeMode::FileREG, 0, None),
        }
    }
}

impl Inode for MountsInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(MountsFile {
            meta: FileMeta {
                path: "/proc/mounts".to_string(),
                inner: Mutex::new(FileMetaInner {
                    flags,
                    inode: Some(this),
                    pos: 0,
                    dirent_index: 0,
                }),
            },
        }))
    }
    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = meta;
    }

    fn load_children_from_disk(&self, this: Arc<dyn Inode>) {
        panic!("Unsupported operation")
    }

    fn delete_child(&self, child_name: &str) {
        panic!("Unsupported operation")
    }
}

pub struct MountsFile {
    meta: FileMeta,
}

impl File for MountsFile {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        debug!("[MountsFile] read");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let info = FILE_SYSTEM_MANAGER.mounts_info();
            let len = info.len();
            let mut inner = self.metadata().inner.lock();
            if inner.pos == len {
                debug!("[MountFile] already read");
                Ok(0)
            } else {
                buf[..len].copy_from_slice(info.as_bytes());
                inner.pos = len;
                Ok(len as isize)
            }
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        debug!("[MountsFile] cannot write");
        Box::pin(async move { Err(SyscallErr::EACCES) })
    }

    fn metadata(&self) -> &FileMeta {
        &self.meta
    }

    fn seek(&self, offset: usize) -> SyscallRet {
        debug!("[MountsFile] seek offset: {}", offset);
        self.meta.inner.lock().pos = offset;
        Ok(offset as isize)
    }
}
