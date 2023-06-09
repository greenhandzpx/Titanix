use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

use log::{error, warn, debug, info};

use crate::{
    fs::{inode::{Inode, InodeMode, InodeMeta, InodeMetaInner}, Mutex, fat32::{file, inode::FAT32FileType, util::shortname_checksum},},
    driver::{block::{BlockDevice, self}},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet, SyscallErr, self},
    mm::Page,
};

use super::{
    SHORTNAME_MAX_LEN,
    time::FAT32Timestamp,
    LONGNAME_MAX_LEN,
    inode::{FAT32Inode, FAT32InodeMeta},
    disk_dentry::{DiskLongDirEntry, DiskDirEntry}, file::FAT32File, SHORTNAME_LEN,
};

const ATTR_READ_ONLY: u8 = 0x01;
const ATTR_HIDDEN: u8 = 0x02;
const ATTR_SYSTEM: u8 = 0x04;
const ATTR_VOLUME_ID: u8 = 0x08;
const ATTR_DIRECTORY: u8 = 0x10;
const ATTR_ARCHIVE: u8 = 0x20;
const ATTR_LONG_NAME: u8 = 0x0F;
const DENTRY_SIZE: usize = 0x20;

const LDIR_NAME1_LEN: usize = 5;
const LDIR_NAME2_LEN: usize = 6;
const LDIR_NAME3_LEN: usize = 2;
const LDIR_NAME_LEN: usize = LDIR_NAME1_LEN + LDIR_NAME2_LEN + LDIR_NAME3_LEN;

macro_rules! dir_attr { ($buf: expr) => { $buf[11] }; }



pub struct FAT32DirEntry {

}

impl FAT32DirEntry {
    pub fn read_dentry(inode: Arc<FAT32Inode>, stream: &mut FAT32File, vec: &mut Vec<Arc<FAT32Inode>>) {
        let mut offset = 0;
        let mut buf: [u8; DENTRY_SIZE] = [0; DENTRY_SIZE];

        #[derive(PartialEq, Eq)]
        enum NextDentryType {
            Idle, Long, Short,
        }

        let mut next_type = NextDentryType::Idle;
        let mut next_id = 0;
        let mut filename_buf: [u8; LONGNAME_MAX_LEN] = [0; LONGNAME_MAX_LEN];
        let mut filename_offset = 0;
        let mut store_chksum: Option<u8> = None;

        loop {
            let ret = stream.read(&mut buf[..], offset);
            offset += ret;
            if ret == 0 {
                break;
            }
            let attr = dir_attr!(buf);
            if attr == ATTR_LONG_NAME {
                let cur_dentry = DiskLongDirEntry::new(&buf);
                let ord = cur_dentry.LDIR_Ord;
                if next_type == NextDentryType::Idle && (ord & 0x40) != 0x40 {
                    info!("not first dentry!");
                }

                if next_type == NextDentryType::Long && ord != next_id {
                    info!("ldir id not match!");
                }

                if next_type == NextDentryType::Short {
                    info!("fail to load a dentry!");
                    continue;
                }

                if store_chksum.is_none() {
                    store_chksum = Some(cur_dentry.LDIR_Chksum);
                } else {
                    if store_chksum.unwrap() != cur_dentry.LDIR_Chksum {
                        info!("shortname chksum not match!");
                    }
                }

                next_id = (ord & 0x3f) - 1;

                if next_id == 0 {
                    next_type = NextDentryType::Short;
                } else {
                    next_type = NextDentryType::Long;
                }

                let mut end = false;

                // load filename
                for i in 0..LDIR_NAME1_LEN {
                    filename_buf[filename_offset] = cur_dentry.LDIR_Name1[i] as u8;
                    filename_offset += 1;
                    if cur_dentry.LDIR_Name1[i] == 0 {
                        end = true;
                        break;
                    }
                }

                if end == false {
                    for i in 0..LDIR_NAME2_LEN {
                        filename_buf[filename_offset] = cur_dentry.LDIR_Name2[i] as u8;
                        filename_offset += 1;
                        if cur_dentry.LDIR_Name3[i] == 0 {
                            end = true;
                            break;
                        }
                    }
                }

                if end == false {
                    for i in 0..LDIR_NAME3_LEN {
                        filename_buf[filename_offset] = cur_dentry.LDIR_Name3[i] as u8;
                        filename_offset += 1;
                        if cur_dentry.LDIR_Name3[i] == 0 {
                            end = true;
                            break;
                        }
                    }
                }

                
            } else {
                let cur_dentry = DiskDirEntry::new(&buf);

                if next_type == NextDentryType::Long {
                    info!("expect long dentry but met with short!");
                }

                if store_chksum.is_some() {
                    let calc_chksum = shortname_checksum(&cur_dentry.DIR_Name[..]);
                    if calc_chksum != store_chksum.unwrap() {
                        info!("shortname chksum not match!");
                    }
                }

                
                store_chksum = None;
                next_type = NextDentryType::Idle;

                let first_cluster: usize = ((cur_dentry.DIR_FstClusHI as usize) << 16) | (cur_dentry.DIR_FstClusLO as usize);
                let file_size = cur_dentry.DIR_FileSize as usize;

                let raw_attr = cur_dentry.DIR_Attr;
                let file_type;
                if (raw_attr & ATTR_DIRECTORY) == ATTR_DIRECTORY {
                    file_type = FAT32FileType::Directory;
                } else {
                    file_type = FAT32FileType::Regfile;
                }


                let meta:FAT32InodeMeta = FAT32InodeMeta {
                    short_name: [0; SHORTNAME_MAX_LEN],
                    long_name: filename_buf,
                    attr: raw_attr,
                    crt_time: FAT32Timestamp {
                        date: cur_dentry.DIR_CrtDate,
                        time: cur_dentry.DIR_CrtTime,
                        tenms: cur_dentry.DIR_CrtTimeTenth,
                    },
                    wrt_time: FAT32Timestamp {
                        date: cur_dentry.DIR_WrtDate,
                        time: cur_dentry.DIR_WrtTime,
                        tenms: 0,
                    },
                    acc_time: FAT32Timestamp {
                        date: cur_dentry.DIR_LstAccDate,
                        time: 0,
                        tenms: 0,
                    }
                };
                
                let new_inode = FAT32Inode::new(Arc::clone(&inode.fat),
                Some(Arc::downgrade(&inode)),
                first_cluster,
                file_size,
                file_type,
                meta);
                vec.push(Arc::new(new_inode));

                while filename_offset > 0 {
                    filename_offset -= 1;
                    filename_buf[filename_offset] = 0;
                }
            }
        }

    }

