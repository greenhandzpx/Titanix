//! A single global instance of [`PidAllocator`] called `PID_ALLOCATOR` allocates
//! pid for user apps.
//!

///
pub mod thread;

use log::{debug, info, warn};
pub use thread::yield_now;

/// Aux header
pub mod aux;
mod manager;
/// System resource
pub mod resource;
// #[allow(clippy::module_inception)]
// mod task;

use crate::{
    config::process::CLONE_STACK_SIZE,
    fs::{FdTable, File},
    loader::get_app_data_by_name,
    mm::{user_check::UserCheck, MemorySpace},
    net::SocketTable,
    process::{
        aux::{AuxHeader, AT_EXECFN, AT_NULL, AT_RANDOM},
        thread::{terminate_all_threads_except_main, tid::tid_alloc},
    },
    processor::{current_process, current_task, current_trap_cx, hart::local_hart, SumGuard},
    signal::{KSigAction, Signal},
    stack_trace,
    sync::{mutex::SpinNoIrqLock, FutexQueue},
    syscall::CloneFlags,
    timer::ffi::ITimerval,
    trap::TrapContext,
    utils::error::{GeneralRet, SyscallErr, SyscallRet},
};
use alloc::{
    collections::BTreeMap,
    string::String,
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};
use thread::Thread;

pub use manager::PROCESS_GROUP_MANAGER;
pub use manager::PROCESS_MANAGER;

///Add init process to the manager
pub fn add_initproc() {
    stack_trace!();

    #[cfg(feature = "submit")]
    let elf_data = get_app_data_by_name("runtestcase").unwrap();

    #[cfg(not(feature = "submit"))]
    let elf_data = get_app_data_by_name("initproc").unwrap();

    let _init_proc = Process::new_initproc(elf_data, None);
    // PROCESS_MANAGER.add_process(_init_proc.pid(), &_init_proc);
}

use self::{resource::RLimit, thread::tid::TidHandle};

/// Process control block inner
pub struct ProcessInner {
    /// Whether this process is a zombie process
    pub is_zombie: bool,
    /// The process's address space
    pub memory_space: MemorySpace,
    /// Parent process
    pub parent: Option<Weak<Process>>,
    /// Children processes
    pub children: Vec<Arc<Process>>,
    // pub children: Vec<Weak<Process>>,
    /// File descriptor table
    pub fd_table: FdTable,
    /// Socket table
    pub socket_table: SocketTable,
    /// TODO: use BTreeMap to query and delete more quickly
    pub threads: BTreeMap<usize, Weak<Thread>>,
    /// Futex queue
    pub futex_queue: FutexQueue,
    /// Exit code of the current process
    /// Note that we may need to put this member in every thread
    pub exit_code: i8,
    /// Current Work Directory
    pub cwd: String,
    /// REAL, VIRTUAL, PROF timer
    pub timers: [ITimerval; 3],
    /// Process Resource
    pub rlimit: RLimit,
    /// gid, the process group id
    pub pgid: usize,
    /// pselect times
    #[cfg(not(feature = "multi_hart"))]
    pub pselect_times: u8,
}

impl ProcessInner {
    ///
    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }
}
/// Process control block
pub struct Process {
    /// pid, i.e. the leader thread's tid
    pid: Arc<TidHandle>,
    // /// mailbox,
    // pub mailbox: Mailbox,
    /// mutable
    pub inner: SpinNoIrqLock<ProcessInner>,
}

impl Process {
    /// Main thread
    pub fn main_thread(&self) -> Option<Weak<Thread>> {
        self.inner.lock().threads.get(&self.pid()).cloned()
    }

    /// Main thread's trap context
    pub fn trap_context_main(&self) -> &mut TrapContext {
        let inner = self.inner.lock();
        assert!(inner.thread_count() > 0);
        unsafe { (*inner.threads.get(&self.pid.0).unwrap().as_ptr()).trap_context_mut() }
    }

