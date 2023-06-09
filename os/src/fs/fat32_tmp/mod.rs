use core::cell::UnsafeCell;
use core::panic;

use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::{boxed::Box, vec::Vec};
use fatfs::{DirEntry, Read, Seek, Write};
use lazy_static::*;
use log::{debug, error, info, warn};

use crate::fs::file::DefaultFile;
use crate::fs::inode::INODE_CACHE;
use crate::mm::PageCache;
use crate::utils::error::{self, AsyscallRet, SyscallErr};
use crate::{
    driver::{block::IoDevice, BLOCK_DEVICE},
    processor::SumGuard,
    stack_trace,
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet},
};

use super::file::FileMetaInner;
use super::inode::InodeMeta;
use super::{file::FileMeta, file_system::FileSystemMeta, File, FileSystem, FileSystemType, Inode};
use super::{InodeMode, OpenFlags};

type Mutex<T> = SpinNoIrqLock<T>;

pub struct Fat32FileSystem {
    fat_fs: fatfs::FileSystem<IoDevice, fatfs::DefaultTimeProvider, fatfs::LossyOemCpConverter>,
    meta: UnsafeCell<Option<FileSystemMeta>>,
}

unsafe impl Send for Fat32FileSystem {}
unsafe impl Sync for Fat32FileSystem {}

unsafe impl Send for Fat32Inode {}
unsafe impl Sync for Fat32Inode {}

impl Fat32FileSystem {
    pub fn new(buffer_cache: IoDevice) -> Self {
        Self {
            fat_fs: fatfs::FileSystem::new(buffer_cache, fatfs::FsOptions::new()).unwrap(),
            meta: UnsafeCell::new(None),
        }
    }
}

impl FileSystem for Fat32FileSystem {
    fn create_root(
        &self,
        parent: Option<Arc<dyn Inode>>,
        mount_point: &str,
    ) -> GeneralRet<Arc<dyn Inode>> {
        debug!("fat32: start to create root inode...");
        let mut root_inode = Fat32RootInode::new(&ROOT_FS, None);
        root_inode.init(parent, mount_point, super::InodeMode::FileDIR, 0)?;
        debug!("fat32: create root inode finished");
        Ok(Arc::new(root_inode))
    }

    fn set_metadata(&mut self, metadata: FileSystemMeta) {
        self.meta = UnsafeCell::new(Some(metadata));
    }

    fn metadata(&self) -> FileSystemMeta {
        unsafe { (*self.meta.get()).as_ref().unwrap().clone() }
    }
    fn set_metadata_ref(&self, metadata: FileSystemMeta) {
        unsafe { *self.meta.get() = Some(metadata) }
    }
}

pub struct Fat32RootInode {
    fs: &'static Fat32FileSystem,
    meta: Option<InodeMeta>,
}

impl Fat32RootInode {
    pub fn new(fs: &'static Fat32FileSystem, meta: Option<InodeMeta>) -> Self {
        Self { fs, meta }
    }
}

