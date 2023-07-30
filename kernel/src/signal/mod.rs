use log::{debug, info};

use crate::{
    config::signal::SIG_NUM,
    processor::{current_task, current_trap_cx},
    trap::UserContext,
};

mod ctx;
mod handler;
pub mod signal_queue;
pub use ctx::SignalContext;
pub use ctx::SignalTrampoline;
pub use handler::SIG_DFL;
pub use handler::SIG_ERR;
pub use handler::SIG_IGN;

pub use self::handler::{core_sig_handler, ign_sig_handler, stop_sig_handler, term_sig_handler};

pub type Signal = usize;

pub const SIGHUP: usize = 1;
pub const SIGINT: usize = 2;
pub const SIGQUIT: usize = 3;
pub const SIGILL: usize = 4;
pub const SIGTRAP: usize = 5;
pub const SIGABRT: usize = 6;
pub const SIGBUS: usize = 7;
pub const SIGFPE: usize = 8;
pub const SIGKILL: usize = 9;
pub const SIGUSR1: usize = 10;
pub const SIGSEGV: usize = 11;
pub const SIGUSR2: usize = 12;
pub const SIGPIPE: usize = 13;
pub const SIGALRM: usize = 14;
pub const SIGTERM: usize = 15;
pub const SIGSTKFLT: usize = 16;
pub const SIGCHLD: usize = 17;
pub const SIGCONT: usize = 18;
pub const SIGSTOP: usize = 19;
pub const SIGTSTP: usize = 20;
pub const SIGTTIN: usize = 21;
pub const SIGTTOU: usize = 22;
pub const SIGURG: usize = 23;
pub const SIGXCPU: usize = 24;
pub const SIGXFSZ: usize = 25;
pub const SIGVTALRM: usize = 26;
pub const SIGPROF: usize = 27;
pub const SIGWINCH: usize = 28;
pub const SIGIO: usize = 29;
pub const SIGPWR: usize = 30;
pub const SIGSYS: usize = 31;
pub const SIGRTMIN: usize = 32;
pub const SIGRT_1: usize = SIGRTMIN + 1;

bitflags! {
    // TODO: not sure u64 or u32
    pub struct SigSet: usize {
        const SIGHUP    = 1 << (SIGHUP -1);
        const SIGINT    = 1 << (SIGINT - 1);
        const SIGQUIT   = 1 << (SIGQUIT - 1);
        const SIGILL    = 1 << (SIGILL - 1);
        const SIGTRAP   = 1 << (SIGTRAP - 1);
        const SIGABRT   = 1 << (SIGABRT - 1);
        const SIGBUS    = 1 << (SIGBUS - 1);
        const SIGFPE    = 1 << (SIGFPE - 1);
        const SIGKILL   = 1 << (SIGKILL - 1);
        const SIGUSR1   = 1 << (SIGUSR1 - 1);
        const SIGSEGV   = 1 << (SIGSEGV - 1);
        const SIGUSR2   = 1 << (SIGUSR2 - 1);
        const SIGPIPE   = 1 << (SIGPIPE - 1);
        const SIGALRM   = 1 << (SIGALRM - 1);
        const SIGTERM   = 1 << (SIGTERM - 1);
        const SIGSTKFLT = 1 << (SIGSTKFLT- 1);
        const SIGCHLD   = 1 << (SIGCHLD - 1);
        const SIGCONT   = 1 << (SIGCONT - 1);
        const SIGSTOP   = 1 << (SIGSTOP - 1);
        const SIGTSTP   = 1 << (SIGTSTP - 1);
        const SIGTTIN   = 1 << (SIGTTIN - 1);
        const SIGTTOU   = 1 << (SIGTTOU - 1);
        const SIGURG    = 1 << (SIGURG - 1);
        const SIGXCPU   = 1 << (SIGXCPU - 1);
        const SIGXFSZ   = 1 << (SIGXFSZ - 1);
        const SIGVTALRM = 1 << (SIGVTALRM - 1);
        const SIGPROF   = 1 << (SIGPROF - 1);
        const SIGWINCH  = 1 << (SIGWINCH - 1);
        const SIGIO     = 1 << (SIGIO - 1);
        const SIGPWR    = 1 << (SIGPWR - 1);
        const SIGSYS    = 1 << (SIGSYS - 1);
        const SIGRTMIN  = 1 << (SIGRTMIN- 1);
        const SIGRT_1   = 1 << (SIGRT_1 - 1);
    }
}

impl SigSet {
    pub fn add_sig(&mut self, signo: usize) {
        self.insert(SigSet::from_bits(1 << (signo - 1)).unwrap());
    }

    pub fn contain_sig(&self, signo: usize) -> bool {
        self.contains(SigSet::from_bits(1 << (signo - 1)).unwrap())
    }

    pub fn remove_sig(&mut self, signo: usize) {
        self.remove(SigSet::from_bits(1 << (signo - 1)).unwrap())
    }
}

#[derive(Clone, Copy)]
pub struct SigHandlerManager {
    sigactions: [KSigAction; SIG_NUM + 1],
}

impl SigHandlerManager {
    pub fn new() -> Self {
        let sigactions: [KSigAction; SIG_NUM + 1] =
            core::array::from_fn(|signo| KSigAction::new(signo, false));
        Self { sigactions }
    }

