use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use crate::{
    driver::BlockDevice,
    fs::{hash_key::HashKey, inode::INODE_CACHE},
    sync::mutex::SpinNoIrqLock,
    utils::{
        async_utils::block_on,
        error::{AgeneralRet, GeneralRet, SyscallErr},
        path,
    },
};

use super::{
    devfs::DevFs, ffi::StatFlags, inode::InodeDevice, procfs::ProcFs, tmpfs::TmpFs,
    FAT32FileSystem, Inode,
};

#[derive(Clone)]
pub enum FsDevice {
    BlockDevice(Arc<dyn BlockDevice>),
    None,
}

impl FsDevice {
    pub fn from_inode_device(dev: InodeDevice) -> Self {
        match dev {
            // InodeDevice::Pipe(_) => Self::None,
            InodeDevice::Device(d) => Self::BlockDevice(d.block_device),
            InodeDevice::LoopDevice(d) => Self::BlockDevice(d),
        }
    }

    pub fn block_device(&self) -> Option<&Arc<dyn BlockDevice>> {
        if let FsDevice::BlockDevice(ret) = &self {
            Some(ret)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub enum FileSystemType {
    VFAT,
    EXT2,
    NFS,
    TmpFS,
    DevTmpFS,
    Proc,
}

impl FileSystemType {
    pub fn fs_type(ftype: &str) -> Self {
        match ftype {
            "vfat" => Self::VFAT,
            "ext2" => Self::EXT2,
            "nfs" => Self::NFS,
            "proc" => Self::Proc,
            "devtmpfs" => Self::DevTmpFS,
            _ => panic!("fstype {} not valid!", ftype),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::VFAT => "vfat".to_string(),
            Self::EXT2 => "ext2".to_string(),
            Self::NFS => "nfs".to_string(),
            Self::TmpFS => "tmpfs".to_string(),
            Self::DevTmpFS => "devtmpfs".to_string(),
            Self::Proc => "proc".to_string(),
        }
    }
}

/// concrete fs must implement `Drop` if sync disk is needed.
pub trait FileSystem: Send + Sync {
    fn mounts_info(&self) -> String {
        let meta = self.metadata();
        let dev_name = meta.dev_name.to_string();
        let mount_point = meta.mount_point.as_str();
        let fstype = meta.fstype;
        let flags = meta.flags;
        let buf_str = dev_name
            + " "
            + mount_point
            + " "
            + fstype.to_string().as_str()
            + " "
            + flags.to_string().as_str()
            + " 0 0\n";
        buf_str
    }

    fn metadata(&self) -> &FileSystemMeta;

    fn sync_fs<'a>(&self) -> AgeneralRet<'a, ()> {
        self.metadata().root_inode.clone().sync()
    }
}

#[derive(Clone)]
pub struct FileSystemMeta {
    /// device name
    pub dev_name: String,
    /// mount point path
    pub mount_point: String,
    /// filesystem type
    pub fstype: FileSystemType,
    /// filesystem flags
    pub flags: StatFlags,
    /// root of the filesystem
    pub root_inode: Arc<dyn Inode>,
    /// fa inode of mount point
    pub fa_inode: Option<Arc<dyn Inode>>,
    /// covered_inode of root
    pub covered_inode: Option<Arc<dyn Inode>>,
    /// covered_fs
    pub covered_fs: Option<Arc<dyn FileSystem>>,
    /// list of dirty inodes
    pub s_dirty: Vec<Arc<dyn Inode>>,
}

pub struct FileSystemManager {
    /// `mount point path` -> concrete file system
    pub fs_mgr: SpinNoIrqLock<BTreeMap<String, Arc<dyn FileSystem>>>,
}

impl FileSystemManager {
    pub const fn new() -> Self {
        Self {
            fs_mgr: SpinNoIrqLock::new(BTreeMap::new()),
        }
    }

    pub fn root_fs(&self) -> Arc<dyn FileSystem> {
        Arc::clone(&self.fs_mgr.lock().get("/").unwrap())
    }

    pub fn root_inode(&self) -> Arc<dyn Inode> {
        Arc::clone(&self.root_fs().metadata().root_inode)
    }

    pub fn mounts_info(&self) -> String {
        let mut res = "".to_string();
        let fs_mgr = self.fs_mgr.lock();
        for (mount_point, fs) in fs_mgr.iter() {
            res += fs.metadata().dev_name.as_str();
            res += " ";
            res += mount_point.as_str();
            res += " ";
            res += fs.metadata().fstype.to_string().as_str();
            res += " ";
            res += fs.metadata().flags.to_string().as_str();
            res += " 0 0\n";
        }
        res
    }

    pub fn mount(
        &self,
        mount_point: &str, // must be absolute path
        dev_name: &str,
        device: FsDevice,
        fstype: FileSystemType,
        flags: StatFlags,
    ) -> GeneralRet<Arc<dyn FileSystem>> {
        // find covered inode, fa inode, etc
        let mount_point_fa = path::get_parent_dir(mount_point);
        let mount_point_name = path::get_name(mount_point);
        let fa_inode;
        let covered_inode;
        let covered_fs;
        let fa_ino;
        if let Some(mount_point_fa) = mount_point_fa {
            (fa_inode, _) = <dyn Inode>::lookup_from_root(&mount_point_fa)?;
            if fa_inode.is_none() {
                log::warn!(
                    "[mount] parent inode doesn't exist, name {}",
                    mount_point_fa
                );
                return Err(SyscallErr::EEXIST);
            }
            let fa_inode_unwrap = Arc::clone(fa_inode.as_ref().unwrap());
            fa_ino = fa_inode_unwrap.metadata().ino;
            let maybe_covered_inode = fa_inode_unwrap.lookup(mount_point_name)?;
            covered_fs = self.fs_mgr.lock().get(mount_point).cloned();
            if maybe_covered_inode.is_none() {
                return Err(SyscallErr::EEXIST);
            }
            covered_inode = Some(maybe_covered_inode.unwrap());
        } else {
            fa_inode = None;
            covered_inode = None;
            covered_fs = None;
            fa_ino = 0;
        }
        let key = HashKey::new(fa_ino, mount_point_name.to_string());
        log::debug!("[mount] mount point {}, hash key {:?}", mount_point, key);
        // remove covered inode, but hashmap store only newest one, so maybe it's useless.
        /*
        if covered_inode.is_some() {
            INODE_CACHE.lock().remove(&key);
        }
        */
        // create fs
        let fs: Arc<dyn FileSystem> = match fstype {
            FileSystemType::VFAT => {
                let ret = FAT32FileSystem::new(
                    Arc::clone(device.block_device().unwrap()),
                    mount_point,
                    dev_name,
                    fstype,
                    flags,
                    fa_inode,
                    covered_inode,
                    covered_fs,
                )?;
                Arc::new(ret)
            }

            FileSystemType::DevTmpFS => {
                let ret = DevFs::new(
                    mount_point,
                    dev_name,
                    fstype,
                    flags,
                    fa_inode,
                    covered_inode,
                    covered_fs,
                )?;
                Arc::new(ret)
            }
            FileSystemType::Proc => {
                let ret = ProcFs::new(
                    mount_point,
                    dev_name,
                    fstype,
                    flags,
                    fa_inode,
                    covered_inode,
                    covered_fs,
                )?;
                Arc::new(ret)
            }
            FileSystemType::TmpFS => {
                let ret = TmpFs::new(
                    mount_point,
                    dev_name,
                    fstype,
                    flags,
                    fa_inode,
                    covered_inode,
                    covered_fs,
                )?;
                Arc::new(ret)
            }
            _ => todo!(),
        };
        // insert root inode into inode cache
        let meta = fs.metadata();
        INODE_CACHE.insert(key.clone(), Arc::clone(&meta.root_inode));
        log::info!(
            "[mount] mount point {} inode ino {}",
            mount_point,
            meta.root_inode.metadata().ino
        );
        // insert file system into file system manager
        let mut fs_locked = self.fs_mgr.lock();
        fs_locked.insert(mount_point.to_string(), Arc::clone(&fs));

        #[cfg(feature = "async_flush")]
        {
            // Write back in background
            let fs_moved = fs.clone();
            crate::process::thread::spawn_kernel_thread(async move {
                loop {
                    crate::timer::timeout_task::ksleep(core::time::Duration::from_secs(5)).await;
                    // log::error!("I'm going to write back!!");
                    if fs_moved.sync_fs().await.is_err() {
                        log::info!(
                            "[fs write back] fs {} must have already been umounted",
                            fs_moved.metadata().mount_point
                        );
                        break;
                    }
                }
            });
        }

        Ok(fs)
    }

    /// TODO: change into async
    pub fn unmount(&self, mount_point: &str) -> GeneralRet<()> {
        // find fs
        let fs = {
            let fs_mgr = self.fs_mgr.lock();
            let maybe_fs = fs_mgr.get(mount_point);
            if maybe_fs.is_some() {
                Some(Arc::clone(maybe_fs.unwrap()))
            } else {
                None
            }
        };
        if fs.is_none() {
            return Err(SyscallErr::EEXIST);
        }
        let fs = fs.unwrap();

        // TODO: this may lead to dead lock since the async function may try to
        // hold a sleeplock.
        let fs_moved = fs.clone();
        block_on(async move {
            if fs_moved.sync_fs().await.is_err() {
                log::error!(
                    "[umount] fs {} must have already been umounted",
                    fs_moved.metadata().mount_point
                );
            }
        });

        let meta = fs.metadata();
        // remove root inode from inode cache
        let fa_ino = {
            if let Some(some_fa_inode) = meta.fa_inode.as_ref() {
                // remove root inode from fa
                some_fa_inode.remove_child(Arc::clone(&meta.root_inode))?;
                some_fa_inode.metadata().ino
            } else {
                0
            }
        };
        let mount_point_name = path::get_name(mount_point);
        let key = HashKey::new(fa_ino, mount_point_name.to_string());
        INODE_CACHE.remove(&key);
        // remove file system from file system manager
        self.fs_mgr.lock().remove(mount_point);
        if meta.covered_inode.is_some() {
            INODE_CACHE.insert(key, Arc::clone(&meta.covered_inode.as_ref().unwrap()));
        }
        if meta.covered_fs.is_some() {
            self.fs_mgr.lock().insert(
                mount_point.to_string(),
                meta.covered_fs.as_ref().unwrap().clone(),
            );
        }
        Ok(())
        // fs will be dropped automatically because Arc = 0
    }
}

pub static FILE_SYSTEM_MANAGER: FileSystemManager = FileSystemManager::new();
