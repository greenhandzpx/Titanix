use core::{
    cell::SyncUnsafeCell,
    mem::size_of,
    sync::atomic::{AtomicUsize, Ordering},
};

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::{Arc, Weak},
    vec::Vec,
};
use lazy_static::*;
use log::{debug, warn};

use crate::{
    driver::block::BlockDevice,
    mm::{Page, PageCache},
    timer::get_time_ms,
    utils::{
        error::{GeneralRet, SyscallRet},
        hash_table::HashTable,
        mem::uninit_memory,
        path::Path,
    },
};

use super::{
    devfs::DevWrapper,
    fat32_tmp::ROOT_FS,
    file::{DefaultFile, FileMeta, FileMetaInner},
    file_system::FILE_SYSTEM_MANAGER,
    hash_name::HashName,
    // dentry::{self, Dentry},
    // inode::OpenFlags,
    pipe::Pipe,
    File,
    FileSystem,
    Mutex,
    OpenFlags,
};

lazy_static! {
    /// Dcache: cache: (parent id, child name) -> dentry
    /// TODO: add max capacity limit and lru policy
    ///
    pub static ref INODE_CACHE: Mutex<HashTable<usize, Arc<dyn Inode>>> = Mutex::new(HashTable::new());
}
#[derive(PartialEq)]
pub enum InodeMode {
    FileSOCK = 0xC, /* socket */
    FileLNK = 0xA,  /* symbolic link */
    FileREG = 0x8,  /* regular file */
    FileBLK = 0x6,  /* block device */
    FileDIR = 0x4,  /* directory */
    FileCHR = 0x2,  /* character device */
    FileFIFO = 0x1, /* FIFO */
                    // TODO add more(like R / W / X etc)
}

static INODE_NUMBER: AtomicUsize = AtomicUsize::new(0);

static INODE_UID_ALLOCATOR: AtomicUsize = AtomicUsize::new(1);

pub trait Inode: Send + Sync {
    fn init(
        &mut self,
        parent: Option<Arc<dyn Inode>>,
        path: &str,
        mode: InodeMode,
        data_len: usize,
    ) -> GeneralRet<()> {
        debug!("start to init inode...");
        let meta = InodeMeta::new(parent, path, mode, data_len);
        self.set_metadata(meta);
        debug!("init inode finished");
        Ok(())
    }
    // fn create(&self, dentry: Arc<dyn Dentry>) -> GeneralRet<Arc<dyn Inode>> {
    //     todo!();
    // }
    // // // you should use the parent inode to call this function and give the target dentry name
    // // fn lookup(&self, target_name: &str) -> GeneralRet<Arc<dyn Dentry>>;
    // fn unlink(self: Arc<Self>, dentry: Arc<dyn Dentry>) -> SyscallRet {
    //     let count = Arc::strong_count(&self);
    //     if count > 1 {
    //         return SyscallRet::Err(crate::utils::error::SyscallErr::EBUSY);
    //     } else {
    //         // TODO: remove dentry, maybe not remove dentry in cache?
    //         return Ok(0);
    //     }
    // }
    // // TODO not sure what the args should be
    // fn rename(&self, old_dentry: &mut Dentry, new_inode: &mut Self, new_dentry: &mut Dentry) {
    //     todo!()
    // }

