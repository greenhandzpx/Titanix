#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{execve, fork, wait};

#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        execve(
            "busybox\0",
            &[
                "busybox\0".as_ptr(),
                "sh\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
            &[
                "PATH=/:/bin:/usr/bin:/usr/local/bin:\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
        );
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let _pid = wait(&mut exit_code);
            // println!(
            //     "[initproc] Released a zombie process, pid={}, exit_code={}",
            //     pid, exit_code,
            // );
        }
    }
    0
}
