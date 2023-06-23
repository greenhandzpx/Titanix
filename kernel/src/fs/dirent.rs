use core::mem::size_of;

use alloc::{sync::Arc, vec::Vec};
use log::debug;

use crate::utils::string::{array_str_len, string_to_array};

use super::Inode;

pub const DIRENT_SIZE: u16 = size_of::<Dirent>() as u16;

pub const MAX_NAME_LEN: usize = 256;

#[derive(Debug)]
#[repr(C)]
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
    pub fn get_dirents(inode: Arc<dyn Inode>, start_index: usize) -> Vec<Self> {
        debug!("[dirent] start_index: {}", start_index);
        let inode_meta = inode.metadata();
        let child = inode_meta.inner.lock().children.clone();
        let mut dirents: Vec<Dirent> = Vec::new();
        for (i, (_, value)) in child.into_iter().enumerate() {
            if i < start_index {
                continue;
            }
            debug!(
                "[dirent] i is: {}, d_name is: {}, d_ino is: {}, d_type: {:?}",
                i,
                value.metadata().name.clone(),
                value.metadata().ino,
                value.metadata().mode
            );
            let mut dirent = Dirent {
                d_ino: value.metadata().ino,
                d_off: 0,
                d_reclen: 0,
                d_type: value.metadata().mode as u8,
                d_name: string_to_array(value.metadata().name.clone()),
            };
            dirent.d_reclen =
                (DIRENT_SIZE as usize - (MAX_NAME_LEN - array_str_len(&dirent.d_name))) as u16;
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
