use core::cell::UnsafeCell;

use alloc::{
    boxed::Box,
    sync::{Arc, Weak},
};
use log::debug;
use riscv::register::scause::Scause;

use crate::{
    mm::{
        frame_alloc, page_table::PTEFlags, FrameTracker, PageTable, PageTableEntry, PhysPageNum,
        VirtAddr, MapPermission,
    },
    processor::current_process,
    sync::mutex::SpinNoIrqLock,
    utils::error::{GeneralRet, SyscallErr}, fs::OpenFlags, syscall::MmapFlags, config::mm::PAGE_SIZE,
};

use super::VmArea;

type Mutex<T> = SpinNoIrqLock<T>;

/// General page fault handler
pub trait PageFaultHandler: Send + Sync {
    /// Handle the specific virtual page fault
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        vma: &VmArea,
        page_table: &mut PageTable,
    ) -> GeneralRet<()>;

    ///
    fn is_legal(&self, scause: Scause) -> bool {
        todo!();
    }

    /// Used for cloning in `fork`
    fn box_clone(&self) -> Box<dyn PageFaultHandler>;

    ///
    fn handle_pte(&self, pte: Option<&mut PageTableEntry>) {
        todo!();
    }
}

/// UStack page fault handler
#[derive(Clone)]
pub struct UStackPageFaultHandler {}

impl PageFaultHandler for UStackPageFaultHandler {
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        vma: &VmArea,
        page_table: &mut PageTable,
    ) -> GeneralRet<()> {
        // todo!()
        debug!("handle ustack page fault");
        // area.map_one(page_table, VirtPageNum::from(va));
        let vpn = va.floor();
        let frame = frame_alloc().unwrap();
        let ppn = frame.ppn;
        let data_frames = unsafe { &mut *vma.data_frames.get() };
        data_frames.0.insert(vpn, Arc::new(frame));
        let pte_flags = PTEFlags::W | PTEFlags::R | PTEFlags::U;
        page_table.map(vpn, ppn, pte_flags);
        page_table.activate();
        Ok(())
    }

    fn is_legal(&self, scause: Scause) -> bool {
        if scause.bits() == 13 || scause.bits() == 15 {
            debug!("ustack page fault is legal");
            true
        } else {
            false
        }
    }
    fn box_clone(&self) -> Box<dyn PageFaultHandler> {
        Box::new(self.clone())
    }
}

///
#[derive(Clone)]
pub struct SBrkPageFaultHandler {}

impl PageFaultHandler for SBrkPageFaultHandler {
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        vma: &VmArea,
        page_table: &mut PageTable,
    ) -> GeneralRet<()> {

        debug!("handle sbrk page fault");
        let vpn = va.floor();
        let frame = frame_alloc().unwrap();
        let ppn = frame.ppn;
        let data_frames = unsafe { &mut *vma.data_frames.get() };
        data_frames.0.insert(vpn, Arc::new(frame));
        let pte_flags = PTEFlags::W | PTEFlags::R | PTEFlags::U;
        page_table.map(vpn, ppn, pte_flags);
        page_table.activate();
        Ok(())

    }

    fn box_clone(&self) -> Box<dyn PageFaultHandler> {
        Box::new(self.clone())
    }
}

///
#[derive(Clone)]
pub struct MmapPageFaultHandler {}

impl PageFaultHandler for MmapPageFaultHandler {

