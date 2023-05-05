#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;


use user_lib::{getpid, clone, CloneFlags};


const MAX_NUM: usize = 1000000;

fn func(id: *const u8) -> isize {
    let id = unsafe {
        *id
    };
    for _ in 0..MAX_NUM {
        println!("hhh {}", id);
    }
    0
}

#[no_mangle]
pub fn main() -> i32 {
    return 0; 
    // for i in 0..4 {
    //     let addr = &i;
    //     clone(func, core::ptr::null::<u8>(), CloneFlags::CLONE_THREAD.bits() as i32, addr as *const i32 as *const u8);
    // }
    // 0
}
