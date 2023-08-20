use super::{address::SocketAddrv4, config::NET_INTERFACE, Mutex, Socket, MAX_BUFFER_SIZE};
use crate::{
    fs::{File, FileMeta, OpenFlags},
    net::address,
    process::thread,
    processor::{current_task, SumGuard},
    stack_trace,
    sync::Event,
    timer::timeout_task::ksleep,
    utils::{
        async_utils::{Select2Futures, SelectOutput},
        error::{GeneralRet, SyscallErr, SyscallRet},
    },
};
use alloc::{boxed::Box, vec};
use core::{future::Future, task::Poll, time::Duration};
use log::{debug, info};
use managed::ManagedSlice;
use smoltcp::{
    iface::SocketHandle,
    phy::PacketMeta,
    socket::{
        self,
        udp::{PacketMetadata, SendError, UdpMetadata},
    },
    wire::{IpEndpoint, IpListenEndpoint},
};

pub struct UdpSocket {
    inner: Mutex<UdpSocketInner>,
    socket_handler: SocketHandle,
    file_meta: FileMeta,
}

#[allow(unused)]
struct UdpSocketInner {
    remote_endpoint: Option<IpEndpoint>,
    recvbuf_size: usize,
    sendbuf_size: usize,
}

impl Socket for UdpSocket {
    fn bind(&self, addr: IpListenEndpoint) -> SyscallRet {
        stack_trace!();
        log::info!("[Udp::bind] bind to {:?}", addr);
        NET_INTERFACE.poll();
        NET_INTERFACE.udp_socket(self.socket_handler, |socket| {
            socket.bind(addr).ok().ok_or(SyscallErr::EINVAL)
        })?;
        NET_INTERFACE.poll();
        Ok(0)
    }

    fn listen(&self) -> SyscallRet {
        stack_trace!();
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn connect<'a>(&'a self, addr_buf: &'a [u8]) -> crate::utils::error::AsyscallRet {
        stack_trace!();
        Box::pin(async move {
            let remote_endpoint = address::endpoint(addr_buf)?;
            log::info!("[Udp::connect] connect to {:?}", remote_endpoint);
            let mut inner = self.inner.lock();
            inner.remote_endpoint = Some(remote_endpoint);
            NET_INTERFACE.poll();
            NET_INTERFACE.udp_socket(self.socket_handler, |socket| {
                let local = socket.endpoint();
                info!("[Udp::connect] local: {:?}", local);
                if local.port == 0 {
                    info!("[Udp::connect] don't have local");
                    let addr = SocketAddrv4::new([0; 16].as_slice());
                    let endpoint = IpListenEndpoint::from(addr);
                    let ret = socket.bind(endpoint);
                    if ret.is_err() {
                        match ret.err().unwrap() {
                            socket::udp::BindError::Unaddressable => {
                                info!("[Udp::bind] unaddr");
                                return Err(SyscallErr::EINVAL);
                            }
                            socket::udp::BindError::InvalidState => {
                                info!("[Udp::bind] invaild state");
                                return Err(SyscallErr::EINVAL);
                            }
                        }
                    }
                    log::info!("[Udp::bind] bind to {:?}", endpoint);
                    Ok(())
                } else {
                    Ok(())
                }
            })?;
            NET_INTERFACE.poll();
            Ok(0)
        })
    }

    fn accept(
        &self,
        _sockfd: u32,
        _addr: usize,
        _addrlen: usize,
    ) -> crate::utils::error::AsyscallRet {
        stack_trace!();
        Box::pin(async move { Err(SyscallErr::EOPNOTSUPP) })
    }

    fn socket_type(&self) -> super::SocketType {
        stack_trace!();
        super::SocketType::SOCK_DGRAM
    }

    fn recv_buf_size(&self) -> usize {
        stack_trace!();
        self.inner.lock().recvbuf_size
    }

    fn set_recv_buf_size(&self, size: usize) {
        stack_trace!();
        self.inner.lock().recvbuf_size = size;
    }

    fn send_buf_size(&self) -> usize {
        stack_trace!();
        self.inner.lock().sendbuf_size
    }

    fn set_send_buf_size(&self, size: usize) {
        stack_trace!();
        self.inner.lock().sendbuf_size = size;
    }

    fn loacl_endpoint(&self) -> IpListenEndpoint {
        stack_trace!();
        NET_INTERFACE.poll();
        let local = NET_INTERFACE.udp_socket(self.socket_handler, |socket| socket.endpoint());
        NET_INTERFACE.poll();
        local
    }

