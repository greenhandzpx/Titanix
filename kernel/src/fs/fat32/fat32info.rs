use super::bpb::BootSector;
use crate::stack_trace;

#[derive(Copy, Clone, Default)]
pub struct FAT32Info {
    pub bk_bootsector_id: usize,
    pub fsinfo_sector_id: usize,
    pub fat_start_sector: usize,
    pub fat_sector_count: usize,
    pub fat_count: usize,
    pub data_start_sector: usize,
    pub sector_per_cluster: usize,
    pub tot_sector_count: usize,
    pub tot_cluster_count: usize,
    pub root_cluster_id: usize,
}

impl FAT32Info {
    pub fn new(bs: BootSector) -> Self {
        stack_trace!();
        let start_sector = (bs.BPB_ReservedSectorCount as usize)
            + (bs.BPB_NumFATs as usize) * (bs.BPB_FATsize32 as usize);
        let cluster_count =
            (bs.BPB_TotSector32 as usize - start_sector) / (bs.BPB_SectorPerCluster as usize);
        Self {
            bk_bootsector_id: bs.BPB_BkBootSec as usize,
            fsinfo_sector_id: bs.BPB_FSInfo as usize,
            fat_start_sector: bs.BPB_ReservedSectorCount as usize,
            fat_sector_count: bs.BPB_FATsize32 as usize,
            fat_count: bs.BPB_NumFATs as usize,
            data_start_sector: start_sector,
            sector_per_cluster: bs.BPB_SectorPerCluster as usize,
            tot_sector_count: bs.BPB_TotSector32 as usize,
            tot_cluster_count: cluster_count as usize,
            root_cluster_id: bs.BPB_RootCluster as usize,
        }
    }

    pub fn cid_to_sid(&self, cluster_id: usize) -> Option<usize> {
        stack_trace!();
        if cluster_id < 2 {
            return None;
        }
        let ret = (cluster_id - 2) * self.sector_per_cluster + self.data_start_sector;
        if ret >= self.tot_sector_count {
            return None;
        }
        Some(ret)
    }
}
