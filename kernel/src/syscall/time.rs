use core::time::Duration;

use log::{debug, info, trace};

use crate::{
    mm::user_check::UserCheck,
    process::{thread::spawn_kernel_thread, PROCESS_MANAGER},
    processor::{current_process, SumGuard},
    signal::SIGALRM,
    stack_trace,
    timer::{
        current_time_duration, current_time_ms,
        posix::current_time_spec,
        posix::TimeVal,
        posix::Tms,
        posix::{ITimerval, TimeSpec},
        timed_task::TimedTaskFuture,
        timeout_task::ksleep,
        CLOCK_MANAGER, CLOCK_REALTIME, TIMER_ABSTIME,
    },
    utils::error::{SyscallErr, SyscallRet},
};

pub fn sys_get_time(time_val_ptr: *mut TimeVal) -> SyscallRet {
    stack_trace!();
    UserCheck::new()
        .check_writable_slice(time_val_ptr as *mut u8, core::mem::size_of::<TimeVal>())?;
    let _sum_guard = SumGuard::new();
    let current_time = current_time_ms();
    let time_val = TimeVal {
        sec: current_time / 1000,
        usec: current_time % 1000 * 1000,
    };
    // debug!("get time of day, time(ms): {}", current_time);
    unsafe {
        time_val_ptr.write_volatile(time_val);
    }
    Ok(0)
}

pub fn sys_clock_settime(clock_id: usize, time_spec_ptr: *const TimeSpec) -> SyscallRet {
    stack_trace!();
    UserCheck::new()
        .check_readable_slice(time_spec_ptr as *const u8, core::mem::size_of::<TimeSpec>())?;
    let _sum_guard = SumGuard::new();
    let time_spec = unsafe { &*time_spec_ptr };
    if (time_spec.sec as isize) < 0 {
        debug!("Cannot set time. sec is negative");
        return Err(SyscallErr::EINVAL);
    } else if (time_spec.nsec as isize) < 0 || time_spec.nsec > 999999999 {
        debug!("Cannot set time. nsec is invalid");
        return Err(SyscallErr::EINVAL);
    } else if clock_id == CLOCK_REALTIME && time_spec.sec < current_time_ms() / 1000 {
        debug!("set the time to a value less than the current value of the CLOCK_MONOTONIC clock.");
        return Err(SyscallErr::EINVAL);
    }

    // calculate the diff
    // arg_timespec - device_timespec = diff
    let dev_spec = current_time_spec();
    let diff_time = Duration::from(*time_spec) - current_time_duration();
    // let diff_spec = TimeDiff {
    //     sec: time_spec.sec as isize - dev_spec.sec as isize,
    //     nsec: time_spec.nsec as isize - dev_spec.nsec as isize,
    // };
    info!(
        "[sys_clock_settime] arg time spec {:?}, dev curr time spec {:?}",
        Duration::from(*time_spec),
        Duration::from(dev_spec)
    );

    let mut manager_unlock = CLOCK_MANAGER.lock();
    manager_unlock.0.insert(clock_id, diff_time);

    Ok(0)
}

pub fn sys_clock_gettime(clock_id: usize, time_spec_ptr: *mut TimeSpec) -> SyscallRet {
    stack_trace!();
    UserCheck::new()
        .check_writable_slice(time_spec_ptr as *mut u8, core::mem::size_of::<TimeSpec>())?;
    let _sum_guard = SumGuard::new();
    let manager_locked = CLOCK_MANAGER.lock();
    let clock = manager_locked.0.get(&clock_id);
    match clock {
        Some(clock) => {
            trace!("[sys_clock_gettime] find the clock, clock id {}", clock_id);
            let dev_time = current_time_duration();
            let clock_time = dev_time + *clock;
            // let time_spec = TimeSpec {
            //     sec: (dev_spec.sec as isize + clock.sec) as usize,
            //     nsec: (dev_spec.nsec as isize + clock.nsec) as usize,
            // };
            info!("[sys_clock_gettime] get time {:?}", clock_time);
            unsafe {
                time_spec_ptr.write_volatile(clock_time.into());
            }
            Ok(0)
        }
        None => {
            trace!("[sys_clock_gettime] Cannot find the clock: {}", clock_id);
            Err(SyscallErr::EINVAL)
        }
    }
}

pub fn sys_clock_getres(clock_id: usize, res: *mut TimeSpec) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(res as *mut u8, core::mem::size_of::<TimeSpec>())?;
    let manager_locked = CLOCK_MANAGER.lock();
    let clock = manager_locked.0.get(&clock_id);
    match clock {
        Some(_clock) => {
            trace!("[sys_clock_getres] find the clock, clock id {}", clock_id);
            let resolution = Duration::from_millis(1);
            info!("[sys_clock_getres] get time {:?}", resolution);
            unsafe {
                res.write_volatile(resolution.into());
            }
            Ok(0)
        }
        None => {
            trace!("[sys_clock_getres] Cannot find the clock: {}", clock_id);
            Err(SyscallErr::EINVAL)
        }
    }
}

