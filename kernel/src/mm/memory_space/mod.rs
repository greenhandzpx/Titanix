use alloc::{collections::BTreeMap, string::ToString, sync::Arc, vec::Vec};
use log::{debug, error, info, trace, warn};
use xmas_elf::ElfFile;

use crate::{
    config::{
        board::MEMORY_END,
        mm::{DL_INTERP_OFFSET, PAGE_SIZE},
        mm::{MMAP_TOP, USER_STACK_SIZE},
    },
    driver::block::MMIO_VIRT,
    fs::{resolve_path, OpenFlags, AT_FDCWD},
    mm::memory_space::page_fault_handler::SBrkPageFaultHandler,
    process::aux::*,
    processor::current_process,
    stack_trace,
    utils::{
        cell::SyncUnsafeCell,
        error::{GeneralRet, SyscallErr},
    },
};

use self::{cow::CowPageManager, vm_area::VmAreaType};
pub use self::{
    page_fault_handler::{CowPageFaultHandler, PageFaultHandler, UStackPageFaultHandler},
    vm_area::VmArea,
};

use super::{
    address::SimpleRange, page_table::PTEFlags, PageTable, PageTableEntry, VPNRange, VirtAddr,
    VirtPageNum,
};

///
pub mod page_fault_handler;
///
pub mod vm_area;

mod cow;

extern "C" {
    fn stext();
    fn strampoline();
    fn sigreturn_trampoline();
    fn etrampoline();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sstack();
    fn estack();
    fn sbss();
    fn ebss();
    fn ekernel();
}

const DL_INTERP: &str = "libc/libc.so";

/// Kernel Space for all processes
pub static mut KERNEL_SPACE: Option<MemorySpace> = None;

///
pub fn init_kernel_space() {
    info!("start to init kernel space...");
    unsafe {
        KERNEL_SPACE = Some(MemorySpace::new_kernel());
    }
}

/// Heap range
pub type HeapRange = SimpleRange<VirtAddr>;

/// memory space structure, controls virtual-memory space
pub struct MemorySpace {
    /// we should ensure modifying page table exclusively(e.g. through process_inner's lock)
    /// TODO: optimization: decrease the lock granularity when handling page fault
    pub page_table: Arc<SyncUnsafeCell<PageTable>>,
    /// start vpn -> vm_area
    areas: SyncUnsafeCell<BTreeMap<VirtPageNum, VmArea>>,
    /// Cow page manager
    cow_pages: CowPageManager,
    /// heap range
    pub heap_range: Option<HeapRange>,
}

impl MemorySpace {
    ///Create an empty `MemorySpace`
    pub fn new_bare() -> Self {
        let page_table = Arc::new(SyncUnsafeCell::new(PageTable::new()));
        Self {
            page_table: page_table,
            areas: SyncUnsafeCell::new(BTreeMap::new()),
            heap_range: None,
            cow_pages: CowPageManager::new(),
        }
    }

    ///Create an empty `MemorySpace` but owns the global kernel mapping
    pub fn new_from_global() -> Self {
        // TODO: optimize:
        // Now we copy all the kernel space's ptes one by one
        // but actually we can only copy the root ppn's corresponding ptes(i.e. level 1)
        // which may be faster (see `PageTable::from_global()`)
        // let kernel_space = unsafe { KERNEL_SPACE.as_ref().unwrap() };
        // let mut new_page_table = PageTable::new();
        // for (_, area) in kernel_space.areas.get_unchecked_mut().iter() {
        //     let pte_flags = PTEFlags::from_bits(area.map_perm.bits()).unwrap();
        //     for vpn in area.vpn_range {
        //         let ppn = kernel_space.translate(vpn).unwrap().ppn();
        //         new_page_table.map(vpn, ppn, pte_flags);
        //     }
        // }
        let new_page_table = PageTable::from_global();
        let page_table = Arc::new(SyncUnsafeCell::new(new_page_table));
        Self {
            page_table,
            areas: SyncUnsafeCell::new(BTreeMap::new()),
            heap_range: None,
            cow_pages: CowPageManager::new(),
        }
    }

