use alloc::sync::Arc;
use fatfs::{IoBase, Read, Seek, Write};
use log::debug;

use crate::{config::board::BLOCK_SIZE, driver::LruBufferCache, stack_trace};

use super::{buffer_cache::BufferCache, BlockDevice};

// Temporarily for fat32
pub struct IoDevice {
    buffer_pool: LruBufferCache,
    offset: usize,
}

// const FAT32_OFFSET: usize = 0x800000;
const FAT32_OFFSET: usize = 0;

unsafe impl Sync for IoDevice {}

impl IoBase for IoDevice {
    // TODO
    // type Error = SyscallErr;
    type Error = ();
}

impl IoDevice {
    pub fn new(block_device: Arc<dyn BlockDevice>) -> Self {
        Self {
            buffer_pool: LruBufferCache::new(block_device),
            offset: FAT32_OFFSET,
        }
    }
}

impl Read for IoDevice {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut start = self.offset;
        // let end = (self.offset + buf.len()).min(self.size as usize);
        let end = self.offset + buf.len();
        if start >= end {
            return Err(());
        }
        let mut start_block = start / BLOCK_SIZE;
        // debug!("start block num {}", start_block);
        let mut read_size = 0usize;
        stack_trace!();
        loop {
            stack_trace!();
            // calculate end of current block
            let mut end_current_block = (start / BLOCK_SIZE + 1) * BLOCK_SIZE;
            end_current_block = end_current_block.min(end);
            // read and update read size
            let block_read_size = end_current_block - start;
            let dst = &mut buf[read_size..read_size + block_read_size];

            if self
                .buffer_pool
                .read_buffer_at(start_block, 0, |data_block: &[u8; BLOCK_SIZE]| {
                    let src = &data_block[start % BLOCK_SIZE..start % BLOCK_SIZE + block_read_size];
                    dst.copy_from_slice(src);
                })
                .is_err()
            {
                self.offset += read_size;
                return Ok(read_size);
            }

            // get_block_cache(
            //     self.get_block_id(start_block as u32, block_device) as usize,
            //     Arc::clone(block_device),
            // )
            // .lock()
            // .read(0, |data_block: &DataBlock| {
            //     let src = &data_block[start % BLOCK_SZ..start % BLOCK_SZ + block_read_size];
            //     dst.copy_from_slice(src);
            // });
            read_size += block_read_size;
            // move to next block
            if end_current_block == end {
                break;
            }
            start_block += 1;
            start = end_current_block;
        }
        self.offset += read_size;
        Ok(read_size)
    }
}

impl Write for IoDevice {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut start = self.offset;
        // let end = (self.offset + buf.len()).min(self.size as usize);
        let end = self.offset + buf.len();
        assert!(start <= end);
        let mut start_block = start / BLOCK_SIZE;
        let mut write_size = 0usize;
        stack_trace!();
        loop {
            // calculate end of current block
            let mut end_current_block = (start / BLOCK_SIZE + 1) * BLOCK_SIZE;
            end_current_block = end_current_block.min(end);
            // write and update write size
            let block_write_size = end_current_block - start;

            if self
                .buffer_pool
                .write_buffer_at(start_block, 0, |data_block: &mut [u8; BLOCK_SIZE]| {
                    let src = &buf[write_size..write_size + block_write_size];
                    let dst =
                        &mut data_block[start % BLOCK_SIZE..start % BLOCK_SIZE + block_write_size];
                    dst.copy_from_slice(src);
                })
                .is_err()
            {
                self.offset += write_size;
                return Ok(write_size);
            }

            // get_block_cache(
            //     self.get_block_id(start_block as u32, block_device) as usize,
            //     Arc::clone(block_device),
            // )
            // .lock()
            // .modify(0, |data_block: &mut DataBlock| {
            //     let src = &buf[write_size..write_size + block_write_size];
            //     let dst = &mut data_block[start % BLOCK_SZ..start % BLOCK_SZ + block_write_size];
            //     dst.copy_from_slice(src);
            // });
            write_size += block_write_size;
            // move to next block
            if end_current_block == end {
                break;
            }
            start_block += 1;
            start = end_current_block;
        }
        Ok(write_size)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.buffer_pool.sync_all_buffers().unwrap();
        Ok(())
    }
}

impl Seek for IoDevice {
    fn seek(&mut self, pos: fatfs::SeekFrom) -> Result<u64, Self::Error> {
        match pos {
            fatfs::SeekFrom::Current(pos) => {
                if pos < 0 {
                    self.offset -= (-pos) as usize;
                } else {
                    self.offset += pos as usize;
                }
            }
            fatfs::SeekFrom::End(pos) => {
                // TODO: since we don't know the underlying
                // block device's exact size, we cannot seek to the end
                todo!()
            }
            fatfs::SeekFrom::Start(pos) => {
                self.offset = FAT32_OFFSET + pos as usize;
            }
        }
        Ok(self.offset as u64)
    }
}
