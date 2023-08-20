use alloc::sync::Arc;

use crate::{
    driver::BlockDevice,
    fs::{
        fat32::SECTOR_SIZE,
        inode::{DevWrapper, InodeDevice, InodeMeta},
        File, Inode,
    },
    stack_trace,
    utils::error::{GeneralRet, SyscallErr},
};

pub struct BlockDeviceInode {
    metadata: InodeMeta,
}

impl Inode for BlockDeviceInode {
    fn open(&self, _this: Arc<dyn Inode>) -> GeneralRet<Arc<dyn File>> {
        stack_trace!();
        todo!()
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
        panic!("Unsupported operation")
    }
    fn delete_child(&self, _child_name: &str) {
        stack_trace!();
        panic!("Unsupported operation delete")
    }

    fn child_removeable(&self) -> GeneralRet<()> {
        stack_trace!();
        Err(SyscallErr::EPERM)
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
        stack_trace!();
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
