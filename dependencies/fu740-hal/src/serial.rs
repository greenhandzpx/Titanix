//! Serial interface

use core::convert::Infallible;
use core::mem;

use embedded_hal::serial;
use nb;

use crate::clock::Clocks;
use crate::pac::{UART0, UART1};
use crate::time::Bps;

mod closed_traits {
    use crate::pac::uart0;
    use core::ops::Deref;

    pub trait UartX: Deref<Target = uart0::RegisterBlock> {}
}
use closed_traits::UartX;
impl UartX for UART0 {}
impl UartX for UART1 {}

/// Serial abstraction
pub struct Serial<UART> {
    uart: UART,
}

/// Serial receiver
pub struct Rx<UART> {
    uart: UART,
}

/// Serial transmitter
pub struct Tx<UART> {
    uart: UART,
}

impl<UART: UartX> Serial<UART> {
    /// Configures a UART peripheral to provide serial communication
    pub fn new(uart: UART, baud_rate: Bps, clocks: &Clocks) -> Self {
        let div = clocks.pclk().0 / baud_rate.0 - 1;
        assert!(div <= 0xffff);
        uart.ie.modify(|_, w| {
            w.txwm().clear_bit();
            w.rxwm().clear_bit()
        });
        unsafe {
            uart.div.write_with_zero(|w| w.bits(div));
        }
        uart.txctrl.modify(|_, w| {
            unsafe {
                w.txcnt().bits(1);
            }
            w.nstop().set_bit();
            w.txen().set_bit()
        });
        uart.rxctrl.modify(|_, w| {
            unsafe {
                w.rxcnt().bits(0);
            }
            w.rxen().set_bit()
        });

        Serial { uart }
    }

    /// Starts listening for an interrupt event
    pub fn listen(self) -> Self {
        self.uart.ie.modify(|_, w| w.rxwm().set_bit());
        self
    }

    /// Stops listening for an interrupt event
    pub fn unlisten(self) -> Self {
        self.uart.ie.modify(|_, w| w.rxwm().clear_bit());
        self
    }

    /// Splits the `Serial` abstraction into a transmitter and a
    /// receiver half
    pub fn split(self) -> (Tx<UART>, Rx<UART>) {
        (
            Tx {
                uart: unsafe { mem::zeroed() },
            },
            Rx { uart: self.uart },
        )
    }

    /// Releases the UART peripheral
    pub fn free(self) -> UART {
        self.uart
    }
}

impl<UART: UartX> serial::Read<u8> for Rx<UART> {
    type Error = Infallible;

    fn read(&mut self) -> nb::Result<u8, Infallible> {
        let rxdata = self.uart.rxdata.read();

        if rxdata.empty().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            Ok(rxdata.data().bits() as u8)
        }
    }
}

impl<UART: UartX> serial::Write<u8> for Tx<UART> {
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> nb::Result<(), Infallible> {
        let txdata = self.uart.txdata.read();

        if txdata.full().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            unsafe {
                self.uart.txdata.write_with_zero(|w| w.data().bits(byte));
            }
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Infallible> {
        if self.uart.ip.read().txwm().bit_is_set() {
            // FIFO count is below the receive watermark (1)
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
