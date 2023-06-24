mod exit;
mod schedule;
#[allow(clippy::module_inception)]
mod thread_loop;
mod thread_resource;
mod thread_state;
mod tid;
mod time;

use self::{thread_state::{ThreadState, ThreadStateAtomic}, time::ThreadTimeInfo};

use super::Process;
use crate::executor;
use crate::signal::SignalContext;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use core::cell::UnsafeCell;
use core::future::Future;

pub use exit::{
    exit_and_terminate_all_threads, terminate_all_threads_except_main, terminate_given_thread,
};

use thread_loop::threadloop;
pub use tid::{TidAddress, TidHandle};

// pub use task::TaskControlBlock;
// pub use task::TaskStatus;

/// Thread control block
pub struct Thread {
    /// immutable
    pub tid: TidHandle,
    /// the process this thread belongs to
    pub process: Arc<Process>,
    // /// whether the user specify the stack
    // pub user_specified_stack: bool,
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
    /// When invoking `exec`, we need to get the ustack base.
    /// Note that ustack_base is the base of all ustacks
    pub ustack_base: usize,
    /// Thread state.
    /// Note that this may be modified by another thread, which
    /// need to be sync
    pub state: ThreadStateAtomic,
    /// Tid address, which may be modified by `set_tid_address` syscall
    pub tid_addr: Option<TidAddress>,
    /// 
    pub time_info: ThreadTimeInfo,
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
        ustack_base: usize,
        user_specified_stack: bool,
    ) -> Self {
        let res = Self {
            tid: process.alloc_tid(),
            process: process.clone(),
            // user_specified_stack,
            inner: UnsafeCell::new(ThreadInner {
                trap_context,
                signal_context: None,
                ustack_base,
                state: ThreadStateAtomic::new(),
                tid_addr: None,
                time_info: ThreadTimeInfo::new(),
                // terminated: AtomicBool::new(false),
            }),
        };
        res.alloc_ustack();
        // debug!("old ustack top {:#x}", trap_context.user_x[2]);
        // debug!("new ustack top {:#x}", res.ustack_top());
        if !user_specified_stack {
            unsafe {
                (*res.inner.get()).trap_context.set_sp(res.ustack_top());
            }
        }
        res
    }

    /// Construct a new thread from the current thread
    pub fn from_current(&self, new_process: Arc<Process>, stack: Option<usize>) -> Self {
        Self {
            tid: new_process.alloc_tid(),
            process: new_process.clone(),
            inner: UnsafeCell::new(ThreadInner {
                trap_context: {
                    let mut trap_context = self.trap_context();
                    if let Some(stack) = stack {
                        trap_context.set_sp(stack);
                    }
                    trap_context
                },
                signal_context: None,
                ustack_base: unsafe { (*self.inner.get()).ustack_base },
                state: ThreadStateAtomic::new(),
                tid_addr: None,
                time_info: ThreadTimeInfo::new(),
                // terminated: AtomicBool::new(false),
            }),
        }
    }

    /// Get the ref of signal context
    pub fn signal_context(&self) -> &SignalContext {
        unsafe { &(*self.inner.get()).signal_context.as_ref().unwrap() }
    }

    /// Set the signal context for the current thread
    pub fn set_signal_context(&self, signal_context: SignalContext) {
        unsafe {
            (*self.inner.get()).signal_context = Some(signal_context);
        }
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
    /// Wake up this thread
    pub fn wake_up(&self) {
        unsafe {
            (*self.inner.get()).state.store(ThreadState::Runnable);
        }
    }
    /// Tid of this thread
    pub fn tid(&self) -> usize {
        self.tid.0
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
