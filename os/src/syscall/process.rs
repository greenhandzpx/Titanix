use core::f32::consts::E;

use crate::config::signal::SIG_NUM;
use crate::fs::inode_tmp::open_file;
use crate::fs::OpenFlags;
use crate::loader::get_app_data_by_name;
use crate::mm::user_check::UserCheck;
use crate::mm::{VPNRange, VirtAddr};
use crate::process::thread::{
    self, exit_and_terminate_all_threads, terminate_given_thread, Thread, TidHandle,
};
use crate::process::PROCESS_MANAGER;
use crate::processor::{current_process, current_task, local_hart, SumGuard, current_trap_cx};
use crate::sbi::shutdown;
use crate::signal::{SigAction, SigInfo, Signal, SigSet};
use crate::timer::get_time_ms;
use crate::trap::TrapContext;
use crate::utils::error::SyscallErr;
use crate::utils::error::SyscallRet;
use crate::utils::string::c_str_to_string;
use crate::{fs, process, stack_trace};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use log::{debug, info, warn};

use super::{TimeSpec, TimeVal, Tms};

// pub fn sys_exit(exit_code: i32) -> SyscallRet {
//     stack_trace!();
//     debug!("sys exit");
//     // exit_current_and_run_next(exit_code);
//     // panic!("Unreachable in sys_exit!");
//     let tid = local_hart().current_task().tid();
//     terminate_given_thread(tid, exit_code);
//     todo!("we still need to set zombie");
//     Ok(0)
// }

pub fn sys_exit(exit_code: i8) -> SyscallRet {
    stack_trace!();
    // // TODO how can we only exit one thread but still let the parent process can wait for the child
    // sys_exit_group(exit_code)
    debug!("sys exit, exit code {}, sepc {:#x}", exit_code, current_trap_cx().sepc);
    let tid = local_hart().current_task().tid();
    terminate_given_thread(tid, exit_code);
    // info!("exit finished");
    Ok(0)
}

pub fn sys_exit_group(exit_code: i8) -> SyscallRet {
    stack_trace!();
    exit_and_terminate_all_threads(exit_code);
    // current_process().set_exit_code(exit_code);
    // current_process().set_zombie();
    // todo!();
    Ok(0)
}

pub async fn sys_yield() -> SyscallRet {
    stack_trace!();
    process::yield_now().await;
    // suspend_current_and_run_next();
    Ok(0)
}

pub fn sys_get_time(time_val_ptr: *mut TimeVal) -> SyscallRet {
    stack_trace!();
    UserCheck::new()
        .check_writable_slice(time_val_ptr as *mut u8, core::mem::size_of::<TimeVal>())?;
    let _sum_guard = SumGuard::new();
    let current_time = get_time_ms();
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
    let start_ms = get_time_ms();
    let end_ms = sleep_ms + start_ms;

    loop {
        let now_ms = get_time_ms();
        if now_ms >= end_ms {
            return Ok(0);
        }
        thread::yield_now().await;
    }
}

pub fn sys_getpid() -> SyscallRet {
    stack_trace!();
    Ok(current_task().as_ref().process.pid() as isize)
}

pub fn sys_getppid() -> SyscallRet {
    stack_trace!();
    let current_process = current_process();
    let parent_process = current_process.inner_handler(move |proc| proc.parent.clone());
    match parent_process {
        Some(parent_process) => Ok(parent_process.upgrade().unwrap().pid() as isize),
        None => Ok(1),
    }
}

bitflags! {
    ///Open file flags
    pub struct CloneFlags: u32 {
        const CLONE_THREAD = 1 << 4;
        const CLONE_VM = 1 << 8;
        const CLONE_FS = 1 << 9;
        const CLONE_FILES = 1 << 10;
    }
}

// pub fn sys_clone(f: usize, _stack: *const u8, flags: i32, arg: *const u8) -> SyscallRet {
//     stack_trace!();
//     let clone_flags = CloneFlags::from_bits(flags.try_into().unwrap()).unwrap();
//     if !clone_flags.contains(CloneFlags::CLONE_THREAD) {
//         // fork

//         // TODO: maybe we should take more flags into account?

//         let current_process = current_process();
//         let new_process = current_process.fork();
//         let new_pid = new_process.pid();
//         // modify trap context of new_task, because it returns immediately after switching
//         let trap_cx = new_process.trap_context_main();
//         // we do not have to move to next instruction since we have done it before
//         // for child process, fork returns 0
//         trap_cx.user_x[10] = 0;
//         // // add new task to scheduler
//         // add_task(new_task);
//         Ok(new_pid as isize)
//     } else {
//         // clone(i.e. create a new thread)

//         debug!("clone a new thread");

