use log::debug;

use crate::{
    fs::InodeMode,
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
