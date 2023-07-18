use alloc::collections::VecDeque;
use log::debug;

use super::{KSigAction, SigHandlerManager, SigInfo, SigSet};

pub struct SigQueue {
    /// Pending sigs
    pub pending_sigs: VecDeque<SigInfo>,
    /// Blocked sigs
    pub blocked_sigs: SigSet,
    /// Signal handlers for every signal
    pub sig_handlers: SigHandlerManager,
}

impl SigQueue {
    pub fn new() -> Self {
        Self {
            pending_sigs: VecDeque::new(),
            blocked_sigs: SigSet::from_bits(0).unwrap(),
            sig_handlers: SigHandlerManager::new(),
        }
    }
    pub fn from_another(pending_sigs: &SigQueue) -> Self {
        Self {
            pending_sigs: VecDeque::new(),
            blocked_sigs: SigSet::empty(),
            sig_handlers: pending_sigs.sig_handlers,
        }
    }
    pub fn send_signal(&mut self, signo: usize) {
        self.pending_sigs.push_back(SigInfo {
            signo: signo as usize,
            errno: 0,
        });
    }

    pub fn check_signal(&mut self) -> Option<(SigInfo, KSigAction, SigSet)> {
        if self.pending_sigs.is_empty() {
            return None;
        }
        // TODO: refactor sig queue to be a bit map,
        // in order to avoid repeated signo.
        let total_len = self.pending_sigs.len();
        let mut cnt = 0;
        while !self.pending_sigs.is_empty() {
            if cnt == total_len {
                return None;
            }
            let sig_info = self.pending_sigs.pop_front().unwrap();
            cnt += 1;
            let signo = sig_info.signo;
            let signo_shift = SigSet::from_bits(1 << (sig_info.signo - 1));
            if signo_shift.is_none() {
                log::error!("[check_signal] unsupported signal {}", sig_info.signo);
                continue;
            }
            let signo_shift = signo_shift.unwrap();

            if self.blocked_sigs.contains(signo_shift) {
                log::info!("sig {} has been blocked", signo);
                self.pending_sigs.push_back(sig_info);
                continue;
            }

            let old_blocked_sigs = self.blocked_sigs;

            // save_context_for_sig_handler(proc.pending_sigs.blocked_sigs);

            if self.sig_handlers.sigactions[signo].is_user_defined {
                self.blocked_sigs |= signo_shift;
                // TODO: only use the first element now
                self.blocked_sigs |= self.sig_handlers.sigactions[sig_info.signo]
                    .sig_action
                    .sa_mask[0];
            }

            return Some((
                sig_info,
                self.sig_handlers.sigactions[signo],
                old_blocked_sigs,
            ));
        }

        None
    }
}
