use core::panic;

use alloc::sync::Arc;

use crate::{
    driver::{block::{BlockDevice, self}},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet, SyscallErr},
};

use self::{bpb::BootSector, fsinfo::FSInfo};

use super::{file_system::FileSystemMeta, FileSystem, FileSystemType, Inode};

// layouts
mod bpb;
mod disk_dentry;
mod fsinfo;
mod util;

const SECTOR_SIZE: usize = 512;

type Mutex<T> = SpinNoIrqLock<T>;

#[derive(Default)]
pub struct FAT32FileSystemMeta {
    sector_per_cluster: usize,    // sector count in a cluster
    reserved_sector_count: usize, // reserved sector count
    num_fat: usize,               // fat count
    fat_size: usize,              // sector count in a FAT
    total_sector_count: usize,    // total sector count
    root_cluster_id: usize,       // root cluster id
    fsinfo_sector_id: usize,      // FSInfo sector id
}


pub struct FAT32FSInfoMeta {
    free_count: u32,  // how many free clusters ? (0xFFFFFFFF for I dont know)
    nxt_free: u32,    // next free clusters id ? (0xFFFFFFFF for I dont know)
}

impl FAT32FSInfoMeta {
    pub fn default() -> Self {
        Self {
            free_count: 0xFFFFFFFF,
            nxt_free: 0xFFFFFFFF,
        }
    }
}

pub struct FAT32FileSystem {
    block_device: Arc<dyn BlockDevice>,
    fs_meta: FAT32FileSystemMeta,
    fsinfo_meta: FAT32FSInfoMeta,
}



impl FAT32FileSystem {
    pub fn new(block_device: Arc<dyn BlockDevice>) -> GeneralRet<Self> {
        let mut data: [u8; 512] = [0; 512];
        // read Boot Sector
        block_device.read_block(0, &mut data[..]);
        let bs = match BootSector::new(&data) {
            Some(bs) => bs,
            None => return Err(SyscallErr::ENONET)
        };
        let fs_meta = FAT32FileSystemMeta {
            sector_per_cluster: bs.BPB_SectorPerCluster as usize,
            reserved_sector_count: bs.BPB_ReservedSectorCount as usize,
            num_fat: bs.BPB_NumFATs as usize,
            fat_size: bs.BPB_FATsize32 as usize,
            total_sector_count: bs.BPB_TotSector32 as usize,
            root_cluster_id: bs.BPB_RootCluster as usize,
            fsinfo_sector_id: bs.BPB_FSInfo as usize,
        };
        // read FSInfo Sector
        block_device.read_block(0, &mut data[..]);
        let mut fsinfo_meta: FAT32FSInfoMeta = FAT32FSInfoMeta::default();
        if let Some(fsinfo) = FSInfo::new(&data) {
            fsinfo_meta = FAT32FSInfoMeta {
                free_count: fsinfo.FSI_Free_Count,
                nxt_free: fsinfo.FSI_Nxt_Free,
            }
        }
        let ret =  Self{
            block_device: Arc::clone(&block_device),
            fs_meta,
            fsinfo_meta
        };
        Ok(ret)
    }
}

impl FileSystem for FAT32FileSystem {
    fn create_root(
        &self,
        parent: Option<Arc<dyn Inode>>,
        mount_point: &str,
    ) -> GeneralRet<Arc<dyn Inode>> {
        todo!()
    }
    fn init(&mut self, mount_point: &str, ftype: FileSystemType) -> GeneralRet<()> {
        todo!()
    }
    fn mount(&self) {}
    fn dirty_inode(&self, inode: Arc<dyn Inode>) {}
    fn write_inode(&self, inode: Arc<dyn Inode>) -> SyscallRet {
        todo!()
    }
    fn sync_fs(&self) -> SyscallRet {
        todo!()
    }
    fn set_metadata(&mut self, meta_data: FileSystemMeta) {
        todo!()
    }
    fn metadata(&self) -> FileSystemMeta {
        todo!()
    }
}
