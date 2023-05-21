//!Implementation of [`PidAllocator`]
use crate::mm::RecycleAllocator;
use crate::sync::mutex::SpinNoIrqLock;
use lazy_static::*;
use log::debug;

lazy_static! {
    pub static ref PID_ALLOCATOR: SpinNoIrqLock<RecycleAllocator> =
        SpinNoIrqLock::new(RecycleAllocator::new(1));
}
///Bind pid lifetime to `PidHandle`
pub struct PidHandle(pub usize);

impl Drop for PidHandle {
    fn drop(&mut self) {
        debug!("drop pid {}", self.0);
        // println!("\u{1B}[33m drop pid {} \u{1B}[0m", self.0);
        PID_ALLOCATOR.lock().dealloc(self.0);
    }
}
///Allocate a pid from PID_ALLOCATOR
pub fn pid_alloc() -> PidHandle {
    PidHandle(PID_ALLOCATOR.lock().alloc())
}
