use super::{address::SocketAddrv4, config::NET_INTERFACE, Mutex, Socket, MAX_BUFFER_SIZE};
use crate::{
    fs::{File, FileMeta, OpenFlags},
    net::{address, RecvFromFlags},
    process::thread,
    processor::{current_task, SumGuard},
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
    handler_loop: SocketHandle,
    handler_dev: SocketHandle,
    file_meta: FileMeta,
}

#[allow(unused)]
struct UdpSocketInner {
    remote_endpoint: Option<IpEndpoint>,
    local_endpoint: IpListenEndpoint,
    recvbuf_size: usize,
    sendbuf_size: usize,
}

impl Socket for UdpSocket {
    fn bind(&self, addr: IpListenEndpoint) -> SyscallRet {
        log::info!("[Udp::bind] bind to {:?}", addr);
        NET_INTERFACE.udp_socket_loop(self.handler_loop, |socket| {
            socket.bind(addr).ok().ok_or(SyscallErr::EINVAL)
        })?;
        NET_INTERFACE.udp_socket_dev(self.handler_dev, |socket| {
            socket.bind(addr).ok().ok_or(SyscallErr::EINVAL)
        })?;
        self.inner.lock().local_endpoint = addr;
        Ok(0)
    }

    fn listen(&self) -> SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn connect<'a>(&'a self, addr_buf: &'a [u8]) -> crate::utils::error::AsyscallRet {
        Box::pin(async move {
            let remote_endpoint = address::endpoint(addr_buf)?;
            log::info!("[Udp::connect] connect to {:?}", remote_endpoint);
            let mut inner = self.inner.lock();
            inner.remote_endpoint = Some(remote_endpoint);
            let is_local = address::is_local(remote_endpoint);
            let poll_f = |socket: &mut smoltcp::socket::udp::Socket<'_>| {
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
            };
            if is_local {
                NET_INTERFACE.udp_socket_loop(self.handler_loop, poll_f)?;
            } else {
                NET_INTERFACE.udp_socket_loop(self.handler_loop, poll_f)?;
            }
            Ok(0)
        })
    }

    fn accept(
        &self,
        _sockfd: u32,
        _addr: usize,
        _addrlen: usize,
    ) -> crate::utils::error::AsyscallRet {
        Box::pin(async move { Err(SyscallErr::EOPNOTSUPP) })
    }

    fn socket_type(&self) -> super::SocketType {
        super::SocketType::SOCK_DGRAM
    }

    fn recv_buf_size(&self) -> usize {
        self.inner.lock().recvbuf_size
    }

    fn set_recv_buf_size(&self, size: usize) {
        self.inner.lock().recvbuf_size = size;
    }

    fn send_buf_size(&self) -> usize {
        self.inner.lock().sendbuf_size
    }

    fn set_send_buf_size(&self, size: usize) {
        self.inner.lock().sendbuf_size = size;
    }

    fn loacl_endpoint(&self) -> IpListenEndpoint {
        self.inner.lock().local_endpoint
    }

    fn remote_endpoint(&self) -> Option<IpEndpoint> {
        self.inner.lock().remote_endpoint
    }

    fn shutdown(&self, how: u32) -> GeneralRet<()> {
        log::info!("[UdpSocket::shutdown] how {}", how);
        Ok(())
    }

    fn set_nagle_enabled(&self, _enabled: bool) -> SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn set_keep_alive(&self, _enabled: bool) -> SyscallRet {
        Err(SyscallErr::EOPNOTSUPP)
    }

    fn recv<'a>(
        &'a self,
        buf: &'a mut [u8],
        flags: RecvFromFlags,
    ) -> crate::utils::error::AsyscallRet {
        log::info!(
            "[Ucp::recv] ({}, {}) enter",
            self.handler_loop,
            self.handler_dev
        );
        let buf_start = buf.as_ptr() as usize;
        Box::pin(async move {
            match Select2Futures::new(
                Select2Futures::new(
                    UdpRecvFuture::new(self, buf_start, buf.len(), flags, true),
                    UdpRecvFuture::new(self, buf_start, buf.len(), flags, false),
                ),
                current_task().wait_for_events(Event::THREAD_EXIT | Event::PROCESS_EXIT),
            )
            .await
            {
                SelectOutput::Output1(ret) => match ret {
                    SelectOutput::Output1(ret) => match ret {
                        Ok(len) => {
                            if len > MAX_BUFFER_SIZE / 2 {
                                // need to be slow
                                ksleep(Duration::from_millis(1)).await;
                            } else {
                                #[cfg(not(feature = "multi_hart"))]
                                thread::yield_now().await;
                            }
                            ret
                        }
                        Err(_) => ret,
                    },
                    SelectOutput::Output2(ret) => match ret {
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
                },
                SelectOutput::Output2(intr) => {
                    log::info!("[TcpSocket::read] interrupt by event {:?}", intr);
                    Err(SyscallErr::EINTR)
                }
            }
        })
    }

    fn send<'a>(&'a self, buf: &'a [u8], flags: RecvFromFlags) -> crate::utils::error::AsyscallRet {
        log::info!(
            "[Ucp::send] ({}, {}) enter",
            self.handler_loop,
            self.handler_dev
        );
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
                            ksleep(Duration::from_millis(1)).await;
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
        let socket_loop = socket::udp::Socket::new(rx_buf, tx_buf);

        let tx_buf = socket::udp::PacketBuffer::new(
            vec![PacketMetadata::EMPTY, PacketMetadata::EMPTY],
            vec![0 as u8; MAX_BUFFER_SIZE],
        );
        let rx_buf = socket::udp::PacketBuffer::new(
            vec![PacketMetadata::EMPTY, PacketMetadata::EMPTY],
            vec![0 as u8; MAX_BUFFER_SIZE],
        );
        let socket_dev = socket::udp::Socket::new(rx_buf, tx_buf);
        let (handler_loop, handler_dev) = NET_INTERFACE.add_socket(socket_loop, socket_dev);
        log::info!("[UdpSocket::new] new ({}, {})", handler_loop, handler_dev);
        Self {
            inner: Mutex::new(UdpSocketInner {
                remote_endpoint: None,
                local_endpoint: IpListenEndpoint::default(),
                recvbuf_size: MAX_BUFFER_SIZE,
                sendbuf_size: MAX_BUFFER_SIZE,
            }),
            handler_loop,
            handler_dev,
            file_meta: FileMeta::new(crate::fs::InodeMode::FileSOCK),
        }
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        log::info!(
            "[UdpSocket::drop] drop socket ({}, {}), remoteep {:?}",
            self.handler_loop,
            self.handler_dev,
            self.inner.lock().remote_endpoint
        );
        NET_INTERFACE.udp_socket_loop(self.handler_loop, |socket| {
            if socket.is_open() {
                socket.close();
            }
        });
        NET_INTERFACE.udp_socket_dev(self.handler_dev, |socket| {
            if socket.is_open() {
                socket.close();
            }
        });
        NET_INTERFACE.remove(self.handler_loop, self.handler_dev);
        NET_INTERFACE.poll_all();
    }
}

