use log::{debug, warn};

use crate::{
    mm::user_check::UserCheck,
    process::PROCESS_MANAGER,
    processor::{current_process, current_task, SumGuard},
    signal::{SigAction, SigInfo, SigSet},
    stack_trace,
    utils::error::{SyscallErr, SyscallRet},
};

pub fn sys_rt_sigaction(sig: i32, act: *const SigAction, oldact: *mut SigAction) -> SyscallRet {
    stack_trace!();
    Ok(0)
    // if sig < 0 || sig as usize >= SIG_NUM {
    //     return Err(SyscallErr::EINVAL);
    // }
    // debug!("[sys_rt_sigaction]: sig {}", sig);
    // current_process().inner_handler(|proc| {
    //     let _sum_guard = SumGuard::new();

    //     if oldact as *const u8 != core::ptr::null::<u8>() {
    //         UserCheck::new()
    //             .check_writable_slice(oldact as *mut u8, core::mem::size_of::<SigAction>())?;
    //         let sig_handler_locked = proc.sig_handler.lock();
    //         let oldact_ref = sig_handler_locked.get(sig as usize);
    //         unsafe {
    //             oldact.copy_from(&oldact_ref.unwrap().sig_action as *const SigAction, core::mem::size_of::<SigAction>());
    //         }
    //     }

    //     debug!("ra1: {:#x}, sp {:#x}", current_trap_cx().user_x[1], current_trap_cx().user_x[2]);

    //     if act as *const u8 != core::ptr::null::<u8>() {
    //         UserCheck::new()
    //             .check_readable_slice(act as *const u8, core::mem::size_of::<SigAction>())?;

    //         let new_sigaction = SigActionKernel {
    //             sig_action: unsafe { *act },
    //             is_user_defined: true,
    //         };
    //         debug!("[sys_rt_sigaction]: set new sig handler {:#x}, sa_mask {:#x}, sa_flags: {:#x}, sa_restorer: {:#x}", new_sigaction.sig_action.sa_handler as *const usize as usize, new_sigaction.sig_action.sa_mask[0], new_sigaction.sig_action.sa_flags, new_sigaction.sig_action.sa_restorer);
    //         proc.sig_handler
    //             .lock()
    //             .set_sigaction(sig as usize, new_sigaction);

    //     }
    //     Ok(0)
    // })
}

enum SigProcmaskHow {
    SigBlock = 0,
    SigUnblock = 1,
    SigSetmask = 2,
}

pub fn sys_rt_sigprocmask(how: i32, set: *const usize, old_set: *mut SigSet) -> SyscallRet {
    stack_trace!();
    current_process().inner_handler(|proc| {
        if old_set as usize != 0 {
            UserCheck::new()
                .check_writable_slice(old_set as *mut u8, core::mem::size_of::<SigSet>())?;
            let _sum_guard = SumGuard::new();
            unsafe {
                *old_set = proc.pending_sigs.blocked_sigs;
            }
        }
        if set as usize == 0 {
            debug!("arg set is null");
            return Ok(0);
        }
        UserCheck::new().check_readable_slice(set as *const u8, core::mem::size_of::<SigSet>())?;
        let _sum_guard = SumGuard::new();
        debug!("[sys_rt_sigprocmask]: how: {}", how);
        match how {
            _ if how == SigProcmaskHow::SigBlock as i32 => {
                if let Some(new_sig_mask) = unsafe { SigSet::from_bits(*set) } {
                    proc.pending_sigs.blocked_sigs |= new_sig_mask;
                    return Ok(0);
                } else {
                    warn!("invalid set arg");
                    return Err(SyscallErr::EINVAL);
                }
            }
            _ if how == SigProcmaskHow::SigUnblock as i32 => {
                if let Some(new_sig_mask) = unsafe { SigSet::from_bits(*set) } {
                    debug!("[sys_rt_sigprocmask]: new sig mask {:?}", new_sig_mask);
                    proc.pending_sigs.blocked_sigs.remove(new_sig_mask);
                    return Ok(0);
                } else {
                    warn!(
                        "[sys_rt_sigprocmask]: invalid set arg, raw sig mask {:#x}",
                        unsafe { *set }
                    );
                    return Err(SyscallErr::EINVAL);
                }
            }
            _ if how == SigProcmaskHow::SigSetmask as i32 => {
                if let Some(new_sig_mask) = unsafe { SigSet::from_bits(*set) } {
                    proc.pending_sigs.blocked_sigs = new_sig_mask;
                    return Ok(0);
                } else {
                    warn!("invalid set arg");
                    return Err(SyscallErr::EINVAL);
                }
            }
            _ => {
                warn!("invalid how");
                return Err(SyscallErr::EINVAL);
            }
        }
    })
}

pub fn sys_rt_sigreturn() -> SyscallRet {
    stack_trace!();
    let signal_context = current_task().signal_context();
    // restore the old sig mask
    current_process().inner_handler(|proc| {
        proc.pending_sigs.blocked_sigs = signal_context.blocked_sigs;
    });
    // restore the old user context
    let trap_context_mut = current_task().trap_context_mut();
    trap_context_mut.user_x = signal_context.user_context.user_x;
    trap_context_mut.sstatus = signal_context.user_context.sstatus;
    trap_context_mut.sepc = signal_context.user_context.sepc;
    Ok(0)
}

pub fn sys_kill(pid: isize, signo: i32) -> SyscallRet {
    stack_trace!();
    // TODO: add permission check for sending signal
    match pid {
        0 => {
            for (_, proc) in PROCESS_MANAGER.lock().0.iter() {
                if let Some(proc) = proc.upgrade() {
                    let sig_info = SigInfo {
                        signo: signo as usize,
                        errno: 0,
                    };
                    debug!(
                        "proc {} send signal {} to proc {}",
                        current_process().pid(),
                        signo,
                        proc.pid()
                    );
                    proc.send_signal(sig_info);
                } else {
                    continue;
                }
            }
        }
        1 => {
            for (_, proc) in PROCESS_MANAGER.lock().0.iter() {
                if let Some(proc) = proc.upgrade() {
                    if proc.pid() == 0 {
                        // init proc
                        continue;
                    }
                    let sig_info = SigInfo {
                        signo: signo as usize,
                        errno: 0,
                    };
                    debug!(
                        "proc {} send signal {} to proc {}",
                        current_process().pid(),
                        signo,
                        proc.pid()
                    );
                    proc.send_signal(sig_info);
                } else {
                    continue;
                }
            }
        }
        _ => {
            let mut pid = pid;
            if pid < 0 {
                pid = -pid;
            }
            if let Some(proc) = PROCESS_MANAGER.lock().0.get(&(pid as usize)) {
                if let Some(proc) = proc.upgrade() {
                    let sig_info = SigInfo {
                        signo: signo as usize,
                        errno: 0,
                    };
                    debug!(
                        "proc {} send signal {} to proc {}",
                        current_process().pid(),
                        signo,
                        proc.pid()
                    );
                    proc.send_signal(sig_info);
                } else {
                    // No such proc
                    return Err(SyscallErr::ESRCH);
                }
            } else {
                // No such proc
                return Err(SyscallErr::ESRCH);
            }
        }
    }
    Ok(0)
}
