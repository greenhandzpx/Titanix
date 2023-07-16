use alloc::sync::Arc;

use crate::{
    config::mm::PAGE_SIZE,
    mm::{
        memory_space::{vm_area::VmAreaType, VmArea},
        KernelAddr, MapPermission, Page, PageBuilder, PhysAddr, VirtAddr,
    },
    process::Process,
    processor::SumGuard,
    trap::UserContext,
};

use super::SigSet;
#[repr(C)]
#[derive(Clone, Copy)]
struct SignalStack {
    sp: usize,
    flags: i32,
    size: usize,
}

impl SignalStack {
    pub fn new() -> Self {
        Self {
            sp: 0,
            flags: 0,
            size: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SigSetDummy {
    pub dummy: [usize; 15],
}

#[repr(C, align(16))]
#[derive(Clone, Copy)]
struct Align16;

#[repr(C)]
#[derive(Clone)]
pub struct SignalContext {
    flags: usize,
    link_ptr: usize,
    stack: SignalStack,
    pub blocked_sigs: SigSet,
    pub blocked_sigs_dummy: SigSetDummy,
    align16: Align16,
    pub user_context: UserContext,
}

impl SignalContext {
    pub fn new(blocked_sigs: SigSet, user_context: UserContext) -> Self {
        Self {
            flags: 0,
            link_ptr: 0,
            stack: SignalStack::new(),
            blocked_sigs,
            blocked_sigs_dummy: SigSetDummy { dummy: [0; 15] },
            align16: Align16,
            user_context,
        }
    }
}

pub struct SignalTrampoline {
    page: Arc<Page>,
    user_addr: VirtAddr,
}

impl SignalTrampoline {
    pub fn new(process: Arc<Process>) -> Self {
        let page = Arc::new(
            PageBuilder::new()
                .permission(MapPermission::R | MapPermission::W | MapPermission::U)
                .build(),
        );
        process.inner_handler(|proc| {
            let trampoline_vma = proc
                .memory_space
                .allocate_area(PAGE_SIZE, page.permission, VmAreaType::Mmap)
                .unwrap();
            let user_addr: VirtAddr = trampoline_vma.start_vpn().into();
            let page_table = trampoline_vma.page_table.get_unchecked_mut();
            page_table.map(
                user_addr.floor(),
                page.data_frame.ppn.into(),
                page.permission.into(),
            );
            proc.memory_space.insert_area(trampoline_vma);
            log::debug!(
                "[SignalTrampoline::new] map sig trampoline, vpn: {:#x}, ppn: {:#x}, flags: {:?}",
                user_addr.floor().0,
                page.data_frame.ppn.0,
                page.permission
            );
            Self { page, user_addr }
        })
    }

    pub fn kernel_addr(&self) -> usize {
        KernelAddr::from(PhysAddr::from(self.page.data_frame.ppn)).0
    }

    pub fn user_addr(&self) -> usize {
        self.user_addr.0
    }

    pub fn set_signal_context(&self, signal_context: SignalContext) {
        let _sum_guard = SumGuard::new();
        let sig_ctx: &mut SignalContext = self.page.reinterpret_mut();
        *sig_ctx = signal_context;
    }

    pub fn signal_context(&self) -> &SignalContext {
        self.page.reinterpret()
    }
}
