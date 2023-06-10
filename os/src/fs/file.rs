use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use log::debug;

use crate::{
    config::mm::PAGE_SIZE,
    mm::memory_set::VmArea,
    processor::SumGuard,
    stack_trace,
    utils::error::{AgeneralRet, AsyscallRet, GeneralRet, SyscallRet},
};

use super::{inode::Inode, InodeState, Mutex, OpenFlags};

pub struct FileMeta {
    /// path to file, need to be absolute path
    pub path: String,
    /// open flags
    pub flags: OpenFlags,
    /// Mutable,
    pub inner: Mutex<FileMetaInner>,
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
    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        todo!()
    }
    fn sync_write(&self, buf: &[u8]) -> SyscallRet {
        todo!()
    }

    fn seek(&self, offset: usize) -> SyscallRet {
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
        Box::pin(async move { self.sync_read(buf) })
    }

    /// For default file, data must be written to page cache first
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        Box::pin(async move {
            stack_trace!();
            let _sum_guard = SumGuard::new();
            let mut file_meta = self.metadata().inner.lock();
            let mut inode_meta = file_meta.inode.as_ref().unwrap().metadata().inner.lock();

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
            let mut file_offset = file_meta.pos;

            while buf_offset < buf_end {
                // Get the page from page cache
                let page = inode_meta
                    .page_cache
                    .as_mut()
                    .unwrap()
                    .get_page(file_offset)?;

                // Read this page
                let page_offset = file_offset % PAGE_SIZE;
                let mut buf_offset_end = buf_offset + (PAGE_SIZE - page_offset);
                if buf_offset_end > buf_end {
                    buf_offset_end = buf_end;
                }

                let bytes = page.write(page_offset, &buf[buf_offset..buf_offset_end])?;
                file_offset += bytes;
                res += bytes;
                buf_offset += bytes;
                if file_offset > inode_meta.data_len {
                    inode_meta.data_len = file_offset;
                }
            }

            drop(inode_meta);
            file_meta.pos = file_offset;
            debug!(
                "[DefaultFile::write]: write {} bytes, buf len {}",
                res,
                buf.len()
            );
            Ok(res as isize)
        })
    }

    fn sync_read(&self, buf: &mut [u8]) -> SyscallRet {
        let _sum_guard = SumGuard::new();
        let mut file_meta = self.metadata().inner.lock();
        let mut inode_meta = file_meta.inode.as_ref().unwrap().metadata().inner.lock();

        // Calculate buf end according to inode meta
        // TODO now calculate buf end at first, which may need modifying
        // beacuse buf end may be changed by other thread
        let mut buf_end = inode_meta.data_len - file_meta.pos;
        if buf_end > buf.len() {
            buf_end = buf.len();
        }

        let mut buf_offset = 0;
        let mut res = 0;
        let mut file_offset = file_meta.pos;

        while buf_offset < buf_end {
            // Get the page from page cache
            let page = inode_meta
                .page_cache
                .as_mut()
                .unwrap()
                .get_page(file_offset)?;

            // Read this page
            let page_offset = file_offset % PAGE_SIZE;
            let mut buf_offset_end = buf_offset + (PAGE_SIZE - page_offset);
            if buf_offset_end > buf_end {
                buf_offset_end = buf_end;
            }

            let bytes = page.read(page_offset, &mut buf[buf_offset..buf_offset_end])?;
            file_offset += bytes;
            res += bytes;
            buf_offset += bytes;
        }

        drop(inode_meta);
        file_meta.pos = file_offset;
        debug!("[DefaultFile::read]: read {} bytes", res);
        Ok(res as isize)
    }

    fn sync_read_all(&self) -> GeneralRet<Vec<u8>> {
        // let mut inner = self.inner.lock();
        let mut buffer = [0u8; 512];
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
