/*
 * @Author: greenhandzpx 893522573@qq.com
 * @Date: 2023-01-28 12:24:08
 * @LastEditors: greenhandzpx 893522573@qq.com
 * @LastEditTime: 2023-02-25 09:26:27
 * @FilePath: /oscomp-kernel/os/src/main.rs
 * @Description:
 *
 * Copyright (c) 2023 by greenhandzpx 893522573@qq.com, All Rights Reserved.
 */
//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`task`]: Task management
//! - [`syscall`]: System call handling and implementation
//! - [`mm`]: Address map using SV39
//! - [`sync`]:Wrap a static data structure inside it so that we are able to access it without any `unsafe`.
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`task::run_tasks()`] and for the first time go to
//! userspace.

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
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]

extern crate alloc;

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
mod lang_items;
mod loader;
pub mod mm;
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
    sync::atomic::{AtomicBool, Ordering},
};

use log::info;

use crate::{
    config::mm::HART_START_ADDR,
    fs::inode_tmp::list_apps,
    mm::KERNEL_SPACE,
    processor::{hart, HARTS},
    sbi::hart_start,
};

global_asm!(include_str!("entry.asm"));
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
static FIRST_HART: AtomicBool = AtomicBool::new(true);

// TODO: We will add multi cores support in the future
#[no_mangle]
/// the rust entry-point of os
// pub fn rust_main(hart_id: usize) -> ! {
pub fn rust_main(hart_id: usize) {
    if FIRST_HART
        .compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        // the first hart
        clear_bss();

        processor::init();
        unsafe {
            processor::set_local_hart(hart_id);
        }
        processor::set_hart_stack();

        utils::logging::init();

        info!(r#"   _______ __              _     "#);
        info!(r#" /_  __(_) /_____ _____  (_)  __"#);
        info!(r#"  / / / / __/ __ `/ __ \/ / |/_/"#);
        info!(r#" / / / / /_/ /_/ / / / / />  <  "#);
        info!(r#"/_/ /_/\__/\__,_/_/ /_/_/_/|_|  "#);
        info!("[kernel] Hello, world!");

        info!(
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
        // list_apps();

        fs::init();

        mm::page_cache_test();

        process::thread::spawn_kernel_thread(async move {
            process::add_initproc();
            // process::scan_prilimary_tests();
            // println!("after initproc!");
        });

        let hart_num = unsafe { HARTS.len() };
        for i in 0..hart_num {
            if i == hart_id {
                continue;
            }
            hart_start(i, HART_START_ADDR);
        }
    } else {
        unsafe {
            processor::set_local_hart(hart_id);
        }
        processor::set_hart_stack();
        unsafe {
            KERNEL_SPACE
                .as_ref()
                .expect("KERNEL SPACE not init yet")
                .activate();
        }
        info!("[kernel] ---------- hart {} started ---------- ", hart_id);
        return;
    }
    executor::run_until_idle();

    // process::run_tasks();
    panic!("Unreachable in rust_main!");
}
