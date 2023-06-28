use crate::{driver::{block::BlockDevice, BLOCK_DEVICE}, utils::error::GeneralRet, fs::FileSystemType};
use alloc::{sync::Arc, vec::Vec, string::ToString};
use log::info;
use lazy_static::lazy_static;
use self::{
    bpb::BootSector,
    fat::FileAllocTable,
    fat32info::FAT32Info,
    inode::FAT32Inode
};

use super::{FileSystem, file_system::FileSystemMeta, posix::StatFlags};

mod bpb;
mod dentry;
mod fat;
mod fat32info;
mod file;
mod fsinfo;
mod inode;
mod time;
mod util;

pub const SECTOR_SIZE:          usize = 512;
const SNAME_LEN:        usize = 11;
const LNAME_MAXLEN:     usize = 256;
const BOOT_SECTOR_ID:       usize = 0;
const FATENTRY_PER_SECTOR:  usize = 128;
const FAT_CACHE_SIZE:       usize = 16;
const FSI_LEADSIG:          u32 = 0x41615252;
const FSI_STRUCSIG:         u32 = 0x61417272;
const FSI_TRAILSIG:         u32 = 0xAA550000;
const FSI_RESERVED1_SIZE:   usize = 480;
const FSI_RESERVED2_SIZE:   usize = 12;
const FSI_NOT_AVAILABLE:    u32 = 0xFFFFFFFF;

pub struct FAT32FileSystem {
    block_device: Arc<dyn BlockDevice>,
    meta: Option<FileSystemMeta>,
}

impl FAT32FileSystem {
    /// do nothing but store block device.
    pub fn new(block_device: Arc<dyn BlockDevice>) -> Self {
        Self {
            block_device: Arc::clone(&block_device),
            meta: None,
        }
    }

    pub fn do_nothing(&self) {

    }

    pub fn rootfs_mount(&mut self) -> Option<()> {
        let mut bs_data: [u8; SECTOR_SIZE] = [0; SECTOR_SIZE];
        self.block_device
            .read_block(BOOT_SECTOR_ID, &mut bs_data[..]);
        let raw_bs: BootSector = BootSector::new(&bs_data);
        if raw_bs.BPB_BytesPerSector as usize != SECTOR_SIZE
            || raw_bs.BPB_RootEntryCount != 0
            || raw_bs.BPB_TotSector16 != 0
            || raw_bs.BPB_FATsize16 != 0
            || raw_bs.BPB_FSVer != 0
        {
            return None;
        }
        let info = Arc::new(FAT32Info::new(raw_bs));
        let fat = Arc::new(FileAllocTable::new(
            Arc::clone(&self.block_device),
            Arc::clone(&info),
        ));
        let root_inode = FAT32Inode::new_root_dentry(Arc::clone(&fat), None, "/", info.root_cluster_id);
        self.meta = Some(FileSystemMeta {
            dev_name: "/dev/mmcblk".to_string(),
            ftype: FileSystemType::VFAT,
            root_inode: Some(Arc::new(root_inode)),
            mnt_flags: false,
            s_dirty: Vec::new(),
            flags: StatFlags::ST_NOSUID,
        });
        Some(())
    }

}

impl FileSystem for FAT32FileSystem {
    fn create_root(
            &self,
            _parent: Option<Arc<dyn super::Inode>>,
            _mount_point: &str,
        ) -> GeneralRet<Arc<dyn super::Inode>> {
        todo!();
    }
    fn set_metadata(&mut self, _metadata: FileSystemMeta) {
        todo!();
    }
    fn metadata(&self) -> FileSystemMeta {
        self.meta.as_ref().unwrap().clone()
    }
}

lazy_static! {
    pub static ref ROOT_FS: FAT32FileSystem = {
        let mut ret = FAT32FileSystem::new(Arc::clone(&BLOCK_DEVICE));
        ret.rootfs_mount();
        ret
    };
}

pub fn init() -> GeneralRet<()> {
    info!("start to init FAT32(rootfs):");
    ROOT_FS.do_nothing();
    info!("FAT32 init ok!");
    Ok(())
}
