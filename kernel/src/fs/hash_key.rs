use alloc::string::String;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct HashKey {
    pub parent_ino: usize,
    pub child_name: String,
}

impl HashKey {
    pub fn new(parent_ino: usize, child_name: String) -> Self {
        Self {
            parent_ino,
            child_name,
        }
    }
}
