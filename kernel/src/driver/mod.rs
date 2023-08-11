#![allow(unused_imports)]
use crate::{mm::memory_space::phys_to_virt, stack_trace, sync::mutex::SpinNoIrqLock};
use alloc::{string::String, sync::Arc, vec::Vec};
use core::{
    any::Any,
    fmt::{self, Write},
};
use smoltcp::{
    phy::{Loopback, RxToken, TxToken},
    wire::{EthernetAddress, IpAddress, IpEndpoint, Ipv4Address},
};

use self::{
    fu740::{sdcard::SDCardWrapper, uart::UartSerial},
    qemu::{virtio_blk::VirtIOBlock, virtio_net::VirtIONetDevice},
    sbi::{console_putchar, SbiChar},
};

pub mod device_tree;
pub mod fu740;
pub mod qemu;
pub mod sbi;

type Mutex<T> = SpinNoIrqLock<T>;

static PRINT_MUTEX: Mutex<()> = Mutex::new(());

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

pub static BLOCK_DEVICE: Mutex<Option<Arc<dyn BlockDevice>>> = Mutex::new(None);
pub static CHAR_DEVICE: Mutex<Option<Arc<dyn CharDevice>>> = Mutex::new(None);
#[cfg(not(feature = "board_u740"))]
pub static NET_DEVICE: Mutex<Option<VirtIONetDevice>> = Mutex::new(None);
#[cfg(feature = "board_u740")]
pub static NET_DEVICE: Mutex<Option<Loopback>> = Mutex::new(None);

fn init_block_device() {
    #[cfg(not(feature = "board_u740"))]
    {
        *BLOCK_DEVICE.lock() = Some(Arc::new(VirtIOBlock::new()));
    }
    #[cfg(feature = "board_u740")]
    {
        *BLOCK_DEVICE.lock() = Some(Arc::new(SDCardWrapper::new()));
    }
}

fn init_char_device() {
    #[cfg(not(feature = "board_u740"))]
    {
        *CHAR_DEVICE.lock() = Some(Arc::new(SbiChar::new()));
    }
    #[cfg(feature = "board_u740")]
    {
        *CHAR_DEVICE.lock() = Some(Arc::new(SbiChar::new()));
    }
}

fn init_net_device() {
    #[cfg(not(feature = "board_u740"))]
    {
        *NET_DEVICE.lock() = Some(VirtIONetDevice::new());
    }
    #[cfg(feature = "board_u740")]
    {
        *NET_DEVICE.lock() = Some(Arc::new(Loopback::new(smoltcp::phy::Medium::Ethernet)));
    }
}

pub fn init(dtb: usize) {
    let dtb = phys_to_virt(dtb);
    device_tree::init(dtb);

    #[cfg(not(feature = "board_u740"))]
    // init_virt_addr();
    init_char_device();
    init_block_device();
    init_net_device();
    #[cfg(feature = "board_u740")]
    {
        fu740::plic::init_plic();
        // fu740::prci::init_prci();
    }
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
