use core::arch::asm;

use crate::{FutexOperations, TimeVal};

const SYSCALL_GETCWD: usize = 17;
const SYSCALL_DUP: usize = 23;
const SYSCALL_DUP3: usize = 24;
const SYSCALL_MOUNT: usize = 40;
const SYSCALL_OPENAT: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_EXIT_GROUP: usize = 94;
const SYSCALL_FUTEX: usize = 98;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_SIGACTION: usize = 134;
const SYSCALL_SIGRETURN: usize = 139;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXECVE: usize = 221;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_WAITPID: usize = 260;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

fn syscall6(id: usize, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x13") args[3],
            in("x14") args[4],
            in("x15") args[5],
            in("x17") id
        );
    }
    ret
}

pub fn sys_mount(
    dev_name: usize,
    target_path: usize,
    ftype: usize,
    flags: u32,
    data: usize,
) -> isize {
    syscall6(
        SYSCALL_MOUNT,
        [dev_name, target_path, ftype, flags as usize, data, 0],
    )
}

pub fn sys_getcwd(path: usize, len: usize) -> isize {
    syscall(SYSCALL_GETCWD, [path, len, 0])
}

pub fn sys_uname(buf: usize) -> isize {
    syscall(SYSCALL_UNAME, [buf, 0, 0])
}

pub fn sys_dup(oldfd: usize) -> isize {
    syscall(SYSCALL_DUP, [oldfd, 0, 0])
}

pub fn sys_dup3(oldfd: usize, newfd: usize, flags: u32) -> isize {
    syscall(SYSCALL_DUP3, [oldfd, newfd, flags as usize])
}

pub fn sys_openat(dirfd: usize, path: &str, flags: u32, mode: u32) -> isize {
    syscall6(
        SYSCALL_OPENAT,
        [
            dirfd,
            path.as_ptr() as usize,
            flags as usize,
            mode as usize,
            0,
            0,
        ],
    )
}

pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> ! {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0]);
    panic!("sys_exit never returns!");
}

pub fn sys_exit_group(exit_code: i32) -> ! {
    syscall(SYSCALL_EXIT_GROUP, [exit_code as usize, 0, 0]);
    panic!("sys_exit never returns!");
}

pub fn sys_futex(uaddr: *const usize, futex_op: FutexOperations, val: usize) -> isize {
    syscall(SYSCALL_FUTEX, [uaddr as usize, futex_op as usize, val])
}
pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time(time_val: *mut TimeVal) -> isize {
    syscall(SYSCALL_GET_TIME, [time_val as usize, 0, 0])
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

pub fn sys_kill(pid: isize, signo: i32) -> isize {
    syscall(SYSCALL_KILL, [pid as usize, signo as usize, 0])
}

pub struct SigAction {
    pub sa_handler: fn(i32),
    pub sa_mask: SigSet,
}

bitflags! {
    pub struct SigSet: usize {
        const SIGHUP = 1 << 1;
        const SIGINT = 1 << 2;
        const SIGILL = 1 << 4;
        const SIGABRT = 1 << 6;
        const SIGBUS = 1 << 7;
        const SIGKILL = 1 << 9;
        const SIGSEGV = 1 << 11;
        const SIGALARM = 1 << 14;
        const SIGTERM = 1 << 15;
        const SIGCHILD = 1 << 17;
        const SIGSTOP = 1 << 19;
    }
}

pub enum Signal {
    SIGHUP = 1,
    SIGINT = 2,
    SIGILL = 4,
    SIGABRT = 6,
    SIGBUS = 7,
    SIGKILL = 9,
    SIGSEGV = 11,
    SIGALARM = 14,
    SIGTERM = 15,
    SIGCHILD = 17,
    SIGSTOP = 19,
}

// fn sigaction_wrapper(f: fn(i32), sig: i32) {
//     f(sig);
//     sigreturn();
// }

pub fn sys_sigaction(sig_no: Signal, act: &SigAction, old_act: &mut SigAction) -> isize {
    syscall(
        SYSCALL_SIGACTION,
        [
            sig_no as usize,
            act as *const _ as usize,
            old_act as *const _ as usize,
        ],
    )
}

pub fn sys_sigreturn() -> isize {
    syscall(SYSCALL_SIGRETURN, [0, 0, 0])
}

pub fn sys_fork() -> isize {
    syscall6(SYSCALL_CLONE, [0, 0, 0, 0, 0, 0])
}

pub fn sys_clone(f: fn(*const u8) -> isize, stack: *const u8, flags: i32, arg: *const u8) -> isize {
    syscall6(
        SYSCALL_CLONE,
        [
            f as *const u8 as usize,
            stack as usize,
            flags as usize,
            arg as usize,
            0,
            0,
        ],
    )
}

pub fn sys_execve(path: &str, args: &[*const u8], envp: &[*const u8]) -> isize {
    syscall(
        SYSCALL_EXECVE,
        [
            path.as_ptr() as usize,
            args.as_ptr() as usize,
            envp.as_ptr() as usize,
        ],
    )
}

pub fn sys_mmap(
    addr: *const u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: usize,
    offset: usize,
) -> isize {
    syscall6(
        SYSCALL_MMAP,
        [
            addr as usize,
            length,
            prot as usize,
            flags as usize,
            fd,
            offset,
        ],
    )
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
}
pub fn sys_pipe(pipe: &mut [i32]) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, [fd, 0, 0])
}
