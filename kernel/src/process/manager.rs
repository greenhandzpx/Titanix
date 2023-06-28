use alloc::{
    collections::BTreeMap,
    sync::{Arc, Weak},
};

use crate::{config::process::INITPROC_PID, sync::mutex::SpinNoIrqLock};
use lazy_static::*;

use super::Process;

/// pid -> process
pub struct ProcessManager(pub SpinNoIrqLock<BTreeMap<usize, Weak<Process>>>);

impl ProcessManager {
    pub fn new() -> Self {
        Self(SpinNoIrqLock::new(BTreeMap::new()))
    }

    pub fn add_process(&self, pid: usize, process: &Arc<Process>) {
        self.0.lock().insert(pid, Arc::downgrade(process));
    }

    pub fn get_process_by_pid(&self, pid: usize) -> Option<Arc<Process>> {
        match self.0.lock().get(&pid) {
            Some(proc) => proc.upgrade(),
            None => None,
        }
    }
    /// Get the init process
    pub fn init_proc(&self) -> Arc<Process> {
        self.0.lock().get(&INITPROC_PID).unwrap().upgrade().unwrap()
    }

    pub fn init_proc_weak(&self) -> Weak<Process> {
        self.0.lock().get(&INITPROC_PID).unwrap().clone()
    }
}

lazy_static! {
    /// Process manager that used for looking for a given process
    pub static ref PROCESS_MANAGER: ProcessManager = ProcessManager::new();
}
