use core::task::Waker;

use alloc::{collections::VecDeque, sync::Weak};

use crate::{fs::File, sync::mutex::SpinNoIrqLock};

pub static POLL_QUEUE: PollQueue = PollQueue::new();

pub fn init() {
    POLL_QUEUE.init();
}

/// We will start a kernel thread to poll this queue
/// again and again when the kernel started.
/// Note that this is just a work-around for those devices
/// that cannot send interrupt by themselves.
pub struct PollQueue {
    inner: SpinNoIrqLock<PollQueueInner>,
}

struct PollQueueInner {
    queue: Option<VecDeque<PollEvent>>,
}

struct PollEvent {
    file: Weak<dyn File>,
    for_read: bool,
    waker: Waker,
}

impl PollQueue {
    pub const fn new() -> Self {
        Self {
            inner: SpinNoIrqLock::new(PollQueueInner { queue: None }),
        }
    }

    pub fn init(&self) {
        let mut inner = self.inner.lock();
        inner.queue = Some(VecDeque::new());
        // inner.waker = Some(waker);
    }

    pub fn register(&self, file: Weak<dyn File>, waker: Waker, for_read: bool) {
        let mut inner = self.inner.lock();
        inner.queue.as_mut().unwrap().push_back(PollEvent {
            file,
            for_read,
            waker,
        });
        // inner.waker.as_ref().unwrap().wake_by_ref();
    }

    pub fn poll(&self) {
        let mut inner = self.inner.lock();
        let totol_num = inner.queue.as_mut().unwrap().len();
        let mut cnt = 0;
        while !inner.queue.as_mut().unwrap().is_empty() {
            if cnt == totol_num {
                break;
            }
            cnt += 1;
            let event = inner.queue.as_mut().unwrap().pop_back().unwrap();
            if let Some(file) = event.file.upgrade() {
                let ret = match event.for_read {
                    true => file.pollin(None),
                    false => file.pollout(None),
                };
                if let Some(ret) = ret.ok() && ret {
                    event.waker.wake_by_ref();
                } else {
                    inner.queue.as_mut().unwrap().push_back(event);
                }
            }
        }
    }
}
