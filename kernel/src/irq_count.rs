use crate::sync::mutex::SpinNoIrqLock;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

type Mutex<T> = SpinNoIrqLock<T>;

pub struct IrqCounter {
    array: Mutex<Vec<usize>>,
}

impl IrqCounter {
    pub const fn new() -> Self {
        Self {
            array: Mutex::new(Vec::new()),
        }
    }
    pub fn add1(&self, id: usize) {
        let mut array = self.array.lock();
        if array.len() <= id {
            for _ in 0..=(id - array.len()) {
                array.push(0);
            }
        }
        array[id] += 1;
    }
    pub fn dump(&self) -> String {
        let array = self.array.lock();
        let mut ret = String::new();
        for i in 0..array.len() {
            if array[i] > 0 {
                ret += &(i.to_string() + ":     " + &array[i].to_string() + "\n");
            }
        }
        ret
    }
}

pub static IRQ_COUNTER: IrqCounter = IrqCounter::new();
