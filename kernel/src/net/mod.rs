use core::{
    mem,
    slice::{self},
};

use alloc::{collections::BTreeMap, sync::Arc};
use smoltcp::wire::{IpAddress, IpEndpoint, IpListenEndpoint};

use crate::{
    fs::{Fd, File},
    mm::user_check::UserCheck,
    processor::{current_process, SumGuard},
    stack_trace,
    utils::error::{GeneralRet, SyscallErr, SyscallRet},
};

use self::{
    address::{SocketAddrv4, SocketAddrv6},
    tcp::TcpSocket,
    udp::UdpSocket,
};

pub mod address;
pub mod config;
mod tcp;
mod udp;

pub use tcp::TCP_MSS;

/// domain
pub const AF_INET: u16 = 2;
pub const AF_INET6: u16 = 10;

bitflags! {
    /// socket type
    pub struct SocketType: u32 {
        /// for TCP
        const SOCK_STREAM = 1 << 0;
        /// for UDP
        const SOCK_DGRAM = 1 << 1;
        /// unused now
        const SOCK_CLOEXEC = 1 << 19;
    }
}

pub const MAX_BUFFER_SIZE: usize = (1 << 16) - 1;

pub enum Socket {
    TcpSocket(TcpSocket),
    UdpSocket(UdpSocket),
}

impl Socket {
    pub fn new(domain: u32, socket_type: u32) -> GeneralRet<usize> {
        match domain as u16 {
            AF_INET | AF_INET6 => {
                let socket_type = SocketType::from_bits(socket_type).ok_or(SyscallErr::EINVAL)?;
                if socket_type.contains(SocketType::SOCK_DGRAM) {
                    let socket = UdpSocket::new();
                    let socket = Arc::new(Socket::UdpSocket(socket));
                    current_process().inner_handler(|proc| {
                        let fd = proc.fd_table.alloc_fd()?;
                        proc.fd_table.put(fd, socket.clone());
                        proc.socket_table.insert(fd, socket);
                        Ok(fd)
                    })
                } else if socket_type.contains(SocketType::SOCK_STREAM) {
                    let socket = TcpSocket::new();
                    let socket = Arc::new(Socket::TcpSocket(socket));
                    current_process().inner_handler(|proc| {
                        let fd = proc.fd_table.alloc_fd()?;
                        proc.fd_table.put(fd, socket.clone());
                        proc.socket_table.insert(fd, socket);
                        Ok(fd)
                    })
                } else {
                    Err(SyscallErr::EINVAL)
                }
            }
            _ => Err(SyscallErr::EINVAL),
        }
    }
}

