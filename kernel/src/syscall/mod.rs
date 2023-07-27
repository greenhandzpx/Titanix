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
const SYSCALL_FTRUNCATE: usize = 46;
const SYSCALL_FACCESSAT: usize = 48;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_FCHMODAT: usize = 53;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_GETDENTS: usize = 61;
const SYSCALL_LSEEK: usize = 62;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_READV: usize = 65;
const SYSCALL_WRITEV: usize = 66;
const SYSCALL_PREAD64: usize = 67;
const SYSCALL_PWRITE64: usize = 68;
const SYSCALL_SENDFILE: usize = 71;
const SYSCALL_PSELECT6: usize = 72;
const SYSCALL_PPOLL: usize = 73;
const SYSCALL_READLINKAT: usize = 78;
const SYSCALL_NEWFSTATAT: usize = 79;
const SYSCALL_FSTAT: usize = 80;
const SYSCALL_SYNC: usize = 81;
const SYSCALL_FSYNC: usize = 82;
const SYSCALL_UTIMENSAT: usize = 88;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_EXIT_GROUP: usize = 94;
const SYSCALL_SET_TID_ADDRESS: usize = 96;
const SYSCALL_FUTEX: usize = 98;
const SYSCALL_SET_ROBUST_LIST: usize = 99;
const SYSCALL_GET_ROBUST_LIST: usize = 100;
const SYSCALL_NANOSLEEP: usize = 101;
const SYSCALL_SETITIMER: usize = 103;
const SYSCALL_CLOCK_SETTIME: usize = 112;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_CLOCK_GETRES: usize = 114;
const SYSCALL_CLOCK_NANOSLEEP: usize = 115;
const SYSCALL_SYSLOG: usize = 116;
const SYSCALL_SCHED_SETSCHEDULER: usize = 119;
const SYSCALL_SCHED_GETSCHEDULER: usize = 120;
const SYSCALL_SCHED_GETPARAM: usize = 121;
const SYSCALL_SCHED_SETAFFINITY: usize = 122;
const SYSCALL_SCHED_GETAFFINITY: usize = 123;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_TKILL: usize = 130;
const SYSCALL_TGKILL: usize = 131;
const SYSCALL_RT_SIGSUSPEND: usize = 133;
const SYSCALL_RT_SIGACTION: usize = 134;
const SYSCALL_RT_SIGPROCMASK: usize = 135;
const SYSCALL_RT_SIGTIMEDWAIT: usize = 137;
const SYSCALL_RT_SIGRETURN: usize = 139;
const SYSCALL_TIMES: usize = 153;
const SYSCALL_SETPGID: usize = 154;
const SYSCALL_GETPGID: usize = 155;
const SYSCALL_SETSID: usize = 157;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_GETRUSAGE: usize = 165;
const SYSCALL_UMASK: usize = 166;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_GETUID: usize = 174;
const SYSCALL_GETEUID: usize = 175;
const SYSCALL_GETEGID: usize = 177;
const SYSCALL_GETTID: usize = 178;
const SYSCALL_SYSINFO: usize = 179;
const SYSCALL_SHMGET: usize = 194;
const SYSCALL_SHMCTL: usize = 195;
const SYSCALL_SHMAT: usize = 196;
const SYSCALL_SOCKET: usize = 198;
const SYSCALL_SOCKETPAIR: usize = 199;
const SYSCALL_BIND: usize = 200;
const SYSCALL_LISTEN: usize = 201;
const SYSCALL_ACCEPT: usize = 202;
const SYSCALL_CONNECT: usize = 203;
const SYSCALL_GETSOCKNAME: usize = 204;
const SYSCALL_SENDTO: usize = 206;
const SYSCALL_RECVFROM: usize = 207;
const SYSCALL_SETSOCKOPT: usize = 208;
const SYSCALL_GETSOCKOPT: usize = 209;
const SYSCALL_BRK: usize = 214;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXECVE: usize = 221;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_MPROTECT: usize = 226;
const SYSCALL_MSYNC: usize = 227;
const SYSCALL_MADVISE: usize = 233;
const SYSCALL_WAIT4: usize = 260;
const SYSCALL_PRLIMIT64: usize = 261;
const SYSCALL_REMANEAT2: usize = 276;
const SYSCALL_MEMBARRIER: usize = 283;

