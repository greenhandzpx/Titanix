pub mod buffer_cache;
// mod io_device_tmp;
mod sdcard;
mod spi;
mod virtio_blk;
use core::any::Any;

use alloc::sync::Arc;

use crate::{
    config::{
        board::MMIO,
        mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS},
    },
    sync::mutex::SpinNoIrqLock,
};

#[cfg(feature = "board_qemu")]
pub type BlockDeviceImpl = virtio_blk::VirtIOBlock;

#[cfg(feature = "board_fu740")]
pub type BlockDeviceImpl = sdcard::SDCardWrapper;

/// MMIO virtual address
pub const MMIO_VIRT: &[(usize, usize)] = &[(
    MMIO[0].0 + (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS),
    MMIO[0].1,
)];

type Mutex<T> = SpinNoIrqLock<T>;

pub static BLOCK_DEVICE: Mutex<Option<Arc<dyn BlockDevice>>> = Mutex::new(None);

pub fn init() {
    *BLOCK_DEVICE.lock() = Some(Arc::new(BlockDeviceImpl::new()));
}

pub trait BlockDevice: Send + Sync + Any {
    ///Read data form block to buffer
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    ///Write data from buffer to block
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
