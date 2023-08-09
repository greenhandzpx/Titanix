use core::task::Waker;

use alloc::{
    boxed::Box,
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};
use log::{debug, trace};

use crate::{
    config::mm::PAGE_SIZE,
    fs::InodeState,
    mm::memory_space::VmArea,
    processor::SumGuard,
    stack_trace,
    sync::mutex::SleepLock,
    timer::ffi::current_time_spec,
    utils::{
        async_utils::block_on,
        error::{AgeneralRet, AsyscallRet, GeneralRet, SyscallErr, SyscallRet},
    },
};

use super::{inode::Inode, InodeMode, Mutex, OpenFlags};

pub struct FileMeta {
    /// Mutable,
    pub inner: Mutex<FileMetaInner>,
    /// pread & pwrite's lock
    pub prw_lock: SleepLock<()>,
}

impl FileMeta {
    pub fn inner_get<T>(&self, f: impl FnOnce(&mut FileMetaInner) -> T) -> T {
        f(&mut self.inner.lock())
    }
    pub fn new(mode: InodeMode) -> Self {
        Self {
            inner: Mutex::new(FileMetaInner {
                // flags,
                inode: None,
                mode,
                pos: 0,
                dirent_index: 0,
                file: None,
            }),
            prw_lock: SleepLock::new(()),
        }
    }
}
pub struct FileMetaInner {
    // /// open flags
    // pub flags: OpenFlags,
    /// inode to which this file refers
    pub inode: Option<Arc<dyn Inode>>,
    /// file type (the same as InodeMode)
    pub mode: InodeMode,
    /// file offset
    pub pos: usize,
    /// current read dirent index
    pub dirent_index: usize,
    /// attached file
    pub file: Option<Weak<dyn File>>,
}

/// It is based on the `std::io::SeekFrom` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(usize),
    /// Sets the offset to the size of this object plus the specified number of bytes.
    End(isize),
    /// Sets the offset to the current position plus the specified number of bytes.
    Current(isize),
}

pub trait File: Send + Sync {
    /// For default file, data must be read from page cache first.
    /// Note that only guarantee the safety of the first PAGE_SIZE bytes

    fn read<'a>(&'a self, buf: &'a mut [u8], flags: OpenFlags) -> AsyscallRet;
    /// For default file, data must be written to page cache first.
    fn write<'a>(&'a self, buf: &'a [u8], flags: OpenFlags) -> AsyscallRet;