impl Inode for Fat32RootInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        let (readable, writable) = flags.read_write();
        let file_meta = FileMeta {
            // TODO: not sure whether this file_name() is absolute path or not
            path: "/".to_string(),
            flags,
            inner: Mutex::new(FileMetaInner {
                inode: Some(this),
                pos: 0,
            }),
        };
        Ok(Arc::new(DefaultFile::new(file_meta)))
    }

    fn metadata(&self) -> &InodeMeta {
        &self.meta.as_ref().unwrap()
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.meta = Some(meta);
    }

    fn load_children(&self, this: Arc<dyn Inode>) {
        debug!("[Fat32RootInode]: load children");
        let mut meta_inner = self.meta.as_ref().unwrap().inner.lock();
        for dentry in self.fs.fat_fs.root_dir().iter() {
            let inode_mode = {
                if dentry.as_ref().unwrap().is_dir() {
                    InodeMode::FileDIR
                } else {
                    InodeMode::FileREG
                }
            };
            let data_len = dentry.as_ref().unwrap().len();
            // let dentry_tmp = dentry.as_ref().cloned();
            let file_name = dentry.as_ref().unwrap().file_name();
            // debug!("[load children]: dentry name {}", file_name);
            let meta = InodeMeta::new(
                Some(this.clone()),
                &file_name,
                inode_mode,
                data_len as usize,
            );
            let file_name = dentry.as_ref().unwrap().file_name();
            let child = Arc::new(Fat32Inode::new(dentry.unwrap(), Some(meta)));
            child.metadata().inner.lock().page_cache = Some(PageCache::new(child.clone(), 3));
            meta_inner.children.insert(file_name, child);
        }
    }

    fn delete_child(&self, child_name: &str) {
        debug!("Try to delete child: {} in root inode", child_name);
        if self.fs.fat_fs.root_dir().remove(child_name).is_err() {
            error!("Error in delete child in root inode");
        };
    }

    fn mknod(
        &self,
        this: Arc<dyn Inode>,
        pathname: &str,
        mode: InodeMode,
        dev_id: usize,
    ) -> GeneralRet<()> {
        debug!("fatfs mknod: {}", pathname);

        let name = pathname.to_string();
        let _new_file = self.fs.fat_fs.root_dir().create_file(&name).unwrap();
        let func = || {
            for dentry in self.fs.fat_fs.root_dir().iter() {
                if dentry.as_ref().unwrap().file_name() == name {
                    return Some(dentry.unwrap());
                }
            }
            return None;
        };
        let new_dentry = func();
        let mut new_inode = Fat32Inode {
            dentry: new_dentry.unwrap(),
            meta: None,
        };
        new_inode.init(Some(this.clone()), pathname, mode, 0)?;
        let key = new_inode.metadata().inner.lock().hash_name.name_hash as usize;
        let new_inode = Arc::new(new_inode);
        INODE_CACHE.lock().insert(key, new_inode.clone());
        this.metadata()
            .inner
            .lock()
            .children
            .insert(new_inode.metadata().name.clone(), new_inode);
        Ok(())
    }

    fn mkdir(&self, this: Arc<dyn Inode>, pathname: &str, mode: InodeMode) -> GeneralRet<()> {
        debug!("fatfs mkdir: {}", pathname);

        let name = pathname.to_string();
        let _new_dir = self.fs.fat_fs.root_dir().create_dir(&name).unwrap();
        let func = || {
            for dentry in self.fs.fat_fs.root_dir().iter() {
                if dentry.as_ref().unwrap().file_name() == name {
                    return Some(dentry.unwrap());
                }
            }
            return None;
        };
        let new_dentry = func();
        let mut new_inode = Fat32Inode {
            dentry: new_dentry.unwrap(),
            meta: None,
        };
        new_inode.init(Some(this.clone()), pathname, mode, 0)?;
        let key = new_inode.metadata().inner.lock().hash_name.name_hash as usize;
        let new_inode = Arc::new(new_inode);
        INODE_CACHE.lock().insert(key, new_inode.clone());
        this.metadata()
            .inner
            .lock()
            .children
            .insert(new_inode.metadata().name.clone(), new_inode);
        Ok(())
    }
}

// pub enum Fat32NodeType {
//     Dir(fatfs::Dir<'static, IoDevice, fatfs::DefaultTimeProvider, fatfs::LossyOemCpConverter>),
//     File(fatfs::File<'static, IoDevice, fatfs::DefaultTimeProvider, fatfs::LossyOemCpConverter>),
// }

pub struct Fat32Inode {
    dentry: DirEntry<'static, IoDevice, fatfs::DefaultTimeProvider, fatfs::LossyOemCpConverter>,
    meta: Option<InodeMeta>,
}

impl Fat32Inode {
    pub fn new(
        dentry: DirEntry<'static, IoDevice, fatfs::DefaultTimeProvider, fatfs::LossyOemCpConverter>,
        meta: Option<InodeMeta>,
    ) -> Self {
        Self { dentry, meta }
    }
}

