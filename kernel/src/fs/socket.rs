use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

use crate::{processor::SumGuard, utils::error::AsyscallRet};

use super::{file::FileMeta, File, Mutex, OpenFlags};

use lazy_static::*;

pub const SOCKETADDR_SIZE: usize = core::mem::size_of::<SocketAddr>();

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
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let mut inner = self.buf.lock();
            let len = inner.len().min(buf.len());
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
            let mut inner = self.buf.lock();
            buf.into_iter().for_each(|ch| inner.push_back(*ch));
            Ok(buf.len() as isize)
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
    pub fn new() -> Self {
        Self {
            socketbuf_mgr: Mutex::new(BTreeMap::new()),
        }
    }
}

lazy_static! {
    pub static ref SOCKETBUF_MANAGER: SocketBufManager = SocketBufManager::new();
}