    fn pread<'a>(&'a self, buf: &'a mut [u8], off: usize) -> AsyscallRet {
        Box::pin(async move {
            self.metadata().prw_lock.lock().await;
            let old_off = self.seek(SeekFrom::Current(0))?;
            self.seek(SeekFrom::Start(off))?;
            let ret = self.read(buf, OpenFlags::default()).await;
            self.seek(SeekFrom::Start(old_off as usize))?;
            ret
        })
    }

    fn pwrite<'a>(&'a self, buf: &'a [u8], off: usize) -> AsyscallRet {
        Box::pin(async move {
            self.metadata().prw_lock.lock().await;
            let old_off = self.seek(SeekFrom::Current(0))?;
            self.seek(SeekFrom::Start(off))?;
            // TODO: need to changed with actual flags
            let ret = self.write(buf, OpenFlags::default()).await;
            self.seek(SeekFrom::Start(old_off as usize))?;
            ret
        })
    }

    fn pollin(&self, _waker: Option<Waker>) -> GeneralRet<bool> {
        // TODO: optimize
        debug!("[File::pollin] enter default");
        Ok(true)
        // todo!()
    }

    fn pollout(&self, _waker: Option<Waker>) -> GeneralRet<bool> {
        debug!("[File::pollout] enter default");
        Ok(true)
        // todo!()
    }

    /// For default file, data must be read from page cache first
    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        block_on(self.read(buf, OpenFlags::default()))
    }

    /// For default file, data must be written to page cache first
    fn sync_write(&self, buf: &[u8]) -> SyscallRet {
        block_on(self.write(buf, OpenFlags::default()))
    }

    /// Return the new offset
    fn seek(&self, pos: SeekFrom) -> SyscallRet {
        let mut meta = self.metadata().inner.lock();
        match pos {
            SeekFrom::Current(off) => {
                if off < 0 {
                    meta.pos -= off.abs() as usize;
                } else {
                    meta.pos += off as usize;
                }
            }
            SeekFrom::Start(off) => {
                meta.pos = off;
            }
            SeekFrom::End(off) => {
                let data_len = meta
                    .inode
                    .as_ref()
                    .unwrap()
                    .metadata()
                    .inner
                    .lock()
                    .data_len;
                if off < 0 {
                    meta.pos = data_len - off.abs() as usize;
                } else {
                    meta.pos = data_len + off as usize;
                }
            }
        }
        Ok(meta.pos)
        // self.metadata().inner.lock().pos = offset;
        // Ok(offset  )
    }

    /// Read all data from this file synchronously
    /// TODO: add async version
    fn read_all_from_start(&self, buffer: &mut Vec<u8>) -> GeneralRet<()> {
        stack_trace!();
        let old_pos = self.seek(SeekFrom::Start(0))?;
        self.seek(SeekFrom::Start(0))?;
        buffer.clear();
        buffer.resize(PAGE_SIZE, 0);
        // *buffer = vec![0u8; PAGE_SIZE];
        let mut idx = 0;
        loop {
            stack_trace!();
            let len = self.sync_read(&mut buffer.as_mut_slice()[idx..idx + PAGE_SIZE])?;
            if len == 0 {
                break;
            }
            idx += len;
            stack_trace!();
            buffer.resize(idx + PAGE_SIZE, 0);
            stack_trace!();
        }
        stack_trace!();
        self.seek(SeekFrom::Start(old_pos))?;
        stack_trace!();
        Ok(())
    }

    // TODO: not sure the args
    fn mmap(&self) -> GeneralRet<VmArea> {
        todo!()
    }

    fn metadata(&self) -> &FileMeta;

    // fn flags(&self) -> OpenFlags {
    //     self.metadata().inner.lock().flags
    // }

    fn truncate(&self, len: usize) -> AgeneralRet<()> {
        Box::pin(async move {
            stack_trace!();
            let (old_pos, inode) = self.metadata().inner_get(|inner| {
                // let flags = inner.flags;
                let inode = inner.inode.as_ref().ok_or(SyscallErr::EINVAL)?.clone();
                Ok((
                    inner.pos,
                    // flags.contains(OpenFlags::WRONLY) || flags.contains(OpenFlags::RDWR),
                    inode,
                ))
            })?;
            // if !writable {
            //     return Err(SyscallErr::EACCES);
            // }
            let old_data_len = inode.metadata().inner.lock().data_len;
            if len < old_data_len {
                inode.metadata().inner.lock().data_len = len;
            } else if len > old_data_len {
                stack_trace!();
                inode.metadata().inner.lock().data_len = len;
                // fill with \0
                let buf = vec![0 as u8; len - old_data_len];
                stack_trace!();
                self.seek(SeekFrom::Start(old_data_len))?;
                self.write(&buf, OpenFlags::default()).await?;
                self.seek(SeekFrom::Start(old_pos))?;
            }
            Ok(())
        })
    }

    fn ioctl(&self, _command: usize, _value: usize) -> SyscallRet {
        log::warn!("[File::ioctl] unsupported");
        Ok(0)
    }
}

/// Default file(i.e. files in the disk)
pub struct DefaultFile {
    metadata: FileMeta,
}

impl DefaultFile {
    pub fn new(metadata: FileMeta) -> Self {
        Self { metadata }
    }
}

// #[async_trait]
impl File for DefaultFile {
    fn metadata(&self) -> &FileMeta {
        &self.metadata
    }

