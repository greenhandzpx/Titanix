use log::{debug, info, warn};

use crate::{
    config::{process::INITPROC_PID, signal::SIG_NUM},
    mm::user_check::UserCheck,
    process::{PROCESS_GROUP_MANAGER, PROCESS_MANAGER},
    processor::{current_process, current_task, SumGuard},
    signal::{ign_sig_handler, KSigAction, SigAction, SigSet, SIG_DFL, SIG_ERR, SIG_IGN},
    stack_trace,
    sync::Event,
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
    if sig < 0 || sig as usize > SIG_NUM {
        return Err(SyscallErr::EINVAL);
    }
    debug!("[sys_rt_sigaction]: sig {}", sig);

    // TODO: not sure whether we should set sigaction for all threads
    let sig_queue = current_task().sig_queue.lock();
    let _sum_guard = SumGuard::new();
    if oldact as *const u8 != core::ptr::null::<u8>() {
        UserCheck::new()
            .check_writable_slice(oldact as *mut u8, core::mem::size_of::<SigAction>())?;
        let k_sig_hand = sig_queue.sig_handlers.get(sig as usize).unwrap();
        unsafe {
            if k_sig_hand.is_user_defined {
                oldact.copy_from(&k_sig_hand.sig_action as *const SigAction, 1);
            } else {
                let mut sig_hand = k_sig_hand.sig_action;
                sig_hand.sa_handler = SIG_DFL;
                oldact.copy_from(&sig_hand as *const SigAction, 1);
            }
            log::info!(
            "[sys_rt_sigaction]: sig {}, get old sig handler {:#x}, sa_mask {:#x}, sa_flags: {:#x}",
            sig,
            (*oldact).sa_handler as *const usize as usize,
            (*oldact).sa_mask[0],
            (*oldact).sa_flags
            );
        }
    }
    drop(sig_queue);

    if act as *const u8 != core::ptr::null::<u8>() {
        UserCheck::new()
            .check_readable_slice(act as *const u8, core::mem::size_of::<SigAction>())?;

        let mut sig_action = unsafe { *act };

        let new_sigaction = match sig_action.sa_handler as usize {
            SIG_DFL => KSigAction::new(sig as usize, false),
            SIG_IGN => {
                sig_action.sa_handler = ign_sig_handler as *const () as usize;
                KSigAction {
                    sig_action,
                    is_user_defined: false,
                }
            }
            SIG_ERR => {
                todo!()
            }
            // // TODO: quite unsafe here!!!
            // _ if sig_action.sa_handler as usize & (1 << 63) > 0 => {
            //     KSigAction {
            //         sig_action,
            //         is_user_defined: false,
            //     }
            // }
            _ => KSigAction {
                sig_action,
                is_user_defined: true,
            },
        };
        log::info!(
                "[sys_rt_sigaction]: sig {}, set new sig handler {:#x}, sa_mask {:?}, sa_flags: {:#x}, sa_restorer: {:#x}",
                sig,
                new_sigaction.sig_action.sa_handler as *const usize as usize,
                new_sigaction.sig_action.sa_mask[0],
                new_sigaction.sig_action.sa_flags,
                new_sigaction.sig_action.sa_restorer,
            );
        current_process().set_sigaction(sig as usize, new_sigaction)?;
    }

    Ok(0)
}

const SIGBLOCK: i32 = 0;
const SIGUNBLOCK: i32 = 1;
const SIGSETMASK: i32 = 2;

pub fn sys_rt_sigprocmask(how: i32, set: *const u32, old_set: *mut SigSet) -> SyscallRet {
    stack_trace!();
    if old_set as usize != 0 {
        UserCheck::new()
            .check_writable_slice(old_set as *mut u8, core::mem::size_of::<SigSet>())?;
    }
    debug!("[sys_rt_sigprocmask]: how: {}", how);

    let mut sig_queue = current_task().sig_queue.lock();
    if old_set as usize != 0 {
        let _sum_guard = SumGuard::new();
        unsafe {
            *old_set = sig_queue.blocked_sigs;
            debug!("[sys_rt_sigprocmask] old set: {:?}", sig_queue.blocked_sigs);
        }
    }
    if set as usize == 0 {
        debug!("arg set is null");
        return Ok(0);
    }
    UserCheck::new().check_readable_slice(set as *const u8, core::mem::size_of::<SigSet>())?;
    let _sum_guard = SumGuard::new();
    let new_sig_mask = unsafe { SigSet::from_bits(*set as usize).ok_or(SyscallErr::EINVAL)? };
    log::info!(
        "[sys_rt_sigprocmask] how {}, new sig mask: {:?}",
        how,
        new_sig_mask
    );
    match how {
        SIGBLOCK => {
            sig_queue.blocked_sigs |= new_sig_mask;
        }
        SIGUNBLOCK => {
            sig_queue.blocked_sigs.remove(new_sig_mask);
        }
        SIGSETMASK => {
            sig_queue.blocked_sigs = new_sig_mask;
        }
        _ => {
            return Err(SyscallErr::EINVAL);
        }
    };
    Ok(0)
}

