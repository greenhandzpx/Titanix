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

pub struct UnixSocket<const N: usize> {
    file_meta: FileMeta,
    read_end: Arc<Pipe<N>>,
    write_end: Arc<Pipe<N>>,
}

impl<const N: usize> Socket for UnixSocket<N> {
    fn bind(&self, _addr: smoltcp::wire::IpListenEndpoint) -> crate::utils::error::SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn listen(&self) -> crate::utils::error::SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn connect(&self, _addr_buf: &[u8]) -> AsyscallRet {
        Box::pin(async move { Err(SyscallErr::EOPNOTSUPP) })
    }

    fn accept(&self, _sockfd: u32, _addr: usize, _addrlen: usize) -> AsyscallRet {
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

    fn set_recv_buf_size(&self, _size: usize) {
        todo!()
    }

    fn set_send_buf_size(&self, _size: usize) {
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

    fn set_nagle_enabled(&self, _enabled: bool) -> crate::utils::error::SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn set_keep_alive(&self, _enabled: bool) -> crate::utils::error::SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn recv<'a>(&'a self, buf: &'a mut [u8], flags: super::RecvFromFlags) -> AsyscallRet {
        todo!()
    }

    fn send<'a>(&'a self, buf: &'a [u8], flags: super::RecvFromFlags) -> AsyscallRet {
        todo!()
    }
}

impl<const N: usize> UnixSocket<N> {
    pub fn new(read_end: Arc<Pipe<N>>, write_end: Arc<Pipe<N>>) -> Self {
        Self {
            file_meta: FileMeta::new(crate::fs::InodeMode::FileSOCK),
            // buf: Mutex::new(VecDeque::new()),
            read_end,
            write_end,
        }
    }
}
impl<const N: usize> File for UnixSocket<N> {
    fn read<'a>(&'a self, buf: &'a mut [u8], flags: OpenFlags) -> AsyscallRet {
        log::info!("[UnixSocket::read] start to read {} bytes...", buf.len());
        self.read_end.read(buf, flags)
    }
    fn write<'a>(&'a self, buf: &'a [u8], flags: OpenFlags) -> AsyscallRet {
        log::info!("[UnixSocket::write] start to write {} bytes...", buf.len());
        self.write_end.write(buf, flags)
    }
    fn metadata(&self) -> &FileMeta {
        &self.file_meta
    }
}

pub fn make_unix_socket_pair<const N: usize>() -> (Arc<UnixSocket<N>>, Arc<UnixSocket<N>>) {
    let (read1, write1) = make_pipe(None);
    let (read2, write2) = make_pipe(None);
    let socket1 = Arc::new(UnixSocket::new(read1, write2));
    let socket2 = Arc::new(UnixSocket::new(read2, write1));
    (socket1, socket2)
}
