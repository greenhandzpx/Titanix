use core::{future::Future, pin::Pin, task::Poll};

use alloc::{boxed::Box, vec::Vec, sync::Arc};

use super::mutex::SpinNoIrqLock;

bitflags! {
    /// 
    pub struct Event: u16 {
        /// Children exit
        const CHILD_EXIT = 1 << 0;
        /// Parent exit
        const PARENT_EXIT = 1 << 1;
    }
}

type EventCallback = Box<dyn Fn(Event) -> (bool, Event) + Send>;

type Mutex<T> = SpinNoIrqLock<T>;

/// Mailbox is used to communicate between processes
pub struct Mailbox {
    inner: Mutex<MailboxInner>,
}

pub struct MailboxInner {
    events: Event,
    callbacks: Vec<EventCallback>,
}

impl Mailbox {

    /// Construct a mailbox
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(MailboxInner {
                events: Event::empty(),
                callbacks: Vec::new(),
            }),
        }
    }

    ///
    pub fn send_event(&self, event: Event) {
        let mut inner = self.inner.lock();
        if inner.events | event != inner.events {
            inner.events |= event;
            let new_event = inner.events;
            // let mut consumed_events = Event::empty();
            inner.callbacks.retain(|e| {
                let (care, _event) = e(new_event);
                // consumed_events |= event; 
                !care
            });
            // inner.events.remove(consumed_events);
        }
    }

    ///
    pub async fn wait_for_event(self: &Arc<Self>, event: Event) {
        WaitForEventFuture::new(event, self.clone()).await
    }
}



struct WaitForEventFuture {
    mailbox: Arc<Mailbox>,
    event: Event,
}

impl WaitForEventFuture {
    pub fn new(event: Event, mailbox: Arc<Mailbox>) -> Self {
        Self {
            mailbox,
            event,
        }
    }
}


impl Future for WaitForEventFuture {

    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.mailbox.inner.lock();
        if inner.events & self.event != Event::empty() {
            inner.events.remove(self.event);
            return Poll::Ready(());
        }
        let self_event = self.event;
        let waker = cx.waker().clone();
        inner.callbacks.push(Box::new(move |events| {
            if events & self_event  != Event::empty() {
                // the events contain what we want
                waker.wake_by_ref();
                (true, self_event)
            } else {
                (false, Event::empty())
            }
        }));
        Poll::Pending
    }
}