use alloc::{boxed::Box, string::ToString, sync::Arc};

use crate::{
    fs::{inode::InodeMeta, Inode, InodeMode},
    utils::path,
};

pub struct TmpInode {
    metadata: InodeMeta,
}

impl TmpInode {
    pub fn new(parent: Option<Arc<dyn Inode>>, name: &str, mode: InodeMode) -> Self {
        let path = match parent {
            Some(ref parent) => path::merge(&parent.metadata().path.clone(), name),
            None => name.to_string(),
        };
        log::info!("[TmpInode::new] path: {}", path);
        Self {
            metadata: InodeMeta::new(parent, &path, mode, 0, None),
        }
    }
}

impl Inode for TmpInode {
    fn metadata(&self) -> &crate::fs::inode::InodeMeta {
        &self.metadata
    }

    fn set_metadata(&mut self, meta: crate::fs::inode::InodeMeta) {
        self.metadata = meta;
    }

    fn read<'a>(
        &'a self,
        _offset: usize,
        _buf: &'a mut [u8],
    ) -> crate::utils::error::AgeneralRet<usize> {
        Box::pin(async move { Ok(0) })
    }

    fn write<'a>(
        &'a self,
        _offset: usize,
        _buf: &'a [u8],
    ) -> crate::utils::error::AgeneralRet<usize> {
        Box::pin(async move { Ok(0) })
    }

    fn load_children_from_disk(&self, _this: alloc::sync::Arc<dyn Inode>) {
        // There is nothing we should do
    }

    fn delete_child(&self, _child_name: &str) {
        // There is nothing we should do
    }

    fn mkdir(
        &self,
        this: alloc::sync::Arc<dyn Inode>,
        name: &str,
        mode: crate::fs::InodeMode,
    ) -> crate::utils::error::GeneralRet<alloc::sync::Arc<dyn Inode>> {
        let child_inode = TmpInode::new(Some(this), name, mode);
        Ok(Arc::new(child_inode))
    }

    fn mknod(
        &self,
        this: alloc::sync::Arc<dyn Inode>,
        name: &str,
        mode: crate::fs::InodeMode,
        _dev_id: Option<usize>,
    ) -> crate::utils::error::GeneralRet<alloc::sync::Arc<dyn Inode>> {
        let child_inode = TmpInode::new(Some(this), name, mode);
        Ok(Arc::new(child_inode))
    }
    fn child_removeable(&self) -> crate::utils::error::GeneralRet<()> {
        Ok(())
    }
}
