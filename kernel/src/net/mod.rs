use alloc::{collections::BTreeMap, sync::Arc};

use crate::fs::{Fd, File};

use self::{tcp::TcpSocket, udp::UdpSocket};

mod tcp;
mod udp;

/// domain
pub const AF_INET: u32 = 2;

pub const MAX_BUFFER_SIZE: usize = (1 << 16) - 1;

pub const TCP_MSS: u32 = 32768;

pub enum Socket {
    TcpSocket(TcpSocket),
    UdpSocket(UdpSocket),
}

impl File for Socket {
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> crate::utils::error::AsyscallRet {
        todo!()
        // match *self {
        //     Socket::TcpSocket(socket) => todo!(),
        //     Socket::UdpSocket(_) => todo!(),
        // }
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> crate::utils::error::AsyscallRet {
        todo!()
    }

    fn metadata(&self) -> &crate::fs::FileMeta {
        todo!()
    }

    fn flags(&self) -> crate::fs::OpenFlags {
        todo!()
    }
}

pub struct SocketTable(BTreeMap<Fd, Arc<Socket>>);

impl SocketTable {
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }
}

// pub trait SocketOp {
//     fn bind() {}
//     fn connect() {}
//     fn accept() {}
// }
