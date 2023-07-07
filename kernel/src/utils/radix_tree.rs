use alloc::{sync::Arc, vec, vec::Vec};
use log::trace;

use crate::{config::fs::RADIX_TREE_MAP_SHIFT, utils::cell::SyncUnsafeCell};

struct RadixTreeLeafNode<T: Clone> {
    key: usize,
    data: T,
}

struct RadixTreeInternalNode<T: Clone> {
    children: SyncUnsafeCell<[Option<RadixTreeNode<T>>; 2 << RADIX_TREE_MAP_SHIFT]>,
}

impl<T: Clone> RadixTreeInternalNode<T> {
    pub fn new() -> Self {
        Self {
            children: SyncUnsafeCell::new(core::array::from_fn(|_| None)),
        }
    }
}

enum RadixTreeNode<T: Clone> {
    // TODO: not sure use `Box` or `Mutex` or something like that
    InternalNode(Arc<RadixTreeInternalNode<T>>),
    LeafNode(RadixTreeLeafNode<T>),
}

/// To simplify, this struct only fits for those whose Key type is `usize`.
/// All of the mutual exclusion should be guaranteed by user.
#[allow(unused)]
pub struct RadixTree<T: Clone> {
    level_num: usize,
    root: Arc<RadixTreeInternalNode<T>>,
}

// TODO: implement iterator for RadixTree<T>

impl<T: Clone> RadixTree<T> {
    #[allow(unused)]
    pub fn new(level_num: usize) -> Self {
        Self {
            level_num,
            root: Arc::new(RadixTreeInternalNode::new()),
        }
    }
    /// Lookup the give key.
    /// Note that elements that have the same low bits will be put into
    /// the same slot. This is what we want because when this structure
    /// is applied in `PageCache`, we will read pages contiguously.
    #[allow(unused)]
    pub fn lookup(&self, key: usize) -> Option<T> {
        let indice = self.indice(key);
        let mut parent = self.root.clone();
        for index in indice {
            let children = parent.children.get_unchecked_mut();
            if let Some(node) = children[index].as_ref() {
                match node {
                    RadixTreeNode::InternalNode(node) => {
                        let node = node.clone();
                        drop(children);
                        parent = node;
                    }
                    RadixTreeNode::LeafNode(node) => {
                        if node.key != key {
                            return None;
                        } else {
                            return Some(node.data.clone());
                        }
                    }
                }
            } else {
                return None;
            }
        }
        return None;
    }

    #[allow(unused)]
    pub fn insert(&mut self, key: usize, value: T) {
        let indice = self.indice(key);
        let mut parent = self.root.clone();
        for (i, index) in indice.iter().enumerate() {
            let children = parent.children.get_unchecked_mut();
            if children[*index].is_none() {
                if i == indice.len() - 1 {
                    trace!("[Radix tree]: insert a new leaf, key: {:#x}", key);
                    children[*index] = Some(RadixTreeNode::LeafNode(RadixTreeLeafNode {
                        key,
                        data: value,
                    }));
                    return;
                }
                children[*index] = Some(RadixTreeNode::InternalNode(Arc::new(
                    RadixTreeInternalNode::new(),
                )));
            }
            let node = children[*index].as_ref().unwrap();
            match node {
                RadixTreeNode::InternalNode(node) => {
                    let node = node.clone();
                    drop(children);
                    parent = node;
                }
                RadixTreeNode::LeafNode(_) => {
                    trace!(
                        "[Radix tree]: replace old leaf with a new one, key: {:#x}",
                        key
                    );
                    children[*index] = Some(RadixTreeNode::LeafNode(RadixTreeLeafNode {
                        key,
                        data: value,
                    }));
                    return;
                }
            }
        }
    }

    #[allow(unused)]
    fn indice(&self, mut key: usize) -> Vec<usize> {
        let mut indice = vec![0usize; self.level_num];
        for i in (0..self.level_num).rev() {
            indice[i] = key & ((1 << RADIX_TREE_MAP_SHIFT) - 1);
            key >>= RADIX_TREE_MAP_SHIFT;
        }
        indice
    }
}
