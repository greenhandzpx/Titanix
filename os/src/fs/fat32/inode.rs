
use alloc::{sync::{Arc, Weak}, vec::Vec, collections::BTreeMap, string::String};
use crate::{
    fs::{inode::{Inode, InodeMode, InodeMeta, InodeMetaInner}, Mutex, fat32_tmp::Fat32File, File, file},
    driver::{block::{BlockDevice, self, BLOCK_DEVICE}},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet, SyscallErr},
    mm::Page,
};

use super::{time::FAT32Timestamp, fat::FileAllocTable, FAT32FileSystemMeta, FAT32Info, file::FAT32File, SHORTNAME_MAX_LEN, LONGNAME_MAX_LEN, dentry::FAT32DirEntry};


#[derive(Copy, Clone)]
pub enum FAT32FileType {
    Regfile,
    Directory,
}

#[derive(Copy, Clone)]
pub struct FAT32InodeMeta {
    pub short_name: [u8; SHORTNAME_MAX_LEN],
    pub long_name: [u8; LONGNAME_MAX_LEN],
    pub attr: u8,
    pub crt_time: FAT32Timestamp,
    pub acc_time: FAT32Timestamp,
    pub wrt_time: FAT32Timestamp,
}

impl FAT32InodeMeta {
    pub fn default() -> Self {
        Self {
            short_name: [0; SHORTNAME_MAX_LEN],
            long_name: [0; LONGNAME_MAX_LEN],
            attr: 0,
            crt_time: FAT32Timestamp::default(),
            acc_time: FAT32Timestamp::default(),
            wrt_time: FAT32Timestamp::default(),
        }
    }
}


pub struct FAT32Inode {
    pub ftype: FAT32FileType,
    pub fat: Arc<FileAllocTable>,
    pub meta: Mutex<FAT32InodeMeta>,
    pub content: Mutex<FAT32File>,
    pub father: Option<Weak<FAT32Inode>>,
    pub child: Mutex<Vec<Arc<FAT32Inode>>>,
    pub child_loaded: Mutex<bool>,
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
            child: Mutex::new(Vec::new()),
            child_loaded: Mutex::new(false),
        }
    }

    pub fn load_inode(&self, arc_self: Arc<Self>) -> Option<()> {
        match self.ftype {
            FAT32FileType::Regfile => None,
            FAT32FileType::Directory => {
                let mut loaded = self.child_loaded.lock();
                if *loaded {
                    None
                } else {
                    let mut content_locked = self.content.lock();
                    let mut child_locked = self.child.lock();
                    FAT32DirEntry::read_dentry(arc_self, &mut content_locked, &mut child_locked);
                    *loaded = true;
                    Some(())
                }
            }
        }
    }

    pub fn sync_inode(&self) -> Option<()> {
        match self.ftype {
            FAT32FileType::Regfile => None,
            FAT32FileType::Directory => {
                let mut loaded = self.child_loaded.lock();
                if *loaded {
                    let mut content_locked = self.content.lock();
                    let mut child_locked = self.child.lock();
                    FAT32DirEntry::write_dentry(&mut content_locked, &mut child_locked);
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    pub fn read(&self, data: &mut [u8], offset: usize) -> usize {
        self.content.lock().read(data, offset)
    }

    pub fn write(&self, data: &[u8], offset: usize) -> usize {
        self.content.lock().write(data, offset)
    }
}
