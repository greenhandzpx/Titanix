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
use crate::{
    signal::{signal_queue::SigQueue, SignalTrampoline},
    stack_trace,
    sync::mutex::SpinNoIrqLock,
    syscall::CloneFlags,
};
use crate::{
    signal::{Signal, SignalContext},
    sync::{Event, Mailbox},
};
use crate::{
    signal::{SIGCHLD, SIGKILL},
    trap::TrapContext,
};
use alloc::sync::Arc;
use core::{cell::UnsafeCell, task::Waker};
pub use schedule::{spawn_kernel_thread, spawn_thread, yield_now};

pub use exit::{
    exit_and_terminate_all_threads, terminate_all_threads_except_main, terminate_given_thread,
};

// pub use task::TaskControlBlock;
// pub use task::TaskStatus;

/// Thread control block
pub struct Thread {
    /// Immutable
    tid: Arc<TidHandle>,
    /// Mailbox for each thread
    mailbox: Mailbox,
    /// Signal trampoline(store ucontext)
    pub sig_trampoline: SignalTrampoline,
    /// The process this thread belongs to
    pub process: Arc<Process>,
    /// Thread local signals.
    /// TODO: should we lock?
    pub sig_queue: SpinNoIrqLock<SigQueue>,
    /// Mutable
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
    /// Thread cpu affinity
    pub cpu_set: CpuSet,
    /// Note that the process may modify this value in the another thread
    /// (e.g. `exec`)
    pub terminated: bool,
}

impl Thread {
    /// Construct a thread control block
    pub fn new(
        process: Arc<Process>,
        main_thread: Option<&Arc<Thread>>,
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
        let sig_queue = match main_thread {
            Some(main_thread) => SigQueue::from_another(&main_thread.sig_queue.lock()),
            None => SigQueue::new(),
        };
        let thread = Self {
            tid: tid.clone(),
            sig_trampoline,
            process: process.clone(),
            mailbox: Mailbox::new(),
            sig_queue: SpinNoIrqLock::new(sig_queue),
            // user_specified_stack,
            inner: UnsafeCell::new(ThreadInner {
                trap_context,
                signal_context: None,
                ustack_top,
                tid_addr: TidAddress::new(),
                time_info: ThreadTimeInfo::new(),
                waker: None,
                // TODO: need to change if multi_hart
                cpu_set: CpuSet::new(1),
                terminated: false,
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
        flags: CloneFlags,
    ) -> Self {
        stack_trace!();
        let sig_trampoline = SignalTrampoline::new(new_process.clone());
        let tid = match tid {
            Some(tid) => tid,
            None => Arc::new(tid_alloc()),
        };
        PROCESS_MANAGER.add(tid.0, &new_process);

        let sig_queue = match flags.contains(CloneFlags::CLONE_SIGHAND) {
            true => SigQueue::from_another(&another.sig_queue.lock()),
            false => SigQueue::new(),
        };
        Self {
            tid: tid.clone(),
            sig_trampoline,
            process: new_process.clone(),
            mailbox: Mailbox::new(),
            sig_queue: SpinNoIrqLock::new(sig_queue),
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
                // TODO: need to change if multi_hart
                cpu_set: CpuSet::new(1),
                terminated: false,
            }),
        }
    }

    /// We can get whatever we want in the inner by providing a handler
    pub unsafe fn inner_handler<T>(&self, f: impl FnOnce(&mut ThreadInner) -> T) -> T {
        f(&mut *self.inner.get())
    }

    /// Wait for some events
    pub async fn wait_for_events(&self, events: Event) -> Event {
        self.mailbox.wait_for_events(events).await
    }

    /// Send signal to this thread
    pub fn recv_signal(&self, signo: Signal) {
        stack_trace!();
        log::info!("[Thread::recv_signal] signo {}", signo);
        match signo {
            SIGKILL => {
                log::info!("[Thread::recv_signal] thread {} recv SIGKILL", self.tid(),);
                self.mailbox.send_event(Event::THREAD_EXIT);
            }
            SIGCHLD => {
                log::info!("[Thread::recv_signal] thread {} recv SIGCHLD", self.tid(),);
                self.mailbox.send_event(Event::CHILD_EXIT);
            }
            _ => {
                self.mailbox.send_event(Event::OTHER_SIGNAL);
            }
        };
        self.sig_queue.lock().recv_signal(signo)
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
        inner.terminated = true;
        // .store(true, core::sync::atomic::Ordering::Release)
    }

    /// Whether this thread has been terminated or not
    pub fn is_zombie(&self) -> bool {
        unsafe {
            (*self.inner.get()).terminated
            // .load(core::sync::atomic::Ordering::Acquire)
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
