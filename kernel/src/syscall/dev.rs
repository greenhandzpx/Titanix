use core::ptr;

use log::debug;

use crate::{
    fs::InodeMode,
    mm::user_check::UserCheck,
    processor::{current_process, SumGuard},
    utils::error::{SyscallErr, SyscallRet},
};

const TIOCGPGRP: usize = 0x540F;

pub fn sys_ioctl(fd: usize, request: usize, arg: usize) -> SyscallRet {
    let _sum_guard = SumGuard::new();
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
            if request == TIOCGPGRP {
                debug!("[sys_ioctl] for tcgetpgrp");
                UserCheck::new()
                    .check_writable_slice(arg as *mut u8, core::mem::size_of::<u32>())?;
                let pid = current_process().pgid();
                unsafe {
                    ptr::write(arg as *mut u32, pid as u32);
                }
            }
            Ok(0)
        }
    }
}