impl File for UdpSocket {
    fn read<'a>(&'a self, buf: &'a mut [u8], flags: OpenFlags) -> crate::utils::error::AsyscallRet {
        log::info!(
            "[Ucp::read] ({}, {}) enter",
            self.handler_loop,
            self.handler_dev
        );
        let mut flags_recv = RecvFromFlags::default();
        if flags.contains(OpenFlags::NONBLOCK) {
            flags_recv = RecvFromFlags::MSG_DONTWAIT;
        }
        self.recv(buf, flags_recv)
    }

    fn write<'a>(&'a self, buf: &'a [u8], flags: OpenFlags) -> crate::utils::error::AsyscallRet {
        log::info!(
            "[Ucp::write] ({}, {}) enter",
            self.handler_loop,
            self.handler_dev
        );
        let mut flags_recv = RecvFromFlags::default();
        if flags.contains(OpenFlags::NONBLOCK) {
            flags_recv = RecvFromFlags::MSG_DONTWAIT;
        }
        self.send(buf, flags_recv)
    }

    fn metadata(&self) -> &crate::fs::FileMeta {
        &self.file_meta
    }

    fn pollin(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        debug!(
            "[Udp::pollin] ({}, {}) enter",
            self.handler_loop, self.handler_dev
        );

        let poll_f = |socket: &mut smoltcp::socket::udp::Socket<'_>| {
            if socket.can_recv() {
                log::info!(
                    "[Udp::pollin] ({}, {}) recv buf have item",
                    self.handler_loop,
                    self.handler_dev
                );
                Ok(true)
            } else {
                if let Some(waker) = waker.clone() {
                    socket.register_recv_waker(&waker);
                }
                Ok(false)
            }
        };
        NET_INTERFACE.poll_all();
        Ok(NET_INTERFACE.udp_socket_loop(self.handler_loop, poll_f)?
            || NET_INTERFACE.udp_socket_dev(self.handler_dev, poll_f)?)
    }

    fn pollout(&self, waker: Option<core::task::Waker>) -> crate::utils::error::GeneralRet<bool> {
        debug!(
            "[Udp::pollout] ({}, {}) enter",
            self.handler_loop, self.handler_dev
        );
        let is_local = address::is_local(self.remote_endpoint().unwrap());
        let poll_f = |socket: &mut smoltcp::socket::udp::Socket<'_>| {
            if socket.can_send() {
                log::info!(
                    "[Udp::pollout] ({}, {}) tx buf have slots",
                    self.handler_loop,
                    self.handler_dev
                );
                Ok(true)
            } else {
                if let Some(waker) = waker {
                    socket.register_send_waker(&waker);
                }
                Ok(false)
            }
        };
        NET_INTERFACE.poll(is_local);
        if is_local {
            NET_INTERFACE.udp_socket_loop(self.handler_loop, poll_f)
        } else {
            NET_INTERFACE.udp_socket_dev(self.handler_dev, poll_f)
        }
    }
}

