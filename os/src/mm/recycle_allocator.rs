use alloc::vec::Vec;

/// Used for allocating pid & tid
pub struct RecycleAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl RecycleAllocator {
    ///Create an empty `RecycleAllocator`
    pub fn new() -> Self {
        RecycleAllocator {
            current: 0,
            // TODO: use heap to replace vec
            recycled: Vec::new(),
        }
    }
    ///Allocate an id
    pub fn alloc(&mut self) -> usize {
        if let Some(id) = self.recycled.pop() {
            id
        } else {
            self.current += 1;
            self.current - 1
        }
    }
    ///Recycle an id
    pub fn dealloc(&mut self, id: usize) {
        assert!(id < self.current);
        assert!(
            !self.recycled.iter().any(|iid| *iid == id),
            "id {} has been deallocated!",
            id
        );
        self.recycled.push(id);
    }
}
