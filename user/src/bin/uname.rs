#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::string::String;
use user_lib::uname;

fn func() {}

#[no_mangle]
pub fn main() -> i32 {
    let un = UtsName {
        sysname: ['\0' as u8; 65],
        nodename: ['\0' as u8; 65],
        release: ['\0' as u8; 65],
        version: ['\0' as u8; 65],
        machine: ['\0' as u8; 65],
        domainname: ['\0' as u8; 65],
    };
    let test_ret = uname(&un as *const UtsName as usize);
    assert!(test_ret >= 0);
    println!("get ret: {}", test_ret);
    println!(
        "Uname: {} {} {} {} {} {}",
        bytes_to_string(un.sysname),
        bytes_to_string(un.nodename),
        bytes_to_string(un.release),
        bytes_to_string(un.version),
        bytes_to_string(un.machine),
        bytes_to_string(un.domainname)
    );
    0
}

pub struct UtsName {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}

pub fn bytes_to_string(bytes: [u8; 65]) -> String {
    let mut string = String::new();
    for &byte in &bytes {
        string.push(byte as char);
    }
    string
}
