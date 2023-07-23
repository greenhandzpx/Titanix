use log::debug;

use crate::{
    mm::user_check::UserCheck,
    process::{
        resource::{CpuSet, RLimit, RLIMIT_SIZE},
        PROCESS_MANAGER,
    },
    processor::SumGuard,
    stack_trace,
    utils::error::{SyscallErr, SyscallRet},
};

pub fn sys_prlimit64(
    _pid: usize,
    resource: u32,
    new_limit: *const RLimit,
    old_limit: *mut RLimit,
) -> SyscallRet {
    stack_trace!();
    debug!("[sys_prlimit64] resource: {}", resource);
    let _sum_guard = SumGuard::new();
    if !old_limit.is_null() {
        UserCheck::new().check_writable_slice(old_limit as *mut u8, RLIMIT_SIZE)?;
        let _sum_guard = SumGuard::new();
        let old_rlimit = RLimit::get_rlimit(resource);
        debug!("[sys_prlimit64] old limit: {:?}", old_rlimit);
        unsafe {
            *old_limit = old_rlimit;
        }
    }
    if new_limit.is_null() {
        debug!("[sys_prlimit64] new limit is null");
        return Ok(0);
    }
    UserCheck::new().check_readable_slice(new_limit as *const u8, RLIMIT_SIZE)?;
    let _sum_guard = SumGuard::new();
    let new_rlimit = unsafe { &*new_limit };
    if new_rlimit.rlim_cur > new_rlimit.rlim_max {
        return Err(SyscallErr::EINVAL);
    }
    new_rlimit.set_rlimit(resource)
}

pub fn sys_sched_getaffinity(pid: usize, cpusetsize: usize, mask: usize) -> SyscallRet {
    stack_trace!();
    debug_assert_eq!(cpusetsize, core::mem::size_of::<CpuSet>());
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(mask as *mut u8, cpusetsize)?;
    if let Some(proc) = PROCESS_MANAGER.get(pid) {
        if let Some(thread) = proc.inner_handler(|proc| {
            if let Some(thread) = proc.threads.get(&pid) {
                thread.upgrade()
            } else {
                None
            }
        }) {
            unsafe {
                let set = (*(thread.inner.get())).cpu_set;
                *(mask as *mut CpuSet) = set;
            }
            Ok(0)
        } else {
            debug!(
                "[sys_sched_getaffinity] No such tid {} in pid {}",
                pid,
                proc.pid()
            );
            Err(SyscallErr::ESRCH)
        }
    } else {
        debug!("[sys_sched_getaffinity] No such process");
        Err(SyscallErr::ESRCH)
    }
}

pub fn sys_sched_setaffinity(pid: usize, cpusetsize: usize, mask: usize) -> SyscallRet {
    stack_trace!();
    debug_assert_eq!(cpusetsize, core::mem::size_of::<CpuSet>());
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_readable_slice(mask as *const u8, cpusetsize)?;
    if let Some(proc) = PROCESS_MANAGER.get(pid) {
        if let Some(thread) = proc.inner_handler(|proc| {
            if let Some(thread) = proc.threads.get(&pid) {
                thread.upgrade()
            } else {
                None
            }
        }) {
            unsafe {
                (*(thread.inner.get())).cpu_set = *(mask as *const CpuSet);
            }
            Ok(0)
        } else {
            debug!(
                "[sys_sched_setaffinity] No such tid {} in pid {}",
                pid,
                proc.pid()
            );
            Err(SyscallErr::ESRCH)
        }
    } else {
        debug!("[sys_sched_setaffinity] No such process");
        Err(SyscallErr::ESRCH)
    }
}

pub fn sys_sched_setscheduler() -> SyscallRet {
    stack_trace!();
    Ok(0)
}

pub fn sys_sched_getscheduler() -> SyscallRet {
    stack_trace!();
    Ok(0)
}

pub fn sys_sched_getparam() -> SyscallRet {
    stack_trace!();
    Ok(0)
}
