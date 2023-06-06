use alloc::{sync::Arc, collections::LinkedList};

use crate::{
    driver::block::{BlockDevice, self},
    fs::Mutex, utils::debug,
};

use log::debug;

use super::{FATENTRY_PER_SECTOR, FAT_CACHE_SIZE, fat32info::FAT32Info, fsinfo::FSInfo, FSI_LEADSIG, FSI_TRAILSIG, FSI_STRUCSIG, FSI_NOT_AVAILABLE};

/// 一个 FAT Sector Buffer 的状态
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FATSectorBufferState {
    Unassigned, // 未分配
    Assigned, // 已分配，未修改
    Dirty, // 已分配，已修改未写回
}

impl Default for FATSectorBufferState {
    fn default() -> Self { Self::Unassigned }
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
    pub fn sync(&mut self, block_device: Arc<dyn BlockDevice>, fatinfo: Arc<FAT32Info>) {
        if self.state == FATSectorBufferState::Dirty {
            for i in 0..fatinfo.fat_count {
                unsafe {
                    block_device.write_block(fatinfo.fat_start_sector + i * fatinfo.fat_sector_count + self.sector_no,
                        core::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * core::mem::size_of::<u32>()));
                }
            }
            self.state = FATSectorBufferState::Assigned;
        }
    }

    pub fn free(&mut self, block_device: Arc<dyn BlockDevice>, fatinfo: Arc<FAT32Info>) {
        self.sync(Arc::clone(&block_device), Arc::clone(&fatinfo));
        self.state = FATSectorBufferState::Unassigned;
    }

    pub fn init(&mut self, block_device: Arc<dyn BlockDevice>, fatinfo: Arc<FAT32Info>, sector_no: usize) {
        self.free(Arc::clone(&block_device), Arc::clone(&fatinfo));
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
            Some(self.data[offset] & 0x0FFFFFFF)
        }
    }

    pub fn write(&mut self, offset: usize, val: u32) -> Option<()> {
        if self.state == FATSectorBufferState::Unassigned || offset >= FATENTRY_PER_SECTOR {
            None
        } else {
            self.data[offset] = (self.data[offset] & 0xF0000000) | (val & 0x0FFFFFFF);
            self.state = FATSectorBufferState::Dirty;
            Some(())
        }
    }
}

struct FATBufferCache {
    data: Mutex<LinkedList<(usize, Arc<Mutex<FATSectorBuffer>>)>>,
    block_device: Arc<dyn BlockDevice>,
    info: Arc<FAT32Info>,
}

impl FATBufferCache {
    pub fn new(block_device: Arc<dyn BlockDevice>, info: Arc<FAT32Info>) -> Self {
        Self {
            data: Mutex::new(LinkedList::new()),
            block_device: Arc::clone(&block_device),
            info: Arc::clone(&info),
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
                if Arc::strong_count(&buffer_replaced) != 1 {
                    debug!("Fatal Error. FAT32 Run out of FAT Buffers!");
                }
                let mut buffer_replaced_locked = buffer_replaced.lock();
                buffer_replaced_locked.init(Arc::clone(&self.block_device), Arc::clone(&self.info), sector_no);
                data_locked.push_front((sector_no, Arc::clone(&buffer_replaced)));
            } else {
                let mut new_buffer = FATSectorBuffer::default();
                new_buffer.init(Arc::clone(&self.block_device), Arc::clone(&self.info), sector_no);
                data_locked.push_front((sector_no, Arc::new(Mutex::new(new_buffer))));
            }

        }
        Arc::clone(&data_locked.front().unwrap().1)
    }

    
    fn read_fat(&self, cluster_id: usize) -> Option<u32> {
        if cluster_id < 2 || cluster_id > self.info.tot_cluster_count + 1 {
            return None;
        }
        let sector_id = cluster_id / FATENTRY_PER_SECTOR;
        let offset = cluster_id / FATENTRY_PER_SECTOR;
        let fat_sector = self.lookup_buffer_cache(sector_id);
        let fat_sector_locked = fat_sector.lock();
        if fat_sector_locked.state == FATSectorBufferState::Unassigned {
            debug!("[FAT] Got an unassigned sector buffer! require = {}", sector_id);
        }
        if fat_sector_locked.sector_no != sector_id {
            debug!("[FAT] Sector buffer is wrong! require = {}, got = {}", sector_id, fat_sector_locked.sector_no);
        }
        fat_sector_locked.read(offset)
    }

    fn write_fat(&self, cluster_id: usize, val: u32) -> Option<()> {
        if cluster_id < 2 || cluster_id > self.info.tot_cluster_count + 1 {
            return None;
        }
        let sector_id = cluster_id / FATENTRY_PER_SECTOR;
        let offset = cluster_id / FATENTRY_PER_SECTOR;
        let fat_sector = self.lookup_buffer_cache(sector_id);
        let mut fat_sector_locked = fat_sector.lock();
        if fat_sector_locked.state == FATSectorBufferState::Unassigned {
            debug!("[FAT] Got an unassigned sector buffer! require = {}", sector_id);
        }
        if fat_sector_locked.sector_no != sector_id {
            debug!("[FAT] Sector buffer is wrong! require = {}, got = {}", sector_id, fat_sector_locked.sector_no);
        }
        fat_sector_locked.write(offset, val)
    }

    fn sync_buffer(&self, sector_id: usize) -> Option<()> {
        if sector_id > (self.info.tot_cluster_count + 1) / FATENTRY_PER_SECTOR {
            None
        } else {
            let fat_sector = self.lookup_buffer_cache(sector_id);
            let mut fat_sector_locked = fat_sector.lock();
            if fat_sector_locked.state == FATSectorBufferState::Unassigned {
                debug!("[FAT] Got an unassigned sector buffer! require = {}", sector_id);
            }
            if fat_sector_locked.sector_no != sector_id {
                debug!("[FAT] Sector buffer is wrong! require = {}, got = {}", sector_id, fat_sector_locked.sector_no);
            }
            fat_sector_locked.sync(Arc::clone(&self.block_device), Arc::clone(&self.info));
            Some(())
        }
    }
    
    fn sync_all_buffers(&self) {
        let data_locked = self.data.lock();
        for buffer in data_locked.iter() {
            buffer.1.lock().sync(Arc::clone(&self.block_device), Arc::clone(&self.info));
        }
    }
}

