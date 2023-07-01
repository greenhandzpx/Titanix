#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{close, exit, fork, kill, openat, read, sleep, OpenFlags, Signal};

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    let pid = fork();
    if pid == 0 {
        // child
        loop {
            println!("child: I'm still alive!");
            sleep(100);
        }
    } else {
        println!("parent will kill child soon...");
        sleep(1000);
        kill(pid, Signal::SIGKILL as i32);
        println!("parent killed child");
        0
    }
}
