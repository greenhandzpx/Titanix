#[macro_use]
pub mod uart;

pub mod block;
pub mod sbi;
pub use block::BLOCK_DEVICE;

pub use block::buffer_cache::Buffer;
pub use block::buffer_cache::LruBufferCache;

pub fn init() {
    block::init();
    uart::init();
}