//         // let f = unsafe {
//         //     core::mem::transmute::<*const (), fn(*const ())->isize>(f as *const ())
//         // };
//         let current_process = current_process();
//         Ok(current_process.create_thread(f, arg) as isize)
//     }
// }

/// TODO: consider more args
pub fn sys_clone(
    flags: usize,
    stack: *const u8,
    _ptid: *const u8,
    _tls: *const u8,
    _ctid: *const u8,
) -> SyscallRet {
    stack_trace!();

    let clone_flags = CloneFlags::from_bits(flags.try_into().unwrap());

    if clone_flags.is_none() && flags != Signal::SIGCHLD as usize {
        warn!("Invalid clone flags {}", flags);
        return Err(SyscallErr::EINVAL);
    }

    let clone_flags = {
        // TODO: This is just a workaround for preliminary test
        if flags == Signal::SIGCHLD as usize {
            CloneFlags::from_bits(0).unwrap()
        } else {
            clone_flags.unwrap()
        }
    };

    if !clone_flags.contains(CloneFlags::CLONE_THREAD) {
        // fork

        // TODO: maybe we should take more flags into account?

        let current_process = current_process();
        let stack = match stack as usize {
            0 => None,
            _ => Some(stack as usize),
        };
        let new_process = current_process.fork(stack)?;
        let new_pid = new_process.pid();
        // modify trap context of new_task, because it returns immediately after switching
        let trap_cx = new_process.trap_context_main();
        // we do not have to move to next instruction since we have done it before
        // for child process, fork returns 0
        trap_cx.user_x[10] = 0;

        let sepc = trap_cx.sepc;
        // info!("fork return, sepc: {:#x} addr: {:#x}", sepc, trap_cx as *mut TrapContext as usize);
        // // add new task to scheduler
        // add_task(new_task);
        Ok(new_pid as isize)
    } else {
        // clone(i.e. create a new thread)

        debug!("clone a new thread");

        // let f = unsafe {
        //     core::mem::transmute::<*const (), fn(*const ())->isize>(f as *const ())
        // };
        let current_process = current_process();
        current_process.create_thread(stack as usize)
    }
}