    /// Get pagetable `root_ppn`
    pub fn token(&self) -> usize {
        self.page_table.get_unchecked_mut().token()
    }

    /// Clip the map areas overlapping with the given vpn range.
    /// Note that there may exist more than one area.
    pub fn clip_vm_areas_overlapping(&mut self, vpn_range: VPNRange) -> GeneralRet<()> {
        stack_trace!();
        let mut removed_areas: Vec<VirtPageNum> = Vec::new();
        let mut clipped_area: Option<VirtPageNum> = None;
        for (start_vpn, vma) in self
            .areas
            .get_mut()
            .range_mut(vpn_range.start()..vpn_range.end())
        {
            if vma.end_vpn() <= vpn_range.end() {
                // The vma is totally included by the given vpn range.
                // We should just remove it.
                removed_areas.push(*start_vpn);
                debug!("[clip_vm_areas_overlapping] remove vma {:?}", vma.vpn_range);
            } else {
                // Else, clip it.
                vma.clip(VPNRange::new(vpn_range.end(), vma.end_vpn()));
                debug!("[clip_vm_areas_overlapping] clip vma {:?}", vma.vpn_range);
                clipped_area = Some(*start_vpn);
            }
        }
        if let Some(clipped_area) = clipped_area {
            let vma = self.areas.get_mut().remove(&clipped_area).unwrap();
            self.areas.get_mut().insert(vma.start_vpn(), vma);
        }
        for start_vpn in removed_areas {
            self.areas.get_mut().remove(&start_vpn);
        }

        if let Some((_, vma)) = self.areas.get_mut().range_mut(..vpn_range.start()).last() {
            if vma.end_vpn() > vpn_range.start() {
                debug!("[clip_vm_areas_overlapping] clip vma {:?}", vma.vpn_range);
                vma.clip(VPNRange::new(vma.start_vpn(), vpn_range.start()));
            }
        }

        Ok(())
    }

    /// Remove vma by start vpn
    pub fn remove_vm_area(&mut self, start_vpn: VirtPageNum) -> Option<VmArea> {
        self.areas.get_unchecked_mut().remove(&start_vpn)
    }

    /// Find the immutable ref of map area by the given vpn
    pub fn find_vm_area_by_vpn(&self, vpn: VirtPageNum) -> Option<&VmArea> {
        // Range query to find the map area that this vpn belongs to
        // debug!("len before {}", self.areas.len());
        if let Some((_, vm_area)) = self.areas.get_unchecked_mut().range(..=vpn).next_back() {
            if vm_area.end_vpn() <= vpn {
                return None;
            }
            debug!(
                "[find_vm_area_by_vpn]: vpn {:#x} map area start {:#x} end {:#x}",
                vpn.0,
                vm_area.start_vpn().0,
                vm_area.end_vpn().0
            );
            // debug!("len after {}", self.areas.len());
            Some(vm_area)
        } else {
            None
        }
    }

    /// Find the mutable ref of map area by the given vpn
    pub fn find_vm_area_mut_by_vpn(&mut self, vpn: VirtPageNum) -> Option<&mut VmArea> {
        if let Some(vma) = self.find_vm_area_mut_by_vpn_included(vpn) {
            if vma.end_vpn().0 == vpn.0 {
                None
            } else {
                Some(vma)
            }
        } else {
            None
        }
    }

    /// Find the mutable ref of map area by the given vpn(including end vpn)
    pub fn find_vm_area_mut_by_vpn_included(&mut self, vpn: VirtPageNum) -> Option<&mut VmArea> {
        // Range query to find the map area that this vpn belongs to
        // debug!("len before {}", self.areas.len());
        if let Some((_, vm_area)) = self.areas.get_mut().range_mut(..=vpn).next_back() {
            if vm_area.end_vpn() < vpn {
                return None;
            }
            debug!(
                "vpn {:#x} map area start {:#x} end {:#x}",
                vpn.0,
                vm_area.start_vpn().0,
                vm_area.end_vpn().0
            );
            // debug!("len after {}", self.areas.len());
            Some(vm_area)
        } else {
            None
        }
    }