impl Socket {
    fn fill_with_endpoint(endpoint: IpEndpoint, addr: usize, addrlen: usize) -> SyscallRet {
        match endpoint.addr {
            IpAddress::Ipv4(_) => {
                let len = mem::size_of::<u16>() + mem::size_of::<SocketAddrv4>();
                UserCheck::new().check_writable_slice(addr as *mut u8, len)?;
                UserCheck::new().check_writable_slice(addrlen as *mut u8, mem::size_of::<u32>())?;
                let addr_buf = unsafe { slice::from_raw_parts_mut(addr as *mut u8, len) };
                SocketAddrv4::from(endpoint).fill(addr_buf, addrlen);
            }
            IpAddress::Ipv6(_) => {
                let len = mem::size_of::<u16>() + mem::size_of::<SocketAddrv6>();
                UserCheck::new().check_writable_slice(addr as *mut u8, len)?;
                UserCheck::new().check_writable_slice(addrlen as *mut u8, mem::size_of::<u32>())?;
                let addr_buf = unsafe { slice::from_raw_parts_mut(addr as *mut u8, len) };
                SocketAddrv6::from(endpoint).fill(addr_buf, addrlen);
            }
        }
        Ok(0)
    }
    pub fn addr(&self, addr: usize, addrlen: usize) -> SyscallRet {
        stack_trace!();
        let _sum_guard = SumGuard::new();
        let local_endpoint = match *self {
            Socket::TcpSocket(ref socket) => socket.addr(),
            Socket::UdpSocket(ref socket) => socket.addr(),
        };
        Self::fill_with_endpoint(local_endpoint, addr, addrlen)
    }
    pub fn peer_addr(&self, addr: usize, addrlen: usize) -> SyscallRet {
        stack_trace!();
        let _sum_guard = SumGuard::new();
        let remote_endpoint = match *self {
            Socket::TcpSocket(ref socket) => socket.peer_addr(),
            Socket::UdpSocket(ref socket) => socket.peer_addr(),
        };
        if remote_endpoint.is_none() {
            return Err(SyscallErr::ENOTCONN);
        }
        Self::fill_with_endpoint(remote_endpoint.unwrap(), addr, addrlen)
    }
    pub fn bind(&self, addr_buf: &[u8]) -> SyscallRet {
        stack_trace!();
        let _sum_guard = SumGuard::new();
        let family = u16::from_ne_bytes(addr_buf[0..2].try_into().expect("family size wrong"));
        log::info!("[sys_bind] addr family {}", family);
        let endpoint = match family {
            AF_INET => {
                let ipv4 = SocketAddrv4::new(addr_buf);
                IpListenEndpoint::from(ipv4)
            }
            AF_INET6 => {
                let ipv6 = SocketAddrv6::new(addr_buf);
                IpListenEndpoint::from(ipv6)
            }
            _ => return Err(SyscallErr::EINVAL),
        };
        log::info!("[sys_bind] bind endpoint: {:?}", endpoint);
        match *self {
            Self::TcpSocket(ref socket) => socket.bind(endpoint),
            Self::UdpSocket(ref socket) => socket.bind(endpoint),
        }
    }
    pub fn listen(&self) -> SyscallRet {
        stack_trace!();
        match *self {
            Socket::TcpSocket(ref socket) => socket.listen(),
            Socket::UdpSocket(_) => Err(SyscallErr::EOPNOTSUPP),
        }
    }

    pub async fn accept(&self, addr: usize, addrlen: usize) -> SyscallRet {
        stack_trace!();
        let (new_socket, peer_addr) = match *self {
            Socket::TcpSocket(ref socket) => {
                let peer_addr = socket.accept().await?;
                log::debug!("[Socket::accept] get peer_addr: {:?}", peer_addr);
                let new_socket = TcpSocket::new();
                new_socket.bind(
                    peer_addr
                        .try_into()
                        .expect("cannot convert to ListenEndpoint"),
                )?;
                let new_socket = Socket::TcpSocket(new_socket);
                (new_socket, peer_addr)
            }
            Socket::UdpSocket(_) => {
                return Err(SyscallErr::EOPNOTSUPP);
            }
        };
        let _sum_guard = SumGuard::new();
        stack_trace!();
        match peer_addr.addr {
            IpAddress::Ipv4(_) => {
                let peer_addr = SocketAddrv4::from(peer_addr);
                if addr != 0 {
                    let len = mem::size_of::<SocketAddrv4>() + mem::size_of::<u16>();
                    UserCheck::new().check_writable_slice(addr as *mut u8, len)?;
                    UserCheck::new()
                        .check_writable_slice(addrlen as *mut u8, mem::size_of::<u32>())?;
                    let addr = unsafe { slice::from_raw_parts_mut(addr as *mut u8, len) };
                    peer_addr.fill(addr, addrlen);
                }
            }
            IpAddress::Ipv6(_) => {
                let peer_addr = SocketAddrv6::from(peer_addr);
                if addr != 0 {
                    let len = mem::size_of::<SocketAddrv6>() + mem::size_of::<u16>();
                    UserCheck::new().check_writable_slice(addr as *mut u8, len)?;
                    UserCheck::new()
                        .check_writable_slice(addrlen as *mut u8, mem::size_of::<u32>())?;
                    let addr = unsafe { slice::from_raw_parts_mut(addr as *mut u8, len) };
                    peer_addr.fill(addr, addrlen);
                }
            }
        }
        stack_trace!();
        let new_socket = Arc::new(new_socket);
        stack_trace!();
        current_process().inner_handler(|proc| {
            let fd = proc.fd_table.alloc_fd()?;
            proc.fd_table.put(fd, new_socket.clone());
            proc.socket_table.insert(fd, new_socket);
            Ok(fd)
        })
    }

