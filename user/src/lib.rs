#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
mod error;
mod lang_items;
mod syscall;
pub use syscall::SigAction;
pub use syscall::SigSet;
pub use syscall::Signal;

#[macro_use]
extern crate bitflags;
extern crate alloc;

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
pub use error::SyscallErr;
use syscall::*;

// const USER_HEAP_SIZE: usize = 16384;
const USER_HEAP_SIZE: usize = 0x32000;

// Note that heap space is allocated in .data segment
// TODO: can we change to dynamically allocate by invoking sys_sbrk?
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
// pub extern "C" fn _start(sp: usize) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut v: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start =
            unsafe { ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile() };
        let len = (0usize..)
            .find(|i| unsafe { ((str_start + *i) as *const u8).read_volatile() == 0 })
            .unwrap();
        v.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            })
            .unwrap(),
        );
    }
    let exit_code = main(argc, v.as_slice());
    // println!("program {} will exit", v[0]);
    exit(exit_code);
}

#[linkage = "weak"]
#[no_mangle]
fn main(_: usize, _: &[&str]) -> i32 {
    panic!("Cannot find main!");
}

bitflags! {
    pub struct OpenFlags: u32 {
        const O_RDONLY = 0;
        const O_WRONLY = 1 << 0;
        const O_RDWR = 1 << 1;
        const O_CLOEXEC = 1 << 7;
        const O_CREATE = 1 << 9;
        const O_TRUNC = 1 << 10;
    }
}
const AT_FDCWD: isize = -100;

#[macro_export]
macro_rules! wexitstatus {
    ($a: expr) => {
        ($a & 0xffffff00) >> 8
    };
}

pub fn getcwd(path: usize, len: usize) -> isize {
    sys_getcwd(path, len)
}

pub fn mount(dev_name: usize, target_path: usize, ftype: usize, flags: u32, data: usize) -> isize {
    sys_mount(dev_name, target_path, ftype, flags, data)
}

pub fn uname(buf: usize) -> isize {
    sys_uname(buf)
}

pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}
pub fn dup3(oldfd: usize, newfd: usize, flags: OpenFlags) -> isize {
    sys_dup3(oldfd, newfd, flags.bits)
}
pub fn openat(path: &str, flags: OpenFlags) -> isize {
    // TODO: change to the version that has `mode` arg
    sys_openat(AT_FDCWD as usize, path, flags.bits, 0)
}
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
}
pub fn exit_group(exit_code: i32) -> ! {
    sys_exit_group(exit_code);
}
pub fn yield_() -> isize {
    sys_yield()
}
pub fn get_time(time_val: &mut TimeVal) -> isize {
    sys_get_time(time_val as *mut TimeVal)
}
pub fn getpid() -> isize {
    sys_getpid()
}
pub enum FutexOperations {
    FutexWait = 1,
    FutexWake = 2,
}

pub fn futex(uaddr: *const usize, futex_op: FutexOperations, val: usize) -> isize {
    sys_futex(uaddr, futex_op, val)
}

pub fn kill(pid: isize, signo: i32) -> isize {
    sys_kill(pid, signo)
}
pub fn sigaction(sig_no: Signal, act: &SigAction, old_act: &mut SigAction) -> isize {
    sys_sigaction(sig_no, act, old_act)
}
pub fn sigreturn() -> isize {
    sys_sigreturn()
}
pub fn fork() -> isize {
    sys_fork()
}

fn clone_wrapper(f: fn(*const u8) -> i32, arg: *const u8) -> isize {
    exit(f(arg))
}

bitflags! {
    ///Open file flags
    pub struct CloneFlags: u32 {
        const CLONE_THREAD = 1 << 4;
        const CLONE_VM = 1 << 8;
        const CLONE_FS = 1 << 9;
        const CLONE_FILES = 1 << 10;
    }
}
/// Since rust's variadic params should be achieved by macro, here we just support
/// passing one arg(but can use a struct to pack more args)
pub fn clone(f: fn(*const u8) -> isize, stack: *const u8, flags: i32, arg: *const u8) -> isize {
    sys_clone(f, stack, flags, arg)
}
pub fn execve(path: &str, args: &[*const u8], envp: &[*const u8]) -> isize {
    sys_execve(path, args, envp)
}

pub fn mmap(
    addr: *const u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: usize,
    offset: usize,
) -> isize {
    sys_mmap(addr, length, prot, flags, fd, offset)
}

pub fn wait(exit_code: &mut i32) -> isize {
    sys_waitpid(-1, exit_code as *mut _)
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    sys_waitpid(pid as isize, exit_code as *mut _)
}
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}
pub fn sleep(period_ms: usize) {
    let mut start = TimeVal { sec: 0, usec: 0 };
    assert_eq!(sys_get_time(&mut start as *mut TimeVal), 0);
    let start_ts = start.sec * 1000 + start.usec / 1000;
    let mut tmp = TimeVal { sec: 0, usec: 0 };
    loop {
        assert_eq!(sys_get_time(&mut tmp as *mut TimeVal), 0);
        let curr_ts = tmp.sec * 1000 + tmp.usec / 1000;
        if start_ts + period_ms <= curr_ts {
            break;
        }
        sys_yield();
    }
}

pub fn pipe(pipe_fd: &mut [i32]) -> isize {
    sys_pipe(pipe_fd)
}

pub fn close(fd: usize) -> isize {
    sys_close(fd)
}
