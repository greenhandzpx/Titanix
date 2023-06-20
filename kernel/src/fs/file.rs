use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use log::trace;

use crate::{
    config::mm::PAGE_SIZE,
    fs::InodeState,
    mm::memory_space::VmArea,
    processor::SumGuard,
    stack_trace,
    timer::get_time_spec,
    utils::{
        async_tools::block_on,
        error::{AsyscallRet, GeneralRet, SyscallRet},
    },
};

use super::{inode::Inode, Mutex, OpenFlags};

bitflags! {
    /// renameat flag
    pub struct Renameat2Flags: u32 {
        /// Go back to renameat
        const RENAME_NONE = 0;
        /// Atomically exchange oldpath and newpath.
        const RENAME_EXCHANGE = 1 << 1;
        /// Don't overwrite newpath of the rename. Return an error if newpath already exists.
        const RENAME_NOREPLACE = 1 << 0;
        /// This operation makes sense only for overlay/union filesystem implementations.
        const RENAME_WHITEOUT = 1 << 2;
    }
}

pub struct FileMeta {
    /// path to file, need to be absolute path
    pub path: String,
    /// open flags
    pub flags: OpenFlags,
    /// Mutable,
    pub inner: Mutex<FileMetaInner>,
}

impl FileMeta {
    pub fn inner_get<T>(&self, f: impl FnOnce(&mut FileMetaInner) -> T) -> T {
        f(&mut self.inner.lock())
    }
}
pub struct FileMetaInner {
    /// inode to which this file refers
    pub inode: Option<Arc<dyn Inode>>,
    /// file offset
    pub pos: usize,
    // TODO: add more like file version
    /// current read dirent index
    pub dirent_index: usize,
}

// #[async_trait]
pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    /// For default file, data must be read from page cache first
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet;
    /// For default file, data must be written to page cache first
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet;

    fn pollin(&self) -> GeneralRet<bool> {
        todo!()
    }

    fn pollout(&self) -> GeneralRet<bool> {
        todo!()
    }

    /// For default file, data must be read from page cache first
    fn sync_read(&self, _buf: &mut [u8]) -> SyscallRet {
        todo!()
    }
    fn sync_write(&self, _buf: &[u8]) -> SyscallRet {
        todo!()
    }

    fn seek(&self, _offset: usize) -> SyscallRet {
        todo!()
    }

    fn offset(&self) -> SyscallRet {
        todo!()
    }

    /// Read all data from this file synchronously
    /// TODO: add async version
    fn sync_read_all(&self) -> GeneralRet<Vec<u8>> {
        todo!()
    }

    // TODO: not sure the args
    fn mmap(&self) -> GeneralRet<VmArea> {
        todo!()
    }

    fn metadata(&self) -> &FileMeta;
}

impl dyn File {
    pub fn open(inode: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        let file = inode.open(inode.clone(), flags)?;
        file.metadata().inner.lock().inode = Some(inode);
        Ok(file)
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
    fn readable(&self) -> bool {
        self.metadata().flags.contains(OpenFlags::RDONLY)
            || self.metadata().flags.contains(OpenFlags::RDWR)
    }

    fn writable(&self) -> bool {
        self.metadata().flags.contains(OpenFlags::RDWR)
    }

    fn metadata(&self) -> &FileMeta {
        &self.metadata
    }

    fn seek(&self, offset: usize) -> SyscallRet {
        self.metadata.inner.lock().pos = offset;
        Ok(offset as isize)
    }

    /// For default file, data must be read from page cache first
    /// TODO: change to real async
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        Box::pin(async move {
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
            inner_lock.st_atim = get_time_spec();
            inner_lock.st_mtim = inner_lock.st_atim;
            inner_lock.st_ctim = inner_lock.st_atim;
            match inner_lock.state {
                InodeState::DirtyData => inner_lock.state = InodeState::DirtyAll,
                InodeState::Synced => inner_lock.state = InodeState::DirtyInode,
                _ => {}
            }

            self.metadata().inner.lock().pos = file_offset;
            trace!("[DefaultFile::read]: read {} bytes", res);
            Ok(res as isize)
        })
    }

    /// For default file, data must be written to page cache first
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
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
            inner_lock.st_atim = get_time_spec();
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
            Ok(res as isize)
        })
    }

    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        block_on(self.read(buf))
    }

    fn sync_read_all(&self) -> GeneralRet<Vec<u8>> {
        // let mut inner = self.inner.lock();
        let mut buffer = [0u8; PAGE_SIZE];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = self.sync_read(&mut buffer)?;
            if len == 0 {
                break;
            }
            v.extend_from_slice(&buffer[..len as usize]);
        }
        Ok(v)
    }
}
