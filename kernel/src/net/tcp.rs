use core::{future::Future, task::Poll, time::Duration};

use alloc::{boxed::Box, vec};
use log::info;
use managed::ManagedSlice;
use smoltcp::{
    iface::SocketHandle,
    socket::{self, tcp},
    wire::{IpAddress, IpEndpoint, IpListenEndpoint},
};

use crate::{
    fs::{File, FileMeta, OpenFlags},
    net::{config::NET_INTERFACE, MAX_BUFFER_SIZE, SHUT_WR},
    processor::{current_task, SumGuard},
    stack_trace,
    sync::Event,
    timer::timeout_task::ksleep,
    utils::{
        async_tools::{Select2Futures, SelectOutput},
        error::{GeneralRet, SyscallErr, SyscallRet},
        random::RNG,
    },
};

use super::Mutex;

pub const TCP_MSS_DEFAULT: u32 = 1 << 15;
pub const TCP_MSS: u32 = if TCP_MSS_DEFAULT > MAX_BUFFER_SIZE as u32 {
    MAX_BUFFER_SIZE as u32
} else {
    TCP_MSS_DEFAULT
};

pub struct TcpSocket {
    inner: Mutex<TcpSocketInner>,
    socket_handler: SocketHandle,
    file_meta: FileMeta,
}

#[allow(unused)]
struct TcpSocketInner {
    local_endpoint: IpListenEndpoint,
    last_state: tcp::State,
    recvbuf_size: usize,
    sendbuf_size: usize,
    // TODO: add more
}

