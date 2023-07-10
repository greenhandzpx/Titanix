//! A single global instance of [`PidAllocator`] called `PID_ALLOCATOR` allocates
//! pid for user apps.
//!

///
pub mod thread;

use log::{debug, info};
pub use thread::yield_now;

/// Aux header
pub mod aux;
mod manager;
mod pid;
/// System resource
pub mod resource;
// #[allow(clippy::module_inception)]
// mod task;

use crate::{
    config::{mm::USER_STACK_SIZE, process::CLONE_STACK_SIZE},
    fs::FdTable,
    loader::get_app_data_by_name,
    mm::{user_check::UserCheck, MemorySpace, RecycleAllocator},
    process::{
        aux::{AuxHeader, AT_EXECFN, AT_NULL, AT_RANDOM},
        pid::tid_alloc,
        thread::terminate_all_threads_except_main,
    },
    processor::{current_process, current_task, hart::local_hart, SumGuard},
    signal::{SigHandlerManager, SigInfo, SigQueue, SIGKILL},
    stack_trace,
    sync::{mutex::SpinNoIrqLock, FutexQueue, Mailbox},
    timer::posix::ITimerval,
    trap::TrapContext,
    utils::error::{GeneralRet, SyscallErr, SyscallRet},
};
use alloc::{
    string::String,
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};
// use lazy_static::*;
use thread::Thread;

pub use manager::PROCESS_MANAGER;

///Add init process to the manager
pub fn add_initproc() {
    stack_trace!();
    let elf_data = get_app_data_by_name("initproc").unwrap();
    let _init_proc = Process::new_initproc(elf_data);
    // PROCESS_MANAGER.add_process(_init_proc.pid(), &_init_proc);

    #[cfg(feature = "user_spin")]
    {
        let elf_data = get_app_data_by_name("user_spin").unwrap();
        let spin_proc = Process::new_initproc(elf_data);
        info!("[add_initproc]: add user spin, pid {}", spin_proc.pid());
        // PROCESS_MANAGER.add_process(spin_proc.pid(), &spin_proc);
    }
}

use self::{pid::TidHandle, resource::RLimit};

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
    /// File descriptor table
    pub fd_table: FdTable,
    /// TODO: use BTreeMap to query and delete more quickly
    pub threads: Vec<Weak<Thread>>,
    /// Signal handlers for every signal
    pub sig_handler: Arc<SpinNoIrqLock<SigHandlerManager>>,
    /// Pending sigs that wait for the prcoess to handle
    pub pending_sigs: SigQueue,
    /// UStack base of all threads(the lowest bound)
    pub ustack_base: usize,
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
    /// mailbox,
    pub mailbox: Arc<Mailbox>,
    /// mutable
    pub inner: SpinNoIrqLock<ProcessInner>,
}

impl Process {
    /// Main thread's trap context
    pub fn trap_context_main(&self) -> &mut TrapContext {
        let inner = self.inner.lock();
        assert!(inner.thread_count() > 0);
        unsafe { (*inner.threads[0].as_ptr()).trap_context_mut() }
    }

    /// Get the process's pid
    pub fn pid(&self) -> usize {
        self.pid.0
    }

    /// We can get whatever we want in the inner by providing a handler
    pub fn inner_handler<T>(&self, f: impl FnOnce(&mut ProcessInner) -> T) -> T {
        f(&mut self.inner.lock())
    }

    ///
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
    pub fn send_signal(&self, sig_info: SigInfo) {
        if sig_info.signo == SIGKILL {
            self.inner_handler(|proc| {
                for thread in proc.threads.iter() {
                    if let Some(thread) = thread.upgrade() {
                        thread.terminate();
                        thread.wake_up();
                    }
                }
            })
        }
        self.inner.lock().pending_sigs.sig_queue.push_back(sig_info);
    }

    ///
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
        debug!("process {} died!", self.pid());
    }
}

