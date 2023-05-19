
use alloc::sync::Arc;

use crate::{
    fs::inode::{Inode, InodeMode, InodeMeta, InodeMetaInner},
    driver::{block::{BlockDevice, self}},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet, SyscallErr},
    mm::Page,
};


pub struct FAT32Inode {
    meta: InodeMeta,
}

impl FAT32Inode {
    fn new(first_cluster: usize) {
        
    }
}

impl Inode for FAT32Inode {
    fn mkdir(&self, this: Arc<dyn Inode>, pathname: &str, mode: InodeMode) -> GeneralRet<()> {
        todo!()
    }
    fn rmdir(&self, name: &str, mode: InodeMode) -> GeneralRet<()> {
        todo!()
    }
    fn mknod(
        &self,
        this: Arc<dyn Inode>,
        pathname: &str,
        mode: InodeMode,
        dev_id: usize,
    ) -> GeneralRet<()> {
        todo!()
    }
    /// Read data from block device
    fn read(&self, offset: usize, buf: &mut [u8]) -> GeneralRet<Arc<Page>> {
        todo!()
    }
    /// Write data to block device
    fn write(&self, offset: usize, buf: &[u8]) -> GeneralRet<usize> {
        todo!()
    }

    fn metadata(&self) -> &InodeMeta {
        todo!()
    }
    fn set_metadata(&mut self, meta: InodeMeta) {
        todo!()
    }

    fn load_children(&self, this: Arc<dyn Inode>) {
        todo!()
    }

    fn delete_child(&self, child_name: &str) {
        todo!()
    }

}