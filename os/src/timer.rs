//! RISC-V timer-related functionality

use crate::config::board::CLOCK_FREQ;
use crate::sbi::set_timer;
use crate::sync::mutex::SpinNoIrqLock;
use alloc::collections::BTreeMap;
use lazy_static::*;
use log::info;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;

/// for clock_gettime
pub const CLOCK_REALTIME: usize = 0;
pub const CLOCK_MONOTONIC: usize = 1;

/// Used for get time
#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Used for nanosleep
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TimeSpec {
    pub sec: usize,
    pub nsec: usize,
}

impl TimeSpec {
    pub fn new() -> Self {
        // new a time spec with machine time
        let current_time = get_time_ms();
        Self {
            sec: current_time / 1000,
            nsec: current_time % 1000000 * 1000000,
        }
    }
}

/// Used for clock_gettime
/// arg_timespec - device_timespec = diff
pub struct TimeDiff {
    pub sec: isize,
    pub nsec: isize,
}

/// Used for times
#[repr(C)]
pub struct Tms {
    pub utime: usize,
    pub stime: usize,
    pub cutime: usize,
    pub cstime: usize,
}

///get current time
fn get_time() -> usize {
    time::read()
}
/// get current time in microseconds
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
/// set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

pub struct ClockManager(pub BTreeMap<usize, TimeDiff>);

lazy_static! {
    /// Clock manager that used for looking for a given process
    pub static ref CLOCK_MANAGER: SpinNoIrqLock<ClockManager> =
        SpinNoIrqLock::new(ClockManager(BTreeMap::new()));
}

pub fn init() {
    CLOCK_MANAGER
        .lock()
        .0
        .insert(CLOCK_MONOTONIC, TimeDiff { sec: 0, nsec: 0 });

    CLOCK_MANAGER
        .lock()
        .0
        .insert(CLOCK_REALTIME, TimeDiff { sec: 0, nsec: 0 });

    info!("init clock manager success");
}
