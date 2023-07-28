#![allow(dead_code)]

use alloc::sync::Arc;
use fu740_pac::{Peripherals, UART0};

use crate::driver::{CharDevice, Mutex};

pub struct UartSerial(Arc<Mutex<UART0>>);

impl UartSerial {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(unsafe { Peripherals::steal().UART0 })))
    }
    fn putc(uart: &UART0, c: u8) {
        while uart.txdata.read().full().bit_is_set() {}
        uart.txdata.modify(|_, w| unsafe { w.data().bits(c) });
    }
}

impl CharDevice for UartSerial {
    fn getchar(&self) -> u8 {
        let uart = self.0.lock();
        if uart.rxdata.read().empty().bit_is_clear() {
            0xff
        } else {
            uart.rxdata.read().data().bits()
        }
    }

    fn puts(&self, str: &[u8]) {
        let uart = self.0.lock();
        for s in str {
            if *s == b'\n' {
                Self::putc(&uart, b'\r'.into());
            }
            Self::putc(&uart, *s);
        }
    }
}
