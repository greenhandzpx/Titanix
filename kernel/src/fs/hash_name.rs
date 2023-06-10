use alloc::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct HashName {
    pub name_hash: u64,
    pub parent: usize,
    pub name: Arc<str>,
}

impl HashName {
    pub fn str2num(name: &str) -> u64 {
        let mut result: usize = 1004535809;
        for c in name.bytes() {
            result = c as usize + (result << 6) + (result << 16) - result;
        }
        result as u64
    }
    pub fn myhash(base: u64, name: u64) -> u64 {
        base.rotate_left(32).wrapping_add(name)
    }
    pub fn hash_name(parent: Option<usize>, name: &str) -> HashName {
        let parent_ptr = match parent {
            Some(p) => p as u64,
            None => 19260817 as u64,
        };
        HashName {
            name_hash: Self::myhash(parent_ptr, Self::str2num(name)),
            parent: parent_ptr as usize,
            name: Arc::from(name),
        }
    }
    pub fn all_same(&self, other: &Self) -> bool {
        if self.name_hash != other.name_hash {
            return false;
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
        if self.name_hash != name_hash {
            return false;
        }
        if *self.name != *name {
            return false;
        }
        return true;
    }
}
