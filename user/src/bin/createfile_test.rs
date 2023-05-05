#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::string::String;
use user_lib::{close, exit, openat, read, write, OpenFlags};

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    let filename = "tmp.txt\0";
    let fd = openat(filename, OpenFlags::O_CREATE);
    if fd <= 0 {
        println!("Error occurred when opening {}", filename);
        exit(-1);
    }

    let buf = "hello world!";
    if write(fd as usize, buf.as_bytes()) < 0 {
        println!("Error occurred when writing {}", filename);
        exit(-1);
    }

    // let mut buf_ret = [0u8; 1024];
    // if read(fd as usize, &mut buf_ret) < 0 {
    // }

    close(fd as usize);
    0
}