    // tmp version
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        vma: &VmArea,
        page_table: &mut PageTable,
    ) -> GeneralRet<()> {
        debug!("handle mmap page fault");
        let backup_file = vma.backup_file.as_ref().ok_or(SyscallErr::ENODEV)?;
        let file = backup_file.file.clone();
        let offset = backup_file.offset + (va.0 - VirtAddr::from(vma.start_vpn()).0);
        debug!("mmap offset {}", offset);
        let open_flags: OpenFlags = vma.map_perm.into();
        // let file = inode.file.open(inode.file.clone(), open_flags)?;
        debug!("mmap backup file name {}", file.metadata().path);
        let data_frames = unsafe {
            &mut (*vma.data_frames.get())
        };
        let frame = frame_alloc().unwrap();
        let ppn = frame.ppn;
        data_frames.0.insert(va.floor(), Arc::new(frame));
        let bytes_array = ppn.bytes_array(); 
        file.seek(offset)?;
        file.sync_read(bytes_array)?;

        let mut pte_flags = vma.map_perm.into();
        pte_flags |= PTEFlags::U;
        page_table.map(va.floor(), ppn, pte_flags);
        page_table.activate();
        Ok(())
    }

    // page cache version
    // fn handle_page_fault(
    //     &self,
    //     va: VirtAddr,
    //     vma: &VmArea,
    //     page_table: &mut PageTable,
    // ) -> GeneralRet<()> {
    //     debug!("handle mmap page fault");
    //     let inode = vma.backup_file.as_ref().ok_or(SyscallErr::ENODEV)?;
    //     let offset = inode.offset + (va.0 - vma.start_vpn().0);
    //     let page = inode
    //         .file
    //         .metadata()
    //         .inner
    //         .lock()
    //         .page_cache
    //         .as_mut()
    //         .unwrap()
    //         .get_page(offset)?;
    //     page.load_all_buffers()?;
    //     let mut pte_flags = vma.map_perm.into();
    //     pte_flags |= PTEFlags::U;
    //     let phy_page_num = PhysPageNum::from(page.inner.lock().data.as_ptr() as usize);
    //     page_table.map(va.floor(), phy_page_num, pte_flags);
    //     page_table.activate();
    //     Ok(())
    // }

    fn box_clone(&self) -> Box<dyn PageFaultHandler> {
        Box::new(self.clone())
    }
}

///
#[derive(Clone)]
pub struct ForkPageFaultHandler {}

impl PageFaultHandler for ForkPageFaultHandler {
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        vma: &VmArea,
        page_table: &mut PageTable,
    ) -> GeneralRet<()> {
        debug!("handle fork page fault(cow)");
        // panic!();
        let data_frames = unsafe { &mut *vma.data_frames.get() };
        let vpn = va.floor();

        if let Some(pte) = page_table.find_pte(vpn) {
            // the page has correlated physical frame
            assert!(pte.flags().contains(PTEFlags::COW));
            assert!(!pte.flags().contains(PTEFlags::W));
            let old_frame = data_frames
                .0
                .get(&vpn)
                .expect("There must a physical frame");

            // modify pte
            let mut pte_flags = pte.flags() | PTEFlags::W;
            pte_flags.remove(PTEFlags::COW);
            // Note that we must hold the process_inner's lock now
            // so it is safe for us to check the ref count.
            if Arc::strong_count(old_frame) == 1 {
                debug!(
                    "[pid {}] ph frame ref cnt is 1, va: {:#x}",
                    current_process().pid(),
                    va.0
                );
                // If the ref cnt is only 1
                // we can just modify the pagetable without
                // allocating new frame
                pte.set_flags(pte_flags);
                page_table.activate();
            } else {
                // Else
                // we should allocate new frame and decrease
                // old frame's ref cnt
                let new_frame = frame_alloc().unwrap();
                // copy old frame's data to the new frame
                new_frame
                    .ppn
                    .bytes_array()
                    .copy_from_slice(&old_frame.ppn.bytes_array());
                // modify page table
                page_table.unmap(vpn);
                page_table.map(vpn, new_frame.ppn, pte_flags);
                page_table.activate();
                // decrease old frame's ref cnt
                data_frames.0.remove(&vpn);
                data_frames.0.insert(vpn, Arc::new(new_frame));
            }
        } else {
            // the page still not allocated (maybe because of lazy alloc(e.g. ustack))
            // we should allocate new frame
            let new_frame = frame_alloc().unwrap();
            let mut pte_flags = PTEFlags::from_bits(vma.map_perm.bits()).unwrap() | PTEFlags::W;
            pte_flags.remove(PTEFlags::COW);
            page_table.map(vpn, new_frame.ppn, pte_flags);
            page_table.activate();
            data_frames.0.insert(vpn, Arc::new(new_frame));
        }

        Ok(())
    }

    fn is_legal(&self, scause: Scause) -> bool {
        // todo!();
        if scause.bits() == 15 {
            debug!("fork(cow) page fault is legal");
            true
        } else {
            false
        }
    }
    fn box_clone(&self) -> Box<dyn PageFaultHandler> {
        Box::new(self.clone())
    }
    // fn handle_pte(&self, pte: Option<&mut PageTableEntry>, ppn: Option<) {
    // }
}

///
#[derive(Clone)]
pub struct ElfPageFaultHandler {}

impl PageFaultHandler for ElfPageFaultHandler {
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        vma: &VmArea,
        page_table: &mut PageTable,
    ) -> GeneralRet<()> {
        todo!()
    }

    fn box_clone(&self) -> Box<dyn PageFaultHandler> {
        Box::new(self.clone())
    }
}
