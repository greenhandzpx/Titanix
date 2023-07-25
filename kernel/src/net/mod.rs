use alloc::{collections::BTreeMap, sync::Arc};

use crate::fs::Fd;

use self::{tcp::TcpSocket, udp::UdpSocket};

mod tcp;
mod udp;

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

pub struct SocketFile {}
