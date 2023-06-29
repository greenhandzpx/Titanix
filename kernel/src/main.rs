//! The main module and entrypoint

#![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(build_hasher_simple_hash_one)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(negative_impls)]
#![feature(sync_unsafe_cell)]
#![feature(linked_list_remove)]
#![feature(core_intrinsics)]
#![feature(const_mut_refs)]
#![feature(poll_ready)]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]

extern crate alloc;
// extern crate intrusive_collections;

#[macro_use]
extern crate bitflags;

#[path = "boards/qemu.rs"]
mod board;
// mod boards;

#[macro_use]
mod console;
mod config;
mod driver;
mod executor;
mod fs;
mod loader;
pub mod mm;
mod panic;
pub mod process;
mod processor;
mod sbi;
mod signal;
///
pub mod sync;
pub mod syscall;
mod timer;
pub mod trap;
mod utils;

use core::{
    arch::{asm, global_asm},
    sync::atomic::{AtomicBool, AtomicU16, AtomicU8, Ordering},
    time::Duration,
};

use log::debug;

use crate::{
    config::mm::{HART_START_ADDR, KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS},
    // fs::inode_tmp::list_apps,
    mm::KERNEL_SPACE,
    process::thread,
    processor::{hart, HARTS},
    sbi::hart_start,
    timer::{timed_task::TimedTaskFuture, timeout_task::ksleep},
};

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));
/// clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

// #[cfg(test)]
// fn test_runner(tests: &[&dyn Fn()]) {
//     println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//     }
// }
///
// pub static FIRST_HART_ID: AtomicU8 = AtomicU8::new(0);
static FIRST_HART: AtomicBool = AtomicBool::new(true);
static INIT_FINISHED: AtomicBool = AtomicBool::new(false);

///
#[no_mangle]
pub fn fake_main(hart_id: usize) {
    unsafe {
        asm!("add sp, sp, {}", in(reg) KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS);
        asm!("la t0, rust_main");
        asm!("add t0, t0, {}", in(reg) KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS);
        asm!("mv a0, {}", in(reg) hart_id);
        asm!("jalr zero, 0(t0)");
    }
}

// TODO: We will add multi cores support in the future
#[no_mangle]
/// the rust entry-point of os
// pub fn rust_main(hart_id: usize) -> ! {
pub fn rust_main(hart_id: usize) {
    if FIRST_HART
        .compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        // The first hart
        clear_bss();

        // processor::init();
        hart::init(hart_id);
        utils::logging::init();

        println!(r#"  _______ __              _     "#);
        println!(r#" /_  __(_) /_____ _____  (_)  __"#);
        println!(r#"  / / / / __/ __ `/ __ \/ / |/_/"#);
        println!(r#" / / / / /_/ /_/ / / / / />  <  "#);
        println!(r#"/_/ /_/\__/\__,_/_/ /_/_/_/|_|  "#);
        println!("[kernel] Hello, world!");
        println!(
            "[kernel] ---------- main hart {} started ---------- ",
            hart_id
        );

        mm::init();
        mm::heap_allocator::heap_test();
        mm::remap_test();
        trap::init();
        //trap::enable_interrupt();
        trap::enable_timer_interrupt();
        timer::set_next_trigger();
        // executor::init();
        // loader::list_apps();
        fs::fat32_tmp::list_apps_fat32();

        fs::init();

        timer::init();

        mm::page_cache_test();

        thread::spawn_kernel_thread(async move {
            process::add_initproc();
        });

        thread::spawn_kernel_thread(async move {
            // TimedTaskFuture::new(Duration::from_secs(3), || {
            //     debug!("I'm awake!! hhh just ignore me");
            //     return true;
            // }, None)
            // .await;
            // loop {
            //     ksleep(Duration::from_secs(5)).await;
            //     debug!("I'm awake!! hhh just ignore me");
            // }
        });

        // INIT_FINISHED.store(true, Ordering::Release);
        INIT_FINISHED.store(true, Ordering::Release);
        // FIRST_HART_ID.store(hart_id as u8, Ordering::SeqCst);

        let hart_num = unsafe { HARTS.len() };
        for i in 0..hart_num {
            if i == hart_id {
                continue;
            }
            hart_start(i, HART_START_ADDR);
        }
    } else {
        // The other harts

        // while !INIT_FINISHED.load(Ordering::Acquire) {}
        while !INIT_FINISHED.load(Ordering::Acquire) {}

        trap::enable_timer_interrupt();
        timer::set_next_trigger();

        hart::init(hart_id);
        unsafe {
            KERNEL_SPACE
                .as_ref()
                .expect("KERNEL SPACE not init yet")
                .activate();
        }
        println!("[kernel] ---------- hart {} started ---------- ", hart_id);

        // return;
    }

    // loop {
    //     executor::run_until_idle();
    // }
    executor::run_forever();
    // panic!("Unreachable in rust_main!");
}
