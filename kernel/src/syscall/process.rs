use core::time::Duration;

use crate::config::process::INITPROC_PID;
use crate::fs::{resolve_path, OpenFlags, AT_FDCWD};
use crate::loader::get_app_data_by_name;
use crate::mm::user_check::UserCheck;
use crate::process::thread::{exit_and_terminate_all_threads, terminate_given_thread};
use crate::processor::{current_process, current_task, current_trap_cx, local_hart, SumGuard};
use crate::sbi::shutdown;
use crate::sync::Event;
use crate::timer::current_time_duration;
use crate::utils::error::SyscallErr;
use crate::utils::error::SyscallRet;
use crate::utils::path;
use crate::utils::string::c_str_to_string;
use crate::{process, stack_trace};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use log::{debug, info, trace, warn};

use super::TimeVal;

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
    info!(
        "[sys_exit]: exit code {}, sepc {:#x}",
        exit_code,
        current_trap_cx().sepc
    );
    let tid = local_hart().current_task().tid();
    terminate_given_thread(tid, exit_code);
    // info!("exit finished");
    Ok(0)
}

pub fn sys_exit_group(exit_code: i8) -> SyscallRet {
    stack_trace!();
    info!(
        "[sys_exit_group]: exit code {}, sepc {:#x}",
        exit_code,
        current_trap_cx().sepc
    );
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
    /// Open file flags
    pub struct CloneFlags: u32 {
        ///
        const SIGCHLD = (1 << 4) | (1 << 0);
        ///
        const CLONE_VM = 1 << 8;
        ///
        const CLONE_FS = 1 << 9;
        ///
        const CLONE_FILES = 1 << 10;
        ///
        const CLONE_SIGHAND = 1 << 11;
        ///
        const CLONE_PIDFD = 1 << 12;
        ///
        const CLONE_PTRACE = 1 << 13;
        ///
        const CLONE_VFORK = 1 << 14;
        ///
        const CLONE_PARENT = 1 << 15;
        ///
        const CLONE_THREAD = 1 << 16;
        ///
        const CLONE_NEWNS = 1 << 17;
        ///
        const CLONE_SYSVSEM = 1 << 18;
        ///
        const CLONE_SETTLS = 1 << 19;
        ///
        const CLONE_PARENT_SETTID = 1 << 20;
        ///
        const CLONE_CHILD_CLEARTID = 1 << 21;
        ///
        const CLONE_DETACHED = 1 << 22;
        ///
        const CLONE_UNTRACED = 1 << 23;
        ///
        const CLONE_CHILD_SETTID = 1 << 24;
        ///
        const CLONE_NEWCGROUP = 1 << 25;
        ///
        const CLONE_NEWUTS = 1 << 26;
        ///
        const CLONE_NEWIPC = 1 << 27;
        ///
        const CLONE_NEWUSER = 1 << 28;
        ///
        const CLONE_NEWPID = 1 << 29;
        ///
        const CLONE_NEWNET = 1 << 30;
        ///
        const CLONE_IO = 1 << 31;
    }
}

/// TODO: consider more args
pub fn sys_clone(
    flags: usize,
    stack: *const u8,
    parent_tid_ptr: usize,
    tls: *const u8,
    chilren_tid_ptr: usize,
) -> SyscallRet {
    stack_trace!();

    let clone_flags = CloneFlags::from_bits(flags.try_into().unwrap());

    if clone_flags.is_none() {
        warn!("Invalid clone flags {}", flags);
        return Err(SyscallErr::EINVAL);
    }

    let clone_flags = clone_flags.unwrap();

    info!("[sys_clone] flags {:?}", clone_flags);

    if clone_flags.contains(CloneFlags::SIGCHLD) || !clone_flags.contains(CloneFlags::CLONE_VM) {
        // fork

        // TODO: maybe we should take more flags into account?
        if clone_flags.contains(CloneFlags::CLONE_CHILD_CLEARTID) {
            debug!("clone process contains CLEARTID");
        }
        if clone_flags.contains(CloneFlags::CLONE_CHILD_SETTID) {
            debug!("clone process contains SETTID");
        }

        let current_process = current_process();
        let stack = match stack as usize {
            0 => None,
            stack => {
                info!("[sys_clone] assign the user stack {:#x}", stack);
                // UserCheck::new().check_writable_slice(stack as *mut u8, USER_STACK_SIZE)?;
                Some(stack as usize)
            }
        };
        let new_process = current_process.fork(stack)?;
        let new_pid = new_process.pid();

        // // modify trap context of new_task, because it returns immediately after switching
        // let trap_cx = new_process.trap_context_main();
        // // we do not have to move to next instruction since we have done it before
        // // for child process, fork returns 0
        // trap_cx.user_x[10] = 0;

        info!(
            "[sys_clone] return new pid: {}, clone flags {:?}, child flags {:?}",
            new_pid,
            clone_flags,
            new_process.inner.lock().pending_sigs.blocked_sigs
        );
        Ok(new_pid as isize)
    } else if clone_flags.contains(CloneFlags::CLONE_VM) {
        // clone(i.e. create a new thread)

        info!("clone a new thread");

        let current_process = current_process();
        current_process.create_thread(
            stack as usize,
            tls as usize,
            parent_tid_ptr,
            chilren_tid_ptr,
            clone_flags,
        )
    } else {
        panic!()
    }
}

