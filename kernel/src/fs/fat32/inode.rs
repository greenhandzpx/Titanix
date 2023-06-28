use alloc::{sync::Arc, boxed::Box};
use crate::{
    fs::{
        inode::{Inode, InodeMeta, InodeMode},
        Mutex, fat32::dentry::FAT32DentryContent,
    },
    utils::error::GeneralRet,
};

use log::info;

use super::{
    file::FAT32File,
    dentry::{FAT32DirEntry, ATTR_DIRECTORY}, fat::FileAllocTable, time::{FAT32_to_unix_time, unix_time_to_timespec}};

pub struct FAT32Inode {
    meta: Option<InodeMeta>,
    file: Mutex<FAT32File>,
}

impl FAT32Inode {
    pub fn new_root_dentry(fat: Arc<FileAllocTable>, fa_inode: Option<Arc<dyn Inode>>, path: &str, first_cluster: usize) -> Self {
        let mode = InodeMode::FileDIR;
        let meta = InodeMeta::new(fa_inode, path, mode, 0, None);
        let file = FAT32File::new(fat, first_cluster, None);
        Self {
            meta: Some(meta),
            file: Mutex::new(file),
        }
    }

    pub fn new(fat: Arc<FileAllocTable>, fa_inode: Option<Arc<dyn Inode>>, dentry: &FAT32DirEntry) -> Self {
        let mode = if (dentry.attr & ATTR_DIRECTORY) == ATTR_DIRECTORY {InodeMode::FileDIR} else {InodeMode::FileREG};
        let meta = InodeMeta::new(fa_inode, &dentry.fname(), mode,
            if mode == InodeMode::FileREG {dentry.filesize as usize} else {0}, None);
        {
            let mut inner_lock = meta.inner.lock();
            inner_lock.st_atim = unix_time_to_timespec(FAT32_to_unix_time(dentry.acc_time));
            inner_lock.st_ctim = unix_time_to_timespec(FAT32_to_unix_time(dentry.crt_time));
            inner_lock.st_mtim = unix_time_to_timespec(FAT32_to_unix_time(dentry.wrt_time));
        }
        let file = FAT32File::new(fat, dentry.fstcluster as usize,
            if mode == InodeMode::FileREG {Some(dentry.filesize as usize)} else {None});
        Self {
            meta: Some(meta),
            file: Mutex::new(file),
        }
    }
}

impl Inode for FAT32Inode {
    fn metadata(&self) -> &InodeMeta {
        &self.meta.as_ref().unwrap()
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.meta = Some(meta);
    }

    fn load_children_from_disk(&self, this: Arc<dyn Inode>) {
        if self.meta.is_none() {
            info!("meta is none!");
            return;
        }
        let meta = self.meta.as_ref().unwrap();
        if meta.mode != InodeMode::FileDIR {
            info!("inode is not dir!");
            return;
        }
        let mut meta_inner = meta.inner.lock();
        let mut content = self.file.lock();
        let fat = Arc::clone(&content.fat);
        let mut dentry_content = FAT32DentryContent::new(&mut content);
        while let Some(dentry) = FAT32DirEntry::read_dentry(&mut dentry_content) {
            let inode = FAT32Inode::new(Arc::clone(&fat), Some(Arc::clone(&this)), &dentry);
            let inode_rc: Arc<dyn Inode> = Arc::new(inode);
            <dyn Inode>::create_page_cache_if_needed(Arc::clone(&inode_rc));
            meta_inner.children.insert(dentry.fname(), Arc::clone(&inode_rc));
        }
    }

    fn read<'a>(&'a self, _offset: usize, _buf: &'a mut [u8]) -> crate::utils::error::AgeneralRet<usize> {
        Box::pin(async move {
            Ok(self.file.lock().read(_buf, _offset))
        })
    }

    fn write<'a>(&'a self, _offset: usize, _buf: &'a [u8]) -> crate::utils::error::AgeneralRet<usize> {
        Box::pin(async move {
            Ok(self.file.lock().write(_buf, _offset))
        })
    }

    fn mkdir(&self, _this: Arc<dyn Inode>, _pathname: &str, _mode: InodeMode) -> GeneralRet<Arc<dyn Inode>> {
        todo!()
    }

    fn mknod(
            &self,
            _this: Arc<dyn Inode>,
            _pathname: &str,
            _mode: InodeMode,
            _dev_id: usize,
        ) -> GeneralRet<Arc<dyn Inode>> {
            todo!()
    }

    fn remove_child(&self, child: Arc<dyn Inode>) -> GeneralRet<isize> {
        todo!()
    }

    fn rmdir(&self, _name: &str, _mode: InodeMode) -> GeneralRet<()> {
        todo!()
    }

    fn unlink(&self, child: Arc<dyn Inode>) -> GeneralRet<isize> {
        todo!()
    }

    fn delete_child(&self, child_name: &str) {
        todo!()
    }
}
