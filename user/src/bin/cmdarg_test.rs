#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    for i in 0..argc {
        println!("argv {}: {}", i, argv[i]);
    }
    0
}
