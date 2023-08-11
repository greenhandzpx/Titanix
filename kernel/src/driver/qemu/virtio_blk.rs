use crate::config::mm::PAGE_SIZE_BITS;
use crate::config::{mm::KERNEL_DIRECT_OFFSET, mm::PAGE_SIZE};
use crate::driver::qemu::VIRTIODEVICEADDR;
use crate::driver::BlockDevice;
use crate::mm::{
    frame_alloc, frame_dealloc, FrameTracker, KernelAddr, PhysAddr, PhysPageNum, StepByOne,
    VirtAddr, KERNEL_SPACE,
};
use crate::sync::mutex::SpinNoIrqLock;
use alloc::vec::Vec;
use log::debug;
use virtio_drivers::{DeviceType, Hal, VirtIOBlk, VirtIOHeader};

use super::{VirtioHal, VIRTIO0};

pub struct VirtIOBlock(SpinNoIrqLock<VirtIOBlk<'static, VirtioHal>>);

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let res = self.0.lock().read_block(block_id, buf);
        if res.is_err() {
            panic!("Error when reading VirtIOBlk, block_id {}", block_id);
        } else {
            res.unwrap()
        }
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .lock()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}
impl VirtIOBlock {
    pub fn new() -> Self {
        unsafe {
            let inner = VIRTIODEVICEADDR.0.lock();
            let vaddr = inner.as_ref().unwrap().get(&DeviceType::Network).unwrap();
            let header = &mut *(*vaddr as *mut VirtIOHeader);
            // let header = &mut *(VIRTIO0 as *mut VirtIOHeader);
            Self(SpinNoIrqLock::new(
                VirtIOBlk::<VirtioHal>::new(header).unwrap(),
            ))
        }
    }
}
