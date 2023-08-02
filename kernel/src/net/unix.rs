use alloc::sync::Arc;
use smoltcp::wire::IpEndpoint;

use crate::{
    fs::{
        pipe::{make_pipe, Pipe},
        File, FileMeta, OpenFlags,
    },
    net::address::SocketAddrv4,
    processor::SumGuard,
    utils::error::AsyscallRet,
};

pub struct UnixSocket {
    file_meta: FileMeta,
    read_end: Arc<Pipe>,
    write_end: Arc<Pipe>,
}

impl UnixSocket {
    pub fn new(read_end: Arc<Pipe>, write_end: Arc<Pipe>) -> Self {
        Self {
            file_meta: FileMeta::new(
                OpenFlags::CLOEXEC | OpenFlags::NONBLOCK | OpenFlags::RDWR,
                crate::fs::InodeMode::FileSOCK,
            ),
            // buf: Mutex::new(VecDeque::new()),
            read_end,
            write_end,
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
        log::info!("[UnixSocket::read] start to read {} bytes...", buf.len());
        self.read_end.read(buf)
    }
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        log::info!("[UnixSocket::write] start to write {} bytes...", buf.len());
        self.write_end.write(buf)
    }
    fn metadata(&self) -> &FileMeta {
        &self.file_meta
    }
    fn flags(&self) -> OpenFlags {
        self.file_meta.inner.lock().flags
    }
}

pub fn make_unix_socket_pair() -> (Arc<UnixSocket>, Arc<UnixSocket>) {
    let (read1, write1) = make_pipe();
    let (read2, write2) = make_pipe();
    let socket1 = Arc::new(UnixSocket::new(read1, write2));
    let socket2 = Arc::new(UnixSocket::new(read2, write1));
    (socket1, socket2)
}
