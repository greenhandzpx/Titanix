use smoltcp::socket;

use crate::fs::{File, FileMeta};

pub struct UdpSocket {
    socket: socket::udp::Socket<'static>,
}
