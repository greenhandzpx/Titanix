use core::sync::atomic::AtomicUsize;

use alloc::{sync::Arc, vec, vec::Vec};
use log::debug;

use crate::{
    fs::InodeState,
    stack_trace,
    timer::ffi::current_time_spec,
    utils::error::{GeneralRet, SyscallErr, SyscallRet},
};

use super::{file::File, resolve_path, Inode, OpenFlags, AT_FDCWD};

pub static MAX_FD: AtomicUsize = AtomicUsize::new(1024);

pub type Fd = usize;

pub struct FdTable {
    fd_table: Vec<Option<Arc<dyn File>>>,
}

impl FdTable {
    pub fn new() -> Self {
        let tty_inode = resolve_path(AT_FDCWD, "/dev/tty", OpenFlags::empty())
            .ok()
            .unwrap();
        // .unwrap();
        let stdin = tty_inode
            .open(tty_inode.clone(), OpenFlags::RDONLY)
            .unwrap();
        let stdout = tty_inode
            .open(tty_inode.clone(), OpenFlags::WRONLY)
            .unwrap();
        let stderr = tty_inode
            .open(tty_inode.clone(), OpenFlags::WRONLY)
            .unwrap();
        Self {
            fd_table: vec![
                // 0 -> stdin
                Some(stdin),
                // 1 -> stdout
                Some(stdout),
                // 2 -> stderr
                Some(stderr),
            ],
        }
    }

    /// Open a file according to the inode.
    /// Return file descriptor.
    pub fn open(&mut self, inode: Arc<dyn Inode>, flags: OpenFlags) -> SyscallRet {
        stack_trace!();
        let mut inner_lock = inode.metadata().inner.lock();
        inner_lock.st_atim = current_time_spec();
        match inner_lock.state {
            InodeState::Synced => {
                inner_lock.state = InodeState::DirtyInode;
            }
            _ => {}
        }
        debug!(
            "[FdTable::open] inode ino: {}, name: {}",
            inode.metadata().ino,
            inode.metadata().name
        );
        // TODO: add to fs's dirty list
        let fd = self.alloc_fd()?;
        let file = inode.open(inode.clone(), flags)?;

        self.put(fd, file);
        debug!("[FdTable::open] find fd: {}", fd);
        Ok(fd)
    }

    pub fn from_another(fd_table: &FdTable) -> GeneralRet<Self> {
        if fd_table.fd_table.len() >= MAX_FD.load(core::sync::atomic::Ordering::Relaxed) {
            return Err(SyscallErr::EMFILE);
        }
        let mut ret = Vec::new();
        for fd in fd_table.fd_table.iter() {
            if fd.is_none() {
                ret.push(None);
            } else {
                ret.push(fd.as_ref().cloned());
            }
        }
        Ok(Self { fd_table: ret })
    }

    /// Get a ref of the given fd
    pub fn get_ref(&self, fd: Fd) -> Option<&Arc<dyn File>> {
        if fd >= self.fd_table.len() {
            None
        } else {
            // let mut cnt = 0;
            // if self.fd_table[fd].is_some() {
            //     let cnt = self.fd_table[fd].as_ref().unwrap()
            // }
            // debug!("get fd {}, ref cnt {}", fd, self.fd_table[fd].)
            self.fd_table[fd].as_ref()
        }
    }

    /// Get the ownership of the given fd by clone
    pub fn get(&self, fd: Fd) -> Option<Arc<dyn File>> {
        if fd >= self.fd_table.len() {
            None
        } else {
            self.fd_table[fd].clone()
        }
    }

    /// Take the ownership of the given fd
    pub fn take(&mut self, fd: Fd) -> Option<Arc<dyn File>> {
        if fd >= self.fd_table.len() {
            None
        } else {
            self.fd_table[fd].take()
        }
    }

    pub fn put(&mut self, fd: Fd, file: Arc<dyn File>) {
        assert!(fd < self.fd_table.len());
        assert!(self.fd_table[fd].is_none());
        self.fd_table[fd] = Some(file);
    }

    pub fn alloc_fd(&mut self) -> GeneralRet<usize> {
        if let Some(fd) = self.free_slot() {
            Ok(fd)
        } else {
            if self.fd_table.len() >= MAX_FD.load(core::sync::atomic::Ordering::Relaxed) {
                return Err(SyscallErr::EMFILE);
            }
            self.fd_table.push(None);
            Ok(self.fd_table.len() - 1)
        }
    }
    pub fn alloc_fd_lower_bound(&mut self, bound: Fd) -> GeneralRet<usize> {
        if let Some(fd) =
            (0..self.fd_table.len()).find(|fd| *fd >= bound && self.fd_table[*fd].is_none())
        {
            Ok(fd)
        } else {
            if bound >= MAX_FD.load(core::sync::atomic::Ordering::Relaxed) {
                return Err(SyscallErr::EMFILE);
            }
            if bound >= self.fd_table.len() {
                self.fd_table.resize(bound + 1, None);
            } else {
                if self.fd_table.len() >= MAX_FD.load(core::sync::atomic::Ordering::Relaxed) {
                    return Err(SyscallErr::EMFILE);
                }
                self.fd_table.push(None)
            }
            Ok(self.fd_table.len() - 1)
        }
    }

    pub fn alloc_spec_fd(&mut self, newfd: Fd) -> GeneralRet<usize> {
        if newfd >= MAX_FD.load(core::sync::atomic::Ordering::Relaxed) {
            return Err(SyscallErr::EMFILE);
        }
        if newfd >= self.fd_table.len() {
            self.fd_table.resize(newfd + 1, None);
        }
        Ok(newfd)
    }

    fn free_slot(&self) -> Option<usize> {
        (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none())
    }
}
