use core::intrinsics::atomic_load_acquire;

use crate::{
    mm::user_check::UserCheck,
    process::thread,
    processor::{current_process, current_task, SumGuard},
    stack_trace,
    sync::{mutex::SpinNoIrqLock, CondVar},
    utils::error::{SyscallErr, SyscallRet},
};

pub enum FutexOperations {
    FutexWait = 1,
    FutexWake = 2,
}

pub async fn sys_futex(uaddr: usize, futex_op: usize, val: usize) -> SyscallRet {
    stack_trace!();
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
            UserCheck::new()
                .check_readable_slice(uaddr as *const u8, core::mem::size_of::<usize>())?;
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
        _ => {
            panic!("Unimplemented futex op")
        }
    }
    todo!()
}
