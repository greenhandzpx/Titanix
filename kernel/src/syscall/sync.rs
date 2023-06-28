use core::intrinsics::atomic_load_acquire;

use log::{debug, warn};

use crate::{
    mm::user_check::UserCheck,
    process::thread::{self, TidAddress},
    processor::{current_process, current_task, SumGuard},
    stack_trace,
    sync::{mutex::SpinNoIrqLock, CondVar},
    syscall::FutexOperations,
    utils::error::{SyscallErr, SyscallRet},
};

pub async fn sys_futex(uaddr: usize, futex_op: usize, val: usize) -> SyscallRet {
    stack_trace!();
    warn!("[sys_futex]: not yet implemented!");
    match futex_op {
        _ if futex_op == FutexOperations::FutexWait as usize => {
            UserCheck::new()
                .check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
            let _sum_guard = SumGuard::new();
            if unsafe { atomic_load_acquire(uaddr as *const usize) } == val {
                current_process().inner_handler(|proc| {
                    if !proc.addr_to_condvar_map.contains_key(&uaddr) {
                        proc.addr_to_condvar_map.insert(uaddr, CondVar::new());
                    }
                    current_task().sleep();
                    let cond_var = proc.addr_to_condvar_map.get(&uaddr).unwrap();
                    cond_var.wait_without_mutex();
                });
                while current_task().is_sleep() {
                    thread::yield_now().await;
                }
            } else {
                return Err(SyscallErr::EAGAIN);
            }
        }
        _ if futex_op == FutexOperations::FutexWake as usize => {
            return futex_wake(uaddr, val);
        }
        _ => {
            panic!("Unplemented futex op")
        }
    }
    Ok(0)
}

/// Futex syscall
pub fn futex_wake(uaddr: usize, val: usize) -> SyscallRet {
    stack_trace!();
    UserCheck::new().check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
    let cnt = current_process().inner_handler(|proc| {
        let mut res = 0;
        if let Some(cond_var) = proc.addr_to_condvar_map.get(&uaddr) {
            for _ in 0..val {
                res += 1;
                cond_var.signal();
            }
        }
        res
    });
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
