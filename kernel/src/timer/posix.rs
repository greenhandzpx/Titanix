use core::time::Duration;

use super::{current_time_ms, MSEC_PER_SEC};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

impl From<Duration> for TimeVal {
    fn from(duration: Duration) -> Self {
        Self {
            sec: duration.as_secs() as usize,
            usec: duration.as_micros() as usize,
        }
    }
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
        let current_time = current_time_ms();
        Self {
            sec: current_time / 1000,
            nsec: current_time % 1000000 * 1000000,
        }
    }
}

impl From<TimeSpec> for Duration {
    fn from(time_spec: TimeSpec) -> Self {
        Duration::new(time_spec.sec as u64, time_spec.nsec as u32) 
    }
}

/// Used for times
#[repr(C)]
pub struct Tms {
    pub utime: usize,
    pub stime: usize,
    pub cutime: usize,
    pub cstime: usize,
}

/// get current time as TimeSpec
pub fn current_time_spec() -> TimeSpec {
    let current_time = current_time_ms();
    let time_spec = TimeSpec {
        sec: current_time / MSEC_PER_SEC,
        nsec: (current_time % MSEC_PER_SEC) * 1000000,
    };
    time_spec
}
