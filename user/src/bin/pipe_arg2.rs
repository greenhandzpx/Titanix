#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{console::getchar, getpid};

#[no_mangle]
pub fn main() -> i32 {
    loop {
        let c = getchar();
        if c as char == '\n' {
            break;
        }
        print!("{}", c as char);
    }
    0
}
