use alloc::sync::Weak;
use log::{info, trace};

use crate::{
    config::{
        board::BLOCK_SIZE,
        mm::{PAGE_SIZE, PAGE_SIZE_BITS},
    },
    fs::Inode,
    mm, stack_trace,
    sync::mutex::{SleepLock, SpinLock},
    utils::error::{GeneralRet, SyscallErr},
};

use super::{FrameTracker, MapPermission};

type Mutex<T> = SleepLock<T>;

/// Note that the process will visit one page through `Arc`
/// which maintains the ref cnt, so we can decide whether
/// one page can be evicted by `Arc::strong_count()`
pub struct Page {
    /// Immutable page permission
    pub permission: SpinLock<MapPermission>,
    /// Physical data frame
    pub data_frame: FrameTracker,
    /// Mutable page inner
    /// TODO: figure out whether we should use Box to decrease
    pub file_info: Option<Mutex<FilePageInfo>>,
}

pub struct FilePageInfo {
    /// Start offset of this page at its related file
    file_offset: usize,
    /// Data block state
    data_states: [DataState; PAGE_SIZE / BLOCK_SIZE],
    /// Inode that this page related to
    inode: Weak<dyn Inode>,
}

#[derive(PartialEq)]
enum DataState {
    Dirty,
    Coherent,
    Outdated,
}

/// Build a page
pub struct PageBuilder {
    permission: MapPermission,
    offset: Option<usize>,
    inode: Option<Weak<dyn Inode>>,
    physical_frame: Option<FrameTracker>,
    is_file_page: bool,
}

impl PageBuilder {
    /// Construct a page builder
    pub fn new() -> Self {
        stack_trace!();
        Self {
            offset: None,
            inode: None,
            physical_frame: None,
            // TODO: defalut is readable and writable
            permission: MapPermission::R | MapPermission::W,
            is_file_page: false,
        }
    }
    /// Whether this page is related to a file
    pub fn is_file_page(mut self) -> Self {
        self.is_file_page = true;
        self
    }
    /// The page's backup file
    pub fn file_info(mut self, file_info: &FilePageInfo) -> Self {
        self.offset = Some(file_info.file_offset);
        self.inode = Some(file_info.inode.clone());
        self.is_file_page = true;
        self
    }
    /// Page map permission
    pub fn permission(mut self, permission: MapPermission) -> Self {
        // stack_trace!();
        // if permission.bits() == 0 {
        //     warn!("permission None: {:?}", permission);
        // }
        self.permission = permission;
        self
    }
    /// Page's file offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some((offset >> PAGE_SIZE_BITS) << PAGE_SIZE_BITS);
        self
    }
    /// Page's backup inode
    pub fn inode(mut self, inode: Weak<dyn Inode>) -> Self {
        self.inode = Some(inode);
        self
    }
    /// Page's physical page frame
    pub fn physical_frame(mut self, frame: FrameTracker) -> Self {
        stack_trace!();
        self.physical_frame = Some(frame);
        self
    }
    /// Build the page
    pub fn build(mut self) -> Page {
        stack_trace!();
        let frame = match self.physical_frame {
            None => mm::frame_alloc().unwrap(),
            Some(_) => self.physical_frame.take().unwrap(),
        };
        Page {
            permission: SpinLock::new(self.permission),
            data_frame: frame,
            file_info: match self.is_file_page {
                true => Some(Mutex::new(FilePageInfo {
                    file_offset: self.offset.unwrap(),
                    data_states: core::array::from_fn(|_| DataState::Outdated),
                    inode: self.inode.unwrap(),
                })),
                false => None,
            },
        }
    }
}

