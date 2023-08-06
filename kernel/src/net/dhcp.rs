use super::config::NET_INTERFACE;
use smoltcp::{iface::SocketHandle, socket::dhcpv4};

pub struct DhcpSocket {
    socket_handler: SocketHandle,
}

impl DhcpSocket {
    pub fn new() -> Self {
        let socket = dhcpv4::Socket::new();
        let socket_handler = NET_INTERFACE.add_socket(socket);
        Self { socket_handler }
    }
    pub fn start() {}
}
