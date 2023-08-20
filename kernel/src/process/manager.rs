use alloc::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};

use crate::{
    config::process::INITPROC_PID, stack_trace, sync::mutex::SpinNoIrqLock,
    utils::error::GeneralRet,
};

use super::Process;

type Tid = usize;
type Pid = usize;
type Gid = usize;

/// tid -> process
pub struct ProcessManager(SpinNoIrqLock<BTreeMap<usize, Weak<Process>>>);

impl ProcessManager {
    pub const fn new() -> Self {
        Self(SpinNoIrqLock::new(BTreeMap::new()))
    }

    pub fn add(&self, tid: Tid, process: &Arc<Process>) {
        stack_trace!();
        self.0.lock().insert(tid, Arc::downgrade(process));
    }

    pub fn remove(&self, tid: Tid) {
        stack_trace!();
        self.0.lock().remove(&tid);
    }

    pub fn get(&self, tid: Tid) -> Option<Arc<Process>> {
        stack_trace!();
        match self.0.lock().get(&tid) {
            Some(proc) => proc.upgrade(),
            None => None,
        }
    }

    /// Get the init process
    pub fn init_proc(&self) -> Arc<Process> {
        stack_trace!();
        self.0.lock().get(&INITPROC_PID).unwrap().upgrade().unwrap()
    }

    pub fn total_num(&self) -> usize {
        stack_trace!();
        let mut cnt = 0;
        let mut pids: BTreeSet<usize> = BTreeSet::new();
        for (_, p) in self.0.lock().iter() {
            if let Some(p) = p.upgrade() {
                if pids.get(&p.pid()).is_some() {
                    continue;
                }
                pids.insert(p.pid());
                cnt += 1;
            }
        }
        cnt
    }

    pub fn for_each(&self, f: impl Fn(&Arc<Process>) -> GeneralRet<()>) -> GeneralRet<()> {
        stack_trace!();
        let mut pids: BTreeSet<usize> = BTreeSet::new();
        for (_, p) in self.0.lock().iter() {
            if let Some(p) = p.upgrade() {
                if pids.get(&p.pid()).is_some() {
                    continue;
                }
                pids.insert(p.pid());
                f(&p)?
            }
        }
        Ok(())
    }
}

/// Process manager that used for looking for a given process
pub static PROCESS_MANAGER: ProcessManager = ProcessManager::new();

/// gid -> pid
pub struct ProcessGroupManager(pub SpinNoIrqLock<BTreeMap<Gid, Vec<Pid>>>);

impl ProcessGroupManager {
    pub const fn new() -> Self {
        Self(SpinNoIrqLock::new(BTreeMap::new()))
    }

    pub fn add_process(&self, pgid: Gid, pid: Pid) {
        stack_trace!();
        let mut inner = self.0.lock();
        let vec = inner.get(&pgid);
        let mut vec = vec.cloned().unwrap();
        vec.push(pid);
        inner.insert(pgid, vec);
    }

    pub fn add_group(&self, pgid: Gid) {
        stack_trace!();
        let mut inner = self.0.lock();
        let mut vec: Vec<usize> = Vec::new();
        if pgid != INITPROC_PID {
            vec.push(pgid);
        }
        inner.insert(pgid, vec);
    }

    pub fn get_group_by_pgid(&self, pgid: Gid) -> Vec<usize> {
        stack_trace!();
        self.0.lock().get(&pgid).cloned().unwrap()
    }

    pub fn set_pgid_by_pid(&self, pid: Pid, new_pgid: Gid, old_pgid: Gid) {
        stack_trace!();
        let mut inner = self.0.lock();
        let old_group_vec = inner.get_mut(&old_pgid).unwrap();
        old_group_vec.retain(|&x| x != pid);
        let new_group_vec = inner.get_mut(&new_pgid);
        if let Some(new_group_vec) = new_group_vec {
            new_group_vec.push(pid);
        } else {
            let new_group: Vec<usize> = vec![new_pgid];
            // let mut new_group = Vec::new();
            // new_group.push(new_pgid);
            inner.insert(new_pgid, new_group);
        }
    }
}

/// Process group manager that used for a given pgid
pub static PROCESS_GROUP_MANAGER: ProcessGroupManager = ProcessGroupManager::new();
