use alloc::vec::Vec;

use crate::stack_trace;

/// Used for allocating pid & tid
pub struct RecycleAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl RecycleAllocator {
    ///Create an empty `RecycleAllocator`
    pub const fn new(init_val: usize) -> Self {
        RecycleAllocator {
            current: init_val,
            // TODO: use heap to replace vec
            recycled: Vec::new(),
        }
    }
    ///Allocate an id
    pub fn alloc(&mut self) -> usize {
        stack_trace!();
        if let Some(id) = self.recycled.pop() {
            id
        } else {
            self.current += 1;
            self.current - 1
        }
    }
    ///Recycle an id
    pub fn dealloc(&mut self, id: usize) {
        stack_trace!();
        assert!(id < self.current);
        assert!(
            !self.recycled.iter().any(|iid| *iid == id),
            "id {} has been deallocated!",
            id
        );
        self.recycled.push(id);
    }
}