struct UdpRecvFuture<'a> {
    socket: &'a UdpSocket,
    // buf: ManagedSlice<'a, u8>,
    buf_start: usize,
    buf_len: usize,
    flags: RecvFromFlags,
    for_loop: bool,
}

impl<'a> UdpRecvFuture<'a> {
    fn new(
        socket: &'a UdpSocket,
        buf_start: usize,
        buf_len: usize,
        flags: RecvFromFlags,
        for_loop: bool,
    ) -> Self {
        Self {
            socket,
            buf_start,
            buf_len,
            flags,
            for_loop,
        }
    }
}

impl<'a> Future for UdpRecvFuture<'a> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let _sum_guard = SumGuard::new();
        NET_INTERFACE.poll_all();
        if self.for_loop {
            NET_INTERFACE.udp_socket_loop(self.socket.handler_loop, |socket| {
                let this = self.get_mut();
                if !socket.can_recv() {
                    log::info!("[UdpRecvFuture::poll] cannot recv yet");
                    if this.flags.contains(RecvFromFlags::MSG_DONTWAIT) {
                        log::info!("[UdpRecvFuture::poll] already set nonblock");
                        return Poll::Ready(Err(SyscallErr::EAGAIN));
                    }
                    socket.register_recv_waker(cx.waker());
                    return Poll::Pending;
                }
                log::info!("[UdpRecvFuture::poll] start to recv...");
                Poll::Ready({
                    let (ret, remote) = if this.flags.bits() & RecvFromFlags::MSG_PEEK.bits() > 0 {
                        info!("[UdpRecvFuture::poll] get flags MSG_PEEK");
                        let (ret, meta) = socket
                            .peek_slice(unsafe {
                                &mut core::slice::from_raw_parts_mut(
                                    this.buf_start as *mut u8,
                                    this.buf_len,
                                )
                            })
                            .ok()
                            .ok_or(SyscallErr::ENOTCONN)?;
                        let endpoint = meta.endpoint;
                        (ret, endpoint)
                    } else {
                        let (ret, meta) = socket
                            .recv_slice(unsafe {
                                &mut core::slice::from_raw_parts_mut(
                                    this.buf_start as *mut u8,
                                    this.buf_len,
                                )
                            })
                            .ok()
                            .ok_or(SyscallErr::ENOTCONN)?;
                        let endpoint = meta.endpoint;
                        (ret, endpoint)
                    };
                    info!(
                        "[UdpRecvFuture::poll] {:?} <- {:?}",
                        socket.endpoint(),
                        remote
                    );
                    this.socket.inner.lock().remote_endpoint = Some(remote);
                    log::debug!("[UdpRecvFuture::poll] recv {} bytes", ret);
                    Ok(ret)
                })
            })
        } else {
            NET_INTERFACE.udp_socket_dev(self.socket.handler_dev, |socket| {
                let this = self.get_mut();
                if !socket.can_recv() {
                    log::info!("[UdpRecvFuture::poll] cannot recv yet");
                    if this.flags.contains(RecvFromFlags::MSG_DONTWAIT) {
                        log::info!("[UdpRecvFuture::poll] already set nonblock");
                        return Poll::Ready(Err(SyscallErr::EAGAIN));
                    }
                    socket.register_recv_waker(cx.waker());
                    return Poll::Pending;
                }
                log::info!("[UdpRecvFuture::poll] start to recv...");
                Poll::Ready({
                    let (ret, remote) = if this.flags.bits() & RecvFromFlags::MSG_PEEK.bits() > 0 {
                        info!("[UdpRecvFuture::poll] get flags MSG_PEEK");
                        let (ret, meta) = socket
                            .peek_slice(unsafe {
                                &mut core::slice::from_raw_parts_mut(
                                    this.buf_start as *mut u8,
                                    this.buf_len,
                                )
                            })
                            .ok()
                            .ok_or(SyscallErr::ENOTCONN)?;
                        let endpoint = meta.endpoint;
                        (ret, endpoint)
                    } else {
                        let (ret, meta) = socket
                            .recv_slice(unsafe {
                                &mut core::slice::from_raw_parts_mut(
                                    this.buf_start as *mut u8,
                                    this.buf_len,
                                )
                            })
                            .ok()
                            .ok_or(SyscallErr::ENOTCONN)?;
                        let endpoint = meta.endpoint;
                        (ret, endpoint)
                    };
                    info!(
                        "[UdpRecvFuture::poll] {:?} <- {:?}",
                        socket.endpoint(),
                        remote
                    );
                    this.socket.inner.lock().remote_endpoint = Some(remote);
                    log::debug!("[UdpRecvFuture::poll] recv {} bytes", ret);
                    Ok(ret)
                })
            })
        }
    }
}

