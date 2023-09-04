#![allow(unused)]
use core::task::Waker;

use alloc::boxed::Box;

use crate::{driver::CharDevice, sync::mutex::SpinLock};

type Callback = Box<dyn Fn(u8) + Send + Sync>;
type Mutex<T> = SpinLock<T>;

pub struct UART {
    base_addr: usize,
    waker: Mutex<Option<Waker>>,
    cb: Callback,
}

impl UART {
    pub fn new(base_addr: usize, cb: Callback) -> Self {
        let ret = Self {
            base_addr,
            waker: Mutex::new(None),
            cb,
        };
        ret.init();
        ret
    }
    fn txdata_ptr(&self) -> *mut u8 {
        self.base_addr as *mut u8
    }
    fn rxdata_ptr(&self) -> *mut u8 {
        self.base_addr as *mut u8
    }
    fn ie_ptr(&self) -> *mut u8 {
        (self.base_addr + 1) as *mut u8
    }
    fn fifo_ctrl_ptr(&self) -> *mut u8 {
        (self.base_addr + 2) as *mut u8
    }
    fn is_ptr(&self) -> *mut u8 {
        (self.base_addr + 2) as *mut u8
    }
    fn line_ctrl_ptr(&self) -> *mut u8 {
        (self.base_addr + 3) as *mut u8
    }
    fn line_status_ptr(&self) -> *mut u8 {
        (self.base_addr + 5) as *mut u8
    }

    pub fn init(&self) {
        unsafe {
            self.ie_ptr().write_volatile(0);
            self.fifo_ctrl_ptr().write_volatile((1 << 0) | (3 << 1));
            self.ie_ptr().write_volatile(1);
        }
    }
    fn putchar(&self, s: u8) {
        unsafe {
            while (self.line_status_ptr().read_volatile() & (1 << 5)) == 0 {}
            self.txdata_ptr().write_volatile(s);
        }
    }
}

impl CharDevice for UART {
    fn puts(&self, s: &[u8]) {
        for c in s {
            if *c == b'\n' {
                self.putchar(b'\r');
            }
            self.putchar(*c);
        }
    }
    fn getchar(&self) -> u8 {
        unsafe {
            if (self.line_status_ptr().read_volatile() & 0x01) == 0x01 {
                self.rxdata_ptr().read_volatile()
            } else {
                0xff
            }
        }
    }
    fn register_waker(&self, waker: core::task::Waker) {
        *self.waker.lock() = Some(waker);
    }
    fn handle_irq(&self) {
        let mut ch = self.getchar();
        loop {
            log::debug!("[UART::handle_irq] ch {}", ch);
            (self.cb)(ch);
            ch = self.getchar();
            if ch == 0xff {
                break;
            }
        }
        if let Some(w) = self.waker.lock().take() {
            w.wake();
        }
    }
}
