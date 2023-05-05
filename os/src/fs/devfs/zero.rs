use crate::{
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, Mutex, OpenFlags,
    },
    processor::SumGuard,
    utils::error::{GeneralRet, SyscallRet},
};
use alloc::{boxed::Box, string::ToString, sync::Arc};
use async_trait::async_trait;
use log::debug;

pub struct ZeroInode {
    metadata: Option<InodeMeta>,
    // dev_fs: SyncUnsafeCell<Option<Arc<DevFs>>>,
}

impl Inode for ZeroInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(ZeroFile {
            meta: FileMeta {
                path: "todo_here()".to_string(),
                // path: self.metadata().path.clone(),
                flags,
                inner: Mutex::new(FileMetaInner {
                    inode: Some(this),
                    pos: 0,
                }),
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
        panic!("Unsupported operation load_children")
    }
    fn delete_child(&self, child_name: &str) {
        panic!("Unsupported operation delete")
    }
}

impl ZeroInode {
    pub fn new() -> Self {
        Self {
            metadata: None,
            // dev_fs: SyncUnsafeCell::new(None),
        }
    }
}

pub struct ZeroFile {
    meta: FileMeta,
}

#[async_trait]
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
    async fn read(&self, buf: &mut [u8]) -> SyscallRet {
        debug!("read /dev/zero");
        let _sum_guard = SumGuard::new();
        buf.fill(0);
        debug!("/dev/zero: fill 0");
        Ok(buf.len() as isize)
    }
    async fn write(&self, buf: &[u8]) -> SyscallRet {
        debug!("write /dev/zero");
        Ok(buf.len() as isize)
    }
}
