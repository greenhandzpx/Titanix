use alloc::{collections::BTreeMap, sync::Weak};

use crate::sync::mutex::SpinNoIrqLock;
use lazy_static::*;

use super::Process;

pub struct ProcessManager(pub BTreeMap<usize, Weak<Process>>);

impl ProcessManager {}

lazy_static! {
    /// Process manager that used for looking for a given process
    pub static ref PROCESS_MANAGER: SpinNoIrqLock<ProcessManager> =
        SpinNoIrqLock::new(ProcessManager(BTreeMap::new()));
}
