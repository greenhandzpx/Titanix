use smoltcp::wire::{IpAddress, IpEndpoint, IpListenEndpoint, Ipv4Address, Ipv6Address};

use super::AF_INET;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct SocketAddrv4 {
    sin_port: [u8; 2],
    sin_addr: [u8; 4],
}

impl SocketAddrv4 {
    /// user check first
    pub fn new(buf: &[u8]) -> Self {
        Self {
            sin_port: buf[2..4].try_into().expect("ipv4 port len err"),
            sin_addr: buf[4..8].try_into().expect("ipv4 addr len err"),
        }
    }
    pub fn fill(&self, addr_buf: &mut [u8], addrlen: usize) {
        addr_buf.fill(0);
        addr_buf[0..2].copy_from_slice(u16::to_be_bytes(AF_INET).as_slice());
        addr_buf[2..4].copy_from_slice(self.sin_port.as_slice());
        addr_buf[4..8].copy_from_slice(self.sin_addr.as_slice());
        unsafe {
            *(addrlen as *mut u32) = 8;
        }
    }
}

impl From<IpEndpoint> for SocketAddrv4 {
    fn from(value: IpEndpoint) -> Self {
        Self {
            sin_port: value.port.to_be_bytes(),
            sin_addr: value
                .addr
                .as_bytes()
                .try_into()
                .expect("ipv4 addr len error"),
        }
    }
}

impl From<SocketAddrv4> for IpEndpoint {
    fn from(value: SocketAddrv4) -> Self {
        // big end
        let port = u16::from_be_bytes(value.sin_port);
        Self::new(IpAddress::Ipv4(Ipv4Address(value.sin_addr)), port)
    }
}

impl From<SocketAddrv4> for IpListenEndpoint {
    fn from(value: SocketAddrv4) -> Self {
        // big end
        let port = u16::from_be_bytes(value.sin_port);
        let addr = Ipv4Address(value.sin_addr);
        if addr.is_unspecified() {
            IpListenEndpoint { addr: None, port }
        } else {
            IpListenEndpoint {
                addr: Some(IpAddress::Ipv4(addr)),
                port,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct SocketAddrv6 {
    sin6_port: [u8; 2],
    sin6_flowinfo: [u8; 4],
    sin6_addr: [u8; 16],
}

impl SocketAddrv6 {
    /// user check first
    pub fn new(buf: &[u8]) -> Self {
        Self {
            sin6_port: buf[2..4].try_into().expect("ipv6 port len err"),
            sin6_flowinfo: buf[4..8].try_into().expect("ipv6 flowinfo len err"),
            sin6_addr: buf[8..24].try_into().expect("ipv6 addr len err"),
        }
    }
    pub fn fill(&self, addr_buf: &mut [u8], addrlen: usize) {
        addr_buf.fill(0);
        addr_buf[0..2].copy_from_slice(u16::to_be_bytes(AF_INET).as_slice());
        addr_buf[2..4].copy_from_slice(self.sin6_port.as_slice());
        addr_buf[4..8].copy_from_slice(self.sin6_flowinfo.as_slice());
        addr_buf[8..24].copy_from_slice(self.sin6_addr.as_slice());
        unsafe {
            *(addrlen as *mut u32) = 24;
        }
    }
}

impl From<IpEndpoint> for SocketAddrv6 {
    fn from(value: IpEndpoint) -> Self {
        Self {
            sin6_port: value.port.to_be_bytes(),
            sin6_flowinfo: [0 as u8; 4],
            sin6_addr: value
                .addr
                .as_bytes()
                .try_into()
                .expect("ipv6 addr len error"),
        }
    }
}

impl From<SocketAddrv6> for IpEndpoint {
    fn from(value: SocketAddrv6) -> Self {
        // big end
        let port = u16::from_be_bytes(value.sin6_port);
        Self::new(IpAddress::Ipv6(Ipv6Address(value.sin6_addr)), port)
    }
}

impl From<SocketAddrv6> for IpListenEndpoint {
    fn from(value: SocketAddrv6) -> Self {
        // big end
        let port = u16::from_be_bytes(value.sin6_port);
        let addr = Ipv6Address(value.sin6_addr);
        if addr.is_unspecified() {
            IpListenEndpoint { addr: None, port }
        } else {
            IpListenEndpoint {
                addr: Some(IpAddress::Ipv6(addr)),
                port,
            }
        }
    }
}