    fn remote_endpoint(&self) -> Option<IpEndpoint> {
        stack_trace!();
        self.inner.lock().remote_endpoint
    }

    fn shutdown(&self, how: u32) -> GeneralRet<()> {
        stack_trace!();
        log::info!("[UdpSocket::shutdown] how {}", how);
        Ok(())
    }

    fn set_nagle_enabled(&self, _enabled: bool) -> SyscallRet {
        stack_trace!();
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn set_keep_alive(&self, _enabled: bool) -> SyscallRet {
        stack_trace!();
        Err(SyscallErr::EOPNOTSUPP)
    }
}

impl UdpSocket {
    pub fn new() -> Self {
        stack_trace!();
        let tx_buf = socket::udp::PacketBuffer::new(
            vec![PacketMetadata::EMPTY, PacketMetadata::EMPTY],
            vec![0 as u8; MAX_BUFFER_SIZE],
        );
        let rx_buf = socket::udp::PacketBuffer::new(
            vec![PacketMetadata::EMPTY, PacketMetadata::EMPTY],
            vec![0 as u8; MAX_BUFFER_SIZE],
        );
        let socket = socket::udp::Socket::new(rx_buf, tx_buf);
        let socket_handler = NET_INTERFACE.add_socket(socket);
        log::info!("[UdpSocket::new] new {}", socket_handler);
        NET_INTERFACE.poll();
        Self {
            inner: Mutex::new(UdpSocketInner {
                remote_endpoint: None,
                recvbuf_size: MAX_BUFFER_SIZE,
                sendbuf_size: MAX_BUFFER_SIZE,
            }),
            socket_handler,
            file_meta: FileMeta::new(crate::fs::InodeMode::FileSOCK),
        }
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        stack_trace!();
        log::info!(
            "[UdpSocket::drop] drop socket {}, remoteep {:?}",
            self.socket_handler,
            self.inner.lock().remote_endpoint
        );
        NET_INTERFACE.udp_socket(self.socket_handler, |socket| {
            if socket.is_open() {
                socket.close();
            }
        });
        NET_INTERFACE.remove(self.socket_handler);
        NET_INTERFACE.poll();
    }
}

impl File for UdpSocket {
    fn read<'a>(&'a self, buf: &'a mut [u8], flags: OpenFlags) -> crate::utils::error::AsyscallRet {
        stack_trace!();
        log::info!("[Ucp::read] {} enter", self.socket_handler);
        Box::pin(async move {
            match Select2Futures::new(
                UdpRecvFuture::new(self, buf, flags),
                current_task().wait_for_events(Event::THREAD_EXIT | Event::PROCESS_EXIT),
            )
            .await
            {
                SelectOutput::Output1(ret) => match ret {
                    Ok(len) => {
                        if len > MAX_BUFFER_SIZE / 2 {
                            // need to be slow
                            ksleep(Duration::from_millis(2)).await;
                        } else {
                            #[cfg(not(feature = "multi_hart"))]
                            thread::yield_now().await;
                        }
                        ret
                    }
                    Err(_) => ret,
                },
                SelectOutput::Output2(intr) => {
                    log::info!("[TcpSocket::read] interrupt by event {:?}", intr);
                    Err(SyscallErr::EINTR)
                }
            }
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8], flags: OpenFlags) -> crate::utils::error::AsyscallRet {
        stack_trace!();
        log::info!("[Udp::write] {} enter", self.socket_handler);
        Box::pin(async move {
            match Select2Futures::new(
                UdpSendFuture::new(self, buf, flags),
                current_task().wait_for_events(Event::THREAD_EXIT | Event::PROCESS_EXIT),
            )
            .await
            {
                SelectOutput::Output1(ret) => match ret {
                    Ok(len) => {
                        if len > MAX_BUFFER_SIZE / 2 {
                            // need to be slow
                            ksleep(Duration::from_millis(2)).await;
                        } else {
                            #[cfg(not(feature = "multi_hart"))]
                            thread::yield_now().await;
                        }
                        ret
                    }
                    Err(_) => ret,
                },
                SelectOutput::Output2(intr) => {
                    log::info!("[TcpSocket::write] interrupt by event {:?}", intr);
                    Err(SyscallErr::EINTR)
                }
            }
        })
    }

    fn metadata(&self) -> &crate::fs::FileMeta {
        stack_trace!();
        &self.file_meta
    }

