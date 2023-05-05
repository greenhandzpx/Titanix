use alloc::sync::Arc;

use crate::{
    fs::{file::FileMeta, inode::InodeMeta, File, Inode, OpenFlags},
    utils::error::GeneralRet,
};

use super::DevFs;

pub struct BlockDeviceFile {
    meta: FileMeta,
}

pub struct BlockDeviceInode {
    metadata: Option<InodeMeta>,
    dev_fs: Option<Arc<DevFs>>,
}

impl Inode for BlockDeviceInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        todo!()
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

impl BlockDeviceInode {
    pub fn new() -> Self {
        Self {
            metadata: None,
            dev_fs: None,
        }
    }
}
