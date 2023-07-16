use log::{debug, info, warn};

use crate::{
    config::signal::SIG_NUM,
    mm::user_check::UserCheck,
    process::PROCESS_MANAGER,
    processor::{current_process, current_task, current_trap_cx, SumGuard},
    signal::{ign_sig_handler, KSigAction, SigAction, SigInfo, SigSet, SIG_DFL, SIG_ERR, SIG_IGN},
    stack_trace,
    utils::error::{SyscallErr, SyscallRet},
};

pub fn sys_rt_sigaction(sig: i32, act: *const SigAction, oldact: *mut SigAction) -> SyscallRet {
    stack_trace!();

    info!(
        "[sys_rt_sigaction]: sig {}, new act ptr {:#x}, old act ptr {:#x}, act size {}",
        sig,
        act as usize,
        oldact as usize,
        core::mem::size_of::<SigAction>()
    );
    // Ok(0)
    if sig < 0 || sig as usize > SIG_NUM {
        return Err(SyscallErr::EINVAL);
    }
    debug!("[sys_rt_sigaction]: sig {}", sig);
    current_process().inner_handler(|proc| {
        let _sum_guard = SumGuard::new();

        if oldact as *const u8 != core::ptr::null::<u8>() {
            UserCheck::new()
                .check_writable_slice(oldact as *mut u8, core::mem::size_of::<SigAction>())?;
            let oldact_ref = proc.pending_sigs.sig_handlers.get(sig as usize);
            unsafe {
                oldact.copy_from(&oldact_ref.unwrap().sig_action as *const SigAction, 1);
                debug!(
                    "[sys_rt_sigaction]: sig {}, get old sig handler {:#x}, sa_mask {:#x}, sa_flags: {:#x}",
                    sig,
                    (*oldact).sa_handler as *const usize as usize,
                    (*oldact).sa_mask[0],
                    (*oldact).sa_flags
                );
            }
        }

        debug!(
            "ra1: {:#x}, sp {:#x}",
            current_trap_cx().user_x[1],
            current_trap_cx().user_x[2]
        );

        if act as *const u8 != core::ptr::null::<u8>() {
            UserCheck::new()
                .check_readable_slice(act as *const u8, core::mem::size_of::<SigAction>())?;

            let mut sig_action = unsafe { *act };

            let new_sigaction = match sig_action.sa_handler as usize {
                SIG_DFL => {
                    KSigAction::new(sig as usize, false)
                }
                SIG_IGN => {
                    sig_action.sa_handler = ign_sig_handler;
                    KSigAction {
                        sig_action,
                        is_user_defined: false,
                    }
                }
                SIG_ERR => {
                    panic!()
                }
                // TODO: quite unsafe here!!!
                _ if sig_action.sa_handler as usize & (1 << 63) > 0 => {
                    KSigAction {
                        sig_action,
                        is_user_defined: false,
                    }
                }
                _ => {
                    KSigAction {
                        sig_action,
                        is_user_defined: true,
                    }
                }
            };
            // debug!("[sys_rt_sigaction]: set new sig handler {:#x}, sa_mask {:#x}, sa_flags: {:#x}, sa_restorer: {:#x}", new_sigaction.sig_action.sa_handler as *const usize as usize, new_sigaction.sig_action.sa_mask[0], new_sigaction.sig_action.sa_flags, new_sigaction.sig_action.sa_restorer);
            info!(
                "[sys_rt_sigaction]: sig {}, set new sig handler {:#x}, sa_mask {:#x}, sa_flags: {:#x}, sa_restorer: {:#x}",
                sig,
                new_sigaction.sig_action.sa_handler as *const usize as usize,
                new_sigaction.sig_action.sa_mask[0],
                new_sigaction.sig_action.sa_flags,
                new_sigaction.sig_action.sa_restorer,
            );
            proc.pending_sigs.sig_handlers
                .set_sigaction(sig as usize, new_sigaction);
            for (_, thread) in proc.threads.iter() {
                if let Some(thread) = thread.upgrade() {
                    unsafe {
                        thread.inner_handler(|th| {
                            th.pending_sigs.lock().sig_handlers.set_sigaction(sig as usize, new_sigaction);
                        })
                    }
                }
            }
        }
        Ok(0)
    })
}

enum SigProcmaskHow {
    SigBlock = 0,
    SigUnblock = 1,
    SigSetmask = 2,
}

