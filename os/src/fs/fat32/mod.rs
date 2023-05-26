use core::panic;

use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use log::{debug, info};

use crate::{
    driver::{block::{BlockDevice, self, IoDevice, BLOCK_DEVICE}},
    sync::mutex::SpinNoIrqLock,
    utils::{error::{GeneralRet, SyscallRet, SyscallErr},path::Path},
};

use self::{bpb::BootSector, fsinfo::FSInfo, inode::FAT32Inode};
use self::{fat::FileAllocTable};

use super::{file_system::FileSystemMeta, FileSystem, FileSystemType, Inode, InodeMode};

// layouts
mod bpb;
mod disk_dentry;
mod fsinfo;
mod util;
mod fat;
mod dentry;
mod inode;
mod time;

const SECTOR_SIZE: usize = 512;
const SHORTNAME_LEN: usize = 11;
const LONGNAME_LEN: usize = 255;

type Mutex<T> = SpinNoIrqLock<T>;

#[derive(Default)]
pub struct FAT32FileSystemMeta {
    sector_per_cluster: usize,    // sector count in a cluster
    reserved_sector_count: usize, // reserved sector count
    num_fat: usize,               // fat count
    fat_size: usize,              // sector count in a FAT
    total_sector_count: usize,    // total sector count
    root_cluster_id: usize,       // root cluster id
    fsinfo_sector_id: usize,      // FSInfo sector id
}

pub struct FAT32FSInfoMeta {
    free_count: u32,  // how many free clusters ? (0xFFFFFFFF for I dont know)
    nxt_free: u32,    // next free clusters id ? (0xFFFFFFFF for I dont know)
}

impl FAT32FSInfoMeta {
    pub fn default() -> Self {
        Self {
            free_count: 0xFFFFFFFF,
            nxt_free: 0xFFFFFFFF,
        }
    }
}

pub struct FAT32FileSystem {
    block_device: Arc<dyn BlockDevice>,
    fat32fs_meta: FAT32FileSystemMeta,
    fsinfo_meta: FAT32FSInfoMeta,
    fat: FileAllocTable,
    fsmeta: Arc<Mutex<FileSystemMeta>>,
}

impl FAT32FileSystem {
    /// 传入一个 block_device，初始化 FAT32 的基本信息。如果出现错误，返回None。
    /// 否则返回新构造出来的 FS，但是还需要调用初始化，将其挂载。
    pub fn new(block_device: Arc<dyn BlockDevice>) -> Option<Self> {
        let mut data: [u8; 512] = [0; 512];
        // read Boot Sector
        block_device.read_block(0, &mut data[..]);
        let bs = BootSector::new(&data)?;
        let fat32fs_meta = FAT32FileSystemMeta {
            sector_per_cluster: bs.BPB_SectorPerCluster as usize,
            reserved_sector_count: bs.BPB_ReservedSectorCount as usize,
            num_fat: bs.BPB_NumFATs as usize,
            fat_size: bs.BPB_FATsize32 as usize,
            total_sector_count: bs.BPB_TotSector32 as usize,
            root_cluster_id: bs.BPB_RootCluster as usize,
            fsinfo_sector_id: bs.BPB_FSInfo as usize,
        };
        // read FSInfo Sector
        block_device.read_block(0, &mut data[..]);
        let fsinfo_meta: FAT32FSInfoMeta;
        if let Some(fsinfo) = FSInfo::new(&data) {
            fsinfo_meta = FAT32FSInfoMeta {
                free_count: fsinfo.FSI_Free_Count,
                nxt_free: fsinfo.FSI_Nxt_Free,
            }
        } else {
            fsinfo_meta = FAT32FSInfoMeta::default();
        }
        let fat = FileAllocTable::new(Arc::clone(&block_device), &fat32fs_meta, &fsinfo_meta);
        let ret =  Self {
            block_device: Arc::clone(&block_device),
            fat32fs_meta,
            fsinfo_meta,
            fat,
            fsmeta: Arc::new(Mutex::new(FileSystemMeta {
                ftype: FileSystemType::VFAT,
                root_inode: None,
                mnt_flags: false,
                s_dirty: Vec::new(),
            })),
        };
        Some(ret)
    }

    fn mount(&self, mount_point: &str) -> GeneralRet<()> {
        let mut fsmeta_locked = self.fsmeta.lock();
        let parent = match Path::get_parent_dir(mount_point) {
            Some(parent_dir) => <dyn Inode>::lookup_from_root_tmp(&parent_dir),
            None => None,
        };
        let root_inode = FAT32Inode::new();
        fsmeta_locked.root_inode = Some(Arc::new(root_inode));
        fsmeta_locked.mnt_flags = true;
        Ok(())
    }

    fn unmount(&self) -> GeneralRet<()> {
        self.sync_fs()?;
        let mut fsmeta_locked = self.fsmeta.lock();
        fsmeta_locked.root_inode = None;
        fsmeta_locked.mnt_flags = false;
        Ok(())
    }

    fn root_inode(&self) -> Option<Arc<dyn Inode>> {
        let fsmeta_locked = self.fsmeta.lock();
        let root_inode = fsmeta_locked.root_inode.as_ref()?;
        Some(Arc::clone(root_inode))
    }

    fn sync_fs(&self) -> GeneralRet<()> {
        todo!()
    }

}

impl FileSystem for FAT32FileSystem {
    // 为 FS 初始化一个根目录，这个函数只能被调用一次。
    fn create_root(
        &self,
        _: Option<Arc<dyn Inode>>,
        _: &str,
    ) -> GeneralRet<Arc<dyn Inode>> {
        panic!("Abolished function!");
    }

    fn init_ref(&self, _: &str, _: FileSystemType) -> GeneralRet<()> {
        panic!("Abolished function!");
    }

    fn init(&mut self, mount_point: &str, _: FileSystemType) -> GeneralRet<()> {
        panic!("Abolished function!");
        Ok(())
    }
    
    fn write_inode(&self, inode: Arc<dyn Inode>) -> SyscallRet {
        todo!()
    }
    
    fn sync_fs(&self) -> SyscallRet {
        todo!()
    }

    /// 设置 FS 的 metadata。
    fn set_metadata(&mut self, meta_data: FileSystemMeta) {
        todo!();
    }

    /// 返回 meatdata 的副本。
    fn metadata(&self) -> FileSystemMeta {
        todo!();
    }
}



lazy_static! {
    pub static ref ROOT_FS: FAT32FileSystem = {
        info!("start to init RootFS...");
        let maybe_ret = FAT32FileSystem::new(Arc::clone(&BLOCK_DEVICE));
        match maybe_ret {
            Some(ret) => {
                info!("init RootFS success!");
                ret
            },
            None => {
                panic!("init RootFS failed!");
            }
        }
        
    };
}

pub fn init() -> GeneralRet<()> {
    info!("start to init FAT32 FS...");
    ROOT_FS.init_ref("/", FileSystemType::VFAT)?;
    let root_inode = ROOT_FS.metadata().root_inode.unwrap();
    root_inode.mkdir(root_inode.clone(), "mnt", InodeMode::FileDIR)?;
    info!("init FAT32 FS success!");
    Ok(())
}