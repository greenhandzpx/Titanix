use super::util::*;

pub struct FSInfo {
    FSI_LeadSig: u32, // must 0x41615252
    FSI_Reserved1: [u8; 480], // must 0
    FSI_StrucSig: u32, // must 0x61417272
    FSI_Free_Count: u32,
    FSI_Nxt_Free: u32,
    FSI_Reserved2: [u8; 12], // must 0
    FSI_TrailSig: u32, // must 0xAA550000
}

impl FSInfo {
    /// Initialize a BPB
    pub fn load(&mut self, src: &[u8; 512]) {
        let mut offset: usize = 0;
        macro_rules ! load {
            ($v: expr) => {
                load_fn(&mut $v, src, &mut offset);
            }
        };
        load!(self.FSI_LeadSig);
        load!(self.FSI_Reserved1);
        load!(self.FSI_StrucSig);
        load!(self.FSI_Free_Count);
        load!(self.FSI_Nxt_Free);
        load!(self.FSI_Reserved2);
        load!(self.FSI_TrailSig);
    }
    
    pub fn store(&mut self, dest: &mut[u8; 512]) {
        let mut offset: usize = 0;
        macro_rules ! store {
            ($v: expr) => {
                store_fn(&$v, dest, &mut offset);
            }
        };
        store!(self.FSI_LeadSig);
        store!(self.FSI_Reserved1);
        store!(self.FSI_StrucSig);
        store!(self.FSI_Free_Count);
        store!(self.FSI_Nxt_Free);
        store!(self.FSI_Reserved2);
        store!(self.FSI_TrailSig);
    }
}
