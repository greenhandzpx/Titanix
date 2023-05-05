use core::mem::size_of;

use alloc::{sync::Arc, vec::Vec};
use log::debug;

use crate::utils::string::string_to_array;

use super::Inode;

pub const DIRENT_SIZE: u16 = size_of::<Dirent>() as u16;

pub const MAX_NAME_LEN: usize = 16;

#[derive(Debug)]
pub struct Dirent {
    /// 64-bit inode number
    pub d_ino: usize,
    /// 64-bit offset to next derent
    pub d_off: usize,
    /// Size of this dirent
    pub d_reclen: u16,
    /// File type
    pub d_type: u8,
    /// File name
    pub d_name: [u8; MAX_NAME_LEN],
}

impl Dirent {
    pub fn get_dirents(inode: Arc<dyn Inode>) -> Vec<Self> {
        let inode_meta = inode.metadata();
        let child = inode_meta.inner.lock().children.clone();
        let mut dirents: Vec<Dirent> = Vec::new();
        for value in child.values() {
            debug!("d_name should be: {}", value.metadata().name.clone());
            let dirent = Dirent {
                d_ino: value.metadata().ino,
                d_off: 0,
                d_reclen: DIRENT_SIZE,
                d_type: value.metadata().mode as u8,
                d_name: string_to_array(value.metadata().name.clone()),
            };
            dirents.push(dirent);
        }
        dirents
    }
    fn station_debug(&self) {
        debug!("station d_ino: {:#x}", &self.d_ino as *const usize as usize);
        debug!("station d_off: {:#x}", &self.d_off as *const usize as usize);
        debug!(
            "station d_reclen: {:#x}",
            &self.d_reclen as *const u16 as usize
        );
        debug!("station d_type: {:#x}", &self.d_type as *const u8 as usize);
        debug!(
            "station d_name: {:#x}",
            &self.d_name[0] as *const u8 as usize
        );
    }
}
