use core::future::Future;
use core::task::Poll;
use core::time::Duration;

use crate::config::process::INITPROC_PID;
use crate::driver::shutdown;
use crate::fs::{resolve_path, OpenFlags, AT_FDCWD};
use crate::mm::user_check::UserCheck;
use crate::process::thread::{exit_and_terminate_all_threads, terminate_given_thread};
use crate::process::{PROCESS_GROUP_MANAGER, PROCESS_MANAGER};
use crate::processor::{current_process, current_task, current_trap_cx, local_hart, SumGuard};
use crate::sync::Event;
use crate::timer::current_time_duration;
use crate::utils::async_utils::{Select2Futures, SelectOutput};
use crate::utils::error::SyscallErr;
use crate::utils::error::SyscallRet;
use crate::utils::path;
use crate::utils::string::c_str_to_string;
use crate::{process, stack_trace};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use log::{debug, info, trace, warn};

use super::TimeVal;

type Pid = usize;

pub fn sys_exit(exit_code: i8) -> SyscallRet {
    stack_trace!();
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
    Ok(0)
}

pub async fn sys_yield() -> SyscallRet {
    stack_trace!();
    process::yield_now().await;
    Ok(0)
}

pub fn sys_getpid() -> SyscallRet {
    stack_trace!();
    let ret = current_process().pid();
    debug!("[sys_getpid] return {}", ret);
    Ok(ret)
}