    /// Get the process's pid
    pub fn pid(&self) -> usize {
        self.pid.0
    }

    /// Get the process's gid
    pub fn pgid(&self) -> usize {
        self.inner.lock().pgid
    }

    /// We can get whatever we want in the inner by providing a handler
    pub fn inner_handler<T>(&self, f: impl FnOnce(&mut ProcessInner) -> T) -> T {
        f(&mut self.inner.lock())
    }

    /// True when all threads have exited
    pub fn is_zombie(&self) -> bool {
        self.inner.lock().is_zombie
    }

    ///
    pub fn set_zombie(&self) {
        self.inner.lock().is_zombie = true;
    }

    ///
    pub fn exit_code(&self) -> i8 {
        self.inner.lock().exit_code
    }

    ///
    pub fn set_exit_code(&self, exit_code: i8) {
        self.inner.lock().exit_code = exit_code;
    }

    ///
    pub fn alloc_fd(&self) -> GeneralRet<usize> {
        self.inner.lock().fd_table.alloc_fd()
    }

    /// Send signal to this process
    pub fn recv_signal(&self, signo: Signal) -> GeneralRet<()> {
        stack_trace!();
        if signo == 0 {
            return Err(SyscallErr::EINVAL);
        }
        log::info!(
            "[Process:recv_signal] proc {} recv signo {}",
            self.pid(),
            signo
        );
        self.inner_handler(|proc| {
            for (_, thread) in proc.threads.iter() {
                if let Some(thread) = thread.upgrade() {
                    thread.recv_signal(signo)
                }
            }
        });
        Ok(())
    }

    /// Set sigaction for all threads in this process
    pub fn set_sigaction(&self, signo: Signal, sigaction: KSigAction) -> GeneralRet<()> {
        self.inner_handler(|proc| {
            for (_, thread) in proc.threads.iter() {
                if let Some(thread) = thread.upgrade() {
                    thread
                        .sig_queue
                        .lock()
                        .sig_handlers
                        .set_sigaction(signo, sigaction)
                }
            }
        });
        Ok(())
    }

    /// Close file
    pub fn close_file(&self, fd: usize) -> SyscallRet {
        let mut inner = self.inner.lock();
        if inner.fd_table.take(fd).is_none() {
            Err(SyscallErr::EBADF)
        } else {
            debug!("close fd {}", fd);
            Ok(0)
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let inner = self.inner.lock();
        for (fd, file) in inner.fd_table.fd_table.iter().enumerate() {
            if file.is_some() {
                log::info!(
                    "[Process::drop] drop fd {}, file ref cnt {}",
                    fd,
                    Arc::strong_count(&file.as_ref().unwrap().file)
                );
            }
        }
        log::info!("process {} died!", self.pid());
    }
}

impl Process {
    /// Create a new process
    pub fn new_initproc(elf_data: &[u8], elf_file: Option<&Arc<dyn File>>) -> Arc<Self> {
        let (memory_space, user_sp_top, entry_point, _auxv) =
            MemorySpace::from_elf(elf_data, elf_file);

        // Alloc a pid
        let pid = Arc::new(tid_alloc());
        let process = Arc::new(Self {
            pid: pid.clone(),
            inner: SpinNoIrqLock::new(ProcessInner {
                is_zombie: false,
                memory_space,
                parent: None,
                children: Vec::new(),
                fd_table: FdTable::new(),
                socket_table: SocketTable::new(),
                threads: BTreeMap::new(),
                // sig_queue: SigQueue::new(),
                futex_queue: FutexQueue::new(),
                exit_code: 0,
                cwd: String::from("/"),
                timers: [ITimerval::default(); 3],
                rlimit: RLimit::new(0, 0),
                pgid: pid.0,
                #[cfg(not(feature = "multi_hart"))]
                pselect_times: 0,
            }),
        });
        let trap_context = TrapContext::app_init_context(entry_point, user_sp_top);
        // create a main thread
        let thread = Arc::new(Thread::new(
            process.clone(),
            None,
            trap_context,
            user_sp_top,
            Some(pid),
        ));

        process
            .inner
            .lock()
            .threads
            .insert(thread.tid(), Arc::downgrade(&thread));
        PROCESS_MANAGER.add(process.pid(), &process);
        PROCESS_GROUP_MANAGER.add_group(process.pgid());
        // Add the main thread into scheduler
        thread::spawn_thread(thread);
        debug!("create a new process, pid {}", process.pid());
        process
    }