    pub fn get(&self, signo: Signal) -> Option<&KSigAction> {
        if signo <= SIG_NUM {
            Some(&self.sigactions[signo])
        } else {
            None
        }
    }

    pub fn set_sigaction(&mut self, signo: Signal, sigaction: KSigAction) {
        if signo <= SIG_NUM {
            self.sigactions[signo] = sigaction;
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SigAction {
    pub sa_handler: usize,
    // pub sa_handler: usize,
    pub sa_flags: u32,
    pub sa_restorer: usize,
    pub sa_mask: [SigSet; 1],
}

impl SigAction {
    pub fn new(signo: usize) -> Self {
        let sa_handler = match signo {
            SIGHUP => term_sig_handler,
            SIGINT => term_sig_handler,
            SIGILL => core_sig_handler,
            SIGABRT => core_sig_handler,
            SIGBUS => core_sig_handler,
            SIGKILL => term_sig_handler,
            SIGSEGV => core_sig_handler,
            SIGALRM => term_sig_handler,
            SIGTERM => term_sig_handler,
            SIGCHLD => ign_sig_handler,
            SIGSTOP => stop_sig_handler,
            _ => ign_sig_handler,
        } as *const () as usize;
        Self {
            sa_handler,
            // sa_sigaction: 0,
            sa_mask: [SigSet::from_bits(0).unwrap(); 1],
            sa_flags: 0,
            sa_restorer: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct KSigAction {
    pub sig_action: SigAction,
    pub is_user_defined: bool,
}

impl KSigAction {
    /// Construct a default handler
    pub fn new(signo: Signal, is_user_defined: bool) -> Self {
        Self {
            is_user_defined,
            sig_action: SigAction::new(signo),
        }
    }
}

// /// Note that we handle only one pending signal every time.
// /// Return true if there is a user-defined pending sig to handle.
// pub fn check_signal_for_current_process() -> bool {
//     // TODO: handle nesting sig handle:
//     // Do we need to save trap contexts like a stack?
//     if let Some((signo, sig_action, old_blocked_sigs)) =
//         current_process().inner_handler(|proc| proc.sig_queue.check_signal())
//     {
//         // Note that serveral sig handlers may be executed at the same time by different threads
//         // since we don't hold the process inner lock
//         handle_signal(signo, sig_action, old_blocked_sigs)
//     } else {
//         false
//     }
// }

/// Note that we handle only one pending signal every time.
/// Return true if there is a user-defined pending sig to handle.
pub fn check_signal_for_current_task() -> bool {
    // TODO: handle nesting sig handle:
    // Do we need to save trap contexts like a stack?
    if let Some((signo, sig_action, old_blocked_sigs)) =
        current_task().sig_queue.lock().check_signal()
    {
        log::info!("[check_signal_for_current_thread] handle signal {}", signo);
        // Note that serveral sig handlers may be executed at the same time by different threads
        // since we don't hold the process inner lock
        handle_signal(signo, sig_action, old_blocked_sigs)
    } else {
        false
    }
}

extern "C" {
    fn sigreturn_trampoline();
}

/// Return true if there is a user-defined pending sig to handle
fn handle_signal(signo: Signal, sig_action: KSigAction, old_blocked_sigs: SigSet) -> bool {
    info!(
        "[handle_signal] signo {}, handler {:#x}",
        signo, sig_action.sig_action.sa_handler as *const usize as usize
    );
    if sig_action.is_user_defined {
        save_context_for_sig_handler(old_blocked_sigs);

        if signo == SIGSEGV {
            log::warn!("[handle_signal] user set handler for SIGSEGV");
        }

        current_trap_cx().sepc = sig_action.sig_action.sa_handler as *const usize as usize;
        // a0
        current_trap_cx().user_x[10] = signo;
        // a2 -> user context
        current_trap_cx().user_x[12] = current_task().sig_trampoline.user_addr();
        // ra
        current_trap_cx().user_x[1] = sigreturn_trampoline as usize;
        info!(
            "[handle_signal] restorer: {:#x}",
            // sig_action.sig_action.sa_restorer
            sigreturn_trampoline as usize,
        );
        true
    } else {
        // Just in kernel mode
        // TODO: change to async
        let handler = unsafe {
            core::mem::transmute::<*const (), fn(usize)>(
                sig_action.sig_action.sa_handler as *const (),
            )
        };
        handler(signo);
        false
    }
}

fn save_context_for_sig_handler(blocked_sigs: SigSet) {
    // Save old sig mask
    // and save old user trap context
    log::debug!(
        "[save_context_for_sig_handler] old blocked sigs {:?}",
        blocked_sigs
    );

    // Save float regs if needed
    current_trap_cx().user_fx.encounter_signal();

    let mut signal_context = SignalContext::new(
        blocked_sigs,
        UserContext::from_trap_context(current_trap_cx()),
    );

    signal_context.user_context.user_x[0] = signal_context.user_context.sepc;
    debug!(
        "[save_context_for_sig_handler] sepc {:#x}",
        signal_context.user_context.sepc
    );
    current_task().set_signal_context(signal_context);
}