    /// Handle page fault synchronously.
    /// Return Some(handler) if async handle should be invoked.
    pub fn page_fault_handler(
        &self,
        va: VirtAddr,
        _scause: usize,
    ) -> GeneralRet<(Arc<dyn PageFaultHandler>, Option<&VmArea>)> {
        stack_trace!();
        // There are serveral kinds of page faults:
        // 1. mmap area
        // 2. sbrk area
        // 3. user stack
        // 4. fork cow area
        // 5. execve elf file
        // 6. dynamic link
        // 7. illegal page fault
        // todo!()
        // find map area
        let vpn = va.floor();
        // First we should query from cow pages
        if self
            .cow_pages
            .page_mgr
            .get_unchecked_mut()
            .0
            .get(&va.floor())
            .is_some()
        {
            return self.cow_pages.page_fault_handler(va);
        }
        // Range query to find the map area that this vpn belongs to
        if let Some(vm_area) = self.find_vm_area_by_vpn(vpn) {
            // vm_area.handle_page_fault(va, page_table)
            // let page_table = unsafe { &mut (*self.page_table.get()) };
            vm_area.page_fault_handler(va)
        } else {
            warn!("memory set len {}", self.areas.get_unchecked_mut().len());
            for area in self.areas.get_unchecked_mut().iter() {
                log::debug!(
                    "area start vpn {:#x}, end vpn {:#x}",
                    area.0 .0,
                    area.1.end_vpn().0
                );
            }
            warn!("no such vma for va {:#x}, vpn {:#x}", va.0, vpn.0);
            Err(SyscallErr::EFAULT)
        }
    }

    /// Insert vm area lazily
    pub fn insert_area(&mut self, vma: VmArea) {
        log::debug!("[insert_area] vpn range {:?}", vma.vpn_range);
        self.push_lazily(vma, None);
    }

