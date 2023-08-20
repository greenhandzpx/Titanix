use alloc::{sync::Arc, vec::Vec};

use crate::{driver::BlockDevice, fs::Mutex, stack_trace};

use log::info;

use super::{
    fat32info::FAT32Info, fsinfo::FSInfo, FATENTRY_PER_SECTOR, FSI_LEADSIG, FSI_NOT_AVAILABLE,
    FSI_STRUCSIG, FSI_TRAILSIG,
};

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

#[derive(Clone, Copy)]
struct FATSectorBuffer {
    sector_no: usize,
    data: [u32; FATENTRY_PER_SECTOR],
    state: FATSectorBufferState,
}

impl Default for FATSectorBuffer {
    fn default() -> Self {
        stack_trace!();
        Self {
            sector_no: usize::default(),
            data: [u32::default(); FATENTRY_PER_SECTOR],
            state: FATSectorBufferState::default(),
        }
    }
}

impl FATSectorBuffer {
    pub fn sync(&mut self, block_device: Arc<dyn BlockDevice>, fatinfo: Arc<FAT32Info>) {
        stack_trace!();
        if self.state == FATSectorBufferState::Dirty {
            for i in 0..fatinfo.fat_count {
                unsafe {
                    block_device.write_block(
                        fatinfo.fat_start_sector + i * fatinfo.fat_sector_count + self.sector_no,
                        core::slice::from_raw_parts(
                            self.data.as_ptr() as *const u8,
                            self.data.len() * core::mem::size_of::<u32>(),
                        ),
                    );
                }
            }
            self.state = FATSectorBufferState::Assigned;
        }
    }

    pub fn free(&mut self, block_device: Arc<dyn BlockDevice>, fatinfo: Arc<FAT32Info>) {
        stack_trace!();
        self.sync(Arc::clone(&block_device), Arc::clone(&fatinfo));
        self.state = FATSectorBufferState::Unassigned;
    }

    pub fn init(
        &mut self,
        block_device: Arc<dyn BlockDevice>,
        fatinfo: Arc<FAT32Info>,
        sector_no: usize,
    ) {
        stack_trace!();
        self.free(Arc::clone(&block_device), Arc::clone(&fatinfo));
        self.state = FATSectorBufferState::Assigned;
        self.sector_no = sector_no;
        unsafe {
            block_device.read_block(
                fatinfo.fat_start_sector + self.sector_no,
                core::slice::from_raw_parts_mut(
                    self.data.as_mut_ptr() as *mut u8,
                    self.data.len() * core::mem::size_of::<u32>(),
                ),
            );
        }
    }

    pub fn read(&self, offset: usize) -> Option<u32> {
        stack_trace!();
        if self.state == FATSectorBufferState::Unassigned || offset >= FATENTRY_PER_SECTOR {
            None
        } else {
            Some(self.data[offset] & 0x0FFFFFFF)
        }
    }

    pub fn write(&mut self, offset: usize, val: u32) -> Option<()> {
        stack_trace!();
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
    data: Vec<FATSectorBuffer>,
    block_device: Arc<dyn BlockDevice>,
    info: Arc<FAT32Info>,
}

impl FATBufferCache {
    pub fn new(block_device: Arc<dyn BlockDevice>, info: Arc<FAT32Info>) -> Self {
        stack_trace!();
        let mut data = Vec::new();
        for sector_no in 0..info.fat_sector_count {
            let mut s = FATSectorBuffer::default();
            s.init(block_device.clone(), info.clone(), sector_no);
            data.push(s);
        }
        Self {
            data,
            block_device: Arc::clone(&block_device),
            info: Arc::clone(&info),
        }
    }

    fn read_fat(&mut self, cluster_id: usize) -> Option<u32> {
        stack_trace!();
        if cluster_id < 2 || cluster_id > self.info.tot_cluster_count + 1 {
            return None;
        }
        let sector_id = cluster_id / FATENTRY_PER_SECTOR;
        let offset = cluster_id % FATENTRY_PER_SECTOR;
        let fat_sector = self.data.get_mut(sector_id).unwrap();
        if fat_sector.state == FATSectorBufferState::Unassigned {
            info!(
                "[FAT] Got an unassigned sector buffer! require = {}",
                sector_id
            );
        }
        if fat_sector.sector_no != sector_id {
            info!(
                "[FAT] Sector buffer is wrong! require = {}, got = {}",
                sector_id, fat_sector.sector_no
            );
        }
        fat_sector.read(offset)
    }

    fn write_fat(&mut self, cluster_id: usize, val: u32) -> Option<()> {
        stack_trace!();
        if cluster_id < 2 || cluster_id > self.info.tot_cluster_count + 1 {
            return None;
        }
        let sector_id = cluster_id / FATENTRY_PER_SECTOR;
        let offset = cluster_id % FATENTRY_PER_SECTOR;
        let fat_sector = self.data.get_mut(sector_id).unwrap();
        if fat_sector.state == FATSectorBufferState::Unassigned {
            info!(
                "[FAT] Got an unassigned sector buffer! require = {}",
                sector_id
            );
        }
        if fat_sector.sector_no != sector_id {
            info!(
                "[FAT] Sector buffer is wrong! require = {}, got = {}",
                sector_id, fat_sector.sector_no
            );
        }
        fat_sector.write(offset, val)
    }

