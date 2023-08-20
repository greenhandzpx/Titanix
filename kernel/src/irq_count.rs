use crate::sync::mutex::SpinNoIrqLock;
use alloc::string::{String, ToString};

type Mutex<T> = SpinNoIrqLock<T>;

pub struct IrqCounter {
    array: Mutex<[usize; 100]>,
}

impl IrqCounter {
    pub const fn new() -> Self {
        Self {
            array: Mutex::new([0; 100]),
        }
    }
    pub fn add1(&self, id: usize) {
        //        println!("add1 {}", id);
        let mut array = self.array.lock();
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