    /// Default operation is to open the default file(i.e. file from disk)
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        let file_meta = FileMeta {
            path: self.metadata().path.clone(),
            flags,
            inner: Mutex::new(FileMetaInner {
                inode: Some(this),
                pos: 0,
            }),
        };
        let file = DefaultFile::new(file_meta);
        Ok(Arc::new(file))
    }

    /// You should call this function through the parent inode
    /// You should give a relative path
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
    /// Read data at the given file offset from block device
    fn read(&self, offset: usize, buf: &mut [u8]) -> GeneralRet<usize> {
        todo!()
    }
    /// Write data to the given file offset in block device
    fn write(&self, offset: usize, buf: &[u8]) -> GeneralRet<usize> {
        todo!()
    }

    fn metadata(&self) -> &InodeMeta;
    fn set_metadata(&mut self, meta: InodeMeta);

    fn lookup(&self, this: Arc<dyn Inode>, name: &str) -> Option<Arc<dyn Inode>> {
        let key = HashName::hash_name(Some(self.metadata().uid), name).name_hash as usize;
        let value = INODE_CACHE.lock().get(&key).cloned();
        match value {
            Some(value) => Some(value.clone()),
            None => {
                debug!(
                    "cannot find child dentry, name: {}, try to find in inode",
                    name
                );
                let target_inode = self.try_find_and_insert_inode(this, name);
                match target_inode {
                    Some(target_inode) => Some(target_inode.clone()),
                    None => None,
                }
            }
        }
    }
    fn try_find_and_insert_inode(
        &self,
        this: Arc<dyn Inode>,
        child_name: &str,
    ) -> Option<Arc<dyn Inode>> {
        let key = HashName::hash_name(Some(self.metadata().uid), child_name).name_hash as usize;

        self.load_children(this);
        debug!(
            "children size {}",
            self.metadata().inner.lock().children.len()
        );
        let target_inode = self
            .metadata()
            .inner
            .lock()
            .children
            .get(child_name)
            .cloned();

        match target_inode {
            Some(target_inode) => {
                // find the inode which related to this subdentry
                INODE_CACHE.lock().insert(key, target_inode.clone());
                Some(target_inode.clone())
            }
            None => {
                debug!("Cannot find {} in children", child_name);
                None
            }
        }
    }
    /// unlink() system call will call this function.
    /// This function will delete the inode in inode cache and call delete() function to delete inode in disk.
    fn unlink(&self, child: Arc<dyn Inode>) -> GeneralRet<isize> {
        let key = child.metadata().inner.lock().hash_name.name_hash as usize;
        debug!("Try to delete child in INODE_CACHE");
        INODE_CACHE.lock().delete(key);
        let child_name = child.metadata().name.clone();
        self.metadata().inner.lock().children.remove(&child_name);
        self.delete_child(&child_name);
        Ok(0)
    }
    /// This function will delete the inode in inode cache.
    fn remove_child(&self, child: Arc<dyn Inode>) -> GeneralRet<isize> {
        let key = child.metadata().inner.lock().hash_name.name_hash as usize;
        debug!("Try to delete child in INODE_CACHE");
        INODE_CACHE.lock().delete(key);
        let child_name = child.metadata().name.clone();
        self.metadata().inner.lock().children.remove(&child_name);
        Ok(0)
    }

    /// Load the children dirs of the current dir
    /// TODO: It may be a bad idea to load all children at one time?
    fn load_children(&self, this: Arc<dyn Inode>);

    /// Delete inode in disk
    /// You should call this function through parent inode.
    /// TODO: This function should be implemented by actual filesystem.
    fn delete_child(&self, child_name: &str);
}

impl dyn Inode {
    /// Look up from root(e.g. "/home/oscomp/workspace")
    pub fn lookup_from_root(
        // file_system: Arc<dyn FileSystem>,
        path: &str,
    ) -> Option<Arc<dyn Inode>> {
        let path_names = Path::path2vec(path);
        // path_names.remove(0);

        let root_fs = FILE_SYSTEM_MANAGER
            .fs_mgr
            .lock()
            .get("/")
            .cloned()
            .expect("No root fs is mounted");

        let mut parent = root_fs.metadata().root_inode.clone().unwrap();

        for name in path_names {
            match parent.lookup(parent.clone(), name) {
                Some(p) => parent = p,
                None => return None,
            }
        }
        Some(parent)
    }
    /// Look up from root(e.g. "/home/oscomp/workspace")
    pub fn lookup_from_root_tmp(
        // file_system: Arc<dyn FileSystem>,
        path: &str,
    ) -> Option<Arc<dyn Inode>> {
        let path_names = Path::path2vec(path);
        // path_names.remove(0);

        let mut parent = ROOT_FS.metadata().root_inode.clone().unwrap();

        for name in path_names {
            match parent.lookup(parent.clone(), name) {
                Some(p) => parent = p,
                None => return None,
            }
        }
        Some(parent)
    }
}

