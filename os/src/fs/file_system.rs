use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use lazy_static::*;
use log::debug;

use crate::{
    fs::inode::INODE_CACHE,
    stack_trace,
    sync::mutex::SpinNoIrqLock,
    utils::{
        error::{GeneralRet, SyscallRet},
        path::Path,
    },
};

use super::{testfs::TestFs, FAT32FileSystem, Inode, Mutex};

#[derive(Clone)]
pub enum FileSystemType {
    VFAT,
    EXT2,
    NFS,
}

impl FileSystemType {
    pub fn fs_type(ftype: String) -> Option<Self> {
        match ftype {
            vfat => Some(Self::VFAT),
            ext2 => Some(Self::EXT2),
            nfs => Some(Self::NFS),
            _ => None,
        }
    }
    pub fn new_fs(&self) -> impl FileSystem {
        match self {
            Self::VFAT => {
                // let fs = FAT32FileSystem::new();
                let fs = TestFs::new();
                fs
            }
            _ => {
                todo!()
            }
        }
    }
}

pub trait FileSystem: Send + Sync {
    fn create_root(
        &self,
        parent: Option<Arc<dyn Inode>>,
        mount_point: &str,
    ) -> GeneralRet<Arc<dyn Inode>>;

    fn init_ref(&self, mount_point: &str, ftype: FileSystemType) -> GeneralRet<()> {
        debug!("start to init fs, mount point {}", mount_point);
        let parent_dir = Path::get_parent_dir(mount_point);
        debug!("parent dir {:?}", parent_dir);
        let parent = {
            if parent_dir.is_none() {
                None
            } else {
                debug!("parent dir {}", parent_dir.as_ref().unwrap());
                <dyn Inode>::lookup_from_root_tmp(&parent_dir.unwrap())
            }
        };

        debug!("start to create root inode...");
        let root_inode = self.create_root(parent, mount_point)?;
        debug!("create root inode success");

        // root_inode.init(parent, mount_point)?;
        let key = root_inode.metadata().inner.lock().hash_name.name_hash as usize;

        let root_inode = {
            INODE_CACHE.lock().insert(key, root_inode.clone());
            INODE_CACHE.lock().get(&key).unwrap().clone()
        };

        let meta = FileSystemMeta {
            ftype,
            root_inode: Some(root_inode),
            mnt_flags: false,
            s_dirty: Vec::new(),
        };

        self.set_metadata_ref(meta);

        Ok(())
    }

    fn init(&mut self, mount_point: &str, ftype: FileSystemType) -> GeneralRet<()> {
        debug!("start to init fs, mount point {}", mount_point);
        let parent_dir = Path::get_parent_dir(mount_point);
        debug!("parent dir {:?}", parent_dir);
        stack_trace!();
        let parent = {
            if parent_dir.is_none() {
                None
            } else {
                debug!("parent dir {}", parent_dir.as_ref().unwrap());
                <dyn Inode>::lookup_from_root_tmp(&parent_dir.unwrap())
            }
        };

        debug!("start to create root inode...");
        let root_inode = self.create_root(parent, mount_point)?;
        debug!("create root inode success");

        // root_inode.init(parent, mount_point)?;
        let key = root_inode.metadata().inner.lock().hash_name.name_hash as usize;

        let root_inode = {
            INODE_CACHE.lock().insert(key, root_inode.clone());
            INODE_CACHE.lock().get(&key).unwrap().clone()
        };

        let meta = FileSystemMeta {
            ftype,
            root_inode: Some(root_inode),
            mnt_flags: false,
            s_dirty: Vec::new(),
        };

        self.set_metadata(meta);

        Ok(())
    }
    fn mount(&self) {
        stack_trace!();
        let mut meta = self.metadata();
        meta.mnt_flags = true;
        self.set_metadata_ref(meta);
    }
    fn dirty_inode(&self, inode: Arc<dyn Inode>) {
        let mut meta = self.metadata();
        meta.s_dirty.push(inode);
        self.set_metadata_ref(meta);
    }
    fn write_inode(&self, inode: Arc<dyn Inode>) -> SyscallRet {
        todo!()
    }
    fn sync_fs(&self) -> SyscallRet {
        todo!()
    }
    fn set_metadata(&mut self, metadata: FileSystemMeta);
    fn set_metadata_ref(&self, metadata: FileSystemMeta) {
        todo!()
    }
    fn metadata(&self) -> FileSystemMeta;
}

#[derive(Clone)]
pub struct FileSystemMeta {
    pub ftype: FileSystemType,
    // /// root of the filesystem
    // pub root: Option<Arc<dyn Dentry>>,
    pub root_inode: Option<Arc<dyn Inode>>,
    // pub inner: Mutex<FileSystemMetaInner>,
    /// flag of the filesystem whether mount
    pub mnt_flags: bool,
    /// list of dirty inodes
    pub s_dirty: Vec<Arc<dyn Inode>>,
}

pub struct FileSystemMetaInner {
    /// flag of the filesystem whether mount
    mnt_flags: bool,
    /// list of dirty inodes
    pub s_dirty: Vec<Arc<dyn Inode>>,
}

pub struct FileSystemManager {
    /// `mount point` -> concrete file system
    pub fs_mgr: SpinNoIrqLock<BTreeMap<String, Arc<dyn FileSystem>>>,
}

impl FileSystemManager {
    pub fn new() -> Self {
        Self {
            fs_mgr: SpinNoIrqLock::new(BTreeMap::new()),
        }
    }
    // pub fn match_file_system(&self, path: &str) -> GeneralRet<Arc<dyn FileSystem>> {
    //     todo!()
    // }
}

lazy_static! {
    pub static ref FILE_SYSTEM_MANAGER: FileSystemManager = FileSystemManager::new();
}

/// Resolve the given path
pub fn resolve_path(path: &str) -> GeneralRet<Arc<dyn Inode>> {
    todo!()
}
