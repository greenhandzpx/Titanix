use core::hash::{BuildHasher, BuildHasherDefault, Hasher};

use alloc::sync::Arc;

use crate::utils::hash_table::Hashable;

#[derive(Clone, PartialEq)]
pub struct HashName {
    pub name_hash: u64,
    pub parent: usize,
    pub name: Arc<str>,
}

#[derive(Default)]
struct MyHasher(u64);

impl Hasher for MyHasher {
    fn write(&mut self, bytes: &[u8]) {
        const MUL: u64 = 130923192384972381;
        const ADD: u64 = 410934879347345269;
        self.0 = bytes.iter().copied().fold(self.0, |x, a| {
            x.wrapping_add((a as u64).wrapping_mul(MUL))
                .wrapping_add(ADD)
        });
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

impl HashName {
    pub fn str2num(name: &str) -> u64 {
        BuildHasherDefault::<MyHasher>::default().hash_one(name)
    }
    pub fn myhash(base: u64, name: u64) -> u64 {
        base.rotate_left(32).wrapping_add(name)
    }
    pub fn hash_name(parent: Option<usize>, name: &str) -> HashName {
        let parent_ptr = match parent {
            Some(p) => p as u64,
            None => 0 as u64,
        };
        HashName {
            name_hash: Self::myhash(parent_ptr, Self::str2num(name)),
            parent: parent_ptr as usize,
            name: Arc::from(name),
        }
    }
    pub fn all_same(&self, other: &Self) -> bool {
        unsafe {
            if self.name_hash != other.name_hash {
                return false;
            }
        }
        if self.parent != other.parent {
            return false;
        }
        if self.name != other.name {
            return false;
        }
        return true;
    }
    pub fn name_same(&self, name_hash: u64, name: &str) -> bool {
        unsafe {
            if self.name_hash != name_hash {
                return false;
            }
        }
        if *self.name != *name {
            return false;
        }
        return true;
    }
}

impl Hashable for HashName {
    // you should call hash_name first
    fn hash(&self) -> usize {
        self.name_hash as usize
    }
}
