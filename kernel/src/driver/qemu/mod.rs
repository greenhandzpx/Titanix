use core::ptr::NonNull;

use alloc::vec::Vec;
use log::debug;
use virtio_drivers::{BufferDirection, Hal};

use crate::{
    config::mm::{KERNEL_DIRECT_OFFSET, PAGE_SIZE_BITS},
    mm::{
        frame_alloc_contig, frame_dealloc, FrameTracker, KernelAddr, PhysAddr, PhysPageNum,
        StepByOne, VirtAddr, KERNEL_SPACE,
    },
    sync::mutex::SpinNoIrqLock,
};

#[allow(unused)]
const VIRTIO0: usize = 0x10001000 + (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS);
#[allow(unused)]
// const VIRTIO8: usize = 0x10008000 + (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS);
const VIRTIO8: usize = 0x10008000 + (KERNEL_DIRECT_OFFSET << PAGE_SIZE_BITS);

pub mod virtio_blk;
pub mod virtio_mmio;
pub mod virtio_net;

static QUEUE_FRAMES: SpinNoIrqLock<Vec<FrameTracker>> = SpinNoIrqLock::new(Vec::new());
pub struct VirtioHal;

unsafe impl Hal for VirtioHal {
    fn dma_alloc(
        pages: usize,
        _direction: BufferDirection,
    ) -> (virtio_drivers::PhysAddr, NonNull<u8>) {
        let mut ppn_base = PhysPageNum(0);
        // We lock the queue in advance to ensure that we can get a contiguous area
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
        (pa.0, unsafe {
            NonNull::new_unchecked(KernelAddr::from(pa).0 as *mut u8)
        })
    }

    unsafe fn dma_dealloc(
        paddr: virtio_drivers::PhysAddr,
        _vaddr: NonNull<u8>,
        pages: usize,
    ) -> i32 {
        let pa = PhysAddr::from(paddr);
        let mut ppn_base: PhysPageNum = pa.into();
        for _ in 0..pages {
            frame_dealloc(ppn_base);
            ppn_base.step();
        }
        0
    }

    unsafe fn mmio_phys_to_virt(
        paddr: virtio_drivers::PhysAddr,
        _size: usize,
    ) -> core::ptr::NonNull<u8> {
        debug!("phy2virt: addr {:#x}", paddr);
        NonNull::new_unchecked(KernelAddr::from(PhysAddr::from(paddr)).0 as *mut u8)
    }

    unsafe fn share(
        buffer: core::ptr::NonNull<[u8]>,
        _direction: virtio_drivers::BufferDirection,
    ) -> virtio_drivers::PhysAddr {
        unsafe {
            (*KERNEL_SPACE
                .as_ref()
                .expect("KERENL SPACE not init yet")
                .page_table
                .get())
            .translate_va(VirtAddr::from(buffer.as_ptr() as *const usize as usize))
            .unwrap()
            .0
        }
        // todo!()
    }

    unsafe fn unshare(
        _paddr: virtio_drivers::PhysAddr,
        _buffer: core::ptr::NonNull<[u8]>,
        _direction: virtio_drivers::BufferDirection,
    ) {
        // todo!()
    }
}
