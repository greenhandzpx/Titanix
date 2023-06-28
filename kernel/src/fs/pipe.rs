use core::future::Future;
use core::task::{Poll, Waker};

use alloc::boxed::Box;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use log::debug;

use crate::config::fs::PIPE_BUF_CAPACITY;
use crate::processor::SumGuard;
use crate::sync::mutex::SpinNoIrqLock;
use crate::utils::error::{AsyscallRet, GeneralRet, SyscallRet};

use super::file::{File, FileMeta};

type Mutex<T> = SpinNoIrqLock<T>;
pub struct Pipe {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
}

impl File for Pipe {
    fn readable(&self) -> bool {
        self.readable
    }

    fn writable(&self) -> bool {
        self.writable
    }

    fn metadata(&self) -> &FileMeta {
        todo!()
    }

    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        assert!(self.readable());
        debug!("start to pipe read {} bytes", buf.len());
        let buf_addr = buf.as_ptr() as usize;
        Box::pin(
            // debug!("start to pipe read {} bytes", buf.len());
            PipeFuture::new(
                self.buffer.clone(),
                buf_addr,
                buf.len(),
                PipeOperation::Read,
            ),
        )
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        assert!(self.writable());
        debug!("start to pipe write {} bytes", buf.len());
        let buf_addr = buf.as_ptr() as usize;
        Box::pin(PipeFuture::new(
            self.buffer.clone(),
            buf_addr,
            buf.len(),
            PipeOperation::Write,
        ))
    }

    fn pollin(&self, waker: Option<Waker>) -> GeneralRet<bool> {
        self.inner_handler(|ring_buffer| {
            if ring_buffer.available_read() > 0 {
                Ok(true)
            } else if ring_buffer.all_write_ends_closed() {
                Ok(true)
            } else {
                debug!("[Pipe::pollin]: no available read");
                if let Some(waker) = waker {
                    ring_buffer.wait_for_reading(waker)
                }
                Ok(false)
            }
        })
    }

    fn pollout(&self, waker: Option<Waker>) -> GeneralRet<bool> {
        self.inner_handler(|ring_buffer| {
            if ring_buffer.available_write() > 0 {
                Ok(true)
            } else if ring_buffer.all_read_ends_closed() {
                Ok(true)
            } else {
                debug!("[Pipe::pollout]: no available write");
                if let Some(waker) = waker {
                    ring_buffer.wait_for_writing(waker)
                }
                Ok(false)
            }
        })
    }
}

