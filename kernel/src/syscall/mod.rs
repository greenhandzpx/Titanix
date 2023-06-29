//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.
const SYSCALL_GETCWD: usize = 17;
const SYSCALL_DUP: usize = 23;
const SYSCALL_DUP3: usize = 24;
const SYSCALL_FCNTL: usize = 25;
const SYSCALL_IOCTL: usize = 29;
const SYSCALL_UNLINK: usize = 35;
const SYSCALL_MKDIR: usize = 34;
const SYSCALL_UMOUNT: usize = 39;
const SYSCALL_MOUNT: usize = 40;
const SYSCALL_STATFS: usize = 43;
const SYSCALL_FACCESSAT: usize = 48;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_GETDENTS: usize = 61;
const SYSCALL_LSEEK: usize = 62;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_READV: usize = 65;
const SYSCALL_WRITEV: usize = 66;
const SYSCALL_SENDFILE: usize = 71;
const SYSCALL_PSELECT6: usize = 72;
const SYSCALL_PPOLL: usize = 73;
const SYSCALL_READLINKAT: usize = 78;
const SYSCALL_NEWFSTATAT: usize = 79;
const SYSCALL_FSTAT: usize = 80;
const SYSCALL_UTIMENSAT: usize = 88;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_EXIT_GROUP: usize = 94;
const SYSCALL_SET_TID_ADDRESS: usize = 96;
const SYSCALL_FUTEX: usize = 98;
const SYSCALL_NANOSLEEP: usize = 101;
const SYSCALL_SETTIMER: usize = 103;
const SYSCALL_CLOCK_SETTIME: usize = 112;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_SYSLOG: usize = 116;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_TGKILL: usize = 131;
const SYSCALL_RT_SIGACTION: usize = 134;
const SYSCALL_RT_SIGPROCMASK: usize = 135;
const SYSCALL_RT_SIGTIMEDWAIT: usize = 137;
const SYSCALL_RT_SIGRETURN: usize = 139;
const SYSCALL_TIMES: usize = 153;
const SYSCALL_SETPGID: usize = 154;
const SYSCALL_GETPGID: usize = 155;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_GETRUSAGE: usize = 165;
const SYSCALL_UMASK: usize = 166;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_GETUID: usize = 174;
const SYSCALL_GETEUID: usize = 175;
const SYSCALL_GETTID: usize = 178;
const SYSCALL_SYSINFO: usize = 179;
const SYSCALL_BRK: usize = 214;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXECVE: usize = 221;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_MPROTECT: usize = 226;
const SYSCALL_WAIT4: usize = 260;
const SYSCALL_PRLIMIT64: usize = 261;
const SYSCALL_REMANEAT2: usize = 276;

const SEEK_SET: u8 = 0;
const SEEK_CUR: u8 = 1;
const SEEK_END: u8 = 2;

mod dev;
mod fs;
mod mm;
mod process;
mod resource;
mod signal;
mod sync;
mod time;

use core::arch::asm;

use dev::*;
use fs::*;
use log::{error, info, trace};
use mm::*;
use process::*;
use signal::*;
pub use sync::futex_wake;
use sync::*;
use time::*;

use crate::{
    fs::posix::{Statfs, Sysinfo},
    mm::MapPermission,
    process::resource::RLimit,
    processor::current_trap_cx,
    signal::{SigAction, SigSet},
    syscall::resource::sys_prlimit64,
    timer::posix::{ITimerval, TimeSpec, TimeVal, Tms},
    utils::error::SyscallRet,
};

