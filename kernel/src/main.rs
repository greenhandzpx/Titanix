//! The main module and entrypoint

#![deny(missing_docs)]
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
#![feature(let_chains)]
#![feature(once_cell)]

extern crate alloc;
// extern crate intrusive_collections;

#[macro_use]
extern crate bitflags;

#[path = "boards/mod.rs"]
mod board;

mod config;
#[macro_use]
mod driver;
mod executor;
mod fs;
mod loader;
pub mod mm;
mod net;
mod panic;
pub mod process;
mod processor;
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

use crate::{
    config::mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS},
    // fs::inode_tmp::list_apps,
    mm::KERNEL_SPACE,
    process::thread,
    processor::hart,
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

// pub static FIRST_HART_ID: AtomicU8 = AtomicU8::new(0);
static FIRST_HART: AtomicBool = AtomicBool::new(true);
static INIT_FINISHED: AtomicBool = AtomicBool::new(false);

#[allow(unused)]
fn hart_start(hart_id: usize) {
    use crate::config::mm::HART_START_ADDR;
    use crate::driver::sbi;
    use crate::processor::HARTS;
    // only start two harts
    let mut has_another = false;
    let hart_num = unsafe { HARTS.len() };
    for i in 0..hart_num {
        #[cfg(feature = "board_u740")]
        if i == 0 {
            continue;
        }
        if has_another {
            break;
        }
        if i == hart_id {
            continue;
        }
        println!("[kernel] start to wake up hart {}...", i);
        sbi::hart_start(i, HART_START_ADDR);
        has_another = true;
    }
}

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
pub fn rust_main(hart_id: usize) {
    if FIRST_HART
        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
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
        mm::remap_test();
        trap::init();
        driver::init();
        executor::init();
        loader::init();
        fs::init();
        timer::init();
        net::config::init();

        thread::spawn_kernel_thread(async move {
            process::add_initproc();
        });

        // barrier
        INIT_FINISHED.store(true, Ordering::SeqCst);

        #[cfg(feature = "multi_hart")]
        hart_start(hart_id);

        trap::enable_timer_interrupt();
        timer::set_next_trigger();
    } else {
        // The other harts
        hart::init(hart_id);

        // barrier
        while !INIT_FINISHED.load(Ordering::SeqCst) {}

        println!(
            "[kernel] ---------- hart {} is starting... ----------",
            hart_id
        );

        #[cfg(feature = "board_u740")]
        {
            driver::fu740::plic::plic_inithart();
        }

        trap::init();
        unsafe {
            KERNEL_SPACE
                .as_ref()
                .expect("KERNEL SPACE not init yet")
                .activate();
        }
        println!("[kernel] ---------- hart {} started ---------- ", hart_id);
        // println!("[other hart] current time {:?}", current_time_duration());

        trap::enable_timer_interrupt();
        timer::set_next_trigger();
    }

    executor::run_forever();
}
