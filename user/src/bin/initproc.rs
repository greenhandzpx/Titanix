#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{console::getchar, execve, fork, getpid, wait, yield_};

#[no_mangle]
fn main() -> i32 {
    println!("initproc!!");
    if fork() == 0 {
        // println!("exec user shell");
        // let c = getchar();
        execve(
            "shell\0",
            &["shell\0".as_ptr(), core::ptr::null::<u8>()],
            &[core::ptr::null::<u8>()],
        );
        // println!("exec after");
    } else {
        println!("[initproc] Wait for children");
        loop {
            let mut exit_code: i32 = 0;
            // println!("exit code addr {:#x}", &exit_code as *const _ as usize);
            // println!("I'm initproc(father)! pid {}", getpid());
            let pid = wait(&mut exit_code);
            // if pid == -1 {
            //     yield_();
            //     continue;
            // }
            println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid, exit_code,
            );
        }
        println!("[initproc] will die!!");
    }
    0
}