/// handle syscall exception with `syscall_id` and other arguments
/// return whether the process should exit or not
pub async fn syscall(syscall_id: usize, args: [usize; 6]) -> SyscallRet {
    trace!(
        "syscall id: {}, sepc {:#x}",
        syscall_id,
        current_trap_cx().sepc
    );
    match syscall_id {
        SYSCALL_GETCWD => sys_getcwd(args[0], args[1]),
        SYSCALL_DUP => sys_dup(args[0]),
        SYSCALL_DUP3 => sys_dup3(args[0], args[1], args[2] as u32),
        SYSCALL_FCNTL => sys_fcntl(args[0], args[1] as i32, args[2] as usize),
        SYSCALL_IOCTL => sys_ioctl(args[0], args[1] as isize, args[2]),
        SYSCALL_UNLINK => sys_unlinkat(args[0] as isize, args[1] as *const u8, args[2] as u32),
        SYSCALL_MKDIR => sys_mkdirat(args[0] as isize, args[1] as *const u8, args[2]),
        SYSCALL_UMOUNT => sys_umount(args[0] as *const u8, args[1] as u32),
        SYSCALL_MOUNT => sys_mount(
            args[0] as *const u8,
            args[1] as *const u8,
            args[2] as *const u8,
            args[3] as u32,
            args[4] as *const u8,
        ),
        SYSCALL_STATFS => sys_statfs(args[0] as *const u8, args[1] as *mut Statfs),
        SYSCALL_FACCESSAT => sys_faccessat(
            args[0] as isize,
            args[1] as *const u8,
            args[2] as u32,
            args[3] as u32,
        ),
        SYSCALL_CHDIR => sys_chdir(args[0] as *const u8),
        SYSCALL_OPEN => sys_openat(
            args[0] as isize,
            args[1] as *const u8,
            args[2] as u32,
            args[3] as u32,
        ),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_PIPE => sys_pipe(args[0] as *mut i32),
        SYSCALL_GETDENTS => sys_getdents(args[0], args[1], args[2]),
        SYSCALL_LSEEK => sys_lseek(args[0], args[1] as isize, args[2] as u8),
        SYSCALL_READ => sys_read(args[0], args[1], args[2]).await,
        SYSCALL_WRITE => sys_write(args[0], args[1], args[2]).await,
        SYSCALL_READV => sys_readv(args[0], args[1], args[2]).await,
        SYSCALL_WRITEV => sys_writev(args[0], args[1], args[2]).await,
        SYSCALL_SENDFILE => {
            sys_sendfile(
                args[0] as isize,
                args[1] as isize,
                args[2],
                args[3] as usize,
            )
            .await
        }
        SYSCALL_PSELECT6 => {
            sys_pselect6(args[0] as i32, args[1], args[2], args[3], args[4], args[5]).await
        }
        SYSCALL_PPOLL => sys_ppoll(args[0], args[1], args[2], args[3]).await,
        SYSCALL_READLINKAT => sys_readlinkat(args[0], args[1], args[2], args[3]),
        SYSCALL_NEWFSTATAT => sys_newfstatat(
            args[0] as isize,
            args[1] as *const u8,
            args[2],
            args[3] as u32,
        ),
        SYSCALL_FSTAT => sys_fstat(args[0], args[1]),
        SYSCALL_UTIMENSAT => sys_utimensat(
            args[0] as isize,
            args[1] as *const u8,
            args[2] as *const TimeSpec,
            args[3] as u32,
        ),
        SYSCALL_EXIT => sys_exit(args[0] as i8),
        SYSCALL_EXIT_GROUP => sys_exit_group(args[0] as i8),
        SYSCALL_SET_TID_ADDRESS => sys_set_tid_address(args[0]),
        SYSCALL_FUTEX => sys_futex(args[0], args[1], args[2]).await,
        SYSCALL_NANOSLEEP => sys_nanosleep(args[0]).await,
        SYSCALL_SETTIMER => sys_settimer(
            args[0] as i32,
            args[1] as *const ITimerval,
            args[2] as *mut ITimerval,
        ),
        SYSCALL_CLOCK_SETTIME => sys_clock_settime(args[0], args[1] as *const TimeSpec),
        SYSCALL_CLOCK_GETTIME => sys_clock_gettime(args[0], args[1] as *mut TimeSpec),
        SYSCALL_SYSLOG => sys_syslog(args[0] as u32, args[1] as *mut u8, args[2] as u32),
        SYSCALL_YIELD => sys_yield().await,
        SYSCALL_KILL => sys_kill(args[0] as isize, args[1] as i32),
        SYSCALL_RT_SIGACTION => sys_rt_sigaction(
            args[0] as i32,
            args[1] as *const SigAction,
            args[2] as *mut SigAction,
        ),
        SYSCALL_RT_SIGPROCMASK => sys_rt_sigprocmask(
            args[0] as i32,
            args[1] as *const u32,
            args[2] as *mut SigSet,
        ),
        SYSCALL_RT_SIGTIMEDWAIT => sys_rt_sigtimedwait(
            args[0] as *const u32,
            args[1] as *const u8,
            args[2] as *const u8,
        ),
        SYSCALL_RT_SIGRETURN => sys_rt_sigreturn(),
        SYSCALL_TIMES => sys_times(args[0] as *mut Tms),
        SYSCALL_SETPGID => sys_setpgid(args[0], args[1]),
        SYSCALL_GETPGID => sys_getpgid(args[0]),
        SYSCALL_UNAME => sys_uname(args[0]),
        SYSCALL_GETRUSAGE => sys_getrusage(args[0] as i32, args[1]),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_GETPPID => sys_getppid(),
        SYSCALL_GETUID => sys_getuid(),
        SYSCALL_GETEUID => sys_geteuid(),
        SYSCALL_GETTID => sys_gettid(),
        SYSCALL_SYSINFO => sys_sysinfo(args[0]),
        SYSCALL_TGKILL => sys_tgkill(args[0] as usize, args[1] as usize, args[2] as i32),
        SYSCALL_BRK => sys_brk(args[0]),
        SYSCALL_MUNMAP => sys_munmap(args[0] as usize, args[1] as usize),
        SYSCALL_CLONE => sys_clone(
            args[0],
            args[1] as *const u8,
            args[2] as *const u8,
            args[3] as *const u8,
            args[4] as *const u8,
        ),
        SYSCALL_EXECVE => sys_execve(
            args[0] as *const u8,
            args[1] as *const usize,
            args[2] as *const usize,
        ),
        SYSCALL_MMAP => sys_mmap(
            args[0],
            args[1],
            args[2] as i32,
            args[3] as i32,
            args[4],
            args[5],
        ),
        SYSCALL_MPROTECT => sys_mprotect(args[0], args[1], args[2] as i32),
        SYSCALL_WAIT4 => sys_wait4(args[0] as isize, args[1], args[2] as i32).await,
        SYSCALL_PRLIMIT64 => sys_prlimit64(
            args[0],
            args[1] as u32,
            args[2] as *const RLimit,
            args[3] as *mut RLimit,
        ),
        SYSCALL_REMANEAT2 => sys_renameat2(
            args[0] as isize,
            args[1] as *const u8,
            args[2] as isize,
            args[3] as *const u8,
            args[4] as u32,
        ),
        _ => {
            // panic!("Unsupported syscall_id: {}", syscall_id);
            error!("Unsupported syscall_id: {}", syscall_id);
            Ok(0)
        }
    }
}

