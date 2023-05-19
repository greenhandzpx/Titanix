use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

use crate::{
    config::{board::BLOCK_SIZE, fs::FILE_PAGE_SIZE, mm::PAGE_SIZE},
    driver::Buffer,
    fs::Inode,
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallErr},
};

type Mutex<T> = SpinNoIrqLock<T>;

/// Note that the process will visit one page through `Arc`
/// which maintains the ref cnt, so we can decide whether
/// one page can be evicted by `Arc::strong_count()`
pub struct Page {
    /// Mutable page inner
    pub inner: Mutex<PageInner>,
}

pub struct PageInner {
    /// Start offset of this page at its related file
    pub file_offset: Option<usize>,
    /// Data
    pub data: [u8; FILE_PAGE_SIZE],
    /// Data block state
    data_states: [DataState; FILE_PAGE_SIZE / BLOCK_SIZE],
    /// Inode that this page related to
    inode: Option<Weak<dyn Inode>>,
}

#[derive(PartialEq)]
enum DataState {
    Dirty,
    Coherent,
    Outdated,
}

pub struct PageBuilder {
    offset: Option<usize>,
    inode: Option<Weak<dyn Inode>>,
}

impl PageBuilder {
    pub fn new() -> Self {
        Self {
            offset: None,
            inode: None,
        }
    }
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn inode(mut self, inode: Weak<dyn Inode>) -> Self {
        self.inode = Some(inode);
        self
    }
    pub fn build(self) -> Page {
        Page {
            inner: Mutex::new(PageInner {
                file_offset: self.offset,
                data_states: core::array::from_fn(|_| DataState::Outdated),
                data: [0; PAGE_SIZE],
                inode: self.inode,
            }),
        }
    }
}

impl Page {
    // /// Create a new page
    // pub fn new(inode: Option<Weak<dyn Inode>>) -> Self {
    //     Self {
    //         inner: Mutex::new(PageInner {
    //             data_states: core::array::from_fn(|_| DataState::Outdated),
    //             data: [0; PAGE_SIZE],
    //             inode,
    //         }),
    //     }
    // }

    /// Read this page.
    /// `offset`: page offset
    pub fn read(&self, offset: usize, buf: &mut [u8]) -> GeneralRet<usize> {
        if offset >= FILE_PAGE_SIZE {
            Err(SyscallErr::E2BIG)
        } else {
            let mut inner = self.inner.lock();
            let mut end = offset + buf.len();
            if end > inner.data.len() {
                end = inner.data.len();
            }
            inner.load_buffer_if_needed(offset, end)?;
            buf.copy_from_slice(&inner.data[offset..end]);
            Ok(end - offset)
        }
    }
    /// Write this page.
    /// `offset`: page offset
    pub fn write(&self, offset: usize, buf: &[u8]) -> GeneralRet<usize> {
        if offset >= FILE_PAGE_SIZE {
            Err(SyscallErr::E2BIG)
        } else {
            let mut inner = self.inner.lock();
            let mut end = offset + buf.len();
            if end > inner.data.len() {
                end = inner.data.len();
            }
            inner.mark_buffer_dirty_if_needed(offset, end);
            inner.data[offset..end].copy_from_slice(buf);
            Ok(end - offset)
        }
    }

    /// Sync all buffers
    pub fn sync(&self) {
        todo!()
    }

    /// Load all buffers
    pub fn load_all_buffers(&self) -> GeneralRet<()> {
        let mut inner = self.inner.lock();
        let len = inner.data.len();
        inner.load_buffer_if_needed(0, len)?;
        Ok(())
    }
}

impl PageInner {
    fn load_buffer_if_needed(&mut self, start_off: usize, end_off: usize) -> GeneralRet<()> {
        let start_buffer_idx = start_off / BLOCK_SIZE;
        let end_buffer_idx = end_off / BLOCK_SIZE;

        for idx in start_buffer_idx..end_buffer_idx {
            if self.data_states[idx] == DataState::Outdated {
                let page_offset = idx * BLOCK_SIZE;
                let file_offset = page_offset + self.file_offset.unwrap();
                self.inode.as_ref().unwrap().upgrade().unwrap().read(
                    file_offset,
                    &mut self.data[page_offset..page_offset + BLOCK_SIZE],
                )?;
                self.data_states[idx] = DataState::Coherent;
            }
        }
        Ok(())
    }

    fn mark_buffer_dirty_if_needed(&mut self, start_off: usize, end_off: usize) {
        let start_buffer_idx = start_off / BLOCK_SIZE;
        let end_buffer_idx = end_off / BLOCK_SIZE;

        for idx in start_buffer_idx..end_buffer_idx {
            if self.data_states[idx] != DataState::Dirty {
                self.data_states[idx] = DataState::Dirty;
            }
        }
    }
}
