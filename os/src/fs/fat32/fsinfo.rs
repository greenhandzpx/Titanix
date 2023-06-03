use crate::fs::fat32::{FSI_LEADSIG, FSI_STRUCSIG, FSI_TRAILSIG, FSI_RESERVED1_SIZE, FSI_RESERVED2_SIZE};

use super::util::*;
#[allow(non_snake_case)]
#[derive(Clone, Copy, Default)]
pub struct FSInfo {
    pub FSI_LeadSig: u32, // must 0x41615252
    // FSI_Reserved1: [u8; 480], // must 0
    pub FSI_StrucSig: u32, // must 0x61417272
    pub FSI_Free_Count: u32,
    pub FSI_Nxt_Free: u32,
    // FSI_Reserved2: [u8; 12], // must 0
    pub FSI_TrailSig: u32, // must 0xAA550000
}

impl FSInfo {
    pub fn new(src: &[u8; 512]) -> Self {
        let mut ret: Self = Self::default();
        ret.load(src);
        ret
    }

    pub fn load(&mut self, src: &[u8; 512]) {
        let mut offset: usize = 0;
        macro_rules ! load {
            ($v: expr) => {
                load_fn(&mut $v, src, &mut offset);
            }
        }
        load!(self.FSI_LeadSig);
        offset += FSI_RESERVED1_SIZE;
        load!(self.FSI_StrucSig);
        load!(self.FSI_Free_Count);
        load!(self.FSI_Nxt_Free);
        offset += FSI_RESERVED2_SIZE;
        load!(self.FSI_TrailSig);
    }
    
    pub fn store(&mut self, dest: &mut[u8; 512]) {
        let mut offset: usize = 0;
        macro_rules ! store {
            ($v: expr) => {
                store_fn(&$v, dest, &mut offset);
            }
        }
        store!(FSI_LEADSIG);
        offset += FSI_RESERVED1_SIZE;
        store!(FSI_STRUCSIG);
        store!(self.FSI_Free_Count);
        store!(self.FSI_Nxt_Free);
        offset += FSI_RESERVED2_SIZE;
        store!(FSI_TRAILSIG);
    }
}
