use core::{intrinsics::atomic_load_acquire, time::Duration};

use log::{debug, info};

use crate::{
    mm::user_check::UserCheck,
    process::thread,
    processor::{current_process, current_task, SumGuard},
    stack_trace,
    sync::{futex_wake, FutexFuture},
    timer::{ffi::TimeSpec, timeout_task::TimeoutTaskFuture},
    utils::error::{SyscallErr, SyscallRet},
};

/// Futex Operations
const FUTEX_WAIT: u32 = 0;
const FUTEX_WAKE: u32 = 1;
const FUTEX_REQUEUE: u32 = 3;
const FUTEX_CMP_REQUEUE: u32 = 4;
const FUTEX_PRIVATE_FLAG: u32 = 128;
#[allow(unused)]
const FUTEX_REAL_TIME: u32 = 256;

pub async fn sys_futex(
    uaddr: usize,
    mut futex_op: u32,
    val: u32,
    timeout_ptr: usize,
    uaddr2: usize,
    val3: u32,
) -> SyscallRet {
    stack_trace!();
    futex_op &= !FUTEX_PRIVATE_FLAG;
    info!(
        "[sys_futex] uaddr {:#x}, futex_op {:#x}, val {:#x}, timeout_ptr(or val2) {:#x}, uaddr2 {:#x}, val3 {:#x}",
        uaddr, futex_op, val, timeout_ptr, uaddr2, val3
    );
    match futex_op {
        FUTEX_WAIT => {
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
        FUTEX_WAKE => {
            let ret = futex_wake(uaddr, val);
            log::info!("[sys_futex] futex wake number {:?}", ret);
            // Yield and let the waiter to fetch the lock
            thread::yield_now().await;
            return ret;
        }
        FUTEX_REQUEUE | FUTEX_CMP_REQUEUE => {
            let _sum_guard = SumGuard::new();
            UserCheck::new()
                .check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
            if futex_op == FUTEX_CMP_REQUEUE {
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

pub fn sys_set_robust_list(_head: usize, _len: usize) -> SyscallRet {
    stack_trace!();
    log::warn!("[sys_set_robust_list]");
    Ok(0)
}

pub fn sys_get_robust_list(_pid: usize, _head_ptr: usize, _len_ptr: usize) -> SyscallRet {
    stack_trace!();
    log::warn!("[sys_get_robust_list]");
    Ok(0)
}

pub fn sys_membarrier() -> SyscallRet {
    stack_trace!();
    Ok(0)
}
