use core::{future::Future, task::Poll};

use alloc::{boxed::Box, vec};
use managed::ManagedSlice;
use smoltcp::{
    phy::PacketMeta,
    socket::{
        self,
        udp::{PacketMetadata, UdpMetadata},
    },
    wire::{IpAddress, IpEndpoint, IpListenEndpoint},
};

use crate::{
    fs::{File, FileMeta, OpenFlags},
    sync::mutex::SpinNoIrqLock,
    utils::error::{SyscallErr, SyscallRet},
};

use super::MAX_BUFFER_SIZE;

type Mutex<T> = SpinNoIrqLock<T>;

const UDP_PACKET_SIZE: usize = 1472;
const MAX_PACKET: usize = MAX_BUFFER_SIZE / UDP_PACKET_SIZE;
pub struct UdpSocket {
    inner: Mutex<UdpSocketInner>,
    file_meta: FileMeta,
}

#[allow(unused)]
struct UdpSocketInner {
    socket: socket::udp::Socket<'static>,
    max_packet: usize,
    remote_endpoint: Option<IpEndpoint>,
    recvbuf_size: usize,
    sendbuf_size: usize,
}

impl UdpSocket {
    pub fn new() -> Self {
        let tx_buf = socket::udp::PacketBuffer::new(
            vec![PacketMetadata::EMPTY, PacketMetadata::EMPTY],
            vec![0 as u8; MAX_BUFFER_SIZE],
        );
        let rx_buf = socket::udp::PacketBuffer::new(
            vec![PacketMetadata::EMPTY, PacketMetadata::EMPTY],
            vec![0 as u8; MAX_BUFFER_SIZE],
        );
        Self {
            inner: Mutex::new(UdpSocketInner {
                max_packet: MAX_PACKET,
                socket: socket::udp::Socket::new(rx_buf, tx_buf),
                remote_endpoint: None,
                recvbuf_size: MAX_BUFFER_SIZE,
                sendbuf_size: MAX_BUFFER_SIZE,
            }),
            file_meta: FileMeta::new(OpenFlags::CLOEXEC | OpenFlags::RDWR),
        }
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
        let local = self.inner.lock().socket.endpoint();
        let addr = if local.addr.is_none() {
            IpAddress::v4(127, 0, 0, 1)
        } else {
            local.addr.unwrap()
        };
        IpEndpoint::new(addr, local.port)
    }

    pub fn peer_addr(&self) -> Option<IpEndpoint> {
        self.inner.lock().remote_endpoint
    }

    pub fn bind(&self, addr: IpListenEndpoint) -> SyscallRet {
        self.inner
            .lock()
            .socket
            .bind(addr)
            .ok()
            .ok_or(SyscallErr::EINVAL)?;
        Ok(0)
    }

    pub async fn connect(&self, remote_endpoint: IpEndpoint) -> SyscallRet {
        let mut inner = self.inner.lock();
        inner.remote_endpoint = Some(remote_endpoint);
        Ok(0)
    }
}

impl File for UdpSocket {
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> crate::utils::error::AsyscallRet {
        Box::pin(UdpRecvFuture::new(self, buf))
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> crate::utils::error::AsyscallRet {
        Box::pin(UdpSendFuture::new(self, buf))
    }

    fn metadata(&self) -> &crate::fs::FileMeta {
        &self.file_meta
    }

    fn flags(&self) -> crate::fs::OpenFlags {
        self.file_meta.inner.lock().flags
    }

    fn pollin(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        let mut inner = self.inner.lock();
        if inner.socket.can_recv() {
            Ok(true)
        } else {
            if let Some(waker) = waker {
                inner.socket.register_recv_waker(&waker);
            }
            Ok(false)
        }
    }

    fn pollout(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        let mut inner = self.inner.lock();
        if inner.socket.can_send() {
            Ok(true)
        } else {
            if let Some(waker) = waker {
                inner.socket.register_send_waker(&waker);
            }
            Ok(false)
        }
    }
}

struct UdpRecvFuture<'a> {
    socket: &'a UdpSocket,
    buf: ManagedSlice<'a, u8>,
}

impl<'a> UdpRecvFuture<'a> {
    fn new<S>(socket: &'a UdpSocket, buf: S) -> Self
    where
        S: Into<ManagedSlice<'a, u8>>,
    {
        Self {
            socket,
            buf: buf.into(),
        }
    }
}

impl<'a> Future for UdpRecvFuture<'a> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let mut inner = self.socket.inner.lock();
        if !inner.socket.can_recv() {
            inner.socket.register_recv_waker(cx.waker());
        }
        let this = self.get_mut();
        Poll::Ready({
            let (ret, _) = inner
                .socket
                .recv_slice(&mut this.buf)
                .ok()
                .ok_or(SyscallErr::ENOTCONN)?;
            Ok(ret)
        })
    }
}

#[allow(unused)]
struct UdpSendFuture<'a> {
    socket: &'a UdpSocket,
    buf: &'a [u8],
}

impl<'a> UdpSendFuture<'a> {
    fn new(socket: &'a UdpSocket, buf: &'a [u8]) -> Self {
        Self { socket, buf }
    }
}

impl<'a> Future for UdpSendFuture<'a> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let mut inner = self.socket.inner.lock();
        if !inner.socket.can_send() {
            inner.socket.register_send_waker(cx.waker());
            return Poll::Pending;
        }
        let this = self.get_mut();
        let meta = UdpMetadata {
            endpoint: inner.remote_endpoint.unwrap(),
            meta: PacketMeta::default(),
        };
        let len = this.buf.len();
        // TODO: update err code
        Poll::Ready({
            inner
                .socket
                .send_slice(&this.buf, meta)
                .ok()
                .ok_or(SyscallErr::ENOBUFS)?;
            Ok(len)
        })
    }
}
