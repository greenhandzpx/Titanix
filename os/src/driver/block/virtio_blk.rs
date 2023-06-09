use super::BlockDevice;
use crate::config::{mm::KERNEL_DIRECT_OFFSET, mm::PAGE_SIZE};
use crate::mm::{
    frame_alloc, frame_dealloc, FrameTracker, KernelAddr, PhysAddr, PhysPageNum, StepByOne,
    VirtAddr, KERNEL_SPACE,
};
use crate::sync::mutex::SpinNoIrqLock;
use alloc::vec::Vec;
use lazy_static::*;
use log::{debug, info};
use virtio_drivers::{Hal, VirtIOBlk, VirtIOHeader};

#[allow(unused)]
// const VIRTIO0: usize = 0x10001000;
const VIRTIO0: usize = 0x10001000 + KERNEL_DIRECT_OFFSET * PAGE_SIZE;

pub struct VirtIOBlock(SpinNoIrqLock<VirtIOBlk<'static, VirtioHal>>);

lazy_static! {
    static ref QUEUE_FRAMES: SpinNoIrqLock<Vec<FrameTracker>> = SpinNoIrqLock::new(Vec::new());
}

impl easy_fs::BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0
            .lock()
            .read_block(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .lock()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}

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
    #[allow(unused)]
    pub fn new() -> Self {
        unsafe {
            let pa = unsafe {
                (*KERNEL_SPACE
                    .as_ref()
                    .expect("KERENL SPACE not init yet")
                    .page_table
                    .get())
                .translate_va(VirtAddr::from(VIRTIO0))
                .unwrap()
                .0
            };
            let bt = unsafe {
                *(VIRTIO0 as *mut u8)
                // *(header as *mut VirtIOHeader as *mut u8)
            };
            let header = &mut *(VIRTIO0 as *mut VirtIOHeader);
            // println!("ver {}", header.verify());
            let ret = Self(SpinNoIrqLock::new(
                VirtIOBlk::<VirtioHal>::new(header).unwrap(),
            ));
            ret
        }
    }
}

pub struct VirtioHal;

impl Hal for VirtioHal {
    fn dma_alloc(pages: usize) -> usize {
        let mut ppn_base = PhysPageNum(0);
        // We lock the queue in advance to ensure that we can get a contiguous area
        let mut queue_frames_locked = QUEUE_FRAMES.lock();
        for i in 0..pages {
            let frame = frame_alloc().unwrap();
            if i == 0 {
                ppn_base = frame.ppn;
            }
            assert_eq!(frame.ppn.0, ppn_base.0 + i);
            queue_frames_locked.push(frame);
        }
        let pa: PhysAddr = ppn_base.into();
        pa.0
    }

    fn dma_dealloc(pa: usize, pages: usize) -> i32 {
        let pa = PhysAddr::from(pa);
        let mut ppn_base: PhysPageNum = pa.into();
        for _ in 0..pages {
            frame_dealloc(ppn_base);
            ppn_base.step();
        }
        0
    }

    fn phys_to_virt(addr: usize) -> usize {
        debug!("phy2virt: addr {:#x}", addr);
        KernelAddr::from(PhysAddr::from(addr)).0
        // addr
        // todo!()
    }

    fn virt_to_phys(vaddr: usize) -> usize {
        unsafe {
            (*KERNEL_SPACE
                .as_ref()
                .expect("KERENL SPACE not init yet")
                .page_table
                .get())
            .translate_va(VirtAddr::from(vaddr))
            .unwrap()
            .0
        }
        // PageTable::from_token(kernel_token())
        //     .translate_va(VirtAddr::from(vaddr))
        //     .unwrap()
        //     .0
    }
}
