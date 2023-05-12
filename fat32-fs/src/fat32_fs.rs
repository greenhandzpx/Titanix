use crate::bpb::BPB;

use super::block_cache::*;
use super::block_dev::*;
use super::BLOCK_SZ;
use alloc::sync::{Arc, Weak};
use spin::Mutex;

pub struct FAT32 {
    block_device: Arc<dyn BlockDevice>,
    bpb: BPB,

    sector_count: usize,
    sector_per_cluster: usize,

    FAT_size: usize,
    FAT_count: usize,
    FAT_start_sector: usize,
    data_start_sector: usize,

    root_cluster: usize,
}

impl FAT32 {

    pub fn get_start_sector(&self, cluster_id: usize) -> Option<usize> {
        if cluster_id < 2 {
            None
        } else {
            let ret = (cluster_id-2) * self.sector_per_cluster + self.data_start_sector;
            if (ret < self.sector_count) {
                Some(ret)
            } else {
                None
            }
        }
    }

    pub fn get_belong_cluster(&self, sector_id: usize) -> Option<usize> {
        if sector_id < self.data_start_sector || sector_id >= self.sector_count {
            None
        } else {
            Some((sector_id-self.data_start_sector)/self.sector_per_cluster+2)
        }
    }

    pub fn open(block_device: Arc<dyn BlockDevice>) -> Arc<Mutex<Self>> {
        get_block_cache(0, Arc::clone(&block_device))
            .lock()
            .read(0, |data: &[u8; 512]| {
                
                let mut bpb : BPB = BPB { ..Default::default() };
                bpb.load(data);
                let fs = Self {
                    block_device: Arc::clone(&block_device),
                    bpb: bpb,
                    sector_count: bpb.BPB_TotSector32 as usize,
                    sector_per_cluster: bpb.BPB_SectorPerCluster as usize,
                    FAT_size: bpb.BPB_FATsize32 as usize,
                    FAT_count: bpb.BPB_NumFATs as usize,
                    FAT_start_sector: bpb.BPB_ReservedSectorCount as usize,
                    data_start_sector: (bpb.BPB_ReservedSectorCount as usize) + (bpb.BPB_FATsize32 as usize) * (bpb.BPB_NumFATs as usize),
                    root_cluster: bpb.BPB_RootCluster as usize,
                };

                Arc::new(Mutex::new(fs))
            })
    }

}

