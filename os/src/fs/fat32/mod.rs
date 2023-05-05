use alloc::sync::Arc;

use crate::{
    driver::LruBufferCache,
    fs::file_system::FILE_SYSTEM_MANAGER,
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet},
};

use super::{file_system::FileSystemMeta, FileSystem, FileSystemType, Inode};

type Mutex<T> = SpinNoIrqLock<T>;

pub struct FAT32FileSystem {}

impl FAT32FileSystem {
    pub fn new(buffer_cache: LruBufferCache) -> Self {
        Self {}
    }
}

impl FileSystem for FAT32FileSystem {
    fn create_root(
        &self,
        parent: Option<Arc<dyn Inode>>,
        mount_point: &str,
    ) -> GeneralRet<Arc<dyn Inode>> {
        todo!()
    }
    fn init(&mut self, mount_point: &str, ftype: FileSystemType) -> GeneralRet<()> {
        todo!()
    }
    fn mount(&self) {}
    fn dirty_inode(&self, inode: Arc<dyn Inode>) {}
    fn write_inode(&self, inode: Arc<dyn Inode>) -> SyscallRet {
        todo!()
    }
    fn sync_fs(&self) -> SyscallRet {
        todo!()
    }
    fn set_metadata(&mut self, meta_data: FileSystemMeta) {
        todo!()
    }
    fn metadata(&self) -> FileSystemMeta {
        todo!()
    }
}