pub fn sys_rt_sigprocmask(how: i32, set: *const u32, old_set: *mut SigSet) -> SyscallRet {
    stack_trace!();
    if old_set as usize != 0 {
        UserCheck::new()
            .check_writable_slice(old_set as *mut u8, core::mem::size_of::<SigSet>())?;
    }
    if set as usize == 0 {
        debug!("arg set is null");
        return Ok(0);
    }
    UserCheck::new().check_readable_slice(set as *const u8, core::mem::size_of::<SigSet>())?;
    let _sum_guard = SumGuard::new();
    debug!("[sys_rt_sigprocmask]: how: {}", how);
    current_process().inner_handler(|proc| {
        if old_set as usize != 0 {
            let _sum_guard = SumGuard::new();
            unsafe {
                *old_set = proc.pending_sigs.blocked_sigs;
                debug!(
                    "[sys_rt_sigprocmask] old set: {:?}",
                    proc.pending_sigs.blocked_sigs
                );
            }
        }
        match how {
            _ if how == SigProcmaskHow::SigBlock as i32 => {
                stack_trace!();
                if let Some(new_sig_mask) = unsafe { SigSet::from_bits(*set as usize) } {
                    debug!("[sys_rt_sigprocmask] new sig mask: {:?}", new_sig_mask);
                    proc.pending_sigs.blocked_sigs |= new_sig_mask;
                    unsafe {
                        current_task().inner_handler(|th| {
                            th.pending_sigs.lock().blocked_sigs |= new_sig_mask;
                        });
                    }
                    return Ok(0);
                } else {
                    info!("invalid set arg");
                    return Err(SyscallErr::EINVAL);
                }
            }
            _ if how == SigProcmaskHow::SigUnblock as i32 => {
                if let Some(new_sig_mask) = unsafe { SigSet::from_bits(*set as usize) } {
                    info!("[sys_rt_sigprocmask]: new sig mask {:?}", new_sig_mask);
                    proc.pending_sigs.blocked_sigs.remove(new_sig_mask);
                    unsafe {
                        current_task().inner_handler(|th| {
                            th.pending_sigs.lock().blocked_sigs.remove(new_sig_mask);
                        });
                    }
                    return Ok(0);
                } else {
                    info!(
                        "[sys_rt_sigprocmask]: invalid set arg, raw sig mask {:#x}",
                        unsafe { *set }
                    );
                    return Err(SyscallErr::EINVAL);
                }
            }
            _ if how == SigProcmaskHow::SigSetmask as i32 => {
                if let Some(new_sig_mask) = unsafe { SigSet::from_bits(*set as usize) } {
                    debug!("[sys_rt_sigprocmask] new sig mask: {:?}", new_sig_mask);
                    proc.pending_sigs.blocked_sigs = new_sig_mask;
                    unsafe {
                        current_task().inner_handler(|th| {
                            th.pending_sigs.lock().blocked_sigs = new_sig_mask;
                        });
                    }
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
        info!(
            "[sys_rt_sigreturn] blocked sigs: {:?}",
            proc.pending_sigs.blocked_sigs
        );
    });
    // restore the old user context
    let trap_context_mut = current_task().trap_context_mut();
    trap_context_mut.user_x = signal_context.user_context.user_x;
    trap_context_mut.user_x[0] = 0;
    trap_context_mut.sstatus = signal_context.user_context.sstatus;
    // trap_context_mut.sepc = signal_context.user_context.sepc;
    trap_context_mut.sepc = signal_context.user_context.user_x[0];
    info!(
        "[sys_rt_sigreturn] sig return, sepc {:#x}",
        trap_context_mut.sepc
    );
    // info!(
    //     "[sys_rt_sigreturn] sig return, user x {:?}",
    //     signal_context.user_context.user_x
    // );
    // info!(
    //     "[sys_rt_sigreturn] sig return, dummy {:?}",
    //     signal_context.blocked_sigs_dummy.dummy
    // );
    Ok(trap_context_mut.user_x[10] as isize)
}

pub fn sys_rt_sigtimedwait(_set: *const u32, _info: *const u8, _timeout: *const u8) -> SyscallRet {
    stack_trace!();
    Ok(0)
}

pub fn sys_tkill(tid: usize, signo: i32) -> SyscallRet {
    stack_trace!();
    if let Some(proc) = PROCESS_MANAGER.get_process_by_tid(tid) {
        if let Some(thread) = proc.inner_handler(|proc| {
            if let Some(thread) = proc.threads.get(&tid) {
                thread.upgrade()
            } else {
                None
            }
        }) {
            let sig_info = SigInfo {
                signo: signo as usize,
                errno: 0,
            };
            thread.send_signal(sig_info);
            Ok(0)
        } else {
            Err(SyscallErr::ESRCH)
        }
    } else {
        Err(SyscallErr::ESRCH)
    }
}

pub fn sys_tgkill(tgid: usize, tid: usize, sig: i32) -> SyscallRet {
    stack_trace!();
    warn!("[sys_tgkill]: tgid {}, tid {}, sig {}", tgid, tid, sig);
    Ok(0)
}

pub fn sys_kill(pid: isize, signo: i32) -> SyscallRet {
    stack_trace!();
    // TODO: add permission check for sending signal
    match pid {
        0 => {
            for (_, proc) in PROCESS_MANAGER.0.lock().iter() {
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
            for (_, proc) in PROCESS_MANAGER.0.lock().iter() {
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
            if let Some(proc) = PROCESS_MANAGER.get_process_by_pid(pid as usize) {
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
        }
    }
    Ok(0)
}
