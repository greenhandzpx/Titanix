use rand_core::RngCore;

use crate::timer::current_time_duration;

pub struct Rng {
    pub seed: usize,
}

pub const BIGPRIME: usize = 1242132739;

impl RngCore for Rng {
    fn next_u32(&mut self) -> u32 {
        let next = self.seed + current_time_duration().as_micros() as usize;
        self.seed = next;
        next as u32
    }

    fn next_u64(&mut self) -> u64 {
        let next = self.seed + current_time_duration().as_micros() as usize;
        self.seed = next;
        next as u64
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for i in 0..dest.len() {
            let number = self.next_u32();
            dest[i] = ((number >> 16) ^ (number << 8) ^ number) as u8;
        }
    }

    fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand_core::Error> {
        todo!()
    }
}

impl Rng {
    pub fn positive_u32(&mut self) -> u32 {
        let mut next = self.seed + current_time_duration().as_micros() as usize;
        while (next & 0xff) as u32 == 0 {
            self.seed = next;
            next = self.seed + current_time_duration().as_micros() as usize;
        }
        (next & 0xff) as u32
    }
}

pub static mut RNG: Rng = Rng { seed: BIGPRIME };
