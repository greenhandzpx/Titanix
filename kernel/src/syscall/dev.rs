use log::debug;

use crate::{
    fs::InodeMode,
    processor::current_process,
    utils::error::{SyscallErr, SyscallRet},
};

pub fn sys_ioctl(fd: usize, request: isize, arg: usize) -> SyscallRet {
    debug!("fd: {}, request: {}, arg:{}", fd, request, arg);
    match fd {
        0 | 1 | 2 => Ok(0),
        _ => {
            let file = current_process()
                .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
                .ok_or(SyscallErr::EBADF)?;
            let inner = file.metadata().inner.lock();
            if inner.inode.as_ref().unwrap().metadata().mode != InodeMode::FileCHR {
                debug!("[sys_ioctl] not a character device");
                return Err(SyscallErr::ENOTTY);
            }
            debug!("[sys_ioctl] unsupported fd resolve");
            Ok(0)
        }
    }
}
