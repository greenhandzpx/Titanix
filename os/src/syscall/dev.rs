use crate::utils::error::SyscallRet;

pub fn sys_ioctl(fd: usize, request: isize, arg: usize) -> SyscallRet {
    if arg == 0 {
        // TODO: doesn't have arg
    } else {
        // TODO: has arg
    }
    Ok(0)
}
