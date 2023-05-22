/// This mod is just temporarily used
use super::file::{File, FileMeta};
use crate::{
    driver::BLOCK_DEVICE,
    processor::SumGuard,
    stack_trace,
    sync::mutex::SpinNoIrqLock,
    utils::error::{AsyscallRet, SyscallRet},
};

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use async_trait::async_trait;
use bitflags::*;
use easy_fs::{EasyFileSystem, Inode};
use lazy_static::*;
use log::{debug, info};
/// A wrapper around a filesystem inode
/// to implement File trait atop
pub struct OSInode {
    readable: bool,
    writable: bool,
    inner: SpinNoIrqLock<OSInodeInner>, // inner: UnSafeCell<OSInodeInner>,
}
/// The OS inode inner in 'UPSafeCell'
pub struct OSInodeInner {
    offset: usize,
    inode: Arc<Inode>,
}

impl OSInode {
    /// Construct an OS inode from a inode
    pub fn new(readable: bool, writable: bool, inode: Arc<Inode>) -> Self {
        Self {
            readable,
            writable,
            inner: unsafe { SpinNoIrqLock::new(OSInodeInner { offset: 0, inode }) },
        }
    }
    /// Read all data inside a inode into vector
    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.lock();
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }
}

lazy_static! {
    pub static ref ROOT_INODE: Arc<Inode> = {
        todo!()
        // let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
        // // println!("hhhhhhhhhhhhh");
        // let ret =Arc::new(EasyFileSystem::root_inode(&efs));
        // ret
    };
}

/// List all files in the filesystems
pub fn list_apps() {
    info!("/************** APPS ****************/");
    for app in ROOT_INODE.ls() {
        info!("{}", app);
    }
    info!("/************************************/");
}

bitflags! {
    ///Open file flags
    pub struct OpenFlags: u32 {
        ///Read only
        const RDONLY = 0;
        ///Write only
        const WRONLY = 1 << 0;
        ///Read & Write
        const RDWR = 1 << 1;
        ///Allow create
        const CREATE = 1 << 9;
        ///Clear file and return an empty one
        const TRUNC = 1 << 10;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}
///Open file with flags
pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
    // TODO support different kinds of files dispatching
    // (e.g. /dev/sda, /proc/1234, /usr/bin)

    stack_trace!();
    let (readable, writable) = flags.read_write();
    if flags.contains(OpenFlags::CREATE) {
        if let Some(inode) = ROOT_INODE.find(name) {
            // clear size
            inode.clear();
            Some(Arc::new(OSInode::new(readable, writable, inode)))
        } else {
            // create file
            ROOT_INODE
                .create(name)
                .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
        }
    } else {
        debug!("name: {}", name);
        ROOT_INODE.find(name).map(|inode| {
            if flags.contains(OpenFlags::TRUNC) {
                inode.clear();
            }
            Arc::new(OSInode::new(readable, writable, inode))
        })
    }
}

// #[async_trait]
impl File for OSInode {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn metadata(&self) -> &FileMeta {
        todo!()
    }
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        // TODO: change into async read
        Box::pin(async move {
            let mut inner = self.inner.lock();
            let mut total_read_size = 0usize;
            let _sum_guard = SumGuard::new();
            total_read_size = inner.inode.read_at(inner.offset, buf);
            inner.offset += total_read_size;

            debug!("read size {}", total_read_size);
            // for slice in buf.iter_mut() {
            //     let read_size = inner.inode.read_at(inner.offset, *slice);
            //     if read_size == 0 {
            //         break;
            //     }
            //     inner.offset += read_size;
            //     total_read_size += read_size;
            // }
            Ok(total_read_size as isize)
        })
    }
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        Box::pin(async move {
            // TODO: change into async write
            let mut inner = self.inner.lock();
            let mut total_write_size = 0usize;
            let _sum_guard = SumGuard::new();
            total_write_size = inner.inode.write_at(inner.offset, buf);
            // for slice in buf.buffers.iter() {
            //     let write_size = inner.inode.write_at(inner.offset, *slice);
            //     assert_eq!(write_size, slice.len());
            //     inner.offset += write_size;
            //     total_write_size += write_size;
            // }
            Ok(total_write_size as isize)
        })
    }
}
