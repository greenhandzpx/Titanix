use alloc::sync::Arc;
use log::debug;
use crate::{
    driver::block::BlockDevice,
    fs::Inode,
};

use self::{
    bpb::BootSector,
    inode::FAT32Inode,
    fat32info::FAT32Info,
    fat::FileAllocTable
};

mod bpb;
mod disk_dentry;
mod fsinfo;
mod util;
mod fat;
mod dentry;
mod inode;
mod time;
mod fat32info;

const SECTOR_SIZE:          usize = 512;
const SHORTNAME_LEN:        usize = 11;
const LONGNAME_LEN:         usize = 255;
const BOOT_SECTOR_ID:       usize = 0;
const FATENTRY_PER_SECTOR:  usize = 128;
const FAT_CACHE_SIZE:       usize = 16;
const FSI_LEADSIG:          u32 = 0x41615252;
const FSI_STRUCSIG:         u32 = 0x61417272;
const FSI_TRAILSIG:         u32 = 0xAA550000;
const FSI_RESERVED1_SIZE:   usize = 480;
const FSI_RESERVED2_SIZE:   usize = 12;
const FSI_NOT_AVAILABLE:    u32 = 0xFFFFFFFF;

pub struct FAT32FileSystemMeta {
    info: Arc<FAT32Info>,
    fat: FileAllocTable,
    root_inode: Arc<FAT32Inode>,
}

pub struct FAT32FileSystem {
    block_device: Arc<dyn BlockDevice>,
    meta: Option<FAT32FileSystemMeta>,
}

impl FAT32FileSystem {
    /// 传入一个 Block Device，但是不做任何事情。
    pub fn new(block_device: Arc<dyn BlockDevice>) -> Self {
        Self {
            block_device: Arc::clone(&block_device),
            meta: None,
        }
    }

    pub fn mount(&mut self, mount_point: &str) -> Option<()> {
        if self.meta.is_some() {
            debug!("尝试挂载一个已挂载的FAT32文件系统");
            return None;
        }
        let mut bs_data: [u8; SECTOR_SIZE] = [0; SECTOR_SIZE];
        self.block_device.read_block(BOOT_SECTOR_ID, &mut bs_data[..]);
        let raw_bs: BootSector = BootSector::new(&bs_data);
        if raw_bs.BPB_BytesPerSector as usize != SECTOR_SIZE
            || raw_bs.BPB_RootEntryCount != 0
            || raw_bs.BPB_TotSector16 != 0
            || raw_bs.BPB_FATsize16 != 0
            || raw_bs.BPB_FSVer != 0 {
                return None;
        }
        let info = Arc::new(FAT32Info::new(raw_bs));
        let fat = FileAllocTable::new(Arc::clone(&self.block_device), Arc::clone(&info));
        todo!("create Root Inode");

    }

    pub fn unmount(&mut self) -> Option<()> {
        if self.meta.is_none() {
            debug!("尝试卸载一个未挂载的FAT32文件系统");
            None
        } else if self.sync_fs().is_some() {
            self.meta = None;
            Some(())
        } else {
            debug!("卸载失败了！");
            None
        }
        
    }

    pub fn root_inode(&self) -> Option<Arc<dyn Inode>> {
        todo!()
    }

    pub fn sync_fs(&self) -> Option<()> {
        if self.meta.is_none() {
            debug!("尝试同步一个未挂载的FAT32文件系统");
            None
        } else {
            self.meta.as_ref().unwrap().fat.sync_fat();
            self.meta.as_ref().unwrap().root_inode.sync_inode();
            Some(())
        }
    }

}
