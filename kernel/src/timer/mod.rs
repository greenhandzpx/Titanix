//! RISC-V timer-related functionality

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::time::Duration;

use crate::config::board::CLOCK_FREQ;
use crate::sbi::set_timer;
use crate::sync::mutex::SpinNoIrqLock;
use alloc::collections::{BTreeMap, LinkedList};
use lazy_static::*;
use log::info;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;

/// for clock_gettime
pub const CLOCK_REALTIME: usize = 0;
pub const CLOCK_MONOTONIC: usize = 1;

/// for utimensat
pub const UTIME_NOW: usize = 1073741823;
pub const UTIME_OMIT: usize = 1073741822;

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

/// get current time
fn get_time() -> usize {
    time::read()
}
/// get current time in microseconds
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
/// get current time in `Duration`
pub fn get_time_duration() -> Duration {
    Duration::from_millis(get_time_ms() as u64)
}
/// get current time as TimeSpec
pub fn get_time_spec() -> TimeSpec {
    let current_time = get_time_ms();
    let time_spec = TimeSpec {
        sec: current_time / MSEC_PER_SEC,
        nsec: (current_time % MSEC_PER_SEC) * 1000000,
    };
    time_spec
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

pub fn handle_timeout_events() {
    let mut timers = TIMER_LIST.timers.lock();
    let current_time = get_time_duration();
    let mut timeout_cnt = 0;
    for timer in timers.iter_mut() {
        if current_time >= timer.expired_time {
            timer.waker.take().unwrap().wake();
            timeout_cnt += 1;
        }
    }
    for _ in 0..timeout_cnt {
        timers.pop_front();
    }
}

struct TimerList {
    timers: SpinNoIrqLock<LinkedList<Timer>>,
}

lazy_static! {
    static ref TIMER_LIST: TimerList = TimerList {
        timers: SpinNoIrqLock::new(LinkedList::new())
    };
}

struct Timer {
    expired_time: Duration,
    waker: Option<Waker>,
    // waker: SyncUnsafeCell<Option<Waker>>,
}

struct SleepFuture {
    expired_time: Duration,
}

impl SleepFuture {
    fn new(duration: Duration) -> Self {
        Self {
            expired_time: get_time_duration() + duration,
        }
    }
}

impl Future for SleepFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        if get_time_duration() >= this.expired_time {
            Poll::Ready(())
        } else {
            let timer = Timer {
                expired_time: this.expired_time,
                waker: Some(cx.waker().clone()),
            };
            TIMER_LIST.timers.lock().push_back(timer);
            Poll::Pending
        }
    }
}

#[allow(unused)]
pub async fn ksleep(duration: Duration) {
    SleepFuture::new(duration).await
}