    /// Fork a new process
    /// `stack` points to the new cloned process's main thread's stack if not `None`
    pub fn fork(
        self: &Arc<Self>,
        stack: Option<usize>,
        flags: CloneFlags,
    ) -> GeneralRet<Arc<Self>> {
        self.clone_process(stack, flags)
    }

    /// Exec a new program.
    /// Note that the return value is `argc`.
    /// When one process invokes `exec`, all of the threads will terminate except the
    /// main thread, and the new program is executed in the main thread.
    pub fn exec(
        &self,
        elf_data: &[u8],
        elf_file: Option<&Arc<dyn File>>,
        args: Vec<String>,
        envs: Vec<String>,
    ) -> SyscallRet {
        stack_trace!();
        log::debug!("[Process::exec] pid {}", current_process().pid());
        log::info!("[Process::exec] elf name {}", args[0]);

        // memory_space with elf program headers/trampoline/trap context/user stack
        // substitute memory_space
        let (memory_space, ustack_top, entry_point, mut auxs) =
            MemorySpace::from_elf(elf_data, elf_file);
        let main_thread = self.inner_handler(|proc| {
            if proc.thread_count() > 1 {
                warn!("[Process:exec] thread count > 1: {}", proc.thread_count());
            }
            // Change hart local context's pagetable (quite important!!!)
            memory_space.activate();
            let hart = local_hart();
            hart.change_page_table(memory_space.page_table.clone());
            // process_inner.memory_space = memory_space;
            proc.threads.get(&self.pid()).unwrap().upgrade().unwrap()
        });
        stack_trace!();

        terminate_all_threads_except_main();

        // TODO: not sure whether we should dealloc ustack here?

        stack_trace!();
        self.inner_handler(|proc| {
            // proc.ustack_base = ustack_base;
            proc.memory_space = memory_space;
            proc.fd_table.close_on_exec();
        });

        let main_thread_inner = unsafe { &mut (*main_thread.inner.get()) };
        main_thread_inner.ustack_top = ustack_top;

        // // alloc new ustack
        // main_thread.alloc_ustack();
        stack_trace!();

        // ----- The following to to push arguments on user stack -----
        let mut user_sp = main_thread_inner.ustack_top;

        // Enable kernel to visit user space
        let _sum_guard = SumGuard::new();
        debug!("exec args len {}", args.len());

        // argv is a vector of each arg's addr
        let mut argv = vec![0; args.len()];
        // envp is a vector of each env's addr
        let mut envp = vec![0; envs.len()];

        // Copy each env to the newly allocated stack
        for i in 0..envs.len() {
            // Here we leave one byte to store a '\0' as a terminator
            user_sp -= envs[i].len() + 1;
            UserCheck::new().check_writable_slice(user_sp as *mut u8, envs[i].len() + 1)?;
            let p = user_sp as *mut u8;
            unsafe {
                envp[i] = user_sp;
                p.copy_from(envs[i].as_ptr(), envs[i].len());
                *((p as usize + envs[i].len()) as *mut u8) = 0;
            }
        }
        user_sp -= user_sp % core::mem::size_of::<usize>();

        stack_trace!();
        // Copy each arg to the newly allocated stack
        for i in 0..args.len() {
            // Here we leave one byte to store a '\0' as a terminator
            user_sp -= args[i].len() + 1;
            UserCheck::new().check_writable_slice(user_sp as *mut u8, args[i].len() + 1)?;
            let p = user_sp as *mut u8;
            unsafe {
                argv[i] = user_sp;
                p.copy_from(args[i].as_ptr(), args[i].len());
                *((p as usize + args[i].len()) as *mut u8) = 0;
            }
        }
        user_sp -= user_sp % core::mem::size_of::<usize>();

        stack_trace!();

        // Copy `platform`
        let platform = "RISC-V64";
        user_sp -= platform.len() + 1;
        user_sp -= user_sp % core::mem::size_of::<usize>();
        let p = user_sp as *mut u8;
        UserCheck::new().check_writable_slice(p as *mut u8, platform.len())?;
        unsafe {
            p.copy_from(platform.as_ptr(), platform.len());
            *((p as usize + platform.len()) as *mut u8) = 0;
        }

        stack_trace!();
        // Copy 16 random bytes(here is 0)
        user_sp -= 16;
        UserCheck::new().check_writable_slice(user_sp as *mut u8, 16)?;
        auxs.push(AuxHeader {
            aux_type: AT_RANDOM,
            value: user_sp,
        });

        stack_trace!();
        // Padding
        user_sp -= user_sp % 16;

        auxs.push(AuxHeader {
            aux_type: AT_EXECFN,
            value: argv[0],
        }); // file name
        auxs.push(AuxHeader {
            aux_type: AT_NULL,
            value: 0,
        }); // end

        stack_trace!();
        // Construct auxv
        debug!("auxv len {}", auxs.len());
        let len = auxs.len() * core::mem::size_of::<AuxHeader>();
        user_sp -= len;
        UserCheck::new().check_writable_slice(user_sp as *mut u8, len)?;
        let auxv_base = user_sp;
        for i in 0..auxs.len() {
            unsafe {
                // *((user_sp + i * core::mem::size_of::<AuxHeader>()) as *mut AuxHeader) = auxs[i];
                *((user_sp + i * core::mem::size_of::<AuxHeader>()) as *mut usize) =
                    auxs[i].aux_type;
                *((user_sp + i * core::mem::size_of::<AuxHeader>() + core::mem::size_of::<usize>())
                    as *mut usize) = auxs[i].value;
            }
        }
        stack_trace!();
        // Construct envp
        let len = (envs.len() + 1) * core::mem::size_of::<usize>();
        user_sp -= len;
        UserCheck::new().check_writable_slice(user_sp as *mut u8, len)?;
        let envp_base = user_sp;
        for i in 0..envs.len() {
            unsafe {
                *((envp_base + i * core::mem::size_of::<usize>()) as *mut usize) = envp[i];
            }
        }
        unsafe {
            *((envp_base + envs.len() * core::mem::size_of::<usize>()) as *mut usize) = 0;
        }
        // Construct argv
        let len = (args.len() + 1) * core::mem::size_of::<usize>();
        user_sp -= len;
        UserCheck::new().check_writable_slice(user_sp as *mut u8, len)?;
        let argv_base = user_sp;
        for i in 0..args.len() {
            unsafe {
                *((argv_base + i * core::mem::size_of::<usize>()) as *mut usize) = argv[i];
            }
        }
        unsafe {
            *((argv_base + args.len() * core::mem::size_of::<usize>()) as *mut usize) = 0;
        }
        // We save the argc just below the argv_base.
        // Note that this is required by POSIX
        user_sp -= core::mem::size_of::<usize>();
        UserCheck::new().check_writable_slice(user_sp as *mut u8, core::mem::size_of::<usize>())?;
        unsafe {
            *(user_sp as *mut usize) = args.len();
        }
        // let argc_addr = user_sp;
        stack_trace!();

        // Initialize trap_cx
        let mut trap_cx = TrapContext::app_init_context(entry_point, user_sp);
        debug!("entry {:#x}, sp {:#x}", entry_point, user_sp);
        let argc = unsafe { *(user_sp as *const usize) };
        debug!("argc {}", argc);

        // trap_cx.user_x[10] = user_sp;
        // a0 -> argc, a1 -> argv, a2 -> envp
        trap_cx.user_x[10] = args.len();
        trap_cx.user_x[11] = argv_base;
        trap_cx.user_x[12] = envp_base;
        trap_cx.user_x[13] = auxv_base;
        log::info!(
            "a0(argc) {:#x}, a1(argv) {:#x}, a2(envp) {:#x} a3(auxv) {:#x} sp {:#x}",
            args.len(),
            argv_base,
            envp_base,
            auxv_base,
            trap_cx.user_x[2],
        );

        main_thread_inner.trap_context = trap_cx;

        // Ok(args.len()  )
        Ok(0)
    }