pub fn sys_execve(path: *const u8, mut args: *const usize, mut envs: *const usize) -> SyscallRet {
    stack_trace!();
    // enable kernel to visit user space
    let _sum_guard = SumGuard::new();
    // transfer the cmd args
    let mut args_vec: Vec<String> = Vec::new();
    loop {
        if unsafe { *args == 0 } {
            break;
        }
        args_vec.push(c_str_to_string(unsafe { (*args) as *const u8 }));
        debug!("exec get an arg {}", args_vec[args_vec.len() - 1]);
        unsafe {
            args = args.add(1);
        }
    }
    let mut envs_vec: Vec<String> = Vec::new();
    loop {
        if unsafe { *envs == 0 } {
            break;
        }
        envs_vec.push(c_str_to_string(unsafe { (*envs) as *const u8 }));
        debug!("exec get an env {}", envs_vec[envs_vec.len() - 1]);
        unsafe {
            envs = envs.add(1);
        }
    }
    envs_vec.push("PATH=/".to_string());
    // UserCheck::new().readable_slice(path, len);
    UserCheck::new().check_c_str(path)?;
    let path = c_str_to_string(path);
    debug!("sys exec {}", path);
    if path == "shell" {
        if let Some(elf_data) = get_app_data_by_name("shell") {
            current_process().exec(elf_data, args_vec, envs_vec)
        } else {
            warn!("[sys_exec] Cannot find this elf file {}", path);
            Err(SyscallErr::EACCES)
        }
    } else {
        if let Some(app_inode) = fs::fat32_tmp::open_file(&path, OpenFlags::RDONLY) {
            let elf_data = app_inode.read_all();
            current_process().exec(&elf_data, args_vec, envs_vec)
        } else {
            warn!("[sys_exec] Cannot find this elf file {}", path);
            Err(SyscallErr::EACCES)
        }
    }
    // if let Some(data) = get_app_data_by_name(&path) {
    //     let process = current_process();
    //     // TODO: pass the cmd args here
    //     process.exec(data, args_vec)
    // } else {
    //     Err(SyscallErr::EACCES)
    // }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub async fn sys_waitpid(pid: isize, exit_status_addr: usize) -> SyscallRet {
    stack_trace!();
    let process = current_process();

    // if exit_status_addr != 0 {
    //     UserCheck::new()
    //         .check_writable_slice(exit_status_addr as *mut u8, core::mem::size_of::<i32>())?;
    // }
    loop {
        stack_trace!();
        let (found_pid, exit_code) = process.inner_handler(move |proc| {
            // find a child process
            if !proc
                .children
                .iter()
                .any(|p| pid == -1 || pid as usize == p.pid())
            {
                if pid == -1 && proc.children.len() == 0 {
                    // system exit, since no children is alive
                    return Ok((-3, 0));
                }
                warn!(
                    "proc[{}] no such pid {} exit code addr {:#x}",
                    current_process().pid(),
                    pid,
                    exit_status_addr
                );
                return Err(SyscallErr::ECHILD);
            }

            stack_trace!();
            let idx = proc
                .children
                .iter()
                .enumerate()
                .find(|(_, p)| p.is_zombie() && (pid == -1 || pid as usize == p.pid()))
                .map(|(idx, _)| idx);
            if let Some(idx) = idx {
                stack_trace!();
                // the child has become zombie
                let child = proc.children.remove(idx);

                // After being removed, the child process may still not be destructed
                // because the child process's threads may still alive and own its reference
                // TODO: not sure whether we should exit all of its threads in advance
                // // confirm that child will be deallocated after removing from children list
                // assert_eq!(Arc::strong_count(&child), 1);
                let found_pid = child.pid();
                // get child's exit code
                let exit_code = child.exit_code();
                debug!("waitpid: found pid {} exit code {}", found_pid, exit_code);
                // info!("waitpid: found pid {} exit code {}", found_pid, exit_code);
                // if exit_status_addr != 0 {
                //     UserCheck::new()
                //         .check_writable_slice(exit_status_addr as *mut u8, core::mem::size_of::<i32>())?;
                //     let _sum_guard = SumGuard::new();
                //     let exit_status_ptr = exit_status_addr as *mut i32;
                //     // debug!("waitpid: write pid to exit_status_ptr before");
                //     info!("waitpid: write pid to exit_status_ptr before, addr {:#x}", exit_status_addr);
                //     unsafe {
                //         exit_status_ptr.write_volatile((exit_code as i32 & 0xff) << 8);
                //         // debug!(
                //         //     "waitpid: write pid to exit_code_ptr after, exit code {:#x}",
                //         //     (*exit_status_ptr & 0xff00) >> 8
                //         // );
                //         info!(
                //             "waitpid: write pid to exit_code_ptr after, exit code {:#x}",
                //             (*exit_status_ptr & 0xff00) >> 8
                //         );
                //     }
                // }
                Ok((found_pid as isize, exit_code as i32))
            } else {
                // the child still alive
                Ok((-1 as isize, 0))
            }
        })?;

        if found_pid == -1 {
            // info!("yield now");
            process::yield_now().await;
        } else if found_pid == -3 {
            // system exit
            info!("os will exit");
            exit_and_terminate_all_threads(0);
            // TODO: not sure where to invoke `shutdown`
            shutdown();
            // return Ok(ret);
        } else {
            if exit_status_addr != 0 {
                UserCheck::new()
                    .check_writable_slice(exit_status_addr as *mut u8, core::mem::size_of::<i32>())?;
                // TODO: here may cause some concurrency problem between we user_check and write it 
                let _sum_guard = SumGuard::new();
                let exit_status_ptr = exit_status_addr as *mut i32;
                debug!("waitpid: write pid to exit_status_ptr before");
                // info!("waitpid: write pid to exit_status_ptr before, addr {:#x}", exit_status_addr);
                unsafe {
                    exit_status_ptr.write_volatile((exit_code as i32 & 0xff) << 8);
                    debug!(
                        "waitpid: write pid to exit_code_ptr after, exit code {:#x}",
                        (*exit_status_ptr & 0xff00) >> 8
                    );
                    // info!(
                    //     "waitpid: write pid to exit_code_ptr after, exit code {:#x}",
                    //     (*exit_status_ptr & 0xff00) >> 8
                    // );
                }
            }
            debug!("ret {}", found_pid);
            // info!("ret {}", found_pid);
            return Ok(found_pid);
        }
    }
}

pub fn sys_rt_sigaction(sig: i32, act: *const SigAction, oldact: *mut SigAction) -> SyscallRet {
    stack_trace!();
    if sig < 0 || sig as usize >= SIG_NUM {
        return Err(SyscallErr::EINVAL);
    }
    current_process().inner_handler(|proc| {
        let _sum_guard = SumGuard::new();

        if oldact as *const u8 != core::ptr::null::<u8>() {
            UserCheck::new()
                .check_writable_slice(oldact as *mut u8, core::mem::size_of::<SigAction>())?;
            let sig_handler_locked = proc.sig_handler.lock();
            let oldact_ref = sig_handler_locked.get(sig as usize);
            unsafe {
                oldact.copy_from(oldact_ref.unwrap(), core::mem::size_of::<SigAction>());
            }
        }
        UserCheck::new()
            .check_readable_slice(act as *const u8, core::mem::size_of::<SigAction>())?;

        proc.sig_handler
            .lock()
            .set_sigaction(sig as usize, unsafe { *act });
        Ok(0)
    })
}

