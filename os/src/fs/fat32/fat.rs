

use alloc::{sync::Arc, collections::LinkedList};
use xmas_elf::sections::SectionData;

use crate::{
    fs::{inode::{Inode, InodeMode, InodeMeta, InodeMetaInner}, File, Mutex},
    driver::{block::{BlockDevice, buffer_cache::LruBufferCache, BLOCK_DEVICE, self}},
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallRet, SyscallErr},
    mm::Page,
};

use super::{FAT32FileSystemMeta, FAT32FSInfoMeta};

const FATENTRY_PER_SECTOR : usize = 128;

/// if data_size < cluster_size*fatentry_per_cluster*fat_cache_count:
/// we don't have cache miss.
const FAT_CACHE_SIZE: usize = 1024;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FATSectorBufferState {
    Unassigned,
    Assigned,
    Dirty,
}

impl Default for FATSectorBufferState {
    fn default() -> Self {
        Self::Unassigned
    }
}

#[derive(Copy, Clone)]
struct FATInfoMeta {
    fat_start_sector: usize,
    fat_size: usize,
    fat_count: usize,
    cluster_count: usize, // max_cluster_id = cluster_count+1
    free_cluster_count: usize,
    nxt_free_cluster: usize,
}

#[derive(Clone, Copy)]
struct FATSectorBuffer {
    sector_no: usize,
    data: [u32; FATENTRY_PER_SECTOR],
    state: FATSectorBufferState,
}

impl Default for FATSectorBuffer {
    fn default() -> Self {
        Self {
            sector_no: usize::default(),
            data: [u32::default(); FATENTRY_PER_SECTOR],
            state: FATSectorBufferState::default(),
        }
    }
}

impl FATSectorBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sync(&mut self, block_device: Arc<dyn BlockDevice>, fatinfo: &FATInfoMeta) {
        if self.state == FATSectorBufferState::Dirty {
            for i in 0..fatinfo.fat_count {
                unsafe {
                    block_device.write_block(fatinfo.fat_start_sector + i * fatinfo.fat_size + self.sector_no,
                        core::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * core::mem::size_of::<u32>()));
                }
            }
            self.state = FATSectorBufferState::Assigned;
        }
    }

    pub fn free(&mut self, block_device: Arc<dyn BlockDevice>, fatinfo: &FATInfoMeta) {
        self.sync(Arc::clone(&block_device), fatinfo);
        self.state = FATSectorBufferState::Unassigned;
    }

    pub fn init(&mut self, block_device: Arc<dyn BlockDevice>, fatinfo: &FATInfoMeta, sector_no: usize) {
        self.free(Arc::clone(&block_device), fatinfo);
        self.state = FATSectorBufferState::Assigned;
        self.sector_no = sector_no;
        unsafe {
            block_device.read_block(fatinfo.fat_start_sector + self.sector_no,
                core::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut u8, self.data.len() * core::mem::size_of::<u32>()));
        }
    }

    pub fn read(&self, offset: usize) -> Option<u32> {
        if self.state == FATSectorBufferState::Unassigned || offset >= FATENTRY_PER_SECTOR {
            None
        } else {
            Some(self.data[offset])
        }
    }

    pub fn write(&mut self, offset: usize, val: u32) -> Option<()> {
        if self.state == FATSectorBufferState::Unassigned || offset >= FATENTRY_PER_SECTOR {
            None
        } else {
            self.data[offset] = val;
            self.state = FATSectorBufferState::Dirty;
            Some(())
        }
    }
}

struct FATBufferCache {
    data: Mutex<LinkedList<(usize, Arc<Mutex<FATSectorBuffer>>)>>,
    block_device: Arc<dyn BlockDevice>,
    fatinfo: FATInfoMeta,
}

impl FATBufferCache {
    pub fn new(block_device: Arc<dyn BlockDevice>, fatinfo: FATInfoMeta) -> Self {
        Self {
            data: Mutex::new(LinkedList::new()),
            block_device,
            fatinfo
        }
    }

    fn lookup_buffer_cache(&self, sector_no: usize) -> Arc<Mutex<FATSectorBuffer>> {
        let mut data_locked = self.data.lock();
        if let Some((idx, _)) = data_locked.iter().enumerate()
            .find(|(_, buffer)| buffer.0 == sector_no) {
                let buffer = data_locked.remove(idx);
                data_locked.push_front(buffer);
        } else {
            if data_locked.len() == FAT_CACHE_SIZE {
                let buffer_replaced = data_locked.pop_back().unwrap().1;
                assert_ne!(Arc::strong_count(&buffer_replaced), 1, "[FAT] Run out of sector buffers!");
                let mut buffer_replaced_locked = buffer_replaced.lock();
                buffer_replaced_locked.init(Arc::clone(&self.block_device), &self.fatinfo, sector_no);
                data_locked.push_front((sector_no, Arc::clone(&buffer_replaced)));
            } else {
                let mut new_buffer = FATSectorBuffer::new();
                new_buffer.init(Arc::clone(&self.block_device), &self.fatinfo, sector_no);
                data_locked.push_front((sector_no, Arc::new(Mutex::new(new_buffer))));
            }

        }
        Arc::clone(&data_locked.front().unwrap().1)
    }

