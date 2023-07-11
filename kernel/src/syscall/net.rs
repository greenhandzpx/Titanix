use log::debug;

use crate::{
    fs::socket::{Socket, SocketAddr, SOCKETADDR_SIZE, SOCKETBUF_MANAGER},
    mm::user_check::UserCheck,
    processor::{current_process, SumGuard},
    stack_trace,
    utils::error::{SyscallErr, SyscallRet},
};

/// domain
const AF_INET: u32 = 2;

/// socket type
const SOCK_DGRAM: u32 = 2;

/// protocol
const IPPROTO_UDP: u32 = 17;

pub fn sys_socket(domain: u32, socket_type: u32, protocol: u32) -> SyscallRet {
    debug!(
        "[sys_socket] domain: {}, type: {}, protocol: {}",
        domain, socket_type, protocol
    );
    let socket = Socket::new();
    current_process().inner_handler(move |proc| {
        let fd = proc.fd_table.alloc_fd()?;
        proc.fd_table.put(fd, socket);
        Ok(fd as isize)
    })
}

pub fn sys_bind(sockfd: u32, addr: *const SocketAddr, addrlen: u32) -> SyscallRet {
    Ok(0)
}

pub fn sys_listen(sockfd: u32, backlog: u32) -> SyscallRet {
    Ok(0)
}

pub fn sys_accept(sockfd: u32, addr: *const SocketAddr, addrlen: u32) -> SyscallRet {
    Ok(0)
}

pub fn sys_connect(sockfd: u32, addr: *const SocketAddr, addrlen: u32) -> SyscallRet {
    Ok(0)
}

pub fn sys_getsockname(sockfd: u32, addr: *const SocketAddr, addrlen: u32) -> SyscallRet {
    Ok(0)
}

pub async fn sys_sendto(
    sockfd: u32,
    buf: usize,
    len: usize,
    flags: u32,
    dest_addr: usize,
    addrlen: u32,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let socket = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::EBADF)?;
    UserCheck::new().check_readable_slice(buf as *const u8, len)?;
    UserCheck::new().check_readable_slice(dest_addr as *const u8, SOCKETADDR_SIZE)?;
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    let dest_addr = unsafe { *(dest_addr as *const SocketAddr) };
    let len = socket.write(buf).await?;
    SOCKETBUF_MANAGER
        .socketbuf_mgr
        .lock()
        .insert(dest_addr, socket);
    Ok(len)
}

pub async fn sys_recvfrom(
    sockfd: u32,
    buf: usize,
    len: usize,
    flags: u32,
    src_addr: usize,
    addrlen: u32,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(buf as *mut u8, len)?;
    let buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, len) };
    UserCheck::new().check_readable_slice(src_addr as *const u8, SOCKETADDR_SIZE)?;
    let src_addr = unsafe { *(src_addr as *const SocketAddr) };
    let socket = SOCKETBUF_MANAGER
        .socketbuf_mgr
        .lock()
        .get(&src_addr)
        .unwrap()
        .clone();
    let len = socket.read(buf).await?;
    Ok(len)
}

pub fn sys_setsockopt(
    sockfd: u32,
    level: u32,
    optname: u32,
    optval: usize,
    optlen: u32,
) -> SyscallRet {
    Ok(0)
}