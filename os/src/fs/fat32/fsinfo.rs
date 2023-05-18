use super::util::*;

const SIZE_RESERVED1: usize = 480;
const SIZE_RESERVED2: usize = 12;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Default)]
pub struct FSInfo {
    FSI_LeadSig: u32, // must 0x41615252
    // FSI_Reserved1: [u8; 480], // must 0
    FSI_StrucSig: u32, // must 0x61417272
    pub FSI_Free_Count: u32,
    pub FSI_Nxt_Free: u32,
    // FSI_Reserved2: [u8; 12], // must 0
    FSI_TrailSig: u32, // must 0xAA550000
}

const FSI_LEADSIG: u32 = 0x41615252;
const FSI_STRUCSIG: u32 = 0x61417272;
const FSI_TRAILSIG: u32 = 0xAA550000;

impl FSInfo {
    pub fn new(src: &[u8; 512]) -> Option<Self> {
        let mut ret: Self = Self::default();
        ret.load(src);
        if ret.FSI_LeadSig != FSI_LEADSIG
            || ret.FSI_TrailSig != FSI_TRAILSIG
            || ret.FSI_StrucSig != FSI_STRUCSIG {
                None
            }
            else {
                Some(ret)
            }
    }

    pub fn load(&mut self, src: &[u8; 512]) {
        let mut offset: usize = 0;
        macro_rules ! load {
            ($v: expr) => {
                load_fn(&mut $v, src, &mut offset);
            }
        };
        load!(self.FSI_LeadSig);
        offset += SIZE_RESERVED1;
        load!(self.FSI_StrucSig);
        load!(self.FSI_Free_Count);
        load!(self.FSI_Nxt_Free);
        offset += SIZE_RESERVED2;
        load!(self.FSI_TrailSig);
    }
    
    pub fn store(&mut self, dest: &mut[u8; 512]) {
        let mut offset: usize = 0;
        macro_rules ! store {
            ($v: expr) => {
                store_fn(&$v, dest, &mut offset);
            }
        };
        store!(FSI_LEADSIG);
        offset += SIZE_RESERVED1;
        store!(FSI_STRUCSIG);
        store!(self.FSI_Free_Count);
        store!(self.FSI_Nxt_Free);
        offset += SIZE_RESERVED2;
        store!(FSI_TRAILSIG);
    }
}
