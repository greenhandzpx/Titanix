mod exit;
mod schedule;
#[allow(clippy::module_inception)]
mod thread_loop;
mod thread_state;
pub mod tid;
mod time;

use self::{
    thread_state::{ThreadState, ThreadStateAtomic},
    tid::{tid_alloc, TidAddress, TidHandle},
    time::ThreadTimeInfo,
};

use super::Process;
use crate::{
    config::mm::PAGE_SIZE,
    executor,
    mm::{
        memory_space::{vm_area::VmAreaType, MapType},
        MapPermission, Page, PageBuilder, VirtAddr,
    },
    signal::{signal_queue::SigQueue, SigInfo, SignalTrampoline},
    stack_trace,
    sync::mutex::SpinNoIrqLock,
};
use crate::{
    mm::{KernelAddr, PhysAddr},
    processor::SumGuard,
    signal::SignalContext,
};
use crate::{sync::OwnedFutexes, trap::TrapContext};
use alloc::sync::Arc;
use core::future::Future;
use core::{cell::UnsafeCell, task::Waker};

pub use exit::{
    exit_and_terminate_all_threads, terminate_all_threads_except_main, terminate_given_thread,
};

use thread_loop::threadloop;

// pub use task::TaskControlBlock;
// pub use task::TaskStatus;

/// Thread control block
pub struct Thread {
    /// immutable
    tid: Arc<TidHandle>,
    /// signal trampoline(store ucontext)
    pub sig_trampoline: SignalTrampoline,
    /// the process this thread belongs to
    pub process: Arc<Process>,
    /// mutable
    pub inner: UnsafeCell<ThreadInner>,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

/// Thread inner,
/// This struct can only be visited by the local hart except the `terminated` field
/// which is the reason why it is an atomic variable
pub struct ThreadInner {
    // TODO: add more members
    /// Trap context that saves both kernel and user msg
    pub trap_context: TrapContext,
    /// Used for signal handle
    pub signal_context: Option<SignalContext>,
    /// Thread state.
    /// Note that this may be modified by another thread, which
    /// need to be sync
    pub state: ThreadStateAtomic,
    /// Tid address, which may be modified by `set_tid_address` syscall
    pub tid_addr: TidAddress,
    /// Time info
    pub time_info: ThreadTimeInfo,
    /// Waker
    pub waker: Option<Waker>,
    /// Ustack top
    pub ustack_top: usize,
    /// Futexes this thread owns
    pub owned_futexes: OwnedFutexes,
    /// Thread local signals.
    /// TODO: should we lock?
    pub pending_sigs: SpinNoIrqLock<SigQueue>,
    // /// Soft irq exit status.
    // /// Note that the process may modify this value in the another thread
    // /// (e.g. `exec`)
    // pub terminated: AtomicBool,
}

impl Thread {
    /// Construct a thread control block
    pub fn new(
        process: Arc<Process>,
        trap_context: TrapContext,
        ustack_top: usize,
        // user_specified_stack: bool,
        tid: Option<Arc<TidHandle>>,
    ) -> Self {
        let sig_trampoline = SignalTrampoline::new(process.clone());
        let thread = Self {
            tid: match tid {
                Some(tid) => tid,
                None => Arc::new(tid_alloc()),
            },
            sig_trampoline,
            process: process.clone(),
            // user_specified_stack,
            inner: UnsafeCell::new(ThreadInner {
                trap_context,
                signal_context: None,
                ustack_top,
                state: ThreadStateAtomic::new(),
                tid_addr: TidAddress::new(),
                time_info: ThreadTimeInfo::new(),
                waker: None,
                owned_futexes: OwnedFutexes::new(),
                pending_sigs: SpinNoIrqLock::new(SigQueue::from_another(
                    &process.inner.lock().pending_sigs,
                )),
                // terminated: AtomicBool::new(false),
            }),
        };
        thread
    }

    /// Construct a new thread from another thread
    pub fn from_another(
        another: &Arc<Thread>,
        new_process: Arc<Process>,
        stack: Option<usize>,
        tid: Option<Arc<TidHandle>>,
    ) -> Self {
        stack_trace!();
        let sig_trampoline = SignalTrampoline::new(new_process.clone());
        Self {
            tid: match tid {
                Some(tid) => tid,
                None => Arc::new(tid_alloc()),
            },
            sig_trampoline,
            process: new_process.clone(),
            inner: UnsafeCell::new(ThreadInner {
                trap_context: {
                    let mut trap_context = another.trap_context();
                    if let Some(stack) = stack {
                        trap_context.set_sp(stack);
                    }
                    trap_context
                },
                signal_context: None,
                ustack_top: unsafe { (*another.inner.get()).ustack_top },
                state: ThreadStateAtomic::new(),
                tid_addr: TidAddress::new(),
                time_info: ThreadTimeInfo::new(),
                waker: None,
                // TODO: not sure whether we should inherit the futexes
                owned_futexes: OwnedFutexes::new(),
                pending_sigs: SpinNoIrqLock::new(SigQueue::from_another(unsafe {
                    &(*another.inner.get()).pending_sigs.lock()
                })),
                // terminated: AtomicBool::new(false),
            }),
        }
    }

    // /// Construct a new thread from the current thread
    // pub fn from_current(
    //     &self,
    //     new_process: Arc<Process>,
    //     stack: Option<usize>,
    //     tid: Option<Arc<TidHandle>>,
    // ) -> Self {
    //     stack_trace!();
    //     let sig_trampoline = Arc::new(
    //         PageBuilder::new()
    //             .permission(MapPermission::R | MapPermission::W | MapPermission::U)
    //             .build(),
    //     );