pub fn sys_getppid() -> SyscallRet {
    stack_trace!();
    let current_process = current_process();
    let parent_process = current_process.inner_handler(move |proc| proc.parent.clone());
    match parent_process {
        Some(parent_process) => {
            let ret = parent_process.upgrade().unwrap().pid();
            debug!("[sys_getppid] return {}", ret);
            Ok(ret)
        }
        None => Ok(INITPROC_PID),
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
pub async fn sys_clone(
    flags: usize,
    stack_ptr: usize,
    parent_tid_ptr: usize,
    tls_ptr: usize,
    chilren_tid_ptr: usize,
) -> SyscallRet {
    stack_trace!();

    let clone_flags = CloneFlags::from_bits(flags.try_into().unwrap());

    if clone_flags.is_none() {
        warn!("Invalid clone flags {}", flags);
        return Err(SyscallErr::EINVAL);
    }

    let clone_flags = clone_flags.unwrap();

    log::info!("[sys_clone] flags {:?}", clone_flags);

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
        let stack = match stack_ptr {
            0 => None,
            stack => {
                info!("[sys_clone] assign the user stack {:#x}", stack);
                // UserCheck::new().check_writable_slice(stack as *mut u8, USER_STACK_SIZE)?;
                Some(stack as usize)
            }
        };
        let new_process = current_process.fork(stack, clone_flags)?;
        let new_pid = new_process.pid();

        log::warn!(
            "[sys_clone] clone a new process, pid {}, clone flags {:?}",
            new_pid,
            clone_flags,
        );
        // thread::yield_now().await;
        Ok(new_pid)
    } else if clone_flags.contains(CloneFlags::CLONE_VM) {
        // clone(i.e. create a new thread)

        let current_process = current_process();
        let new_tid = current_process.create_thread(
            stack_ptr,
            tls_ptr,
            parent_tid_ptr,
            chilren_tid_ptr,
            clone_flags,
        );
        // process::thread::yield_now().await;
        log::info!("[sys_clone] clone a new thread, tid {:?}", new_tid);
        new_tid
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

    let mut path = path::path_process(AT_FDCWD, path as *const u8)?;
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
    } else if path.ends_with("sleep") || path.ends_with("ls") {
        path = "/busybox".to_string();
        args_vec.push("busybox".to_string());
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
    if !envs.is_null() {
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
    }

    envs_vec.push("PATH=/:/bin:/sbin:/usr/bin:/usr/local/bin:/usr/local/sbin:".to_string());
    envs_vec.push("LD_LIBRARY_PATH=/:/lib:/lib64/lp64d:/usr/lib:".to_string());

    let app_inode = resolve_path(AT_FDCWD, &path, OpenFlags::RDONLY);
    if app_inode.is_err() {
        log::warn!("[sys_execve] cannot find file {}", path);
        return Err(app_inode.err().unwrap());
    }
    let app_inode = app_inode.unwrap();
    let app_file = app_inode.open(app_inode.clone())?;
    let elf_data_arc = app_inode.metadata().inner.lock().elf_data.clone();
    let elf_data = elf_data_arc.get_unchecked_mut();
    if elf_data.is_empty() {
        app_file.read_all_from_start(elf_data)?;
    }
    // app_file.read_all_from_start(elf_data)?;
    current_process().exec(&elf_data, Some(&app_file), args_vec, envs_vec)
}

bitflags! {
    struct WaitOption: i32 {
        const WNOHANG = 1;
        const WUNTRACED = 1 << 1;
        const WCONTINUED = 1 << 3;
    }
}

struct WaitFuture {
    options: WaitOption,
    pid: Pid,
    exit_status_addr: usize,
}
impl WaitFuture {
    fn new(options: WaitOption, pid: Pid, exit_status_addr: usize) -> Self {
        Self {
            options,
            pid,
            exit_status_addr,
        }
    }
}
impl Future for WaitFuture {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let process = current_process();
        if let Some((child_pid, exit_code)) = process.inner_handler(|proc| {
            if process.pid() == INITPROC_PID && proc.children.is_empty() {
                // system exit
                info!("os will exit");
                println!("[kernel] kernel will shutdown...");
                shutdown();
            }
            if !proc
                .children
                .iter()
                .any(|p| self.pid as isize == -1 || self.pid == p.pid())
            {
                info!(
                    "[sys_wait4] proc[{}] no such pid {} exit code addr {:#x}",
                    current_process().pid(),
                    self.pid,
                    self.exit_status_addr
                );
                return Err(SyscallErr::ECHILD);
            }
            let idx = proc
                .children
                .iter()
                .enumerate()
                .find(|(_, p)| p.is_zombie() && (self.pid as isize == -1 || self.pid == p.pid()))
                .map(|(idx, _)| idx);

            if let Some(idx) = idx {
                // the child has become zombie
                let child = proc.children.remove(idx);
                let found_pid = child.pid();
                // get child's exit code
                let exit_code = child.exit_code();
                info!(
                    "[sys_wait4] found pid {} exit code {}",
                    found_pid, exit_code
                );
                Ok(Some((found_pid, exit_code as i32)))
            } else {
                debug!(
                    "[sys_wait4] no such pid, children size {}",
                    proc.children.len()
                );
                if proc.children.len() > 0 {
                    debug!("[sys_wait4] first child pid {}", proc.children[0].pid());
                }
                Ok(None)
            }
        })? {
            if self.exit_status_addr != 0 {
                UserCheck::new().check_writable_slice(
                    self.exit_status_addr as *mut u8,
                    core::mem::size_of::<i32>(),
                )?;
                // TODO: here may cause some concurrency problem between we user_check and write it
                let _sum_guard = SumGuard::new();
                let exit_status_ptr = self.exit_status_addr as *mut i32;
                debug!(
                    "[sys_wait4] write pid to exit_status_ptr {:#x} before",
                    self.exit_status_addr
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
            debug!("[sys_wait4] ret {}", child_pid);
            Poll::Ready(Ok(child_pid))
        } else {
            if self.options.contains(WaitOption::WNOHANG) {
                Poll::Ready(Ok(0))
            } else {
                current_task().register_event_waiter(Event::CHILD_EXIT, cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

pub async fn sys_wait4(pid: isize, exit_status_addr: usize, options: i32) -> SyscallRet {
    stack_trace!();
    info!("[sys_wait4]: enter, pid {}, options {:#x}", pid, options);

    let options = WaitOption::from_bits(options).ok_or(SyscallErr::EINVAL)?;

    let mut concernd_events = Event::all();
    concernd_events.remove(Event::CHILD_EXIT);
    match Select2Futures::new(
        WaitFuture::new(options, pid as usize, exit_status_addr),
        current_task().wait_for_events(concernd_events),
    )
    .await
    {
        SelectOutput::Output1(ret) => ret,
        SelectOutput::Output2(intr) => {
            log::warn!("[sys_wait4] interrupt by event {:?}", intr);
            Err(SyscallErr::EINTR)
        }
    }
}

pub fn sys_getuid() -> SyscallRet {
    stack_trace!();
    // TODO: not sure
    info!("get uid 0");
    Ok(0)
}

pub fn sys_getpgid(pid: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    if pid == 0 {
        let pgid = current_process().pgid();
        info!("get pgid, pid {}, pgid {}", pid, pgid);
        Ok(pgid)
    } else {
        let proc = PROCESS_MANAGER.get(pid);
        if proc.is_none() {
            Err(SyscallErr::ESRCH)
        } else {
            let proc = proc.unwrap();
            let pgid = proc.pgid();
            info!("get pgid, pid {}, pgid {}", pid, pgid);
            Ok(pgid)
        }
    }
}

pub fn sys_setpgid(pid: usize, pgid: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    if (pgid as isize) < 0 {
        return Err(SyscallErr::EINVAL);
    }
    info!("set pgid, pid {}, pgid {}", pid, pgid);
    if pid == 0 {
        let current_pgid = current_process().pgid();
        let new_pgid = if pgid == 0 {
            current_process().pid()
        } else {
            pgid
        };
        let pid = current_process().pid();
        PROCESS_GROUP_MANAGER.set_pgid_by_pid(pid, new_pgid, current_pgid);
        current_process().inner_handler(|proc| proc.pgid = new_pgid);
    } else {
        let proc = PROCESS_MANAGER.get(pid);
        if proc.is_none() {
            return Err(SyscallErr::ESRCH);
        } else {
            let proc = proc.unwrap();
            let current_pgid = proc.pgid();
            let new_pgid = if pgid == 0 { proc.pgid() } else { pgid };
            let pid = current_process().pid();
            PROCESS_GROUP_MANAGER.set_pgid_by_pid(pid, new_pgid, current_pgid);
            current_process().inner_handler(|proc| proc.pgid = new_pgid);
        }
    }
    Ok(0)
}

pub fn sys_geteuid() -> SyscallRet {
    stack_trace!();
    info!("get euid 0");
    // TODO
    Ok(0)
}

pub fn sys_getegid() -> SyscallRet {
    stack_trace!();
    info!("get egid, egid {}", 0);
    Ok(0)
}

pub fn sys_gettid() -> SyscallRet {
    stack_trace!();
    let tid = current_task().tid();
    Ok(tid)
}

pub fn sys_setsid() -> SyscallRet {
    // creates a new session if the calling process is not a process group leader.
    // its session ID is made the same as its process ID (if it is a leader)
    let pid = current_process().pid();
    current_process().inner_handler(|proc| {
        if pid != proc.pgid {
            debug!("[sys_setsid] current process is a child");
            // create a new session
            PROCESS_GROUP_MANAGER.set_pgid_by_pid(pid, pid, proc.pgid);
            proc.pgid = pid;
        }
    });
    Ok(pid)
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
            for (_, thread) in proc.threads.iter() {
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
