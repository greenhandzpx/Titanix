use core::time::Duration;

use crate::config::process::INITPROC_PID;
use crate::fs::{resolve_path, OpenFlags, AT_FDCWD};
use crate::loader::get_app_data_by_name;
use crate::mm::user_check::UserCheck;
use crate::mm::{VPNRange, VirtAddr};
use crate::process::thread::{exit_and_terminate_all_threads, terminate_given_thread};
use crate::processor::{current_process, current_task, current_trap_cx, local_hart, SumGuard};
use crate::sbi::shutdown;
use crate::sync::Event;
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
    debug!(
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
    debug!(
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
    ///Open file flags
    pub struct CloneFlags: u32 {
        const SIGCHLD = (1 << 4) | (1 << 0);
        const CLONE_VM = 1 << 8;
        const CLONE_FS = 1 << 9;
        const CLONE_FILES = 1 << 10;
        const CLONE_SIGHAND = 1 << 11;
        const CLONE_PTRACE = 1 << 13;
        const CLONE_VFORK = 1 << 14;
        const CLONE_PARENT = 1 << 15;
        const CLONE_THREAD = 1 << 16;
        const CLONE_PARENT_SETTID = 1 << 20;
        const CLONE_CHILD_CLEARTID = 1 << 21;
        const CLONE_UNTRACED = 1 << 23;
        const CLONE_CHILD_SETTID = 1 << 24;
        const CLONE_NEWIPC = 1 << 27;
        const CLONE_NEWPID = 1 << 29;
        const CLONE_IO = 1 << 31;
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

    if clone_flags.is_none() {
        warn!("Invalid clone flags {}", flags);
        return Err(SyscallErr::EINVAL);
    }

    let clone_flags = clone_flags.unwrap();
    // let clone_flags = {
    //     // TODO: This is just a workaround for preliminary test
    //     if flags == Signal::SIGCHLD as usize {
    //         CloneFlags::from_bits(0).unwrap()
    //     } else {
    //         clone_flags.unwrap()
    //     }
    // };

    if clone_flags.contains(CloneFlags::SIGCHLD) || !clone_flags.contains(CloneFlags::CLONE_THREAD)
    {
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

    // info!("path1 {:#x}", path as usize);
    // enable kernel to visit user space
    let _sum_guard = SumGuard::new();
    // transfer the cmd args
    let mut args_vec: Vec<String> = Vec::new();
    loop {
        if unsafe { *args == 0 } {
            break;
        }
        // TODO: add user check
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
        // TODO: add user check
        envs_vec.push(c_str_to_string(unsafe { (*envs) as *const u8 }));
        debug!("exec get an env {}", envs_vec[envs_vec.len() - 1]);
        unsafe {
            envs = envs.add(1);
        }
    }
    envs_vec.push("PATH=/".to_string());
    // UserCheck::new().readable_slice(path, len);
    UserCheck::new().check_c_str(path)?;
    // let path = c_str_to_string(path);
    let path = path::path_process(AT_FDCWD, path as *const u8).unwrap();
    debug!("sys exec {}", path);
    // print_dir_tree();

    if path == "/shell" {
        if let Some(elf_data) = get_app_data_by_name("shell") {
            current_process().exec(elf_data, args_vec, envs_vec)
        } else {
            warn!("[sys_exec] Cannot find this elf file {}", path);
            Err(SyscallErr::EACCES)
        }
    } else {
        if let Some(app_inode) = resolve_path(&path, OpenFlags::RDONLY) {
            let app_file = app_inode.open(app_inode.clone(), OpenFlags::RDONLY)?;
            trace!("try to read all data in file {}", path);
            let elf_data = app_file.sync_read_all()?;
            current_process().exec(&elf_data, args_vec, envs_vec)
            // if app_file.metadata().flags.contains(OpenFlags::CLOEXEC) {
            //     current_process().close_file(fd)
            // }
        } else {
            warn!("[sys_exec] Cannot find this elf file {}", path);
            Err(SyscallErr::EACCES)
        }
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
    info!("[sys_waitpid]: enter, pid {}, options {:#x}", pid, options);

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
                debug!("waitpid: found pid {} exit code {}", found_pid, exit_code);

                Ok(Some((false, found_pid as isize, exit_code as i32)))
            } else {
                // the child still alive
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
                        "waitpid: write pid to exit_status_ptr {:#x} before",
                        exit_status_addr
                    );
                    // info!("waitpid: write pid to exit_status_ptr before, addr {:#x}", exit_status_addr);
                    unsafe {
                        exit_status_ptr.write_volatile((exit_code as i32 & 0xff) << 8);
                        debug!(
                            "waitpid: write pid to exit_code_ptr after, exit code {:#x}",
                            (*exit_status_ptr & 0xff00) >> 8
                        );
                    }
                }
                // info!("ret {}", found_pid);
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

pub fn sys_brk(addr: usize) -> SyscallRet {
    stack_trace!();
    debug!("handle sys brk");
    if addr == 0 {
        debug!("[sys_brk]: addr: 0");
        return Ok(current_process()
            .inner_handler(|proc| proc.memory_space.heap_range.unwrap().end().0)
            as isize);
    }

    current_process().inner_handler(|proc| {
        let heap_start: VirtAddr = proc.memory_space.heap_range.unwrap().start();
        let current_heap_end: VirtAddr = proc.memory_space.heap_range.unwrap().end();
        let new_heap_end: VirtAddr = addr.into();
        debug!(
            "[sys_brk]: old heap end: {:#x}, new heap end: {:#x}",
            current_heap_end.0, new_heap_end.0
        );
        if addr > current_heap_end.0 {
            // allocate memory lazily
            if proc
                .memory_space
                .check_vpn_range_conflict(heap_start.floor(), new_heap_end.ceil())
            {
                warn!("[sys_brk]: new addr invalid");
                Err(SyscallErr::ENOMEM)
            } else {
                let heap_vma = proc
                    .memory_space
                    .find_vm_area_mut_by_vpn_included(heap_start.floor())
                    .unwrap();
                // modify vma
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                // modify process info(lazy allocation)
                proc.memory_space
                    .heap_range
                    .as_mut()
                    .unwrap()
                    .modify_right_bound(new_heap_end);
                debug!(
                    "new heap end {:#x}",
                    proc.memory_space.heap_range.unwrap().end().0
                );
                Ok(proc.memory_space.heap_range.unwrap().end().0 as isize)
            }
        } else {
            // deallocate memory
            if addr < heap_start.0 {
                Err(SyscallErr::ENOMEM)
            } else {
                let heap_vma = proc
                    .memory_space
                    .find_vm_area_mut_by_vpn(heap_start.floor())
                    .unwrap();
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                let data_frames = unsafe { &mut (*heap_vma.data_frames.get()) };
                // modify vma
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                let page_table = unsafe { &mut (*proc.memory_space.page_table.get()) };
                let removed_vpns = VPNRange::new(new_heap_end.ceil(), current_heap_end.ceil());
                for vpn in removed_vpns {
                    if data_frames.0.contains_key(&vpn) {
                        data_frames.0.remove(&vpn);
                        page_table.unmap(vpn);
                    }
                }
                page_table.activate();
                // modify process info
                proc.memory_space
                    .heap_range
                    .unwrap()
                    .modify_right_bound(new_heap_end);
                // Ok(0)
                Ok(proc.memory_space.heap_range.unwrap().end().0 as isize)
            }
        }
    })
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
            for thread in proc.threads.iter() {
                if let Some(thread) = thread.upgrade() {
                    user_time += unsafe { (*thread.inner.get()).time_info.user_time };
                    sys_time += unsafe { (*thread.inner.get()).time_info.sys_time };
                }
            }
            usage.ru_utime = user_time.into();
            usage.ru_stime = sys_time.into();
        }),
        _ => {
            panic!()
        }
    }
    debug!(
        "[sys_getrusage]: ru_utime {:?}, ru_stime {:?}",
        usage.ru_utime, usage.ru_stime
    );
    Ok(0)
}
