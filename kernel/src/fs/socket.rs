use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

use crate::{
    mm::user_check::UserCheck, process::thread, processor::SumGuard, utils::error::AsyscallRet,
};

use super::{file::FileMeta, File, Mutex, OpenFlags};

pub const SOCKETADDR_SIZE: usize = core::mem::size_of::<SocketAddr>();

pub const MAX_BUFFER_SIZE: usize = 1 << 16 - 1;

pub struct Socket {
    pub flags: OpenFlags,
    pub buf: Mutex<VecDeque<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct SocketAddr {
    sa_family: u32,
    sa_data: [u8; 14],
}

impl Socket {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            flags: OpenFlags::CLOEXEC | OpenFlags::NONBLOCK,
            buf: Mutex::new(VecDeque::new()),
        })
    }
}

impl File for Socket {
    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        true
    }

    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let mut inner = self.buf.lock();
            let len = inner.len().min(buf.len());
            UserCheck::new().check_writable_slice(buf.as_ptr() as *mut u8, len)?;
            inner
                .drain(..len)
                .zip(buf.into_iter())
                .for_each(|(src, dst)| *dst = src);
            Ok(len as isize)
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            loop {
                if {
                    let mut inner = self.buf.lock();
                    if inner.len() + buf.len() > MAX_BUFFER_SIZE {
                        true
                    } else {
                        buf.into_iter().for_each(|ch| inner.push_back(*ch));
                        false
                    }
                } {
                    thread::yield_now().await;
                } else {
                    return Ok(buf.len() as isize);
                }
            }
        })
    }

    fn metadata(&self) -> &FileMeta {
        todo!()
    }

    fn flags(&self) -> OpenFlags {
        self.flags
    }
}

pub struct SocketBufManager {
    pub socketbuf_mgr: Mutex<BTreeMap<SocketAddr, Arc<dyn File>>>,
}

impl SocketBufManager {
    pub const fn new() -> Self {
        Self {
            socketbuf_mgr: Mutex::new(BTreeMap::new()),
        }
    }
}

pub static SOCKETBUF_MANAGER: SocketBufManager = SocketBufManager::new();
