use alloc::{boxed::Box, string::String, sync::Arc};
use log::{debug, info, trace, warn};
use riscv::register::scause::Scause;

use crate::{
    mm::{
        address::KernelAddr, frame_alloc, page::PageBuilder, page_table::PTEFlags, MapPermission,
        PhysPageNum, VirtAddr,
    },
    process::Process,
    processor::{current_process, SumGuard},
    stack_trace,
    syscall::MmapFlags,
    utils::error::{AgeneralRet, GeneralRet, SyscallErr},
};

use super::{MemorySpace, VmArea};

// type Mutex<T> = SpinNoIrqLock<T>;

/// General page fault handler
pub trait PageFaultHandler: Send + Sync {
    /// Handle the specific virtual page fault synchronously.
    /// Return true if no async handler should be invoked
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        memory_space: &MemorySpace,
        vma: Option<&VmArea>,
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
        memory_space: &MemorySpace,
        vma: Option<&VmArea>,
    ) -> GeneralRet<bool> {
        stack_trace!();
        info!(
            "[UStackPageFaultHandler] handle ustack page fault, va {:#x}",
            va.0
        );
        let vpn = va.floor();
        let frame = frame_alloc().unwrap();
        let ppn = frame.ppn;
        let page = PageBuilder::new()
            .permission(vma.as_ref().unwrap().map_perm)
            .physical_frame(frame)
            .build();
        let data_frames = unsafe { &mut *vma.as_ref().unwrap().data_frames.get() };
        data_frames.0.insert(vpn, Arc::new(page));
        let pte_flags = PTEFlags::W | PTEFlags::R | PTEFlags::U;
        let page_table = memory_space.page_table.get_unchecked_mut();
        page_table.map(vpn, ppn, pte_flags);
        page_table.activate();
        info!(
            "[UStackPageFaultHandler] handle ustack page fault finished, pa {:#x}",
            ppn.0,
        );
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
        memory_space: &MemorySpace,
        vma: Option<&VmArea>,
    ) -> GeneralRet<bool> {
        // todo!()
        stack_trace!();
        // Box::pin(async move {
        debug!("handle sbrk page fault, va {:#x}", va.0);
        let vpn = va.floor();
        let frame = frame_alloc().unwrap();
        let ppn = frame.ppn;
        let page = PageBuilder::new()
            .permission(vma.as_ref().unwrap().map_perm)
            .physical_frame(frame)
            .build();
        let data_frames = unsafe { &mut *vma.as_ref().unwrap().data_frames.get() };
        let page_table = memory_space.page_table.get_unchecked_mut();
        // if let Some(frame) = data_frames.0.get(&vpn) {
        //     warn!("[sbrk page fault handler]: already exist phyiscal frame {:#x} for va {:#x}, pte flags {:?}", ppn.0, va.0, page_table.find_pte(va.floor()).unwrap().flags());
        // }
        data_frames.0.insert(vpn, Arc::new(page));
        let pte_flags = PTEFlags::W | PTEFlags::R | PTEFlags::X | PTEFlags::U;
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
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        _memory_space: &MemorySpace,
        _vma: Option<&VmArea>,
    ) -> GeneralRet<bool> {
        debug!("handle mmap page fault, va {:#x}", va.0);
        Ok(false)
    }

    fn handle_page_fault_async(
        &self,
        va: VirtAddr,
        process: &'static Arc<Process>,
    ) -> AgeneralRet<()> {
        stack_trace!();
        Box::pin(async move {
            debug!("handle mmap page fault asynchronously");
            let (inode, map_perm, mmap_flags, start_vpn) = process.inner_handler(|proc| {
                let vma = proc
                    .memory_space
                    .find_vm_area_by_vpn(va.floor())
                    .ok_or(SyscallErr::EFAULT)?;
                Ok((
                    vma.backup_file
                        .as_ref()
                        .cloned()
                        .ok_or(SyscallErr::ENODEV)?,
                    vma.map_perm,
                    vma.mmap_flags.unwrap(),
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
                .get_page(offset, Some(map_perm))?;
            page.load_all_buffers().await?;
            // let mut pte_flags = vma.map_perm.into();
            let pte_flags: PTEFlags = PTEFlags::from(map_perm) | PTEFlags::U;
            trace!(
                "file page content {:?}",
                String::from_utf8(page.bytes_array().to_vec())
            );

            let page = match (
                pte_flags.contains(PTEFlags::W),
                mmap_flags.contains(MmapFlags::MAP_PRIVATE),
            ) {
                (true, true) => {
                    // Copy on write
                    let frame = frame_alloc().unwrap();
                    frame.ppn.bytes_array().copy_from_slice(&page.bytes_array());
                    let file_info = page.file_info.as_ref().unwrap().lock().await;
                    Arc::new(
                        PageBuilder::new()
                            .permission(map_perm)
                            .file_info(&file_info)
                            .physical_frame(frame)
                            .build(),
                    )
                }
                _ => page,
            };

            debug!(
                "[MmapPageFaultHandler]: va {:#x}, ppn {:#x}, map perm {:?}",
                va.0, page.data_frame.ppn.0, map_perm
            );

            process.inner_handler(|proc| {
                let page_table = proc.memory_space.page_table.get_unchecked_mut();
                page_table.map(va.floor(), page.data_frame.ppn, pte_flags);
                page_table.activate();
                let vma = proc.memory_space.find_vm_area_by_vpn(va.floor()).unwrap();
                vma.data_frames
                    .get_unchecked_mut()
                    .0
                    .insert(va.floor(), page);
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
pub struct CowPageFaultHandler {}

impl PageFaultHandler for CowPageFaultHandler {
    fn handle_page_fault(
        &self,
        va: VirtAddr,
        memory_space: &MemorySpace,
        _vma: Option<&VmArea>,
    ) -> GeneralRet<bool> {
        stack_trace!();
        debug!("handle cow page fault(cow), va {:#x}", va.0);

        let vpn = va.floor();
        let page_table = memory_space.page_table.get_unchecked_mut();

        if let Some(pte) = page_table.find_pte(vpn) {
            stack_trace!();
            // the page has correlated physical frame
            debug_assert!(pte.flags().contains(PTEFlags::COW));
            debug_assert!(!pte.flags().contains(PTEFlags::W));
            let shared_page = memory_space
                .cow_pages
                .page_mgr
                .get_unchecked_mut()
                .0
                .get(&va.floor())
                .unwrap();

            if !shared_page.permission.contains(MapPermission::W) {
                warn!("pagefault illegal although cow since map perm doesn't contain W, va {:#x}, ppn {:#x}, map perm {:?}, pte flags {:?}", va.0, pte.ppn().0, shared_page.permission, pte.flags());
                return Err(SyscallErr::EFAULT);
            }

            // modify pte
            let mut pte_flags = pte.flags() | PTEFlags::W;
            pte_flags.remove(PTEFlags::COW);

            // Note that we must hold the process_inner's lock now
            // so it is safe for us to check the ref count.

            let page = match Arc::strong_count(shared_page) {
                1 => {
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
                    memory_space
                        .cow_pages
                        .page_mgr
                        .get_unchecked_mut()
                        .0
                        .remove(&vpn)
                        .unwrap()
                }
                _ => {
                    stack_trace!();
                    // Else
                    // we should allocate new frame and decrease
                    // old frame's ref cnt
                    let new_frame = frame_alloc().unwrap();
                    // copy old frame's data to the new frame
                    let _sum_guard = SumGuard::new();
                    new_frame
                        .ppn
                        .bytes_array()
                        .copy_from_slice(&shared_page.bytes_array());
                    stack_trace!();
                    // modify page tableray());
                    // modify page table
                    page_table.unmap(vpn);
                    page_table.activate();
                    page_table.map(vpn, new_frame.ppn, pte_flags);
                    // decrease old frame's ref cnt
                    memory_space
                        .cow_pages
                        .page_mgr
                        .get_unchecked_mut()
                        .0
                        .remove(&vpn);
                    stack_trace!();
                    Arc::new(
                        PageBuilder::new()
                            .permission(shared_page.permission)
                            .physical_frame(new_frame)
                            .build(),
                    )
                }
            };
            stack_trace!();
            let old_vma = memory_space.find_vm_area_by_vpn(vpn).unwrap();
            let data_frames = old_vma.data_frames.get_unchecked_mut();
            data_frames.0.insert(vpn, page);
        } else {
            panic!();
            // // the page still not allocated (maybe because of lazy alloc(e.g. ustack))
            // // we should allocate new frame
            // info!("no such frame in cow, va {:#x}", va.0);
            // let new_frame = frame_alloc().unwrap();
            // let mut pte_flags = PTEFlags::from_bits(vma.map_perm.bits()).unwrap() | PTEFlags::W;
            // pte_flags.remove(PTEFlags::COW);
            // page_table.map(vpn, new_frame.ppn, pte_flags);
            // page_table.activate();
            // data_frames.0.insert(vpn, Arc::new(new_frame));
        }

        debug!("handle cow page fault(cow) finished, va {:#x}", va.0);
        Ok(true)
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
        _memory_space: &MemorySpace,
        _vma: Option<&VmArea>,
    ) -> GeneralRet<bool> {
        todo!()
    }

    fn arc_clone(&self) -> Arc<dyn PageFaultHandler> {
        Arc::new(self.clone())
    }
}