    fn pollin(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        stack_trace!();
        debug!("[Udp::pollin] {} enter", self.socket_handler);
        NET_INTERFACE.poll();
        NET_INTERFACE.udp_socket(self.socket_handler, |socket| {
            if socket.can_recv() {
                log::info!("[Udp::pollin] {} recv buf have item", self.socket_handler);
                Ok(true)
            } else {
                if let Some(waker) = waker {
                    socket.register_recv_waker(&waker);
                }
                Ok(false)
            }
        })
    }

    fn pollout(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        stack_trace!();
        debug!("[Udp::pollout] {} enter", self.socket_handler);
        NET_INTERFACE.poll();
        NET_INTERFACE.udp_socket(self.socket_handler, |socket| {
            if socket.can_send() {
                log::info!("[Udp::pollout] {} tx buf have slots", self.socket_handler);
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

struct UdpRecvFuture<'a> {
    socket: &'a UdpSocket,
    buf: ManagedSlice<'a, u8>,
    flags: OpenFlags,
}

impl<'a> UdpRecvFuture<'a> {
    fn new<S>(socket: &'a UdpSocket, buf: S, flags: OpenFlags) -> Self
    where
        S: Into<ManagedSlice<'a, u8>>,
    {
        Self {
            socket,
            buf: buf.into(),
            flags,
        }
    }
}

impl<'a> Future for UdpRecvFuture<'a> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        stack_trace!();
        let _sum_guard = SumGuard::new();
        NET_INTERFACE.poll();
        let ret = NET_INTERFACE.udp_socket(self.socket.socket_handler, |socket| {
            if !socket.can_recv() {
                log::info!("[UdpRecvFuture::poll] cannot recv yet");
                if self.flags.contains(OpenFlags::NONBLOCK) {
                    log::info!("[UdpRecvFuture::poll] already set nonblock");
                    return Poll::Ready(Err(SyscallErr::EAGAIN));
                }
                socket.register_recv_waker(cx.waker());
                return Poll::Pending;
            }
            log::info!("[UdpRecvFuture::poll] start to recv...");
            let this = self.get_mut();
            Poll::Ready({
                let (ret, meta) = socket
                    .recv_slice(&mut this.buf)
                    .ok()
                    .ok_or(SyscallErr::ENOTCONN)?;
                let remote = Some(meta.endpoint);
                info!(
                    "[UdpRecvFuture::poll] {:?} <- {:?}",
                    socket.endpoint(),
                    remote
                );
                this.socket.inner.lock().remote_endpoint = remote;
                log::debug!("[UdpRecvFuture::poll] recv {} bytes", ret);
                Ok(ret)
            })
        });
        NET_INTERFACE.poll();
        ret
    }
}

#[allow(unused)]
struct UdpSendFuture<'a> {
    socket: &'a UdpSocket,
    buf: &'a [u8],
    flags: OpenFlags,
}

impl<'a> UdpSendFuture<'a> {
    fn new(socket: &'a UdpSocket, buf: &'a [u8], flags: OpenFlags) -> Self {
        Self { socket, buf, flags }
    }
}

impl<'a> Future for UdpSendFuture<'a> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        stack_trace!();
        let _sum_guard = SumGuard::new();
        NET_INTERFACE.poll();
        let ret = NET_INTERFACE.udp_socket(self.socket.socket_handler, |socket| {
            if !socket.can_send() {
                log::info!("[UdpSendFuture::poll] cannot send yet");
                if self.flags.contains(OpenFlags::NONBLOCK) {
                    log::info!("[UdpSendFuture::poll] already set nonblock");
                    return Poll::Ready(Err(SyscallErr::EAGAIN));
                }
                socket.register_send_waker(cx.waker());
                return Poll::Pending;
            }
            log::info!("[UdpSendFuture::poll] start to send...");
            let remote = self.socket.inner.lock().remote_endpoint;
            let this = self.get_mut();
            let meta = UdpMetadata {
                endpoint: remote.unwrap(),
                meta: PacketMeta::default(),
            };
            let len = this.buf.len();
            info!(
                "[UdpSendFuture::poll] {:?} -> {:?}",
                socket.endpoint(),
                remote
            );
            // TODO: update err code
            let ret = socket.send_slice(&this.buf, meta);
            Poll::Ready(if let Some(err) = ret.err() {
                if err == SendError::Unaddressable {
                    Err(SyscallErr::ENOTCONN)
                } else {
                    Err(SyscallErr::ENOBUFS)
                }
            } else {
                log::debug!("[UdpSendFuture::poll] send {} bytes", len);
                Ok(len)
            })
        });
        NET_INTERFACE.poll();
        ret
    }
}
