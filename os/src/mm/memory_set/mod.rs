use core::{arch::asm, cell::SyncUnsafeCell};

use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec, vec::Vec};
use log::{debug, error, info, trace, warn};
use riscv::register::{satp, scause::Scause};

use crate::{
    config::{
        board::MEMORY_END,
        board::MMIO,
        mm::PAGE_SIZE,
        mm::{MMAP_TOP, USER_STACK_SIZE},
    },
    driver::block::MMIO_VIRT,
    mm::memory_set::page_fault_handler::SBrkPageFaultHandler,
    process::aux::*,
    stack_trace,
    utils::error::{GeneralRet, SyscallErr},
};

pub use self::{
    page_fault_handler::{ForkPageFaultHandler, PageFaultHandler, UStackPageFaultHandler},
    vm_area::VmArea,
};

use super::{
    address::SimpleRange, page_table::PTEFlags, PageTable, PageTableEntry, PhysAddr, VirtAddr,
    VirtPageNum,
};

///
pub mod page_fault_handler;
///
pub mod vm_area;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    // fn sbss_with_stack();
    fn sstack();
    fn estack();
    fn sbss();
    fn ebss();
    fn ekernel();
}

// lazy_static! {
// }

/// Kernel Space for all processes
pub static mut KERNEL_SPACE: Option<MemorySet> = None;

///
pub fn init_kernel_space() {
    info!("start to init kernel space...");
    unsafe {
        KERNEL_SPACE = Some(MemorySet::new_kernel());
    }
}
// lazy_static! {
//     /// a memory set instance through lazy_static! managing kernel space
//     pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> =
//         Arc::new(unsafe { UPSafeCell::new(MemorySet::new_kernel()) });
// }

/// Heap range
pub type HeapRange = SimpleRange<VirtAddr>;

/// memory set structure, controls virtual-memory space
pub struct MemorySet {
    /// we should ensure modifying page table exclusively(e.g. through process_inner's lock)
    /// TODO: optimization: decrease the lock granularity when handling page fault
    pub page_table: Arc<SyncUnsafeCell<PageTable>>,
    /// start vpn -> vm_area
    areas: BTreeMap<VirtPageNum, VmArea>,
    /// heap range
    pub heap_range: Option<HeapRange>,
}

impl MemorySet {
    ///Create an empty `MemorySet`
    pub fn new_bare() -> Self {
        Self {
            page_table: Arc::new(SyncUnsafeCell::new(PageTable::new())),
            areas: BTreeMap::new(),
            heap_range: None,
        }
    }

    ///Create an empty `MemorySet` but owns the global kernel mapping
    pub fn new_from_global() -> Self {
        let kernel_space = unsafe { KERNEL_SPACE.as_ref().unwrap() };
        // TODO: optimize:
        // Now we copy all the kernel space's ptes one by one
        // but actually we can only copy the root ppn's corresponding ptes(i.e. level 1)
        // which may be faster (see `PageTable::from_global()`)
        let mut new_page_table = PageTable::new();
        for (_, area) in kernel_space.areas.iter() {
            let pte_flags = PTEFlags::from_bits(area.map_perm.bits()).unwrap();
            for vpn in area.vpn_range {
                let ppn = kernel_space.translate(vpn).unwrap().ppn();
                new_page_table.map(vpn, ppn, pte_flags);
            }
        }
        Self {
            page_table: Arc::new(SyncUnsafeCell::new(new_page_table)),
            areas: BTreeMap::new(),
            heap_range: None,
        }
        // Self {
        //     page_table: Arc::new(SyncUnsafeCell::new(PageTable::from_global())),
        //     areas: BTreeMap::new(),
        // }
    }

    /// Get pagetable `root_ppn`
    pub fn token(&self) -> usize {
        unsafe { (*self.page_table.get()).token() }
    }

