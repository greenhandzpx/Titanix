#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exit, fork, wait};

const LEN: usize = 100;

#[no_mangle]
pub fn main() -> i32 {
    if fork() == 0 {
        let p = 5u64;
        let m = 998244353u64;
        let iter: usize = 14000000;
        let mut s = [0u64; LEN];
        let mut cur = 0usize;
        s[cur] = 1;
        for i in 1..=iter {
            let next = if cur + 1 == LEN { 0 } else { cur + 1 };
            s[next] = s[cur] * p % m;
            cur = next;
            if i % 10000 == 0 {
                println!("power_5 [{}/{}]", i, iter);
            }
        }
        println!("{}^{} = {}(MOD {})", p, iter, s[cur], m);
        println!("Test power_5 OK!");
    } else {
        let p = 7u64;
        let m = 998244353u64;
        let iter: usize = 16000000;
        let mut s = [0u64; LEN];
        let mut cur = 0usize;
        s[cur] = 1;
        for i in 1..=iter {
            let next = if cur + 1 == LEN { 0 } else { cur + 1 };
            s[next] = s[cur] * p % m;
            cur = next;
            if i % 10000 == 0 {
                println!("power_7 [{}/{}]", i, iter);
            }
        }
        println!("{}^{} = {}(MOD {})", p, iter, s[cur], m);
        println!("Test power_7 OK!");

        let mut exit_code: i32 = 0;
        wait(&mut exit_code);
    }

    0
}
