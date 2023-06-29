use core::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::{Arc, Weak},
};
use hashbrown::HashMap;
use lazy_static::*;
use log::{debug, info, warn};

use crate::{
    driver::block::BlockDevice,
    mm::PageCache,
    timer::posix::TimeSpec,
    utils::{
        error::{AgeneralRet, GeneralRet},
        path,
    },
};

use super::{
    file::{DefaultFile, FileMeta, FileMetaInner},
    file_system::FILE_SYSTEM_MANAGER,
    hash_key::HashKey,
    // dentry::{self, Dentry},
    // inode::OpenFlags,
    pipe::Pipe,
    File,
    FileSystem,
    Mutex,
    OpenFlags,
};

lazy_static! {
    /// Dcache: cache: (parent ino, child name) -> dentry
    /// TODO: add max capacity limit and lru policy
    ///
    pub static ref INODE_CACHE: Mutex<HashMap<HashKey, Arc<dyn Inode>>> = Mutex::new(HashMap::new());
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InodeMode {
    FileSOCK = 0xC000, /* socket */
    FileLNK = 0xA000,  /* symbolic link */
    FileREG = 0x8000,  /* regular file */
    FileBLK = 0x6000,  /* block device */
    FileDIR = 0x4000,  /* directory */
    FileCHR = 0x2000,  /* character device */
    FileFIFO = 0x1000, /* FIFO */
                       // TODO add more(like R / W / X etc)
}

/// Inode state flags
#[derive(Clone, Copy, Debug)]
pub enum InodeState {
    /// init, the inode may related to an inode in disk, but not load data from disk
    Init = 0x1,
    /// inode dirty, data which is pointed to by inode is not dirty
    DirtyInode = 0x2,
    /// data already changed but not yet sync (inode not change)
    DirtyData = 0x3,
    /// inode and date changed together
    DirtyAll = 0x4,
    /// already sync
    Synced = 0x5,
}

static INODE_NUMBER: AtomicUsize = AtomicUsize::new(0);

pub trait Inode: Send + Sync {
    fn init(
        &mut self,
        parent: Option<Arc<dyn Inode>>,
        path: &str,
        mode: InodeMode,
        data_len: usize,
    ) -> GeneralRet<()> {
        debug!("start to init inode...");
        let meta = InodeMeta::new(parent, path, mode, data_len, None);
        self.set_metadata(meta);
        debug!("init inode finished");
        Ok(())
    }

    /// Default operation is to open the default file(i.e. file from disk)
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        debug!("[inode] open");
        let file_meta = FileMeta {
            path: self.metadata().path.clone(),
            inner: Mutex::new(FileMetaInner {
                flags,
                inode: Some(this),
                pos: 0,
                dirent_index: 0,
            }),
        };
        let file = DefaultFile::new(file_meta);
        Ok(Arc::new(file))
    }

    /// You should call this function through the parent inode
    /// You should give a absolute path
    fn mkdir(
        &self,
        _this: Arc<dyn Inode>,
        _pathname: &str,
        _mode: InodeMode,
    ) -> GeneralRet<Arc<dyn Inode>> {
        todo!()
    }
    fn rmdir(&self, _name: &str, _mode: InodeMode) -> GeneralRet<()> {
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
    /// Read data at the given file offset from block device
    fn read<'a>(&'a self, _offset: usize, _buf: &'a mut [u8]) -> AgeneralRet<usize> {
        todo!()
    }
    /// Write data to the given file offset in block device
    fn write<'a>(&'a self, _offset: usize, _buf: &'a [u8]) -> AgeneralRet<usize> {
        todo!()
    }

    fn metadata(&self) -> &InodeMeta;
    fn set_metadata(&mut self, meta: InodeMeta);

    fn lookup(&self, this: Arc<dyn Inode>, name: &str) -> Option<Arc<dyn Inode>> {
        let key = HashKey::new(self.metadata().ino, name.to_string());
        let value = INODE_CACHE.lock().get(&key).cloned();
        match value {
            Some(value) => Some(value.clone()),
            None => {
                debug!(
                    "[lookup] cannot find child dentry, name: {}, try to find in inode",
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
        let children = self.metadata().inner.lock().children.clone();
        let target_inode = children.get(child_name).cloned();
        if target_inode.is_some() {
            debug!("[try_find_and_insert_inode] find in children");
            return target_inode;
        }
        <dyn Inode>::load_children(this.clone());
        debug!(
            "[try_find_and_insert_inode] children size {}",
            self.metadata().inner.lock().children.len()
        );

        let key = HashKey::new(self.metadata().ino, child_name.to_string());
        let target_inode = INODE_CACHE.lock().get(&key).cloned();

        match target_inode {
            Some(target_inode) => {
                // find the inode which related to this subdentry
                Some(target_inode)
            }
            None => {
                debug!(
                    "[try_find_and_insert_inode] Cannot find {} in children",
                    child_name
                );
                None
            }
        }
    }
    /// unlink() system call will call this function.
    /// This function will delete the inode in inode cache and call delete() function to delete inode in disk.
    fn unlink(&self, child: Arc<dyn Inode>) -> GeneralRet<isize> {
        let key = HashKey::new(self.metadata().ino, child.metadata().name.clone());
        debug!("Try to delete child in INODE_CACHE");
        INODE_CACHE.lock().remove(&key);
        let child_name = child.metadata().name.clone();
        self.metadata().inner.lock().children.remove(&child_name);
        self.delete_child(&child_name);
        Ok(0)
    }
    /// This function will delete the inode in cache (which means delete inode in parent's children list).
    fn remove_child(&self, child: Arc<dyn Inode>) -> GeneralRet<isize> {
        let key = HashKey::new(self.metadata().ino, child.metadata().name.clone());
        debug!("Try to delete child in INODE_CACHE");
        INODE_CACHE.lock().remove(&key);
        let child_name = child.metadata().name.clone();
        self.metadata().inner.lock().children.remove(&child_name);
        Ok(0)
    }

    /// Load the children dirs of the current dir.
    /// The state of inode loaded from disk should be synced
    /// TODO: It may be a bad idea to load all children at one time?
    fn load_children_from_disk(&self, this: Arc<dyn Inode>);

    /// Delete inode in disk.
    /// You should call this function through parent inode.
    /// TODO: This function should be implemented by actual filesystem.
    fn delete_child(&self, child_name: &str);


    fn sync(&self);
}

impl dyn Inode {
    /// Load children and insert them into INODE_CACHE
    pub fn load_children(parent: Arc<dyn Inode>) {
        debug!("[load_children] enter");
        let state = parent.metadata().inner.lock().state;
        debug!("[load_children] inode state: {:?}", state);
        match state {
            InodeState::Init => {
                // load children from disk
                parent.load_children_from_disk(parent.clone());
                parent.metadata().inner.lock().state = InodeState::Synced;
                let mut cache_lock = INODE_CACHE.lock();
                let children = parent.metadata().inner.lock().children.clone();
                for child in children {
                    debug!(
                        "[load_children] insert to INODE_CACHE, name: {}",
                        child.1.metadata().name
                    );
                    let key = HashKey::new(parent.metadata().ino, child.1.metadata().name.clone());
                    cache_lock.insert(key, child.1);
                }
            }
            _ => {
                // do nothing
            }
        }
        debug!("[load_children] leave");
    }
    /// Look up from root(e.g. "/home/oscomp/workspace")
    pub fn lookup_from_root(
        // file_system: Arc<dyn FileSystem>,
        path: &str,
    ) -> Option<Arc<dyn Inode>> {
        let path_names = path::path2vec(path);
        // path_names.remove(0);
        let mut parent = Arc::clone(&FILE_SYSTEM_MANAGER.root_inode());

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
        let path_names = path::path2vec(path);
        // path_names.remove(0);

        let mut parent = Arc::clone(&FILE_SYSTEM_MANAGER.root_inode());

        for (i, name) in path_names.into_iter().enumerate() {
            debug!("[lookup_from_root_tmp] round: {}, name: {}", i, name);
            match parent.lookup(parent.clone(), name) {
                Some(p) => {
                    debug!("[lookup_from_root_tmp] inode name: {}", p.metadata().name);
                    parent = p
                }
                None => return None,
            }
        }
        Some(parent)
    }

    pub fn create_page_cache_if_needed(this: Arc<dyn Inode>) {
        let mut meta_locked = this.metadata().inner.lock();
        if meta_locked.page_cache.is_none() {
            meta_locked.page_cache = Some(Arc::new(PageCache::new(this.clone(), 3)));
        }
    }
}

pub struct InodeMeta {
    /// inode number
    pub ino: usize,
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
    pub inner: Mutex<InodeMetaInner>,
}

impl InodeMeta {
    pub fn inner_get<T>(&self, f: impl FnOnce(&mut InodeMetaInner) -> T) -> T {
        f(&mut self.inner.lock())
    }
    pub fn inner_set(&self, inner: InodeMetaInner) {
        *self.inner.lock() = inner;
    }
}

#[derive(Clone)]
pub struct InodeMetaInner {
    // /// inode' file's size
    // pub size: usize,
    /// last access time, need to flush to disk.
    pub st_atim: TimeSpec,
    /// last modification time, need to flush to disk
    pub st_mtim: TimeSpec,
    /// last status change time, need to flush to disk
    pub st_ctim: TimeSpec,
    /// parent
    pub parent: Option<Weak<dyn Inode>>,
    /// children list (name, inode)
    pub children: BTreeMap<String, Arc<dyn Inode>>,
    /// page cache of the related file
    pub page_cache: Option<Arc<PageCache>>,
    /// file content len
    pub data_len: usize,
    /// inode state
    pub state: InodeState,
}

impl InodeMeta {
    pub fn new(
        parent: Option<Arc<dyn Inode>>,
        path: &str,
        mode: InodeMode,
        data_len: usize,
        device: Option<InodeDevice>,
    ) -> Self {
        let name = path::get_name(path);
        let parent = match parent {
            Some(parent) => Some(Arc::downgrade(&parent)),
            None => None,
        };
        Self {
            ino: INODE_NUMBER.fetch_add(1, Ordering::Relaxed),
            mode,
            rdev: None,
            device,
            path: path.to_string(),
            name: name.to_string(),
            inner: Mutex::new(InodeMetaInner {
                // size: 0,
                st_atim: TimeSpec::new(),
                st_mtim: TimeSpec::new(),
                st_ctim: TimeSpec::new(),
                parent,
                children: BTreeMap::new(),
                page_cache: None,
                data_len,
                state: InodeState::Init,
            }),
        }
    }
}

pub enum InodeDevice {
    Pipe(Pipe),
    Device(DevWrapper),
    // TODO: add more
}

pub struct DevWrapper {
    pub block_device: Arc<dyn BlockDevice>,
    pub dev_id: usize,
}
