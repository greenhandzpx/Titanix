use core::time::Duration;

use log::debug;

use crate::{timer::{TimeVal, current_time_ms, TimeSpec, CLOCK_REALTIME, current_time_spec, TimeDiff, CLOCK_MANAGER, Tms, ksleep}, utils::error::{SyscallRet, SyscallErr}, processor::SumGuard, stack_trace, mm::user_check::UserCheck, process::thread};


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
    let diff_spec = TimeDiff {
        sec: time_spec.sec as isize - dev_spec.sec as isize,
        nsec: time_spec.nsec as isize - dev_spec.nsec as isize,
    };

    let mut manager_unlock = CLOCK_MANAGER.lock();
    manager_unlock.0.insert(clock_id, diff_spec);

    Ok(0)
}

pub fn sys_clock_gettime(clock_id: usize, time_spec_ptr: *mut TimeSpec) -> SyscallRet {
    stack_trace!();
    UserCheck::new()
        .check_writable_slice(time_spec_ptr as *mut u8, core::mem::size_of::<TimeSpec>())?;
    let _sum_guard = SumGuard::new();
    let manager_unlock = CLOCK_MANAGER.lock();
    let clock = manager_unlock.0.get(&clock_id);
    match clock {
        Some(clock) => {
            debug!("Find the clock");
            let dev_spec = current_time_spec();
            let time_spec = TimeSpec {
                sec: (dev_spec.sec as isize + clock.sec) as usize,
                nsec: (dev_spec.nsec as isize + clock.nsec) as usize,
            };
            unsafe {
                time_spec_ptr.write_volatile(time_spec);
            }
            Ok(0)
        }
        None => {
            debug!("Cannot find the clock: {}", clock_id);
            Err(SyscallErr::EINVAL)
        }
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
    // let start_ms = current_time_ms();
    // let end_ms = sleep_ms + start_ms;

    // loop {
    //     let now_ms = current_time_ms();
    //     if now_ms >= end_ms {
    //         return Ok(0);
    //     }
    //     thread::yield_now().await;
    // }
}