use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use lazy_static::*;
use log::debug;

use crate::{
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::InodeMeta,
        File, Inode, InodeMode, Mutex, OpenFlags,
    },
    processor::SumGuard,
    utils::error::{AsyscallRet, GeneralRet, SyscallErr, SyscallRet},
};

lazy_static! {
    pub static ref MEM_INFO: Mutex<Arc<Meminfo>> = Mutex::new(Arc::new(Meminfo::new()));
}

const TOTAL_MEM: usize = 16251136;
const FREE_MEM: usize = 327680;
const BUFFER: usize = 373336;
const CACHED: usize = 10391984;
const TOTAL_SWAP: usize = 4194300;

/// Mapping to free output: https://access.redhat.com/solutions/406773.
pub struct Meminfo {
    /// General memory
    pub total_mem: usize,
    pub free_mem: usize,
    pub avail_mem: usize,
    /// Buffer and cache
    pub buffers: usize,
    pub cached: usize,
    /// Swap space
    pub total_swap: usize,
    pub free_swap: usize,
    /// Share memory
    pub shmem: usize,
    pub slab: usize,
}
impl Meminfo {
    pub fn new() -> Self {
        Self {
            total_mem: TOTAL_MEM,
            free_mem: FREE_MEM,
            avail_mem: TOTAL_MEM - FREE_MEM,
            buffers: BUFFER,
            cached: CACHED,
            total_swap: TOTAL_SWAP,
            free_swap: TOTAL_SWAP,
            shmem: 0,
            slab: 0,
        }
    }
    pub fn serialize(&self) -> String {
        let mut res = "".to_string();
        let end = " KB\n";
        let total_mem = "MemTotal:\t".to_string() + self.total_mem.to_string().as_str() + end;
        let free_mem = "MemFree:\t".to_string() + self.free_mem.to_string().as_str() + end;
        let avail_mem = "MemAvailable:\t".to_string() + self.avail_mem.to_string().as_str() + end;
        let buffers = "Buffers:\t".to_string() + self.buffers.to_string().as_str() + end;
        let cached = "Cached:\t".to_string() + self.cached.to_string().as_str() + end;
        let cached_swap = "SwapCached:\t".to_string() + 0.to_string().as_str() + end;
        let total_swap = "SwapTotal:\t".to_string() + self.total_swap.to_string().as_str() + end;
        let free_swap = "SwapFree:\t".to_string() + self.free_swap.to_string().as_str() + end;
        let shmem = "Shmem:\t".to_string() + self.shmem.to_string().as_str() + end;
        let slab = "Slab:\t".to_string() + self.slab.to_string().as_str() + end;
        res += total_mem.as_str();
        res += free_mem.as_str();
        res += avail_mem.as_str();
        res += buffers.as_str();
        res += cached.as_str();
        res += cached_swap.as_str();
        res += total_swap.as_str();
        res += free_swap.as_str();
        res += shmem.as_str();
        res += slab.as_str();
        res
    }
}

pub struct MeminfoInode {
    metadata: InodeMeta,
}

impl MeminfoInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        Self {
            metadata: InodeMeta::new(Some(parent), path, InodeMode::FileREG, 0, None),
        }
    }
}

impl Inode for MeminfoInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(MeminfoFile {
            meta: FileMeta {
                path: "/proc/meminfo".to_string(),
                inner: Mutex::new(FileMetaInner {
                    flags,
                    inode: Some(this),
                    pos: 0,
                    dirent_index: 0,
                }),
            },
        }))
    }

    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = meta;
    }

    fn load_children_from_disk(&self, this: Arc<dyn Inode>) {
        panic!("Unsupported operation")
    }

    fn delete_child(&self, child_name: &str) {
        panic!("Unsupported operation")
    }
}

pub struct MeminfoFile {
    meta: FileMeta,
}

impl File for MeminfoFile {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        debug!("[MeminfoFile] read");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let meminfo = MEM_INFO.lock();
            let info = meminfo.serialize();
            let len = info.len();
            let mut inner = self.metadata().inner.lock();
            debug!("[MeminfoFile] info size: {}", len);
            if inner.pos == len {
                debug!("[MeminfoFile] already read");
                Ok(0)
            } else {
                buf[..len].copy_from_slice(info.as_bytes());
                inner.pos = len;
                Ok(len as isize)
            }
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        debug!("[MeminfoFile] cannot write");
        Box::pin(async move { Err(SyscallErr::EACCES) })
    }

    fn metadata(&self) -> &FileMeta {
        &self.meta
    }

    fn seek(&self, offset: usize) -> SyscallRet {
        debug!("[MeminfoFile] seek offset: {}", offset);
        self.meta.inner.lock().pos = offset;
        Ok(offset as isize)
    }
}
