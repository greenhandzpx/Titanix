use rand_core::RngCore;

pub struct Rng {
    pub seed: usize,
}

pub const BIGPRIME: usize = 1242132739;

impl RngCore for Rng {
    fn next_u32(&mut self) -> u32 {
        let next = (self.seed.rotate_left(7) + 114514) * 13;
        self.seed = next;
        (next & 0xff) as u32
    }

    fn next_u64(&mut self) -> u64 {
        let next = (self.seed.rotate_left(7) + 114514) * 13;
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
