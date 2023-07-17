use alloc::{
    collections::BTreeMap,
    sync::{Arc, Weak},
    vec::Vec,
};

use crate::{config::process::INITPROC_PID, sync::mutex::SpinNoIrqLock};
use lazy_static::*;

use super::Process;

/// tid -> process
pub struct ProcessManager(pub SpinNoIrqLock<BTreeMap<usize, Weak<Process>>>);

impl ProcessManager {
    pub fn new() -> Self {
        Self(SpinNoIrqLock::new(BTreeMap::new()))
    }

    pub fn add(&self, tid: usize, process: &Arc<Process>) {
        self.0.lock().insert(tid, Arc::downgrade(process));
    }

    pub fn remove(&self, tid: usize) {
        self.0.lock().remove(&tid);
    }

    pub fn get(&self, tid: usize) -> Option<Arc<Process>> {
        match self.0.lock().get(&tid) {
            Some(proc) => proc.upgrade(),
            None => None,
        }
    }

    // pub fn get_process_by_pid(&self, pid: usize) -> Option<Arc<Process>> {
    //     match self.0.lock().get(&pid) {
    //         Some(proc) => proc.upgrade(),
    //         None => None,
    //     }
    // }
    // pub fn get_process_by_tid(&self, tid: usize) -> Option<Arc<Process>> {
    //     match self.0.lock().range(..=tid).last() {
    //         Some((pid, proc)) => {
    //             log::info!("[get_process_by_tid] process pid {}", pid);
    //             proc.upgrade()
    //         }
    //         None => {
    //             log::warn!("[get_process_by_tid] process len {}", self.0.lock().len());
    //             None
    //         }
    //     }
    // }
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

/// gid -> pid
pub struct ProcessGroupManager(pub SpinNoIrqLock<BTreeMap<usize, Vec<usize>>>);

impl ProcessGroupManager {
    pub fn new() -> Self {
        Self(SpinNoIrqLock::new(BTreeMap::new()))
    }

    pub fn add_process(&self, pgid: usize, pid: usize) {
        let mut inner = self.0.lock();
        let vec = inner.get(&pgid);
        let mut vec = vec.cloned().unwrap();
        vec.push(pid);
        inner.insert(pgid, vec);
    }

    pub fn add_group(&self, pgid: usize) {
        let mut inner = self.0.lock();
        let mut vec: Vec<usize> = Vec::new();
        if pgid != INITPROC_PID {
            vec.push(pgid);
        }
        inner.insert(pgid, vec);
    }

    pub fn get_group_by_pgid(&self, pgid: usize) -> Vec<usize> {
        self.0.lock().get(&pgid).cloned().unwrap()
    }

    pub fn set_pgid_by_pid(&self, pid: usize, new_pgid: usize, old_pgid: usize) {
        let mut inner = self.0.lock();
        let mut old_group_vec = inner.get(&old_pgid).cloned().unwrap();
        old_group_vec.retain(|&x| x != pid);
        let new_group_vec = inner.get(&new_pgid).cloned();
        let new_group_vec = if new_group_vec.is_none() {
            let mut vec = Vec::new();
            vec.push(new_pgid);
            inner.insert(new_pgid, vec.clone());
            vec
        } else {
            let mut vec = new_group_vec.unwrap();
            vec.push(pid);
            vec
        };
        inner.insert(old_pgid, old_group_vec);
        inner.insert(new_pgid, new_group_vec);
    }
}

lazy_static! {
    /// Process group manager that used for a given pgid
    pub static ref PROCESS_GROUP_MANAGER: ProcessGroupManager = ProcessGroupManager::new();
}
