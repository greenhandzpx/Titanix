use fu740_pac as pac;
use log::info;
use pac::SPI2;

pub struct SPIImpl {
    pub spi: SPI2,
}

pub trait SPI {
    fn init(&self);
    fn configure(&self, protocal: u8, endianness: bool, cs_active_high: u32, csid: u32);
    fn set_clk_rate(&self, div: u32) -> u32;
    fn recv_data(&self, chip_select: u32, rx: &mut [u8]);
    fn send_data(&self, chip_select: u32, tx: &[u8]);
    fn fill_data(&self, chip_select: u32, value: u32, tx_len: usize);
    fn switch_cs(&self, enable: bool, csid: u32);
}

impl SPIImpl {
    pub fn new(spi: SPI2) -> Self {
        Self { spi }
    }
}

impl SPI for SPIImpl {
    fn init(&self) {
        info!("[SPI] SPIImpl init start");
        unsafe {
            self.spi.ie.modify(|_, w| w.bits(0));
            self.spi.ie.modify(|_, w| w.txwm().bit(true));
            self.spi.ie.modify(|_, w| w.rxwm().bit(false));
            self.spi.delay0.modify(|_, w| w.cssck().bits(1));
            self.spi.delay0.modify(|_, w| w.sckcs().bits(1));
            self.spi.fctrl.modify(|_, w| w.en().clear_bit());
        }
        info!("[SPI] SPIImpl init finished");
    }
    fn configure(&self, protocal: u8, endianness: bool, cs_active_high: u32, csid: u32) {
        unsafe {
            self.spi.fmt.modify(|_, w| w.proto().bits(protocal));
            self.spi.fmt.modify(|_, w| w.endian().bit(!endianness));
            self.spi.fmt.modify(|_, w| w.dir().clear_bit());
            self.spi.csdef.modify(|_, w| w.bits(cs_active_high));
            self.spi.csid.modify(|_, w| w.bits(csid));
        }
    }
    fn set_clk_rate(&self, div: u32) -> u32 {
        unsafe {
            //sckdiv `div` Field only [11:0] 12bit
            self.spi.sckdiv.modify(|_, w| w.bits(div));
        }
        div
    }
    fn send_data(&self, chip_select: u32, tx: &[u8]) {
        unsafe {
            // set direction
            self.spi.fmt.modify(|_, w| w.dir().set_bit());
            // chip_select
            self.spi.csid.modify(|_, w| w.bits(chip_select));
            // csmode: hold mode
            self.spi.csmode.modify(|_, w| w.mode().bits(2));
        }
        let len = tx.len();
        let mut remaining = len;
        while remaining != 0usize {
            let n_words = if 8usize < remaining { 8 } else { remaining };
            // set watermark
            unsafe {
                self.spi.txmark.modify(|_, w| w.bits(1));
            }
            // wait for spi
            while !self.spi.ip.read().txwm().bits() {
                // loop
            }
            // enque spi
            for _ in 0..n_words {
                unsafe {
                    self.spi
                        .txdata
                        .modify(|_, w| w.data().bits(tx[len - remaining]));
                }
                remaining = remaining - 1;
            }
        }
        // 释放csmode
        unsafe {
            self.spi.csmode.modify(|_, w| w.mode().bits(0)); //hold mode:2  |  auto:0 | off:3
        }
    }
    fn recv_data(&self, chip_select: u32, rx: &mut [u8]) {
        unsafe {
            // clear fmt::direction
            self.spi.fmt.modify(|_, w| w.dir().clear_bit());
            self.spi.csid.modify(|_, w| w.bits(chip_select));
            self.spi.csmode.modify(|_, w| w.mode().bits(2)); //hold mode:2  |  auto:0 | off:3
        }
        let len = rx.len();
        let mut remaining = len;

        while remaining != 0usize {
            // words need to be transferred in a single round
            let n_words = 1; // if 8usize < remaining { 8 } else { remaining };
                             // enqueue n_words junk for transmission
            for _ in 0..n_words {
                unsafe {
                    self.spi.txdata.modify(|_, w| w.bits(0xff)); //默认发ff
                }
            }
            // set watermark
            unsafe {
                self.spi.rxmark.modify(|_, w| w.bits(n_words as u32 - 1));
            }
            // wait for spi
            while !self.spi.ip.read().rxwm().bits() {
                // loop
            }
            // read out all data from rx fifo
            for _ in 0..n_words {
                rx[len - remaining] = self.spi.rxdata.read().data().bits();
                remaining = remaining - 1;
            }
        }
        // 释放csmode
        unsafe {
            self.spi.csmode.modify(|_, w| w.mode().bits(0)); //hold mode:2  |  auto:0 | off:3
        }
    }
    fn fill_data(&self, _chip_select: u32, _value: u32, _tx_len: usize) {
        panic!("spi-fill_data Unimplemented");
    }
    fn switch_cs(&self, enable: bool, csid: u32) {
        // hold mode:2  |  auto:0 | off:3
        // self.spi.csmode.modify(|_,w| w.mode().bits(if enable {2} else {3}));
        unsafe {
            self.spi
                .csmode
                .modify(|_, w| w.mode().bits(if enable { 2 } else { 0 }));
            self.spi.csid.modify(|_, w| w.bits(csid));
        }
    }
}
