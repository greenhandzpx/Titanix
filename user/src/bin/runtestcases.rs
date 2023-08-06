#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use user_lib::{execve, fork, wait, waitpid};

#[macro_use]
extern crate user_lib;

// const TESTCASES: [&str; 0] = [];

const TESTCASES: [&str; 11] = [
    "busybox_testcode.sh",
    "time-test",
    "iperf_testcode.sh",
    "netperf_testcode.sh",
    "lmbench_testcode.sh",
    "libctest_testcode.sh",
    "lua_testcode.sh",
    "iozone_testcode.sh",
    "libc-bench",
    "unixbench_testcode.sh",
    "cyclictest_testcode.sh",
];


#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        for testcase in TESTCASES {
            let pid = fork();
            if pid == 0 {
                let testname = testcase.to_string() + "\0";
                if execve(
                    &testname,
                    &[testname.as_ptr(), core::ptr::null::<u8>()],
                    &[core::ptr::null::<u8>()],
                ) != 0
                {
                    println!("Error when executing!");
                    return 0;
                }
            } else {
                let mut exit_code: i32 = 0;
                waitpid(pid as usize, &mut exit_code);
            }
        }
        println!(" !TEST FINISH! ");
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let _pid = wait(&mut exit_code);
            // println!(
            //     "[initproc] Released a zombie process, pid={}, exit_code={}",
            //     pid, exit_code,
            // );
        }
    }
    0
}
