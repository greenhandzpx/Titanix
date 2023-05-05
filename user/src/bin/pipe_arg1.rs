#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::getpid;

#[no_mangle]
pub fn main() -> i32 {
    println!("pid {}: father proc send msg to child", getpid());
    0
}
