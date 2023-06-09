use crate::fs::inode::open_file;
use crate::fs::OpenFlags;
use crate::loader::get_app_data_by_name;
use crate::mm::user_check::UserCheck;
use crate::mm::{VPNRange, VirtAddr};
use crate::process::thread::{self, exit_and_terminate_all_threads, terminate_given_thread};
use crate::processor::{current_process, current_task, current_trap_cx, local_hart, SumGuard};
use crate::sbi::shutdown;
use crate::signal::Signal;
use crate::timer::{get_time_ms, TimeDiff, CLOCK_MANAGER, CLOCK_REALTIME, get_time_spec};
use crate::utils::error::SyscallErr;
use crate::utils::error::SyscallRet;
use crate::utils::string::c_str_to_string;
use crate::{fs, process, stack_trace};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use log::{debug, info, trace, warn};

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

pub fn sys_clock_settime(clock_id: usize, time_spec_ptr: *const TimeSpec) -> SyscallRet {
    stack_trace!();
    UserCheck::new()
        .check_readable_slice(time_spec_ptr as *const u8, core::mem::size_of::<TimeSpec>())?;
    let _sum_guard = SumGuard::new();
    let time_spec = unsafe { &*time_spec_ptr };
    if (time_spec.sec as isize) < 0 {
        debug!("Cannot set time. sec is negative");
        return Err(SyscallErr::EINVAL);
    } else if (time_spec.nsec as isize) < 0 || time_spec.nsec > 999999999 {
        debug!("Cannot set time. nsec is invalid");
        return Err(SyscallErr::EINVAL);
    } else if clock_id == CLOCK_REALTIME && time_spec.sec < get_time_ms() / 1000 {
        debug!("set the time to a value less than the current value of the CLOCK_MONOTONIC clock.");
        return Err(SyscallErr::EINVAL);
    }

    // calculate the diff
    // arg_timespec - device_timespec = diff
    let dev_spec = get_time_spec();
    let diff_spec = TimeDiff {
        sec: time_spec.sec as isize - dev_spec.sec as isize,
        nsec: time_spec.nsec as isize - dev_spec.nsec as isize,
    };

    let mut manager_unlock = CLOCK_MANAGER.lock();
    manager_unlock.0.insert(clock_id, diff_spec);

    Ok(0)
}

pub fn sys_clock_gettime(clock_id: usize, time_spec_ptr: *mut TimeSpec) -> SyscallRet {
    stack_trace!();
    UserCheck::new()
        .check_writable_slice(time_spec_ptr as *mut u8, core::mem::size_of::<TimeSpec>())?;
    let _sum_guard = SumGuard::new();
    let manager_unlock = CLOCK_MANAGER.lock();
    let clock = manager_unlock.0.get(&clock_id);
    match clock {
        Some(clock) => {
            debug!("Find the clock");
            let dev_spec = get_time_spec();
            let time_spec = TimeSpec {
                sec: (dev_spec.sec as isize + clock.sec) as usize,
                nsec: (dev_spec.nsec as isize + clock.nsec) as usize,
            };
            unsafe {
                time_spec_ptr.write_volatile(time_spec);
            }
            Ok(0)
        }
        None => {
            debug!("Cannot find the clock: {}", clock_id);
            Err(SyscallErr::EINVAL)
        }
    }
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
        const CLONE_CHILD_CLEARTID = 1 << 5;
        const CLONE_VM = 1 << 8;
        const CLONE_FS = 1 << 9;
        const CLONE_FILES = 1 << 10;
        const CLONE_CHILD_SETTID = 1 << 12;
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
        if let Some(app_inode) = open_file(&path, OpenFlags::RDONLY) {
            let app_file = app_inode.open(app_inode.clone(), OpenFlags::RDONLY)?;
            trace!("try to read all data in file {}", path);
            let elf_data = app_file.sync_read_all()?;
            current_process().exec(&elf_data, args_vec, envs_vec)
        } else {
            warn!("[sys_exec] Cannot find this elf file {}", path);
            Err(SyscallErr::EACCES)
        }
        // if let Some(app_inode) = fs::fat32_tmp::open_file(&path, OpenFlags::RDONLY) {
        //     let elf_data = app_inode.read_all();
        //     current_process().exec(&elf_data, args_vec, envs_vec)
        // } else {
        //     warn!("[sys_exec] Cannot find this elf file {}", path);
        //     Err(SyscallErr::EACCES)
        // }
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

pub fn sys_brk(addr: usize) -> SyscallRet {
    stack_trace!();
    debug!("handle sys brk");
    if addr == 0 {
        debug!("[sys_brk]: addr: 0");
        return Ok(current_process()
            .inner_handler(|proc| proc.memory_set.heap_range.unwrap().end().0)
            as isize);
    }

    current_process().inner_handler(|proc| {
        let heap_start: VirtAddr = proc.memory_set.heap_range.unwrap().start();
        let current_heap_end: VirtAddr = proc.memory_set.heap_range.unwrap().end();
        let new_heap_end: VirtAddr = addr.into();
        debug!(
            "[sys_brk]: old heap end: {:#x}, new heap end: {:#x}",
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
                    "new heap end {:#x}",
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
