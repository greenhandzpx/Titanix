use log::debug;

use crate::{
    fs::{resolve_path, InodeMode, OpenFlags},
    mm::user_check::UserCheck,
    processor::{current_process, SumGuard},
    stack_trace,
    utils::error::{SyscallErr, SyscallRet},
};

pub fn sys_ioctl(fd: usize, request: usize, arg: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    log::warn!("[sys_ioctl] fd: {}, request: {}, arg:{}", fd, request, arg);
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;
    if file.metadata().inner.lock().mode != InodeMode::FileCHR {
        debug!("[sys_ioctl] not a character device");
        return Err(SyscallErr::ENOTTY);
    }
    file.ioctl(request, arg)
}

pub async fn sys_getrandom(buf: usize, buflen: usize, _flags: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(buf as *mut u8, buflen)?;
    let inode = resolve_path(0, "/dev/urandom", OpenFlags::RDONLY)?;
    let file = inode.open(inode.clone(), OpenFlags::RDONLY)?;
    let buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, buflen) };
    let ret = file.read(buf).await?;
    log::info!("[sys_read] read {} len", ret);
    Ok(ret)
}
