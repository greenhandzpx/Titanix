#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exit, fork, get_time, sleep, waitpid, TimeVal};

fn sleepy() {
    let time: usize = 100;
    for i in 0..5 {
        sleep(time);
        println!("sleep {} x {} msecs.", i + 1, time);
    }
    exit(0);
}

#[no_mangle]
pub fn main() -> i32 {
    let mut current_time = TimeVal {
        sec: 0,
        usec: 0,
    };
    get_time(&mut current_time);
    let current_time = current_time.sec * 1000 + current_time.usec / 1000;
    let pid = fork();
    let mut exit_code: i32 = 0;
    if pid == 0 {
        sleepy();
    }
    assert!(waitpid(pid as usize, &mut exit_code) == pid && exit_code == 0);

    let mut current_time1 = TimeVal {
        sec: 0,
        usec: 0,
    };
    get_time(&mut current_time1);
    let current_time1 = current_time1.sec * 1000 + current_time1.usec / 1000;
    println!("use {} msecs.", current_time1 - current_time);
    println!("sleep pass.");
    0
}
