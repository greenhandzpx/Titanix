#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{
    close, exit, fork, kill, openat, read, sigaction, sigreturn, sleep, OpenFlags, SigAction,
    SigSet, Signal,
};

fn sig_handler(signo: i32) {
    println!("I received a signal {}!", signo);
    sigreturn();
}

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    let pid = fork();
    if pid == 0 {
        // child
        let act = SigAction {
            sa_handler: sig_handler,
            sa_mask: SigSet::from_bits(0).unwrap(),
        };
        let mut old_act = SigAction {
            sa_handler: sig_handler,
            sa_mask: SigSet::from_bits(0).unwrap(),
        };
        sigaction(Signal::SIGABRT, &act, &mut old_act);
        loop {
            println!("child: I'm still alive!");
            sleep(100);
        }
    } else {
        println!("parent will abort child soon...");
        sleep(700);
        kill(pid, Signal::SIGABRT as i32);
        println!("parent will kill child soon...");
        sleep(700);
        kill(pid, Signal::SIGKILL as i32);
        println!("parent killed child");
        0
    }
}
