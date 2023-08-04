use log::info;
use smoltcp::wire::IpListenEndpoint;

use crate::{
    mm::user_check::UserCheck,
    net::{address::SocketAddrv4, make_unix_socket_pair, Socket, TCP_MSS},
    processor::{current_process, SumGuard},
    stack_trace,
    utils::error::{SyscallErr, SyscallRet},
};

/// level
const SOL_SOCKET: u32 = 1;
const SOL_TCP: u32 = 6;

/// option name
const TCP_MAXSEG: u32 = 2;
const SO_SNDBUF: u32 = 7;
const SO_RCVBUF: u32 = 8;
const TCP_NODELAY: u32 = 1;

pub fn sys_socket(domain: u32, socket_type: u32, protocol: u32) -> SyscallRet {
    stack_trace!();
    info!(
        "[sys_socket] domain: {}, type: {}, protocol: {}",
        domain, socket_type, protocol
    );
    let sockfd = Socket::new(domain, socket_type)?;
    info!("[sys_socket] new sockfd: {}", sockfd);
    Ok(sockfd)
}

pub fn sys_bind(sockfd: u32, addr: usize, addrlen: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_readable_slice(addr as *const u8, addrlen as usize)?;
    let addr_buf = unsafe { core::slice::from_raw_parts(addr as *const u8, addrlen as usize) };
    let socket = current_process()
        .inner_handler(|proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    let endpoint = Socket::endpoint(addr_buf)?;
    match *socket {
        Socket::TcpSocket(ref socket) => socket.bind(endpoint),
        Socket::UdpSocket(ref socket) => current_process().inner_handler(|proc| {
            let res = proc.socket_table.can_bind(endpoint);
            if res.is_none() {
                info!("[sys_bind] not find port exist");
                socket.bind(endpoint)
            } else {
                let (_, sock) = res.unwrap();
                proc.socket_table.insert(sockfd as usize, sock.clone());
                stack_trace!();
                proc.fd_table.take(sockfd as usize);
                proc.fd_table.put(sockfd as usize, sock);
                Ok(0)
            }
        }),
        Socket::UnixSocket(_) => todo!(),
    }
}

pub fn sys_listen(sockfd: u32, _backlog: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket = current_process()
        .inner_handler(|proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    socket.listen()
}

pub async fn sys_accept(sockfd: u32, addr: usize, addrlen: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket = current_process()
        .inner_handler(|proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    socket.accept(sockfd, addr, addrlen).await
}

pub async fn sys_connect(sockfd: u32, addr: usize, addrlen: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_readable_slice(addr as *const u8, addrlen as usize)?;
    let addr_buf = unsafe { core::slice::from_raw_parts(addr as *const u8, addrlen as usize) };
    let socket = current_process()
        .inner_handler(|proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    socket.connect(addr_buf).await
}

pub fn sys_getsockname(sockfd: u32, addr: usize, addrlen: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket = current_process()
        .inner_handler(|proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    socket.addr(addr, addrlen)
}

pub fn sys_getpeername(sockfd: u32, addr: usize, addrlen: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket = current_process()
        .inner_handler(|proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    socket.peer_addr(addr, addrlen)
}

pub async fn sys_sendto(
    sockfd: u32,
    buf: usize,
    len: usize,
    _flags: u32,
    dest_addr: usize,
    addrlen: u32,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket_file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::EBADF)?;
    UserCheck::new().check_readable_slice(buf as *const u8, len)?;
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };

    let socket = current_process()
        .inner_handler(move |proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    log::info!("[sys_sendto] get socket sockfd: {}", sockfd);
    let len = match *socket {
        Socket::TcpSocket(_) => socket_file.write(buf).await?,
        Socket::UdpSocket(ref udp) => {
            info!("[sys_sendto] socket is udp");
            UserCheck::new().check_readable_slice(dest_addr as *const u8, addrlen as usize)?;
            if udp.addr().addr.is_unspecified() || udp.addr().port == 0 {
                let addr = SocketAddrv4::new([0; 16].as_slice());
                let endpoint = IpListenEndpoint::from(addr);
                udp.bind(endpoint)?;
            }
            let dest_addr =
                unsafe { core::slice::from_raw_parts(dest_addr as *const u8, addrlen as usize) };
            socket.connect(dest_addr).await?;
            socket_file.write(buf).await?
        }
        Socket::UnixSocket(_) => {
            info!("[sys_sendto] socket is unix");
            todo!()
            // UserCheck::new().check_readable_slice(dest_addr as *const u8, addrlen as usize)?;
            // let dest_addr =
            //     unsafe { core::slice::from_raw_parts(dest_addr as *const u8, addrlen as usize) };
            // let endpoint = unix.addr(dest_addr);
            // let ret = socket_file.write(buf).await?;
            // UNIX_SOCKET_BUF_MANAGER
            //     .buf_mgr
            //     .lock()
            //     .insert(endpoint, socket_file);
            // ret
        }
    };
    Ok(len)
}

pub async fn sys_recvfrom(
    sockfd: u32,
    buf: usize,
    len: u32,
    _flags: u32,
    src_addr: usize,
    addrlen: usize,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket_file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::EBADF)?;
    UserCheck::new().check_writable_slice(buf as *mut u8, len as usize)?;
    let buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, len as usize) };
    let socket = current_process()
        .inner_handler(move |proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    info!("[sys_recvfrom] get socket sockfd: {}", sockfd);
    match *socket {
        Socket::UnixSocket(_) => {
            todo!("[sys_sendto] socket is unix");
        }
        Socket::TcpSocket(_) => {
            let len = socket_file.read(buf).await?;
            if src_addr != 0 {
                socket.peer_addr(src_addr, addrlen)?;
            }
            Ok(len)
        }
        Socket::UdpSocket(_) => {
            let len = socket_file.read(buf).await?;
            if src_addr != 0 {
                socket.peer_addr(src_addr, addrlen)?;
            }
            Ok(len)
        }
    }
}

pub fn sys_getsockopt(
    sockfd: u32,
    level: u32,
    optname: u32,
    optval_ptr: usize,
    optlen: usize,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    match (level, optname) {
        (SOL_TCP, TCP_MAXSEG) => {
            // return max tcp fregment size (MSS)
            let len = core::mem::size_of::<u32>();
            UserCheck::new().check_writable_slice(optval_ptr as *mut u8, len)?;
            UserCheck::new().check_writable_slice(optlen as *mut u8, len)?;
            unsafe {
                *(optval_ptr as *mut u32) = TCP_MSS;
                *(optlen as *mut u32) = len as u32;
            }
        }
        (SOL_SOCKET, SO_SNDBUF | SO_RCVBUF) => {
            let len = core::mem::size_of::<u32>();
            UserCheck::new().check_writable_slice(optval_ptr as *mut u8, len)?;
            UserCheck::new().check_writable_slice(optlen as *mut u8, len)?;
            let socket = current_process()
                .inner_handler(move |proc| proc.socket_table.get_ref(sockfd as usize).cloned())
                .ok_or(SyscallErr::ENOTSOCK)?;
            match optname {
                SO_SNDBUF => {
                    let size = socket.send_buf_size();
                    unsafe {
                        *(optval_ptr as *mut u32) = size as u32;
                        *(optlen as *mut u32) = 4;
                    }
                }
                SO_RCVBUF => {
                    let size = socket.recv_buf_size();
                    unsafe {
                        *(optval_ptr as *mut u32) = size as u32;
                        *(optlen as *mut u32) = 4;
                    }
                }
                _ => {}
            }
        }
        _ => {
            log::warn!("[sys_getsockopt] level: {}, optname: {}", level, optname);
        }
    }
    Ok(0)
}

pub fn sys_setsockopt(
    sockfd: u32,
    level: u32,
    optname: u32,
    optval_ptr: usize,
    optlen: u32,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket = current_process()
        .inner_handler(move |proc| proc.socket_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::ENOTSOCK)?;
    match (level, optname) {
        (SOL_SOCKET, SO_SNDBUF | SO_RCVBUF) => {
            UserCheck::new().check_readable_slice(optval_ptr as *mut u8, optlen as usize)?;
            let size = unsafe { *(optval_ptr as *mut u32) };
            match optname {
                SO_SNDBUF => {
                    socket.set_send_buf_size(size as usize);
                }
                SO_RCVBUF => {
                    socket.set_recv_buf_size(size as usize);
                }
                _ => {}
            }
        }
        (SOL_TCP, TCP_NODELAY) => {
            // close Nagle’s Algorithm
            UserCheck::new().check_readable_slice(optval_ptr as *const u8, optlen as usize)?;
            let enabled = unsafe { *(optval_ptr as *const u32) };
            log::debug!("[sys_setsockopt] set TCPNODELY: {}", enabled);
            match enabled {
                0 => socket.set_nagle_enabled(true)?,
                _ => socket.set_nagle_enabled(false)?,
            };
        }
        _ => {
            log::warn!("[sys_setsockopt] level: {}, optname: {}", level, optname);
        }
    }
    Ok(0)
}

pub fn sys_shutdown(sockfd: u32, how: u32) -> SyscallRet {
    stack_trace!();
    log::info!("[sys_shutdown] sockfd {}, how {}", sockfd, how);
    // current_process().close_file(sockfd as usize)?;
    current_process().inner_handler(|proc| {
        let socket = proc
            .socket_table
            .get_ref(sockfd as usize)
            .ok_or(SyscallErr::EBADF)?
            .clone();
        socket.shutdown(how)?;

        Ok(0)
    })
}

pub fn sys_socketpair(domain: u32, socket_type: u32, protocol: u32, sv: usize) -> SyscallRet {
    stack_trace!();
    info!(
        "[sys_socketpair] domain {}, type {}, protocol {}, sv {}",
        domain, socket_type, protocol, sv
    );
    let len = 2 * core::mem::size_of::<u32>();
    UserCheck::new().check_writable_slice(sv as *mut u8, len)?;
    let _sum_guard = SumGuard::new();
    let sv = unsafe { core::slice::from_raw_parts_mut(sv as *mut u32, len) };
    let (socket1, socket2) = make_unix_socket_pair();

    let (fd1, fd2) = current_process().inner_handler(move |proc| {
        let fd1 = proc.fd_table.alloc_fd()?;
        proc.fd_table.put(fd1, socket1);
        let fd2 = proc.fd_table.alloc_fd()?;
        proc.fd_table.put(fd2, socket2);
        Ok((fd1, fd2))
    })?;

    sv[0] = fd1 as u32;
    sv[1] = fd2 as u32;
    info!("[sys_socketpair] new sv: {:?}", sv);
    Ok(0)
}
