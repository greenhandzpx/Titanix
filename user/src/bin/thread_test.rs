#![no_std]
#![no_main]

use alloc::vec;
use user_lib::{clone, exit, sleep, CloneFlags};

#[macro_use]
extern crate user_lib;
extern crate alloc;

fn func(num: *const u8) -> isize {
    let num = unsafe { *(num as *const i32) };
    for _ in 0..100 {
        println!("clone thread arg: {}", num);
    }
    exit(0)
}

#[no_mangle]
pub fn main(_argc: usize, _argv: &[&str]) -> i32 {
    let nums = vec![1, 2, 3];
    // let num1 = 1;
    // let num2 = 2;
    // let num3 = 3;
    for i in 0..nums.len() {
        clone(
            func,
            core::ptr::null::<u8>(),
            CloneFlags::CLONE_THREAD.bits() as i32,
            &nums[i] as *const i32 as *const u8,
        );
    }
    // clone(func, core::ptr::null::<u8>(), CloneFlags::CLONE_THREAD.bits() as i32, &num as *const i32 as *const u8);
    let time = 1000;
    println!("sleep for {} ms...", time);
    sleep(time);
    println!("sleep over");
    0
}
