#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::string::String;
use user_lib::{close, exit, openat, read, OpenFlags};

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    let fd = openat("/dev/zero\0", OpenFlags::O_RDONLY);
    if fd <= 0 {
        println!("Error occurred when opening file");
        exit(-1);
        // panic!("Error occurred when opening file");
    } else {
        println!("open file success");
    }
    let fd = fd as usize;
    let mut buf = [0u8; 16];
    let mut s = String::new();
    loop {
        let size = read(fd, &mut buf) as usize;
        s.push_str(core::str::from_utf8(&buf[..size]).unwrap());
        break;
        if size == 0 {
            break;
        }
        // break;
    }
    println!("hh {}", s);
    close(fd);
    0
}