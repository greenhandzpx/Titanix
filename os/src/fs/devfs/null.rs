use crate::{
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, Mutex, OpenFlags,
    },
    utils::error::{GeneralRet, SyscallRet},
};
use alloc::{boxed::Box, string::ToString, sync::Arc};
use async_trait::async_trait;
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

#[async_trait]
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
    async fn read(&self, buf: &mut [u8]) -> SyscallRet {
        debug!("read /dev/null");
        Ok(0)
    }
    async fn write(&self, buf: &[u8]) -> SyscallRet {
        debug!("write /dev/null");
        Ok(buf.len() as isize)
    }
}
