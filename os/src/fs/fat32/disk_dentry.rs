use super::util::*;

#[allow(non_snake_case)]
#[derive(Copy, Clone, Default)]
// Dentry and part of long dentry
pub struct DiskDirEntry {
    DIR_Name: [u8; 11],
    DIR_Attr: u8,
    DIR_NTRes: u8,
    DIR_CrtTimeTenth: u8,
    DIR_CrtTime: u16,
    DIR_CrtDate: u16,
    DIR_LstAccDate: u16,
    DIR_FstClusHI: u16,
    DIR_WrtTime: u16,
    DIR_WrtDate: u16,
    DIR_FstClusLO: u16,
    DIR_FileSize: u32,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Default)]
pub struct DiskLongDirEntry {
    LDIR_Ord: u8,
    LDIR_Name1: [u16; 5],
    LDIR_Attr: u8,
    LDIR_Type: u8,
    LDIR_Chksum: u8,
    LDIR_Name2: [u16; 6],
    LDIR_FstClusLO: u16,
    LDIR_Name3: [u16; 2],
}


impl DiskDirEntry {
    /// Initialize a BPB
    pub fn load(&mut self, src: &[u8; 32]) {
        let mut offset: usize = 0;
        macro_rules ! load {
            ($v: expr) => {
                load_fn(&mut $v, src, &mut offset);
            }
        }
        load!(self.DIR_Name);
        load!(self.DIR_Attr);
        load!(self.DIR_NTRes);
        load!(self.DIR_CrtTimeTenth);
        load!(self.DIR_CrtTime);
        load!(self.DIR_CrtDate);
        load!(self.DIR_LstAccDate);
        load!(self.DIR_FstClusHI);
        load!(self.DIR_WrtTime);
        load!(self.DIR_WrtDate);
        load!(self.DIR_FstClusLO);
        load!(self.DIR_FileSize);
    }
    
    pub fn store(&mut self, dest: &mut[u8; 32]) {
        let mut offset: usize = 0;
        macro_rules ! store {
            ($v: expr) => {
                store_fn(&$v, dest, &mut offset);
            }
        }
        store!(self.DIR_Name);
        store!(self.DIR_Attr);
        store!(self.DIR_NTRes);
        store!(self.DIR_CrtTimeTenth);
        store!(self.DIR_CrtTime);
        store!(self.DIR_CrtDate);
        store!(self.DIR_LstAccDate);
        store!(self.DIR_FstClusHI);
        store!(self.DIR_WrtTime);
        store!(self.DIR_WrtDate);
        store!(self.DIR_FstClusLO);
        store!(self.DIR_FileSize);
    }
}

impl DiskLongDirEntry {
    /// Initialize a BPB
    pub fn load(&mut self, src: &[u8; 32]) {
        let mut offset: usize = 0;
        macro_rules ! load {
            ($v: expr) => {
                load_fn(&mut $v, src, &mut offset);
            }
        }
        load!(self.LDIR_Ord);
        load!(self.LDIR_Name1);
        load!(self.LDIR_Attr);
        load!(self.LDIR_Type);
        load!(self.LDIR_Chksum);
        load!(self.LDIR_Name2);
        load!(self.LDIR_FstClusLO);
        load!(self.LDIR_Name3);
    }
    
    pub fn store(&mut self, dest: &mut[u8; 32]) {
        let mut offset: usize = 0;
        macro_rules ! store {
            ($v: expr) => {
                store_fn(&$v, dest, &mut offset);
            }
        }
        store!(self.LDIR_Ord);
        store!(self.LDIR_Name1);
        store!(self.LDIR_Attr);
        store!(self.LDIR_Type);
        store!(self.LDIR_Chksum);
        store!(self.LDIR_Name2);
        store!(self.LDIR_FstClusLO);
        store!(self.LDIR_Name3);
    }
}

