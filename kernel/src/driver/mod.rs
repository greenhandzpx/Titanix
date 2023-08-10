#![allow(unused_imports)]
use crate::{println, processor::hart::local_hart, stack_trace, sync::mutex::SpinNoIrqLock};
use alloc::sync::Arc;
use core::{
    any::Any,
    fmt::{self, Write},
};

use self::{
    fu740::{sdcard::SDCard, uart::UART},
    plic::{initplic, PLIC},
    qemu::virtio_blk::VirtIOBlock,
    sbi::{console_putchar, SbiChar},
};

pub mod fu740;
pub mod plic;
pub mod qemu;
pub mod sbi;

type Mutex<T> = SpinNoIrqLock<T>;

static PRINT_MUTEX: Mutex<()> = Mutex::new(());

pub fn intr_handler() {
    let mut plic = PLIC::new(0xffff_ffc0_0c00_0000);
    let hart_id = local_hart().hart_id();
    let context_id = hart_id * 2;
    let intr = plic.claim(context_id);
    if intr != 0 {
        #[cfg(feature = "board_u740")]
        match intr {
            39 => { // uart
            }
            _ => {
                panic!("unexpected interrupt {}", intr);
            }
        }
        #[cfg(feature = "board_qemu")]
        match intr {}
        plic.complete(context_id, intr);
    }
}

// Block Device
pub trait BlockDevice: Send + Sync + Any {
    ///Read data form block to buffer
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    ///Write data from buffer to block
    fn write_block(&self, block_id: usize, buf: &[u8]);
}

// Character Device
pub trait CharDevice: Send + Sync {
    fn getchar(&self) -> u8;
    fn puts(&self, char: &[u8]);
}

// Net Device
pub trait NetDevice: smoltcp::phy::Device {}

pub static BLOCK_DEVICE: Mutex<Option<Arc<dyn BlockDevice>>> = Mutex::new(None);
pub static CHAR_DEVICE: Mutex<Option<Arc<dyn CharDevice>>> = Mutex::new(None);

fn init_block_device() {
    #[cfg(not(feature = "board_u740"))]
    {
        *BLOCK_DEVICE.lock() = Some(Arc::new(VirtIOBlock::new()));
    }
    #[cfg(feature = "board_u740")]
    {
        *BLOCK_DEVICE.lock() = Some(Arc::new(SDCard::new(0xffff_ffc0_1005_0000)));
    }
}

fn init_char_device() {
    #[cfg(not(feature = "board_u740"))]
    {
        *CHAR_DEVICE.lock() = Some(Arc::new(SbiChar::new()));
    }
    #[cfg(feature = "board_u740")]
    {
        *CHAR_DEVICE.lock() = Some(Arc::new(UART::new(0xffff_ffc0_1001_0000)));
    }
}

pub fn init() {
    initplic(0xffff_ffc0_0c00_0000);
    init_char_device();
    init_block_device();
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let char_device = CHAR_DEVICE.lock();
        if let Some(cd) = char_device.as_ref() {
            cd.puts(s.as_bytes());
        } else {
            for s in s.as_bytes() {
                console_putchar(*s as usize);
            }
        }
        Ok(())
    }
}

pub fn getchar() -> u8 {
    let char_device = CHAR_DEVICE.lock();
    if let Some(cd) = char_device.as_ref() {
        cd.clone().getchar()
    } else {
        0xff
    }
}

pub fn print(args: fmt::Arguments<'_>) {
    let _lock = PRINT_MUTEX.lock();
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
/// print string macro
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::driver::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
/// println string macro
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::driver::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub fn shutdown() -> ! {
    sbi::shutdown()
}

pub fn set_timer(timer: usize) {
    sbi::set_timer(timer)
}
