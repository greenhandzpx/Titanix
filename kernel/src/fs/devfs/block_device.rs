use alloc::sync::Arc;

use crate::{
    driver::BlockDevice,
    fs::{
        fat32::SECTOR_SIZE,
        inode::{DevWrapper, InodeDevice, InodeMeta},
        File, Inode, OpenFlags,
    },
    utils::error::GeneralRet,
};

pub struct BlockDeviceInode {
    metadata: InodeMeta,
}

impl Inode for BlockDeviceInode {
    fn open(&self, _this: Arc<dyn Inode>, _flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        todo!()
    }
    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = meta;
    }

    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }
    fn load_children_from_disk(&self, _this: Arc<dyn Inode>) {
        panic!("Unsupported operation")
    }
    fn delete_child(&self, _child_name: &str) {
        panic!("Unsupported operation delete")
    }
}

impl BlockDeviceInode {
    #[allow(unused)]
    pub fn new(
        parent: Arc<dyn Inode>,
        path: &str,
        block_device: Arc<dyn BlockDevice>,
        dev_id: usize,
    ) -> Self {
        let metadata = InodeMeta::new(
            Some(parent),
            path,
            crate::fs::InodeMode::FileBLK,
            SECTOR_SIZE,
            Some(InodeDevice::Device(DevWrapper {
                block_device,
                dev_id,
            })),
        );
        Self { metadata }
    }
}