pub struct InodeMeta {
    /// inode number
    pub ino: usize,
    /// data address
    pub data: usize,
    /// type of inode
    pub mode: InodeMode,
    // pub i_op: Arc<dyn InodeOperations + Sync + Send>,
    /// device id (only for block device and char device)
    pub rdev: Option<usize>,
    /// inode's device
    pub device: Option<InodeDevice>,
    /// path to this inode
    pub path: String,
    /// name which doesn't have slash
    pub name: String,
    /// a inode's unique id
    pub uid: usize,
    pub inner: Mutex<InodeMetaInner>,
}

pub struct InodeMetaInner {
    // pub offset: usize,
    /// inode' file's size
    pub size: usize,
    /// last access time, need to flush to disk.
    pub st_atime: i64,
    /// last modification time, need to flush to disk
    pub st_mtime: i64,
    /// last status change time, need to flush to disk
    pub st_ctime: i64,
    /// hash name(Note that this doesn't consider the parent uid)
    pub hash_name: HashName,
    /// parent
    pub parent: Option<Weak<dyn Inode>>,
    /// brother list
    pub brothers: BTreeMap<String, Weak<dyn Inode>>,
    /// children list
    pub children: BTreeMap<String, Arc<dyn Inode>>,
    /// page cache of the related file
    pub page_cache: Option<PageCache>,
    /// data len
    pub data_len: usize,
}

impl InodeMeta {
    pub fn new(
        parent: Option<Arc<dyn Inode>>,
        path: &str,
        mode: InodeMode,
        data_len: usize,
    ) -> Self {
        let name = Path::get_name(path);
        let parent_uid = match parent.as_ref() {
            Some(parent) => Some(parent.metadata().uid),
            None => None,
        };
        let parent = match parent {
            Some(parent) => Some(Arc::downgrade(&parent)),
            None => None,
        };
        Self {
            ino: 0,
            data: 0,
            mode,
            rdev: None,
            device: None,
            path: path.to_string(),
            name: name.to_string(),
            uid: INODE_UID_ALLOCATOR.fetch_add(1, Ordering::Relaxed),
            inner: Mutex::new(InodeMetaInner {
                size: 0,
                st_atime: (get_time_ms() / 1000) as i64,
                st_mtime: (get_time_ms() / 1000) as i64,
                st_ctime: (get_time_ms() / 1000) as i64,
                parent,
                brothers: BTreeMap::new(),
                children: BTreeMap::new(),
                hash_name: HashName::hash_name(parent_uid, name),
                page_cache: None,
                data_len,
            }),
        }
    }
}

pub enum InodeDevice {
    Pipe(Pipe),
    Device(Arc<DevWrapper>),
    // TODO: add more
}

pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<dyn Inode>> {
    // let inode = <dyn Inode>::lookup_from_root_tmp(name);
    let inode = <dyn Inode>::lookup_from_root_tmp(name);
    debug!("open file, name: {}", name);
    // inode
    if flags.contains(OpenFlags::CREATE) {
        if inode.is_some() {
            return inode;
        }
        let parent_path = Path::get_parent_dir(name).unwrap();
        let parent = <dyn Inode>::lookup_from_root_tmp(&parent_path);
        let child_name = Path::get_name(name);
        if let Some(parent) = parent {
            debug!("create file {}", name);
            if flags.contains(OpenFlags::DIRECTORY) {
                parent
                    .mkdir(parent.clone(), child_name, InodeMode::FileDIR)
                    .unwrap();
            } else {
                // TODO dev id
                parent
                    .mknod(parent.clone(), child_name, InodeMode::FileREG, 0)
                    .unwrap();
            }
            <dyn Inode>::lookup_from_root_tmp(name)
        } else {
            warn!("parent dir {} doesn't exist", parent_path);
            return None;
        }
    } else {
        inode
    }
}