pub fn sys_execve(path: *const u8, mut args: *const usize, mut envs: *const usize) -> SyscallRet {
    stack_trace!();

    info!(
        "[sys_execve] enter, path ptr {:#x} ,args ptr {:#x}, envs ptr {:#x}",
        path as usize, args as usize, envs as usize
    );
    // enable kernel to visit user space
    let _sum_guard = SumGuard::new();

    UserCheck::new().check_c_str(path)?;
    let mut path = path::path_process(AT_FDCWD, path as *const u8)?.unwrap();
    info!("[sys_execve] path {}", path);

    // transfer the cmd args
    let mut args_vec: Vec<String> = Vec::new();
    // Actually, we should open the sh file and read #!...
    // find the interpreter and option args
    // but we default all the shell script should be start with "#! busybox sh"
    // So we should push /busybox and sh into the args vec
    if path.ends_with(".sh") {
        path = "/busybox".to_string();
        args_vec.push("busybox".to_string());
        args_vec.push("sh".to_string());
    }

    UserCheck::new().check_c_str(args as *const u8)?;
    loop {
        if unsafe { *args == 0 } {
            break;
        }
        //// TODO: add user check
        UserCheck::new().check_c_str(unsafe { (*args) as *const u8 })?;
        args_vec.push(c_str_to_string(unsafe { (*args) as *const u8 }));
        debug!("exec get an arg {}", args_vec[args_vec.len() - 1]);
        unsafe {
            args = args.add(1);
        }
    }

    let mut envs_vec: Vec<String> = Vec::new();
    UserCheck::new().check_c_str(envs as *const u8)?;
    loop {
        if unsafe { *envs == 0 } {
            break;
        }
        //// TODO: add user check
        UserCheck::new().check_c_str(unsafe { (*envs) as *const u8 })?;
        envs_vec.push(c_str_to_string(unsafe { (*envs) as *const u8 }));
        debug!("exec get an env {}", envs_vec[envs_vec.len() - 1]);
        unsafe {
            envs = envs.add(1);
        }
    }
    envs_vec.push("PATH=/:".to_string());

    if path.ends_with("shell") || path.ends_with("busybox") {
        if let Some(elf_data) = get_app_data_by_name(&path[1..]) {
            current_process().exec(elf_data, args_vec, envs_vec)
        } else {
            warn!("[sys_exec] Cannot find this elf file {}", path);
            Err(SyscallErr::EACCES)
        }
    } else if path.eq("/bin/true") {
        if let Some(elf_data) = get_app_data_by_name(&path[5..]) {
            current_process().exec(elf_data, args_vec, envs_vec)
        } else {
            warn!("[sys_exec] Cannot find this elf file {}", path);
            Err(SyscallErr::EACCES)
        }
    } else {
        let app_inode = resolve_path(path.as_str(), OpenFlags::RDONLY)?;
        let app_file = app_inode.open(app_inode.clone(), OpenFlags::RDONLY)?;
        let elf_data = app_file.sync_read_all()?;
        current_process().exec(&elf_data, args_vec, envs_vec)
    }
}

bitflags! {
    struct WaitOption: i32 {
        const WNOHANG = 1;
        const WUNTRACED = 1 << 1;
        const WCONTINUED = 1 << 3;
    }
}

