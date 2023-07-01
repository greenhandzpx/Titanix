use alloc::{collections::LinkedList, sync::Arc};

use crate::{
    config::board::BLOCK_SIZE,
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallErr, SyscallRet},
};

use super::BlockDevice;

type Mutex<T> = SpinNoIrqLock<T>;

const BUFFER_POOL_SIZE: usize = 16;

const BUFFER_SIZE: usize = 512;

/// Trait for buffer cache devices
pub trait BufferCache {
    /// Do some modifications at the given offset of the given block
    fn write_buffer_at<T>(
        &self,
        block_no: usize,
        offset: usize,
        f: impl FnOnce(&mut T),
    ) -> GeneralRet<()>;

    /// Read data at the given offset of the given block
    fn read_buffer_at<T>(
        &self,
        block_no: usize,
        offset: usize,
        f: impl FnOnce(&T),
    ) -> GeneralRet<()>;

    /// Sync the given block
    fn sync_buffer(&self, block_no: usize) -> GeneralRet<()>;

    /// Sync all buffers
    fn sync_all_buffers(&self) -> GeneralRet<()>;
}

pub struct Buffer {
    data: [u8; BUFFER_SIZE],
    block_no: usize,
    dirty: bool,
    block_device: Arc<dyn BlockDevice>,
}

impl Buffer {
    fn new(block_no: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut data = [0u8; BUFFER_SIZE];
        block_device.read_block(block_no, &mut data);
        Self {
            data,
            block_no,
            dirty: false,
            block_device,
        }
    }

    fn write_at<T, R>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> R) -> R {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BUFFER_SIZE);
        self.dirty = true;
        let src_addr = unsafe { &mut *((&mut self.data[offset] as *mut u8) as *mut T) };
        f(src_addr)
    }

    fn read_at<T, R>(&self, offset: usize, f: impl FnOnce(&T) -> R) -> R {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BUFFER_SIZE);
        let dst_addr = unsafe { &*((&self.data[offset] as *const u8) as *const T) };
        f(dst_addr)
    }

    fn sync(&self) {
        if self.dirty {
            self.block_device.write_block(self.block_no, &self.data);
        }
    }
}

pub struct LruBufferCache {
    /// block no -> buffer
    /// TODO: optimize with rcu or something
    buffer_queue: Mutex<LinkedList<(usize, Arc<Mutex<Buffer>>)>>,
    /// TODO: not sure whether we should put the block device into seperate buffer cache or not
    block_device: Arc<dyn BlockDevice>,
}

impl BufferCache for LruBufferCache {
    fn write_buffer_at<T>(
        &self,
        block_no: usize,
        offset: usize,
        f: impl FnOnce(&mut T),
    ) -> GeneralRet<()> {
        let buffer_cache = self.look_up_buffer_cache(block_no);
        buffer_cache
            .ok_or(SyscallErr::ENOBUFS)?
            .lock()
            .write_at(offset, f);
        Ok(())
    }

    fn read_buffer_at<T>(
        &self,
        block_no: usize,
        offset: usize,
        f: impl FnOnce(&T),
    ) -> GeneralRet<()> {
        let buffer_cache = self.look_up_buffer_cache(block_no);
        buffer_cache
            .ok_or(SyscallErr::ENOBUFS)?
            .lock()
            .read_at(offset, f);
        Ok(())
    }

    fn sync_buffer(&self, block_no: usize) -> GeneralRet<()> {
        // TODO(optimize): we don't need to flush the buffer if it isn't in cache
        let buffer_cache = self.look_up_buffer_cache(block_no);
        buffer_cache.ok_or(SyscallErr::ENOBUFS)?.lock().sync();
        Ok(())
    }

    fn sync_all_buffers(&self) -> GeneralRet<()> {
        let buffer_queue_locked = self.buffer_queue.lock();
        for buffer in buffer_queue_locked.iter() {
            buffer.1.lock().sync();
        }
        Ok(())
    }
}

impl LruBufferCache {
    pub fn new(block_device: Arc<dyn BlockDevice>) -> Self {
        Self {
            buffer_queue: Mutex::new(LinkedList::new()),
            block_device,
        }
    }

    fn look_up_buffer_cache(&self, block_no: usize) -> Option<Arc<Mutex<Buffer>>> {
        // TODO: use hash table to query more quickly
        let mut buffer_queue_locked = self.buffer_queue.lock();
        if let Some((idx, _)) = buffer_queue_locked
            .iter()
            .enumerate()
            .find(|(_, buffer)| buffer.0 == block_no)
        {
            // Adjust the buffer's location in the queue
            let buffer = buffer_queue_locked.remove(idx);
            buffer_queue_locked.push_front(buffer);
        } else {
            // Cannot find the corresponding cache
            if buffer_queue_locked.len() == BUFFER_POOL_SIZE {
                // Buffer has been full
                // evict old buffer by lru policy
                let buffer_back = &buffer_queue_locked.back().unwrap().1;
                if Arc::strong_count(buffer_back) != 1 {
                    // all of the buffers are in use, then our os can only panic
                    panic!("Run out of buffers");
                }
                let buffer_back_locked = buffer_back.lock();
                if buffer_back_locked.dirty {
                    buffer_back_locked.sync();
                }
                drop(buffer_back_locked);
                buffer_queue_locked.pop_back();
            }
            buffer_queue_locked.push_front((
                block_no,
                Arc::new(Mutex::new(Buffer::new(block_no, self.block_device.clone()))),
            ));
        }
        Some(buffer_queue_locked.front().unwrap().1.clone())
    }
}
