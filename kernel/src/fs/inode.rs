use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::boxed::Box;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::{Arc, Weak},
    vec::Vec,
};
use hashbrown::HashMap;
use lazy_static::*;
use log::{debug, info};

use crate::sync::mutex::SleepLock;
use crate::utils::error::SyscallErr;
use crate::{
    driver::block::BlockDevice,
    fs::HashKey,
    fs::PageCache,
    timer::posix::TimeSpec,
    utils::{
        error::{AgeneralRet, GeneralRet},
        path,
    },
};

use super::FILE_SYSTEM_MANAGER;
use super::{
    file::{DefaultFile, FileMeta, FileMetaInner},
    pipe::Pipe,
    File, Mutex, OpenFlags,
};

lazy_static! {
    /// Dcache: cache: (parent ino, child name) -> dentry
    /// TODO: add max capacity limit and lru policy
    ///
    pub static ref INODE_CACHE: Mutex<HashMap<HashKey, Arc<dyn Inode>>> = Mutex::new(HashMap::new());
    pub static ref FAST_PATH_CACHE: Mutex<HashMap<String, Arc<dyn Inode>>> = Mutex::new(HashMap::new());
}

impl FAST_PATH_CACHE {
    /// return (target inode, is in fast path)
    pub fn get(path: String) -> (Option<Arc<dyn Inode>>, bool) {
        if FAST_PATH.contains(&path.as_str()) {
            let target = FAST_PATH_CACHE.lock().get(&path).cloned();
            return (target, true);
        } else {
            return (None, false);
        }
    }
    pub fn insert(path: String, inode: Arc<dyn Inode>) {
        FAST_PATH_CACHE.lock().insert(path, inode);
    }
}

