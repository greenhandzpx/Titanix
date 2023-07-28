use alloc::string::{String, ToString};
use log::info;

use super::{
    file::FAT32File, time::FAT32Timestamp, util::shortname_checksum, LNAME_MAXLEN, SNAME_LEN,
};

// const ATTR_READ_ONLY: u8 = 0x01;
// const ATTR_HIDDEN: u8 = 0x02;
// const ATTR_SYSTEM: u8 = 0x04;
// const ATTR_VOLUME_ID: u8 = 0x08;
pub const ATTR_DIRECTORY: u8 = 0x10;
// const ATTR_ARCHIVE: u8 = 0x20;
const ATTR_LONG_NAME: u8 = 0x0F;
const DENTRY_SIZE: usize = 0x20;

pub trait DentryReader {
    fn read_dentry(&mut self, data: &mut [u8]) -> usize;
}

pub trait DentryWriter: Send + Sync {
    fn write_dentry(&mut self, data: &[u8]);
}

pub struct FAT32DentryContent<'a> {
    file: &'a mut FAT32File,
    offset: usize,
}

impl<'a> FAT32DentryContent<'a> {
    pub fn new(file: &'a mut FAT32File) -> Self {
        Self { file, offset: 0 }
    }

    pub fn seek(&mut self, offset: usize) {
        self.offset = offset
    }
}

impl DentryReader for FAT32DentryContent<'_> {
    fn read_dentry(&mut self, data: &mut [u8]) -> usize {
        let ret = self.file.read(data, self.offset);
        self.offset += ret;
        ret
    }
}

impl DentryWriter for FAT32DentryContent<'_> {
    fn write_dentry(&mut self, data: &[u8]) {
        let ret = self.file.write(data, self.offset);
        self.offset += ret;
    }
}

pub struct FAT32DirEntry {
    pub lname: [u16; LNAME_MAXLEN],
    pub sname: [u8; SNAME_LEN],
    pub attr: u8,
    pub crt_time: FAT32Timestamp,
    pub wrt_time: FAT32Timestamp,
    pub acc_time: FAT32Timestamp,
    pub fstcluster: u32,
    pub filesize: u32,
}

impl FAT32DirEntry {
    pub fn fname(&self) -> String {
        let mut lname_len = 0;
        while lname_len < LNAME_MAXLEN && self.lname[lname_len] != 0 {
            lname_len += 1;
        }
        if lname_len > 0 {
            String::from_utf16_lossy(&self.lname[0..lname_len])
        } else {
            let base = &self.sname[0..8];
            let ext = &self.sname[8..11];
            let mut base_len = 8;
            let mut ext_len = 3;
            while base_len > 0 && base[base_len - 1] == b' ' {
                base_len -= 1;
            }
            while ext_len > 0 && ext[ext_len - 1] == b' ' {
                ext_len -= 1;
            }
            if ext_len > 0 {
                String::from_utf8_lossy(&base[0..base_len]).to_string()
                    + "."
                    + &String::from_utf8_lossy(&ext[0..ext_len]).to_string()
            } else {
                String::from_utf8_lossy(&base[0..base_len]).to_string()
            }
        }
    }

