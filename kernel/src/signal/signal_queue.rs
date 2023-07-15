use alloc::collections::VecDeque;
use log::debug;

use crate::config::signal::SIG_NUM;

use super::{KSigAction, SigHandlerManager, SigInfo, SigSet};

pub struct SigQueue {
    /// Pending sigs
    pub sig_queue: VecDeque<SigInfo>,
    /// Blocked sigs
    /// Signal handlers for every signal
    pub blocked_sigs: SigSet,
    pub sig_handlers: SigHandlerManager,
}

impl SigQueue {
    pub fn new() -> Self {
        Self {
            sig_queue: VecDeque::new(),
            blocked_sigs: SigSet::from_bits(0).unwrap(),
            sig_handlers: SigHandlerManager::new(),
        }
    }
    pub fn from_another(sig_queue: &SigQueue) -> Self {
        Self {
            sig_queue: VecDeque::new(),
            blocked_sigs: SigSet::empty(),
            sig_handlers: sig_queue.sig_handlers,
        }
    }
    pub fn send_signal(&mut self, signo: usize) {
        self.sig_queue.push_back(SigInfo {
            signo: signo as usize,
            errno: 0,
        });
    }

    pub fn check_signal(&mut self) -> Option<(SigInfo, KSigAction, SigSet)> {
        if self.sig_queue.is_empty() {
            return None;
        }
        let sig_info = self.sig_queue.pop_front().unwrap();
        assert!(sig_info.signo <= SIG_NUM);

        debug!("find a sig {}", sig_info.signo);

        let signo = sig_info.signo;

        let signo_shift = SigSet::from_bits(1 << (sig_info.signo - 1)).unwrap();

        if self.blocked_sigs.contains(signo_shift) {
            debug!("sig {} has been blocked", signo);
            return None;
        }

        let old_blocked_sigs = self.blocked_sigs;

        // save_context_for_sig_handler(proc.pending_sigs.blocked_sigs);

        self.blocked_sigs |= signo_shift;
        // TODO: only use the first element now
        self.blocked_sigs |= self.sig_handlers.sigactions[sig_info.signo]
            .sig_action
            .sa_mask[0];

        Some((
            sig_info,
            self.sig_handlers.sigactions[signo],
            old_blocked_sigs,
        ))
    }
}
