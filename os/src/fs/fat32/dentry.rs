
use alloc::{
    sync::Arc,
    vec::Vec,
};

use log::{error, warn, debug, info};

use crate::{
    fs::{inode::{Inode, InodeMode, InodeMeta, InodeMetaInner}, Mutex,},
    driver::{block::{BlockDevice, self}},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet, SyscallErr, self},
    mm::Page,
};

use super::{
    SHORTNAME_LEN,
    time::FAT32Timestamp,
    LONGNAME_LEN,
    inode::FAT32Inode,
    disk_dentry::{DiskLongDirEntry, DiskDirEntry},
};

const ATTR_READ_ONLY: u8 = 0x01;
const ATTR_HIDDEN: u8 = 0x02;
const ATTR_SYSTEM: u8 = 0x04;
const ATTR_VOLUME_ID: u8 = 0x08;
const ATTR_DIRECTORY: u8 = 0x10;
const ATTR_ARCHIVE: u8 = 0x20;
const ATTR_LONG_NAME: u8 = 0x0F;
const DENTRY_SIZE: usize = 0x20;

const ATTR_ADDR: usize = 11;

/// 1:1 对应磁盘上一个文件的 DirEntry
/// 可能由多个 DirEntry 组成
pub struct FAT32DirEntry {
    block_device: Arc<dyn BlockDevice>,
    belong_dir: Arc<FAT32Inode>,
    inode: Arc<FAT32Inode>,
    info: Mutex<FAT32FileInfo>,
}


pub struct FAT32FileInfo {
    short_name: [u8; SHORTNAME_LEN],
    long_name: [u16; LONGNAME_LEN],
    attr: u8,
    crt_time: FAT32Timestamp,
    acc_time: FAT32Timestamp,
    wrt_time: FAT32Timestamp,
    fst_cluster: u32,
    file_size: u32,
}

impl FAT32FileInfo {
    pub fn new() -> Self {
        todo!()
    }
}

impl FAT32DirEntry {
    pub fn new(
        block_device: Arc<dyn BlockDevice>,
        belong_dir: Arc<FAT32Inode>,
        dentry_data: &[u8],
        data_ptr: &mut usize,
    ) -> Option<Self> {
        let data: &[u8] = &dentry_data[*data_ptr..(*data_ptr + DENTRY_SIZE)];
        *data_ptr += DENTRY_SIZE;
        if data[0] == 0x00 {

        }



        let inode = FAT32DirEntry::fuck();
        let info = FAT32FileInfo::new();
        Some(Self {
            block_device: Arc::clone(&block_device),
            belong_dir: Arc::clone(&belong_dir),
            inode: Arc::new(inode),
            info: Mutex::new(info),
        })
    }

    pub fn fuck() -> FAT32Inode {
        todo!();
    }

}

pub struct DentryReader {

}

impl DentryReader {
    fn read_data(&self, data: &mut [u8]) -> Option<()> {
        todo!();
    }
}

pub fn resolve_dentry(dentry_reader: DentryReader) -> Vec<FAT32Inode> {
    let mut ret = Vec::new();
    let mut data: [u8; DENTRY_SIZE] = [0; DENTRY_SIZE];

    let mut nxt_longdir = false; // if we expect nxt is long dir
    let mut nxt_shortdir = false; // if we expect nxt is short dir
    let mut nxt_longdir_id = 0; // next longdir id
    let mut chksum: u8 = 0;
    while dentry_reader.read_data(&mut data).is_some() {
        if data[0] == 0x00 { break; }
        if data[0] == 0xE5 { continue; }
        if data[0] == 0x05 { data[0] = 0xE5; }
        if data[ATTR_ADDR] == ATTR_LONG_NAME {
            let processed_data = DiskLongDirEntry::new(&data);
            let mut id = processed_data.LDIR_Ord;
            let first_longdir = (id & 0x40) == 0x40;
            id &= !(0x40 as u8);
            if nxt_shortdir {
                error!("we expect a short dir, but meet with a long dir");
            } else if nxt_longdir {
                if first_longdir {
                    error!("we expect not first long dir, but meet with first long dir");
                }
                if nxt_longdir_id != id {
                    error!("we expect long dir id {}, but meet with id {}", nxt_longdir_id, id);
                }
            } else {
                if first_longdir == false {
                    error!("we expect first long dir or short dir, but meet with not first long dir");
                }
            }
            let mut name_part: [u16; 13] = [0; 13];
            for i in 0..5 { name_part[i] = processed_data.LDIR_Name1[i]; }
            for i in 5..11 { name_part[i] = processed_data.LDIR_Name2[i - 5]; }
            for i in 11..13 { name_part[i] = processed_data.LDIR_Name3[i - 11]; }
            if id == 1 {
                nxt_longdir = false;
                nxt_shortdir = true;
            } else {
                nxt_longdir = true;
                nxt_shortdir = false;
                nxt_longdir_id = id - 1;
            }
            if first_longdir {
                chksum = processed_data.LDIR_Chksum;
            } else {
                if chksum != processed_data.LDIR_Chksum {
                    error!("LDIR chksum is not consistent!, {} != {}", chksum, processed_data.LDIR_Chksum);
                }
            }
        } else {
            let processed_data = DiskDirEntry::new(&data);
            todo!();
        }
    }
    ret
}