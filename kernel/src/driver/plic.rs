use core::{ops::BitAnd, panic};
use alloc::sync::Arc;

use crate::{driver::Mutex, println, sync::mutex::SpinNoIrqLock, utils::error::GeneralRet};

pub struct PLIC {
    base_addr: usize,
}

impl PLIC {
    pub fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }
    fn priority_ptr(&self, intr_id: usize) -> *mut u32 {
        (self.base_addr + intr_id * 4) as *mut u32
    }
    pub fn priority(&self, intr_id: usize) -> u32 {
        unsafe { self.priority_ptr(intr_id).read_volatile() & 7 }
    }
    pub fn set_priority(&mut self, intr_id: usize, priority: u32) {
        unsafe { self.priority_ptr(intr_id).write_volatile(priority & 7); }
    }
    fn pending_ptr(&self, intr_id: usize) -> *mut u32 {
        (self.base_addr + 0x1000 + 4 * (intr_id / 32)) as *mut u32
    }
    pub fn pending(&self, intr_id: usize) -> bool {
        unsafe { ((self.pending_ptr(intr_id).read_volatile() >> (intr_id % 32)) & 1) == 1 }
    }
    fn intr_enable_ptr(&self, intr_id: usize, context_id: usize) -> *mut u32 {
        (self.base_addr + 0x2000 + 0x80 * context_id + 4 * (intr_id / 32)) as *mut u32
    }
    pub fn intr_enable(&self, intr_id: usize, context_id: usize) -> bool {
        unsafe { ((self.intr_enable_ptr(intr_id, context_id).read_volatile() >> (intr_id % 32)) & 1) == 1 }
    }
    pub fn set_intr_enable(&mut self, intr_id: usize, context_id: usize) {
        let ptr = self.intr_enable_ptr(intr_id, context_id);
        unsafe {
            let r = ptr.read_volatile();
            ptr.write_volatile(r | (1 << (intr_id % 32)));
        }
    }
    pub fn set_intr_disable(&mut self, intr_id: usize, context_id: usize) {
        let ptr = self.intr_enable_ptr(intr_id, context_id);
        unsafe {
            let r = ptr.read_volatile();
            ptr.write_volatile(r & !(1 << (intr_id % 32)));
        }
    }
    fn threshold_ptr(&self, context_id: usize) -> *mut u32 {
        (self.base_addr + 0x200000 + 0x1000 * context_id) as *mut u32
    }
    pub fn threshold(&self, context_id: usize) -> u32 {
        unsafe { self.threshold_ptr(context_id).read_volatile() & 7 }
    }
    pub fn set_threshold(&mut self, context_id: usize, threshold: u32) {
        unsafe { self.threshold_ptr(context_id).write_volatile(threshold & 7); }
    }
    fn claim_complete_ptr(&self, context_id: usize) -> *mut u32 {
        (self.base_addr + 0x200000 + 0x1000 * context_id + 4) as *mut u32
    }
    fn claim(&mut self, context_id: usize) -> usize {
        unsafe { self.claim_complete_ptr(context_id).read_volatile() as usize }
    }
    fn complete(&mut self, context_id: usize, intr_id: usize) {
        unsafe { self.claim_complete_ptr(context_id).write_volatile(intr_id as u32); }
    }
    
}
