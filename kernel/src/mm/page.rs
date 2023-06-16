use alloc::sync::Weak;
use log::trace;

use crate::{
    config::{board::BLOCK_SIZE, mm::PAGE_SIZE},
    fs::Inode,
    mm,
    sync::mutex::SleepLock,
    utils::error::{GeneralRet, SyscallErr},
};

use super::FrameTracker;

type Mutex<T> = SleepLock<T>;

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
    /// Data frame
    pub data_frame: FrameTracker,
    // pub data: [u8; PAGE_SIZE],
    /// Data block state
    data_states: [DataState; PAGE_SIZE / BLOCK_SIZE],
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
                data_frame: mm::frame_alloc().unwrap(),
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
    pub async fn read(&self, offset: usize, buf: &mut [u8]) -> GeneralRet<usize> {
        if offset >= PAGE_SIZE {
            Err(SyscallErr::E2BIG)
        } else {
            let mut inner = self.inner.lock().await;
            let mut end = offset + buf.len();
            if end > PAGE_SIZE {
                end = PAGE_SIZE;
            }
            inner.load_buffer_if_needed(offset, end).await?;
            buf.copy_from_slice(&inner.data_frame.ppn.bytes_array()[offset..end]);
            Ok(end - offset)
        }
    }
    /// Write this page.
    /// `offset`: page offset
    pub async fn write(&self, offset: usize, buf: &[u8]) -> GeneralRet<usize> {
        trace!(
            "[Page::write]: page addr {:#x}",
            self as *const Self as usize
        );
        if offset >= PAGE_SIZE {
            Err(SyscallErr::E2BIG)
        } else {
            // let mut inner = self.inner.lock();
            let mut end = offset + buf.len();
            if end > PAGE_SIZE {
                end = PAGE_SIZE;
            }
            self.inner
                .lock()
                .await
                .mark_buffer_dirty_if_needed(offset, end)
                .await?;
            let inner = self.inner.lock().await;
            inner.data_frame.ppn.bytes_array()[offset..end].copy_from_slice(buf);
            Ok(end - offset)
        }
    }

    /// Sync all buffers
    pub fn sync(&self) {
        todo!()
    }

    /// Load all buffers
    pub async fn load_all_buffers(&self) -> GeneralRet<()> {
        trace!(
            "[Page::load_all_buffers]: page addr {:#x}",
            self.bytes_array_ptr().await as usize
        );
        // let mut inner = self.inner.lock();
        let len = PAGE_SIZE;
        self.inner
            .lock()
            .await
            .load_buffer_if_needed(0, len)
            .await?;
        Ok(())
    }

    /// Get the raw pointer of this page
    pub async fn bytes_array_ptr(&self) -> *const u8 {
        self.inner
            .lock()
            .await
            .data_frame
            .ppn
            .bytes_array()
            .as_ptr()
    }

    /// Get the bytes array of this page
    pub async fn bytes_array(&self) -> &'static [u8] {
        self.inner.lock().await.data_frame.ppn.bytes_array()
    }
}

impl PageInner {
    async fn load_buffer_if_needed(&mut self, start_off: usize, end_off: usize) -> GeneralRet<()> {
        let start_buffer_idx = start_off / BLOCK_SIZE;
        let end_buffer_idx = (end_off - 1 + BLOCK_SIZE) / BLOCK_SIZE;

        for idx in start_buffer_idx..end_buffer_idx {
            if self.data_states[idx] == DataState::Outdated {
                trace!(
                    "outdated block, idx {}, start_page_off {:#x}",
                    idx,
                    start_off
                );
                let page_offset = idx * BLOCK_SIZE;
                let file_offset = page_offset + self.file_offset.unwrap();
                self.inode
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .read(
                        file_offset,
                        &mut self.data_frame.ppn.bytes_array()
                            [page_offset..page_offset + BLOCK_SIZE],
                    )
                    .await?;
                self.data_states[idx] = DataState::Coherent;
            }
        }
        Ok(())
    }

    async fn mark_buffer_dirty_if_needed(
        &mut self,
        start_off: usize,
        end_off: usize,
    ) -> GeneralRet<()> {
        let start_buffer_idx = start_off / BLOCK_SIZE;
        let end_buffer_idx = (end_off - 1 + BLOCK_SIZE) / BLOCK_SIZE;
        trace!("start {}, end {}", start_buffer_idx, end_buffer_idx);

        for idx in start_buffer_idx..end_buffer_idx {
            if self.data_states[idx] == DataState::Outdated {
                let page_offset = idx * BLOCK_SIZE;
                let file_offset = page_offset + self.file_offset.unwrap();
                self.inode
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .read(
                        file_offset,
                        &mut self.data_frame.ppn.bytes_array()
                            [page_offset..page_offset + BLOCK_SIZE],
                    )
                    .await?;
                trace!(
                    "outdated block, idx {}, start_page_off {:#x}",
                    idx,
                    start_off
                );
                self.data_states[idx] = DataState::Coherent;
            }
            if self.data_states[idx] != DataState::Dirty {
                self.data_states[idx] = DataState::Dirty;
            }
        }
        Ok(())
    }
}