pub fn sys_rt_sigreturn() -> SyscallRet {
    stack_trace!();
    let signal_context = current_task().signal_context();
    // restore the old sig mask
    current_task().sig_queue.lock().blocked_sigs = signal_context.blocked_sigs;
    info!(
        "[sys_rt_sigreturn] blocked sigs: {:?}",
        current_task().sig_queue.lock().blocked_sigs
    );
    // restore the old user context
    let trap_context_mut = current_task().trap_context_mut();
    signal_context
        .user_context
        .restore_trap_context(trap_context_mut);

    info!(
        "[sys_rt_sigreturn] sig return, sepc {:#x}",
        trap_context_mut.sepc
    );
    Ok(trap_context_mut.user_x[10])
}

pub fn sys_rt_sigtimedwait(_set: *const u32, _info: *const u8, _timeout: *const u8) -> SyscallRet {
    stack_trace!();
    Ok(0)
}

pub fn sys_tkill(tid: usize, signo: i32) -> SyscallRet {
    stack_trace!();
    if let Some(proc) = PROCESS_MANAGER.get(tid) {
        if let Some(thread) = proc.inner_handler(|proc| {
            if let Some(thread) = proc.threads.get(&tid) {
                thread.upgrade()
            } else {
                None
            }
        }) {
            thread.recv_signal(signo as usize);
            Ok(0)
        } else {
            log::warn!("No such tid {} in pid {}", tid, proc.pid());
            Err(SyscallErr::ESRCH)
        }
    } else {
        log::warn!("no such pid for tid {}", tid);
        Err(SyscallErr::ESRCH)
    }
}

pub fn sys_tgkill(tgid: usize, tid: usize, sig: i32) -> SyscallRet {
    stack_trace!();
    warn!("[sys_tgkill]: tgid {}, tid {}, sig {}", tgid, tid, sig);
    Ok(0)
}

/// pid == 0 then sig is sent to every process in the process group of current process
/// pid == -1 then sig is sent to every process which current process has permission ( except init proc )
/// pid > 0 then sig is sent to the process with the ID specified by pid
/// pid < -1 the sig is sent to every process in process group whose ID is -pid
pub fn sys_kill(pid: isize, signo: i32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    log::info!("send signal {} to proc {}", signo, pid);
    // TODO: add permission check for sending signal
    match pid {
        0 => {
            let pid = current_process().pid();
            if let Some(proc) = PROCESS_MANAGER.get(pid) {
                let pgid = proc.pgid();
                let vec = PROCESS_GROUP_MANAGER.get_group_by_pgid(pgid);
                for id in vec {
                    debug!("[sys_kill] pid {} in pgid {}", id, pgid);
                    if id == pid {
                        continue;
                    }
                    if let Some(proc) = PROCESS_MANAGER.get(id) {
                        debug!("send signal {} to proc {} in pgid {} ", signo, id, pgid);
                        if signo != 0 {
                            proc.recv_signal(signo as usize)?;
                        }
                    } else {
                        // No such proc
                        debug!("[sys_kill] cannot find proc {}", id);
                        return Err(SyscallErr::ESRCH);
                    }
                }
            } else {
                // No such proc
                return Err(SyscallErr::ESRCH);
            }
        }
        -1 => {
            PROCESS_MANAGER.for_each(|p| {
                if p.pid() == INITPROC_PID {
                    return Ok(());
                }
                debug!(
                    "proc {} send signal {} to proc {}",
                    current_process().pid(),
                    signo,
                    p.pid()
                );
                if signo != 0 {
                    p.recv_signal(signo as usize)?;
                }
                Ok(())
            })?;
        }
        _ if pid > 0 => {
            if let Some(proc) = PROCESS_MANAGER.get(pid as usize) {
                info!(
                    "proc {} send signal {} to proc {}",
                    current_process().pid(),
                    signo,
                    proc.pid()
                );
                if signo != 0 {
                    proc.recv_signal(signo as usize)?;
                }
            } else {
                // No such proc
                return Err(SyscallErr::ESRCH);
            }
        }
        _ if pid < -1 => {
            let pid = -pid;
            let vec = PROCESS_GROUP_MANAGER.get_group_by_pgid(pid as usize);
            for id in vec {
                if let Some(proc) = PROCESS_MANAGER.get(id) {
                    debug!(
                        "[sys_kill] proc {} send signal {} to proc {}",
                        current_process().pid(),
                        signo,
                        proc.pid()
                    );
                    if signo != 0 {
                        proc.recv_signal(signo as usize)?;
                    }
                } else {
                    // No such proc
                    return Err(SyscallErr::ESRCH);
                }
            }
        }
        _ => {}
    }

    Ok(0)
}

pub fn sys_umask(_mask: u32) -> SyscallRet {
    Ok(0x777)
}

pub async fn sys_rt_sigsuspend(mask: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_readable_slice(mask as *const u8, core::mem::size_of::<SigSet>())?;
    let mask = unsafe { *(mask as *const SigSet) };
    log::info!("[sys_rt_sigsuspend] set mask: {:?}", mask);
    // retore old sigset
    let old_blocked_sigs = {
        let mut sig_queue = current_task().sig_queue.lock();
        let ret = sig_queue.blocked_sigs;
        sig_queue.blocked_sigs = mask;
        ret
    };
    current_task().wait_for_events(Event::all()).await;
    current_task().sig_queue.lock().blocked_sigs = old_blocked_sigs;
    Err(SyscallErr::EINTR)
}
