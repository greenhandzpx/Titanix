//! RISC-V timer-related functionality

pub mod poll;
pub mod posix;
pub mod timed_task;

use core::time::Duration;

use crate::config::board::CLOCK_FREQ;
use crate::sbi::set_timer;
use crate::sync::mutex::SpinNoIrqLock;
use alloc::collections::BTreeMap;
use lazy_static::*;
use log::{debug, info};
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
const USEC_PER_SEC: usize = 1000000;

/// for clock_gettime
pub const CLOCK_REALTIME: usize = 0;
pub const CLOCK_MONOTONIC: usize = 1;

/// for utimensat
pub const UTIME_NOW: usize = 1073741823;
pub const UTIME_OMIT: usize = 1073741822;

/// Used for get time

/// Used for clock_gettime
/// arg_timespec - device_timespec = diff
pub struct TimeDiff {
    pub sec: isize,
    pub nsec: isize,
}

/// get current time
fn get_time() -> usize {
    time::read()
}
/// get current time in milliseconds
pub fn current_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
/// get current time in microseconds
pub fn current_time_us() -> usize {
    time::read() / (CLOCK_FREQ / USEC_PER_SEC)
}
/// get current time in `Duration`
pub fn current_time_duration() -> Duration {
    Duration::from_micros(current_time_us() as u64)
}

/// set the next timer interrupt
pub fn set_next_trigger() {
    let next_trigger = get_time() + CLOCK_FREQ / TICKS_PER_SEC;
    // debug!("next trigger {}", next_trigger);
    set_timer(next_trigger);
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
