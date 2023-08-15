use core::ptr::NonNull;

use crate::config::{mm::KERNEL_DIRECT_OFFSET, mm::PAGE_SIZE};
use crate::driver::BlockDevice;
use crate::mm::{
    frame_alloc, frame_alloc_contig, frame_dealloc, FrameTracker, KernelAddr, PhysAddr,
    PhysPageNum, StepByOne, VirtAddr, KERNEL_SPACE,
};
use crate::sync::mutex::SpinNoIrqLock;
use alloc::vec::Vec;
use log::debug;
use virtio_drivers::{BufferDirection, Hal};
// use virtio_drivers::{Hal, VirtIOBlk, VirtIOHeader};
use virtio_drivers::device::blk::VirtIOBlk;
use virtio_drivers::transport::mmio::{MmioTransport, VirtIOHeader};

use super::VirtioHal;

#[allow(unused)]
// const VIRTIO0: usize = 0x10001000;
const VIRTIO0: usize = 0x10001000 + KERNEL_DIRECT_OFFSET * PAGE_SIZE;

pub struct VirtIOBlock(SpinNoIrqLock<VirtIOBlk<VirtioHal, MmioTransport>>);

unsafe impl Send for VirtIOBlock {}
unsafe impl Sync for VirtIOBlock {}

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let res = self.0.lock().read_blocks(block_id, buf);
        if res.is_err() {
            panic!("Error when reading VirtIOBlk, block_id {}", block_id);
        } else {
            res.unwrap()
        }
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .lock()
            .write_blocks(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}
impl VirtIOBlock {
    pub fn new() -> Self {
        unsafe {
            let header = &mut *(VIRTIO0 as *mut VirtIOHeader);
            Self(SpinNoIrqLock::new(
                VirtIOBlk::<VirtioHal, MmioTransport>::new(
                    MmioTransport::new(header.into()).unwrap(),
                )
                .unwrap(),
            ))
        }
    }
}