impl Inode for Fat32Inode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        debug!("[Fat32Inode]: open: name: {}", self.dentry.file_name());
        let file_meta = FileMeta {
            // TODO: not sure whether this file_name() is absolute path or not
            path: self.dentry.file_name(),
            flags,
            inner: Mutex::new(FileMetaInner {
                inode: Some(this),
                pos: 0,
            }),
        };
        Ok(Arc::new(DefaultFile::new(file_meta)))
    }

    fn metadata(&self) -> &InodeMeta {
        &self.meta.as_ref().unwrap()
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.meta = Some(meta);
    }

    fn load_children(&self, this: Arc<dyn Inode>) {
        if self.dentry.is_file() {
            panic!("Cannot load a file's children");
        }
        debug!("[Fat32Inode]: load children");
        let mut meta_inner = self.meta.as_ref().unwrap().inner.lock();
        for dentry in self.dentry.to_dir().iter() {
            let inode_mode = {
                if dentry.as_ref().unwrap().is_dir() {
                    InodeMode::FileDIR
                } else {
                    InodeMode::FileREG
                }
            };
            let data_len = dentry.as_ref().unwrap().len();
            // let dentry_tmp = dentry.as_ref().cloned();
            let file_name = dentry.as_ref().unwrap().file_name();
            let meta = InodeMeta::new(
                Some(this.clone()),
                &file_name,
                inode_mode,
                data_len as usize,
            );
            let file_name = dentry.as_ref().unwrap().file_name();
            let child = Arc::new(Fat32Inode::new(dentry.unwrap(), Some(meta)));
            child.metadata().inner.lock().page_cache = Some(PageCache::new(child.clone(), 3));
            meta_inner.children.insert(file_name, child);
        }
    }

    fn delete_child(&self, child_name: &str) {
        debug!("Try to delete child: {} in normal inode", child_name);
        if self.dentry.to_dir().remove(child_name).is_err() {
            error!("Error in deleting child in normal inode");
        };
    }

    fn mknod(
        &self,
        this: Arc<dyn Inode>,
        pathname: &str,
        mode: InodeMode,
        dev_id: usize,
    ) -> GeneralRet<()> {
        debug!("fatfs mknod: {}", pathname);

        let name = pathname.to_string();
        if self.dentry.is_file() {
            return Err(SyscallErr::ENOTDIR);
        }
        let _new_file = self.dentry.to_dir().create_file(&name).unwrap();
        let func = || {
            for dentry in self.dentry.to_dir().iter() {
                if dentry.as_ref().unwrap().file_name() == name {
                    return Some(dentry.unwrap());
                }
            }
            return None;
        };
        let new_dentry = func();
        let mut new_inode = Fat32Inode {
            dentry: new_dentry.unwrap(),
            meta: None,
        };
        new_inode.init(Some(this.clone()), pathname, mode, 0)?;
        let key = new_inode.metadata().inner.lock().hash_name.name_hash as usize;
        let new_inode = Arc::new(new_inode);
        INODE_CACHE.lock().insert(key, new_inode.clone());
        this.metadata()
            .inner
            .lock()
            .children
            .insert(new_inode.metadata().name.clone(), new_inode);
        Ok(())
    }

    fn read(&self, offset: usize, buf: &mut [u8]) -> GeneralRet<usize> {
        if self.dentry.is_dir() {
            return Err(SyscallErr::EISDIR);
        }
        let mut file = self.dentry.to_file();
        if file.seek(fatfs::SeekFrom::Start(offset as u64)).is_err() {
            return Err(SyscallErr::EINVAL);
        }
        if let Some(bytes) = file.read(buf).ok() {
            return Ok(bytes);
        } else {
            warn!("fatfs read file failed!");
            return Err(SyscallErr::EINVAL);
        }
    }

    fn write(&self, offset: usize, buf: &[u8]) -> GeneralRet<usize> {
        if self.dentry.is_dir() {
            return Err(SyscallErr::EISDIR);
        }
        let mut file = self.dentry.to_file();
        if file.seek(fatfs::SeekFrom::Start(offset as u64)).is_err() {
            return Err(SyscallErr::EINVAL);
        }
        if let Some(bytes) = file.write(buf).ok() {
            return Ok(bytes);
        } else {
            warn!("fatfs write file failed!");
            return Err(SyscallErr::EINVAL);
        }
    }
}


lazy_static! {
    pub static ref ROOT_FS: Fat32FileSystem = {
        debug!("ROOT_FS: start to init...");
        let buffer_cache = IoDevice::new(BLOCK_DEVICE.clone());
        let ret = Fat32FileSystem::new(buffer_cache);
        debug!("ROOT_FS: init finished");
        ret
    };
}

/// List all files in the filesystems
pub fn list_apps_fat32() {
    info!("/************** APPS ****************/");
    for app in ROOT_FS.fat_fs.root_dir().iter() {
        info!("{}", app.unwrap().file_name());
    }
    info!("/************************************/");
}

pub fn init() -> GeneralRet<()> {
    info!("start to init fatfs...");

    // unsafe {
    //     let root_fs = &mut (*(&ROOT_FS as *const Fat32FileSystem as *mut Fat32FileSystem));
    //     ROOT_FS.init("/", FileSystemType::VFAT).unwrap();
    // }
    ROOT_FS.init_ref("/", FileSystemType::VFAT)?;
    let root_inode = ROOT_FS.metadata().root_inode.unwrap();
    root_inode.mkdir(root_inode.clone(), "mnt", InodeMode::FileDIR)?;

    // FILE_SYSTEM_MANAGER
    //     .fs_mgr
    //     .lock()
    //     .insert("/".to_string(), Arc::new(test_fs));
    info!("init fatfs success");

    Ok(())
}
