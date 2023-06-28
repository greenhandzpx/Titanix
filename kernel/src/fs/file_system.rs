use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use lazy_static::*;
use log::debug;

use crate::{
    fs::{hash_key::HashKey, inode::INODE_CACHE, InodeMode},
    stack_trace,
    sync::mutex::SpinNoIrqLock,
    utils::{
        error::{GeneralRet, SyscallRet},
        path,
    },
};

use super::{posix::StatFlags, testfs::TestFs, Inode};

#[derive(Clone)]
pub enum FileSystemType {
    VFAT,
    EXT2,
    NFS,
}

impl FileSystemType {
    pub fn fs_type(ftype: &str) -> Option<Self> {
        match ftype {
            "vfat" => Some(Self::VFAT),
            "ext2" => Some(Self::EXT2),
            "nfs" => Some(Self::NFS),
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
    pub fn to_string(&self) -> String {
        match self {
            Self::VFAT => "FAT32".to_string(),
            Self::EXT2 => "EXT2".to_string(),
            Self::NFS => "NFS".to_string(),
            _ => "".to_string(),
        }
    }
}

pub trait FileSystem: Send + Sync {
    fn create_root(
        &self,
        parent: Option<Arc<dyn Inode>>,
        mount_point: &str,
    ) -> GeneralRet<Arc<dyn Inode>>;

    fn init_ref(
        &self,
        dev_name: String,
        mount_point: &str,
        ftype: FileSystemType,
        flags: StatFlags,
    ) -> GeneralRet<()> {
        debug!("start to init fs, mount point {}", mount_point);
        let parent_dir = path::get_parent_dir(mount_point);
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
        let root_inode = self.create_root(parent.clone(), mount_point)?;
        debug!("create root inode success");

        let parent_ino = {
            if parent.is_none() {
                0
            } else {
                parent.unwrap().metadata().ino
            }
        };
        let child_name = path::get_name(mount_point);
        let key = HashKey::new(parent_ino, child_name.to_string());

        let root_inode = {
            INODE_CACHE.lock().insert(key.clone(), root_inode.clone());
            INODE_CACHE.lock().get(&key).unwrap().clone()
        };

        let meta = FileSystemMeta {
            dev_name,
            ftype,
            root_inode: Some(root_inode),
            mnt_flags: false,
            s_dirty: Vec::new(),
            flags,
        };

        self.set_metadata_ref(meta);

        Ok(())
    }

    fn init(
        &mut self,
        dev_name: String,
        mount_point: &str,
        ftype: FileSystemType,
        flags: StatFlags,
    ) -> GeneralRet<()> {
        debug!("start to init fs, mount point {}", mount_point);
        let parent_dir = path::get_parent_dir(mount_point);
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
        let root_inode = self.create_root(parent.clone(), mount_point)?;
        debug!("create root inode success");

        let parent_ino = {
            if parent.is_none() {
                0
            } else {
                parent.unwrap().metadata().ino
            }
        };
        let child_name = path::get_name(mount_point);
        let key = HashKey::new(parent_ino, child_name.to_string());

        let root_inode = {
            INODE_CACHE.lock().insert(key.clone(), root_inode.clone());
            INODE_CACHE.lock().get(&key).unwrap().clone()
        };

        let meta = FileSystemMeta {
            dev_name,
            ftype,
            root_inode: Some(root_inode),
            mnt_flags: false,
            s_dirty: Vec::new(),
            flags,
        };

        self.set_metadata(meta);

        Ok(())
    }
    fn mounts_info(&self) -> String {
        let dev_name = self.metadata().dev_name.to_string();
        let root_inode = self.metadata().root_inode.unwrap();
        let mount_point = root_inode.metadata().path.as_str();
        let ftype = self.metadata().ftype;
        let flags = self.metadata().flags;
        let buf_str = dev_name
            + " "
            + mount_point
            + " "
            + ftype.to_string().as_str()
            + " "
            + flags.to_string().as_str()
            + " 0 0\n";
        buf_str
    }
    fn mount(&self) -> GeneralRet<isize> {
        stack_trace!();
        let mut meta = self.metadata();
        meta.mnt_flags = true;
        self.set_metadata_ref(meta);
        Ok(0)
    }
    fn umount(&self) -> GeneralRet<isize> {
        self.sync_fs()?;
        Ok(0)
    }
    fn dirty_inode(&self, inode: Arc<dyn Inode>) {
        let mut meta = self.metadata();
        meta.s_dirty.push(inode);
        self.set_metadata_ref(meta);
    }
    fn write_inode(&self, inode: Arc<dyn Inode>) -> SyscallRet {
        todo!()
    }
    fn sync_fs(&self) -> GeneralRet<isize> {
        todo!()
    }
    fn set_metadata(&mut self, metadata: FileSystemMeta);
    fn set_metadata_ref(&self, metadata: FileSystemMeta) {
        todo!()
    }
    fn metadata(&self) -> FileSystemMeta;
    // fn free_blocks(&self) -> u64;
}

#[derive(Clone)]
pub struct FileSystemMeta {
    /// device name
    pub dev_name: String,
    /// filesystem type
    pub ftype: FileSystemType,
    /// root of the filesystem
    pub root_inode: Option<Arc<dyn Inode>>,
    // pub inner: Mutex<FileSystemMetaInner>,
    /// flag of the filesystem whether mount
    pub mnt_flags: bool,
    /// list of dirty inodes
    pub s_dirty: Vec<Arc<dyn Inode>>,
    /// filesystem flags
    pub flags: StatFlags,
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
    pub fn mounts_info(&self) -> String {
        let mut res = "".to_string();
        let fs_mgr = self.fs_mgr.lock();
        for (_mount_point, fs) in fs_mgr.iter() {
            res += fs.mounts_info().as_str();
        }
        res
    }
    // pub fn match_file_system(&self, path: &str) -> GeneralRet<Arc<dyn FileSystem>> {
    //     todo!()
    // }
}

lazy_static! {
    pub static ref FILE_SYSTEM_MANAGER: FileSystemManager = FileSystemManager::new();
}
