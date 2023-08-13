use core::future::Future;
use core::task::{Poll, Waker};

use alloc::boxed::Box;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use log::debug;

use crate::processor::{current_task, SumGuard};
use crate::stack_trace;
use crate::sync::Event;
use crate::utils::async_utils::{Select2Futures, SelectOutput};
use crate::utils::error::{AsyscallRet, GeneralRet, SyscallErr, SyscallRet};

use super::file::{File, FileMeta, SeekFrom};
use super::{Mutex, OpenFlags};

pub struct Pipe<const N: usize> {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeRingBuffer<N>>>,
    meta: FileMeta,
}

impl<const N: usize> File for Pipe<N> {
    fn metadata(&self) -> &FileMeta {
        &self.meta
    }

    fn seek(&self, _pos: SeekFrom) -> SyscallRet {
        Err(SyscallErr::ESPIPE)
    }

    fn read<'a>(&'a self, buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        log::info!("[Pipe::read] start to pipe read {} bytes", buf.len());
        let buf_addr = buf.as_ptr() as usize;
        Box::pin(
            // debug!("start to pipe read {} bytes", buf.len());
            async move {
                // TODO: not sure event
                match Select2Futures::new(
                    PipeFuture::new(
                        self.buffer.clone(),
                        buf_addr,
                        buf.len(),
                        PipeOperation::Read,
                    ),
                    current_task().wait_for_events(Event::THREAD_EXIT | Event::PROCESS_EXIT),
                )
                .await
                {
                    SelectOutput::Output1(want) => want,
                    SelectOutput::Output2(event) => {
                        log::info!("[Pipe::read] interrupt by event {:?}", event);
                        Err(SyscallErr::EINTR)
                    }
                }
            },
        )
    }

    fn write<'a>(&'a self, buf: &'a [u8], _flags: OpenFlags) -> AsyscallRet {
        log::info!("[Pipe::write] start to pipe write {} bytes", buf.len());
        let buf_addr = buf.as_ptr() as usize;
        Box::pin(async move {
            // TODO: not sure event
            match Select2Futures::new(
                PipeFuture::new(
                    self.buffer.clone(),
                    buf_addr,
                    buf.len(),
                    PipeOperation::Write,
                ),
                current_task().wait_for_events(Event::THREAD_EXIT | Event::PROCESS_EXIT),
            )
            .await
            {
                SelectOutput::Output1(want) => want,
                SelectOutput::Output2(event) => {
                    log::info!("[Pipe::write] interrupt by event {:?}", event);
                    Err(SyscallErr::EINTR)
                }
            }
        })
    }

    fn pollin(&self, waker: Option<Waker>) -> GeneralRet<bool> {
        debug!("[Pipe::pollin] enter");
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

impl<const N: usize> Pipe<N> {
    fn new(flags: OpenFlags, buffer: Arc<Mutex<PipeRingBuffer<N>>>) -> Self {
        let readable = flags.contains(OpenFlags::RDONLY);
        let writable = flags.contains(OpenFlags::WRONLY);
        let meta = FileMeta::new(super::InodeMode::FileFIFO);
        Self {
            readable,
            writable,
            buffer,
            meta,
        }
    }

    fn inner_handler<T>(&self, f: impl FnOnce(&mut PipeRingBuffer<N>) -> T) -> T {
        f(&mut self.buffer.lock())
    }
}

impl<const N: usize> Drop for Pipe<N> {
    fn drop(&mut self) {
        log::info!("[Pipe::drop] start drop..., writable {}", self.writable);
        if self.writable {
            // Write end,
            // we should wake up all read waiters(if any)
            let mut buffer = self.buffer.lock();
            while !buffer.read_waiters.is_empty() {
                let waker = buffer.read_waiters.pop().unwrap();
                log::info!("[Pipe::drop] wake up");
                waker.wake();
            }
        } else if self.readable {
            let mut buffer = self.buffer.lock();
            while !buffer.write_waiters.is_empty() {
                let waker = buffer.write_waiters.pop().unwrap();
                log::info!("[Pipe::drop] wake up");
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

struct PipeRingBuffer<const N: usize> {
    // arr: [u8; PIPE_BUF_CAPACITY],
    arr: [u8; N],
    // arr: Vec<u8>,
    head: usize,
    tail: usize,
    status: RingBufferStatus,
    write_end: Option<Weak<Pipe<N>>>,
    read_end: Option<Weak<Pipe<N>>>,
    read_waiters: Vec<Waker>,
    write_waiters: Vec<Waker>,
}

impl<const N: usize> PipeRingBuffer<N> {
    fn new() -> Self {
        // let arr = vec![0 as u8; buf_capcity];
        Self {
            arr: [0; N],
            head: 0,
            tail: 0,
            status: RingBufferStatus::EMPTY,
            write_end: None,
            read_end: None,
            read_waiters: Vec::new(),
            write_waiters: Vec::new(),
        }
    }

    fn set_write_end(&mut self, write_end: &Arc<Pipe<N>>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }

    fn set_read_end(&mut self, read_end: &Arc<Pipe<N>>) {
        self.read_end = Some(Arc::downgrade(read_end));
    }

    fn read_range(&mut self, buf: &mut [u8]) -> usize {
        self.status = RingBufferStatus::NORMAL;
        let end = match self.tail > self.head {
            true => self.tail,
            false => N,
        };
        let ret = (end - self.head).min(buf.len());
        let end = self.head + ret;
        buf[..ret].copy_from_slice(&mut self.arr[self.head..end]);
        self.head = end % N;
        if self.head == self.tail {
            self.status = RingBufferStatus::EMPTY;
        }
        ret
    }

    fn write_range(&mut self, buf: &[u8]) -> usize {
        self.status = RingBufferStatus::NORMAL;
        let end = match self.head > self.tail {
            true => self.head,
            false => N,
        };
        let ret = (end - self.tail).min(buf.len());
        let end = self.tail + ret;
        self.arr[self.tail..end].copy_from_slice(&buf[..ret]);
        self.tail = end % N;
        if self.tail == self.head {
            self.status = RingBufferStatus::FULL;
        }
        ret
    }

    #[allow(unused)]
    fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::NORMAL;
        let c = self.arr[self.head];
        self.head = (self.head + 1) % N;
        if self.head == self.tail {
            self.status = RingBufferStatus::EMPTY;
        }
        c
    }

    #[allow(unused)]
    fn write_byte(&mut self, byte: u8) {
        self.status = RingBufferStatus::NORMAL;
        self.arr[self.tail] = byte;
        self.tail = (self.tail + 1) % N;
        if self.tail == self.head {
            self.status = RingBufferStatus::FULL;
        }
    }

    fn available_read(&self) -> usize {
        if self.status == RingBufferStatus::EMPTY {
            0
        } else {
            if self.tail > self.head {
                self.tail - self.head
            } else {
                self.tail + N - self.head
            }
        }
    }

    fn available_write(&self) -> usize {
        if self.status == RingBufferStatus::FULL {
            0
        } else {
            N - self.available_read()
        }
    }

    fn all_write_ends_closed(&self) -> bool {
        log::info!(
            "[all_write_end_closed] write end ref cnt {}",
            self.write_end.as_ref().unwrap().strong_count()
        );
        self.write_end.as_ref().unwrap().upgrade().is_none()
    }

    fn all_read_ends_closed(&self) -> bool {
        debug!(
            "read end ref cnt {}",
            self.read_end.as_ref().unwrap().strong_count()
        );
        self.read_end.as_ref().unwrap().upgrade().is_none()
    }

    fn wait_for_reading(&mut self, waker: Waker) {
        self.read_waiters.push(waker);
    }

    fn wake(&mut self, for_reader: bool) {
        let queue = match for_reader {
            true => &mut self.read_waiters,
            false => &mut self.write_waiters,
        };
        while !queue.is_empty() {
            let waker = queue.pop().unwrap();
            log::trace!("[Pipe::wake] wake up");
            waker.wake();
        }
    }

    fn wait_for_writing(&mut self, waker: Waker) {
        self.write_waiters.push(waker);
    }
}

/// Return (read_end, write_end)
pub fn make_pipe<const N: usize>(flags: Option<OpenFlags>) -> (Arc<Pipe<N>>, Arc<Pipe<N>>) {
    debug!("create a pipe");
    let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
    let flags = match flags {
        Some(flags) => flags,
        None => OpenFlags::empty(),
    };
    let read_end = Arc::new(Pipe::new(flags | OpenFlags::RDONLY, buffer.clone()));
    let write_end = Arc::new(Pipe::new(flags | OpenFlags::WRONLY, buffer.clone()));

    buffer.lock().set_write_end(&write_end);
    buffer.lock().set_read_end(&read_end);
    (read_end, write_end)
}

#[allow(unused)]
enum PipeOperation {
    Read,
    Write,
}

struct PipeFuture<const N: usize> {
    buffer: Arc<Mutex<PipeRingBuffer<N>>>,
    user_buf: usize,
    user_buf_len: usize,
    already_put: usize,
    operation: PipeOperation,
}

impl<const N: usize> PipeFuture<N> {
    #[allow(unused)]
    pub fn new(
        buffer: Arc<Mutex<PipeRingBuffer<N>>>,
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

impl<const N: usize> Future for PipeFuture<N> {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        stack_trace!();
        // if current_task().is_zombie() {
        //     return Poll::Ready(Ok(0));
        // }
        let _sum_guard = SumGuard::new();
        if self.user_buf_len == 0 {
            return Poll::Ready(Ok(0));
        }
        let this = unsafe { self.get_unchecked_mut() };
        let mut ring_buffer = this.buffer.lock();
        match this.operation {
            PipeOperation::Read => {
                debug!("[PipeFuture::poll] read");
                let buf = unsafe {
                    core::slice::from_raw_parts_mut(this.user_buf as *mut u8, this.user_buf_len)
                };
                let loop_read = ring_buffer.available_read();
                if loop_read == 0 {
                    if ring_buffer.all_write_ends_closed() {
                        // all of the buffer's write ends have
                        // been closed, then just end reading
                        log::info!("[PipeFuture::poll] all write ends has closed");
                        return Poll::Ready(Ok(this.already_put));
                    } else {
                        ring_buffer.wait_for_reading(cx.waker().clone());
                        log::info!("[PipeFuture::poll] nothing to read, wait...");
                        return Poll::Pending;
                    }
                }
                this.already_put += ring_buffer.read_range(&mut buf[this.already_put..]);
                ring_buffer.wake(false);
                debug!("[PipeFuture::poll] read return {}", this.already_put);
                return Poll::Ready(Ok(this.already_put));
            }
            PipeOperation::Write => {
                debug!("[PipeFuture::poll] write");
                let buf = unsafe {
                    core::slice::from_raw_parts(this.user_buf as *const u8, this.user_buf_len)
                };
                let loop_write = ring_buffer.available_write();
                if loop_write == 0 {
                    if ring_buffer.all_read_ends_closed() {
                        // all of the buffer's read ends have
                        // been closed, then just end writing
                        return Poll::Ready(Ok(this.already_put));
                    } else {
                        ring_buffer.wait_for_writing(cx.waker().clone());
                        debug!("[PipeFuture::poll] no space to write, wait...");
                        return Poll::Pending;
                    }
                }
                debug!("[PipeFuture::poll] available write {}", loop_write);
                this.already_put += ring_buffer.write_range(&buf[this.already_put..]);
                ring_buffer.wake(true);
                debug!("[PipeFuture::poll] write return {}", this.already_put);
                return Poll::Ready(Ok(this.already_put));
            }
        }
    }
}
