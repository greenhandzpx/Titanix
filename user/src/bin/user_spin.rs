#![no_std]
#![no_main]

// use user_lib::println;

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let mut cnt = 0;
    loop {
        cnt += 1;
        if cnt == 200000 {
            // println!("[background]: spin");
            cnt = 0;
        }
    }
}
