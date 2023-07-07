#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, sleep, TimeVal};

#[no_mangle]
pub fn main() -> i32 {
    println!("into sleep test!");
    let mut start = TimeVal { sec: 0, usec: 0 };
    get_time(&mut start);
    let start = start.sec * 1000 + start.usec / 1000;
    println!("current time_msec = {}", start);
    sleep(100);
    let mut end = TimeVal { sec: 0, usec: 0 };
    get_time(&mut end);
    let end = end.sec * 1000 + end.usec / 1000;
    println!(
        "time_msec = {} after sleeping 100 ticks, delta = {}ms!",
        end,
        end - start
    );
    println!("r_sleep passed!");
    0
}