    //     Self {
    //         tid: match tid {
    //             Some(tid) => tid,
    //             None => Arc::new(tid_alloc()),
    //         },
    //         sig_trampoline,
    //         process: new_process.clone(),
    //         inner: UnsafeCell::new(ThreadInner {
    //             trap_context: {
    //                 let mut trap_context = self.trap_context();
    //                 if let Some(stack) = stack {
    //                     trap_context.set_sp(stack);
    //                 }
    //                 trap_context
    //             },
    //             signal_context: None,
    //             ustack_top: unsafe { (*self.inner.get()).ustack_top },
    //             state: ThreadStateAtomic::new(),
    //             tid_addr: TidAddress::new(),
    //             time_info: ThreadTimeInfo::new(),
    //             waker: None,
    //             // TODO: not sure whether we should inherit the futexes
    //             owned_futexes: OwnedFutexes::new(),
    //             pending_sigs: SpinNoIrqLock::new(SigQueue::from_another(unsafe {
    //                 &(*self.inner.get()).pending_sigs.lock()
    //             })),
    //             // terminated: AtomicBool::new(false),
    //         }),
    //     }
    // }

    /// We can get whatever we want in the inner by providing a handler
    pub unsafe fn inner_handler<T>(&self, f: impl FnOnce(&mut ThreadInner) -> T) -> T {
        f(&mut *self.inner.get())
    }

    /// Send signal to this process
    pub fn send_signal(&self, sig_info: SigInfo) {
        log::debug!("[Thread::send_signal] signo {}", sig_info.signo);
        let inner = unsafe { &mut *self.inner.get() };
        inner.pending_sigs.lock().sig_queue.push_back(sig_info);
    }
    /// Get the ref of signal context
    pub fn signal_context(&self) -> &SignalContext {
        self.sig_trampoline.signal_context()
        // unsafe { &(*self.inner.get()).signal_context.as_ref().unwrap() }
    }

    /// Set the signal context for the current thread
    pub fn set_signal_context(&self, signal_context: SignalContext) {
        self.sig_trampoline.set_signal_context(signal_context)
    }

    // /// Signal trampoline start addr
    // pub fn signal_trampoline_addr(&self) -> usize {
    //     KernelAddr::from(PhysAddr::from(self.sig_trampoline.data_frame.ppn)).0
    // }

    /// Get the copied trap context
    pub fn trap_context(&self) -> TrapContext {
        unsafe { (*self.inner.get()).trap_context }
    }

    /// Get the mutable ref of trap context
    pub fn trap_context_mut(&self) -> &mut TrapContext {
        unsafe { &mut (*self.inner.get()).trap_context }
    }

    /// Get the ref of trap context
    pub fn trap_context_ref(&self) -> &TrapContext {
        unsafe { &(*self.inner.get()).trap_context }
    }

    /// Terminate this thread
    pub fn terminate(&self) {
        // unsafe {
        //     (*self.inner.get()).state.store(ThreadState::Zombie);
        //     // (*self.inner.get())
        //     //     .terminated
        //     //     .store(true, Ordering::Relaxed)
        // }
        let inner = unsafe { &mut (*self.inner.get()) };
        inner.state.store(ThreadState::Zombie);
    }

    /// Whether this thread has been terminated or not
    pub fn is_zombie(&self) -> bool {
        unsafe { (*self.inner.get()).state.load() == ThreadState::Zombie }
    }
    /// Whether this thread is runnable or not
    pub fn is_runnable(&self) -> bool {
        unsafe { (*self.inner.get()).state.load() == ThreadState::Runnable }
    }
    /// Whether this thread is sleep or not
    pub fn is_sleep(&self) -> bool {
        unsafe { (*self.inner.get()).state.load() == ThreadState::Sleep }
    }
    /// Let this thread sleep
    /// Note that we now only use this state in sys_futex
    pub fn sleep(&self) {
        unsafe {
            (*self.inner.get()).state.store(ThreadState::Sleep);
        }
    }
    // /// Wake up this thread
    // pub fn wake_up(&self) {
    //     unsafe {
    //         (*self.inner.get()).state.store(ThreadState::Runnable);
    //     }
    // }
    /// Tid of this thread
    pub fn tid(&self) -> usize {
        self.tid.0
    }
    /// Wake up this thread
    pub fn wake_up(&self) {
        unsafe { (*self.inner.get()).waker.as_ref().unwrap().wake_by_ref() }
    }
    /// Set waker for this thread
    pub fn set_waker(&self, waker: Waker) {
        unsafe {
            (*self.inner.get()).waker = Some(waker);
        }
    }
}

/// Yield the current thread (and the scheduler will switch to next thread)
pub async fn yield_now() {
    schedule::YieldFuture(false).await;
}

/// Spawn a new user thread
pub fn spawn_thread(thread: Arc<Thread>) {
    // let future = schedule::OutermostFuture::new(thread.clone(), async {});
    let future = schedule::UserTaskFuture::new(thread.clone(), threadloop(thread));
    let (runnable, task) = executor::spawn(future);
    runnable.schedule();
    task.detach();
}

/// Spawn a new kernel thread(used for doing some kernel init work or timed tasks)
pub fn spawn_kernel_thread<F: Future<Output = ()> + Send + 'static>(kernel_thread: F) {
    let future = schedule::KernelTaskFuture::new(kernel_thread);
    let (runnable, task) = executor::spawn(future);
    runnable.schedule();
    task.detach();
}
