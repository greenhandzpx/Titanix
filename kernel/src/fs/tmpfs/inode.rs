use alloc::{boxed::Box, string::ToString, sync::Arc};

use crate::fs::{inode::InodeMeta, Inode, InodeMode};

pub struct TmpInode {
    metadata: InodeMeta,
}

impl TmpInode {
    pub fn new(parent: Option<Arc<dyn Inode>>, name: &str, mode: InodeMode) -> Self {
        let path = match parent {
            Some(ref parent) => parent.metadata().path.clone() + name,
            None => name.to_string(),
        };
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

    fn delete_child(&self, child_name: &str) {
        self.metadata().inner.lock().children.remove(child_name);
    }

    fn mkdir(
        &self,
        this: alloc::sync::Arc<dyn Inode>,
        name: &str,
        mode: crate::fs::InodeMode,
    ) -> crate::utils::error::GeneralRet<alloc::sync::Arc<dyn Inode>> {
        let child_inode = TmpInode::new(Some(this), name, mode);
        let child_inode = Arc::new(child_inode);
        self.metadata
            .inner
            .lock()
            .children
            .insert(name.to_string(), child_inode.clone());
        Ok(child_inode)
    }

    fn mknod(
        &self,
        this: alloc::sync::Arc<dyn Inode>,
        name: &str,
        mode: crate::fs::InodeMode,
        _dev_id: Option<usize>,
    ) -> crate::utils::error::GeneralRet<alloc::sync::Arc<dyn Inode>> {
        let child_inode = TmpInode::new(Some(this), name, mode);
        let child_inode: Arc<dyn Inode> = Arc::new(child_inode);
        child_inode.create_page_cache_if_needed();
        self.metadata
            .inner
            .lock()
            .children
            .insert(name.to_string(), child_inode.clone());
        Ok(child_inode)
    }
}