    /// Assume that no conflicts.
    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
        vma_type: VmAreaType,
    ) {
        self.push(
            VmArea::new(
                start_va,
                end_va,
                MapType::Framed,
                permission,
                None,
                None,
                self.page_table.clone(),
                vma_type,
            ),
            0,
            None,
        );
    }

    /// Insert framed area without allocating physical memory
    pub fn insert_framed_area_lazily(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
        handler: Option<Arc<dyn PageFaultHandler>>,
        vma_type: VmAreaType,
    ) {
        self.push_lazily(
            VmArea::new(
                start_va,
                end_va,
                MapType::Framed,
                permission,
                handler,
                None,
                self.page_table.clone(),
                vma_type,
            ),
            None,
        );
    }

    ///Remove `VmArea` that starts with `start_vpn`
    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some(area) = self.areas.get_unchecked_mut().get_mut(&start_vpn) {
            area.unmap_lazily();
            self.areas.get_unchecked_mut().remove(&start_vpn);
        }
    }
    /// Add the map area to memory set and map the map area(allocating physical frames)
    fn push(&mut self, mut vm_area: VmArea, data_offset: usize, data: Option<&[u8]>) {
        stack_trace!();
        // let pgtbl_ref = self.page_table.get_unchecked_mut();
        vm_area.map();
        if let Some(data) = data {
            vm_area.copy_data_with_offset(data_offset, data);
        }
        self.areas
            .get_unchecked_mut()
            .insert(vm_area.start_vpn(), vm_area);
    }
    /// Only add the map area to memory set (without allocating physical frames)
    fn push_lazily(&self, vm_area: VmArea, _: Option<&[u8]>) {
        self.areas
            .get_unchecked_mut()
            .insert(vm_area.start_vpn(), vm_area);
        // self.areas.push(vm_area);
    }
    /// Without kernel stacks.
    pub fn new_kernel() -> Self {
        let mut memory_space = Self::new_bare();
        info!("[kernel] trampoline {:#x}", sigreturn_trampoline as usize);
        // // map trampoline
        // memory_space.map_trampoline();
        // map kernel sections
        info!(
            "[kernel].text [{:#x}, {:#x}) [{:#x}, {:#x})",
            stext as usize, strampoline as usize, etrampoline as usize, etext as usize
        );
        info!(
            "[kernel].text.trampoline [{:#x}, {:#x})",
            strampoline as usize, etrampoline as usize,
        );
        info!(
            "[kernel].rodata [{:#x}, {:#x})",
            srodata as usize, erodata as usize
        );
        info!(
            "[kernel].data [{:#x}, {:#x})",
            sdata as usize, edata as usize
        );
        info!(
            "[kernel].stack [{:#x}, {:#x})",
            sstack as usize, estack as usize
        );
        info!("[kernel].bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
        info!(
            "[kernel]physical mem [{:#x}, {:#x})",
            ekernel as usize, MEMORY_END as usize
        );

        info!("[kernel]mapping .text section");
        memory_space.push(
            VmArea::new(
                (stext as usize).into(),
                (strampoline as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
                None,
                None,
                memory_space.page_table.clone(),
                VmAreaType::Elf,
            ),
            0,
            None,
        );
        memory_space.push(
            VmArea::new(
                (etrampoline as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
                None,
                None,
                memory_space.page_table.clone(),
                VmAreaType::Elf,
            ),
            0,
            None,
        );
        info!("[kernel]mapping .rodata section");
        memory_space.push(
            VmArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
                None,
                None,
                memory_space.page_table.clone(),
                VmAreaType::Elf,
            ),
            0,
            None,
        );
        info!("[kernel]mapping .data section");
        memory_space.push(
            VmArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
                None,
                None,
                memory_space.page_table.clone(),
                VmAreaType::Elf,
            ),
            0,
            None,
        );
        // add stack section in `linker.ld`
        info!("[kernel]mapping .stack section");
        memory_space.push(
            VmArea::new(
                (sstack as usize).into(),
                (estack as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
                None,
                None,
                memory_space.page_table.clone(),
                VmAreaType::Elf,
            ),
            0,
            None,
        );
        info!("[kernel]mapping .bss section");
        memory_space.push(
            VmArea::new(
                (sbss as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
                None,
                None,
                memory_space.page_table.clone(),
                VmAreaType::Elf,
            ),
            0,
            None,
        );
        info!("[kernel]mapping signal-return trampoline");
        memory_space.push(
            VmArea::new(
                (strampoline as usize).into(),
                (etrampoline as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X | MapPermission::U,
                None,
                None,
                memory_space.page_table.clone(),
                VmAreaType::Elf,
            ),
            0,
            None,
        );
        // info!("{:#x}", unsafe { *(strampoline as usize as *const usize) });
        info!("[kernel]mapping physical memory");
        memory_space.push(
            VmArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
                None,
                None,
                memory_space.page_table.clone(),
                VmAreaType::Physical,
            ),
            0,
            None,
        );
        info!("[kernel]mapping memory-mapped registers");
        for pair in MMIO_VIRT {
            // println!("start va: {:#x}", (*pair).0);
            // println!("end va: {:#x}", (*pair).0 + (*pair).1);
            memory_space.push(
                VmArea::new(
                    (*pair).0.into(),
                    ((*pair).0 + (*pair).1).into(),
                    MapType::Direct,
                    MapPermission::R | MapPermission::W,
                    None,
                    None,
                    memory_space.page_table.clone(),
                    VmAreaType::MMIO,
                ),
                0,
                None,
            );
        }
        memory_space
    }

    /// Map the sections in the elf.
    /// Return the max end vpn and the first section's va.
    fn map_elf(&mut self, elf: &ElfFile, offset: VirtAddr) -> (VirtPageNum, VirtAddr) {
        let elf_header = elf.header;
        let ph_count = elf_header.pt2.ph_count();

        let mut max_end_vpn = offset.floor();
        let mut head_va = 0;
        info!("[map_elf]: entry point {:#x}", elf.header.pt2.entry_point());

        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va: VirtAddr = (ph.virtual_addr() as usize + offset.0).into();
                let end_va: VirtAddr =
                    ((ph.virtual_addr() + ph.mem_size()) as usize + offset.0).into();
                if head_va == 0 {
                    head_va = start_va.0;
                }
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                let vm_area = VmArea::new(
                    start_va,
                    end_va,
                    MapType::Framed,
                    map_perm,
                    None,
                    None,
                    self.page_table.clone(),
                    VmAreaType::Elf,
                );
                max_end_vpn = vm_area.vpn_range.end();

                let map_offset = start_va.0 - start_va.floor().0 * PAGE_SIZE;
                self.push(
                    vm_area,
                    map_offset,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
                info!(
                    "[map_elf]: {:#x}, {:#x}, map_perm: {:?}",
                    start_va.0, end_va.0, map_perm
                );
            }
        }

        (max_end_vpn, head_va.into())
    }

    /// Include sections in elf and TrapContext and user stack,
    /// also returns user_sp and entry point.
    /// TODO: resolve elf file lazily
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize, Vec<AuxHeader>) {
        let mut memory_space = Self::new_from_global();

        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = elf_header.pt2.ph_count();

        let mut entry_point = elf_header.pt2.entry_point() as usize;

        let mut auxv: Vec<AuxHeader> = Vec::with_capacity(64);

        auxv.push(AuxHeader {
            aux_type: AT_PHENT,
            value: elf.header.pt2.ph_entry_size() as usize,
        }); // ELF64 header 64bytes
        auxv.push(AuxHeader {
            aux_type: AT_PHNUM,
            value: ph_count as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_PAGESZ,
            value: PAGE_SIZE as usize,
        });

        if let Some(interp_entry_point) = memory_space.load_dl_interp_if_needed(&elf) {
            auxv.push(AuxHeader {
                aux_type: AT_BASE,
                value: DL_INTERP_OFFSET,
            });
            entry_point = interp_entry_point;
        } else {
            auxv.push(AuxHeader {
                aux_type: AT_BASE,
                value: 0,
            });
        }
        // _at_base = memory_space.load_dl(&elf);
        // if _at_base != 0 {
        //     auxv.push(AuxHeader {
        //         aux_type: AT_BASE,
        //         value: DYNAMIC_LINKER as usize,
        //     });
        //     _at_base += DYNAMIC_LINKER;
        // }

        auxv.push(AuxHeader {
            aux_type: AT_FLAGS,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_ENTRY,
            value: elf.header.pt2.entry_point() as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_UID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_EUID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_GID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_EGID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_PLATFORM,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_HWCAP,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_CLKTCK,
            value: 100 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_SECURE,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_NOTELF,
            value: 0x112d as usize,
        });

        let (max_end_vpn, head_va) = memory_space.map_elf(&elf, 0.into());

        let ph_head_addr = head_va.0 + elf.header.pt2.ph_offset() as usize;
        debug!("[from_elf] AT_PHDR  ph_head_addr is {:X} ", ph_head_addr);
        auxv.push(AuxHeader {
            aux_type: AT_PHDR,
            value: ph_head_addr as usize,
        });

        // map user stack with U flags
        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_bottom: usize = max_end_va.into();
        // guard page
        user_stack_bottom += PAGE_SIZE;

        // We will add the ustack to memory set later by `Thread` itself
        // Now we add heap section
        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;

        let ustack_vma = VmArea::new(
            user_stack_bottom.into(),
            user_stack_top.into(),
            MapType::Framed,
            MapPermission::U | MapPermission::R | MapPermission::W,
            Some(Arc::new(UStackPageFaultHandler {})),
            None,
            memory_space.page_table.clone(),
            VmAreaType::Stack,
        );
        memory_space.push(ustack_vma, 0, None);
        debug!(
            "[from_elf] map ustack: {:#x}, {:#x}",
            user_stack_bottom, user_stack_top,
        );

        // guard page
        let heap_start_va = user_stack_top + PAGE_SIZE;
        let map_perm = MapPermission::U | MapPermission::R | MapPermission::W;
        let heap_vma = VmArea::new(
            heap_start_va.into(),
            heap_start_va.into(),
            MapType::Framed,
            map_perm,
            Some(Arc::new(SBrkPageFaultHandler {})),
            None,
            memory_space.page_table.clone(),
            VmAreaType::Brk,
        );
        memory_space.push(heap_vma, 0, None);
        memory_space.heap_range = Some(HeapRange::new(heap_start_va.into(), heap_start_va.into()));
        debug!(
            "[from_elf] map heap: {:#x}, {:#x}",
            heap_start_va, heap_start_va
        );

        (memory_space, user_stack_top, entry_point, auxv)
    }

    /// Check whether the elf file is dynamic linked and
    /// if so, load the dl interpreter.
    /// Return the interpreter's entry point(at the base of DL_INTERP_OFFSET) if so.
    fn load_dl_interp_if_needed(&mut self, elf: &ElfFile) -> Option<usize> {
        let elf_header = elf.header;
        let ph_count = elf_header.pt2.ph_count();

        let mut is_dl = false;
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Interp {
                is_dl = true;
                break;
            }
        }

        if is_dl {
            info!("[load_dl] encounter a dl elf");
            let interp_inode = resolve_path(DL_INTERP, OpenFlags::RDONLY).ok().unwrap();
            let interp_file = interp_inode
                .open(interp_inode.clone(), OpenFlags::RDONLY)
                .ok()
                .unwrap();
            let interp_elf_data = interp_file.sync_read_all().ok().unwrap();
            let interp_elf = xmas_elf::ElfFile::new(&interp_elf_data).unwrap();
            self.map_elf(&interp_elf, DL_INTERP_OFFSET.into());

            Some(interp_elf.header.pt2.entry_point() as usize + DL_INTERP_OFFSET)
        } else {
            debug!("[load_dl] encounter a static elf");
            None
        }
    }

    ///Clone a same `MemorySpace`
    pub fn from_existed_user(user_space: &Self) -> Self {
        stack_trace!();
        // let mut memory_space = Self::new_bare();
        let mut memory_space = Self::new_from_global();
        // copy data sections/trap_context/user_stack
        for (_, area) in user_space.areas.get_unchecked_mut().iter() {
            let mut new_area = VmArea::from_another(area, memory_space.page_table.clone());
            // memory_space.push(new_area, None);
            // copy data from another space
            for vpn in area.vpn_range {
                if let Some(ppn) = user_space.translate(vpn) {
                    let src_ppn = ppn.ppn();
                    let dst_ppn = new_area.map_one(vpn, None);
                    dst_ppn.bytes_array().copy_from_slice(src_ppn.bytes_array());
                }
                // let src_ppn = user_space.translate(vpn).unwrap().ppn();
                // let dst_ppn = memory_space.translate(vpn).unwrap().ppn();
                // dst_ppn
                //     .get_bytes_array()
                //     .copy_from_slice(src_ppn.get_bytes_array());
            }
            memory_space.push_lazily(new_area, None);
        }
        memory_space.heap_range = user_space.heap_range;
        memory_space
    }
    ///Clone a same `MemorySpace`
    pub fn from_existed_user_lazily(user_space: &mut Self) -> Self {
        // TODO: optimize: no need to new a CowPageManager
        let mut memory_space = Self::new_from_global();
        // SAFETY: the process inner has been locked when invoking this function
        memory_space.cow_pages =
            CowPageManager::from_another(&user_space.cow_pages, memory_space.page_table.clone());

        let new_pagetable = memory_space.page_table.get_unchecked_mut();

        // copy data sections/trap_context/user_stack
        for (_, area) in user_space.areas.get_unchecked_mut().iter() {
            let new_area = VmArea::from_another(area, memory_space.page_table.clone());
            info!(
                "[from_existed_user_lazily] area range [{:#x}, {:#x})",
                new_area.start_vpn().0,
                new_area.end_vpn().0
            );
            // copy data from another space
            for vpn in area.vpn_range {
                // SAFETY: we've locked the process inner before calling this function
                if let Some(ph_frame) = area.data_frames.get_unchecked_mut().0.get(&vpn) {
                    // If there is related physcial frame, then we let the child and father share it.
                    let pte = user_space
                        .page_table
                        .get_unchecked_mut()
                        .find_pte(vpn)
                        .unwrap();
                    trace!(
                        "change vpn {:#x} to cow page, ppn {:#x}, pte flags {:?}",
                        vpn.0,
                        ph_frame.data_frame.ppn.0,
                        pte.flags()
                    );

                    let (pte_flags, ppn) = match area.vma_type {
                        VmAreaType::Shm => {
                            // If shared memory,
                            // then we don't need to modify the pte flags,
                            // i.e. no copy-on-write.
                            info!("[from_existed_user_lazily] vma type {:?}", area.vma_type);
                            new_area
                                .data_frames
                                .get_unchecked_mut()
                                .0
                                .insert(vpn, ph_frame.clone());
                            (pte.flags(), ph_frame.data_frame.ppn)
                        }
                        _ => {
                            // Else,
                            // copy-on-write
                            let mut new_flags = pte.flags() | PTEFlags::COW;
                            new_flags.remove(PTEFlags::W);
                            pte.set_flags(new_flags);
                            debug_assert!(pte.flags().contains(PTEFlags::COW));
                            debug_assert!(!pte.flags().contains(PTEFlags::W));
                            user_space
                                .cow_pages
                                .page_mgr
                                .get_unchecked_mut()
                                .0
                                .insert(vpn, ph_frame.clone());
                            memory_space
                                .cow_pages
                                .page_mgr
                                .get_unchecked_mut()
                                .0
                                .insert(vpn, ph_frame.clone());
                            let ppn = ph_frame.data_frame.ppn;
                            area.data_frames.get_unchecked_mut().0.remove(&vpn);
                            (new_flags, ppn)
                        }
                    };

                    new_pagetable.map(vpn, ppn, pte_flags);
                } else {
                    trace!("no ppn for vpn {:#x}", vpn.0);
                }
            }
            memory_space.push_lazily(new_area, None);
        }
        memory_space.heap_range = user_space.heap_range;
        // user_space.activate();
        // new_pagetable.activate();

        memory_space
    }
    ///Refresh TLB with `sfence.vma`
    pub fn activate(&self) {
        self.page_table.get_unchecked_mut().activate()
    }
    ///Translate throuth pagetable
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        unsafe { (*self.page_table.get()).translate(vpn) }
    }
    ///Remove all `VmArea`
    pub fn recycle_data_pages(&mut self) {
        //*self = Self::new_bare();
        self.areas.get_unchecked_mut().clear();
    }

    /// Allocate an unused area by specific start va.
    /// Note that length is counted by byte.
    pub fn allocate_spec_area(
        &mut self,
        length: usize,
        map_permission: MapPermission,
        start_va: VirtAddr,
        vma_type: VmAreaType,
    ) -> GeneralRet<Option<VmArea>> {
        if length == 0 {
            return Ok(None);
        }
        let length_rounded = (length - 1 + PAGE_SIZE) / PAGE_SIZE * PAGE_SIZE;
        let end_va: VirtAddr = (start_va.0 + length_rounded).into();
        debug!(
            "[allocate_spec_area] start va {:#x}, end va {:#x}",
            start_va.0, end_va.0
        );
        if start_va.0 % PAGE_SIZE != 0 {
            return Err(SyscallErr::EINVAL);
        }
        // TODO: just sanity check, should find a safer way
        // TODO: check more carefully
        self.clip_vm_areas_overlapping(VPNRange::new(start_va.floor(), end_va.ceil()))?;
        // if self.find_vm_area_by_vpn(start_va.floor()).is_some() {
        //     warn!("[allocate_spec_area] conflicted vm area!");
        //     return None;
        // }

        Ok(Some(VmArea::new(
            start_va,
            end_va,
            MapType::Framed,
            map_permission,
            None,
            None,
            self.page_table.clone(),
            vma_type,
        )))
    }

    /// Allocate an unused area(mostly for mmap).
    /// Note that length is counted by byte.
    pub fn allocate_area(
        &self,
        length: usize,
        map_permission: MapPermission,
        vma_type: VmAreaType,
    ) -> Option<VmArea> {
        if length == 0 {
            return None;
        }
        let mut last_start = MMAP_TOP;
        // traverse reversely
        let length_rounded = (length - 1 + PAGE_SIZE) / PAGE_SIZE * PAGE_SIZE;
        for (start_vpn, vma) in self.areas.get_unchecked_mut().iter().rev() {
            log::debug!(
                "key start {:#x}, start {:#x}, end {:#x}",
                start_vpn.0,
                vma.start_vpn().0,
                vma.end_vpn().0
            );
            let curr_end = vma.end_vpn().0 * PAGE_SIZE;
            if last_start - curr_end >= length_rounded {
                let new_start = last_start - length_rounded;
                log::debug!("[allocate_area] [{:#x}, {:#x}]", new_start, last_start);
                return Some(VmArea::new(
                    new_start.into(),
                    last_start.into(),
                    MapType::Framed,
                    map_permission,
                    None,
                    None,
                    self.page_table.clone(),
                    vma_type,
                ));
            }
            last_start = vma.start_vpn().0 * PAGE_SIZE;
        }
        error!("[allocate area] cannot find any unused vm area!!");
        None
    }

    /// Check whether the given vpn range conflicts with other vma.
    /// Note that the start_vpn must have been in memory set.
    pub fn check_vpn_range_conflict(&self, start_vpn: VirtPageNum, end_vpn: VirtPageNum) -> bool {
        for vma in self.areas.get_unchecked_mut().iter() {
            if *vma.0 == start_vpn {
                continue;
            }
            if vma.1.end_vpn() > start_vpn && vma.1.start_vpn() < end_vpn {
                debug!(
                    "conflict vpn range: input vpnr: [{:#x}, {:#x}], old vpnr: [{:#x}, {:#x}]",
                    start_vpn.0,
                    end_vpn.0,
                    vma.1.start_vpn().0,
                    vma.1.end_vpn().0
                );
                return true;
            }
        }
        return false;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
/// map type for memory set: identical or framed
pub enum MapType {
    /// vpn == ppn
    Identical,
    /// vpn == ppn + offset
    Direct,
    ///
    Framed,
}

bitflags! {
    /// map permission corresponding to that in pte: `R W X U`
    pub struct MapPermission: u16 {
        ///Readable
        const R = 1 << 1;
        ///Writable
        const W = 1 << 2;
        ///Excutable
        const X = 1 << 3;
        ///Accessible in U mode
        const U = 1 << 4;
        // /// COW when fork
        // const COW = 1 << 8;
    }
}

#[allow(unused)]
///Check PageTable running correctly
pub fn remap_test() {
    // todo!();
    info!("remap_test start...");
    let kernel_space = unsafe { KERNEL_SPACE.as_ref().unwrap() };
    // let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_text: VirtAddr = (stext as usize + (etext as usize - stext as usize) / 2).into();
    let mid_rodata: VirtAddr =
        (srodata as usize + (erodata as usize - srodata as usize) / 2).into();
    let mid_data: VirtAddr = (sdata as usize + (edata as usize - sdata as usize) / 2).into();
    debug!(
        "mid text {:#x}, mid rodata {:#x}, mid data {:#x}",
        mid_text.0, mid_rodata.0, mid_data.0
    );
    unsafe {
        assert!(!(*kernel_space.page_table.get())
            .translate(mid_text.floor())
            .unwrap()
            .writable(),);
        assert!(!(*kernel_space.page_table.get())
            .translate(mid_rodata.floor())
            .unwrap()
            .writable(),);
        assert!(!(*kernel_space.page_table.get())
            .translate(mid_data.floor())
            .unwrap()
            .executable(),);
    }
    info!("remap_test passed!");
}

/// Handle different kinds of page fault
pub async fn handle_page_fault(va: VirtAddr, scause: usize) -> GeneralRet<()> {
    stack_trace!();
    if let Some(handler) = current_process().inner_handler(|proc| {
        let (handler, vma) = proc.memory_space.page_fault_handler(va, scause)?;
        if !handler.handle_page_fault(va, &proc.memory_space, vma)? {
            Ok(Some(handler))
        } else {
            Ok(None)
        }
    })? {
        debug!("handle pagefault asynchronously, va: {:#x}", va.0);
        handler.handle_page_fault_async(va, current_process()).await
    } else {
        Ok(())
    }
}
