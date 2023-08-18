use alloc::{string::ToString, sync::Arc, vec::Vec};

use crate::utils::{error::GeneralRet, path};

use self::inode::TmpInode;

use super::{
    ffi::StatFlags, file_system::FileSystemMeta, FileSystem, FileSystemType, Inode, InodeMode,
};

pub mod inode;

pub struct TmpFs {
    metadata: FileSystemMeta,
}

impl TmpFs {
    pub fn new(
        mount_point: &str,
        dev_name: &str,
        fstype: FileSystemType,
        flags: StatFlags,
        fa_inode: Option<Arc<dyn Inode>>,
        covered_inode: Option<Arc<dyn Inode>>,
        covered_fs: Option<Arc<dyn FileSystem>>,
    ) -> GeneralRet<Self> {
        let mut root_inode = TmpInode::new(
            fa_inode.clone(),
            path::get_name(mount_point),
            InodeMode::FileDIR,
        );
        root_inode.root_init(Option::clone(&fa_inode), mount_point, InodeMode::FileDIR, 0)?;
        let root_inode = Arc::new(root_inode);
        Ok(Self {
            metadata: FileSystemMeta {
                dev_name: dev_name.to_string(),
                mount_point: mount_point.to_string(),
                fstype,
                flags,
                root_inode,
                fa_inode,
                covered_inode,
                covered_fs,
                s_dirty: Vec::new(),
            },
        })
    }
}

impl FileSystem for TmpFs {
    fn metadata(&self) -> &super::file_system::FileSystemMeta {
        &self.metadata
    }
}
