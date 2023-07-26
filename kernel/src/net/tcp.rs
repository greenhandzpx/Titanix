use core::{
    future::Future,
    task::{Poll, Waker},
};

use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use managed::ManagedSlice;
use smoltcp::socket;

use crate::{
    fs::{File, FileMeta},
    net::MAX_BUFFER_SIZE,
    sync::mutex::SpinNoIrqLock,
    utils::error::{SyscallErr, SyscallRet},
};

type Mutex<T> = SpinNoIrqLock<T>;

pub struct TcpSocket {
    inner: Mutex<TcpSocketInner>,
}

struct TcpSocketInner {
    socket: socket::tcp::Socket<'static>,
    // TODO: add more
}

impl TcpSocket {
    pub fn new() -> Self {
        let tx_buf = socket::tcp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        let rx_buf = socket::tcp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        Self {
            inner: Mutex::new(TcpSocketInner {
                socket: socket::tcp::Socket::new(rx_buf, tx_buf),
            }),
        }
    }

    pub fn connect(&self) {
        todo!()
    }

    pub fn accept(&self) {
        todo!()
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
        todo!()
    }

    fn flags(&self) -> crate::fs::OpenFlags {
        todo!()
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
        if !inner.socket.may_recv() {
            return Poll::Ready(Err(SyscallErr::ENOTCONN));
        }
        if !inner.socket.can_recv() {
            inner.socket.register_recv_waker(cx.waker());
            return Poll::Pending;
        }
        let this = self.get_mut();
        // TODO: update err code
        Poll::Ready(
            inner
                .socket
                .recv_slice(&mut this.buf)
                .ok()
                .ok_or(SyscallErr::ENOTCONN),
        )
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
        if !inner.socket.may_send() {
            return Poll::Ready(Err(SyscallErr::ENOTCONN));
        }
        if !inner.socket.can_send() {
            inner.socket.register_send_waker(cx.waker());
            return Poll::Pending;
        }
        let this = self.get_mut();
        // TODO: modify err code
        Poll::Ready(
            inner
                .socket
                .send_slice(&this.buf)
                .ok()
                .ok_or(SyscallErr::ENOTCONN),
        )
    }
}
