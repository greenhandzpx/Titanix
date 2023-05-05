pub mod buffer_cache;
// mod io_device_tmp;
mod virtio_blk;
use core::any::Any;

// pub use io_device_tmp::IoDevice;
mod io_device;
pub use io_device::IoDevice;

use alloc::sync::Arc;
// use easy_fs::BlockDevice;
use lazy_static::*;

use crate::config::{board::MMIO, mm::KERNEL_DIRECT_OFFSET, mm::PAGE_SIZE};

#[cfg(feature = "board_qemu")]
pub type BlockDeviceImpl = virtio_blk::VirtIOBlock;

#[cfg(feature = "board_k210")]
pub type BlockDeviceImpl = sdcard::SDCardWrapper;

/// MMIO virtual address
pub const MMIO_VIRT: &[(usize, usize)] =
    &[(MMIO[0].0 + KERNEL_DIRECT_OFFSET * PAGE_SIZE, MMIO[0].1)];

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = {
        let ret = Arc::new(BlockDeviceImpl::new());
        ret
    };
    // pub static ref BLOCK_DEVICE: Arc<dyn easy_fs::BlockDevice> = {
    //     let ret = Arc::new(BlockDeviceImpl::new());
    //     ret
    // };
}

pub trait BlockDevice: Send + Sync + Any {
    ///Read data form block to buffer
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    ///Write data from buffer to block
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