impl Drop for FATBufferCache {
    fn drop(&mut self) {
        self.sync_all_buffers();
    }
}

pub struct FATMeta {
    free_count: usize,
    nxt_free: usize,
}

pub struct FileAllocTable {
    pub block_device: Arc<dyn BlockDevice>,
    pub info: Arc<FAT32Info>,
    fatcache: FATBufferCache,
    fatmeta: Arc<Mutex<FATMeta>>,
}

impl FileAllocTable {
    pub fn new(block_device: Arc<dyn BlockDevice>, info: Arc<FAT32Info>) -> Self {
        let mut fsinfo_data: [u8; 512] = [0; 512];
        block_device.read_block(info.fsinfo_sector_id, &mut fsinfo_data);
        let fsinfo_raw = FSInfo::new(&fsinfo_data);
        if fsinfo_raw.FSI_LeadSig != FSI_LEADSIG
            || fsinfo_raw.FSI_TrailSig != FSI_TRAILSIG
            || fsinfo_raw.FSI_StrucSig != FSI_STRUCSIG {
            debug!("fsinfo magic number 有误！");
        }
        let free_count = fsinfo_raw.FSI_Free_Count as usize;
        let nxt_free = fsinfo_raw.FSI_Nxt_Free as usize;
        
        let mut ret = Self {
            block_device: Arc::clone(&block_device),
            info: Arc::clone(&info),
            fatcache: FATBufferCache::new(Arc::clone(&block_device), Arc::clone(&info)),
            fatmeta: Arc::new(Mutex::new(FATMeta{ free_count, nxt_free})),
        };
        ret.stat_free();
        ret
    }
    fn stat_free(&self) {
        let mut fatmeta_locked = self.fatmeta.lock();
        if fatmeta_locked.free_count == (FSI_NOT_AVAILABLE as usize) || fatmeta_locked.nxt_free == (FSI_NOT_AVAILABLE as usize) {
            fatmeta_locked.free_count = 0;
            fatmeta_locked.nxt_free = 0;
            for i in 0..self.info.tot_cluster_count {
                let cluster_id = i+2;
                let fatentry = self.read_fat(cluster_id).unwrap() & 0x0FFFFFFF;
                if fatentry == 0 {
                    fatmeta_locked.free_count += 1;
                } else {
                    fatmeta_locked.nxt_free = cluster_id;
                }
            }
        }
    }

    pub fn read_fat(&self, cluster_id: usize) -> Option<u32> {
        self.fatcache.read_fat(cluster_id)
    }

    pub fn write_fat(&self, cluster_id: usize, val: u32) -> Option<()> {
        self.fatcache.write_fat(cluster_id, val)
    }
    
    pub fn sync_fat(&self) {
        self.fatcache.sync_all_buffers();
    }

    fn alloc_cluster_inner(&self) -> Option<usize> {
        let mut fatmeta_locked = self.fatmeta.lock();
        if fatmeta_locked.nxt_free != self.info.tot_cluster_count + 1 {
            fatmeta_locked.nxt_free += 1;
            fatmeta_locked.free_count -= 1;
            Some(fatmeta_locked.nxt_free)
        } else {
            for i in 0..self.info.tot_cluster_count {
                let cluster_id = i+2;
                let fatentry = self.read_fat(cluster_id).unwrap() & 0x0FFFFFFF;
                if fatentry == 0 {
                    fatmeta_locked.free_count -= 1;
                    return Some(cluster_id);
                }
            }
            None
        }
    }

    pub fn alloc_cluster(&self, prev: Option<usize>) -> Option<usize> {
        if let Some(ret) = self.alloc_cluster_inner() {
            if let Some(pre) = prev {
                if self.read_fat(pre).unwrap() < 0x0FFFFFF8 {
                    debug!("尝试在非 FAT 链末尾写数据");
                }
                self.write_fat(pre, ret as u32);
            }
            self.write_fat(ret, 0x0FFFFFFF);
            Some(ret)
        } else {
            None
        }
    }

    pub fn free_cluster(&self, cluster_id: usize, prev: Option<usize>) -> Option<()> {
        if let Some(pre) = prev {
            if self.read_fat(pre).unwrap() as usize != cluster_id {
                debug!("给定的前驱位置不对");
                return None;
            }
            self.write_fat(pre, 0x0FFFFFFF);
        }
        self.write_fat(cluster_id, 0);
        let mut fatmeta_locked = self.fatmeta.lock();
        fatmeta_locked.free_count += 1;
        Some(())
    }

    
}
