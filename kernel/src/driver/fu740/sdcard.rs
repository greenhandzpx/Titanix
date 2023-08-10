use super::spi::SPI;
use crate::{driver::BlockDevice, println, sync::mutex::SpinNoIrqLock};
use async_task::spawn_unchecked;
use log::{error, info};
pub const BLOCK_SIZE: usize = 512;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SDCardCMD {
    /// Software reset
    CMD0 = 0,
    /// Check voltage range (SDC v2)
    CMD8 = 8,
    /// Read CSD register
    CMD9 = 9,
    /// Read CID register
    CMD10 = 10,
    /// Stop to read data
    CMD12 = 12,
    /// Change R/W block size
    CMD16 = 16,
    /// Read single block
    CMD17 = 17,
    /// Read multiple blocks
    CMD18 = 18,
    /// Number of blocks to erase (SDC)
    ACMD23 = 23,
    /// Write single block
    CMD24 = 24,
    /// Write multiple blocks
    CMD25 = 25,
    /// Initiate initialization process (SDC)
    ACMD41 = 41,
    /// Leading command for ACMD
    CMD55 = 55,
    /// Read OCR
    CMD58 = 58,
    /// Enable/disable CRC check
    CMD59 = 59,
}

#[derive(Debug)]
pub enum SDCardInitError {
    CMDFailed(SDCardCMD),
}

pub struct SDCardSPI {
    spi: SPI,
    hc: bool,
}

impl SDCardSPI {
    fn write_data(&self, data: &[u8]) {
        self.spi.configure(0, 0, 1, 0);
        self.spi.send_data(0, data);
    }
    fn read_data(&self, data: &mut [u8]) {
        self.spi.configure(0, 0, 1, 0);
        self.spi.recv_data(0, data);
    }
    fn send_cmd(&self, cmd: SDCardCMD, arg: u32, crc: u8) {
        self.spi.switch_cs(true, 0);
        self.write_data(&[
            ((cmd as u8) | 0x40),
            (arg >> 24) as u8,
            ((arg >> 16) & 0xff) as u8,
            ((arg >> 8) & 0xff) as u8,
            (arg & 0xff) as u8,
            crc,
        ]);
    }
    fn get_resp(&self) -> u8 {
        let mut result = [0u8];
        const TRY_CNT: usize = 0x0FFF;
        for _ in 0..TRY_CNT {
            self.read_data(&mut result);
            if result[0] != 0xFF {
                return result[0];
            }
        }
        0xFF
    }
    fn end_cmd(&self) {
        self.spi.switch_cs(false, 0);
        self.write_data(&[0xff]);
    }
    fn acmd_resp(&self, acmd: SDCardCMD, arg: u32, crc: u8) -> u8 {
        self.send_cmd(SDCardCMD::CMD55, 0, 0);
        self.get_resp();
        self.end_cmd();
        self.send_cmd(acmd, arg, crc);
        let ret = self.get_resp();
        self.end_cmd();
        ret
    }
    pub fn read_block(&self, sector: usize, buf: &mut [u8]) -> Result<(), ()> {
        if buf.len() != BLOCK_SIZE {
            return Err(());
        }
        let sector = if self.hc { sector } else { sector << 9 };
        // send CMD17
        self.end_cmd();
        self.end_cmd();
        self.write_data(&[0xff; 10]);
        self.send_cmd(SDCardCMD::CMD17, sector as u32, 0);
        let resp = self.get_resp();
        if resp != 0x00 {
            self.end_cmd();
            self.end_cmd();
            return Err(());
        }
        let resp2 = self.get_resp();
        if resp2 != 0xfe {
            self.end_cmd();
            self.end_cmd();
            return Err(());
        }
        self.read_data(buf);
        let mut crc_frame = [0u8, 0u8];
        self.read_data(&mut crc_frame);
        self.end_cmd();
        self.end_cmd();
        Ok(())
    }

