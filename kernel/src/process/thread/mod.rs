mod exit;
mod schedule;
#[allow(clippy::module_inception)]
mod thread_loop;
pub mod tid;
mod time;

use self::{
    tid::{tid_alloc, TidAddress, TidHandle},
    time::ThreadTimeInfo,
};

use super::{resource::CpuSet, Process, PROCESS_MANAGER};
use crate::signal::SignalContext;
use crate::trap::TrapContext;
use crate::{
    signal::{signal_queue::SigQueue, SignalTrampoline},
    stack_trace,
    sync::mutex::SpinNoIrqLock,
};
use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;
use core::{cell::UnsafeCell, task::Waker};
pub use schedule::{spawn_kernel_thread, spawn_thread, yield_now};

pub use exit::{
    exit_and_terminate_all_threads, terminate_all_threads_except_main, terminate_given_thread,
};

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
    /// Tid address, which may be modified by `set_tid_address` syscall
    pub tid_addr: TidAddress,
    /// Time info
    pub time_info: ThreadTimeInfo,
    /// Waker
    pub waker: Option<Waker>,
    /// Ustack top
    pub ustack_top: usize,
    /// Thread local signals.
    /// TODO: should we lock?
    pub sig_queue: SpinNoIrqLock<SigQueue>,
    /// Thread cpu affinity
    pub cpu_set: CpuSet,
    /// Note that the process may modify this value in the another thread
    /// (e.g. `exec`)
    pub terminated: AtomicBool,
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
        let tid = match tid {
            Some(tid) => tid,
            None => Arc::new(tid_alloc()),
        };
        let thread = Self {
            tid: tid.clone(),
            sig_trampoline,
            process: process.clone(),
            // user_specified_stack,
            inner: UnsafeCell::new(ThreadInner {
                trap_context,
                signal_context: None,
                ustack_top,
                tid_addr: TidAddress::new(),
                time_info: ThreadTimeInfo::new(),
                waker: None,
                sig_queue: SpinNoIrqLock::new(SigQueue::from_another(
                    &process.inner.lock().sig_queue,
                )),
                // TODO: need to change if multi_hart
                cpu_set: CpuSet::new(1),
                terminated: AtomicBool::new(false),
            }),
        };
        PROCESS_MANAGER.add(tid.0, &process);
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
        let tid = match tid {
            Some(tid) => tid,
            None => Arc::new(tid_alloc()),
        };
        PROCESS_MANAGER.add(tid.0, &new_process);
        Self {
            tid: tid.clone(),
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
                tid_addr: TidAddress::new(),
                time_info: ThreadTimeInfo::new(),
                waker: None,
                sig_queue: SpinNoIrqLock::new(SigQueue::from_another(unsafe {
                    &(*another.inner.get()).sig_queue.lock()
                })),
                // TODO: need to change if multi_hart
                cpu_set: CpuSet::new(1),
                terminated: AtomicBool::new(false),
            }),
        }
    }

    /// We can get whatever we want in the inner by providing a handler
    pub unsafe fn inner_handler<T>(&self, f: impl FnOnce(&mut ThreadInner) -> T) -> T {
        f(&mut *self.inner.get())
    }

    /// Send signal to this process
    pub fn send_signal(&self, signo: usize) {
        log::info!("[Thread::send_signal] signo {}", signo);
        let inner = unsafe { &mut *self.inner.get() };
        inner.sig_queue.lock().pending_sigs.add(signo);
    }

    /// Get the ref of signal context
    pub fn signal_context(&self) -> &SignalContext {
        self.sig_trampoline.signal_context()
    }

    /// Set the signal context for the current thread
    pub fn set_signal_context(&self, signal_context: SignalContext) {
        self.sig_trampoline.set_signal_context(signal_context)
    }

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
        let inner = unsafe { &mut (*self.inner.get()) };
        inner
            .terminated
            .store(true, core::sync::atomic::Ordering::Release)
    }

    /// Whether this thread has been terminated or not
    pub fn is_zombie(&self) -> bool {
        unsafe {
            (*self.inner.get())
                .terminated
                .load(core::sync::atomic::Ordering::Acquire)
        }
    }

    /// Tid of this thread
    pub fn tid(&self) -> usize {
        self.tid.0
    }

    /// Wake up this thread.
    /// This is called mostly because of signal
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
