use alloc::collections::VecDeque;
use lazy_static::*;
use log::debug;

use crate::{
    config::signal::SIG_NUM,
    processor::{current_process, current_task, current_trap_cx},
    signal::signal_handler::default_sig_handler,
    trap::UserContext,
};

mod signal_context;
mod signal_handler;
pub use signal_context::SignalContext;
pub use signal_handler::SIG_DFL;
pub use signal_handler::SIG_IGN;
pub use signal_handler::SIG_ERR;

pub use self::signal_handler::{core_sig_handler, ign_sig_handler, stop_sig_handler, term_sig_handler};

// pub enum Signal {
pub const SIGHUP: usize = 1;
pub const SIGINT: usize = 2;
pub const SIGILL: usize = 4;
pub const SIGABRT: usize = 6;
pub const SIGBUS: usize = 7;
pub const SIGKILL: usize = 9;
pub const SIGSEGV: usize = 11;
pub const SIGALRM: usize = 14;
pub const SIGTERM: usize = 15;
pub const SIGCHLD: usize = 17;
pub const SIGSTOP: usize = 19;
// }

bitflags! {
    pub struct SigSet: usize {
        const SIGHUP = 1 << 1;
        const SIGINT = 1 << 2;
        const SIGILL = 1 << 4;
        const SIGABRT = 1 << 6;
        const SIGBUS = 1 << 7;
        const SIGKILL = 1 << 9;
        const SIGSEGV = 1 << 11;
        const SIGALRM = 1 << 14;
        const SIGTERM = 1 << 15;
        const SIGCHLD = 1 << 17;
        const SIGSTOP = 1 << 19;
        const SIGRTMIN = 1 << 32;
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
    pub sa_flags: u32,
    // pub sa_flags: usize,
    pub sa_restorer: usize,
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

pub fn check_signal_for_current_process() {
    loop {
        if let Some((sig_info, sig_action)) = current_process().inner_handler(|proc| {
            if proc.pending_sigs.sig_queue.is_empty() {
                return None;
            }
            let sig_info = proc.pending_sigs.sig_queue.pop_front().unwrap();
            assert!(sig_info.signo < SIG_NUM);

            debug!("find a sig {}", sig_info.signo);

            let signo = sig_info.signo;

            let signo_shift = SigSet::from_bits(1 << sig_info.signo).unwrap();

            if proc.pending_sigs.blocked_sigs.contains(signo_shift) {
                return None;
            }

            save_context_for_sig_handler(proc.pending_sigs.blocked_sigs);

            proc.pending_sigs.blocked_sigs |= signo_shift;
            // TODO: only use the first element now
            proc.pending_sigs.blocked_sigs |= proc.sig_handler.lock().sigactions[sig_info.signo]
                .sig_action
                .sa_mask[0];

            Some((sig_info, proc.sig_handler.lock().sigactions[signo]))
        }) {
            // Note that serveral sig handlers may be executed at the same time by different threads
            // since we don't hold the process inner lock
            handle_signal(sig_info.signo, sig_action);
        } else {
            break;
        }
    }
}

fn handle_signal(signo: usize, sig_action: KSigAction) {
    debug!("handle signal {}", signo);
    if sig_action.is_user_defined {
        current_trap_cx().sepc = sig_action.sig_action.sa_handler as *const usize as usize;
        // a0
        current_trap_cx().user_x[10] = signo;
        if sig_action.sig_action.sa_restorer != 0 {
            // ra
            current_trap_cx().user_x[1] = sig_action.sig_action.sa_restorer;
        }
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
