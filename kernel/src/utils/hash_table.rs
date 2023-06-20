use alloc::vec;
use alloc::{string::String, vec::Vec};
use log::trace;

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for String {
    // http://www.cse.yorku.ca/~oz/hash.html
    fn hash(&self) -> usize {
        let mut result: usize = 1004535809;
        for c in self.bytes() {
            result = c as usize + (result << 6) + (result << 16) - result;
        }
        result
    }
}

impl Hashable for usize {
    fn hash(&self) -> usize {
        *self
    }
}

#[derive(Clone)]
struct HashCell<Key, Value> {
    key: Key,
    value: Option<Value>,
}

pub struct HashTable<Key, Value> {
    cells: Vec<HashCell<Key, Value>>,
    taken_count: usize,
}

impl<Key, Value> HashTable<Key, Value>
where
    Key: Clone + Default + PartialEq + Hashable,
    Value: Clone,
{
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 13;
        let ret = Self {
            // cells: Vec::with_capacity(INITIAL_CAPACITY),
            cells: vec![
                HashCell {
                    key: Key::default(),
                    value: None,
                };
                INITIAL_CAPACITY
            ],
            taken_count: 0,
        };
        trace!("[hash_table] hash table capacity {}", ret.cells.len());
        ret
    }

    pub fn extend(&mut self) {
        trace!("[hash_table] enter extend");
        assert!(self.cells.len() > 0);
        let mut new_self = Self {
            cells: vec![
                HashCell {
                    key: Key::default(),
                    value: None,
                };
                self.cells.len() * 2 + 1
            ],
            taken_count: 0,
        };

        trace!("extend capacity to {}", new_self.cells.len());

        for cell in self.cells.iter() {
            if cell.value.is_some() {
                new_self.insert(cell.key.clone(), cell.value.clone().unwrap());
            }
        }

        *self = new_self;
    }

    pub fn insert(&mut self, key: Key, new_value: Value) {
        trace!("[hash_table] enter insert");
        if let Some(old_value) = self.get_mut(&key) {
            *old_value = new_value;
        } else {
            if self.taken_count >= self.cells.len() {
                self.extend();
            }
            assert!(self.taken_count < self.cells.len());

            let mut index = key.hash() % self.cells.len();

            trace!("index: {}", index);

            while self.cells[index].value.is_some() {
                index = (index + 1) % self.cells.len();
            }

            self.cells[index].key = key;
            self.cells[index].value = Some(new_value);
            self.taken_count += 1;
        }
        trace!("[hash_table] leave insert");
    }

    pub fn delete(&mut self, key: Key) {
        trace!("[hash_table] enter delete, key: {}", key.hash());
        if let Some(index) = self.get_index(&key) {
            self.cells[index].value.take();
        }
    }

    fn get_index(&self, key: &Key) -> Option<usize> {
        trace!(
            "[hash_table] enter get_index key:{}, len:{}",
            key.hash(),
            self.cells.len()
        );
        let mut index = key.hash() % self.cells.len();
        for _ in 0..self.cells.len() {
            if !self.cells[index].value.is_some() {
                break;
            }

            if self.cells[index].key == *key {
                break;
            }

            index = (index + 1) % self.cells.len();
        }

        if self.cells[index].value.is_some() && self.cells[index].key == *key {
            Some(index)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, key: &Key) -> Option<&Value> {
        trace!("[hash_table] enter get key: {}", key.hash());
        if let Some(index) = self.get_index(key) {
            self.cells[index].value.as_ref()
            // Some(&self.cells[index].value)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        trace!("[hash_table] enter get_mut key: {}", key.hash());
        if let Some(index) = self.get_index(key) {
            self.cells[index].value.as_mut()
            // Some(&mut self.cells[index].value)
        } else {
            None
        }
    }
}
