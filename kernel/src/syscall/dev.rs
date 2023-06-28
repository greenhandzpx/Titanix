use log::debug;

use crate::utils::error::SyscallRet;

pub fn sys_ioctl(fd: usize, request: isize, arg: usize) -> SyscallRet {
    debug!("fd: {}, request: {}, arg:{}", fd, request, arg);
    match fd {
        0 | 1 | 2 => Ok(0),
        _ => {
            debug!("[sys_ioctl] unsupported fd");
            Ok(0)
        }
    }
}