impl Pipe {
    pub fn read_end_with_buffer(buffer: Arc<Mutex<PipeRingBuffer>>) -> Self {
        Self {
            readable: true,
            writable: false,
            buffer,
        }
    }
    pub fn write_end_with_buffer(buffer: Arc<Mutex<PipeRingBuffer>>) -> Self {
        Self {
            readable: false,
            writable: true,
            buffer,
        }
    }
    fn inner_handler<T>(&self, f: impl FnOnce(&mut PipeRingBuffer) -> T) -> T {
        f(&mut self.buffer.lock())
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        if self.writable {
            // Write end,
            // we should wake up all read waiters(if any)
            let mut buffer = self.buffer.lock();
            while !buffer.read_waiters.is_empty() {
                let waker = buffer.read_waiters.pop().unwrap();
                waker.wake();
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum RingBufferStatus {
    FULL,
    EMPTY,
    NORMAL,
}

pub struct PipeRingBuffer {
    arr: [u8; PIPE_BUF_CAPACITY],
    head: usize,
    tail: usize,
    status: RingBufferStatus,
    write_end: Option<Weak<Pipe>>,
    read_end: Option<Weak<Pipe>>,
    read_waiters: Vec<Waker>,
    write_waiters: Vec<Waker>,
}

impl PipeRingBuffer {
    pub fn new() -> Self {
        Self {
            arr: [0; PIPE_BUF_CAPACITY],
            head: 0,
            tail: 0,
            status: RingBufferStatus::EMPTY,
            write_end: None,
            read_end: None,
            read_waiters: Vec::new(),
            write_waiters: Vec::new(),
        }
    }

    pub fn set_write_end(&mut self, write_end: &Arc<Pipe>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }

    pub fn set_read_end(&mut self, read_end: &Arc<Pipe>) {
        self.read_end = Some(Arc::downgrade(read_end));
    }

    pub fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::NORMAL;
        let c = self.arr[self.head];
        self.head = (self.head + 1) % PIPE_BUF_CAPACITY;
        if self.head == self.tail {
            self.status = RingBufferStatus::EMPTY;
        }
        // TODO: optimize: read all bytes and then notify
        while !self.write_waiters.is_empty() {
            let waker = self.write_waiters.pop().unwrap();
            waker.wake();
        }
        c
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.status = RingBufferStatus::NORMAL;
        self.arr[self.tail] = byte;
        self.tail = (self.tail + 1) % PIPE_BUF_CAPACITY;
        if self.tail == self.head {
            self.status = RingBufferStatus::FULL;
        }
        // TODO: optimize: write all bytes and then notify
        while !self.read_waiters.is_empty() {
            let waker = self.read_waiters.pop().unwrap();
            waker.wake();
        }
    }

    pub fn available_read(&self) -> usize {
        if self.status == RingBufferStatus::EMPTY {
            0
        } else {
            if self.tail > self.head {
                self.tail - self.head
            } else {
                self.tail + PIPE_BUF_CAPACITY - self.head
            }
        }
    }

    pub fn available_write(&self) -> usize {
        if self.status == RingBufferStatus::FULL {
            0
        } else {
            PIPE_BUF_CAPACITY - self.available_read()
        }
    }

    pub fn all_write_ends_closed(&self) -> bool {
        debug!(
            "writen end ref cnt {}",
            self.write_end.as_ref().unwrap().strong_count()
        );
        self.write_end.as_ref().unwrap().upgrade().is_none()
    }

    pub fn all_read_ends_closed(&self) -> bool {
        debug!(
            "read end ref cnt {}",
            self.read_end.as_ref().unwrap().strong_count()
        );
        self.read_end.as_ref().unwrap().upgrade().is_none()
    }

    pub fn wait_for_reading(&mut self, waker: Waker) {
        self.read_waiters.push(waker);
    }

    pub fn wait_for_writing(&mut self, waker: Waker) {
        self.write_waiters.push(waker);
    }
}

/// Return (read_end, write_end)
pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    debug!("create a pipe");
    let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
    let read_end = Arc::new(Pipe::read_end_with_buffer(buffer.clone()));
    let write_end = Arc::new(Pipe::write_end_with_buffer(buffer.clone()));
    buffer.lock().set_write_end(&write_end);
    buffer.lock().set_read_end(&read_end);
    (read_end, write_end)
}

enum PipeOperation {
    Read,
    Write,
}

struct PipeFuture {
    buffer: Arc<Mutex<PipeRingBuffer>>,
    user_buf: usize,
    user_buf_len: usize,
    already_put: usize,
    operation: PipeOperation,
}

impl PipeFuture {
    pub fn new(
        buffer: Arc<Mutex<PipeRingBuffer>>,
        user_buf: usize,
        user_buf_len: usize,
        operation: PipeOperation,
    ) -> Self {
        Self {
            buffer,
            user_buf,
            user_buf_len,
            already_put: 0,
            operation,
        }
    }
}

impl Future for PipeFuture {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let _sum_guard = SumGuard::new();
        if self.user_buf_len == 0 {
            return Poll::Ready(Ok(0));
        }
        let this = unsafe { self.get_unchecked_mut() };
        let mut ring_buffer = this.buffer.lock();
        match this.operation {
            PipeOperation::Read => {
                let buf = unsafe {
                    core::slice::from_raw_parts_mut(this.user_buf as *mut u8, this.user_buf_len)
                };
                let loop_read = ring_buffer.available_read();
                if loop_read == 0 {
                    if ring_buffer.all_write_ends_closed() {
                        // all of the buffer's write ends have
                        // been closed, then just end reading
                        return Poll::Ready(Ok(this.already_put as isize));
                    } else {
                        ring_buffer.wait_for_reading(cx.waker().clone());
                        return Poll::Pending;
                    }
                }
                for _ in 0..loop_read {
                    buf[this.already_put] = ring_buffer.read_byte();
                    this.already_put += 1;
                    if this.already_put == this.user_buf_len {
                        return Poll::Ready(Ok(this.already_put as isize));
                    }
                }
                ring_buffer.wait_for_reading(cx.waker().clone());
                return Poll::Pending;
            }
            PipeOperation::Write => {
                let buf = unsafe {
                    core::slice::from_raw_parts(this.user_buf as *const u8, this.user_buf_len)
                };
                let loop_write = ring_buffer.available_write();
                if loop_write == 0 {
                    if ring_buffer.all_read_ends_closed() {
                        // all of the buffer's read ends have
                        // been closed, then just end writing
                        return Poll::Ready(Ok(this.already_put as isize));
                    } else {
                        ring_buffer.wait_for_writing(cx.waker().clone());
                        return Poll::Pending;
                    }
                }
                for _ in 0..loop_write {
                    ring_buffer.write_byte(buf[this.already_put]);
                    this.already_put += 1;
                    if this.already_put == this.user_buf_len {
                        return Poll::Ready(Ok(this.already_put as isize));
                    }
                }
                ring_buffer.wait_for_writing(cx.waker().clone());
                return Poll::Pending;
            }
        }
    }
}
