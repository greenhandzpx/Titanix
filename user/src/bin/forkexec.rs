#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{execve, fork, getpid, wait};

#[no_mangle]
pub fn main() -> i32 {
    println!("pid {}: parent start forking ...", getpid());
    let pid = fork();
    if pid == 0 {
        // child process
        println!(
            "pid {}: forked child start execing hello_world app ... ",
            getpid()
        );
        execve(
            "hello_world",
            &["hello_world\0".as_ptr(), core::ptr::null::<u8>()],
            &[core::ptr::null::<u8>()],
        );
        100
    } else {
        // parent process
        let mut exit_code: i32 = 0;
        println!("pid {}: ready waiting child ...", getpid());
        assert_eq!(pid, wait(&mut exit_code));
        assert_eq!(exit_code, 0);
        println!(
            "pid {}: got child info:: pid {}, exit code: {}",
            getpid(),
            pid,
            exit_code
        );
        0
    }
}
