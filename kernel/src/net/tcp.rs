use alloc::{vec, vec::Vec};
use smoltcp::socket;

use crate::fs::{File, FileMeta};

pub struct TcpSocket {
    socket: socket::tcp::Socket<'static>,
}

impl TcpSocket {
    pub fn new() -> Self {
        todo!()
    }
}
