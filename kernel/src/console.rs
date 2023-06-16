//! SBI console driver, for text output
use crate::{sbi::console_putchar, sync::mutex::{SpinNoIrqLock, SleepLock}};
use core::fmt::{self, Write};
use lazy_static::*;

struct Stdout;

const PRINT_LOCKED: bool = true;

lazy_static!{
    static ref PRINT_MUTEX: SpinNoIrqLock<()> = SpinNoIrqLock::new(());

}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments<'_>) {
    if PRINT_LOCKED {
        let _locked = PRINT_MUTEX.lock();
        Stdout.write_fmt(args).unwrap();
    } else {
        Stdout.write_fmt(args).unwrap();
    }
}

#[macro_export]
/// print string macro
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
/// println string macro
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
