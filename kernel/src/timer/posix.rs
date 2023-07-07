use core::time::Duration;

use super::{current_time_duration, current_time_ms, MSEC_PER_SEC};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

impl From<TimeVal> for Duration {
    fn from(time_val: TimeVal) -> Self {
        Duration::new(time_val.sec as u64, (time_val.usec * 1000) as u32)
    }
}

impl From<Duration> for TimeVal {
    fn from(duration: Duration) -> Self {
        Self {
            sec: duration.as_secs() as usize,
            usec: duration.subsec_micros() as usize,
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

impl From<Duration> for TimeSpec {
    fn from(duration: Duration) -> Self {
        Self {
            sec: duration.as_secs() as usize,
            nsec: duration.subsec_nanos() as usize,
        }
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
    current_time_duration().into()
}

/// Process's timer
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct ITimerval {
    /// timer interval for periodic timer
    pub it_interval: TimeVal,
    /// time until next expiration
    pub it_value: TimeVal,
}