    pub fn read_dentry(reader: &mut dyn DentryReader) -> Option<Self> {
        let mut read_buf: [u8; DENTRY_SIZE] = [0; DENTRY_SIZE];
        let mut next_id: Option<u8> = None;
        let mut s_chksum: Option<u8> = None;
        let mut lname: [u16; LNAME_MAXLEN] = [0; LNAME_MAXLEN];

        macro_rules! lsb16 {
            ($data_idx: expr) => {
                (read_buf[$data_idx + 1] as u16) << 8 | (read_buf[$data_idx] as u16)
            };
        }
        macro_rules! s_lname {
            ($data_idx: expr, $lname_idx: expr) => {
                let data = lsb16!($data_idx);
                if data != 0xFFFF && data != 0 {
                    if $lname_idx >= LNAME_MAXLEN {
                        info!("[Dentry] Too long lname!");
                        return None;
                    }
                    lname[$lname_idx] = lsb16!($data_idx);
                }
            };
        }

        loop {
            let ret = reader.read_dentry(&mut read_buf[..]);
            if ret != DENTRY_SIZE {
                return None;
            }
            if read_buf[0] == 0xE5 {
                continue;
            }
            if read_buf[0] == 0x00 {
                return None;
            }
            if read_buf[0] == 0x05 {
                read_buf[0] = 0xE5;
            }

            let attr = read_buf[11];
            if attr == ATTR_LONG_NAME {
                let ord = read_buf[0];
                let real_ord = ord & 0x3f;
                let lname_offset = ((real_ord as usize) - 1) * 13;
                let chksum = read_buf[13];

                // check id
                if next_id.is_some() {
                    if next_id.unwrap() != ord {
                        info!("[Dentry] LDir ID not match!");
                        return None;
                    }
                } else {
                    if (ord & 0x40) != 0x40 {
                        info!("[Dentry] Not first dentry!");
                        return None;
                    }
                }

                next_id = match real_ord {
                    1 => None,
                    _ => Some(real_ord - 1),
                };

                // check chksum
                if s_chksum.is_some() {
                    if s_chksum.unwrap() != chksum {
                        info!("[Dentry] Chksum not match!");
                        return None;
                    }
                } else {
                    s_chksum = Some(chksum);
                }

                // filename
                s_lname!(1, lname_offset);
                s_lname!(3, lname_offset + 1);
                s_lname!(5, lname_offset + 2);
                s_lname!(7, lname_offset + 3);
                s_lname!(9, lname_offset + 4);
                s_lname!(14, lname_offset + 5);
                s_lname!(16, lname_offset + 6);
                s_lname!(18, lname_offset + 7);
                s_lname!(20, lname_offset + 8);
                s_lname!(22, lname_offset + 9);
                s_lname!(24, lname_offset + 10);
                s_lname!(28, lname_offset + 11);
                s_lname!(30, lname_offset + 12);
            } else {
                if next_id.is_some() {
                    info!("[Dentry] Expect long name but met with short!");
                    return None;
                }

                if s_chksum.is_some() {
                    let calc_chksum = shortname_checksum(&read_buf[0..11]);
                    if calc_chksum != s_chksum.unwrap() {
                        info!("[Dentry] Chksum not match!");
                        return None;
                    }
                }

                let mut sname: [u8; SNAME_LEN] = [0; SNAME_LEN];
                for i in 0..SNAME_LEN {
                    sname[i] = read_buf[i];
                }

                return Some(Self {
                    lname,
                    sname,
                    attr,
                    crt_time: FAT32Timestamp {
                        date: lsb16!(16),
                        time: lsb16!(14),
                        tenms: read_buf[13],
                    },
                    wrt_time: FAT32Timestamp {
                        date: lsb16!(24),
                        time: lsb16!(22),
                        tenms: 0,
                    },
                    acc_time: FAT32Timestamp {
                        date: lsb16!(18),
                        time: 0,
                        tenms: 0,
                    },
                    fstcluster: (lsb16!(20) as u32) << 16 | (lsb16!(26) as u32),
                    filesize: (lsb16!(30) as u32) << 16 | (lsb16!(28) as u32),
                });
            }
        }
    }

    fn write_dentry(&self, writer: &mut dyn DentryWriter) {
        let mut lname_len = 0;
        while lname_len < LNAME_MAXLEN && self.lname[lname_len] != 0 {
            lname_len += 1;
        }
        let mut write_buf: [u8; DENTRY_SIZE] = [0; DENTRY_SIZE];
        let ldir_count = (lname_len + 12) / 13;
        let chksum = shortname_checksum(&self.sname[..]);

        macro_rules! lsb16 {
            ($data_idx: expr, $data: expr) => {
                write_buf[$data_idx] = (($data & 0xFF) as u8);
                write_buf[$data_idx + 1] = ((($data >> 8) & 0xFF) as u8);
            };
        }

        macro_rules! s_lname {
            ($data_idx: expr, $lname_idx: expr) => {
                lsb16!($data_idx, {
                    if $lname_idx == lname_len {
                        0
                    } else if $lname_idx > lname_len {
                        0xFFFF
                    } else {
                        self.lname[$lname_idx]
                    }
                });
            };
        }

        for ldir_id in (1..=ldir_count).rev() {
            let lname_offset = (ldir_id - 1) * 13;
            s_lname!(1, lname_offset);
            s_lname!(3, lname_offset + 1);
            s_lname!(5, lname_offset + 2);
            s_lname!(7, lname_offset + 3);
            s_lname!(9, lname_offset + 4);
            s_lname!(14, lname_offset + 5);
            s_lname!(16, lname_offset + 6);
            s_lname!(18, lname_offset + 7);
            s_lname!(20, lname_offset + 8);
            s_lname!(22, lname_offset + 9);
            s_lname!(24, lname_offset + 10);
            s_lname!(28, lname_offset + 11);
            s_lname!(30, lname_offset + 12);

            write_buf[0] = (ldir_id as u8) | {
                if ldir_id == lname_len {
                    0x40
                } else {
                    0
                }
            };
            write_buf[11] = ATTR_LONG_NAME;
            write_buf[12] = 0;
            write_buf[13] = chksum;
            write_buf[26] = 0;
            write_buf[27] = 0;
            writer.write_dentry(&write_buf[..]);
        }
        for i in 0..SNAME_LEN {
            write_buf[i] = self.sname[i];
        }
        write_buf[11] = self.attr;
        write_buf[12] = 0;
        write_buf[13] = self.crt_time.tenms;
        lsb16!(16, self.crt_time.time);
        lsb16!(18, self.acc_time.date);
        lsb16!(20, (((self.fstcluster >> 16) & 0xFFFF) as u16));
        lsb16!(22, self.wrt_time.time);
        lsb16!(24, self.wrt_time.date);
        lsb16!(26, ((self.fstcluster & 0xFFFF) as u16));
        lsb16!(28, ((self.filesize & 0xFFFF) as u16));
        lsb16!(30, (((self.filesize >> 16) & 0xFFFF) as u16));
        writer.write_dentry(&write_buf[..]);
    }
}