mod dev;
mod fs;
mod mm;
mod net;
mod process;
mod resource;
mod signal;
mod sync;
mod time;

use dev::*;
use fs::*;
use log::error;
use mm::*;
use net::*;
pub use process::CloneFlags;
use process::*;
use resource::*;
use signal::*;
use sync::*;
use time::*;

use crate::{
    fs::ffi::Statfs,
    mm::MapPermission,
    process::resource::RLimit,
    signal::{SigAction, SigSet},
    strace,
    timer::ffi::{ITimerval, TimeSpec, TimeVal, Tms},
    utils::error::SyscallRet,
};

macro_rules! sys_handler {
    ($handler: ident, $args: tt) => {
        {
            strace!(
                "{}, args: {:?}, sepc: {:#x}",
                stringify!($handler),
                $args,
                crate::processor::current_trap_cx().sepc
            );
            $handler$args
        }
    };
    ($handler: ident, $args: tt, $await: tt) => {
        {
            strace!(
                "{}, args: {:?}, sepc: {:#x}",
                stringify!($handler),
                $args,
                crate::processor::current_trap_cx().sepc
            );
            $handler$args.$await
        }
    };
}

/// Handle syscall exception with `syscall_id` and other arguments.
pub async fn syscall(syscall_id: usize, args: [usize; 6]) -> SyscallRet {
    // strace!(
    //     "syscall: {}, args: {:?}, sepc: {:#x}",
    //     SYSCALL_NAMES[syscall_id],
    //     args,
    //     crate::processor::current_trap_cx().sepc
    // );
    match syscall_id {
        SYSCALL_GETCWD => sys_handler!(sys_getcwd, (args[0], args[1])),
        SYSCALL_DUP => sys_handler!(sys_dup, (args[0])),
        SYSCALL_DUP3 => sys_handler!(sys_dup3, (args[0], args[1], args[2] as u32)),
        SYSCALL_FCNTL => sys_handler!(sys_fcntl, (args[0], args[1] as i32, args[2] as usize)),
        SYSCALL_IOCTL => sys_handler!(sys_ioctl, (args[0], args[1], args[2])),
        SYSCALL_UNLINK => sys_handler!(
            sys_unlinkat,
            (args[0] as isize, args[1] as *const u8, args[2] as u32)
        ),
        SYSCALL_MKDIR => sys_handler!(
            sys_mkdirat,
            (args[0] as isize, args[1] as *const u8, args[2])
        ),
        SYSCALL_UMOUNT => sys_handler!(sys_umount, (args[0], args[1] as u32), await),
        SYSCALL_MOUNT => sys_handler!(
            sys_mount,
            (
                args[0] as *const u8,
                args[1] as *const u8,
                args[2] as *const u8,
                args[3] as u32,
                args[4] as *const u8,
            )
        ),
        SYSCALL_STATFS => sys_handler!(sys_statfs, (args[0] as *const u8, args[1] as *mut Statfs)),
        SYSCALL_FTRUNCATE => sys_handler!(sys_ftruncate, (args[0], args[1]), await),
        SYSCALL_FACCESSAT => sys_handler!(
            sys_faccessat,
            (
                args[0] as isize,
                args[1] as *const u8,
                args[2] as u32,
                args[3] as u32,
            )
        ),
        SYSCALL_CHDIR => sys_handler!(sys_chdir, (args[0] as *const u8)),
        SYSCALL_FCHMODAT => sys_handler!(sys_fchmodat, ()),
        SYSCALL_OPEN => sys_handler!(
            sys_openat,
            (
                args[0] as isize,
                args[1] as *const u8,
                args[2] as u32,
                args[3] as u32,
            )
        ),
        SYSCALL_CLOSE => sys_handler!(sys_close, (args[0])),
        SYSCALL_PIPE => sys_handler!(sys_pipe, (args[0] as *mut i32)),
        SYSCALL_GETDENTS => sys_handler!(sys_getdents, (args[0], args[1], args[2])),
        SYSCALL_LSEEK => sys_handler!(sys_lseek, (args[0], args[1] as isize, args[2] as u8)),
        SYSCALL_READ => sys_handler!(sys_read, (args[0], args[1], args[2]), await),
        SYSCALL_WRITE => sys_handler!(sys_write, (args[0], args[1], args[2]), await),
        SYSCALL_READV => sys_handler!(sys_readv, (args[0], args[1], args[2]), await),
        SYSCALL_WRITEV => sys_handler!(sys_writev, (args[0], args[1], args[2]), await),
        SYSCALL_PREAD64 => sys_handler!(sys_pread64, (args[0], args[1], args[2], args[3]), await),
        SYSCALL_PWRITE64 => sys_handler!(sys_pwrite64, (args[0], args[1], args[2], args[3]), await),
        SYSCALL_SENDFILE => sys_handler!(
            sys_sendfile, (
                args[0] as isize  ,
                args[1] as isize  ,
                args[2],
                args[3] as usize,
            )
            , await
        ),
        SYSCALL_PSELECT6 => sys_handler!(
            sys_pselect6, (args[0] as i32, args[1], args[2], args[3], args[4], args[5]), await
        ),
        SYSCALL_PPOLL => sys_handler!(sys_ppoll, (args[0], args[1], args[2], args[3]), await),
        SYSCALL_READLINKAT => sys_handler!(sys_readlinkat, (args[0], args[1], args[2], args[3])),
        SYSCALL_NEWFSTATAT => sys_handler!(
            sys_newfstatat,
            (
                args[0] as isize,
                args[1] as *const u8,
                args[2],
                args[3] as u32,
            )
        ),
        SYSCALL_FSTAT => sys_handler!(sys_fstat, (args[0], args[1])),
        SYSCALL_SYNC => sys_handler!(sys_sync, (), await),
        SYSCALL_FSYNC => sys_handler!(sys_fsync, (args[0]), await),
        SYSCALL_UTIMENSAT => sys_handler!(
            sys_utimensat,
            (
                args[0] as isize,
                args[1] as *const u8,
                args[2] as *const TimeSpec,
                args[3] as u32,
            )
        ),
        SYSCALL_EXIT => sys_handler!(sys_exit, (args[0] as i8)),
        SYSCALL_EXIT_GROUP => sys_handler!(sys_exit_group, (args[0] as i8)),
        SYSCALL_SET_TID_ADDRESS => sys_handler!(sys_set_tid_address, (args[0])),
        SYSCALL_FUTEX => {
            sys_handler!(sys_futex, (args[0], args[1] as u32, args[2] as u32, args[3], args[4], args[5] as u32), await)
        }
        SYSCALL_SET_ROBUST_LIST => {
            sys_handler!(sys_set_robust_list, (args[0], args[1]))
        }
        SYSCALL_GET_ROBUST_LIST => {
            sys_handler!(sys_get_robust_list, (args[0], args[1], args[2]))
        }
        SYSCALL_NANOSLEEP => sys_handler!(sys_nanosleep, (args[0]), await),
        SYSCALL_SETITIMER => sys_handler!(
            sys_setitimer,
            (
                args[0] as i32,
                args[1] as *const ITimerval,
                args[2] as *mut ITimerval,
            )
        ),
        SYSCALL_CLOCK_SETTIME => {
            sys_handler!(sys_clock_settime, (args[0], args[1] as *const TimeSpec))
        }
        SYSCALL_CLOCK_GETTIME => {
            sys_handler!(sys_clock_gettime, (args[0], args[1] as *mut TimeSpec))
        }
        SYSCALL_CLOCK_GETRES => sys_handler!(sys_clock_getres, (args[0], args[1] as *mut TimeSpec)),
        SYSCALL_CLOCK_NANOSLEEP => sys_handler!(
            sys_clock_nanosleep,
            (
                args[0],
                args[1] as u32,
                args[2],
                args[3]
            ), await
        ),
        SYSCALL_SYSLOG => sys_handler!(
            sys_syslog,
            (args[0] as u32, args[1] as *mut u8, args[2] as u32)
        ),
        SYSCALL_SCHED_SETSCHEDULER => sys_handler!(sys_sched_setscheduler, ()),
        SYSCALL_SCHED_GETSCHEDULER => sys_handler!(sys_sched_getscheduler, ()),
        SYSCALL_SCHED_GETPARAM => sys_handler!(sys_sched_getparam, ()),
        SYSCALL_SCHED_SETAFFINITY => {
            sys_handler!(sys_sched_setaffinity, (args[0], args[1], args[2]))
        }
        SYSCALL_SCHED_GETAFFINITY => {
            sys_handler!(sys_sched_getaffinity, (args[0], args[1], args[2]))
        }
        SYSCALL_YIELD => sys_handler!(sys_yield, (), await),
        SYSCALL_KILL => sys_handler!(sys_kill, (args[0] as isize, args[1] as i32)),
        SYSCALL_RT_SIGACTION => sys_handler!(
            sys_rt_sigaction,
            (
                args[0] as i32,
                args[1] as *const SigAction,
                args[2] as *mut SigAction,
            )
        ),
        SYSCALL_RT_SIGPROCMASK => sys_handler!(
            sys_rt_sigprocmask,
            (
                args[0] as i32,
                args[1] as *const u32,
                args[2] as *mut SigSet,
            )
        ),
        SYSCALL_RT_SIGTIMEDWAIT => sys_handler!(
            sys_rt_sigtimedwait,
            (
                args[0] as *const u32,
                args[1] as *const u8,
                args[2] as *const u8,
            )
        ),
        SYSCALL_RT_SIGRETURN => sys_handler!(sys_rt_sigreturn, ()),
        SYSCALL_TIMES => sys_handler!(sys_times, (args[0] as *mut Tms)),
        SYSCALL_SETPGID => sys_handler!(sys_setpgid, (args[0], args[1])),
        SYSCALL_GETPGID => sys_handler!(sys_getpgid, (args[0])),
        SYSCALL_SETSID => sys_handler!(sys_setsid, ()),
        SYSCALL_UNAME => sys_handler!(sys_uname, (args[0])),
        SYSCALL_GETRUSAGE => sys_handler!(sys_getrusage, (args[0] as i32, args[1])),
        SYSCALL_UMASK => sys_handler!(sys_umask, (args[0] as u32)),
        SYSCALL_GET_TIME => sys_handler!(sys_get_time, (args[0] as *mut TimeVal)),
        SYSCALL_GETPID => sys_handler!(sys_getpid, ()),
        SYSCALL_GETPPID => sys_handler!(sys_getppid, ()),
        SYSCALL_GETUID => sys_handler!(sys_getuid, ()),
        SYSCALL_GETEUID => sys_handler!(sys_geteuid, ()),
        SYSCALL_GETEGID => sys_handler!(sys_getegid, ()),
        SYSCALL_GETTID => sys_handler!(sys_gettid, ()),
        SYSCALL_SYSINFO => sys_handler!(sys_sysinfo, (args[0])),
        SYSCALL_SHMGET => sys_handler!(sys_shmget, (args[0], args[1], args[2] as u32)),
        SYSCALL_SHMCTL => sys_handler!(sys_shmctl, ()),
        SYSCALL_SHMAT => sys_handler!(sys_shmat, (args[0], args[1], args[2] as u32)),
        SYSCALL_SOCKET => {
            sys_handler!(sys_socket, (args[0] as u32, args[1] as u32, args[2] as u32))
        }
        SYSCALL_SOCKETPAIR => sys_handler!(
            sys_socketpair,
            (args[0] as u32, args[1] as u32, args[2] as u32, args[3])
        ),
        SYSCALL_BIND => sys_handler!(sys_bind, (args[0] as u32, args[1], args[2] as u32)),
        SYSCALL_LISTEN => sys_handler!(sys_listen, (args[0] as u32, args[1] as u32)),
        SYSCALL_ACCEPT => sys_handler!(sys_accept, (args[0] as u32, args[1], args[2]), await),
        SYSCALL_CONNECT => {
            sys_handler!(sys_connect, (args[0] as u32, args[1], args[2] as u32), await)
        }
        SYSCALL_GETSOCKNAME => sys_handler!(sys_getsockname, (args[0] as u32, args[1], args[2])),
        SYSCALL_SENDTO => sys_handler!(
            sys_sendto,
            (
                args[0] as u32,
                args[1],
                args[2],
                args[3] as u32,
                args[4],
                args[5] as u32
            ), await
        ),
        SYSCALL_RECVFROM => sys_handler!(
            sys_recvfrom,
            (
                args[0] as u32,
                args[1],
                args[2] as u32,
                args[3] as u32,
                args[4],
                args[5]
            ), await
        ),
        SYSCALL_SETSOCKOPT => sys_handler!(
            sys_setsockopt,
            (
                args[0] as u32,
                args[1] as u32,
                args[2] as u32,
                args[3],
                args[4] as u32
            )
        ),
        SYSCALL_GETSOCKOPT => sys_handler!(
            sys_getsockopt,
            (
                args[0] as u32,
                args[1] as u32,
                args[2] as u32,
                args[3],
                args[4]
            )
        ),
        SYSCALL_TKILL => sys_handler!(sys_tkill, (args[0], args[1] as i32)),
        SYSCALL_TGKILL => sys_handler!(
            sys_tgkill,
            (args[0] as usize, args[1] as usize, args[2] as i32)
        ),
        SYSCALL_RT_SIGSUSPEND => sys_handler!(sys_rt_sigsuspend, (args[0]), await),
        SYSCALL_BRK => sys_handler!(sys_brk, (args[0])),
        SYSCALL_MUNMAP => sys_handler!(sys_munmap, (args[0] as usize, args[1] as usize)),
        SYSCALL_CLONE => sys_handler!(
            sys_clone,
            (
                args[0],
                args[1] as *const u8,
                args[2],
                args[3] as *const u8,
                args[4],
            )
        ),
        SYSCALL_EXECVE => sys_handler!(
            sys_execve,
            (
                args[0] as *const u8,
                args[1] as *const usize,
                args[2] as *const usize,
            )
        ),
        SYSCALL_MMAP => sys_handler!(
            sys_mmap,
            (
                args[0],
                args[1],
                args[2] as i32,
                args[3] as i32,
                args[4],
                args[5],
            )
        ),
        SYSCALL_MPROTECT => sys_handler!(sys_mprotect, (args[0], args[1], args[2] as i32)),
        SYSCALL_MSYNC => sys_handler!(sys_msync, (args[0], args[1], args[2] as i32)),
        SYSCALL_MADVISE => sys_handler!(sys_madvise, ()),
        SYSCALL_WAIT4 => {
            sys_handler!(sys_wait4, (args[0] as isize  , args[1], args[2] as i32), await)
        }
        SYSCALL_PRLIMIT64 => sys_handler!(
            sys_prlimit64,
            (
                args[0],
                args[1] as u32,
                args[2] as *const RLimit,
                args[3] as *mut RLimit,
            )
        ),
        SYSCALL_REMANEAT2 => sys_handler!(
            sys_renameat2,
            (
                args[0] as isize,
                args[1] as *const u8,
                args[2] as isize,
                args[3] as *const u8,
                args[4] as u32,
            )
        ),
        SYSCALL_MEMBARRIER => sys_handler!(sys_membarrier, ()),
        _ => {
            // panic!("Unsupported syscall_id: {}", syscall_id);
            error!("Unsupported syscall_id: {}", syscall_id);
            Ok(0)
        }
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
        /// Stack
        const MAP_STACK = 1 << 17;
    }
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
