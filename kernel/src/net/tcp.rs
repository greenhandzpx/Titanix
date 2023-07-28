use core::{future::Future, task::Poll};

use alloc::{boxed::Box, vec};
use log::{debug, info};
use managed::ManagedSlice;
use smoltcp::{
    iface::SocketHandle,
    socket::{self, tcp},
    wire::{IpAddress, IpEndpoint, IpListenEndpoint},
};

use crate::{
    fs::{File, FileMeta, OpenFlags},
    net::{config::NET_INTERFACE, MAX_BUFFER_SIZE},
    process::thread,
    sync::mutex::SpinNoIrqLock,
    utils::{
        error::{GeneralRet, SyscallErr, SyscallRet},
        random::RNG,
    },
};

type Mutex<T> = SpinNoIrqLock<T>;

pub const TCP_MSS: u32 = 32768;
pub struct TcpSocket {
    inner: Mutex<TcpSocketInner>,
    file_meta: FileMeta,
}

#[allow(unused)]
struct TcpSocketInner {
    socket_handler: SocketHandle,
    mss: u32,
    local_endpoint: IpListenEndpoint,
    recvbuf_size: usize,
    sendbuf_size: usize,
    // TODO: add more
}

impl TcpSocket {
    pub fn new() -> Self {
        let tx_buf = socket::tcp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        let rx_buf = socket::tcp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        let sock = socket::tcp::Socket::new(rx_buf, tx_buf);
        Self {
            inner: Mutex::new(TcpSocketInner {
                // socket_handler: NET_INTERFACE.add_socket(socket::Socket::Tcp(sock)),
                socket_handler: todo!(),
                mss: TCP_MSS,
                local_endpoint: IpListenEndpoint {
                    addr: None,
                    port: unsafe { RNG.positive_u32() as u16 },
                },
                recvbuf_size: MAX_BUFFER_SIZE,
                sendbuf_size: MAX_BUFFER_SIZE,
            }),
            file_meta: FileMeta::new(OpenFlags::CLOEXEC | OpenFlags::RDWR),
        }
    }
    pub fn is_ipv4(&self) -> bool {
        let inner = self.inner.lock();
        match inner.local_endpoint.addr.unwrap() {
            IpAddress::Ipv4(_) => true,
            IpAddress::Ipv6(_) => false,
        }
    }

    pub fn socket_handler(&self) -> SocketHandle {
        self.inner.lock().socket_handler
    }

    pub fn recv_buf_size(&self) -> usize {
        self.inner.lock().recvbuf_size
    }
    pub fn set_recv_buf_size(&self, size: usize) {
        self.inner.lock().recvbuf_size = size;
    }
    pub fn send_buf_size(&self) -> usize {
        self.inner.lock().sendbuf_size
    }
    pub fn set_send_buf_size(&self, size: usize) {
        self.inner.lock().sendbuf_size = size;
    }

    pub fn addr(&self) -> IpEndpoint {
        let local = self.inner.lock().local_endpoint.clone();
        let addr = if local.addr.is_none() {
            IpAddress::v4(127, 0, 0, 1)
        } else {
            local.addr.unwrap()
        };
        IpEndpoint::new(addr, local.port)
    }

    pub fn peer_addr(&self) -> Option<IpEndpoint> {
        // NET_INTERFACE
        //     .get_tcp(self.socket_handler())
        //     .remote_endpoint()
        todo!()
    }

    pub fn bind(&self, addr: IpListenEndpoint) -> SyscallRet {
        info!("[Tcp::bind] bind addr: {:?}", addr);
        self.inner.lock().local_endpoint = addr;
        Ok(0)
    }

    pub fn listen(&self) -> SyscallRet {
        let inner = self.inner.lock();
        let local = inner.local_endpoint;
        info!("[Tcp::listen] listening: {:?}", local);
        // NET_INTERFACE
        //     .get_tcp(inner.socket_handler)
        //     .listen(local)
        //     .ok()
        //     .ok_or(SyscallErr::EADDRINUSE)?;
        Ok(0)
    }

    pub async fn accept(&self) -> GeneralRet<IpEndpoint> {
        loop {
            // if !NET_INTERFACE.get_tcp(self.socket_handler()).is_open() {
            //     return Err(SyscallErr::EINVAL);
            // }
            // if !NET_INTERFACE.get_tcp(self.socket_handler()).is_listening() {
            //     return Err(SyscallErr::EINVAL);
            // }
            // if NET_INTERFACE
            //     .get_tcp(self.socket_handler())
            //     .remote_endpoint()
            //     .is_none()
            // {
            //     thread::yield_now().await;
            // } else {
            //     return Ok(NET_INTERFACE
            //         .get_tcp(self.socket_handler())
            //         .remote_endpoint()
            //         .unwrap());
            // }
            todo!()
        }
    }

