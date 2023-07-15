use alloc::collections::VecDeque;
use log::{debug, info};

use crate::{
    config::signal::SIG_NUM,
    processor::{current_process, current_task, current_trap_cx},
    trap::UserContext,
};

mod signal_context;
mod signal_handler;
pub use signal_context::SignalContext;
pub use signal_handler::SIG_DFL;
pub use signal_handler::SIG_ERR;
pub use signal_handler::SIG_IGN;

pub use self::signal_handler::{
    core_sig_handler, ign_sig_handler, stop_sig_handler, term_sig_handler,
};

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

bitflags! {
    pub struct SigSet: u32{
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
    }
}

pub struct SigHandlerManager {
    sigactions: [KSigAction; SIG_NUM],
}

impl SigHandlerManager {
    pub fn new() -> Self {
        let sigactions: [KSigAction; SIG_NUM] =
            core::array::from_fn(|signo| KSigAction::new(signo, false));
        Self { sigactions }
    }

    pub fn get(&self, signo: usize) -> Option<&KSigAction> {
        if signo < SIG_NUM {
            Some(&self.sigactions[signo])
        } else {
            None
        }
    }

    pub fn set_sigaction(&mut self, signo: usize, sigaction: KSigAction) {
        if signo < SIG_NUM {
            self.sigactions[signo] = sigaction;
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SigAction {
    pub sa_handler: fn(usize),
    pub sa_mask: [SigSet; 1],
    // pub sa_flags: usize,
    pub sa_restorer: usize,
    pub sa_flags: u32,
    // pub sa_sigaction: fn(i32, *const u8, *const u8),
    // pub sa_sigaction: usize,
    // pub sa_mask: [SigSet; 2],
    // pub sa_restorer: fn(),
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
        };
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
    pub fn new(signo: usize, is_user_defined: bool) -> Self {
        Self {
            is_user_defined,
            sig_action: SigAction::new(signo),
        }
    }
}

/// Note that we handle only one pending signal every time
pub fn check_signal_for_current_process() {
    // TODO: handle nesting sig handle:
    // Do we need to save trap contexts like a stack?
    if let Some((sig_info, sig_action, old_blocked_sigs)) =
        current_process().inner_handler(|proc| {
            if proc.pending_sigs.sig_queue.is_empty() {
                return None;
            }
            let sig_info = proc.pending_sigs.sig_queue.pop_front().unwrap();
            assert!(sig_info.signo < SIG_NUM);

            debug!("find a sig {}", sig_info.signo);

            let signo = sig_info.signo;

            let signo_shift = SigSet::from_bits(1 << (sig_info.signo - 1)).unwrap();

            debug!("current blocked sig: {:?}", proc.pending_sigs.blocked_sigs);

            if proc.pending_sigs.blocked_sigs.contains(signo_shift) {
                debug!("sig {} has been blocked", signo);
                return None;
            }

            let old_blocked_sigs = proc.pending_sigs.blocked_sigs;

            // save_context_for_sig_handler(proc.pending_sigs.blocked_sigs);

            proc.pending_sigs.blocked_sigs |= signo_shift;
            // TODO: only use the first element now
            proc.pending_sigs.blocked_sigs |= proc.sig_handler.lock().sigactions[sig_info.signo]
                .sig_action
                .sa_mask[0];

            Some((
                sig_info,
                proc.sig_handler.lock().sigactions[signo],
                old_blocked_sigs,
            ))
        })
    {
        // Note that serveral sig handlers may be executed at the same time by different threads
        // since we don't hold the process inner lock
        handle_signal(sig_info.signo, sig_action, old_blocked_sigs);
        // } else {
        //     break;
    }
    // }
}

extern "C" {
    fn sigreturn_trampoline();
}

fn handle_signal(signo: usize, sig_action: KSigAction, old_blocked_sigs: SigSet) {
    info!(
        "[handle_signal] signo {}, handler {:#x}",
        signo, sig_action.sig_action.sa_handler as *const usize as usize
    );
    if sig_action.is_user_defined {
        save_context_for_sig_handler(old_blocked_sigs);

        current_trap_cx().sepc = sig_action.sig_action.sa_handler as *const usize as usize;
        // a0
        current_trap_cx().user_x[10] = signo;
        // if sig_action.sig_action.sa_restorer != 0 {
        // ra
        info!(
            "[handle_signal] restorer: {:#x}",
            // sig_action.sig_action.sa_restorer
            sigreturn_trampoline as usize,
        );
        // current_trap_cx().user_x[1] = sig_action.sig_action.sa_restorer;
        current_trap_cx().user_x[1] = sigreturn_trampoline as usize;
        // }
    } else {
        // Just in kernel mode
        // TODO: change to async
        (sig_action.sig_action.sa_handler)(signo);
    }
    // current_trap_cx().sepc = sigaction_handler_wrapper as *const u8 as usize;
    // // a0
    // current_trap_cx().user_x[10] = sig_handler;
    // // a1
    // current_trap_cx().user_x[11] = signo;
}

fn save_context_for_sig_handler(blocked_sigs: SigSet) {
    // save old sig mask
    // and save old user trap context
    let signal_context = SignalContext {
        blocked_sigs,
        user_context: UserContext::from_trap_context(current_task().trap_context_ref()),
    };
    debug!(
        "[save_context_for_sig_handler] sepc {:#x}",
        signal_context.user_context.sepc
    );
    current_task().set_signal_context(signal_context);
}

pub struct SigInfo {
    pub signo: usize,
    pub errno: usize,
}

pub struct SigQueue {
    pub sig_queue: VecDeque<SigInfo>,
    pub blocked_sigs: SigSet,
}

impl SigQueue {
    pub fn new() -> Self {
        Self {
            sig_queue: VecDeque::new(),
            blocked_sigs: SigSet::from_bits(0).unwrap(),
        }
    }
    pub fn send_signal(&mut self, signo: usize) {
        self.sig_queue.push_back(SigInfo {
            signo: signo as usize,
            errno: 0,
        });
    }
}