impl Page {
    /// Reinterpret this page
    pub fn reinterpret<T>(&self) -> &'static T {
        self.data_frame.ppn.reinterpret()
    }

    /// Reinterpret this page
    pub fn reinterpret_mut<T>(&self) -> &'static mut T {
        self.data_frame.ppn.reinterpret_mut()
    }

    /// Read this page.
    /// `offset`: page offset
    pub async fn read(&self, offset: usize, buf: &mut [u8]) -> GeneralRet<usize> {
        if offset >= PAGE_SIZE {
            Err(SyscallErr::E2BIG)
        } else {
            let mut end = offset + buf.len();
            if end > PAGE_SIZE {
                end = PAGE_SIZE;
            }
            self.load_buffer_if_needed(offset, end).await?;
            buf.copy_from_slice(&self.data_frame.ppn.bytes_array()[offset..end]);
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
            self.mark_buffer_dirty_if_needed(offset, end).await?;
            self.data_frame.ppn.bytes_array()[offset..end].copy_from_slice(buf);
            Ok(end - offset)
        }
    }

    /// Sync all buffers if needed
    pub async fn sync(&self) -> GeneralRet<()> {
        let file_info = self.file_info.as_ref().unwrap().lock().await;
        let inode = file_info.inode.upgrade().ok_or(SyscallErr::EBADF)?;
        let data_len = inode.metadata().inner.lock().data_len;
        log::trace!(
            "[Page::sync] sync page, file offset {:#x}",
            file_info.file_offset
        );
        for idx in 0..PAGE_SIZE / BLOCK_SIZE {
            match file_info.data_states[idx] {
                DataState::Dirty => {
                    let page_offset = idx * BLOCK_SIZE;
                    let file_offset = file_info.file_offset + page_offset;
                    log::trace!(
                        "[Page::sync] sync block of the page, file offset {:#x}",
                        file_offset
                    );
                    // In case of truncate
                    if data_len <= file_offset {
                        info!("[Page::sync] file has been truncated, now len {:#x}, page's file offset {:#x}", data_len, file_offset);
                        return Ok(());
                    }
                    inode
                        .write(
                            file_offset,
                            &self.bytes_array()[page_offset..page_offset + BLOCK_SIZE],
                        )
                        .await?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Load all buffers
    pub async fn load_all_buffers(&self) -> GeneralRet<()> {
        trace!(
            "[Page::load_all_buffers]: page addr {:#x}",
            self.bytes_array_ptr() as usize
        );
        // let mut inner = self.inner.lock();
        let len = PAGE_SIZE;
        self.load_buffer_if_needed(0, len).await?;
        Ok(())
    }

    /// Get the raw pointer of this page
    pub fn bytes_array_ptr(&self) -> *const u8 {
        self.data_frame.ppn.bytes_array().as_ptr()
    }

    /// Get the bytes array of this page
    pub fn bytes_array(&self) -> &'static [u8] {
        self.data_frame.ppn.bytes_array()
    }

    async fn load_buffer_if_needed(&self, start_off: usize, end_off: usize) -> GeneralRet<()> {
        let start_buffer_idx = start_off / BLOCK_SIZE;
        let end_buffer_idx = (end_off - 1 + BLOCK_SIZE) / BLOCK_SIZE;

        let mut file_info = self.file_info.as_ref().unwrap().lock().await;
        for idx in start_buffer_idx..end_buffer_idx {
            if file_info.data_states[idx] == DataState::Outdated {
                let page_offset = idx * BLOCK_SIZE;
                let file_offset = page_offset + file_info.file_offset;
                trace!(
                    "outdated block, idx {}, start_page_off {:#x}, file_off {:#x}, backup-file off {:#x}",
                    idx,
                    start_off,
                    file_offset,
                    file_info.file_offset
                );
                file_info
                    .inode
                    .upgrade()
                    .unwrap()
                    .read(
                        file_offset,
                        &mut self.data_frame.ppn.bytes_array()
                            [page_offset..page_offset + BLOCK_SIZE],
                    )
                    .await?;
                file_info.data_states[idx] = DataState::Coherent;
            }
        }
        Ok(())
    }

    async fn mark_buffer_dirty_if_needed(
        &self,
        start_off: usize,
        end_off: usize,
    ) -> GeneralRet<()> {
        let start_buffer_idx = start_off / BLOCK_SIZE;
        let end_buffer_idx = (end_off - 1 + BLOCK_SIZE) / BLOCK_SIZE;
        trace!("start {}, end {}", start_buffer_idx, end_buffer_idx);

        let mut file_info = self.file_info.as_ref().unwrap().lock().await;

        for idx in start_buffer_idx..end_buffer_idx {
            if file_info.data_states[idx] == DataState::Outdated {
                let page_offset = idx * BLOCK_SIZE;
                let file_offset = page_offset + file_info.file_offset;
                file_info
                    .inode
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
                file_info.data_states[idx] = DataState::Coherent;
            }
            if file_info.data_states[idx] != DataState::Dirty {
                file_info.data_states[idx] = DataState::Dirty;
            }
        }
        Ok(())
    }
}
