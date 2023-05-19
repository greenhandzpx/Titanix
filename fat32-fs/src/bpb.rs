use core::default;

use super::util::*;


#[derive(Copy, Clone, Default)]
// Boot Sector with BPB, load from Sector 0
pub struct BPB {

    // and BS
    pub BS_jmpBoot: [u8; 3],             // ignore.
    pub BS_OEMName: [u8; 8],             // usually "MSWIN4.1"
    
    // essential BPB
    pub BPB_BytesPerSector: u16,         // must 512, ..., 4096. usually 512
    pub BPB_SectorPerCluster: u8,        // must 1,2,4,...,128.
    pub BPB_ReservedSectorCount: u16,    // usually 32 for FAT32
    pub BPB_NumFATs: u8,                 // usually 2 for FAT32
    pub BPB_RootEntryCount: u16,         // must 0 for FAT32
    pub BPB_TotSector16: u16,            // must 0 for FAT32
    pub BPB_Media: u8,                   // usually 0xF8, ignore.
    pub BPB_FATsize16: u16,              // must 0 for FAT32
    pub BPB_SectorPerTrack: u16,         // use for int 0x13, ignore.
    pub BPB_NumHeads: u16,               // use for int 0x13, ignore.
    pub BPB_HiddSec: u32,                // Hideden Sector Count, ignore.
    pub BPB_TotSector32: u32,            // Total Sector count.

    // BPB For FAT32
    pub BPB_FATsize32: u32,              // FAT Sector count.
    pub BPB_ExtFlags: u16,               // usually 0
    pub BPB_FSVer: u16,                  // must 0 in current version
    pub BPB_RootCluster: u32,            // Root Cluster id, usally 2
    pub BPB_FSInfo: u16,                 // FSINFO Sector id, usually 1
    pub BPB_BkBootSec: u16,              // Backup Sector id, usually 6
    pub BPB_Reserved: [u8; 12],          // 0
    pub BPB_DrvNum: u8,                  // use for 0x13, 0x80
    pub BPB_Reserved1: u8,               // 0
    pub BPB_BootSig: u8,                 // 0x29
    pub BPB_VolID: u32,                  // xjb Generator

    // BS(end)
    pub BS_VolLabel: [u8; 11],             // 11*0x30
    pub BS_FileSysType: [u8; 8],         // FAT32 0x30*3
}

impl BPB {
    /// Initialize a BPB
    pub fn load(&mut self, src: &[u8; 512]) {
        let mut offset: usize = 0;
        macro_rules ! load {
            ($v: expr) => {
                load_fn(&mut $v, src, &mut offset);
            }
        };
        load!(self.BS_jmpBoot);
        load!(self.BS_OEMName);

        load!(self.BPB_BytesPerSector);
        load!(self.BPB_SectorPerCluster);
        load!(self.BPB_ReservedSectorCount);
        load!(self.BPB_NumFATs);
        load!(self.BPB_RootEntryCount);
        load!(self.BPB_TotSector16);
        load!(self.BPB_Media);
        load!(self.BPB_FATsize16);
        load!(self.BPB_SectorPerTrack);
        load!(self.BPB_NumHeads);
        load!(self.BPB_HiddSec);
        load!(self.BPB_TotSector32);
        
        load!(self.BPB_FATsize32);
        load!(self.BPB_ExtFlags);
        load!(self.BPB_FSVer);
        load!(self.BPB_RootCluster);
        load!(self.BPB_FSInfo);
        load!(self.BPB_BkBootSec);
        load!(self.BPB_Reserved);
        load!(self.BPB_DrvNum);
        load!(self.BPB_Reserved1);
        load!(self.BPB_BootSig);
        load!(self.BPB_VolID);

        load!(self.BS_VolLabel);
        load!(self.BS_FileSysType);
    }
    
    pub fn store(&mut self, dest: &mut[u8; 512]) {
        let mut offset: usize = 0;
        macro_rules ! store {
            ($v: expr) => {
                store_fn(&$v, dest, &mut offset);
            }
        };
        store!(self.BS_jmpBoot);
        store!(self.BS_OEMName);

        store!(self.BPB_BytesPerSector);
        store!(self.BPB_SectorPerCluster);
        store!(self.BPB_ReservedSectorCount);
        store!(self.BPB_NumFATs);
        store!(self.BPB_RootEntryCount);
        store!(self.BPB_TotSector16);
        store!(self.BPB_Media);
        store!(self.BPB_FATsize16);
        store!(self.BPB_SectorPerTrack);
        store!(self.BPB_NumHeads);
        store!(self.BPB_HiddSec);
        store!(self.BPB_TotSector32);
        
        store!(self.BPB_FATsize32);
        store!(self.BPB_ExtFlags);
        store!(self.BPB_FSVer);
        store!(self.BPB_RootCluster);
        store!(self.BPB_FSInfo);
        store!(self.BPB_BkBootSec);
        store!(self.BPB_Reserved);
        store!(self.BPB_DrvNum);
        store!(self.BPB_Reserved1);
        store!(self.BPB_BootSig);
        store!(self.BPB_VolID);

        store!(self.BS_VolLabel);
        store!(self.BS_FileSysType);
    }
}
