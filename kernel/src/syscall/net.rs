use log::{debug, info};

use crate::{
    fs::{
        inode::InodeDevice,
        socket::{
            SocketAddr, SocketFile, MAX_BUFFER_SIZE, SOCKETADDR_SIZE, SOCKETBUF_MANAGER, TCP_MSS,
        },
    },
    mm::user_check::UserCheck,
    processor::{current_process, SumGuard},
    stack_trace,
    utils::error::{SyscallErr, SyscallRet},
};

/// socket type
const SOCK_DGRAM: u32 = 2;

/// protocol
const IPPROTO_UDP: u32 = 17;

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
    let socket_file = SocketFile::new()?;
    current_process().inner_handler(move |proc| {
        let fd = proc.fd_table.alloc_fd()?;
        proc.fd_table.put(fd, socket_file);
        Ok(fd as isize)
    })
}

pub fn sys_bind(sockfd: u32, addr: *const SocketAddr, addrlen: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_readable_slice(addr as *const u8, addrlen as usize)?;
    let addr = unsafe { *addr };
    let socket_file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::EBADF)?;
    let file_inner = socket_file.metadata().inner.lock();
    let socket = file_inner
        .inode
        .as_ref()
        .unwrap()
        .metadata()
        .device
        .as_ref()
        .unwrap();
    match socket {
        InodeDevice::Socket(socket) => {
            socket.inner.lock().addr = addr;
        }
        _ => {
            return Err(SyscallErr::EBADF);
        }
    }
    Ok(0)
}

pub fn sys_listen(sockfd: u32, backlog: u32) -> SyscallRet {
    stack_trace!();
    Ok(0)
}

pub fn sys_accept(sockfd: u32, addr: *const SocketAddr, addrlen: u32) -> SyscallRet {
    stack_trace!();
    Ok(0)
}

pub fn sys_connect(sockfd: u32, addr: *const SocketAddr, addrlen: u32) -> SyscallRet {
    stack_trace!();
    Ok(0)
}

pub fn sys_getsockname(sockfd: u32, addr: *mut SocketAddr, addrlen: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(addr as *mut u8, SOCKETADDR_SIZE)?;
    UserCheck::new().check_writable_slice(addrlen as *mut u8, core::mem::size_of::<u32>())?;
    let socket_file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(sockfd as usize).cloned())
        .ok_or(SyscallErr::EBADF)?;
    let file_inner = socket_file.metadata().inner.lock();
    let socket = file_inner
        .inode
        .as_ref()
        .unwrap()
        .metadata()
        .device
        .as_ref()
        .unwrap();
    match socket {
        InodeDevice::Socket(socket) => {
            let socket_addr = socket.inner.lock().addr.clone();
            debug!("[sys_getsockname] get addr: {:?}", socket_addr);
            unsafe {
                *addr = socket_addr;
                *(addrlen as *mut u32) = SOCKETADDR_SIZE as u32;
            }
        }
        _ => {
            return Err(SyscallErr::EBADF);
        }
    }
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

pub fn sys_getsockopt(
    sockfd: u32,
    level: u32,
    optname: u32,
    optval_ptr: usize,
    optlen: usize,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    match optname {
        TCP_MAXSEG => {
            // return max tcp fregment size (MSS)
            let len = core::mem::size_of::<u32>();
            UserCheck::new().check_writable_slice(optval_ptr as *mut u8, len)?;
            UserCheck::new().check_writable_slice(optlen as *mut u8, len)?;
            unsafe {
                *(optval_ptr as *mut u32) = TCP_MSS;
                *(optlen as *mut u32) = len as u32;
            }
        }
        SO_SNDBUF | SO_RCVBUF => {
            let socket_file = current_process()
                .inner_handler(|proc| proc.fd_table.get(sockfd as usize))
                .ok_or(SyscallErr::EBADF)?;
            let file_inner = socket_file.metadata().inner.lock();
            let socket = file_inner
                .inode
                .as_ref()
                .unwrap()
                .metadata()
                .device
                .as_ref()
                .unwrap();
            match socket {
                InodeDevice::Socket(socket) => {
                    let size = match optname {
                        SO_RCVBUF => socket.inner.lock().recvbuf_size,
                        SO_SNDBUF => socket.inner.lock().sendbuf_size,
                        _ => MAX_BUFFER_SIZE,
                    };
                    let len = core::mem::size_of::<u32>();
                    UserCheck::new().check_writable_slice(optval_ptr as *mut u8, len)?;
                    UserCheck::new().check_writable_slice(optlen as *mut u8, len)?;
                    unsafe {
                        *(optval_ptr as *mut u32) = size;
                        *(optlen as *mut u32) = len as u32;
                    }
                }
                _ => {
                    info!("[sys_getsockopt] device isn't Socket");
                    return Err(SyscallErr::EBADF);
                }
            }
        }
        _ => {
            debug!("[sys_getsockopt] optname: {}", optname);
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
    match optname {
        SO_SNDBUF | SO_RCVBUF => {
            let socket_file =
                current_process().inner_handler(|proc| proc.fd_table.get(sockfd as usize));
            if socket_file.is_none() {
                info!("[sys_setsockopt] sockfd is bad");
                return Err(SyscallErr::EBADF);
            }
            let socket_file = socket_file.unwrap();
            let file_inner = socket_file.metadata().inner.lock();
            let socket = file_inner
                .inode
                .as_ref()
                .unwrap()
                .metadata()
                .device
                .as_ref()
                .unwrap();
            match socket {
                InodeDevice::Socket(socket) => {
                    let len = core::mem::size_of::<u32>();
                    UserCheck::new().check_readable_slice(optval_ptr as *mut u8, len)?;
                    let size = unsafe { *(optval_ptr as *const u32) };
                    match optname {
                        SO_RCVBUF => {
                            socket.inner.lock().recvbuf_size = size;
                        }
                        SO_SNDBUF => {
                            socket.inner.lock().sendbuf_size = size;
                        }
                        _ => {}
                    }
                }
                _ => {
                    info!("[sys_getsockopt] device isn't Socket");
                    return Err(SyscallErr::EBADF);
                }
            }
        }
        _ => {
            debug!("[sys_getsockopt] optname: {}", optname);
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
    let socket1 = SocketFile::new()?;
    let socket2 = SocketFile::new()?;
    current_process().inner_handler(move |proc| {
        let fd1 = proc.fd_table.alloc_fd()?;
        proc.fd_table.put(fd1, socket1);
        let fd2 = proc.fd_table.alloc_fd()?;
        proc.fd_table.put(fd2, socket2);
        sv[0] = fd1 as u32;
        sv[1] = fd2 as u32;
        Ok(0)
    })
}