impl Process {
    /// Create a new process
    pub fn new_initproc(elf_data: &[u8]) -> Arc<Self> {
        let (memory_space, user_sp_base, entry_point, auxv) = MemorySpace::from_elf(elf_data);
        // let debug_pa = memory_space.translate(VirtAddr::from(entry_point).floor()).unwrap().ppn().0;
        // println!("entry pa {:#x}", debug_pa);
        // Alloc a pid
        let pid = Arc::new(tid_alloc());
        let process = Arc::new(Self {
            pid: pid.clone(),
            mailbox: Arc::new(Mailbox::new()),
            inner: SpinNoIrqLock::new(ProcessInner {
                is_zombie: false,
                memory_space,
                parent: None,
                children: Vec::new(),
                fd_table: FdTable::new(),
                threads: Vec::new(),
                sig_handler: Arc::new(SpinNoIrqLock::new(SigHandlerManager::new())),
                pending_sigs: SigQueue::new(),
                ustack_base: user_sp_base,
                futex_queue: FutexQueue::new(),
                exit_code: 0,
                cwd: String::from("/"),
                timers: [ITimerval::default(); 3],
                rlimit: RLimit::new(0, 0),
            }),
        });
        let trap_context =
            TrapContext::app_init_context(entry_point, user_sp_base + USER_STACK_SIZE);
        // create a main thread
        let thread = Arc::new(Thread::new(
            process.clone(),
            trap_context,
            user_sp_base,
            false,
            Some(pid),
        ));
        // thread.alloc_ustack();

        process.inner.lock().threads.push(Arc::downgrade(&thread));
        PROCESS_MANAGER.add_process(process.pid(), &process);
        // Add the main thread into scheduler
        thread::spawn_thread(thread);
        debug!("create a new process, pid {}", process.pid());
        process
    }

    /// Fork a new process
    /// `stack` points to the new cloned process's main thread's stack if not `None`
    pub fn fork(self: &Arc<Self>, stack: Option<usize>) -> GeneralRet<Arc<Self>> {
        self.clone_process(stack)
    }

    /// Exec a new program.
    /// Note that the return value is `argc`.
    /// When one process invokes `exec`, all of the threads will terminate except the
    /// main thread, and the new program is executed in the main thread.
    pub fn exec(&self, elf_data: &[u8], args: Vec<String>, envs: Vec<String>) -> SyscallRet {
        stack_trace!();
        debug!("exec pid {}", current_process().pid());

        // memory_space with elf program headers/trampoline/trap context/user stack
        // substitute memory_space
        let (memory_space, ustack_base, entry_point, mut auxs) = MemorySpace::from_elf(elf_data);
        let task_ptr: *const Thread = self.inner_handler(|proc| {
            assert_eq!(proc.thread_count(), 1);
            // Change hart local context's pagetable (quite important!!!)
            memory_space.activate();
            let hart = local_hart();
            hart.change_page_table(memory_space.page_table.clone());
            // process_inner.memory_space = memory_space;
            proc.threads[0].as_ptr()
        });

        terminate_all_threads_except_main();
        // Then we alloc user resource for main thread again
        // since memory_space has been changed
        let task = unsafe {
            &*task_ptr
            // &*process_inner.threads[0].as_ptr()
        };
        let task_inner = unsafe { &mut *task.inner.get() };

        // TODO: not sure whether we should dealloc ustack here?

        self.inner_handler(|proc| {
            proc.ustack_base = ustack_base;
            proc.memory_space = memory_space;
        });
        // // dealloc old ustack
        // task.dealloc_ustack();
        // self.inner.lock().memory_space = memory_space;
        task_inner.ustack_base = ustack_base;
        // alloc new ustack
        task.alloc_ustack();

        // ----- The following to to push arguments on user stack -----

        let mut user_sp = task.ustack_top();
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

        // Copy `platform`
        let platform = "RISC-V64";
        user_sp -= platform.len() + 1;
        user_sp -= user_sp % core::mem::size_of::<usize>();
        let p = user_sp as *mut u8;
        unsafe {
            p.copy_from(platform.as_ptr(), platform.len());
            *((p as usize + platform.len()) as *mut u8) = 0;
        }

        // Copy 16 random bytes(here is 0)
        user_sp -= 16;
        auxs.push(AuxHeader {
            aux_type: AT_RANDOM,
            value: user_sp,
        });

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

        // // // Make the user_sp aligned to 8B for k210 platform
        // // let len = user_sp % core::mem::size_of::<usize>();
        // // user_sp -= len;

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
        debug!(
            "a0(argc) {:#x}, a1(argv) {:#x}, a2(envp) {:#x} a3(auxv) {:#x}",
            args.len(),
            argv_base,
            envp_base,
            auxv_base
        );

        task_inner.trap_context = trap_cx;
        // Ok(args.len() as isize)
        Ok(0)
    }