    fn read_fat(&self, cluster_id: usize) -> Option<u32> {
        if cluster_id < 2 || cluster_id > self.fatinfo.cluster_count + 1 {
            None
        } else {
            let sector_id = cluster_id / FATENTRY_PER_SECTOR;
            let offset = cluster_id / FATENTRY_PER_SECTOR;
            let fat_sector = self.lookup_buffer_cache(sector_id);
            let fat_sector_locked = fat_sector.lock();
            assert_ne!(fat_sector_locked.state, FATSectorBufferState::Unassigned, "[FAT] Got an unassigned sector buffer! require = {}", sector_id);
            assert_ne!(fat_sector_locked.sector_no, sector_id, "[FAT] Sector buffer is wrong! require = {}, got = {}", sector_id, fat_sector_locked.sector_no);
            fat_sector_locked.read(offset)
        }
    }

    fn write_fat(&self, cluster_id: usize, val: u32) -> Option<()> {
        if cluster_id < 2 || cluster_id > self.fatinfo.cluster_count + 1 {
            None
        } else {
            let sector_id = cluster_id / FATENTRY_PER_SECTOR;
            let offset = cluster_id / FATENTRY_PER_SECTOR;
            let fat_sector = self.lookup_buffer_cache(sector_id);
            let mut fat_sector_locked = fat_sector.lock();
            assert_ne!(fat_sector_locked.state, FATSectorBufferState::Unassigned, "[FAT] Got an unassigned sector buffer! require = {}", sector_id);
            assert_ne!(fat_sector_locked.sector_no, sector_id, "[FAT] Sector buffer is wrong! require = {}, got = {}", sector_id, fat_sector_locked.sector_no);
            fat_sector_locked.write(offset, val)
        }
    }

    fn sync_buffer(&self, sector_id: usize) -> Option<()> {
        if sector_id > (self.fatinfo.cluster_count + 1) / FATENTRY_PER_SECTOR {
            None
        } else {
            let fat_sector = self.lookup_buffer_cache(sector_id);
            let mut fat_sector_locked = fat_sector.lock();
            assert_ne!(fat_sector_locked.state, FATSectorBufferState::Unassigned, "[FAT] Got an unassigned sector buffer! require = {}", sector_id);
            assert_ne!(fat_sector_locked.sector_no, sector_id, "[FAT] Sector buffer is wrong! require = {}, got = {}", sector_id, fat_sector_locked.sector_no);
            fat_sector_locked.sync(Arc::clone(&self.block_device), &self.fatinfo);
            Some(())
        }
    }
    
    fn sync_all_buffers(&self) {
        let mut data_locked = self.data.lock();
        for buffer in data_locked.iter() {
            buffer.1.lock().sync(Arc::clone(&self.block_device), &self.fatinfo);
        }
    }

}

impl Drop for FATBufferCache {
    fn drop(&mut self) {
        self.sync_all_buffers();
    }
}

pub struct FileAllocTable {
    block_device: Arc<dyn BlockDevice>,
    fatinfo: Mutex<FATInfoMeta>,
    fatcache: FATBufferCache,
}

impl FileAllocTable {
    pub fn new(block_device: Arc<dyn BlockDevice>, fsmeta: &FAT32FileSystemMeta, fsinfometa: &FAT32FSInfoMeta) -> Self {
        let fatinfo: FATInfoMeta = FATInfoMeta {
                fat_start_sector: (fsmeta.reserved_sector_count),
                fat_size: (fsmeta.fat_size),
                fat_count: (fsmeta.num_fat),
                cluster_count: ((fsmeta.total_sector_count - fsmeta.reserved_sector_count - fsmeta.fat_size * fsmeta.num_fat) / fsmeta.sector_per_cluster),
                free_cluster_count: (fsinfometa.free_count as usize),
                nxt_free_cluster: (fsinfometa.nxt_free as usize),
            };

        Self {
            block_device: Arc::clone(&block_device),
            fatinfo: Mutex::new(fatinfo),
            fatcache: FATBufferCache::new(Arc::clone(&block_device), fatinfo),
        }
    }

    fn read_fat(&self, cluster_id: usize) -> Option<u32> {
        self.fatcache.read_fat(cluster_id)
    }

    fn write_fat(&self, cluster_id: usize, val: u32) -> Option<()> {
        self.fatcache.write_fat(cluster_id, val)
    }
    
    fn sync_fat(&self) {
        self.fatcache.sync_all_buffers();
    }

    fn fsinfo(&self) -> FAT32FSInfoMeta {
        let fatinfo_lock = self.fatinfo.lock();
        FAT32FSInfoMeta {
            free_count: fatinfo_lock.free_cluster_count as u32,
            nxt_free: fatinfo_lock.nxt_free_cluster as u32,
        }
    }

    pub fn alloc_cluster(&self) -> Option<usize> {
        todo!()
    }

    pub fn free_cluster(&self, cluster_id: usize, prev_cluster_id: Option<usize>) -> Option<()> {
        todo!()
    }

    
}
