//! A single global instance of [`PidAllocator`] called `PID_ALLOCATOR` allocates
//! pid for user apps.
//!

///
pub mod thread;

use log::{debug, info, warn};
pub use thread::yield_now;

mod manager;
mod pid;
/// Aux header
pub mod aux;
// #[allow(clippy::module_inception)]
// mod task;

use crate::{
    config::{mm::USER_STACK_SIZE, process::CLONE_STACK_SIZE},
    fs::FdTable,
    loader::get_app_data_by_name,
    mm::{user_check::UserCheck, MemorySet, RecycleAllocator},
    process::{thread::terminate_all_threads_except_main, aux::{AuxHeader, AT_EXECFN, AT_NULL, AT_RANDOM}},
    processor::{current_process, current_task, local_hart, SumGuard},
    signal::{SigHandlerManager, SigInfo, SigQueue},
    stack_trace,
    sync::{mutex::SpinNoIrqLock, CondVar},
    trap::TrapContext,
    utils::error::{GeneralRet, SyscallRet},
};
use alloc::{
    collections::BTreeMap,
    string::String,
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};
// use lazy_static::*;
use thread::Thread;

pub use manager::PROCESS_MANAGER;
pub use pid::{pid_alloc, PidHandle};

// pub enum TaskStatus {
//     Running,
//     Sleep,
//     Zombie,
// }

///
pub static mut INITPROC: Option<Arc<Process>> = None;

///Add init process to the manager
pub fn add_initproc() {
    stack_trace!();
    // debug!("add initproc");
    // let init_inode = fs::fat32_tmp::open_file("initproc", fs::fat32_tmp::OpenFlags::RDONLY).expect("Cannot find `initproc`!!");
    // let shell_inode = fs::fat32_tmp::open_file("usershell", fs::fat32_tmp::OpenFlags::RDONLY);
    // if shell_inode.is_none() {
    //     warn!("Cannot find user_shell");
    // }
    // let elf_data = init_inode.read_all();
    // unsafe { INITPROC = Some(Process::new(&elf_data)) }
    let elf_data = get_app_data_by_name("initproc").unwrap();
    unsafe { INITPROC = Some(Process::new(elf_data)) }
}



// const PRELIMINARY_TESTS: [&str; 31] = [
//     "brk",
//     "chdir",
//     "clone",
//     "close",
//     "dup2",
//     "dup",
//     "execve",
//     "exit",
//     "fork",
//     "fstat",
//     "getcwd",
//     "getdents",
//     "getpid",
//     "getppid",
//     "gettimeofday",
//     "mkdir_",
//     "mmap",
//     "mount",
//     "munmap",
//     "openat",
//     "open",
//     "pipe",
//     "read",
//     "times",
//     "umount",
//     "uname",
//     "unlink",
//     "wait",
//     "waitpid",
//     "write",
//     "yield",
// ];

// /// Scan all prilimary tests
// pub fn scan_prilimary_tests() {
//     info!("---------- SCAN PRELIMINARY TEST -----------\n");
//     for test in PRELIMINARY_TESTS {
//         let inode = fs::fat32_tmp::open_file(test, OpenFlags::RDONLY);
//         if inode.is_none() {
//             continue;
//         }
//         let inode = inode.unwrap();
//         let elf_data = inode.read_all();
//         Process::new(&elf_data);
//     }
// }

use self::thread::TidHandle;

///
pub struct ProcessInner {
    /// Whether this process is a zombie process
    pub is_zombie: bool,
    /// The process's address space
    pub memory_set: MemorySet,
    /// Parent process
    pub parent: Option<Weak<Process>>,
    /// Children processes
    pub children: Vec<Arc<Process>>,
    /// File descriptor table
    pub fd_table: FdTable,
    /// Allocate tid
    pub tid_allocator: RecycleAllocator,
    /// TODO: use BTreeMap to query and delete more quickly
    pub threads: Vec<Weak<Thread>>,
    /// Signal handlers for every signal
    pub sig_handler: Arc<SpinNoIrqLock<SigHandlerManager>>,
    /// Pending sigs that wait for the prcoess to handle
    pub pending_sigs: SigQueue,
    /// UStack base of all threads(the lowest bound)
    pub ustack_base: usize,
    /// Addr -> Condvar map
    pub addr_to_condvar_map: BTreeMap<usize, CondVar>,
    /// Exit code of the current process
    /// Note that we may need to put this member in every thread
    pub exit_code: i8,
    /// Current Work Directory
    /// Maybe change to Dentry later.
    pub cwd: String,
}