enum SigProcmaskHow {
    SigBlock = 0,
    SigUnblock = 1,
    SigSetmask = 2,
}

pub fn sys_rt_sigprocmask(how: i32, set: *const usize, old_set: *mut SigSet) -> SyscallRet {
    current_process().inner_handler(|proc| {
        if old_set as usize != 0 {
            UserCheck::new().check_writable_slice(old_set as *mut u8, core::mem::size_of::<SigSet>())?;
            let _sum_guard = SumGuard::new();
            unsafe {
                *old_set = proc.pending_sigs.blocked_sigs;
            }
        }
        if set as usize == 0 {
            debug!("arg set is null");
            return Ok(0)
        } 
        UserCheck::new().check_readable_slice(set as *const u8, core::mem::size_of::<SigSet>())?;
        match how {
            _ if how == SigProcmaskHow::SigBlock as i32 => {
                if let Some(new_sig_mask) = unsafe {
                    SigSet::from_bits(*set)
                } {
                    proc.pending_sigs.blocked_sigs |= new_sig_mask;
                    return Ok(0)
                } else {
                    debug!("invalid set arg");
                    return Err(SyscallErr::EINVAL);
                }
            }
            _ if how == SigProcmaskHow::SigUnblock as i32 => {
                if let Some(new_sig_mask) = unsafe {
                    SigSet::from_bits(*set)
                } {
                    proc.pending_sigs.blocked_sigs.remove(new_sig_mask);
                    return Ok(0)
                } else {
                    debug!("invalid set arg");
                    return Err(SyscallErr::EINVAL);
                }
            }
            _ if how == SigProcmaskHow::SigSetmask as i32 => {
                if let Some(new_sig_mask) = unsafe {
                    SigSet::from_bits(*set)
                } {
                    proc.pending_sigs.blocked_sigs = new_sig_mask;
                    return Ok(0)
                } else {
                    debug!("invalid set arg");
                    return Err(SyscallErr::EINVAL);
                }
            }
            _ => {
                debug!("invalid how");
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

pub fn sys_brk(addr: usize) -> SyscallRet {
    stack_trace!();
    if addr == 0 {
        return Ok(current_process()
            .inner_handler(|proc| proc.memory_set.heap_range.unwrap().end().0)
            as isize);
    }

    current_process().inner_handler(|proc| {
        let heap_start: VirtAddr = proc.memory_set.heap_range.unwrap().start();
        let current_heap_end: VirtAddr = proc.memory_set.heap_range.unwrap().end();
        let new_heap_end: VirtAddr = addr.into();
        debug!(
            "[sys_brk]: old heap end: {}, new heap end: {}",
            current_heap_end.0, new_heap_end.0
        );
        if addr > current_heap_end.0 {
            // allocate memory lazily
            if proc
                .memory_set
                .check_vpn_range_conflict(heap_start.floor(), new_heap_end.ceil())
            {
                warn!("[sys_brk]: new addr invalid");
                Err(SyscallErr::ENOMEM)
            } else {
                let heap_vma = proc
                    .memory_set
                    .find_vm_area_mut_by_vpn_included(heap_start.floor())
                    .unwrap();
                // modify vma
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                // modify process info(lazy allocation)
                proc.memory_set
                    .heap_range
                    .as_mut()
                    .unwrap()
                    .modify_right_bound(new_heap_end);
                debug!(
                    "new heap end {}",
                    proc.memory_set.heap_range.unwrap().end().0
                );
                Ok(0)
            }
        } else {
            // deallocate memory
            if addr < heap_start.0 {
                Err(SyscallErr::ENOMEM)
            } else {
                let heap_vma = proc
                    .memory_set
                    .find_vm_area_mut_by_vpn(heap_start.floor())
                    .unwrap();
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                let data_frames = unsafe { &mut (*heap_vma.data_frames.get()) };
                // modify vma
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                let page_table = unsafe { &mut (*proc.memory_set.page_table.get()) };
                let removed_vpns = VPNRange::new(new_heap_end.ceil(), current_heap_end.ceil());
                for vpn in removed_vpns {
                    if data_frames.0.contains_key(&vpn) {
                        data_frames.0.remove(&vpn);
                        page_table.unmap(vpn);
                    }
                }
                page_table.activate();
                // modify process info
                proc.memory_set
                    .heap_range
                    .unwrap()
                    .modify_right_bound(new_heap_end);
                Ok(0)
            }
        }
    })
}


pub fn sys_getuid() -> SyscallRet {
    // TODO: not sure
    Ok(0)
}