    /// Create a new thread
    /// TODO: take more args into account
    pub fn create_thread(
        self: &Arc<Self>,
        stack: usize,
        tls_ptr: usize,
        parent_tid_ptr: usize,
        child_tid_ptr: usize,
        flags: CloneFlags,
    ) -> SyscallRet {
        self.clone_thread(stack, tls_ptr, parent_tid_ptr, child_tid_ptr, flags)
    }

    fn clone_thread(
        self: &Arc<Self>,
        stack: usize,
        tls_ptr: usize,
        parent_tid_ptr: usize,
        child_tid_ptr: usize,
        flags: CloneFlags,
    ) -> SyscallRet {
        // Note that the user mode code will put the `func` and `arg` in
        // 0(stack) and 8(stack)

        UserCheck::new().check_writable_slice(stack as *mut u8, CLONE_STACK_SIZE)?;

        let _sum_guard = SumGuard::new();

        let entry_point = unsafe { *(stack as *const usize) };
        let arg = unsafe {
            let arg_addr = stack + core::mem::size_of::<usize>();
            *(arg_addr as *const usize)
        };

        // let mut trap_context = TrapContext::app_init_context(entry_point, stack);
        let mut trap_context = *current_trap_cx();
        trap_context.set_entry_point(entry_point);
        trap_context.set_sp(stack);
        trap_context.user_x[10] = arg as usize;
        // Thread local storage
        trap_context.user_x[4] = tls_ptr;
        // Global pointer
        trap_context.user_x[3] = current_trap_cx().user_x[3];
        log::info!("[clone_thread] gp {:#x}", trap_context.user_x[3]);

        let new_thread = Arc::new(Thread::new(
            self.clone(),
            Some(current_task()),
            trap_context,
            stack,
            None,
        ));
        // attach the new thread to process
        current_process()
            .inner
            .lock()
            .threads
            .insert(new_thread.tid(), Arc::downgrade(&new_thread));

        let tid = new_thread.tid();

        let new_thread_inner = unsafe { &mut (*new_thread.inner.get()) };
        if flags.contains(CloneFlags::CLONE_CHILD_CLEARTID) {
            new_thread_inner.tid_addr.clear_tid_address = Some(child_tid_ptr);
            UserCheck::new()
                .check_writable_slice(child_tid_ptr as *mut u8, core::mem::size_of::<usize>())?;
            // unsafe {
            //     *(child_tid_ptr as *mut usize) = tid;
            // }
            unsafe {
                *(child_tid_ptr as *mut usize) = 0;
            }
            log::info!(
                "[clone_thread] CLONE_CHILD_CLEARTID: child tid ptr {:#x}, tid {}",
                child_tid_ptr,
                tid
            );
        }
        if flags.contains(CloneFlags::CLONE_CHILD_SETTID) {
            new_thread_inner.tid_addr.set_tid_address = Some(child_tid_ptr);
            UserCheck::new()
                .check_writable_slice(child_tid_ptr as *mut u8, core::mem::size_of::<usize>())?;
            unsafe {
                *(child_tid_ptr as *mut usize) = tid;
            }
            log::info!(
                "[clone_thread] CLONE_CHILD_SETTID: child tid ptr {:#x}, tid {}",
                child_tid_ptr,
                tid
            );
        }
        if flags.contains(CloneFlags::CLONE_PARENT_SETTID) {
            UserCheck::new()
                .check_writable_slice(parent_tid_ptr as *mut u8, core::mem::size_of::<usize>())?;
            unsafe {
                *(parent_tid_ptr as *mut usize) = tid;
            }
            log::info!(
                "[clone_thread] CLONE_PARENT_SETTID: parent tid ptr {:#x}, tid {}",
                parent_tid_ptr,
                tid
            );
        }
        // if flags.contains(CloneFlags::CLONE_SIGHAND) {
        //     todo!()
        // }

        thread::spawn_thread(new_thread);

        info!(
            "[Process::clone_thread] start func {:#x}, tls {:#x}, start arg {:#x}",
            unsafe { *((*((stack + 8) as *const usize)) as *const usize) },
            tls_ptr,
            unsafe { *((*((stack + 8) as *const usize) + 8) as *const usize) }
        );

        info!(
            "[Process::clone_thread] clone a new thread, tid {}, sp {:#x}, sepc {:#x}",
            tid, stack, entry_point
        );
        Ok(tid)
    }