    pub fn write_dentry(stream: &mut FAT32File, vec: &mut Vec<Arc<FAT32Inode>>) {
        let mut offset = 0;

        for inode in vec {
            let meta_locked = inode.meta.lock();

            let mut name_len = 0;
            while name_len < LONGNAME_MAX_LEN && meta_locked.long_name[name_len] != 0 {
                name_len += 1;
            }

            let sname_chksum = shortname_checksum(&meta_locked.short_name[..]);
            let mut wdata: [u8; DENTRY_SIZE] = [0; DENTRY_SIZE];

            let ldir_cnt = (name_len + LDIR_NAME_LEN - 1) / LDIR_NAME_LEN;
            for i in 0..ldir_cnt {
                let mut ldentry = DiskLongDirEntry::default();
                let id = ldir_cnt - i;
                ldentry.LDIR_Ord = (id as u8) | match i { 0 => 0x40, _ => 0};
                ldentry.LDIR_Attr = ATTR_LONG_NAME;
                ldentry.LDIR_Chksum = sname_chksum;
                let start_pos = (id - 1) * LDIR_NAME_LEN;
                for i in 0..LDIR_NAME_LEN {
                    let pos = start_pos + i;
                    let mut val: u16 = 0;
                    if pos > name_len {
                        val = 0xFFFF;
                    }
                    if pos < name_len {
                        val = meta_locked.long_name[i] as u16;
                    }
                    if i < LDIR_NAME1_LEN {
                        ldentry.LDIR_Name1[i] = val;
                    } else if i < LDIR_NAME1_LEN + LDIR_NAME2_LEN {
                        ldentry.LDIR_Name2[i - LDIR_NAME1_LEN] = val;
                    } else {
                        ldentry.LDIR_Name3[i - LDIR_NAME1_LEN - LDIR_NAME2_LEN] = val;
                    }
                }
                ldentry.store(&mut wdata[..]);
                let ret = stream.write(&wdata[..], offset);
                offset += ret;
            }

            let mut dentry = DiskDirEntry::default();
            let first_cluster = inode.content.lock().first_cluster();
            let file_size = inode.content.lock().modify_size(0);

            for i in 0..SHORTNAME_LEN {
                dentry.DIR_Name[i] = meta_locked.short_name[i];
            }
            dentry.DIR_Attr = meta_locked.attr;
            dentry.DIR_CrtTimeTenth = meta_locked.crt_time.tenms;
            dentry.DIR_CrtTime = meta_locked.crt_time.time;
            dentry.DIR_CrtDate = meta_locked.crt_time.date;
            dentry.DIR_LstAccDate = meta_locked.acc_time.date;
            dentry.DIR_FstClusHI = ((first_cluster >> 16) & 0xFFFF) as u16;
            dentry.DIR_WrtTime = meta_locked.wrt_time.time;
            dentry.DIR_WrtDate = meta_locked.wrt_time.date;
            dentry.DIR_FstClusLO = (first_cluster & 0xFFFF) as u16;
            dentry.DIR_FileSize = file_size as u32;

            dentry.store(&mut wdata[..]);
            let ret = stream.write(&wdata[..], offset);
            offset += ret;
        }
    }
}
