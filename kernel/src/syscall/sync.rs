use core::{intrinsics::atomic_load_acquire, time::Duration};

use log::{debug, info};

use crate::{
    mm::user_check::UserCheck,
    process::thread,
    processor::{current_process, current_task, SumGuard},
    stack_trace,
    sync::{futex_wake, FutexFuture},
    timer::{posix::TimeSpec, timeout_task::TimeoutTaskFuture},
    utils::error::{SyscallErr, SyscallRet},
};

/// Futex Operations
enum FutexOperations {
    /// Wait
    FutexWait = 0,
    /// Wake up
    FutexWake = 1,
    ///
    FutexRequeue = 3,
    ///
    FutexCmpRequeue = 4,
    /// Private
    FutexPrivateFlag = 128,
    /// Real time
    FutexClockRealTime = 256,
}

pub async fn sys_futex(
    uaddr: usize,
    mut futex_op: u32,
    val: u32,
    timeout_ptr: usize,
    uaddr2: usize,
    val3: u32,
) -> SyscallRet {
    stack_trace!();
    futex_op &= !(FutexOperations::FutexPrivateFlag as u32);
    info!(
        "[sys_futex] uaddr {:#x}, futex_op {:#x}, val {:#x}, timeout_ptr(or val2) {:#x}, uaddr2 {:#x}, val3 {:#x}",
        uaddr, futex_op, val, timeout_ptr, uaddr2, val3
    );
    match futex_op {
        _ if futex_op == FutexOperations::FutexWait as u32 => {
            let _sum_guard = SumGuard::new();
            UserCheck::new()
                .check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
            if unsafe { atomic_load_acquire(uaddr as *const u32) } == val {
                let timeout = match timeout_ptr {
                    0 => {
                        debug!("[sys_futex]: infinite timeout");
                        None
                    }
                    _ => {
                        UserCheck::new().check_readable_slice(
                            timeout_ptr as *const u8,
                            core::mem::size_of::<TimeSpec>(),
                        )?;
                        Some(Duration::from(unsafe { *(timeout_ptr as *const TimeSpec) }))
                    }
                };
                let future = FutexFuture::new(uaddr.into(), val);
                if let Some(timeout) = timeout {
                    info!("[sys_futex]: timeout {:?}", timeout);
                    TimeoutTaskFuture::new(timeout, future).await;
                } else {
                    future.await;
                }
            } else {
                log::info!("[sys_futex] wait: val has changed, return");
                return Err(SyscallErr::EAGAIN);
            }
        }
        _ if futex_op == FutexOperations::FutexWake as u32 => {
            let ret = futex_wake(uaddr, val);
            log::info!("[sys_futex] futex wake number {:?}", ret);
            // Yield and let the waiter to fetch the lock
            thread::yield_now().await;
            return ret;
        }
        _ if futex_op == FutexOperations::FutexRequeue as u32
            || futex_op == FutexOperations::FutexCmpRequeue as u32 =>
        {
            let _sum_guard = SumGuard::new();
            UserCheck::new()
                .check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
            if futex_op == FutexOperations::FutexCmpRequeue as u32 {
                let uaddr_val = unsafe { atomic_load_acquire(uaddr as *const u32) };
                if uaddr_val == val3 {
                } else {
                    log::warn!(
                        "[sys_futex] the value isn't {:#x} anymore, is {:#x}",
                        val3,
                        uaddr_val
                    );
                    return Err(SyscallErr::EAGAIN);
                }
            }

            let val2 = timeout_ptr;
            return Ok(current_process().inner_handler(|proc| {
                proc.futex_queue
                    .requeue_waiters(uaddr.into(), uaddr2.into(), val as usize, val2)
            }) as isize);
        }
        _ => {
            panic!("Unplemented futex op, {}", futex_op)
        }
    }
    Ok(0)
}

/// Futex syscall
pub fn sys_set_tid_address(tid_ptr: usize) -> SyscallRet {
    stack_trace!();
    debug!("tid_ptr: {:#x}", tid_ptr);
    if UserCheck::new()
        .check_writable_slice(tid_ptr as *mut u8, core::mem::size_of::<usize>())
        .is_err()
    {
        return Ok(current_task().tid() as isize);
    }
    let _sum_guard = SumGuard::new();
    unsafe {
        *(tid_ptr as *mut usize) = current_task().tid();
    }
    let inner = unsafe { &mut (*current_task().inner.get()) };
    inner.tid_addr.clear_tid_address = Some(tid_ptr);
    Ok(current_task().tid() as isize)
}

pub fn sys_set_robust_list(head: usize, len: usize) -> SyscallRet {
    stack_trace!();
    log::warn!("[sys_set_robust_list]");
    Ok(0)
}

pub fn sys_get_robust_list(pid: usize, head_ptr: usize, len_ptr: usize) -> SyscallRet {
    stack_trace!();
    log::warn!("[sys_get_robust_list]");
    Ok(0)
}

pub fn sys_membarrier() -> SyscallRet {
    stack_trace!();
    Ok(0)
}