    pub fn write_block(&self, sector: usize, buf: &[u8]) -> Result<(), ()> {
        if buf.len() != BLOCK_SIZE {
            return Err(());
        }
        let sector = if self.hc { sector } else { sector << 9 };
        // send CMD24 and wait response(0)
        self.send_cmd(SDCardCMD::CMD24, sector as u32, 0);
        let resp = self.get_resp();
        if resp != 0x00 {
            self.end_cmd();
            self.end_cmd();
            return Err(());
        }
        self.write_data(&[0xff, 0xfe]);
        self.write_data(&buf);
        self.write_data(&[0xff, 0xff]);
        let data_resp = self.get_resp();
        if data_resp & 0x1F != 0x05 {
            self.end_cmd();
            self.end_cmd();
            return Err(());
        }
        while self.get_resp() != 0xFF {}
        self.end_cmd();
        self.end_cmd();
        Ok(())
    }

    fn software_reset(&self) -> Result<(), SDCardInitError> {
        // try software reset
        const TRY_CNT: usize = 20;
        for _ in 0..TRY_CNT {
            self.send_cmd(SDCardCMD::CMD0, 0, 0x95);
            let resp = self.get_resp();
            self.end_cmd();
            if resp == 0x01 {
                return Ok(());
            }
        }
        Err(SDCardInitError::CMDFailed(SDCardCMD::CMD0))
    }

    fn check_voltage_range(&mut self) -> Result<(), SDCardInitError> {
        self.send_cmd(SDCardCMD::CMD8, 0x01AA, 0x87);
        let mut result = self.get_resp();
        let mut frame = [0u8; 4];
        self.read_data(&mut frame);
        self.end_cmd();
        if result != 0x01 {
            result = 0xff;
            while result != 0x00 {
                result = self.acmd_resp(SDCardCMD::ACMD41, 0x40000000, 0);
            }
        } else {
            let mut cnt = 0xff;
            while result != 0x00 {
                result = self.acmd_resp(SDCardCMD::ACMD41, 0x40000000, 0);
                if cnt == 0 {
                    return Err(SDCardInitError::CMDFailed(SDCardCMD::ACMD41));
                } else {
                    cnt -= 1;
                }
            }
            cnt = 0xff;
            result = 0xff;
            while (result != 0x0) && (result != 0x1) {
                self.send_cmd(SDCardCMD::CMD58, 0, 1);
                result = self.get_resp();
                self.read_data(&mut frame);
                self.end_cmd();
                if cnt == 0 {
                    return Err(SDCardInitError::CMDFailed(SDCardCMD::CMD58));
                } else {
                    cnt -= 1;
                }
            }
            for i in 0..4 {
                info!("sdcard: OCR[{}] = 0x{:x}", i, frame[i]);
            }
            if frame[0] & 0x40 != 0 {
                info!("sdcard is SDHC/SDXC!");
                self.hc = true;
            }
        }
        Ok(())
    }

    pub fn new(base_addr: usize) -> Result<Self, SDCardInitError> {
        let mut ret = Self {
            spi: SPI::new(base_addr),
            hc: false,
        };
        ret.spi.init();
        ret.spi.set_clk_rate(3000);
        ret.spi.switch_cs(false, 0);
        ret.spi.configure(0, 0, 1, 0);
        ret.write_data(&[0xff; 10]);
        ret.software_reset()?;
        ret.check_voltage_range()?;
        ret.spi.switch_cs(false, 0);
        ret.write_data(&[0xff; 10]);
        ret.spi.set_clk_rate(3);
        Ok(ret)
    }
}

pub struct SDCard(SpinNoIrqLock<SDCardSPI>);

impl SDCard {
    pub fn new(base_addr: usize) -> Self {
        let sb = SDCardSPI::new(base_addr).unwrap();
        Self(SpinNoIrqLock::new(sb))
    }
}

impl BlockDevice for SDCard {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0.lock().read_block(block_id, buf).unwrap();
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0.lock().write_block(block_id, buf).unwrap();
    }
}
