use core::{
    future::Future,
    pin::Pin,
    task::{Poll, Waker},
};

use alloc::vec::Vec;

use crate::stack_trace;

use super::mutex::SpinNoIrqLock;

bitflags! {
    ///
    pub struct Event: u16 {
        /// Children exit
        const CHILD_EXIT = 1 << 0;
        /// Self exit (thread)
        const THREAD_EXIT = 1 << 1;
        /// Self exit (process)
        const PROCESS_EXIT = 1 << 2;
        /// Other kinds of signal
        const OTHER_SIGNAL = 1 << 3;
    }
}

// type EventCallback = Box<dyn Fn(Event) -> (bool, Event) + Send>;

type Mutex<T> = SpinNoIrqLock<T>;

/// Mailbox is used to communicate between processes
pub struct Mailbox {
    inner: Mutex<MailboxInner>,
}

pub struct MailboxInner {
    events: Event,
    // callbacks: Vec<EventCallback>,
    callbacks: Vec<(Event, Waker)>,
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

    /// Send event to this mailbox
    pub fn recv_event(&self, event: Event) {
        stack_trace!();
        log::info!("[send_event] send event {:?}...", event);
        let mut inner = self.inner.lock();
        log::debug!("[send_event] callback len {}", inner.callbacks.len());
        // if inner.events | event != inner.events {
        inner.events |= event;
        let new_event = inner.events;
        inner.callbacks.retain(|(e, waker)| {
            let cared_events = e.intersection(new_event);
            log::debug!("[send_event] recv event {:?}, we care {:?}", new_event, e,);
            if !cared_events.is_empty() {
                log::info!(
                    "[send_event] recv event {:?}, which is what we want",
                    cared_events
                );
                waker.wake_by_ref();
                false
            } else {
                true
            }
        });
        // }
        log::info!("[send_event] send event {:?} finished", event);
    }

    /// Wait for some event
    pub async fn wait_for_events(&self, events: Event) -> Event {
        WaitForEventFuture::new(events, self).await
    }

    /// Register for waiting some event
    pub fn register_event_waiter(&self, mut events: Event, waker: Waker) -> bool {
        stack_trace!();
        log::info!("[register_event_waiter] register event {:?}...", events);
        let mut inner = self.inner.lock();
        // Remove those that will wake the same task as us.
        inner.callbacks.retain(|(e, w)| {
            if w.will_wake(&waker) {
                events |= *e;
                false
            } else {
                true
            }
        });
        inner.callbacks.push((events, waker));
        log::debug!(
            "[register_event_waiter] callback len {}",
            inner.callbacks.len()
        );
        inner.events.intersects(events)
    }
}

struct WaitForEventFuture<'a> {
    mailbox: &'a Mailbox,
    events: Event,
    // has_added_cb: bool,
}

impl<'a> WaitForEventFuture<'a> {
    pub fn new(events: Event, mailbox: &'a Mailbox) -> Self {
        Self {
            mailbox,
            events,
            // has_added_cb: false,
        }
    }
}

impl<'a> Future for WaitForEventFuture<'a> {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<Self::Output> {
        stack_trace!();
        let mut inner = self.mailbox.inner.lock();
        let happend_events = inner.events.intersection(self.events);
        if !happend_events.is_empty() {
            inner.events.remove(happend_events);
            log::info!(
                "[WaitForEventFuture::poll] find events {:?}",
                happend_events
            );
            return Poll::Ready(happend_events);
        }
        // let this = unsafe { Pin::get_unchecked_mut(self) };
        // if !this.has_added_cb {
        // this.has_added_cb = true;
        let mut self_events = self.events;
        let waker = cx.waker().clone();
        // Remove those that will wake the same task as us.
        inner.callbacks.retain(|(e, w)| {
            if w.will_wake(&waker) {
                self_events |= *e;
                false
            } else {
                true
            }
        });
        inner.callbacks.push((self_events, waker));
        // }
        Poll::Pending
    }
}
