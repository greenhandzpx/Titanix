use alloc::{boxed::Box, vec, vec::Vec};
use smoltcp::socket;

use crate::{
    fs::{File, FileMeta},
    net::MAX_BUFFER_SIZE,
    sync::mutex::SpinNoIrqLock,
};

type Mutex<T> = SpinNoIrqLock<T>;

pub struct TcpSocket {
    socket: Mutex<socket::tcp::Socket<'static>>,
    // TODO: add more
}

impl TcpSocket {
    pub fn new() -> Self {
        let tx_buf = socket::tcp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        let rx_buf = socket::tcp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        Self {
            socket: Mutex::new(socket::tcp::Socket::new(rx_buf, tx_buf)),
        }
    }
}

impl File for TcpSocket {
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> crate::utils::error::AsyscallRet {
        Box::pin(async move {
            let len = self
                .socket
                .lock()
                .recv(|input_buf| {
                    let len = buf.len().min(input_buf.len());
                    buf[..len].copy_from_slice(&input_buf[..len]);
                    (len, len)
                })
                .expect("[TcpSocket::read] err when reading!");
            Ok(len)
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> crate::utils::error::AsyscallRet {
        Box::pin(async move {
            let len = self
                .socket
                .lock()
                .send_slice(buf)
                .expect("[TcpSocket::write] err when writing!");
            Ok(len)
        })
    }

    fn metadata(&self) -> &FileMeta {
        todo!()
    }

    fn flags(&self) -> crate::fs::OpenFlags {
        todo!()
    }
}
