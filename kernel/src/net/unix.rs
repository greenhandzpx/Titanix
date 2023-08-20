use super::Socket;
use crate::{
    fs::{
        pipe::{make_pipe, Pipe},
        File, FileMeta, OpenFlags,
    },
    stack_trace,
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
        stack_trace!();
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn listen(&self) -> crate::utils::error::SyscallRet {
        stack_trace!();
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn connect(&self, _addr_buf: &[u8]) -> AsyscallRet {
        stack_trace!();
        Box::pin(async move { Err(SyscallErr::EOPNOTSUPP) })
    }

    fn accept(&self, _sockfd: u32, _addr: usize, _addrlen: usize) -> AsyscallRet {
        stack_trace!();
        Box::pin(async move { Err(SyscallErr::EOPNOTSUPP) })
    }

    fn socket_type(&self) -> super::SocketType {
        stack_trace!();
        todo!()
    }

    fn recv_buf_size(&self) -> usize {
        stack_trace!();
        todo!()
    }

    fn send_buf_size(&self) -> usize {
        stack_trace!();
        todo!()
    }

    fn set_recv_buf_size(&self, _size: usize) {
        stack_trace!();
        todo!()
    }

    fn set_send_buf_size(&self, _size: usize) {
        stack_trace!();
        todo!()
    }

    fn loacl_endpoint(&self) -> smoltcp::wire::IpListenEndpoint {
        stack_trace!();
        todo!()
    }

    fn remote_endpoint(&self) -> Option<IpEndpoint> {
        stack_trace!();
        None
    }

    fn shutdown(&self, how: u32) -> crate::utils::error::GeneralRet<()> {
        stack_trace!();
        log::info!("[UnixSocket::shutdown] how {}", how);
        Ok(())
    }

    fn set_nagle_enabled(&self, _enabled: bool) -> crate::utils::error::SyscallRet {
        stack_trace!();
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn set_keep_alive(&self, _enabled: bool) -> crate::utils::error::SyscallRet {
        stack_trace!();
        Err(SyscallErr::EOPNOTSUPP)
    }
}

impl<const N: usize> UnixSocket<N> {
    pub fn new(read_end: Arc<Pipe<N>>, write_end: Arc<Pipe<N>>) -> Self {
        stack_trace!();
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
        stack_trace!();
        log::info!("[UnixSocket::read] start to read {} bytes...", buf.len());
        self.read_end.read(buf, flags)
    }
    fn write<'a>(&'a self, buf: &'a [u8], flags: OpenFlags) -> AsyscallRet {
        stack_trace!();
        log::info!("[UnixSocket::write] start to write {} bytes...", buf.len());
        self.write_end.write(buf, flags)
    }
    fn metadata(&self) -> &FileMeta {
        stack_trace!();
        &self.file_meta
    }
}

pub fn make_unix_socket_pair<const N: usize>() -> (Arc<UnixSocket<N>>, Arc<UnixSocket<N>>) {
    stack_trace!();
    let (read1, write1) = make_pipe(None);
    let (read2, write2) = make_pipe(None);
    let socket1 = Arc::new(UnixSocket::new(read1, write2));
    let socket2 = Arc::new(UnixSocket::new(read2, write1));
    (socket1, socket2)
}
