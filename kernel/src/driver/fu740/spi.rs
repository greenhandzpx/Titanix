#![allow(unused)]
pub struct SPI {
    base_addr: usize,
}

impl SPI {
    pub fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }
    fn sckdiv_ptr(&self) -> *mut u32 {
        self.base_addr as *mut u32
    }
    fn sckmode_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x4) as *mut u32
    }
    fn csid_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x10) as *mut u32
    }
    fn csdef_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x14) as *mut u32
    }
    fn csmode_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x18) as *mut u32
    }
    fn delay0_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x28) as *mut u32
    }
    fn delay1_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x2C) as *mut u32
    }
    fn fmt_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x40) as *mut u32
    }
    fn txdata_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x48) as *mut u32
    }
    fn rxdata_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x4C) as *mut u32
    }
    fn txmark_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x50) as *mut u32
    }
    fn rxmark_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x54) as *mut u32
    }
    fn fctrl_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x60) as *mut u32
    }
    fn ffmt_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x64) as *mut u32
    }
    fn ie_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x70) as *mut u32
    }
    fn ip_ptr(&self) -> *mut u32 {
        (self.base_addr + 0x74) as *mut u32
    }
    pub fn init(&self) {
        unsafe {
            // set ie txwm | rxwm
            self.ie_ptr().write_volatile(2);
            // set delay0 cssck = 1, sckcs = 1
            self.delay0_ptr().write_volatile((1 << 16) | 1);
            // disable directly memory-mapped mode
            self.fctrl_ptr().write_volatile(0);
        }
    }
    pub fn configure(&self, protocol: u32, endian: u32, csdef: u32, csid: u32) {
        unsafe {
            let fmt = (protocol & 3) | ((endian & 1) << 2) | (0 << 3) | (8 << 16);
            self.fmt_ptr().write_volatile(fmt);
            self.csdef_ptr().write_volatile(csdef);
            self.csid_ptr().write_volatile(csid);
        }
    }
    pub fn set_clk_rate(&self, div: u32) {
        unsafe {
            self.sckdiv_ptr().write_volatile(div);
        }
    }
    /// true for Tx, false for Rx
    fn set_direction(&self, direction: bool) {
        unsafe {
            let fmt = self.fmt_ptr().read_volatile();
            if direction {
                self.fmt_ptr().write_volatile(fmt | (1 << 3)); // set dir to 1 for Tx
            } else {
                self.fmt_ptr().write_volatile(fmt & !(1 << 3));
            }
        }
    }
    pub fn switch_cs(&self, csmode_hold: bool, csid: u32) {
        unsafe {
            self.csmode_ptr()
                .write_volatile(if csmode_hold { 2 } else { 0 });
            self.csid_ptr().write_volatile(csid);
        }
    }
    pub fn send_data(&self, csid: u32, tx: &[u8]) {
        self.set_direction(true);
        self.switch_cs(true, csid);
        const CHUNK_LEN: usize = 8;
        for s in tx.chunks(CHUNK_LEN) {
            let n = s.len();
            unsafe {
                self.txmark_ptr().write_volatile(1);
                while (self.ip_ptr().read_volatile() & 1) == 0 {
                    // wait
                }
                for i in 0..n {
                    self.txdata_ptr().write_volatile(s[i].into());
                }
            }
        }
        self.switch_cs(false, csid);
    }
    pub fn recv_data(&self, csid: u32, rx: &mut [u8]) {
        self.set_direction(false);
        self.switch_cs(true, csid);
        const CHUNK_LEN: usize = 8;
        for s in rx.chunks_mut(CHUNK_LEN) {
            let n = s.len();
            unsafe {
                self.rxmark_ptr().write_volatile((n - 1) as u32);
                for _ in 0..n {
                    self.txdata_ptr().write_volatile(0xff as u32);
                }
                while (self.ip_ptr().read_volatile() & 2) == 0 {
                    // wait
                }
                for i in 0..n {
                    s[i] = (self.rxdata_ptr().read_volatile() & 0xff) as u8;
                }
            }
        }
        self.switch_cs(false, csid);
    }
}
