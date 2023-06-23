use alloc::{sync::Arc, vec::Vec};

use crate::fs::{inode::InodeMeta, Inode, InodeMode};

pub struct MountsInode {
    metadata: InodeMeta,
    content: Vec<u8>,
}
impl MountsInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        Self {
            metadata: InodeMeta::new(Some(parent), path, InodeMode::FileREG, 0),
            content: Vec::new(),
        }
    }
}

impl Inode for MountsInode {
    fn metadata(&self) -> &InodeMeta {
        todo!()
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        todo!()
    }

    fn load_children_from_disk(&self, this: Arc<dyn Inode>) {
        todo!()
    }

    fn delete_child(&self, child_name: &str) {
        todo!()
    }
}
