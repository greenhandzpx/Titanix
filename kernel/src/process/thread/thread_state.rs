use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy, PartialEq)]
pub enum ThreadState {
    Runnable,
    Sleep,
    Stopped,
    Zombie,
}

pub struct ThreadStateAtomic(AtomicUsize);

impl ThreadStateAtomic {
    pub fn new() -> Self {
        Self(AtomicUsize::new(ThreadState::Runnable as usize))
    }

    pub fn store(&self, thread_state: ThreadState) {
        self.0.store(thread_state as usize, Ordering::Release);
    }

    pub fn load(&self) -> ThreadState {
        let inner = self.0.load(Ordering::Acquire);
        match inner {
            _ if inner == ThreadState::Runnable as usize => ThreadState::Runnable,
            _ if inner == ThreadState::Sleep as usize => ThreadState::Sleep,
            _ if inner == ThreadState::Stopped as usize => ThreadState::Stopped,
            _ if inner == ThreadState::Zombie as usize => ThreadState::Zombie,
            _ => panic!("Unknown thread state"),
        }
    }
}
