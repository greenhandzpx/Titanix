use alloc::{sync::Arc, vec, vec::Vec};

use super::{file::File, resolve_path, OpenFlags};

pub struct FdTable {
    fd_table: Vec<Option<Arc<dyn File>>>,
}

impl FdTable {
    pub fn new() -> Self {
        let tty_inode = resolve_path("/dev/tty", OpenFlags::empty()).ok().unwrap();
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

    pub fn from_another(fd_table: &FdTable) -> Self {
        let mut ret = Vec::new();
        for fd in fd_table.fd_table.iter() {
            if fd.is_none() {
                ret.push(None);
            } else {
                ret.push(fd.as_ref().cloned());
            }
        }
        Self { fd_table: ret }
    }

    /// Get a ref of the given fd
    pub fn get_ref(&self, fd: usize) -> Option<&Arc<dyn File>> {
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
    pub fn get(&self, fd: usize) -> Option<Arc<dyn File>> {
        if fd >= self.fd_table.len() {
            None
        } else {
            self.fd_table[fd].clone()
        }
    }

    /// Take the ownership of the given fd
    pub fn take(&mut self, fd: usize) -> Option<Arc<dyn File>> {
        if fd >= self.fd_table.len() {
            None
        } else {
            self.fd_table[fd].take()
        }
    }

    pub fn put(&mut self, fd: usize, file: Arc<dyn File>) {
        assert!(fd < self.fd_table.len());
        assert!(self.fd_table[fd].is_none());
        self.fd_table[fd] = Some(file);
    }

    pub fn alloc_fd(&mut self) -> usize {
        if let Some(fd) = self.free_slot() {
            fd
        } else {
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
    }
    pub fn alloc_fd_lower_bound(&mut self, bound: usize) -> usize {
        if let Some(fd) =
            (0..self.fd_table.len()).find(|fd| *fd >= bound && self.fd_table[*fd].is_none())
        {
            fd
        } else {
            if bound >= self.fd_table.len() {
                self.fd_table.resize(bound + 1, None);
            } else {
                self.fd_table.push(None)
            }
            self.fd_table.len() - 1
        }
    }

    pub fn alloc_spec_fd(&mut self, newfd: usize) -> usize {
        if newfd >= self.fd_table.len() {
            self.fd_table.resize(newfd + 1, None);
        }
        newfd
    }

    fn free_slot(&self) -> Option<usize> {
        (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none())
    }
}