    pub async fn connect(&self, remote_endpoint: IpEndpoint) -> SyscallRet {
        loop {
            let local = self.inner.lock().local_endpoint;
            let handler = self.socket_handler();
            debug!(
                "[Tcp::connect] local: {:?}, remote: {:?}",
                local, remote_endpoint
            );
            // let ret = NET_INTERFACE.inner_handler(|inner| {
            //     inner.sockets.get_mut::<tcp::Socket>(handler).connect(
            //         inner.iface.context(),
            //         remote_endpoint,
            //         local,
            //     )
            // });
            // if ret.is_err() {
            //     debug!("[Tcp::connect] connect ret: {:?}", ret.err().unwrap());
            //     thread::yield_now().await;
            // } else {
            //     return Ok(0);
            // }
            todo!()
        }
    }
}

impl File for TcpSocket {
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> crate::utils::error::AsyscallRet {
        Box::pin(TcpRecvFuture::new(self, buf))
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> crate::utils::error::AsyscallRet {
        Box::pin(TcpSendFuture::new(self, buf))
    }

    fn metadata(&self) -> &FileMeta {
        &self.file_meta
    }

    fn flags(&self) -> crate::fs::OpenFlags {
        todo!()
    }

    fn pollin(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        debug!("[Tcp::pollin] enter");
        let mut inner = self.inner.lock();
        // if NET_INTERFACE.get_tcp(inner.socket_handler).can_recv() {
        //     log::info!("[Tcp::pollin] recv buf have item");
        //     Ok(true)
        // } else {
        //     if let Some(waker) = waker {
        //         NET_INTERFACE
        //             .get_tcp(inner.socket_handler)
        //             .register_recv_waker(&waker);
        //     }
        //     Ok(false)
        // }
        todo!()
    }

    fn pollout(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        debug!("[Tcp::pollout] enter");
        // let mut inner = self.inner.lock();
        // if NET_INTERFACE.get_tcp(inner.socket_handler).can_send() {
        //     Ok(true)
        // } else {
        //     if let Some(waker) = waker {
        //         NET_INTERFACE
        //             .get_tcp(inner.socket_handler)
        //             .register_send_waker(&waker);
        //     }
        //     Ok(false)
        // }
        todo!()
    }
}

struct TcpRecvFuture<'a> {
    socket: &'a TcpSocket,
    buf: ManagedSlice<'a, u8>,
}

impl<'a> TcpRecvFuture<'a> {
    fn new<S>(socket: &'a TcpSocket, buf: S) -> Self
    where
        S: Into<ManagedSlice<'a, u8>>,
    {
        Self {
            socket,
            buf: buf.into(),
        }
    }
}

impl<'a> Future for TcpRecvFuture<'a> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let mut inner = self.socket.inner.lock();
        // if !NET_INTERFACE.get_tcp(inner.socket_handler).may_recv() {
        //     return Poll::Ready(Err(SyscallErr::ENOTCONN));
        // }
        // if let socket = NET_INTERFACE.get_tcp(inner.socket_handler) {
        //     if socket.can_recv() {
        //         socket.register_recv_waker(cx.waker());
        //         return Poll::Pending;
        //     }
        // }
        // let this = self.get_mut();
        // // TODO: update err code
        // Poll::Ready(
        //     NET_INTERFACE
        //         .get_tcp(inner.socket_handler)
        //         .recv_slice(&mut this.buf)
        //         .ok()
        //         .ok_or(SyscallErr::ENOTCONN),
        // )
        todo!()
    }
}

struct TcpSendFuture<'a> {
    socket: &'a TcpSocket,
    buf: &'a [u8],
}

impl<'a> TcpSendFuture<'a> {
    fn new(socket: &'a TcpSocket, buf: &'a [u8]) -> Self {
        Self { socket, buf }
    }
}

impl<'a> Future for TcpSendFuture<'a> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let mut inner = self.socket.inner.lock();
        // if let socket = NET_INTERFACE.get_tcp(inner.socket_handler) {
        //     if !socket.may_send() {
        //         return Poll::Ready(Err(SyscallErr::ENOTCONN));
        //     }
        //     if !socket.can_send() {
        //         socket.register_send_waker(cx.waker());
        //         return Poll::Pending;
        //     }
        // }
        // let this = self.get_mut();
        // // TODO: modify err code
        // Poll::Ready(
        //     NET_INTERFACE
        //         .get_tcp(inner.socket_handler)
        //         .send_slice(&this.buf)
        //         .ok()
        //         .ok_or(SyscallErr::ENOTCONN),
        // )
        todo!()
    }
}