    fn clone_process(
        self: &Arc<Self>,
        stack: Option<usize>,
        flags: CloneFlags,
    ) -> GeneralRet<Arc<Self>> {
        stack_trace!();
        let child = self.inner_handler(move |parent_inner| {
            assert_eq!(parent_inner.thread_count(), 1);
            let pid = Arc::new(tid_alloc());
            debug!(
                "fork: child's pid {}, parent's pid {} before",
                pid.0, self.pid.0
            );
            // clone parent's memory_space completely including trampoline/ustacks/trap_cxs
            // here we just copy on write
            let memory_space =
                MemorySpace::from_existed_user_lazily(&mut parent_inner.memory_space);
            parent_inner.memory_space.activate();
            // let memory_space = MemorySpace::from_existed_user(&parent_inner.memory_space);

            debug!("fork: child's pid {}, parent's pid {}", pid.0, self.pid.0);
            // create child process pcb
            let child_fd_table = FdTable::from_another(&parent_inner.fd_table)?;
            let child_socket_table = SocketTable::from_another(&parent_inner.socket_table)?;

            let child = Arc::new(Self {
                pid,
                inner: SpinNoIrqLock::new(ProcessInner {
                    is_zombie: false,
                    memory_space,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    fd_table: child_fd_table,
                    socket_table: child_socket_table,
                    threads: BTreeMap::new(),
                    // sig_queue: child_sig_queue,
                    futex_queue: FutexQueue::new(),
                    exit_code: 0,
                    cwd: parent_inner.cwd.clone(),
                    timers: [ITimerval::default(); 3],
                    rlimit: parent_inner.rlimit.clone(),
                    pgid: parent_inner.pgid,
                    #[cfg(not(feature = "multi_hart"))]
                    pselect_times: 0,
                }),
            });
            debug!("fork: child cwd {}", parent_inner.cwd);
            // add child
            parent_inner.children.push(Arc::clone(&child));

            Ok(child)
        })?;

        // create main thread of child process
        // note that we copy the parent's current thread's trap context
        // to child's main thread
        let main_thread = Arc::new(Thread::from_another(
            current_task(),
            child.clone(),
            stack,
            Some(child.pid.clone()),
            flags,
        ));
        // attach task to child process
        child
            .inner
            .lock()
            .threads
            .insert(main_thread.tid(), Arc::downgrade(&main_thread));

        PROCESS_MANAGER.add(child.pid(), &child);
        PROCESS_GROUP_MANAGER.add_process(child.pgid(), child.pid());
        // add this thread to scheduler
        main_thread.trap_context_mut().user_x[10] = 0;
        // info!("fork return1, sepc: {:#x}", main_thread.trap_context_mut().sepc);
        thread::spawn_thread(main_thread);
        Ok(child)
    }
}
