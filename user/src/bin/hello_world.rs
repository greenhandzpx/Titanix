#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::getpid;

fn func() {}

#[no_mangle]
pub fn main() -> i32 {
    // let ptr = func as *mut u8;
    // unsafe {
    //     println!("func addr {:#x}", ptr as usize);
    //     *ptr = 10;
    //     println!("ptr val {}", *ptr);
    // }

    println!("pid {}: Hello world from user mode program!", getpid());
    0
}