/// Used for sig action.
/// Note that this function is called in user mode only
pub fn user_sigreturn() {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") 0 as isize => ret,
            in("x11") 0,
            in("x12") 0,
            in("x17") SYSCALL_RT_SIGRETURN
        );
    }
}

bitflags! {
    /// Mmap permissions
    pub struct MmapProt: u32 {
        /// None
        const PROT_NONE = 0;
        /// Readable
        const PROT_READ = 1 << 0;
        /// Writable
        const PROT_WRITE = 1 << 1;
        /// Executable
        const PROT_EXEC = 1 << 2;
    }
}

impl From<MmapProt> for MapPermission {
    fn from(prot: MmapProt) -> Self {
        let mut map_permission = MapPermission::from_bits(0).unwrap();
        if prot.contains(MmapProt::PROT_READ) {
            map_permission |= MapPermission::R;
        }
        if prot.contains(MmapProt::PROT_WRITE) {
            map_permission |= MapPermission::W;
        }
        if prot.contains(MmapProt::PROT_EXEC) {
            map_permission |= MapPermission::X;
        }
        map_permission
    }
}

bitflags! {
    /// Mmap flags
    pub struct MmapFlags: u32 {
        /// Shared
        const MAP_SHARED = 1;
        /// Private
        const MAP_PRIVATE = 1 << 1;
        /// Fixed
        const MAP_FIXED = 1 << 4;
        /// Anonymous
        const MAP_ANONYMOUS = 1 << 5;
    }
}

/// Futex Operations
pub enum FutexOperations {
    /// Wait
    FutexWait = 1,
    /// Wake up
    FutexWake = 2,
}

/// Poll Fd
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PollFd {
    /// Fd
    pub fd: i32,
    /// Requested events
    pub events: i16,
    /// Returned events
    pub revents: i16,
}

bitflags! {
    /// Poll events
    pub struct PollEvents: i16 {
        /// There is data to read
        const POLLIN = 1 << 0;
        /// Execption about fd
        const POLLPRI = 1 << 1;
        /// There is data to write
        const POLLOUT = 1 << 2;
        /// Error condition
        const POLLERR = 1 << 3;
        /// Hang up
        const POLLHUP = 1 << 4;
        /// Invalid request: fd not open
        const POLLNVAL = 1 << 5;
    }
}
