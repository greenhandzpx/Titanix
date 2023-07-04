use core::fmt;

use lazy_static::lazy_static;
use log::{self, Level, LevelFilter, Log, Metadata, Record};

use crate::{
    processor::{current_process, current_task, hart_idle_now, local_hart},
    sync::mutex::SpinNoIrqLock as Mutex,
    timer::current_time_duration,
};

lazy_static! {
    static ref LOG_LOCK: Mutex<()> = Mutex::new(());
}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        // _ => LevelFilter::Off,
        _ => LevelFilter::Error,
    });
}

/// Add escape sequence to print with color in Linux console
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

/// Print msg with color
pub fn print_in_color(args: fmt::Arguments, color_code: u8) {
    // use crate::arch::io;
    // let _guard = LOG_LOCK.lock();
    // io::putfmt(with_color!(args, color_code));
    crate::console::print(with_color!(args, color_code));
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        if hart_idle_now() {
            print_in_color(
                format_args!(
                    "[{:>5}][{}:{}][{},-,-][{:?}] {}\n",
                    record.level(),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    local_hart().hart_id(),
                    current_time_duration(),
                    record.args()
                ),
                level_to_color_code(record.level()),
            );
        } else {
            print_in_color(
                format_args!(
                    "[{:>5}][{}:{}][{},{},{}][{:?}] {}\n",
                    record.level(),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    local_hart().hart_id(),
                    current_process().pid(),
                    current_task().tid(),
                    current_time_duration(),
                    record.args()
                ),
                level_to_color_code(record.level()),
            );
        }
        //}
    }
    fn flush(&self) {}
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 36,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}

#[allow(unused)]
pub const STRACE_COLOR_CODE: u8 = 35; // Purple

/// Syscall trace
#[macro_export]
#[cfg(feature = "strace")]
macro_rules! strace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        use crate::{
            processor::{current_process, current_task, local_hart},
            timer::current_time_duration,
        };
        $crate::utils::logging::print_in_color(
            format_args!(concat!("[SYSCALL][{},{},{}][{:?}] ", $fmt, "\n"),
            local_hart().hart_id(),
            current_process().pid(),
            current_task().tid(),
            current_time_duration()
            $(, $($arg)+)?),
            crate::utils::logging::STRACE_COLOR_CODE);
    }
}
/// Syscall trace
#[macro_export]
#[cfg(not(feature = "strace"))]
macro_rules! strace {
    ($fmt: literal $(, $($arg: tt)+)?) => {};
}
