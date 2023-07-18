//! RISC-V timer-related functionality

pub mod io_multiplex;
pub mod posix;
pub mod timed_task;
pub mod timeout_task;

use core::{cmp::Reverse, task::Waker, time::Duration};

use crate::config::board::CLOCK_FREQ;
use crate::sbi::set_timer;
use crate::sync::mutex::SpinNoIrqLock;
use alloc::collections::{BTreeMap, BinaryHeap};
use lazy_static::*;
use log::info;
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

/// for clock_nanosleep
pub const TIMER_ABSTIME: usize = 1;

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

/// clock stores the deviation: arg time - dev time(current_time)
pub struct ClockManager(pub BTreeMap<usize, Duration>);

lazy_static! {
    /// Clock manager that used for looking for a given process
    pub static ref CLOCK_MANAGER: SpinNoIrqLock<ClockManager> =
        SpinNoIrqLock::new(ClockManager(BTreeMap::new()));
}

pub fn init() {
    CLOCK_MANAGER
        .lock()
        .0
        .insert(CLOCK_MONOTONIC, Duration::ZERO);

    CLOCK_MANAGER
        .lock()
        .0
        .insert(CLOCK_REALTIME, Duration::ZERO);

    info!("init clock manager success");
}

pub fn handle_timeout_events() {
    // debug!("[handle_timeout_events]: start..., sepc {:#x}", sepc::read());
    let current_time = current_time_duration();
    let mut timers = TIMER_QUEUE.timers.lock();
    // TODO: should we use SleepLock instead of SpinLock? It seems that the locking time may be a little long.
    loop {
        if let Some(timer) = timers.peek() {
            log::trace!(
                "[handle_timeout_events] find a timer, current ts: {:?}, expired ts: {:?}",
                current_time,
                timer.0.expired_time
            );
            if current_time >= timer.0.expired_time {
                let mut timer = timers.pop().unwrap();
                timer.0.waker.take().unwrap().wake();
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

struct TimerQueue {
    timers: SpinNoIrqLock<BinaryHeap<Reverse<Timer>>>,
}

impl TimerQueue {
    fn add_timer(&self, timer: Timer) {
        self.timers.lock().push(Reverse(timer))
    }
}

lazy_static! {
    static ref TIMER_QUEUE: TimerQueue = TimerQueue {
        timers: SpinNoIrqLock::new(BinaryHeap::new())
    };
}

struct Timer {
    expired_time: Duration,
    waker: Option<Waker>,
    // waker: SyncUnsafeCell<Option<Waker>>,
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.expired_time == other.expired_time
    }
}

impl PartialOrd for Timer {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.expired_time < other.expired_time {
            Some(core::cmp::Ordering::Less)
        } else if self.expired_time > other.expired_time {
            Some(core::cmp::Ordering::Greater)
        } else {
            Some(core::cmp::Ordering::Equal)
        }
    }
}

impl Eq for Timer {}

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.expired_time < other.expired_time {
            core::cmp::Ordering::Less
        } else if self.expired_time > other.expired_time {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}
