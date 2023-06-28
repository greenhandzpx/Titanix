#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{execve, fork, wait};

#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        // execve(
        //     "shell\0",
        //     &["shell\0".as_ptr(), core::ptr::null::<u8>()],
        //     &[core::ptr::null::<u8>()],
        // );
        execve(
            "busybox\0",
            &["busybox\0".as_ptr(), "sh\0".as_ptr(), core::ptr::null::<u8>()],
            &[core::ptr::null::<u8>()],
        );
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid, exit_code,
            );
        }
    }
    0
}