    #[allow(unused)]
    fn sync_buffer(&mut self, sector_id: usize) -> Option<()> {
        stack_trace!();
        if sector_id > (self.info.tot_cluster_count + 1) / FATENTRY_PER_SECTOR {
            None
        } else {
            let fat_sector = self.data.get_mut(sector_id).unwrap();
            if fat_sector.state == FATSectorBufferState::Unassigned {
                info!(
                    "[FAT] Got an unassigned sector buffer! require = {}",
                    sector_id
                );
            }
            if fat_sector.sector_no != sector_id {
                info!(
                    "[FAT] Sector buffer is wrong! require = {}, got = {}",
                    sector_id, fat_sector.sector_no
                );
            }
            fat_sector.sync(Arc::clone(&self.block_device), Arc::clone(&self.info));
            Some(())
        }
    }

    fn sync_all_buffers(&mut self) {
        stack_trace!();
        for buffer in self.data.iter_mut() {
            buffer.sync(Arc::clone(&self.block_device), Arc::clone(&self.info));
        }
    }
}

impl Drop for FATBufferCache {
    fn drop(&mut self) {
        stack_trace!();
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
    fatcache: Mutex<FATBufferCache>,
    fatmeta: Arc<Mutex<FATMeta>>,
}

impl FileAllocTable {
    pub fn new(block_device: Arc<dyn BlockDevice>, info: Arc<FAT32Info>) -> Self {
        stack_trace!();
        let mut fsinfo_data: [u8; 512] = [0; 512];
        block_device.read_block(info.fsinfo_sector_id, &mut fsinfo_data);
        let fsinfo_raw = FSInfo::new(&fsinfo_data);
        if fsinfo_raw.FSI_LeadSig != FSI_LEADSIG
            || fsinfo_raw.FSI_TrailSig != FSI_TRAILSIG
            || fsinfo_raw.FSI_StrucSig != FSI_STRUCSIG
        {
            info!("fsinfo magic number wrong!");
        }
        let free_count = fsinfo_raw.FSI_Free_Count as usize;
        let nxt_free = fsinfo_raw.FSI_Nxt_Free as usize;
        let ret = Self {
            block_device: Arc::clone(&block_device),
            info: Arc::clone(&info),
            fatcache: Mutex::new(FATBufferCache::new(
                Arc::clone(&block_device),
                Arc::clone(&info),
            )),
            fatmeta: Arc::new(Mutex::new(FATMeta {
                free_count,
                nxt_free,
            })),
        };
        ret.stat_free();
        ret
    }
    fn stat_free(&self) {
        stack_trace!();
        let mut fatmeta_locked = self.fatmeta.lock();
        if fatmeta_locked.free_count == (FSI_NOT_AVAILABLE as usize)
            || fatmeta_locked.nxt_free == (FSI_NOT_AVAILABLE as usize)
        {
            fatmeta_locked.free_count = 0;
            fatmeta_locked.nxt_free = 0;
            for i in 0..self.info.tot_cluster_count {
                let cluster_id = i + 2;
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
        stack_trace!();
        self.fatcache.lock().read_fat(cluster_id)
    }

    pub fn write_fat(&self, cluster_id: usize, val: u32) -> Option<()> {
        stack_trace!();
        self.fatcache.lock().write_fat(cluster_id, val)
    }

    #[allow(unused)]
    pub fn sync_fat(&self) {
        stack_trace!();
        self.fatcache.lock().sync_all_buffers();
    }

    fn alloc_cluster_inner(&self) -> Option<usize> {
        stack_trace!();
        let mut fatmeta_locked = self.fatmeta.lock();
        // println!(
        //     "[FileAllocTable::alloc_cluster_inner] tot_cluster_count: {}, nxt_free: {}",
        //     self.info.tot_cluster_count, fatmeta_locked.nxt_free
        // );
        if fatmeta_locked.nxt_free != self.info.tot_cluster_count + 1 {
            fatmeta_locked.nxt_free += 1;
            fatmeta_locked.free_count -= 1;
            Some(fatmeta_locked.nxt_free)
        } else {
            for i in 0..self.info.tot_cluster_count {
                let cluster_id = i + 2;
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
        stack_trace!();
        if let Some(ret) = self.alloc_cluster_inner() {
            if let Some(pre) = prev {
                if self.read_fat(pre).unwrap() < 0x0FFFFFF8 {
                    info!("write data at non fat link tail!");
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
        stack_trace!();
        if let Some(pre) = prev {
            if self.read_fat(pre).unwrap() as usize != cluster_id {
                info!("not a right pre!");
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