pub async fn sys_wait4(pid: isize, exit_status_addr: usize, options: i32) -> SyscallRet {
    stack_trace!();
    let process = current_process();

    // if exit_status_addr != 0 {
    //     UserCheck::new()
    //         .check_writable_slice(exit_status_addr as *mut u8, core::mem::size_of::<i32>())?;
    // }
    info!("[sys_wait4]: enter, pid {}, options {:#x}", pid, options);

    let options = WaitOption::from_bits(options).ok_or(SyscallErr::EINVAL)?;

    loop {
        if let Some((os_exit, found_pid, exit_code)) = process.inner_handler(|proc| {
            if process.pid() == INITPROC_PID && proc.children.is_empty() {
                return Ok(Some((true, 0, 0)));
            }
            if !proc
                .children
                .iter()
                .any(|p| pid == -1 || pid as usize == p.pid())
            {
                info!(
                    "proc[{}] no such pid {} exit code addr {:#x}",
                    current_process().pid(),
                    pid,
                    exit_status_addr
                );
                return Err(SyscallErr::ECHILD);
            }
            let idx = proc
                .children
                .iter()
                .enumerate()
                .find(|(_, p)| p.is_zombie() && (pid == -1 || pid as usize == p.pid()))
                .map(|(idx, _)| idx);
            if let Some(idx) = idx {
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
                info!(
                    "[sys_wait4] found pid {} exit code {}",
                    found_pid, exit_code
                );

                Ok(Some((false, found_pid as isize, exit_code as i32)))
            } else {
                // the child still alive
                debug!(
                    "[sys_wait4] no such pid, children size {}",
                    proc.children.len()
                );
                if proc.children.len() > 0 {
                    debug!("[sys_wait4] first child pid {}", proc.children[0].pid());
                }
                // Ok((-1 as isize, 0))
                Ok(None)
            }
        })? {
            if os_exit {
                // system exit
                info!("os will exit");
                exit_and_terminate_all_threads(0);
                // TODO: not sure where to invoke `shutdown`
                shutdown();
            } else {
                if exit_status_addr != 0 {
                    UserCheck::new().check_writable_slice(
                        exit_status_addr as *mut u8,
                        core::mem::size_of::<i32>(),
                    )?;
                    // TODO: here may cause some concurrency problem between we user_check and write it
                    let _sum_guard = SumGuard::new();
                    let exit_status_ptr = exit_status_addr as *mut i32;
                    debug!(
                        "wait4: write pid to exit_status_ptr {:#x} before",
                        exit_status_addr
                    );
                    // info!("waitpid: write pid to exit_status_ptr before, addr {:#x}", exit_status_addr);
                    unsafe {
                        exit_status_ptr.write_volatile((exit_code as i32 & 0xff) << 8);
                        debug!(
                            "wait4: write pid to exit_code_ptr after, exit code {:#x}",
                            (*exit_status_ptr & 0xff00) >> 8
                        );
                    }
                }
                debug!("[sys_wait4] ret {}", found_pid);
                return Ok(found_pid);
            }
        } else {
            if options.contains(WaitOption::WNOHANG) {
                return Ok(0);
            }
            process.mailbox.wait_for_event(Event::CHILD_EXIT).await;
        }
    }
}

pub fn sys_getuid() -> SyscallRet {
    stack_trace!();
    // TODO: not sure
    info!("get uid");
    Ok(0)
}

pub fn sys_getpgid(_pid: usize) -> SyscallRet {
    stack_trace!();
    info!("get pgid, pid {}", _pid);
    // TODO
    Ok(0)
}

pub fn sys_setpgid(_pid: usize, _gid: usize) -> SyscallRet {
    stack_trace!();
    info!("set pgid, pid {}, gid {}", _pid, _gid);
    // TODO
    Ok(0)
}

pub fn sys_geteuid() -> SyscallRet {
    stack_trace!();
    info!("get euid");
    // TODO
    Ok(0)
}

pub fn sys_getegid() -> SyscallRet {
    stack_trace!();
    info!("[sys_getegid] get egid");
    Ok(0)
}

pub fn sys_gettid() -> SyscallRet {
    stack_trace!();
    let tid = current_task().tid();
    Ok(tid as isize)
}

#[repr(C)]
struct RUsage {
    /// user CPU time used
    ru_utime: TimeVal,
    /// system CPU time used
    ru_stime: TimeVal,
    /// maximum resident set size
    ru_maxrss: usize,
    /// integral shared memory size
    ru_ixrss: usize,
    /// integral unshared data size
    ru_idrss: usize,
    /// integral unshared stack size
    ru_isrss: usize,
    /// page reclaims (soft page faults)
    ru_minflt: usize,
    /// page faults (hard page faults)
    ru_majflt: usize,
    /// swaps
    ru_nswap: usize,
    /// block input operations
    ru_inblock: usize,
    /// block output operations
    ru_oublock: usize,
    /// IPC messages sent
    ru_msgsnd: usize,
    /// IPC messages received
    ru_msgrcv: usize,
    /// signals received
    ru_nsignals: usize,
    /// voluntary context switches
    ru_nvcsw: usize,
    /// involuntary context switches
    ru_nivcsw: usize,
}

const RUSAGE_SELF: i32 = 0;

pub fn sys_getrusage(who: i32, usage: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(usage as *mut u8, core::mem::size_of::<RUsage>())?;
    let usage = unsafe { &mut *(usage as *mut RUsage) };

    match who {
        RUSAGE_SELF => current_process().inner_handler(|proc| {
            let mut user_time = Duration::ZERO;
            let mut sys_time = Duration::ZERO;
            let mut start_ts = Duration::ZERO;
            for thread in proc.threads.iter() {
                if let Some(thread) = thread.upgrade() {
                    // TODO: is it ok to just read the other thread's unsafe cell data?
                    user_time += unsafe { (*thread.inner.get()).time_info.user_time };
                    sys_time += unsafe { (*thread.inner.get()).time_info.sys_time };
                    if start_ts.is_zero() {
                        start_ts = unsafe { (*thread.inner.get()).time_info.start_ts };
                    }
                }
            }
            usage.ru_utime = user_time.into();
            usage.ru_stime = sys_time.into();
            trace!(
                "[sys_getrusage]: process real time {:?}",
                current_time_duration() - start_ts
            );
        }),
        _ => {
            panic!()
        }
    }
    trace!(
        "[sys_getrusage]: ru_utime {:?}, ru_stime {:?}, current ts {:?}",
        usage.ru_utime,
        usage.ru_stime,
        current_time_duration(),
    );
    Ok(0)
}