    /// Find the immutable ref of map area by the given vpn
    pub fn find_vm_area_by_vpn(&self, vpn: VirtPageNum) -> Option<&VmArea> {
        // Range query to find the map area that this vpn belongs to
        // debug!("len before {}", self.areas.len());
        if let Some((_, vm_area)) = self.areas.range(..=vpn).next_back() {
            if vm_area.end_vpn() <= vpn {
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
        if let Some((_, vm_area)) = self.areas.range_mut(..=vpn).next_back() {
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

    /// Handle page fault
    pub fn handle_page_fault(&mut self, va: VirtAddr, scause: usize) -> GeneralRet<()> {
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
        // Range query to find the map area that this vpn belongs to
        if let Some(vm_area) = self.find_vm_area_by_vpn(vpn) {
            // vm_area.handle_page_fault(va, page_table)
            let page_table = unsafe { &mut (*self.page_table.get()) };
            vm_area.handle_page_fault(va, page_table)
        } else {
            warn!("memory set len {}", self.areas.len());
            for area in self.areas.iter() {
                warn!(
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
    pub fn insert_area(&mut self, mut vma: VmArea) {
        self.push_lazily(vma, None);
    }

    /// Assume that no conflicts.
    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push(
            VmArea::new(start_va, end_va, MapType::Framed, permission, None, None),
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
        handler: Option<Box<dyn PageFaultHandler>>,
    ) {
        self.push_lazily(
            VmArea::new(start_va, end_va, MapType::Framed, permission, handler, None),
            None,
        );
    }

    ///Remove `VmArea` that starts with `start_vpn`
    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some(area) = self.areas.get_mut(&start_vpn) {
            let pgtbl_ref = unsafe { &mut (*self.page_table.get()) };
            // area.unmap(pgtbl_ref);
            area.unmap_lazily(pgtbl_ref);
            self.areas.remove(&start_vpn);
        }
        // if let Some((idx, area)) = self
        //     .areas
        //     .iter_mut()
        //     .enumerate()
        //     .find(|(_, area)| area.vpn_range.get_start() == start_vpn)
        // {
        //     let pgtbl_ref = unsafe { &mut (*self.page_table.get()) };
        //     area.unmap(pgtbl_ref);
        //     self.areas.remove(idx);
        // }
    }
    /// Add the map area to memory set and map the map area(allocating physical frames)
    fn push(&mut self, mut vm_area: VmArea, data_offset: usize, data: Option<&[u8]>) {
        stack_trace!();
        let pgtbl_ref = unsafe { &mut (*self.page_table.get()) };
        vm_area.map(pgtbl_ref);
        if let Some(data) = data {
            vm_area.copy_data_with_offset(pgtbl_ref, data_offset, data);
        }
        self.areas.insert(vm_area.start_vpn(), vm_area);
    }
    /// Only add the map area to memory set (without allocating physical frames)
    fn push_lazily(&mut self, vm_area: VmArea, _: Option<&[u8]>) {
        // debug!("push lazily");
        self.areas.insert(vm_area.start_vpn(), vm_area);
        // self.areas.push(vm_area);
    }
    /// Without kernel stacks.
    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new_bare();
        // // map trampoline
        // memory_set.map_trampoline();
        // map kernel sections
        info!(
            "[kernel].text [{:#x}, {:#x})",
            stext as usize, etext as usize
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
        memory_set.push(
            VmArea::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
                None,
                None,
            ),
            0,
            None,
        );
        info!("[kernel]mapping .rodata section");
        memory_set.push(
            VmArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
                None,
                None,
            ),
            0,
            None,
        );
        info!("[kernel]mapping .data section");
        memory_set.push(
            VmArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
                None,
                None,
            ),
            0,
            None,
        );
        // add stack section in `linker.ld`
        info!("[kernel]mapping .stack section");
        memory_set.push(
            VmArea::new(
                (sstack as usize).into(),
                (estack as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
                None,
                None,
            ),
            0,
            None,
        );
        info!("[kernel]mapping .bss section");
        memory_set.push(
            VmArea::new(
                (sbss as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
                None,
                None,
            ),
            0,
            None,
        );
        info!("[kernel]mapping physical memory");
        memory_set.push(
            VmArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
                None,
                None,
            ),
            0,
            None,
        );
        info!("[kernel]mapping memory-mapped registers");
        for pair in MMIO_VIRT {
            // println!("start va: {:#x}", (*pair).0);
            // println!("end va: {:#x}", (*pair).0 + (*pair).1);
            memory_set.push(
                VmArea::new(
                    (*pair).0.into(),
                    ((*pair).0 + (*pair).1).into(),
                    MapType::Direct,
                    MapPermission::R | MapPermission::W,
                    None,
                    None,
                ),
                0,
                None,
            );
        }
        memory_set
    }
    /// Include sections in elf and trampoline and TrapContext and user stack,
    /// also returns user_sp and entry point.
    /// TODO: resolve elf file lazily
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize, Vec<AuxHeader>) {
        // let mut memory_set = Self::new_bare();
        let mut memory_set = Self::new_from_global();
        // // map trampoline
        // memory_set.map_trampoline();

        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = elf_header.pt2.ph_count();

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
        auxv.push(AuxHeader {
            aux_type: AT_BASE,
            value: 0 as usize,
        });

        // _at_base = memory_set.load_dl(&elf);

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

        let mut max_end_vpn = VirtPageNum(0);
        let mut head_va = 0;
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
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
                let vm_area = VmArea::new(start_va, end_va, MapType::Framed, map_perm, None, None);
                max_end_vpn = vm_area.vpn_range.end();

                // let seg_size = (end_va.ceil().0 - start_va.floor().0) * PAGE_SIZE;
                // let mut raw_data = vec![0 as u8; seg_size];
                // let _ = &elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize].iter().enumerate().for_each(|(i, byte)| {
                //     raw_data[i + start_va.0 - start_va.floor().0 * PAGE_SIZE] = *byte;
                // });
                // memory_set.push(
                //     vm_area,
                //     Some(&raw_data),
                // );
                let offset = start_va.0 - start_va.floor().0 * PAGE_SIZE;
                memory_set.push(
                    vm_area,
                    offset,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
                debug!(
                    "from elf: {:#x}, {:#x}, map_perm: {:#x}",
                    start_va.0,
                    end_va.0,
                    map_perm.bits()
                );
                // let magic = 0x1213d0;
                // if start_va.0 < magic && magic < end_va.0 {
                //     trace!("{:#x}: raw value: {:#x}", magic, )
                // }
            }
        }

        let ph_head_addr = head_va + elf.header.pt2.ph_offset() as usize;
        debug!("from_elf: AT_PHDR  ph_head_addr is {:X} ", ph_head_addr);
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
        // guard page
        let heap_start_va = user_stack_top + PAGE_SIZE;
        let map_perm = MapPermission::U | MapPermission::R | MapPermission::W | MapPermission::X;
        let heap_vma = VmArea::new(
            heap_start_va.into(),
            heap_start_va.into(),
            MapType::Framed,
            map_perm,
            Some(Box::new(SBrkPageFaultHandler {})),
            None,
        );
        memory_set.push(heap_vma, 0, None);
        memory_set.heap_range = Some(HeapRange::new(heap_start_va.into(), heap_start_va.into()));
        debug!(
            "from elf: map heap: {:#x}, {:#x}",
            heap_start_va, heap_start_va
        );

        // let user_stack_top = user_stack_bottom + USER_STACK_SIZE;
        // memory_set.push(
        //     VmArea::new(
        //         user_stack_bottom.into(),
        //         user_stack_top.into(),
        //         MapType::Framed,
        //         MapPermission::R | MapPermission::W | MapPermission::U,
        //     ),
        //     None,
        // );
        // // map TrapContext
        // memory_set.push(
        //     VmArea::new(
        //         TRAP_CONTEXT.into(),
        //         TRAMPOLINE.into(),
        //         MapType::Framed,
        //         MapPermission::R | MapPermission::W,
        //     ),
        //     None,
        // );
        (
            memory_set,
            user_stack_bottom,
            elf.header.pt2.entry_point() as usize,
            auxv,
        )
    }
    ///Clone a same `MemorySet`
    pub fn from_existed_user(user_space: &Self) -> Self {
        // let mut memory_set = Self::new_bare();
        let mut memory_set = Self::new_from_global();
        let new_pagetable = unsafe { &mut (*memory_set.page_table.get()) };
        // // map trampoline
        // memory_set.map_trampoline();
        // copy data sections/trap_context/user_stack
        for (_, area) in user_space.areas.iter() {
            let mut new_area = VmArea::from_another(area);
            // memory_set.push(new_area, None);
            // copy data from another space
            for vpn in area.vpn_range {
                if let Some(ppn) = user_space.translate(vpn) {
                    let src_ppn = ppn.ppn();
                    let dst_ppn = new_area.map_one(new_pagetable, vpn);
                    dst_ppn.bytes_array().copy_from_slice(src_ppn.bytes_array());
                }
                // let src_ppn = user_space.translate(vpn).unwrap().ppn();
                // let dst_ppn = memory_set.translate(vpn).unwrap().ppn();
                // dst_ppn
                //     .get_bytes_array()
                //     .copy_from_slice(src_ppn.get_bytes_array());
            }
            memory_set.push_lazily(new_area, None);
        }
        memory_set
    }
    ///Clone a same `MemorySet`
    pub fn from_existed_user_lazily(user_space: &mut Self) -> Self {
        // let mut memory_set = Self::new_bare();
        let mut memory_set = Self::new_from_global();
        // SAFETY: the process inner has been locked when invoking this function
        let new_pagetable = unsafe { &mut (*memory_set.page_table.get()) };

        for (_, area) in user_space.areas.iter_mut() {
            // clear write bit && add cow bit
            if area.map_perm.contains(MapPermission::W)
                || area.map_perm.contains(MapPermission::COW)
            {
                area.map_perm |= MapPermission::COW;
                area.map_perm.remove(MapPermission::W);
                area.handler = Some(Box::new(ForkPageFaultHandler {}));
            }
        }

        // copy data sections/trap_context/user_stack
        for (_, area) in user_space.areas.iter() {
            let mut new_area = VmArea::from_another(area);
            // memory_set.push(new_area, None);
            // copy data from another space
            for vpn in area.vpn_range {
                // SAFETY: we've locked the process inner before calling this function
                if let Some(ph_frame) = unsafe { (*area.data_frames.get()).0.get(&vpn) } {
                    // If there is related physcial frame, then we let the child and father share it.
                    let pte = unsafe { (*user_space.page_table.get()).find_pte(vpn).unwrap() };

                    if pte.flags().contains(PTEFlags::W) || pte.flags().contains(PTEFlags::COW) {
                        let mut new_flags = pte.flags() | PTEFlags::COW;
                        new_flags.remove(PTEFlags::W);
                        pte.set_flags(new_flags);
                        assert!(pte.flags().contains(PTEFlags::COW));
                        assert!(!pte.flags().contains(PTEFlags::W));
                    }
                    // new_area
                    //     .data_frames
                    //     .get_mut()
                    //     .0
                    //     .insert(vpn, ph_frame.clone());
                    let _dst_ppn = new_area.map_one_lazily(new_pagetable, vpn, ph_frame);
                    // dst_ppn
                    //     .get_bytes_array()
                    //     .copy_from_slice(src_ppn.get_bytes_array());
                } else {
                    debug!("no ppn for vpn {:#x}", vpn.0);
                }
            }
            memory_set.push_lazily(new_area, None);
        }
        user_space.activate();
        new_pagetable.activate();
        memory_set
    }
    ///Refresh TLB with `sfence.vma`
    pub fn activate(&self) {
        unsafe {
            let page_table  = &mut (*self.page_table.get());
            page_table.activate();
            // let satp = (*self.page_table.get()).token();
            // // println!("satp {:#x}", satp);
            // satp::write(satp);
            // asm!("sfence.vma");
        }
    }
    ///Translate throuth pagetable
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        unsafe { (*self.page_table.get()).translate(vpn) }
    }
    ///Remove all `VmArea`
    pub fn recycle_data_pages(&mut self) {
        //*self = Self::new_bare();
        self.areas.clear();
    }

    /// Allocate an unused area(mostly for mmap)
    /// Note that length is counted by byte
    pub fn find_unused_area(&self, length: usize, map_permission: MapPermission) -> Option<VmArea> {
        if length == 0 {
            return None;
        }
        let mut last_start = MMAP_TOP;
        // traverse reversely
        let length_rounded = (length - 1 + PAGE_SIZE) / PAGE_SIZE * PAGE_SIZE;
        for vma in self.areas.iter().rev() {
            // debug!("start {:#x}, end {:#x}", vma.1.start_vpn().0, vma.1.end_vpn().0);
            let curr_end = vma.1.end_vpn().0 * PAGE_SIZE;
            if last_start - curr_end >= length_rounded {
                let new_start = last_start - length_rounded;
                debug!("find an unused area: [{:#x}, {:#x}]", new_start, last_start);
                return Some(VmArea::new(
                    new_start.into(),
                    last_start.into(),
                    MapType::Framed,
                    map_permission,
                    None,
                    None,
                ));
            }
            last_start = vma.1.start_vpn().0 * PAGE_SIZE;
        }
        error!("Cannot find any unused vm area!!");
        None
    }

    /// Check whether the given vpn range conflicts with other vma.
    /// Note that the start_vpn must have been in memory set.
    pub fn check_vpn_range_conflict(&self, start_vpn: VirtPageNum, end_vpn: VirtPageNum) -> bool {
        for vma in self.areas.iter() {
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
        /// COW when fork
        const COW = 1 << 8;
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
    let mid_rodata: VirtAddr = (srodata as usize + (erodata as usize - srodata as usize ) / 2).into();
    let mid_data: VirtAddr = (sdata as usize + (edata as usize - sdata as usize) / 2).into();
    debug!("mid text {:#x}, mid rodata {:#x}, mid data {:#x}", mid_text.0, mid_rodata.0, mid_data.0);
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
