use log::debug;

use crate::{
    mm::user_check::UserCheck,
    net::{Socket, TCP_MSS},
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

pub fn sys_socket(domain: u32, socket_type: u32, protocol: u32) -> SyscallRet {
    stack_trace!();
    log::info!(
        "[sys_socket] domain: {}, type: {}, protocol: {}",
        domain,
        socket_type,
        protocol
    );
    let sockfd = Socket::new(domain, socket_type)?;
    log::info!("[sys_socket] new sockfd: {}", sockfd);
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
    socket.bind(addr_buf)
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
    socket.accept(addr, addrlen).await
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

pub async fn sys_sendto(
    sockfd: u32,
    buf: usize,
    len: usize,
    _flags: u32,
    _dest_addr: usize,
    _addrlen: u32,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::EBADF)?;
    UserCheck::new().check_readable_slice(buf as *const u8, len)?;
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    let len = socket.write(buf).await?;
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
    let len = socket_file.read(buf).await?;
    if src_addr != 0 {
        let socket = current_process()
            .inner_handler(move |proc| proc.socket_table.get_ref(sockfd as usize).cloned())
            .ok_or(SyscallErr::ENOTSOCK)?;
        socket.peer_addr(src_addr, addrlen)?;
    }
    Ok(len)
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
            debug!("[sys_getsockopt] level: {}, optname: {}", level, optname);
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
    match (level, optname) {
        (SOL_SOCKET, SO_SNDBUF | SO_RCVBUF) => {
            UserCheck::new().check_readable_slice(optval_ptr as *mut u8, optlen as usize)?;
            let size = unsafe { *(optval_ptr as *mut u32) };
            let socket = current_process()
                .inner_handler(move |proc| proc.socket_table.get_ref(sockfd as usize).cloned())
                .ok_or(SyscallErr::ENOTSOCK)?;
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
        _ => {
            debug!("[sys_setsockopt] level: {}, optname: {}", level, optname);
        }
    }
    Ok(0)
}

pub fn sys_socketpair(domain: u32, socket_type: u32, protocol: u32, sv: usize) -> SyscallRet {
    stack_trace!();
    debug!(
        "[sys_socketpair] domain {}, type {}, protocol {}, sv {}",
        domain, socket_type, protocol, sv
    );
    let len = 2 * core::mem::size_of::<u32>();
    UserCheck::new().check_writable_slice(sv as *mut u8, len)?;
    let _sum_guard = SumGuard::new();
    let sv = unsafe { core::slice::from_raw_parts_mut(sv as *mut u32, len) };
    sv[0] = Socket::new(domain, socket_type)? as u32;
    sv[1] = Socket::new(domain, socket_type)? as u32;
    Ok(0)
}