pub const FAST_PATH: [&str; 4] = ["/dev/null", "/dev/zero", "/dev/tty", "/dev/urandom"];

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
            inner: Mutex::new(FileMetaInner {
                flags,
                inode: Some(this),
                pos: 0,
                dirent_index: 0,
            }),
            prw_lock: SleepLock::new(()),
        };
        let file = DefaultFile::new(file_meta);
        Ok(Arc::new(file))
    }

    /// You should call this function through the parent inode
    /// You should give a absolute path
    fn mkdir(
        &self,
        _this: Arc<dyn Inode>,
        _name: &str,
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
        _name: &str,
        _mode: InodeMode,
        _dev_id: Option<usize>,
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

    fn lookup(&self, this: Arc<dyn Inode>, name: &str) -> GeneralRet<Option<Arc<dyn Inode>>> {
        if this.metadata().mode != InodeMode::FileDIR {
            return Err(SyscallErr::ENOTDIR);
        }
        debug!("[lookup] child name: {}", name);
        let key = HashKey::new(self.metadata().ino, name.to_string());
        let value = INODE_CACHE.lock().get(&key).cloned();
        match value {
            Some(value) => Ok(Some(value.clone())),
            None => {
                debug!(
                    "[lookup] cannot find child dentry, name: {}, try to find in inode",
                    name
                );
                let target_inode = self.try_find_and_insert_inode(this, name);
                match target_inode {
                    Some(target_inode) => Ok(Some(target_inode.clone())),
                    None => Ok(None),
                }
            }
        }
    }
    fn lookup_from_this(
        &self,
        this: Arc<dyn Inode>,
        path: &str,
    ) -> GeneralRet<Option<Arc<dyn Inode>>> {
        let path_names = path::path2vec(path);

        let mut parent = this.clone();

        let path = path::merge(&this.metadata().path, path);
        let path = path::format(&path);
        let (target, fast_path) = FAST_PATH_CACHE::get(path.clone());
        if target.is_some() {
            debug!("[lookup_from_root] find in fast path cache");
            return Ok(target);
        }

        debug!("[lookup_from_root] mismatch in fast path cache");

        for (i, name) in path_names.clone().into_iter().enumerate() {
            debug!("[lookup_from_inode] round: {}, name: {}", i, name);
            match parent.lookup(parent.clone(), name)? {
                Some(p) => {
                    debug!("[lookup_from_this] inode name: {}", p.metadata().name);
                    parent = p;
                }
                None => {
                    if i == path_names.len() - 1 {
                        return Ok(None);
                    }
                    return Err(SyscallErr::ENOENT);
                }
            }
        }

        if fast_path {
            debug!("[lookup_from_root] insert into fast path cache: {}", path);
            FAST_PATH_CACHE::insert(path, parent.clone());
        }
        Ok(Some(parent))
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
        if this.metadata().name.eq("tmp") {
            // tmp in memory, don't load children
            debug!("[try_find_and_insert_inode] parent is tmp, not need to load children");
            return None;
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

    /// unlink() system call will call this method.
    /// This method will delete the inode in inode cache and call delete() function to delete inode in disk.
    fn unlink(&self, child: Arc<dyn Inode>) -> GeneralRet<isize> {
        let key = HashKey::new(self.metadata().ino, child.metadata().name.clone());
        debug!("Try to delete child in INODE_CACHE");
        INODE_CACHE.lock().remove(&key);
        let child_name = child.metadata().name.clone();
        self.metadata().inner.lock().children.remove(&child_name);
        self.delete_child(&child_name);
        Ok(0)
    }

    /// This method will delete the inode in cache (which means deleting inode in parent's children list).
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
    /// This method should be called by parent inode.
    /// TODO: This function should be implemented by actual filesystem.
    fn delete_child(&self, child_name: &str);

    // fn sync(&self);

    /// Sync the inode's metadata
    /// Note that this method only sync this inode itself, not including its children.
    fn sync_metedata(&self) {
        // TODO: not yet implement
        // log::error!("sync dir!!");
    }
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
    ) -> GeneralRet<Option<Arc<dyn Inode>>> {
        let path_names = path::path2vec(path);

        let mut parent = Arc::clone(&FILE_SYSTEM_MANAGER.root_inode());

        let path = path::format(path);
        let (target, fast_path) = FAST_PATH_CACHE::get(path.clone());
        if target.is_some() {
            debug!("[lookup_from_root] find in fast path cache");
            return Ok(target);
        }

        debug!("[lookup_from_root] mismatch in fast path cache");

        for (i, name) in path_names.clone().into_iter().enumerate() {
            debug!("[lookup_from_root] round: {}, name: {}", i, name);
            match parent.lookup(parent.clone(), name)? {
                Some(p) => {
                    debug!("[lookup_from_root] inode name: {}", p.metadata().name);
                    parent = p
                }
                None => {
                    if i == path_names.len() - 1 {
                        return Ok(None);
                    }
                    return Err(SyscallErr::ENOENT);
                }
            }
        }

        if fast_path {
            debug!("[lookup_from_root] insert into fast path cache: {}", path);
            FAST_PATH_CACHE::insert(path, parent.clone());
        }
        Ok(Some(parent))
    }

    pub fn create_page_cache_if_needed(this: Arc<dyn Inode>) {
        let mut meta_locked = this.metadata().inner.lock();
        if meta_locked.page_cache.is_none() {
            meta_locked.page_cache = Some(Arc::new(PageCache::new(this.clone(), 3)));
        }
    }

    /// Sync this inode.
    /// If the inode is a dir, sync it's metadata and all of its children recursively.
    /// If the inode is a regular file, sync its content.
    // #[async_recursion]
    pub fn sync<'a>(this: Arc<dyn Inode>) -> AgeneralRet<'a, ()> {
        Box::pin(async move {
            match this.metadata().mode {
                InodeMode::FileDIR => {
                    log::trace!("[Inode::sync] sync dir..., name {}", this.metadata().name);
                    this.sync_metedata();
                    let mut children_set: Vec<Arc<dyn Inode>> = Vec::new();
                    for (_, child) in this.metadata().inner.lock().children.iter() {
                        children_set.push(child.clone());
                    }

                    for child in children_set {
                        <dyn Inode>::sync(child).await?;
                    }
                    log::trace!(
                        "[Inode::sync] sync dir finished, name {}",
                        this.metadata().name
                    );
                }
                InodeMode::FileREG => {
                    let name = this.metadata().name.clone();
                    log::trace!("[Inode::sync] sync reg file..., name {}", name);
                    this.sync_metedata();
                    <dyn Inode>::sync_reg_file(this).await?;
                    log::trace!("[Inode::sync] sync reg file finished, name {}", name);
                }
                _ => {}
            }
            Ok(())
        })
    }

    /// Sync regular file's content
    async fn sync_reg_file(this: Arc<dyn Inode>) -> GeneralRet<()> {
        let page_cache = this.metadata().inner.lock().page_cache.clone();
        if let Some(page_cache) = page_cache {
            page_cache.sync().await?;
        } else {
            log::trace!("[sync_reg_file] {} no page cache yet", this.metadata().path);
        }
        Ok(())
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
    /// name which doesn't have slash
    pub name: String,
    /// path
    pub path: String,
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

#[derive(Clone)]
pub enum InodeDevice {
    Pipe(Arc<Pipe>),
    Device(DevWrapper),
    // TODO: add more
}

#[derive(Clone)]
pub struct DevWrapper {
    pub block_device: Arc<dyn BlockDevice>,
    pub dev_id: usize,
}
