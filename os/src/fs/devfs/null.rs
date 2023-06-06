use core::pin::Pin;

use crate::{
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, Mutex, OpenFlags,
    },
    utils::error::{AsyscallRet, GeneralRet, SyscallRet},
};
use alloc::{boxed::Box, string::ToString, sync::Arc};
use log::debug;

pub struct NullInode {
    metadata: Option<InodeMeta>,
    // dev_fs: SyncUnsafeCell<Option<Arc<DevFs>>>,
}

impl Inode for NullInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(NullFile {
            meta: FileMeta {
                path: "todo_here()".to_string(),
                flags,
                inner: Mutex::new(FileMetaInner {
                    inode: Some(this),
                    pos: 0,
                }),
                // path: self.metadata().path.clone(),
            },
        }))
    }
    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = Some(meta);
    }
    fn metadata(&self) -> &InodeMeta {
        &self.metadata.as_ref().unwrap()
    }
    fn load_children(&self, this: Arc<dyn Inode>) {
        panic!("Unsupported operation")
    }
    fn delete_child(&self, child_name: &str) {
        panic!("Unsupported operation delete")
    }
}

impl NullInode {
    pub fn new() -> Self {
        Self {
            metadata: None,
            // dev_fs: SyncUnsafeCell::new(None),
        }
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
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        debug!("read /dev/null");
        Box::pin(async move { Ok(0) })
    }
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        debug!("write /dev/null");
        Box::pin(async move { Ok(0) })
    }
}