    /// Create a new thread
    /// TODO: take more args into account
    pub fn create_thread(self: &Arc<Self>, stack: usize) -> SyscallRet {
        self.clone_thread(stack)
    }

    fn clone_thread(self: &Arc<Self>, stack: usize) -> SyscallRet {
        // Note that the user mode code will put the `func` and `arg` in
        // 0(stack) and 8(stack)

        UserCheck::new().check_writable_slice(stack as *mut u8, CLONE_STACK_SIZE)?;

        let _sum_guard = SumGuard::new();

        let entry_point = unsafe { *(stack as *const usize) };
        let arg = unsafe {
            let arg_addr = stack + 8;
            *(arg_addr as *const usize)
        };

        let mut trap_context = TrapContext::app_init_context(entry_point, stack);
        trap_context.user_x[10] = arg as usize;

        let ustack_base = self.inner_handler(|proc| proc.ustack_base);
        let new_thread = Arc::new(Thread::new(
            self.clone(),
            trap_context,
            ustack_base,
            true,
            None,
        ));
        // attach the new thread to process
        current_process()
            .inner
            .lock()
            .threads
            .push(Arc::downgrade(&new_thread));
        let tid = new_thread.tid();
        thread::spawn_thread(new_thread);

        info!("[Process::clone_thread] clone a new thread, tid {}", tid);
        Ok(tid as isize)
    }

    fn clone_process(self: &Arc<Self>, stack: Option<usize>) -> GeneralRet<Arc<Self>> {
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

            // alloc a pid
            debug!("fork: child's pid {}, parent's pid {}", pid.0, self.pid.0);
            // create child process pcb
            let child_fd_table = FdTable::from_another(&parent_inner.fd_table)?;
            let child = Arc::new(Self {
                pid,
                mailbox: Arc::new(Mailbox::new()),
                inner: SpinNoIrqLock::new(ProcessInner {
                    is_zombie: false,
                    memory_space,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    fd_table: child_fd_table,
                    threads: Vec::new(),
                    sig_handler: Arc::new(SpinNoIrqLock::new(SigHandlerManager::new())),
                    pending_sigs: SigQueue::new(),
                    ustack_base: parent_inner.ustack_base,
                    futex_queue: FutexQueue::new(),
                    exit_code: 0,
                    cwd: parent_inner.cwd.clone(),
                    timers: [ITimerval::default(); 3],
                    rlimit: parent_inner.rlimit.clone(),
                }),
            });
            // add child
            parent_inner.children.push(Arc::clone(&child));

            Ok(child)
        })?;

        // create main thread of child process
        // note that we copy the parent's current thread's trap context
        // to child's main thread
        let main_thread =
            Arc::new(current_task().from_current(child.clone(), stack, Some(child.pid.clone())));
        // attach task to child process
        child
            .inner
            .lock()
            .threads
            .push(Arc::downgrade(&main_thread));

        PROCESS_MANAGER.add_process(child.pid(), &child);
        // add this thread to scheduler
        main_thread.trap_context_mut().user_x[10] = 0;
        // info!("fork return1, sepc: {:#x}", main_thread.trap_context_mut().sepc);
        thread::spawn_thread(main_thread);
        Ok(child)
    }
}