#[allow(unused)]
struct UdpSendFuture<'a> {
    socket: &'a UdpSocket,
    buf: &'a [u8],
    flags: RecvFromFlags,
}

impl<'a> UdpSendFuture<'a> {
    fn new(socket: &'a UdpSocket, buf: &'a [u8], flags: RecvFromFlags) -> Self {
        Self { socket, buf, flags }
    }
}

impl<'a> Future for UdpSendFuture<'a> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let _sum_guard = SumGuard::new();
        let is_local = address::is_local(self.socket.remote_endpoint().unwrap());
        NET_INTERFACE.poll(is_local);
        let ret = if is_local {
            NET_INTERFACE.udp_socket_loop(self.socket.handler_loop, |socket| {
                if !socket.can_send() {
                    log::info!("[UdpSendFuture::poll] cannot send yet");
                    if self.flags.contains(RecvFromFlags::MSG_DONTWAIT) {
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
            })
        } else {
            NET_INTERFACE.udp_socket_dev(self.socket.handler_dev, |socket| {
                if !socket.can_send() {
                    log::info!("[UdpSendFuture::poll] cannot send yet");
                    if self.flags.contains(RecvFromFlags::MSG_DONTWAIT) {
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
            })
        };
        NET_INTERFACE.poll(is_local);
        ret
    }
}
