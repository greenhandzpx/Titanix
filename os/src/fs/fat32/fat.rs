
use alloc::sync::{Arc, Weak};

use crate::{
    fs::{inode::{Inode, InodeMode, InodeMeta, InodeMetaInner}, File},
    driver::{block::{BlockDevice, buffer_cache::LruBufferCache}},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet, SyscallErr},
    mm::Page,
};

use super::{FAT32FileSystemMeta, FAT32FSInfoMeta};

const FATENTRY_PER_SECTOR : usize = 128;

// I am doing cached read and write, don't be too hurry.

struct FATSector {
    sector_no: usize,
    data: [u32; FATENTRY_PER_SECTOR],
    dirty: bool,
    fat: Arc<FileAllocTable>,
}

impl FATSector {
    fn new(sector_no: usize, fat: Arc<FileAllocTable>) -> FATSector {
        let mut data: [u32; FATENTRY_PER_SECTOR] = [0; FATENTRY_PER_SECTOR];
        unsafe {
            fat.block_device.read_block(fat.fat_start_sector + sector_no,
                core::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, data.len() * core::mem::size_of::<u32>()));
        }
        Self {
            sector_no,
            data,
            dirty: false,
            fat: Arc::clone(&fat),
        }
    }

    fn sync(&mut self) {
        if self.dirty {
            for i in 0..self.fat.fat_count {
                unsafe {
                    self.fat.block_device.write_block(self.fat.fat_start_sector + i * self.fat.fat_size + self.sector_no,
                        core::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * core::mem::size_of::<u32>()));
                }
            }
            self.dirty = false;
        }
    }

    fn read(&self, offset: usize) -> u32 {
        self.data[offset]
    }

    fn write(&mut self, offset: usize, val: u32) {
        self.dirty = true;
        self.data[offset] = val;
    }
}


pub struct FileAllocTable {
    block_device: Arc<dyn BlockDevice>,

    fat_start_sector: usize,
    fat_size: usize,
    fat_count: usize,
    cluster_count: usize, // max_cluster_id = cluster_count+1

    free_cluster_count: usize,
    nxt_free_cluster: usize,

    fatcache: LruBufferCache,
}



impl FileAllocTable {

    pub fn new(block_device: Arc<dyn BlockDevice>, fsmeta: &FAT32FileSystemMeta, fsinfometa: &FAT32FSInfoMeta) -> Self {
        Self {
            block_device: Arc::clone(&block_device),
            fat_start_sector: fsmeta.reserved_sector_count,
            fat_size: fsmeta.fat_size,
            fat_count: fsmeta.num_fat,
            cluster_count: (fsmeta.total_sector_count - fsmeta.reserved_sector_count - fsmeta.fat_size * fsmeta.num_fat) / fsmeta.sector_per_cluster,
            free_cluster_count: fsinfometa.free_count as usize,
            nxt_free_cluster: fsinfometa.nxt_free as usize,
            fatcache: LruBufferCache::new(Arc::clone(&block_device)),
        }
    }

    // TODO: cache read and write

    fn read_fat(&self, cluster_id: usize) -> Option<u32> {
        if cluster_id < 2 || cluster_id > self.cluster_count + 1 { return None; }
        let mut buf: [u8; 512] = [0; 512];
        self.block_device.read_block(self.fat_start_sector + cluster_id / FATENTRY_PER_SECTOR, &mut buf[..]);
        unsafe {
            Some(*((&mut buf as *mut _ as *mut u32).add(cluster_id % FATENTRY_PER_SECTOR)))
        }
    }

    fn write_fat(&self, cluster_id: usize, val: u32) -> Option<()> {
        if cluster_id < 2 || cluster_id > self.cluster_count + 1 { return None; }
        let mut buf: [u8; 512] = [0; 512];
        
        self.block_device.read_block(self.fat_start_sector + cluster_id / FATENTRY_PER_SECTOR, &mut buf[..]);
        unsafe {
            *((&mut buf as *mut _ as *mut u32).add(cluster_id % FATENTRY_PER_SECTOR)) = val;
        }
        self.block_device.write_block(self.fat_start_sector + cluster_id / FATENTRY_PER_SECTOR, &buf[..]);
        Some(())
    }
    
    fn sync_fat(&self) {

    }

    pub fn alloc_cluster(&self) -> Option<usize> {
        None
    }

    pub fn free_cluster(&self, cluster_id: usize, prev_cluster_id: Option<usize>) -> Option<()> {
        None
    }

    
}

