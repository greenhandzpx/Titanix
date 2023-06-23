use super::util::*;

#[allow(non_snake_case)]
#[derive(Copy, Clone, Default)]
// Dentry and part of long dentry
pub struct DiskDirEntry {
    pub DIR_Name: [u8; 11],
    pub DIR_Attr: u8,
    pub DIR_NTRes: u8,
    pub DIR_CrtTimeTenth: u8,
    pub DIR_CrtTime: u16,
    pub DIR_CrtDate: u16,
    pub DIR_LstAccDate: u16,
    pub DIR_FstClusHI: u16,
    pub DIR_WrtTime: u16,
    pub DIR_WrtDate: u16,
    pub DIR_FstClusLO: u16,
    pub DIR_FileSize: u32,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Default)]
pub struct DiskLongDirEntry {
    pub LDIR_Ord: u8,
    pub LDIR_Name1: [u16; 5],
    pub LDIR_Attr: u8,
    pub LDIR_Type: u8,
    pub LDIR_Chksum: u8,
    pub LDIR_Name2: [u16; 6],
    pub LDIR_FstClusLO: u16,
    pub LDIR_Name3: [u16; 2],
}

impl DiskDirEntry {
    /// Initialize a BPB
    pub fn load(&mut self, src: &[u8]) {
        let mut offset: usize = 0;
        macro_rules! load {
            ($v: expr) => {
                load_fn(&mut $v, src, &mut offset);
            };
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

    pub fn store(&mut self, dest: &mut [u8]) {
        let mut offset: usize = 0;
        macro_rules! store {
            ($v: expr) => {
                store_fn(&$v, dest, &mut offset);
            };
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

    pub fn new(src: &[u8]) -> Self {
        let mut ret = Self::default();
        ret.load(src);
        ret
    }
}

impl DiskLongDirEntry {
    /// Initialize a BPB
    pub fn load(&mut self, src: &[u8]) {
        let mut offset: usize = 0;
        macro_rules! load {
            ($v: expr) => {
                load_fn(&mut $v, src, &mut offset);
            };
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

    pub fn store(&mut self, dest: &mut [u8]) {
        let mut offset: usize = 0;
        macro_rules! store {
            ($v: expr) => {
                store_fn(&$v, dest, &mut offset);
            };
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
    pub fn new(src: &[u8]) -> Self {
        let mut ret = Self::default();
        ret.load(src);
        ret
    }
}
