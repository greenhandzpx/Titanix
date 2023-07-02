use alloc::{collections::BTreeMap, sync::Arc};
use log::{debug, error, trace, warn};

use crate::{
    config::{mm::KERNEL_DIRECT_OFFSET, mm::PAGE_SIZE},
    fs::File,
    mm::{
        address::{StepByOne, VPNRange},
        frame_alloc,
        page::{self, PageBuilder},
        page_table::PTEFlags,
        Page, PageTable, PhysPageNum, VirtAddr, VirtPageNum,
    },
    stack_trace,
    syscall::MmapFlags,
    utils::{
        cell::SyncUnsafeCell,
        error::{GeneralRet, SyscallErr},
    },
};

use super::{page_fault_handler::PageFaultHandler, MapPermission, MapType};

///
#[derive(Clone)]
pub struct PageManager(pub BTreeMap<VirtPageNum, Arc<Page>>);

impl PageManager {
    ///
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

/// Backup file struct
#[derive(Clone)]
pub struct BackupFile {
    /// Offset of the related file
    pub offset: usize,
    /// Src file
    // pub file: Arc<dyn Inode>,
    /// TODO: refactor
    pub file: Arc<dyn File>,
}

/// map area structure, controls a contiguous piece of virtual memory
pub struct VmArea {
    /// Vpn range
    pub vpn_range: VPNRange,
    /// We don't need to use lock because we've locked the process
    /// inner every time we handle page fault
    pub data_frames: SyncUnsafeCell<PageManager>,
    /// Map type
    pub map_type: MapType,
    /// Map permission
    pub map_perm: MapPermission,
    /// Mmap flags
    pub mmap_flags: Option<MmapFlags>,
    /// Page fault handler that is invoked when page fault
    pub handler: Option<Arc<dyn PageFaultHandler>>,
    /// Backup file(only used for mmap)
    pub backup_file: Option<BackupFile>,
    /// Page table.
    /// Note that this member must be visited with process lock holding
    pub page_table: Arc<SyncUnsafeCell<PageTable>>,
}

impl Drop for VmArea {
    fn drop(&mut self) {
        self.do_unmap_area(self.vpn_range);
    }
}

impl VmArea {
    /// Construct a new vma
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
        handler: Option<Arc<dyn PageFaultHandler>>,
        backup_file: Option<BackupFile>,
        page_table: Arc<SyncUnsafeCell<PageTable>>,
    ) -> Self {
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        // println!("start va {:#x}", start_va.0);
        // println!("end va {:#x}", end_va.0);
        // println!("start vpn {:#x}", start_vpn.0);
        // println!("end vpn {:#x}", end_vpn.0);
        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            data_frames: SyncUnsafeCell::new(PageManager::new()),
            map_type,
            map_perm,
            mmap_flags: None,
            handler,
            backup_file,
            page_table,
        }
    }
    /// Construct a vma from another vma.
    /// Note that we won't copy the physical data frames.
    pub fn from_another(another: &Self, page_table: Arc<SyncUnsafeCell<PageTable>>) -> Self {
        Self {
            vpn_range: VPNRange::new(another.vpn_range.start(), another.vpn_range.end()),
            data_frames: SyncUnsafeCell::new(PageManager::new()),
            map_type: another.map_type,
            map_perm: another.map_perm,
            mmap_flags: another.mmap_flags,
            handler: match another.handler.as_ref() {
                Some(handler) => Some(handler.arc_clone()),
                None => None,
            },
            backup_file: another.backup_file.clone(),
            page_table,
        }
    }

    /// Start vpn
    pub fn start_vpn(&self) -> VirtPageNum {
        self.vpn_range.start()
    }

    /// End vpn
    pub fn end_vpn(&self) -> VirtPageNum {
        self.vpn_range.end()
    }

    /// Page fault handler
    pub fn page_fault_handler(
        &self,
        va: VirtAddr,
        // page_table: &mut PageTable,
    ) -> GeneralRet<(Arc<dyn PageFaultHandler>, Option<&Self>)> {
        if let Some(handler) = self.handler.as_ref() {
            Ok((handler.clone(), Some(self)))
            // if !handler.handle_page_fault(va, Some(self), page_table)? {
            //     Ok(self.handler.as_ref().cloned())
            // } else {
            //     Ok(None)
            // }
        } else {
            warn!("No page fault handler for va {:#x}", va.0);
            Err(SyscallErr::EFAULT)
        }
    }

    /// Alloc a new physical frame and add the given va to the pagetable
    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) -> PhysPageNum {
        stack_trace!();
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0 - KERNEL_DIRECT_OFFSET);
                // println!("ppn {:#x}, vpn {:#x}", ppn.0, vpn.0);
                // ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = PageBuilder::new()
                    .permission(self.map_perm)
                    .physical_frame(frame_alloc().unwrap())
                    .build();
                ppn = frame.data_frame.ppn;
                self.data_frames.get_mut().0.insert(vpn, Arc::new(frame));
            }
            MapType::Direct => {
                ppn = PhysPageNum(vpn.0 - KERNEL_DIRECT_OFFSET);
                // println!("vpn {:#x}, ppn {:#x}", vpn.0, ppn.0);
                // todo!()
            }
        }
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        // debug!("vpn {:#x} pg ph {:#x}", vpn.0, ppn.0);
        page_table.map(vpn, ppn, pte_flags);
        ppn
    }

    /// Unmap a page
    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        stack_trace!();
        if self.map_type == MapType::Framed {
            self.data_frames.get_mut().0.remove(&vpn);
        }
        page_table.unmap(vpn);
    }
    /// Some of the pages don't have correlated phyiscal frame
    pub fn unmap_one_lazily(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        stack_trace!();
        if self.map_type == MapType::Framed {
            self.data_frames.get_mut().0.remove(&vpn);
        }
        if page_table.find_pte(vpn).is_some() {
            page_table.unmap(vpn);
        }
    }
    /// Map all pages this vma owns
    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }

    /// Unmap all pages this vma owns
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(page_table, vpn);
        }
    }

    /// Some of the pages don't have correlated phyiscal frame
    pub fn unmap_lazily(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one_lazily(page_table, vpn);
        }
    }

    /// Data: at the offset of the start va.
    /// Assume that all frames were cleared before.
    pub fn copy_data_with_offset(
        &mut self,
        page_table: &mut PageTable,
        mut offset: usize,
        data: &[u8],
    ) {
        stack_trace!();
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.start();
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE - offset)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .bytes_array()[offset..offset + src.len()];
            dst.fill(0);
            dst.copy_from_slice(src);
            start += PAGE_SIZE - offset;
            offset = 0;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }
    /// Data: start-aligned but maybe with shorter length.
    /// Assume that all frames were cleared before.
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.start();
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .bytes_array()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }

    /// Unmap a memory range in this area.
    /// Note that this method won't do any validity check,
    /// which means the removed vpn range must have at least
    /// one bound that equals to the old vpn range.
    fn do_unmap_area(&mut self, removed_vpn_range: VPNRange) {
        stack_trace!();
        trace!("[do_unmap_area] removed vpn range {:?}", removed_vpn_range);
        // Free phyical page frames
        self.data_frames
            .get_mut()
            .0
            .retain(|vpn, _| *vpn < removed_vpn_range.start() || *vpn >= removed_vpn_range.end());
        // Unmap from page table
        let page_table = self.page_table.get_unchecked_mut();
        for vpn in self.vpn_range {
            if vpn >= removed_vpn_range.start() && vpn < removed_vpn_range.end() {
                page_table.unmap_nopanic(vpn);
            }
        }
        // Write back to disk if needed
        if let Some(backup_file) = self.backup_file.as_mut() {
            backup_file.offset += VirtAddr::from(removed_vpn_range.start()).0
                - VirtAddr::from(self.vpn_range.start()).0;
            if self.mmap_flags.unwrap().contains(MmapFlags::MAP_SHARED) {
                backup_file.file.sync().ok().unwrap();
            }
        }
    }

    /// Clip the vm area.
    pub fn clip(&mut self, new_vpn_range: VPNRange) {
        debug!(
            "[VmArea::clip] old range {:?}, new range {:?}",
            self.vpn_range, new_vpn_range
        );
        assert!(new_vpn_range.start() >= self.start_vpn() && new_vpn_range.end() <= self.end_vpn());
        if self.start_vpn() < new_vpn_range.start() {
            self.do_unmap_area(VPNRange::new(self.start_vpn(), new_vpn_range.start()));
        }
        if self.end_vpn() > new_vpn_range.end() {
            self.do_unmap_area(VPNRange::new(new_vpn_range.end(), self.end_vpn()));
        }
        self.page_table.get_unchecked_mut().activate();
        self.vpn_range = new_vpn_range;
    }

    /// Unmap a memory range in this area.
    /// Return the new splited vma if any.
    pub fn unmap_area(&mut self, vpn_range: VPNRange) -> GeneralRet<Option<VmArea>> {
        stack_trace!();

        if vpn_range.start() < self.vpn_range.start() || vpn_range.end() > self.vpn_range.end() {
            warn!("[VmArea::unmap_area] invalid vpn range: {:?}", vpn_range);
            return Err(SyscallErr::EINVAL);
        }

        match (
            vpn_range.start() == self.start_vpn(),
            vpn_range.end() == self.end_vpn(),
        ) {
            (true, false) | (true, true) => {
                self.do_unmap_area(vpn_range);
                self.vpn_range = VPNRange::new(vpn_range.end(), self.end_vpn());
                self.page_table.get_unchecked_mut().activate();
                Ok(None)
            }
            (false, true) => {
                self.do_unmap_area(vpn_range);
                self.vpn_range = VPNRange::new(self.start_vpn(), vpn_range.start());
                self.page_table.get_unchecked_mut().activate();
                Ok(None)
            }
            (false, false) => {
                self.do_unmap_area(vpn_range);
                self.vpn_range = VPNRange::new(self.start_vpn(), vpn_range.start());

                let mut splited_vma = VmArea::from_another(self, self.page_table.clone());
                splited_vma.vpn_range = VPNRange::new(vpn_range.end(), self.vpn_range.end());
                for (vpn, page) in self.data_frames.get_unchecked_mut().0.iter() {
                    if *vpn >= splited_vma.vpn_range.start() && *vpn < splited_vma.vpn_range.end() {
                        splited_vma
                            .data_frames
                            .get_unchecked_mut()
                            .0
                            .insert(*vpn, page.clone());
                    }
                }
                for (vpn, _) in splited_vma.data_frames.get_unchecked_mut().0.iter() {
                    self.data_frames.get_unchecked_mut().0.remove(vpn);
                }
                if let Some(backup_file) = splited_vma.backup_file.as_mut() {
                    backup_file.offset +=
                        VirtAddr::from(vpn_range.end()).0 - VirtAddr::from(self.start_vpn()).0;
                }
                self.page_table.get_unchecked_mut().activate();
                Ok(Some(splited_vma))
            } // (true, true) => Ok(None),
        }
    }
}
