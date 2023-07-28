use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};
use smoltcp::wire::IpEndpoint;

use crate::{
    fs::{File, FileMeta, OpenFlags},
    net::address::SocketAddrv4,
    process::thread,
    processor::SumGuard,
    utils::error::AsyscallRet,
};

use super::{Mutex, MAX_BUFFER_SIZE};

pub struct UnixSocket {
    pub file_meta: FileMeta,
    pub buf: Mutex<VecDeque<u8>>,
}

impl UnixSocket {
    pub fn new() -> Self {
        Self {
            file_meta: FileMeta::new(OpenFlags::CLOEXEC | OpenFlags::NONBLOCK | OpenFlags::RDWR),
            buf: Mutex::new(VecDeque::new()),
        }
    }
    pub fn addr(&self, addr_buf: &[u8]) -> IpEndpoint {
        let _sum_guard = SumGuard::new();
        let endpoint = {
            let ipv4 = SocketAddrv4::new(addr_buf);
            IpEndpoint::from(ipv4)
        };
        log::info!("[Unix::addr] {:?}", endpoint);
        endpoint
    }
}
impl File for UnixSocket {
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let mut inner = self.buf.lock();
            let len = inner.len().min(buf.len());
            inner
                .drain(..len)
                .zip(buf.into_iter())
                .for_each(|(src, dst)| *dst = src);
            Ok(len)
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
                    return Ok(buf.len());
                }
            }
        })
    }

    fn metadata(&self) -> &FileMeta {
        &self.file_meta
    }
    fn flags(&self) -> OpenFlags {
        self.file_meta.inner.lock().flags
    }
}
pub struct UnixSocketBufManager {
    pub buf_mgr: Mutex<BTreeMap<IpEndpoint, Arc<dyn File>>>,
}
impl UnixSocketBufManager {
    pub const fn new() -> Self {
        Self {
            buf_mgr: Mutex::new(BTreeMap::new()),
        }
    }
}
pub static UNIX_SOCKET_BUF_MANAGER: UnixSocketBufManager = UnixSocketBufManager::new();
