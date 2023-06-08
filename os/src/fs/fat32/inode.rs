
use alloc::{sync::{Arc, Weak}, vec::Vec};
use crate::{
    fs::{inode::{Inode, InodeMode, InodeMeta, InodeMetaInner}, Mutex, fat32_tmp::Fat32File, File, file},
    driver::{block::{BlockDevice, self, BLOCK_DEVICE}},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet, SyscallErr},
    mm::Page,
};

use super::{SHORTNAME_LEN, time::FAT32Timestamp, LONGNAME_LEN, fat::FileAllocTable, FAT32FileSystemMeta, FAT32Info, file::FAT32File};


#[derive(Copy, Clone)]
pub enum FAT32FileType {
    Regfile,
    Directory,
}

#[derive(Copy, Clone)]
pub struct FAT32InodeMeta {
    short_name: [u8; SHORTNAME_LEN],
    long_name: [u16; LONGNAME_LEN],
    attr: u8,
    crt_time: FAT32Timestamp,
    acc_time: FAT32Timestamp,
    wrt_time: FAT32Timestamp,
}

impl FAT32InodeMeta {
    pub fn default() -> Self {
        Self {
            short_name: [0; SHORTNAME_LEN],
            long_name: [0; LONGNAME_LEN],
            attr: 0,
            crt_time: FAT32Timestamp::default(),
            acc_time: FAT32Timestamp::default(),
            wrt_time: FAT32Timestamp::default(),
        }
    }
}

pub struct FAT32Inode {
    ftype: FAT32FileType,
    fat: Arc<FileAllocTable>,
    meta: Mutex<FAT32InodeMeta>,
    content: Mutex<FAT32File>,
    father: Option<Weak<FAT32Inode>>,
    sons: Option<Vec<Arc<FAT32Inode>>>,
}

impl FAT32Inode {
    pub fn new(
        fat: Arc<FileAllocTable>,
        father: Option<Weak<FAT32Inode>>,
        first_cluster: usize,
        file_size: usize,
        file_type: FAT32FileType,
        meta: FAT32InodeMeta,
    ) -> Self {
        Self {
            ftype: file_type,
            fat: Arc::clone(&fat),
            meta: Mutex::new(meta),
            content: Mutex::new(FAT32File::new(
                        Arc::clone(&fat),
                        first_cluster,
                        match file_type {
                            FAT32FileType::Regfile => Some(file_size),
                            FAT32FileType::Directory => None,
                        }
                    )),
            father,
            sons: None,
        }
    }

    pub fn sync_inode(&self) {
        match self.ftype {
            FAT32FileType::Directory => {
                if self.sons.is_some() {
                    todo!();
                }
            }
            _ => {}
        }
    }

    pub fn read(&self, data: &mut [u8], offset: usize) -> usize {
        self.content.lock().read(data, offset)
    }

    pub fn write(&self, data: &[u8], offset: usize) -> usize {
        self.content.lock().write(data, offset)
    }
}
