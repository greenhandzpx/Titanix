use alloc::collections::VecDeque;
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
        const SIGRTMIN = 1 << 32;
    }
}

pub struct SigHandlerManager {
    sigactions: [SigActionKernel; SIG_NUM],
}

impl SigHandlerManager {
    pub fn new() -> Self {
        let mut sigactions: [SigActionKernel; SIG_NUM] =
            core::array::from_fn(|_| SigActionKernel::new(false));
        sigactions[Signal::SIGABRT as usize].sig_action.sa_handler = core_sig_handler;
        sigactions[Signal::SIGHUP as usize].sig_action.sa_handler = term_sig_handler;
        sigactions[Signal::SIGINT as usize].sig_action.sa_handler = term_sig_handler;
        sigactions[Signal::SIGKILL as usize].sig_action.sa_handler = term_sig_handler;
        sigactions[Signal::SIGBUS as usize].sig_action.sa_handler = core_sig_handler;
        sigactions[Signal::SIGSEGV as usize].sig_action.sa_handler = core_sig_handler;
        sigactions[Signal::SIGSTOP as usize].sig_action.sa_handler = stop_sig_handler;
        sigactions[Signal::SIGCHLD as usize].sig_action.sa_handler = ign_sig_handler;
        sigactions[Signal::SIGALRM as usize].sig_action.sa_handler = term_sig_handler;
        sigactions[Signal::SIGTERM as usize].sig_action.sa_handler = term_sig_handler;
        sigactions[Signal::SIGILL as usize].sig_action.sa_handler = core_sig_handler;
        Self { sigactions }
    }

    pub fn get(&self, signo: usize) -> Option<&SigActionKernel> {
        if signo < SIG_NUM {
            Some(&self.sigactions[signo])
        } else {
            None
        }
    }

    pub fn set_sigaction(&mut self, signo: usize, sigaction: SigActionKernel) {
        if signo < SIG_NUM {
            self.sigactions[signo] = sigaction;
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SigAction {
    pub sa_handler: fn(usize),
    pub sa_flags: usize,
    // pub sa_sigaction: fn(i32, *const u8, *const u8),
    // pub sa_sigaction: usize,
    pub sa_restorer: usize,
    pub sa_mask: [SigSet; 1],
    // pub sa_mask: [SigSet; 2],
    // pub sa_restorer: fn(),
}

impl SigAction {
    pub fn new() -> Self {
        Self {
            sa_handler: default_sig_handler,
            // sa_sigaction: 0,
            sa_mask: [SigSet::from_bits(0).unwrap(); 1],
            sa_flags: 0,
            sa_restorer: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SigActionKernel {
    pub sig_action: SigAction,
    pub is_user_defined: bool,
}

impl SigActionKernel {
    pub fn new(is_user_defined: bool) -> Self {
        Self {
            is_user_defined,
            sig_action: SigAction::new(),
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

fn handle_signal(signo: usize, sig_action: SigActionKernel) {
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
    pub fn send_signal(&mut self, sig: Signal) {
        self.sig_queue.push_back(SigInfo {
            signo: sig as usize,
            errno: 0,
        });
    }
}