    pub async fn connect(&self, addr_buf: &[u8]) -> SyscallRet {
        stack_trace!();
        let _sum_guard = SumGuard::new();
        let family = u16::from_ne_bytes(addr_buf[0..2].try_into().expect("family size wrong"));
        log::info!("[sys_connect] addr family {}", family);
        let endpoint = match family {
            AF_INET => {
                let ipv4 = SocketAddrv4::new(addr_buf);
                IpEndpoint::from(ipv4)
            }
            AF_INET6 => {
                let ipv6 = SocketAddrv6::new(addr_buf);
                IpEndpoint::from(ipv6)
            }
            _ => return Err(SyscallErr::EINVAL),
        };
        match *self {
            Socket::TcpSocket(ref socket) => socket.connect(endpoint).await,
            Socket::UdpSocket(ref socket) => socket.connect(endpoint).await,
        }
    }

    pub fn recv_buf_size(&self) -> usize {
        match *self {
            Socket::TcpSocket(ref socket) => socket.recv_buf_size(),
            Socket::UdpSocket(ref socket) => socket.recv_buf_size(),
        }
    }
    pub fn send_buf_size(&self) -> usize {
        match *self {
            Socket::TcpSocket(ref socket) => socket.send_buf_size(),
            Socket::UdpSocket(ref socket) => socket.send_buf_size(),
        }
    }
    pub fn set_recv_buf_size(&self, size: usize) {
        match *self {
            Socket::TcpSocket(ref socket) => socket.set_recv_buf_size(size),
            Socket::UdpSocket(ref socket) => socket.set_recv_buf_size(size),
        }
    }
    pub fn set_send_buf_size(&self, size: usize) {
        match *self {
            Socket::TcpSocket(ref socket) => socket.set_send_buf_size(size),
            Socket::UdpSocket(ref socket) => socket.set_send_buf_size(size),
        }
    }
}

impl File for Socket {
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> crate::utils::error::AsyscallRet {
        match *self {
            Socket::TcpSocket(ref socket) => socket.read(buf),
            Socket::UdpSocket(ref socket) => socket.read(buf),
        }
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> crate::utils::error::AsyscallRet {
        match *self {
            Socket::TcpSocket(ref socket) => socket.write(buf),
            Socket::UdpSocket(ref socket) => socket.write(buf),
        }
    }

    fn metadata(&self) -> &crate::fs::FileMeta {
        match *self {
            Socket::TcpSocket(ref socket) => socket.metadata(),
            Socket::UdpSocket(ref socket) => socket.metadata(),
        }
    }

    fn flags(&self) -> crate::fs::OpenFlags {
        match *self {
            Socket::TcpSocket(ref socket) => socket.flags(),
            Socket::UdpSocket(ref socket) => socket.flags(),
        }
    }

    fn pollin(&self, waker: Option<core::task::Waker>) -> GeneralRet<bool> {
        match *self {
            Socket::TcpSocket(ref socket) => socket.pollin(waker),
            Socket::UdpSocket(ref socket) => socket.pollin(waker),
        }
    }

    fn pollout(&self, waker: Option<core::task::Waker>) -> GeneralRet<bool> {
        match *self {
            Socket::TcpSocket(ref socket) => socket.pollout(waker),
            Socket::UdpSocket(ref socket) => socket.pollout(waker),
        }
    }
}

pub struct SocketTable(BTreeMap<Fd, Arc<Socket>>);

impl SocketTable {
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn insert(&mut self, key: Fd, value: Arc<Socket>) {
        self.0.insert(key, value);
    }
    pub fn get_ref(&self, fd: Fd) -> Option<&Arc<Socket>> {
        self.0.get(&fd)
    }
}

// pub trait SocketOp {
//     fn bind() {}
//     fn connect() {}
//     fn accept() {}
// }