pub async fn sys_clock_nanosleep(
    _clock_id: usize,
    flags: u32,
    request: usize,
    remain: usize,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let size = core::mem::size_of::<TimeSpec>();
    UserCheck::new().check_readable_slice(request as *const u8, size)?;
    let request: Duration = unsafe { *(request as *const TimeSpec) }.into();
    let has_remain = if (remain as *mut TimeSpec).is_null() {
        false
    } else {
        true
    };
    let current = current_time_duration();
    if flags as usize == TIMER_ABSTIME {
        // request time is absolutely
        if request.le(&current) {
            return Ok(0);
        }
        let sleep = request - current;
        ksleep(sleep).await;
        return Ok(0);
    } else {
        // request time is relative
        ksleep(request).await;
        if has_remain {
            UserCheck::new().check_writable_slice(remain as *mut u8, size)?;
            unsafe {
                *(remain as *mut TimeSpec) = Duration::ZERO.into();
            }
        }
        return Ok(0);
    }
}

pub fn sys_times(buf: *mut Tms) -> SyscallRet {
    stack_trace!();
    UserCheck::new().check_writable_slice(buf as *mut u8, core::mem::size_of::<Tms>())?;
    let _sum_guard = SumGuard::new();
    let tms = unsafe { &mut *buf };
    // TODO: need to modify
    tms.stime = 1;
    tms.utime = 1;
    tms.cstime = 1;
    tms.cutime = 1;
    Ok(0)
}

pub async fn sys_nanosleep(time_val_ptr: usize) -> SyscallRet {
    stack_trace!();
    let sleep_ms = {
        UserCheck::new()
            .check_readable_slice(time_val_ptr as *const u8, core::mem::size_of::<TimeVal>())?;
        let _sum_guard = SumGuard::new();

        let time_val_ptr = time_val_ptr as *const TimeSpec;
        let time_val = unsafe { &(*time_val_ptr) };
        time_val.sec * 1000 + time_val.nsec / 1000000
    };
    ksleep(Duration::from_millis(sleep_ms as u64)).await;
    Ok(0)
}

const ITIMER_REAL: i32 = 0;
const ITIMER_VIRTUAL: i32 = 1;
const ITIMER_PROF: i32 = 2;

pub fn sys_setitimer(
    which: i32,
    new_value: *const ITimerval,
    old_value: *mut ITimerval,
) -> SyscallRet {
    stack_trace!();

    let current_pid = current_process().pid();

    UserCheck::new()
        .check_readable_slice(new_value as *const u8, core::mem::size_of::<ITimerval>())?;

    let _sum_guard = SumGuard::new();

    let new_value = unsafe { &*new_value };
    let interval = Duration::from(new_value.it_interval);
    let next_timeout = Duration::from(new_value.it_value);
    info!(
        "[sys_settimer]: which {}, new_value{{ interval:{:?}, value:{:?} }}",
        which, interval, next_timeout
    );

    let idx = match which {
        ITIMER_REAL => {
            let callback = move || {
                if let Some(process) = PROCESS_MANAGER.get(current_pid) {
                    let mut proc = process.inner.lock();
                    let timer = &mut proc.timers[ITIMER_REAL as usize];
                    if Duration::from(timer.it_value).is_zero() {
                        timer.it_value = Duration::ZERO.into();
                        return false;
                    } else {
                        let expired_time = current_time_duration() + interval;
                        timer.it_value = expired_time.into();
                        proc.pending_sigs.send_signal(SIGALRM)
                    }
                    if interval.is_zero() {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            };
            if next_timeout.is_zero() {
                // Disarm the old timer
                current_process().inner_handler(|proc| {
                    proc.timers[ITIMER_REAL as usize].it_value = Duration::ZERO.into();
                });
            } else {
                current_process().inner_handler(|proc| {
                    proc.timers[ITIMER_REAL as usize].it_value =
                        (current_time_duration() + next_timeout).into();
                });
                spawn_kernel_thread(async move {
                    TimedTaskFuture::new(interval, callback, next_timeout + current_time_duration())
                        .await
                });
            }
            which
        }
        ITIMER_VIRTUAL => {
            todo!()
        }
        ITIMER_PROF => {
            todo!()
        }
        _ => {
            return Err(SyscallErr::EINVAL);
        }
    };

    if old_value as usize != 0 {
        UserCheck::new()
            .check_writable_slice(old_value as *mut u8, core::mem::size_of::<ITimerval>())?;
        let old_value = unsafe { &mut *old_value };
        *old_value = current_process().inner_handler(|proc| {
            let next_trigger_ts = Duration::from(proc.timers[idx as usize].it_value);
            let mut value = next_trigger_ts;
            if !value.is_zero() {
                value -= current_time_duration();
            }
            proc.timers[idx as usize].it_value = value.into();
            proc.timers[idx as usize]
        })
    }
    Ok(0)
}