    /// For default file, data must be read from page cache first
    /// TODO: change to real async
    fn read<'a>(&'a self, buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        Box::pin(async move {
            stack_trace!();
            let _sum_guard = SumGuard::new();
            let (pos, inode) = self
                .metadata()
                .inner_get(|inner| (inner.pos, inner.inode.as_ref().cloned().unwrap()));
            let (data_len, page_cache) = inode
                .metadata()
                .inner_get(|inner| (inner.data_len, inner.page_cache.as_ref().cloned().unwrap()));
            // Calculate buf end according to inode meta
            // TODO now calculate buf end at first, which may need modifying
            // beacuse buf end may be changed by other thread
            let mut buf_end = data_len - pos;
            if buf_end > buf.len() {
                buf_end = buf.len();
            }

            let mut buf_offset = 0;
            let mut res = 0;
            let mut file_offset = pos;

            stack_trace!();
            while buf_offset < buf_end {
                // Get the page from page cache
                let page = page_cache.get_page(file_offset, None)?;

                // Read this page
                let page_offset = file_offset % PAGE_SIZE;
                let mut buf_offset_end = buf_offset + (PAGE_SIZE - page_offset);
                if buf_offset_end > buf_end {
                    buf_offset_end = buf_end;
                }

                let bytes = page
                    .read(page_offset, &mut buf[buf_offset..buf_offset_end])
                    .await?;
                file_offset += bytes;
                res += bytes;
                buf_offset += bytes;
            }

            let mut inner_lock = inode.metadata().inner.lock();
            inner_lock.st_atim = current_time_spec();
            match inner_lock.state {
                InodeState::DirtyData => inner_lock.state = InodeState::DirtyAll,
                InodeState::Synced => inner_lock.state = InodeState::DirtyInode,
                _ => {}
            }

            self.metadata().inner.lock().pos = file_offset;
            trace!("[DefaultFile::read]: read {} bytes", res);
            Ok(res)
        })
    }

    /// For default file, data must be written to page cache first
    fn write<'a>(&'a self, buf: &'a [u8], _flags: OpenFlags) -> AsyscallRet {
        Box::pin(async move {
            stack_trace!();
            let _sum_guard = SumGuard::new();
            let (pos, inode) = self
                .metadata()
                .inner_get(|inner| (inner.pos, inner.inode.as_ref().cloned().unwrap()));
            let page_cache = inode
                .metadata()
                .inner
                .lock()
                .page_cache
                .as_ref()
                .cloned()
                .unwrap();

            // Calculate buf end according to inode meta
            // TODO now calculate buf end at first, which may need modifying
            // beacuse buf end may change by other thread

            // let mut buf_end = inode_meta.data_len - file_meta.pos;
            // if buf_end > buf.len() {
            //     buf_end = buf.len();
            // }
            let buf_end = buf.len();

            let mut buf_offset = 0;
            let mut res = 0;
            let mut file_offset = pos;

            while buf_offset < buf_end {
                // Get the page from page cache
                let page = page_cache.get_page(file_offset, None)?;

                // Read this page
                let page_offset = file_offset % PAGE_SIZE;
                let mut buf_offset_end = buf_offset + (PAGE_SIZE - page_offset);
                if buf_offset_end > buf_end {
                    buf_offset_end = buf_end;
                }

                let bytes = page
                    .write(page_offset, &buf[buf_offset..buf_offset_end])
                    .await?;
                file_offset += bytes;
                res += bytes;
                buf_offset += bytes;
                inode.metadata().inner_get(|inner| {
                    if file_offset > inner.data_len {
                        inner.data_len = file_offset;
                    }
                });
            }

            let mut inner_lock = inode.metadata().inner.lock();
            inner_lock.st_atim = current_time_spec();
            inner_lock.st_ctim = inner_lock.st_atim;
            inner_lock.st_mtim = inner_lock.st_atim;
            match inner_lock.state {
                InodeState::DirtyInode => inner_lock.state = InodeState::DirtyAll,
                InodeState::Synced => inner_lock.state = InodeState::DirtyData,
                _ => {}
            }

            self.metadata().inner.lock().pos = file_offset;
            trace!(
                "[DefaultFile::write]: write {} bytes, buf len {}",
                res,
                buf.len()
            );
            Ok(res)
        })
    }
}
