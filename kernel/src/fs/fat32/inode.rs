use crate::{
    fs::{
        fat32::dentry::FAT32DentryContent,
        inode::{Inode, InodeMeta, InodeMode},
        Mutex,
    },
    utils::{
        error::{GeneralRet, SyscallErr},
        path,
    },
};
use alloc::{boxed::Box, string::ToString, sync::Arc};

use log::info;

use super::{
    dentry::{FAT32DirEntry, ATTR_DIRECTORY},
    fat::FileAllocTable,
    file::FAT32File,
    time::{unix_time_to_timespec, FAT32_to_unix_time},
};

pub struct FAT32Inode {
    meta: Option<InodeMeta>,
    fat: Arc<FileAllocTable>,
    file: Mutex<FAT32File>,
}

impl FAT32Inode {
    pub fn new_root(
        fat: Arc<FileAllocTable>,
        fa_inode: Option<Arc<dyn Inode>>,
        path: &str,
        first_cluster: usize,
    ) -> Self {
        let mode = InodeMode::FileDIR;
        let meta = InodeMeta::new(fa_inode, path, mode, 0, None);
        let file = FAT32File::new(Arc::clone(&fat), first_cluster, None);
        Self {
            meta: Some(meta),
            fat: Arc::clone(&fat),
            file: Mutex::new(file),
        }
    }

    pub fn from_dentry(
        fat: Arc<FileAllocTable>,
        fa_inode: Option<Arc<dyn Inode>>,
        dentry: &FAT32DirEntry,
    ) -> Self {
        let mode = if (dentry.attr & ATTR_DIRECTORY) == ATTR_DIRECTORY {
            InodeMode::FileDIR
        } else {
            InodeMode::FileREG
        };
        let meta = InodeMeta::new(
            fa_inode,
            &dentry.fname(),
            mode,
            if mode == InodeMode::FileREG {
                dentry.filesize as usize
            } else {
                0
            },
            None,
        );
        {
            let mut inner_lock = meta.inner.lock();
            inner_lock.st_atim = unix_time_to_timespec(FAT32_to_unix_time(dentry.acc_time));
            inner_lock.st_ctim = unix_time_to_timespec(FAT32_to_unix_time(dentry.crt_time));
            inner_lock.st_mtim = unix_time_to_timespec(FAT32_to_unix_time(dentry.wrt_time));
        }
        let file = FAT32File::new(
            Arc::clone(&fat),
            dentry.fstcluster as usize,
            if mode == InodeMode::FileREG {
                Some(dentry.filesize as usize)
            } else {
                None
            },
        );
        Self {
            meta: Some(meta),
            fat: Arc::clone(&fat),
            file: Mutex::new(file),
        }
    }

    pub fn new(
        fat: Arc<FileAllocTable>,
        fa_inode: Arc<dyn Inode>,
        filename: &str,
        mode: InodeMode,
    ) -> Self {
        let meta = InodeMeta::new(Some(fa_inode), filename, mode, 0, None);
        let file = FAT32File::new(
            Arc::clone(&fat),
            0,
            if mode == InodeMode::FileREG {
                Some(0)
            } else {
                None
            },
        );
        Self {
            meta: Some(meta),
            fat: Arc::clone(&fat),
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
            let inode = FAT32Inode::from_dentry(Arc::clone(&fat), Some(Arc::clone(&this)), &dentry);
            let inode_rc: Arc<dyn Inode> = Arc::new(inode);
            inode_rc.create_page_cache_if_needed();
            meta_inner
                .children
                .insert(dentry.fname(), Arc::clone(&inode_rc));
        }
    }

    fn read<'a>(
        &'a self,
        _offset: usize,
        _buf: &'a mut [u8],
    ) -> crate::utils::error::AgeneralRet<usize> {
        Box::pin(async move { Ok(self.file.lock().read(_buf, _offset)) })
    }

    fn write<'a>(
        &'a self,
        _offset: usize,
        _buf: &'a [u8],
    ) -> crate::utils::error::AgeneralRet<usize> {
        Box::pin(async move { Ok(self.file.lock().write(_buf, _offset)) })
    }

    fn mkdir(
        &self,
        this: Arc<dyn Inode>,
        name: &str,
        mode: InodeMode,
    ) -> GeneralRet<Arc<dyn Inode>> {
        if self.metadata().mode != InodeMode::FileDIR {
            return Err(SyscallErr::ENOTDIR);
        }
        let fat = Arc::clone(&self.fat);
        let s_inode = FAT32Inode::new(fat, this, name, mode);
        let inode: Arc<dyn Inode> = Arc::new(s_inode);
        self.metadata()
            .inner
            .lock()
            .children
            .insert(name.to_string(), Arc::clone(&inode));
        Ok(inode)
    }

    fn mknod(
        &self,
        this: Arc<dyn Inode>,
        name: &str,
        mode: InodeMode,
        _dev_id: Option<usize>,
    ) -> GeneralRet<Arc<dyn Inode>> {
        if self.metadata().mode != InodeMode::FileDIR {
            return Err(SyscallErr::ENOTDIR);
        }
        let fat = Arc::clone(&self.fat);
        let s_inode = FAT32Inode::new(fat, this, name, mode);
        let inode: Arc<dyn Inode> = Arc::new(s_inode);
        inode.create_page_cache_if_needed();
        self.metadata()
            .inner
            .lock()
            .children
            .insert(name.to_string(), Arc::clone(&inode));
        Ok(inode)
    }

    fn delete_child(&self, child_name: &str) {
        self.metadata().inner.lock().children.remove(child_name);
    }
}
