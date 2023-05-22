use alloc::{boxed::Box, string::String, sync::Arc};

use crate::{
    config::fs::FILE_PAGE_SIZE,
    mm::memory_set::VmArea,
    utils::error::{AsyscallRet, GeneralRet, SyscallRet, AgeneralRet},
};

use super::{inode::Inode, Mutex, OpenFlags};

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

    /// For default file, data must be read from page cache first
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        Box::pin(async move {
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
                    .get_page(file_meta.pos)?;

                // Read this page
                let page_offset = file_offset % FILE_PAGE_SIZE;
                let mut buf_offset_end = buf_offset + (FILE_PAGE_SIZE - page_offset);
                if buf_offset_end > buf_end {
                    buf_offset_end = buf_end;
                }

                let bytes = page.read(page_offset, &mut buf[buf_offset..buf_offset_end])?;
                file_offset += bytes;
                res += bytes;
                buf_offset = buf_offset_end;
            }

            drop(inode_meta);
            file_meta.pos = file_offset;
            Ok(res as isize)
        })
    }

    /// For default file, data must be written to page cache first
    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        Box::pin(async move {
            let mut file_meta = self.metadata().inner.lock();
            let mut inode_meta = file_meta.inode.as_ref().unwrap().metadata().inner.lock();

            // Calculate buf end according to inode meta
            // TODO now calculate buf end at first, which may need modifying
            // beacuse buf end may change by other thread
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
                    .get_page(file_meta.pos)?;

                // Read this page
                let page_offset = file_offset % FILE_PAGE_SIZE;
                let mut buf_offset_end = buf_offset + (FILE_PAGE_SIZE - page_offset);
                if buf_offset_end > buf_end {
                    buf_offset_end = buf_end;
                }

                let bytes = page.write(page_offset, &buf[buf_offset..buf_offset_end])?;
                file_offset += bytes;
                res += bytes;
                buf_offset = buf_offset_end;
            }

            drop(inode_meta);
            file_meta.pos = file_offset;
            Ok(res as isize)
        })
    }
}
