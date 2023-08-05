use super::Socket;
use crate::{
    fs::{
        pipe::{make_pipe, Pipe},
        File, FileMeta, OpenFlags,
    },
    utils::error::{AsyscallRet, SyscallErr},
};
use alloc::{boxed::Box, sync::Arc};
use smoltcp::wire::IpEndpoint;

pub struct UnixSocket {
    file_meta: FileMeta,
    read_end: Arc<Pipe>,
    write_end: Arc<Pipe>,
}

impl Socket for UnixSocket {
    fn bind(&self, addr: smoltcp::wire::IpListenEndpoint) -> crate::utils::error::SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn listen(&self) -> crate::utils::error::SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn connect(&self, addr_buf: &[u8]) -> AsyscallRet {
        Box::pin(async move { Err(SyscallErr::EOPNOTSUPP) })
    }

    fn accept(&self, sockfd: u32, addr: usize, addrlen: usize) -> AsyscallRet {
        Box::pin(async move { Err(SyscallErr::EOPNOTSUPP) })
    }

    fn socket_type(&self) -> super::SocketType {
        todo!()
    }

    fn recv_buf_size(&self) -> usize {
        todo!()
    }

    fn send_buf_size(&self) -> usize {
        todo!()
    }

    fn set_recv_buf_size(&self, size: usize) {
        todo!()
    }

    fn set_send_buf_size(&self, size: usize) {
        todo!()
    }

    fn loacl_endpoint(&self) -> smoltcp::wire::IpListenEndpoint {
        todo!()
    }

    fn remote_endpoint(&self) -> Option<IpEndpoint> {
        None
    }

    fn shutdown(&self, how: u32) -> crate::utils::error::GeneralRet<()> {
        log::info!("[UnixSocket::shutdown] how {}", how);
        Ok(())
    }

    fn set_nagle_enabled(&self, enabled: bool) -> crate::utils::error::SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }
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
