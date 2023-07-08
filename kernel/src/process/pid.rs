//!Implementation of [`TidAllocator`]
use crate::config::process::INITPROC_PID;
use crate::mm::RecycleAllocator;
use crate::sync::mutex::SpinNoIrqLock;
use lazy_static::*;
use log::debug;

lazy_static! {
    pub static ref PID_ALLOCATOR: SpinNoIrqLock<RecycleAllocator> =
        SpinNoIrqLock::new(RecycleAllocator::new(INITPROC_PID));
}
///Bind pid lifetime to `TidHandle`
pub struct TidHandle(pub usize);

impl Drop for TidHandle {
    fn drop(&mut self) {
        debug!("drop pid {}", self.0);
        // println!("\u{1B}[33m drop pid {} \u{1B}[0m", self.0);
        PID_ALLOCATOR.lock().dealloc(self.0);
    }
}
///Allocate a pid from PID_ALLOCATOR
pub fn tid_alloc() -> TidHandle {
    TidHandle(PID_ALLOCATOR.lock().alloc())
}
