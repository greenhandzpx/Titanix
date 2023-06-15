use alloc::{boxed::Box, string::String, sync::Arc};
use log::{debug, info, trace};
use riscv::register::scause::Scause;

use crate::{
    mm::{
        address::KernelAddr, frame_alloc, page_table::PTEFlags, PageTable, PhysPageNum, VirtAddr,
    },
    process::Process,
    processor::current_process,
    utils::error::{AgeneralRet, GeneralRet, SyscallErr},
};

use super::VmArea;

// type Mutex<T> = SpinNoIrqLock<T>;

/// General page fault handler
pub trait PageFaultHandler: Send + Sync {
    /// Handle the specific virtual page fault synchronously.
    /// Return true if no async handler should be invoked
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        vma: &VmArea,
        page_table: &mut PageTable,
    ) -> GeneralRet<bool>;

    ///
    fn is_legal(&self, _scause: Scause) -> bool {
        todo!();
    }

    /// Used for cloning in `fork`
    fn arc_clone(&self) -> Arc<dyn PageFaultHandler>;

    /// Handle the specific virtual page fault asynchronously.
    fn handle_page_fault_async(
        &self,
        _va: VirtAddr,
        _process: &'static Arc<Process>, // vma: &VmArea,
    ) -> AgeneralRet<()> {
        todo!()
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
    ) -> GeneralRet<bool> {
        // Box::pin(async move {
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
        Ok(true)
        // })
    }

    fn is_legal(&self, scause: Scause) -> bool {
        if scause.bits() == 13 || scause.bits() == 15 {
            debug!("ustack page fault is legal");
            true
        } else {
            false
        }
    }
    fn arc_clone(&self) -> Arc<dyn PageFaultHandler> {
        Arc::new(self.clone())
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
    ) -> GeneralRet<bool> {
        // todo!()
        // Box::pin(async move {
        debug!("handle sbrk page fault");
        let vpn = va.floor();
        let frame = frame_alloc().unwrap();
        let ppn = frame.ppn;
        let data_frames = unsafe { &mut *vma.data_frames.get() };
        data_frames.0.insert(vpn, Arc::new(frame));
        let pte_flags = PTEFlags::W | PTEFlags::R | PTEFlags::U;
        page_table.map(vpn, ppn, pte_flags);
        page_table.activate();
        Ok(true)
        // })
    }

    fn arc_clone(&self) -> Arc<dyn PageFaultHandler> {
        Arc::new(self.clone())
    }
}

///
#[derive(Clone)]
pub struct MmapPageFaultHandler {}

impl PageFaultHandler for MmapPageFaultHandler {
    // // tmp version
    // fn handle_page_fault(
    //     &self,
    //     va: VirtAddr,
    //     vma: &VmArea,
    //     page_table: &mut PageTable,
    // ) -> GeneralRet<()> {
    //     debug!("handle mmap page fault");
    //     let backup_file = vma.backup_file.as_ref().ok_or(SyscallErr::ENODEV)?;
    //     let file = backup_file.file.clone();
    //     let offset = backup_file.offset + (va.0 - VirtAddr::from(vma.start_vpn()).0);
    //     debug!("mmap offset {}", offset);
    //     let open_flags: OpenFlags = vma.map_perm.into();
    //     // let file = inode.file.open(inode.file.clone(), open_flags)?;
    //     debug!("mmap backup file name {}", file.metadata().path);
    //     let data_frames = unsafe { &mut (*vma.data_frames.get()) };
    //     let frame = frame_alloc().unwrap();
    //     let ppn = frame.ppn;
    //     data_frames.0.insert(va.floor(), Arc::new(frame));
    //     let bytes_array = ppn.bytes_array();
    //     file.seek(offset)?;
    //     file.sync_read(bytes_array)?;

    //     let mut pte_flags = vma.map_perm.into();
    //     pte_flags |= PTEFlags::U;
    //     debug!("ppn {:#x}", ppn.0);
    //     page_table.map(va.floor(), ppn, pte_flags);
    //     page_table.activate();
    //     Ok(())
    // }

    // page cache version
    fn handle_page_fault(
        &self,
        _va: VirtAddr,
        _vma: &VmArea,
        _page_table: &mut PageTable,
    ) -> GeneralRet<bool> {
        debug!("handle mmap page fault");
        Ok(false)
        // Box::pin(async move {
        // })
    }

