use core::{intrinsics::atomic_load_acquire, time::Duration};

use log::{debug, error, info};

use crate::{
    mm::user_check::UserCheck,
    process::thread::tid::TidAddress,
    processor::{current_process, current_task, SumGuard},
    stack_trace,
    sync::FutexFuture,
    timer::{posix::TimeSpec, timeout_task::TimeoutTaskFuture},
    utils::error::{SyscallErr, SyscallRet},
};

/// Futex Operations
enum FutexOperations {
    /// Wait
    FutexWait = 0,
    /// Wake up
    FutexWake = 1,
    /// Private
    FutexPrivateFlag = 128,
    /// Real time
    FutexClockRealTime = 256,
}

pub async fn sys_futex(
    uaddr: usize,
    futex_op: u32,
    val: u32,
    timeout_ptr: usize,
    uaddr2: usize,
    val3: u32,
) -> SyscallRet {
    stack_trace!();
    // todo!("[sys_futex]: not yet implemented!");
    if futex_op & FutexOperations::FutexPrivateFlag as u32 == 0 {
        error!("[sys_futex] unsupported operation");
        return Ok(0);
    }
    info!(
        "[sys_futex] uaddr {:#x}, futex_op {:#x}, val {:#x}, uaddr2 {:#x}, val3 {:#x}",
        uaddr, futex_op, val, uaddr2, val3
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
                return Err(SyscallErr::EAGAIN);
            }
        }
        _ if futex_op == FutexOperations::FutexWake as u32 => {
            return futex_wake(uaddr, val);
        }
        _ => {
            panic!("Unplemented futex op")
        }
    }
    Ok(0)
}

/// Futex syscall
pub fn futex_wake(uaddr: usize, val: u32) -> SyscallRet {
    stack_trace!();
    UserCheck::new().check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
    let cnt =
        current_process().inner_handler(|proc| proc.futex_queue.wake(uaddr.into(), val as usize));
    return Ok(cnt as isize);
}

pub fn sys_set_tid_address(tid_ptr: usize) -> SyscallRet {
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
    inner.tid_addr = Some(TidAddress { addr: tid_ptr });
    Ok(current_task().tid() as isize)
}
