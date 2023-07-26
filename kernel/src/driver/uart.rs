use crate::driver::sbi::{console_getchar, console_putchar};
use crate::sync::mutex::SpinNoIrqLock;
use core::fmt::{self, Write};
use fu740_pac::{Peripherals, UART0};

pub static UART: SpinNoIrqLock<Option<UART0>> = SpinNoIrqLock::new(None);

pub fn uart_transmit(c: u8, uart: &UART0) {
    while uart.txdata.read().full().bit_is_set() {}
    uart.txdata.modify(|_, w| unsafe { w.data().bits(c) });
}

pub fn uart_recieve(uart: &UART0) -> u8 {
    if uart.rxdata.read().empty().bit_is_clear() {
        0xff
    } else {
        uart.rxdata.read().data().bits()
    }
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        #[cfg(feature = "board_u740")]
        {
            let uart_locked = UART.lock();
            if let Some(uart) = uart_locked.as_ref() {
                for c in s.chars() {
                    if (c as u8) == b'\n' {
                        uart_transmit(b'\r', uart);
                    }
                    uart_transmit(c as u8, uart);
                }
            } else {
                for c in s.chars() {
                    console_putchar(c as usize);
                }
            }
            Ok(())
        }
        #[cfg(not(feature = "board_u740"))]
        {
            let uart_locked = UART.lock();
            for c in s.chars() {
                console_putchar(c as usize)
            }
            Ok(())
        }
    }
}

pub fn print(args: fmt::Arguments<'_>) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
/// print string macro
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::driver::uart::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
/// println string macro
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::driver::uart::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub fn getchar() -> u8 {
    #[cfg(feature = "board_u740")]
    {
        let uart_locked = UART.lock();
        if let Some(uart) = uart_locked.as_ref() {
            uart_recieve(uart)
        } else {
            0xff
        }
    }
    #[cfg(not(feature = "board_u740"))]
    {
        console_getchar()
    }
}

pub fn init() {
    let mut uart_locked = UART.lock();
    *uart_locked = Some(unsafe { Peripherals::steal() }.UART0);
    drop(uart_locked);
}
