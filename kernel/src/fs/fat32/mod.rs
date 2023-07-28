use self::{bpb::BootSector, fat::FileAllocTable, fat32info::FAT32Info, inode::FAT32Inode};
use crate::{
    driver::BlockDevice,
    fs::FileSystemType,
    utils::error::{GeneralRet, SyscallErr},
};
use alloc::{string::ToString, sync::Arc, vec::Vec};

use super::{ffi::StatFlags, file_system::FileSystemMeta, FileSystem, Inode};

mod bpb;
mod dentry;
mod fat;
mod fat32info;
mod file;
mod fsinfo;
mod inode;
mod time;
mod util;

pub const SECTOR_SIZE: usize = 512;
const SNAME_LEN: usize = 11;
const LNAME_MAXLEN: usize = 256;
const BOOT_SECTOR_ID: usize = 0;
const FATENTRY_PER_SECTOR: usize = 128;
const FAT_CACHE_SIZE: usize = 16;
const FSI_LEADSIG: u32 = 0x41615252;
const FSI_STRUCSIG: u32 = 0x61417272;
const FSI_TRAILSIG: u32 = 0xAA550000;
const FSI_RESERVED1_SIZE: usize = 480;
const FSI_RESERVED2_SIZE: usize = 12;
const FSI_NOT_AVAILABLE: u32 = 0xFFFFFFFF;

pub struct FAT32FileSystem {
    fat: Arc<FileAllocTable>,
    meta: FileSystemMeta,
}

impl FAT32FileSystem {
    /// do nothing but store block device.
    pub fn new(
        block_device: Arc<dyn BlockDevice>,
        mount_point: &str,
        dev_name: &str,
        fstype: FileSystemType,
        flags: StatFlags,
        fa_inode: Option<Arc<dyn Inode>>,
        covered_inode: Option<Arc<dyn Inode>>,
    ) -> GeneralRet<Self> {
        let mut bs_data: [u8; SECTOR_SIZE] = [0; SECTOR_SIZE];
        block_device.read_block(BOOT_SECTOR_ID, &mut bs_data[..]);
        let raw_bs: BootSector = BootSector::new(&bs_data);
        if raw_bs.BPB_BytesPerSector as usize != SECTOR_SIZE
            || raw_bs.BPB_RootEntryCount != 0
            || raw_bs.BPB_TotSector16 != 0
            || raw_bs.BPB_FATsize16 != 0
            || raw_bs.BPB_FSVer != 0
        {
            return Err(SyscallErr::EINVAL);
        }
        let info = Arc::new(FAT32Info::new(raw_bs));
        let fat = Arc::new(FileAllocTable::new(
            Arc::clone(&block_device),
            Arc::clone(&info),
        ));
        let root_inode = FAT32Inode::new_root(
            Arc::clone(&fat),
            Option::clone(&fa_inode),
            mount_point,
            info.root_cluster_id,
        );
        let root_inode: Arc<dyn Inode> = Arc::new(root_inode);
        let meta = FileSystemMeta {
            dev_name: dev_name.to_string(),
            mount_point: mount_point.to_string(),
            fstype,
            flags,
            root_inode,
            fa_inode,
            covered_inode,
            s_dirty: Vec::new(),
        };
        let ret = Self {
            fat: Arc::clone(&fat),
            meta,
        };
        Ok(ret)
    }
}

impl FileSystem for FAT32FileSystem {
    // fn sync_fs(&self) {
    //     todo!()
    // }

    fn metadata(&self) -> &FileSystemMeta {
        &self.meta
    }
}

impl Drop for FAT32FileSystem {
    fn drop(&mut self) {
        self.sync_fs();
    }
}
