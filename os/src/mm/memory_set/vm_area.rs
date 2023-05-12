use alloc::{boxed::Box, collections::BTreeMap, sync::Arc};
use core::cell::UnsafeCell;
use log::{debug, warn};

use crate::{
    config::{mm::KERNEL_DIRECT_OFFSET, mm::PAGE_SIZE},
    fs::{File, Inode},
    mm::{
        address::{StepByOne, VPNRange},
        frame_alloc,
        page_table::PTEFlags,
        FrameTracker, PageTable, PhysPageNum, VirtAddr, VirtPageNum,
    },
    syscall::MmapFlags,
    utils::error::{GeneralRet, SyscallErr},
};

use super::{page_fault_handler::PageFaultHandler, MapPermission, MapType, MemorySet};

///
pub struct FrameManager(pub BTreeMap<VirtPageNum, Arc<FrameTracker>>);

impl FrameManager {
    ///
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

/// Backup file struct
pub struct BackupFile {
    /// Offset of the related file
    pub offset: usize,
    /// Src file
    // pub file: Arc<dyn Inode>,
    /// TODO: refactor
    pub file: Arc<dyn File>,
}

struct VmAreaBuilder {
    // TODO
}

/// map area structure, controls a contiguous piece of virtual memory
pub struct VmArea {
    /// Vpn range
    pub vpn_range: VPNRange,
    /// We don't need to use lock because we've locked the process
    /// inner every time we handle page fault
    pub data_frames: UnsafeCell<FrameManager>,
    /// Map type
    pub map_type: MapType,
    /// Map permission
    pub map_perm: MapPermission,
    /// Mmap flags
    pub mmap_flags: Option<MmapFlags>,
    /// Page fault handler that is invoked when page fault
    pub handler: Option<Box<dyn PageFaultHandler>>,
    /// Backup file(only used for mmap)
    pub backup_file: Option<BackupFile>,
}

impl VmArea {
    ///
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
        handler: Option<Box<dyn PageFaultHandler>>,
        backup_file: Option<BackupFile>,
    ) -> Self {
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        // println!("start va {:#x}", start_va.0);
        // println!("end va {:#x}", end_va.0);
        // println!("start vpn {:#x}", start_vpn.0);
        // println!("end vpn {:#x}", end_vpn.0);
        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            data_frames: UnsafeCell::new(FrameManager::new()),
            map_type,
            map_perm,
            mmap_flags: None,
            handler,
            backup_file,
        }
    }
    ///
    pub fn from_another(another: &Self) -> Self {
        let mut ret = Self {
            vpn_range: VPNRange::new(another.vpn_range.start(), another.vpn_range.end()),
            data_frames: UnsafeCell::new(FrameManager::new()),
            map_type: another.map_type,
            map_perm: another.map_perm,
            mmap_flags: None,
            handler: None,
            backup_file: None,
        };
        if another.handler.is_some() {
            ret.handler = Some(another.handler.as_ref().unwrap().box_clone());
        }
        ret
    }

    ///
    pub fn start_vpn(&self) -> VirtPageNum {
        self.vpn_range.start()
    }

    ///
    pub fn end_vpn(&self) -> VirtPageNum {
        self.vpn_range.end()
    }
    ///
    pub fn handle_page_fault(&self, va: VirtAddr, page_table: &mut PageTable) -> GeneralRet<()> {
        if let Some(handler) = self.handler.as_ref() {
            handler.handle_page_fault(va, self, page_table)
        } else {
            warn!("No page fault handler for va {:#x}", va.0);
            Err(SyscallErr::EFAULT)
        }
    }

    /// alloc a new physical frame and add the given va to the pagetable
    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) -> PhysPageNum {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
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

    /// only add the given va and pa to the pagetable without allocating new physical frame
    pub fn map_one_lazily(
        &mut self,
        page_table: &mut PageTable,
        vpn: VirtPageNum,
        ph_frame: &Arc<FrameTracker>,
    ) -> PhysPageNum {
        match self.map_type {
            MapType::Framed => {
                self.data_frames.get_mut().0.insert(vpn, ph_frame.clone());
            }
            _ => {
                panic!("Unsupported map type");
            }
        }
        let ppn = ph_frame.ppn;
        // debug!("ppn {:#x}", ppn.0);
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        // debug!("vpn {:#x} pg ph {:#x}", vpn.0, ppn.0);
        page_table.map(vpn, ppn, pte_flags);
        ppn
    }
    ///
    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frames.get_mut().0.remove(&vpn);
        }
        page_table.unmap(vpn);
    }
    /// Some of the pages don't have correlated phyiscal frame
    pub fn unmap_one_lazily(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frames.get_mut().0.remove(&vpn);
        }
        if page_table.find_pte(vpn).is_some() {
            page_table.unmap(vpn);
        }
    }
    ///
    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }
    ///
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
    /// data: start-aligned but maybe with shorter length
    /// assume that all frames were cleared before
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
}
