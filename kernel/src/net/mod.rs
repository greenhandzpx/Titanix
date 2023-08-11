use crate::{
    fs::{Fd, FdInfo, File, OpenFlags},
    net::{tcp::TcpSocket, udp::UdpSocket},
    processor::{current_process, SumGuard},
    stack_trace,
    sync::mutex::SpinNoIrqLock,
    utils::error::{AsyscallRet, GeneralRet, SyscallErr, SyscallRet},
};
use alloc::{collections::BTreeMap, sync::Arc};
use log::info;
use smoltcp::wire::{IpEndpoint, IpListenEndpoint};

type Mutex<T> = SpinNoIrqLock<T>;

pub mod address;
pub mod config;
mod tcp;
mod udp;
mod unix;

pub use tcp::TCP_MSS;
pub use unix::make_unix_socket_pair;
// pub use unix::UNIX_SOCKET_BUF_MANAGER;

/// domain
pub const AF_UNIX: u16 = 1;
pub const AF_INET: u16 = 2;
pub const AF_INET6: u16 = 10;

/// shutdown
#[allow(unused)]
pub const SHUT_RD: u32 = 0;
pub const SHUT_WR: u32 = 1;
#[allow(unused)]
pub const SHUT_RDWR: u32 = 2;

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

// pub const MAX_BUFFER_SIZE: usize = 1 << 15;
// pub const MAX_BUFFER_SIZE: usize = 1 << 16;
pub const MAX_BUFFER_SIZE: usize = 1 << 17;

pub trait Socket: File {
    fn bind(&self, addr: IpListenEndpoint) -> SyscallRet;
    fn listen(&self) -> SyscallRet;
    fn connect<'a>(&'a self, addr_buf: &'a [u8]) -> AsyscallRet;
    fn accept(&self, sockfd: u32, addr: usize, addrlen: usize) -> AsyscallRet;
    fn socket_type(&self) -> SocketType;
    fn recv_buf_size(&self) -> usize;
    fn send_buf_size(&self) -> usize;
    fn set_recv_buf_size(&self, size: usize);
    fn set_send_buf_size(&self, size: usize);
    fn loacl_endpoint(&self) -> IpListenEndpoint;
    fn remote_endpoint(&self) -> Option<IpEndpoint>;
    fn shutdown(&self, how: u32) -> GeneralRet<()>;
    fn set_nagle_enabled(&self, enabled: bool) -> SyscallRet;
}

impl dyn Socket {
    pub fn alloc(domain: u32, socket_type: u32) -> GeneralRet<usize> {
        log::info!("[Socket::new] domain: {}", domain);
        match domain as u16 {
            AF_INET | AF_INET6 => {
                let socket_type = SocketType::from_bits(socket_type).ok_or(SyscallErr::EINVAL)?;
                let flags = if socket_type.contains(SocketType::SOCK_CLOEXEC) {
                    OpenFlags::RDWR | OpenFlags::CLOEXEC
                } else {
                    OpenFlags::RDWR
                };
                info!("[Socket::alloc] flags: {:?}", flags);
                if socket_type.contains(SocketType::SOCK_DGRAM) {
                    let socket = UdpSocket::new();
                    let socket = Arc::new(socket);
                    current_process().inner_handler(|proc| {
                        let fd = proc.fd_table.alloc_fd()?;
                        proc.fd_table.put(fd, FdInfo::new(socket.clone(), flags));
                        proc.socket_table.insert(fd, socket);
                        Ok(fd)
                    })
                } else if socket_type.contains(SocketType::SOCK_STREAM) {
                    let socket = TcpSocket::new();
                    let socket = Arc::new(socket);
                    current_process().inner_handler(|proc| {
                        let fd = proc.fd_table.alloc_fd()?;
                        proc.fd_table.put(fd, FdInfo::new(socket.clone(), flags));
                        proc.socket_table.insert(fd, socket);
                        Ok(fd)
                    })
                } else {
                    Err(SyscallErr::EINVAL)
                }
            }
            AF_UNIX => {
                todo!()
                // let socket = UnixSocket::new();
                // let socket = Arc::new(Socket::UnixSocket(socket));
                // current_process().inner_handler(|proc| {
                //     let fd = proc.fd_table.alloc_fd()?;
                //     proc.fd_table.put(fd, socket.clone());
                //     proc.socket_table.insert(fd, socket);
                //     Ok(fd)
                // })
            }
            _ => Err(SyscallErr::EINVAL),
        }
    }
    pub fn addr(self: &Arc<Self>, addr: usize, addrlen: usize) -> SyscallRet {
        stack_trace!();
        let _sum_guard = SumGuard::new();
        let local_endpoint = self.loacl_endpoint();
        let local_endpoint = address::to_endpoint(local_endpoint);
        address::fill_with_endpoint(local_endpoint, addr, addrlen)
    }
    pub fn peer_addr(self: &Arc<Self>, addr: usize, addrlen: usize) -> SyscallRet {
        stack_trace!();
        let _sum_guard = SumGuard::new();
        let remote_endpoint = self.remote_endpoint();
        if remote_endpoint.is_none() {
            return Err(SyscallErr::ENOTCONN);
        }
        address::fill_with_endpoint(remote_endpoint.unwrap(), addr, addrlen)
    }
}

pub struct SocketTable(BTreeMap<Fd, Arc<dyn Socket>>);

impl SocketTable {
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn insert(&mut self, key: Fd, value: Arc<dyn Socket>) {
        self.0.insert(key, value);
    }
    pub fn get_ref(&self, fd: Fd) -> Option<&Arc<dyn Socket>> {
        self.0.get(&fd)
    }
    pub fn take(&mut self, fd: Fd) -> Option<Arc<dyn Socket>> {
        self.0.remove(&fd)
    }
    pub fn from_another(socket_table: &SocketTable) -> GeneralRet<Self> {
        let mut ret = BTreeMap::new();
        for (sockfd, socket) in socket_table.0.iter() {
            ret.insert(*sockfd, socket.clone());
        }
        Ok(Self(ret))
    }
    pub fn can_bind(&self, endpoint: IpListenEndpoint) -> Option<(Fd, Arc<dyn Socket>)> {
        for (sockfd, socket) in self.0.clone() {
            if socket.socket_type().contains(SocketType::SOCK_DGRAM) {
                if socket.loacl_endpoint().eq(&endpoint) {
                    log::info!("[SockTable::can_bind] find port exist");
                    return Some((sockfd, socket));
                }
            }
        }
        None
    }
}
