use smoltcp::socket;

use crate::fs::{File, FileMeta};

pub struct UdpSocket {
    socket: socket::udp::Socket<'static>,
}

impl UdpSocket {
    pub fn new() -> Self {
        todo!()
        // let tx_buf = socket::udp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        // let rx_buf = socket::udp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        // Self {
        //     socket: socket::udp::Socket::new(rx_buf, tx_buf)
        // }
    }
}