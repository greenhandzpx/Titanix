use alloc::{sync::Arc, vec, vec::Vec};
use log::debug;

use crate::{
    config::fs::MAX_FD_NUM,
    fs::InodeState,
    process::resource::{RLimit, RLIM_INFINITY},
    stack_trace,
    timer::ffi::current_time_spec,
    utils::error::{GeneralRet, SyscallErr, SyscallRet},
};

use super::{file::File, resolve_path, Inode, OpenFlags, AT_FDCWD};

pub type Fd = usize;

pub struct FdTable {
    pub fd_table: Vec<Option<FdInfo>>,
    /// max fd
    rlimit: RLimit,
}

#[derive(Clone)]
pub struct FdInfo {
    pub file: Arc<dyn File>,
    pub flags: OpenFlags,
}

impl FdInfo {
    pub fn new(file: Arc<dyn File>, flags: OpenFlags) -> Self {
        Self { file, flags }
    }
}

impl FdTable {
    pub fn new() -> Self {
        let tty_inode = resolve_path(AT_FDCWD, "/dev/tty", OpenFlags::empty())
            .ok()
            .unwrap();
        // .unwrap();
        let stdin = FdInfo::new(
            tty_inode.open(tty_inode.clone()).unwrap(),
            OpenFlags::RDONLY,
        );
        let stdout = FdInfo::new(
            tty_inode.open(tty_inode.clone()).unwrap(),
            OpenFlags::WRONLY,
        );
        let stderr = FdInfo::new(
            tty_inode.open(tty_inode.clone()).unwrap(),
            OpenFlags::WRONLY,
        );
        Self {
            fd_table: vec![
                // 0 -> stdin
                Some(stdin),
                // 1 -> stdout
                Some(stdout),
                // 2 -> stderr
                Some(stderr),
            ],
            rlimit: RLimit {
                rlim_cur: MAX_FD_NUM,
                rlim_max: RLIM_INFINITY,
            },
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
        let file = inode.open(inode.clone())?;
        let fd_info = FdInfo::new(file, flags);

        self.put(fd, fd_info);
        debug!("[FdTable::open] find fd: {}", fd);
        Ok(fd)
    }

    pub fn from_another(fd_table: &FdTable) -> GeneralRet<Self> {
        // if fd_table.fd_table.len() >= MAX_FD.load(core::sync::atomic::Ordering::Relaxed) {
        //     return Err(SyscallErr::EMFILE);
        // }
        let mut ret = Vec::new();
        for fd in fd_table.fd_table.iter() {
            if fd.is_none() {
                ret.push(None);
            } else {
                ret.push(fd.as_ref().cloned());
            }
        }
        Ok(Self {
            fd_table: ret,
            rlimit: fd_table.rlimit,
        })
    }

    /// Get a ref of the given fd
    pub fn get_ref(&self, fd: Fd) -> Option<&FdInfo> {
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
    pub fn get(&self, fd: Fd) -> Option<FdInfo> {
        if fd >= self.fd_table.len() {
            None
        } else {
            self.fd_table[fd].clone()
        }
    }

    /// Get the ownership of the given fd by clone
    pub fn get_mut(&mut self, fd: Fd) -> Option<&mut FdInfo> {
        if fd >= self.fd_table.len() {
            None
        } else {
            self.fd_table[fd].as_mut()
        }
    }

    /// Take the ownership of the given fd
    pub fn take(&mut self, fd: Fd) -> Option<FdInfo> {
        if fd >= self.fd_table.len() {
            None
        } else {
            self.fd_table[fd].take()
        }
    }

    pub fn put(&mut self, fd: Fd, fd_info: FdInfo) {
        assert!(fd < self.fd_table.len());
        assert!(self.fd_table[fd].is_none());
        self.fd_table[fd] = Some(fd_info);
    }

    pub fn alloc_fd(&mut self) -> GeneralRet<usize> {
        if let Some(fd) = self.free_slot() {
            Ok(fd)
        } else {
            if self.fd_table.len() >= self.rlimit.rlim_cur {
                return Err(SyscallErr::EMFILE);
            }
            self.fd_table.push(None);
            // println!("[alloc_fd] alloc {}", self.fd_table.len() - 1);
            Ok(self.fd_table.len() - 1)
        }
    }
    pub fn alloc_fd_lower_bound(&mut self, bound: Fd) -> GeneralRet<usize> {
        if let Some(fd) =
            (0..self.fd_table.len()).find(|fd| *fd >= bound && self.fd_table[*fd].is_none())
        {
            Ok(fd)
        } else {
            if bound >= self.rlimit.rlim_cur {
                return Err(SyscallErr::EMFILE);
            }
            if bound >= self.fd_table.len() {
                self.fd_table.resize(bound + 1, None);
            } else {
                if self.fd_table.len() >= self.rlimit.rlim_cur {
                    return Err(SyscallErr::EMFILE);
                }
                self.fd_table.push(None)
            }
            Ok(self.fd_table.len() - 1)
        }
    }

    pub fn alloc_spec_fd(&mut self, newfd: Fd) -> GeneralRet<usize> {
        if newfd >= self.rlimit.rlim_cur {
            return Err(SyscallErr::EMFILE);
        }
        if newfd >= self.fd_table.len() {
            self.fd_table.resize(newfd + 1, None);
        }
        Ok(newfd)
    }

    pub fn close_on_exec(&mut self) {
        for (_fd, file) in self.fd_table.iter_mut().enumerate() {
            if let Some(f) = file {
                if f.flags.contains(OpenFlags::CLOEXEC) {
                    // log::error!("[close_on_exec] close fd {}, flags {:?}", fd, f.flags);
                    *file = None;
                }
            }
        }
    }

    pub fn set_rlimit(&mut self, rlimit: RLimit) {
        self.rlimit = rlimit;
    }

    pub fn rlimit(&self) -> RLimit {
        self.rlimit
    }

    fn free_slot(&self) -> Option<usize> {
        (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none())
    }
}
