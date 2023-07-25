use alloc::{collections::BTreeMap, sync::Arc};

use crate::fs::Fd;

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
