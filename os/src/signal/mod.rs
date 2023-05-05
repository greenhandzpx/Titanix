use alloc::{boxed::Box, collections::VecDeque};

use crate::{
    config::signal::SIG_NUM,
    mm::user_check::UserCheck,
    processor::{current_process, current_task, current_trap_cx},
    signal::{self, signal_handler::default_sig_handler},
    syscall::user_sigreturn,
    trap::UserContext,
};

mod signal_context;
mod signal_handler;
pub use signal_context::SignalContext;

use self::signal_handler::{core_sig_handler, ign_sig_handler, stop_sig_handler, term_sig_handler};

pub enum Signal {
    SIGHUP = 1,
    SIGINT = 2,
    SIGILL = 4,
    SIGABRT = 6,
    SIGBUS = 7,
    SIGKILL = 9,
    SIGSEGV = 11,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGCHLD = 17,
    SIGSTOP = 19,
}

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
    }
}

pub struct SigHandlerManager {
    sigactions: [SigAction; SIG_NUM],
}

impl SigHandlerManager {
    pub fn new() -> Self {
        let mut sigactions: [SigAction; SIG_NUM] = core::array::from_fn(|_| SigAction::new());
        sigactions[Signal::SIGABRT as usize].sa_handler = core_sig_handler;
        sigactions[Signal::SIGHUP as usize].sa_handler = term_sig_handler;
        sigactions[Signal::SIGINT as usize].sa_handler = term_sig_handler;
        sigactions[Signal::SIGKILL as usize].sa_handler = term_sig_handler;
        sigactions[Signal::SIGBUS as usize].sa_handler = core_sig_handler;
        sigactions[Signal::SIGSEGV as usize].sa_handler = core_sig_handler;
        sigactions[Signal::SIGSTOP as usize].sa_handler = stop_sig_handler;
        sigactions[Signal::SIGCHLD as usize].sa_handler = ign_sig_handler;
        sigactions[Signal::SIGALRM as usize].sa_handler = term_sig_handler;
        sigactions[Signal::SIGTERM as usize].sa_handler = term_sig_handler;
        sigactions[Signal::SIGILL as usize].sa_handler = core_sig_handler;
        Self { sigactions }
    }

    pub fn get(&self, signo: usize) -> Option<&SigAction> {
        if signo < SIG_NUM {
            Some(&self.sigactions[signo])
        } else {
            None
        }
    }

    pub fn set_sigaction(&mut self, signo: usize, sigaction: SigAction) {
        if signo < SIG_NUM {
            self.sigactions[signo] = sigaction;
        }
    }
}

#[derive(Clone, Copy)]
pub struct SigAction {
    pub sa_handler: fn(usize),
    pub sa_mask: SigSet,
}

impl SigAction {
    pub fn new() -> Self {
        Self {
            sa_handler: default_sig_handler,
            sa_mask: SigSet::from_bits(0).unwrap(),
        }
    }
}

pub fn check_signal_for_current_process() {
    loop {
        if let Some((sig_info, sig_handler)) = current_process().inner_handler(|proc| {
            if proc.pending_sigs.sig_queue.is_empty() {
                return None;
            }
            let sig_info = proc.pending_sigs.sig_queue.pop_front().unwrap();
            assert!(sig_info.signo < SIG_NUM);

            let signo = sig_info.signo;

            let signo_shift = SigSet::from_bits(1 << sig_info.signo).unwrap();

            if proc.pending_sigs.blocked_sigs.contains(signo_shift) {
                return None;
            }

            save_context_for_sig_handler(proc.pending_sigs.blocked_sigs);

            proc.pending_sigs.blocked_sigs |= signo_shift;
            proc.pending_sigs.blocked_sigs |=
                proc.sig_handler.lock().sigactions[sig_info.signo].sa_mask;

            Some((
                sig_info,
                proc.sig_handler.lock().sigactions[signo].sa_handler as *const u8 as usize,
            ))
        }) {
            // Note that serveral sig handlers may be executed at the same time by different threads
            // since we don't hold the process inner lock
            handle_signal(sig_info.signo, sig_handler);
        } else {
            break;
        }
    }
}

fn handle_signal(signo: usize, sig_handler: usize) {
    current_trap_cx().sepc = sig_handler;
    // a0
    current_trap_cx().user_x[10] = signo;
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
}
