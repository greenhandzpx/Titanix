use fu740_pac::uart0::rxctrl;

use crate::{driver::CharDevice, println};

pub struct UART {
    base_addr: usize,
}

impl UART {
    pub fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }
    fn txdata_ptr(&self) -> *mut u32 {
        self.base_addr as *mut u32
    }
    fn rxdata_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x4) as *mut u32
    }
    fn txctrl_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x8) as *mut u32
    }
    fn rxctrl_ptr(&self) -> *mut u32 {
        (self.base_addr + 0xC) as *mut u32
    }
    fn ie_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x10) as *mut u32
    }
    fn ip_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x14) as *mut u32
    }
    fn div_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x18) as *mut u32
    }

    pub fn init(&self) {
        unsafe {
            // set ie rxwm enable, num_entries > rxcnt
            // set ie txwm enable, num_entries < txcnt
            // when rx is not empty, send intr
            // when tx is empty, send intr
            self.ie_ptr().write_volatile(3);
            self.set_rxcnt(0);
            self.set_txcnt(1);
        }
    }

    fn set_rxcnt(&self, rxcnt: u32) {
        unsafe {
            let rxctrl = self.rxctrl_ptr().read_volatile();
            self.rxctrl_ptr()
                .write_volatile((rxctrl & !(7 << 16)) | ((rxcnt & 7) << 16));
        }
    }

    fn set_txcnt(&self, txcnt: u32) {
        unsafe {
            let txctrl = self.txctrl_ptr().read_volatile();
            self.txctrl_ptr()
                .write_volatile((txctrl & !(7 << 16)) | ((txcnt & 7) << 16));
        }
    }

    fn putchar(&self, s: u8) {
        unsafe {
            while self.txdata_ptr().read_volatile() & 0x8000_0000 == 0x8000_0000 {}
            self.txdata_ptr().write_volatile(s as u32);
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
            let ret = self.rxdata_ptr().read_volatile();
            if ret & 0x8000_0000 == 0x8000_0000 {
                0xff
            } else {
                (ret & 0xff) as u8
            }
        }
    }
}
