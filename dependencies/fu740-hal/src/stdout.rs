//! Stdout based on the UART hooked up to FTDI or J-Link

use crate::{
    clock::Clocks,
    pac::UART0,
    serial::{Serial, Tx},
    time::Bps,
};
use core::fmt::{self, Write};
use embedded_hal::prelude::*;
use nb::block;
use riscv::interrupt;

static mut STDOUT: Option<SerialWrapper> = None;

struct SerialWrapper(Tx<UART0>);

impl fmt::Write for SerialWrapper {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            if *byte == b'\n' {
                let res = block!(self.0.write(b'\r'));

                if res.is_err() {
                    return Err(::core::fmt::Error);
                }
            }

            let res = block!(self.0.write(*byte));

            if res.is_err() {
                return Err(::core::fmt::Error);
            }
        }
        Ok(())
    }
}

/// Configures stdout
pub fn configure(uart: UART0, baud_rate: Bps, clocks: &Clocks) {
    let serial = Serial::new(uart, baud_rate, clocks);
    let (tx, _) = serial.split();

    interrupt::free(|_| unsafe {
        STDOUT.replace(SerialWrapper(tx));
    })
}

/// Writes string to stdout
pub fn write_str(s: &str) {
    interrupt::free(|_| unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_str(s);
        }
    })
}

/// Writes formatted string to stdout
pub fn write_fmt(args: fmt::Arguments) {
    interrupt::free(|_| unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_fmt(args);
        }
    })
}

/// Macro for printing to the serial standard output
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        $crate::stdout::write_str($s)
    };
    ($($tt:tt)*) => {
        $crate::stdout::write_fmt(format_args!($($tt)*))
    };
}

/// Macro for printing to the serial standard output, with a newline.
#[macro_export]
macro_rules! sprintln {
    () => {
        $crate::stdout::write_str("\n")
    };
    ($s:expr) => {
        $crate::stdout::write_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::stdout::write_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}