impl ProcessInner {
    ///
    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }
}
/// Process control block
pub struct Process {
    /// immutable
    pid: PidHandle,
    /// mutable
    inner: SpinNoIrqLock<ProcessInner>,
}

impl Process {
    ///
    pub fn alloc_tid(&self) -> TidHandle {
        TidHandle(self.inner.lock().tid_allocator.alloc())
    }

    ///
    pub fn dealloc_tid(&self, tid: usize) {
        self.inner.lock().tid_allocator.dealloc(tid);
    }

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
    pub fn alloc_fd(&self) -> usize {
        self.inner.lock().fd_table.alloc_fd()
    }

    /// Send signal to this process
    pub fn send_signal(&self, sig_info: SigInfo) {
        self.inner.lock().pending_sigs.sig_queue.push_back(sig_info);
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        debug!("process {} died!", self.pid());
        // println!("\u{1B}[31m process [pid {}] died! \u{1B}[0m", self.get_pid());
    }
}

impl Process {
    /// Create a new process
    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        let (memory_set, user_sp_base, entry_point, auxv) = MemorySet::from_elf(elf_data);
        // let debug_pa = memory_set.translate(VirtAddr::from(entry_point).floor()).unwrap().ppn().0;
        // println!("entry pa {:#x}", debug_pa);
        // Alloc a pid
        let pid_handle = pid_alloc();
        let process = Arc::new(Self {
            pid: pid_handle,
            inner: SpinNoIrqLock::new(ProcessInner {
                is_zombie: false,
                memory_set,
                parent: None,
                children: Vec::new(),
                fd_table: FdTable::new(),
                tid_allocator: RecycleAllocator::new(0),
                threads: Vec::new(),
                sig_handler: Arc::new(SpinNoIrqLock::new(SigHandlerManager::new())),
                pending_sigs: SigQueue::new(),
                ustack_base: user_sp_base,
                addr_to_condvar_map: BTreeMap::new(),
                exit_code: 0,
                cwd: String::from("/"),
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
        ));
        // thread.alloc_ustack();

        process.inner.lock().threads.push(Arc::downgrade(&thread));
        // Add the main thread into scheduler
        thread::spawn_thread(thread);
        PROCESS_MANAGER
            .lock()
            .0
            .insert(process.pid(), Arc::downgrade(&process));
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
        debug!("exec pid {}", current_process().pid());
        stack_trace!();
        let (memory_set, ustack_base, entry_point, mut auxs) = MemorySet::from_elf(elf_data);
        let task_ptr: *const Thread = self.inner_handler(|proc| {
            assert_eq!(proc.thread_count(), 1);
            // memory_set with elf program headers/trampoline/trap context/user stack
            // substitute memory_set
            memory_set.activate();
            // Change hart local context's pagetable (quite important!!!)
            let hart = local_hart();
            hart.change_page_table(memory_set.page_table.clone());
            // process_inner.memory_set = memory_set;
            proc.threads[0].as_ptr()
        });

        terminate_all_threads_except_main();
        // Then we alloc user resource for main thread again
        // since memory_set has been changed
        let task = unsafe {
            &*task_ptr
            // &*process_inner.threads[0].as_ptr()
        };
        let task_inner = unsafe { &mut *task.inner.get() };

        // TODO: not sure whether we should dealloc ustack here?

        self.inner_handler(|proc| {
            proc.ustack_base = ustack_base;
            proc.memory_set = memory_set;
        });
        // // dealloc old ustack
        // task.dealloc_ustack();
        // self.inner.lock().memory_set = memory_set;
        task_inner.ustack_base = ustack_base;
        // alloc new ustack
        task.alloc_ustack();


        // ---- The following to to push arguments on user stack ----

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
                *((user_sp + i * core::mem::size_of::<AuxHeader>()) as *mut usize) = auxs[i].aux_type;
                *((user_sp + i * core::mem::size_of::<AuxHeader>() + core::mem::size_of::<usize>()) as *mut usize) = auxs[i].value;
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
        debug!("a0(argc) {:#x}, a1(argv) {:#x}, a2(envp) {:#x} a3(auxv) {:#x}", args.len(), argv_base, envp_base, auxv_base);

        task_inner.trap_context = trap_cx;
        Ok(args.len() as isize)
    }

    // /// Create a new thread
    // pub fn create_thread(self: &Arc<Self>, f: usize, arg: *const u8) -> usize {

    //     // Note that the user mode code will put the `func` and `arg` in
    //     // 0(stack) and 8(stack)

    //     // Here we give a dummy sp since it should be replaced in `app_init_context`
    //     let mut trap_context = TrapContext::app_init_context(f, 0);
    //     trap_context.user_x[10] = arg as usize;

    //     let ustack_base = self.inner_handler(|proc| proc.ustack_base);
    //     let new_thread = Arc::new(Thread::new(self.clone(), trap_context, ustack_base));
    //     // attach the new thread to process
    //     current_process()
    //         .inner
    //         .lock()
    //         .threads
    //         .push(Arc::downgrade(&new_thread));
    //     let tid = new_thread.tid();
    //     thread::spawn_thread(new_thread);
    //     tid
    // }

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
        let new_thread = Arc::new(Thread::new(self.clone(), trap_context, ustack_base, true));
        // attach the new thread to process
        current_process()
            .inner
            .lock()
            .threads
            .push(Arc::downgrade(&new_thread));
        let tid = new_thread.tid();
        thread::spawn_thread(new_thread);
        Ok(tid as isize)
    }

