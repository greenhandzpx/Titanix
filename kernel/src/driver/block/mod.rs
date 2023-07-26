pub mod buffer_cache;
// mod io_device_tmp;
mod sdcard;
mod spi;
mod virtio_blk;
use core::any::Any;

use alloc::sync::Arc;

use crate::{
    config::mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS},
    mm::MapPermission,
    sync::mutex::SpinNoIrqLock,
};

#[cfg(feature = "board_u740")]
pub type BlockDeviceImpl = sdcard::SDCardWrapper;

#[cfg(feature = "board_qemu")]
pub type BlockDeviceImpl = virtio_blk::VirtIOBlock;

#[cfg(not(any(feature = "board_qemu", feature = "board_u740")))]
pub type BlockDeviceImpl = virtio_blk::VirtIOBlock;

use crate::config::board::MMIO;

type Mutex<T> = SpinNoIrqLock<T>;

pub static BLOCK_DEVICE: Mutex<Option<Arc<dyn BlockDevice>>> = Mutex::new(None);

pub fn init() {
    #[cfg(not(feature = "tmpfs"))]
    {
        *BLOCK_DEVICE.lock() = Some(Arc::new(BlockDeviceImpl::new()));
    }
}

pub trait BlockDevice: Send + Sync + Any {
    ///Read data form block to buffer
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    ///Write data from buffer to block
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