impl TcpSocket {
    pub fn new() -> Self {
        stack_trace!();
        let tx_buf = socket::tcp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        let rx_buf = socket::tcp::SocketBuffer::new(vec![0 as u8; MAX_BUFFER_SIZE]);
        let socket = socket::tcp::Socket::new(rx_buf, tx_buf);
        let socket_handler = NET_INTERFACE.add_socket(socket);
        log::info!("[TcpSocket::new] new {}", socket_handler);
        NET_INTERFACE.poll();
        Self {
            socket_handler,
            inner: Mutex::new(TcpSocketInner {
                local_endpoint: IpListenEndpoint {
                    addr: None,
                    port: unsafe { RNG.positive_u32() as u16 },
                },
                last_state: tcp::State::Closed,
                recvbuf_size: MAX_BUFFER_SIZE,
                sendbuf_size: MAX_BUFFER_SIZE,
            }),
            file_meta: FileMeta::new(
                OpenFlags::CLOEXEC | OpenFlags::RDWR,
                crate::fs::InodeMode::FileSOCK,
            ),
        }
    }
    pub fn is_ipv4(&self) -> bool {
        let inner = self.inner.lock();
        match inner.local_endpoint.addr.unwrap() {
            IpAddress::Ipv4(_) => true,
            IpAddress::Ipv6(_) => false,
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
    pub fn set_nagle_enabled(&self, enabled: bool) {
        NET_INTERFACE.tcp_socket(self.socket_handler, |socket| {
            socket.set_nagle_enabled(enabled)
        })
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
        NET_INTERFACE.poll();
        let ret = NET_INTERFACE.tcp_socket(self.socket_handler, |socket| socket.remote_endpoint());
        NET_INTERFACE.poll();
        ret
    }

    pub fn bind(&self, addr: IpListenEndpoint) -> SyscallRet {
        info!("[Tcp::bind] bind to: {:?}", addr);
        self.inner.lock().local_endpoint = addr;
        Ok(0)
    }

    pub fn listen(&self) -> SyscallRet {
        let local = self.inner.lock().local_endpoint;
        info!("[Tcp::listen] listening: {:?}", local);
        NET_INTERFACE.poll();
        NET_INTERFACE.tcp_socket(self.socket_handler, |socket| {
            let ret = socket.listen(local).ok().ok_or(SyscallErr::EADDRINUSE);
            self.inner.lock().last_state = socket.state();
            ret
        })?;
        NET_INTERFACE.poll();
        Ok(0)
    }

    /// TODO: change to future
    pub async fn accept(&self) -> GeneralRet<IpEndpoint> {
        match Select2Futures::new(
            TcpAcceptFuture::new(self),
            current_task().wait_for_events(Event::all()),
        )
        .await
        {
            SelectOutput::Output1(ret) => ret,
            SelectOutput::Output2(intr) => {
                log::info!("[TcpSocket::accept] interrupt by {:?}", intr);
                Err(SyscallErr::EINTR)
            }
        }
    }

    pub fn _connect(&self, remote_endpoint: IpEndpoint) -> GeneralRet<()> {
        NET_INTERFACE.poll();
        let local = self.inner.lock().local_endpoint;
        info!(
            "[Tcp::connect] local: {:?}, remote: {:?}",
            local, remote_endpoint
        );
        NET_INTERFACE.inner_handler(|inner| {
            let socket = inner.sockets.get_mut::<tcp::Socket>(self.socket_handler);
            let ret = socket.connect(inner.iface.context(), remote_endpoint, local);
            if ret.is_err() {
                log::info!("[Tcp::connect] {} connect error occur", self.socket_handler);
                match ret.err().unwrap() {
                    tcp::ConnectError::Unaddressable => return Err(SyscallErr::EINVAL),
                    tcp::ConnectError::InvalidState => return Err(SyscallErr::EISCONN),
                }
            }
            info!("berfore poll socket state: {}", socket.state());
            Ok(())
        })?;
        NET_INTERFACE.poll();
        Ok(())
    }

    pub async fn connect(&self, remote_endpoint: IpEndpoint) -> SyscallRet {
        self._connect(remote_endpoint)?;
        loop {
            NET_INTERFACE.poll();
            let state = NET_INTERFACE.tcp_socket(self.socket_handler, |socket| socket.state());
            match state {
                tcp::State::Closed => {
                    // close but not already connect, retry
                    info!(
                        "[Tcp::connect] {} already closed, try again",
                        self.socket_handler
                    );
                    self._connect(remote_endpoint)?;
                }
                tcp::State::Established => {
                    info!(
                        "[Tcp::connect] {} connected, state {:?}",
                        self.socket_handler, state
                    );
                    return Ok(0);
                }
                _ => {
                    info!(
                        "[Tcp::connect] {} not connect yet, state {:?}",
                        self.socket_handler, state
                    );
                }
            }
        }
    }

    pub fn shutdown(&self, how: u32) -> GeneralRet<()> {
        log::info!("[TcpSocket::shutdown] how {}", how);
        NET_INTERFACE.tcp_socket(self.socket_handler, |socket| match how {
            SHUT_WR => socket.close(),
            _ => socket.abort(),
        });
        NET_INTERFACE.poll();
        Ok(())
    }
}

impl Drop for TcpSocket {
    fn drop(&mut self) {
        log::info!(
            "[TcpSocket::drop] drop socket {}, localep {:?}",
            self.socket_handler,
            self.inner.lock().local_endpoint
        );
        NET_INTERFACE.tcp_socket(self.socket_handler, |socket| {
            log::info!("[TcpSocket::drop] before state is {:?}", socket.state());
            if socket.is_open() {
                socket.close();
            }
            log::info!("[TcpSocket::drop] after state is {:?}", socket.state());
        });
        NET_INTERFACE.poll();
        NET_INTERFACE.remove(self.socket_handler);
        NET_INTERFACE.poll();
    }
}

impl File for TcpSocket {
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> crate::utils::error::AsyscallRet {
        log::info!("[Tcp::read] {} enter", self.socket_handler);
        Box::pin(async move {
            match Select2Futures::new(
                TcpRecvFuture::new(self, buf),
                current_task().wait_for_events(Event::THREAD_EXIT | Event::PROCESS_EXIT),
            )
            .await
            {
                SelectOutput::Output1(ret) => {
                    ksleep(Duration::from_millis(3)).await;
                    ret
                }
                SelectOutput::Output2(intr) => {
                    log::info!("[TcpSocket::read] interrupt by event {:?}", intr);
                    Err(SyscallErr::EINTR)
                }
            }
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> crate::utils::error::AsyscallRet {
        log::info!("[Tcp::write] {} enter", self.socket_handler);
        Box::pin(async move {
            match Select2Futures::new(
                TcpSendFuture::new(self, buf),
                current_task().wait_for_events(Event::THREAD_EXIT | Event::PROCESS_EXIT),
            )
            .await
            {
                SelectOutput::Output1(ret) => {
                    ksleep(Duration::from_millis(3)).await;
                    ret
                }
                SelectOutput::Output2(intr) => {
                    log::info!("[TcpSocket::write] interrupt by event {:?}", intr);
                    Err(SyscallErr::EINTR)
                }
            }
        })
    }

    fn metadata(&self) -> &FileMeta {
        &self.file_meta
    }

    fn flags(&self) -> crate::fs::OpenFlags {
        self.file_meta.inner.lock().flags
    }

    fn pollin(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        info!("[Tcp::pollin] {} enter", self.socket_handler);
        NET_INTERFACE.poll();
        NET_INTERFACE.tcp_socket(self.socket_handler, |socket| {
            if socket.can_recv() {
                log::info!("[Tcp::pollin] {} recv buf have item", self.socket_handler);
                Ok(true)
            } else if socket.state() == tcp::State::CloseWait
                || socket.state() == tcp::State::FinWait2
                || socket.state() == tcp::State::TimeWait
                || (self.inner.lock().last_state == tcp::State::Listen
                    && socket.state() == tcp::State::Established)
                || socket.state() == tcp::State::SynReceived
            {
                log::info!("[Tcp::pollin] state become {:?}", socket.state());
                Ok(true)
            } else {
                log::info!("[Tcp::pollin] nothing to read, state {:?}", socket.state());
                if let Some(waker) = waker {
                    socket.register_recv_waker(&waker);
                }
                Ok(false)
            }
        })
    }

    fn pollout(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        info!("[Tcp::pollout] {} enter", self.socket_handler);
        NET_INTERFACE.poll();
        NET_INTERFACE.tcp_socket(self.socket_handler, |socket| {
            if socket.can_send() {
                log::info!("[Tcp::pollout] {} tx buf have slots", self.socket_handler);
                Ok(true)
            } else {
                if let Some(waker) = waker {
                    socket.register_send_waker(&waker);
                }
                Ok(false)
            }
        })
    }
}

struct TcpAcceptFuture<'a> {
    socket: &'a TcpSocket,
}

impl<'a> TcpAcceptFuture<'a> {
    fn new(socket: &'a TcpSocket) -> Self {
        Self { socket }
    }
}

impl<'a> Future for TcpAcceptFuture<'a> {
    type Output = GeneralRet<IpEndpoint>;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> Poll<Self::Output> {
        NET_INTERFACE.poll();
        let ret = NET_INTERFACE.tcp_socket(self.socket.socket_handler, |socket| {
            if !socket.is_open() {
                log::info!("[TcpAcceptFuture::poll] this socket is not open");
                return Poll::Ready(Err(SyscallErr::EINVAL));
            }
            if socket.state() == tcp::State::SynReceived
                || socket.state() == tcp::State::Established
            {
                self.socket.inner.lock().last_state = socket.state();
                log::info!("[TcpAcceptFuture::poll] state become {:?}", socket.state());
                return Poll::Ready(Ok(socket.remote_endpoint().unwrap()));
            }
            log::info!(
                "[TcpAcceptFuture::poll] not syn yet, state {:?}",
                socket.state()
            );
            socket.register_recv_waker(cx.waker());
            Poll::Pending
        });
        NET_INTERFACE.poll();
        ret
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
        let _sum_guard = SumGuard::new();
        NET_INTERFACE.poll();
        let ret = NET_INTERFACE.tcp_socket(self.socket.socket_handler, |socket| {
            if socket.state() == tcp::State::CloseWait || socket.state() == tcp::State::TimeWait {
                log::info!("[TcpRecvFuture::poll] state become {:?}", socket.state());
                return Poll::Ready(Ok(0));
            }
            if !socket.may_recv() {
                log::info!(
                    "[TcpRecvFuture::poll] err when recv, state {:?}",
                    socket.state()
                );
                return Poll::Ready(Err(SyscallErr::ENOTCONN));
            }
            log::info!("[TcpRecvFuture::poll] state {:?}", socket.state());
            if !socket.can_recv() {
                socket.register_recv_waker(cx.waker());
                log::info!("[TcpRecvFuture::poll] cannot recv yet");
                return Poll::Pending;
            }
            log::info!("[TcpRecvFuture::poll] start to recv...");
            let this = self.get_mut();
            info!(
                "[TcpRecvFuture::poll] {:?} <- {:?}",
                socket.local_endpoint(),
                socket.remote_endpoint()
            );
            Poll::Ready(match socket.recv_slice(&mut this.buf) {
                Ok(nbytes) => {
                    log::info!("[TcpRecvFuture::poll] recv {} bytes", nbytes);
                    Ok(nbytes)
                }
                Err(_) => Err(SyscallErr::ENOTCONN),
            })
        });
        NET_INTERFACE.poll();
        ret
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
        let _sum_guard = SumGuard::new();
        NET_INTERFACE.poll();
        let ret = NET_INTERFACE.tcp_socket(self.socket.socket_handler, |socket| {
            if !socket.may_send() {
                log::info!("[TcpSendFuture::poll] err when send");
                return Poll::Ready(Err(SyscallErr::ENOTCONN));
            }
            if !socket.can_send() {
                socket.register_send_waker(cx.waker());
                log::info!("[TcpSendFuture::poll] cannot send yet");
                return Poll::Pending;
            }
            log::info!("[TcpSendFuture::poll] start to send...");
            let this = self.get_mut();
            info!(
                "[TcpSendFuture::poll] {:?} -> {:?}",
                socket.local_endpoint(),
                socket.remote_endpoint()
            );
            Poll::Ready(match socket.send_slice(&mut this.buf) {
                Ok(nbytes) => {
                    log::info!("[TcpSendFuture::poll] send {} bytes", nbytes);
                    Ok(nbytes)
                }
                Err(_) => Err(SyscallErr::ENOTCONN),
            })
        });
        NET_INTERFACE.poll();
        ret
    }
}