    fn clone_process(self: &Arc<Self>, stack: Option<usize>) -> GeneralRet<Arc<Self>> {
        let child = self.inner_handler(move |parent_inner| {
            assert_eq!(parent_inner.thread_count(), 1);
            let pid = pid_alloc();
            debug!(
                "fork: child's pid {}, parent's pid {} before",
                pid.0, self.pid.0
            );
            // clone parent's memory_set completely including trampoline/ustacks/trap_cxs
            // here we just copy on write
            let memory_set = MemorySet::from_existed_user_lazily(&mut parent_inner.memory_set);
            // let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set);

            // alloc a pid
            debug!("fork: child's pid {}, parent's pid {}", pid.0, self.pid.0);
            // create child process pcb
            let child = Arc::new(Self {
                pid,
                inner: SpinNoIrqLock::new(ProcessInner {
                    is_zombie: false,
                    memory_set,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    fd_table: FdTable::from_another(&parent_inner.fd_table),
                    tid_allocator: RecycleAllocator::new(0),
                    threads: Vec::new(),
                    sig_handler: Arc::new(SpinNoIrqLock::new(SigHandlerManager::new())),
                    pending_sigs: SigQueue::new(),
                    ustack_base: parent_inner.ustack_base,
                    addr_to_condvar_map: BTreeMap::new(),
                    exit_code: 0,
                    cwd: parent_inner.cwd.clone(),
                }),
            });
            // add child
            parent_inner.children.push(Arc::clone(&child));

            child
        });

        // create main thread of child process
        // note that we copy the parent's current thread's trap context
        // to child's main thread
        let main_thread = Arc::new(current_task().from_current(child.clone(), stack));
        // attach task to child process
        let child_clone = child.clone();
        child_clone
            .inner
            .lock()
            .threads
            .push(Arc::downgrade(&main_thread));
        PROCESS_MANAGER
            .lock()
            .0
            .insert(child.pid(), Arc::downgrade(&child));
        // add this thread to scheduler
        main_thread.trap_context_mut().user_x[10] = 0;
        // info!("fork return1, sepc: {:#x}", main_thread.trap_context_mut().sepc);
        thread::spawn_thread(main_thread);
        Ok(child)
    }
}
