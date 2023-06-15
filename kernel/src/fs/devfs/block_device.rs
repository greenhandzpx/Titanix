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
    metadata: InodeMeta,
    dev_fs: Option<Arc<DevFs>>,
}

impl Inode for BlockDeviceInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        todo!()
    }
    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = meta;
    }

    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }
    fn load_children_from_disk(&self, this: Arc<dyn Inode>) {
        panic!("Unsupported operation")
    }
    fn delete_child(&self, child_name: &str) {
        panic!("Unsupported operation delete")
    }
}

impl BlockDeviceInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        let metadata = InodeMeta::new(Some(parent), path, crate::fs::InodeMode::FileBLK, 0);
        Self {
            metadata,
            dev_fs: None,
        }
    }
}