    fn handle_page_fault_async(
        &self,
        va: VirtAddr,
        process: &'static Arc<Process>, // vma: &VmArea,
                                        // page_table: &mut PageTable,
    ) -> AgeneralRet<()> {
        Box::pin(async move {
            debug!("handle mmap page fault asynchronously");
            let (inode, mut pte_flags, start_vpn) = process.inner_handler(|proc| {
                let vma = proc
                    .memory_set
                    .find_vm_area_by_vpn(va.floor())
                    .ok_or(SyscallErr::EFAULT)?;
                Ok((
                    vma.backup_file
                        .as_ref()
                        .cloned()
                        .ok_or(SyscallErr::ENODEV)?,
                    PTEFlags::from(vma.map_perm),
                    vma.start_vpn(),
                ))
            })?;

            let offset = inode.offset + (va.0 - VirtAddr::from(start_vpn).0);
            debug!("handle mmap page fault, offset {:#x}", offset);
            let page = inode
                .file
                .metadata()
                .inner
                .lock()
                .inode
                .as_ref()
                .unwrap()
                .metadata()
                .inner
                .lock()
                .page_cache
                .as_mut()
                .unwrap()
                .get_page(offset)?;
            page.load_all_buffers().await?;
            // let mut pte_flags = vma.map_perm.into();
            pte_flags |= PTEFlags::U;
            let phy_page_num =
                PhysPageNum::from(KernelAddr::from(page.bytes_array_ptr().await as usize));
            trace!(
                "file page content {:?}",
                String::from_utf8(page.bytes_array().await.to_vec())
            );
            trace!(
                "phy page num {:#x}, kernel addr {:#x}",
                phy_page_num.0,
                page.bytes_array_ptr().await as usize
            );

            process.inner_handler(|proc| {
                let page_table = unsafe { &mut *proc.memory_set.page_table.get() };
                page_table.map(va.floor(), phy_page_num, pte_flags);
                page_table.activate();
            });
            Ok(())
        })
    }

    fn arc_clone(&self) -> Arc<dyn PageFaultHandler> {
        Arc::new(self.clone())
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
    ) -> GeneralRet<bool> {
        // todo!()
        // Box::pin(async move {
        debug!("handle fork page fault(cow), va {:#x}", va.0);
        // panic!();
        let data_frames = unsafe { &mut *vma.data_frames.get() };
        let vpn = va.floor();

        if let Some(pte) = page_table.find_pte(vpn) {
            // the page has correlated physical frame
            debug_assert!(pte.flags().contains(PTEFlags::COW));
            debug_assert!(!pte.flags().contains(PTEFlags::W));
            let old_frame = data_frames
                .0
                .get(&vpn)
                .expect("There must a physical frame");

            // modify pte
            let mut pte_flags = pte.flags() | PTEFlags::W;
            pte_flags.remove(PTEFlags::COW);

            // // Else
            // // we should allocate new frame and decrease
            // // old frame's ref cnt
            // let new_frame = frame_alloc().unwrap();
            // // copy old frame's data to the new frame
            // new_frame
            //     .ppn
            //     .bytes_array()
            //     .copy_from_slice(&old_frame.ppn.bytes_array());
            // // modify page table
            // page_table.unmap(vpn);
            // page_table.map(vpn, new_frame.ppn, pte_flags);
            // page_table.activate();
            // // decrease old frame's ref cnt
            // debug!("ph frame ref cnt {}", Arc::strong_count(old_frame));
            // data_frames.0.remove(&vpn);
            // data_frames.0.insert(vpn, Arc::new(new_frame));

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
            info!("no such frame in cow, va {:#x}", va.0);
            let new_frame = frame_alloc().unwrap();
            let mut pte_flags = PTEFlags::from_bits(vma.map_perm.bits()).unwrap() | PTEFlags::W;
            pte_flags.remove(PTEFlags::COW);
            page_table.map(vpn, new_frame.ppn, pte_flags);
            page_table.activate();
            data_frames.0.insert(vpn, Arc::new(new_frame));
        }

        Ok(true)
        // })
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
    fn arc_clone(&self) -> Arc<dyn PageFaultHandler> {
        Arc::new(self.clone())
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
        _va: VirtAddr,
        _vma: &VmArea,
        _page_table: &mut PageTable,
    ) -> GeneralRet<bool> {
        todo!()
    }

    fn arc_clone(&self) -> Arc<dyn PageFaultHandler> {
        Arc::new(self.clone())
    }
}
