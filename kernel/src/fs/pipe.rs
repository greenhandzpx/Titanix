use alloc::boxed::Box;
use alloc::sync::{Arc, Weak};
use log::debug;

use crate::process;
use crate::processor::SumGuard;
use crate::sync::mutex::SpinNoIrqLock;
use crate::utils::error::AsyscallRet;

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
        Box::pin(async move {
            // debug!("start to pipe read {} bytes", buf.len());
            let _sum_guard = SumGuard::new();
            let want_to_read = buf.len();
            let mut buf_iter = buf.into_iter();
            let mut already_read = 0usize;
            loop {
                if let Some(ret) = self.inner_handler(|ring_buffer| {
                    let loop_read = ring_buffer.available_read();
                    if loop_read == 0 {
                        if ring_buffer.all_write_ends_closed() {
                            // all of the buffer's write ends have
                            // been closed, then just end reading
                            return Some(already_read);
                        }
                        return None;
                    }
                    for _ in 0..loop_read {
                        if let Some(byte_ref) = buf_iter.next() {
                            *byte_ref = ring_buffer.read_byte();
                            already_read += 1;
                            if already_read == want_to_read {
                                return Some(want_to_read);
                            }
                        } else {
                            // TODO: Some error happened?
                            return Some(already_read);
                        }
                    }
                    return None;
                }) {
                    // debug!("read {} bytes over", ret);
                    return Ok(ret as isize);
                } else {
                    process::yield_now().await;
                }
            }
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        assert!(self.writable());
        debug!("start to pipe write {} bytes", buf.len());
        Box::pin(async move {
            // debug!("satp(1) {:#x}", satp::read().bits());
            let _sum_guard = SumGuard::new();
            let want_to_write = buf.len();
            let mut buf_iter = buf.into_iter();
            let mut already_write = 0usize;
            loop {
                if let Some(ret) = self.inner_handler(|ring_buffer| {
                    let loop_write = ring_buffer.available_write();
                    if loop_write == 0 {
                        return None;
                        // drop(ring_buffer);
                        // suspend_current_and_run_next();
                        // continue;
                    }
                    // write at most loop_write bytes
                    for _ in 0..loop_write {
                        if let Some(byte_ref) = buf_iter.next() {
                            ring_buffer.write_byte(*byte_ref);
                            already_write += 1;
                            if already_write == want_to_write {
                                return Some(want_to_write);
                            }
                        } else {
                            return Some(already_write);
                        }
                    }
                    return None;
                }) {
                    debug!("pipe write {} bytes over", ret);
                    return Ok(ret as isize);
                } else {
                    debug!("no available write slots");
                    process::yield_now().await;
                }
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
const RING_BUFFER_SIZE: usize = 32;

#[derive(Copy, Clone, PartialEq)]
enum RingBufferStatus {
    FULL,
    EMPTY,
    NORMAL,
}

pub struct PipeRingBuffer {
    arr: [u8; RING_BUFFER_SIZE],
    head: usize,
    tail: usize,
    status: RingBufferStatus,
    write_end: Option<Weak<Pipe>>,
}

impl PipeRingBuffer {
    pub fn new() -> Self {
        Self {
            arr: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: RingBufferStatus::EMPTY,
            write_end: None,
        }
    }

    pub fn set_write_end(&mut self, write_end: &Arc<Pipe>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }

    pub fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::NORMAL;
        let c = self.arr[self.head];
        self.head = (self.head + 1) % RING_BUFFER_SIZE;
        if self.head == self.tail {
            self.status = RingBufferStatus::EMPTY;
        }
        c
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.status = RingBufferStatus::NORMAL;
        self.arr[self.tail] = byte;
        self.tail = (self.tail + 1) % RING_BUFFER_SIZE;
        if self.tail == self.head {
            self.status = RingBufferStatus::FULL;
        }
    }

    pub fn available_read(&self) -> usize {
        if self.status == RingBufferStatus::EMPTY {
            0
        } else {
            if self.tail > self.head {
                self.tail - self.head
            } else {
                self.tail + RING_BUFFER_SIZE - self.head
            }
        }
    }

    pub fn available_write(&self) -> usize {
        if self.status == RingBufferStatus::FULL {
            0
        } else {
            RING_BUFFER_SIZE - self.available_read()
        }
    }

    pub fn all_write_ends_closed(&self) -> bool {
        // debug!(
        //     "writen end ref cnt {}",
        //     self.write_end.as_ref().unwrap().strong_count()
        // );
        self.write_end.as_ref().unwrap().upgrade().is_none()
    }
}

/// Return (read_end, write_end)
pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    debug!("create a pipe");
    let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
    let read_end = Arc::new(Pipe::read_end_with_buffer(buffer.clone()));
    let write_end = Arc::new(Pipe::write_end_with_buffer(buffer.clone()));
    buffer.lock().set_write_end(&write_end);
    (read_end, write_end)
}
