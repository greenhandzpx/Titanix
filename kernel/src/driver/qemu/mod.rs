use alloc::{collections::BTreeMap, vec::Vec};
use virtio_drivers::{DeviceType, Hal};

use crate::{
    config::mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS},
    mm::{
        frame_alloc, frame_alloc_contig, frame_dealloc, FrameTracker, KernelAddr, PhysAddr,
        PhysPageNum, StepByOne, VirtAddr, KERNEL_SPACE,
    },
    println,
    sync::mutex::SpinNoIrqLock,
};

pub mod virtio_blk;
pub mod virtio_mmio;
pub mod virtio_net;

#[allow(unused)]
const VIRTIO0: usize = 0x10001000 + (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS);
#[allow(unused)]
const VIRTIO8: usize = 0x10008000 + (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS);

// static VIRTIODEVICEADDR: VirtioDeviceAddr = VirtioDeviceAddr::new();

// struct VirtioDeviceAddr(SpinNoIrqLock<Option<BTreeMap<DeviceType, usize>>>);
// impl VirtioDeviceAddr {
//     const fn new() -> Self {
//         Self(SpinNoIrqLock::new(None))
//     }
//     fn init(&self) {
//         *self.0.lock() = Some(BTreeMap::new());
//     }
// }

// pub fn init_virt_addr() {
//     VIRTIODEVICEADDR.init();
// }

static QUEUE_FRAMES: SpinNoIrqLock<Vec<FrameTracker>> = SpinNoIrqLock::new(Vec::new());

pub struct VirtioHal;

impl Hal for VirtioHal {
    fn dma_alloc(pages: usize) -> usize {
        let mut ppn_base = PhysPageNum(0);
        let mut queue_frames_locked = QUEUE_FRAMES.lock();
        let mut frames = frame_alloc_contig(pages);
        for i in 0..pages {
            let frame = frames.pop().unwrap();
            if i == pages - 1 {
                ppn_base = frame.ppn;
            }
            // println!("ppn {}", frame.ppn.0);
            // assert_eq!(frame.ppn.0, ppn_base.0 + i);
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
        log::debug!("phy2virt: addr {:#x}", addr);
        KernelAddr::from(PhysAddr::from(addr)).0
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
    }
}
