use alloc::sync::Arc;

use crate::{
    mm::{page_table::PTEFlags, PageTable, VirtAddr},
    utils::{cell::SyncUnsafeCell, error::GeneralRet},
};

use super::{CowPageFaultHandler, PageFaultHandler, PageManager, VmArea};

pub struct CowPageManager {
    pub page_mgr: SyncUnsafeCell<PageManager>,
    page_fault_handler: Arc<dyn PageFaultHandler>,
}

impl CowPageManager {
    pub fn new() -> Self {
        Self {
            page_mgr: SyncUnsafeCell::new(PageManager::new()),
            page_fault_handler: CowPageFaultHandler {}.arc_clone(),
        }
    }

    pub fn from_another(another: &Self, page_table: Arc<SyncUnsafeCell<PageTable>>) -> Self {
        // TODO: optimize: only need to map the leaf page
        let page_mgr = SyncUnsafeCell::new(another.page_mgr.get_unchecked_mut().clone());
        for (vpn, page) in another.page_mgr.get_unchecked_mut().0.iter() {
            log::trace!(
                "[CowPageManager::from_another]: map vpn {:#x}, ppn {:#x}, map perm {:?}",
                vpn.0,
                page.data_frame.ppn.0,
                *page.permission.lock()
            );
            let mut pte_flags: PTEFlags = (*page.permission.lock()).into();
            pte_flags |= PTEFlags::COW | PTEFlags::U;
            pte_flags.remove(PTEFlags::W);
            page_table
                .get_unchecked_mut()
                .map(*vpn, page.data_frame.ppn, pte_flags);
        }
        // page_table.get_unchecked_mut().activate();
        Self {
            page_mgr,
            page_fault_handler: another.page_fault_handler.arc_clone(),
        }
    }

    pub fn page_fault_handler(
        &self,
        _va: VirtAddr,
    ) -> GeneralRet<(Arc<dyn PageFaultHandler>, Option<&VmArea>)> {
        Ok((self.page_fault_handler.clone(), None))
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.page_mgr.get_mut().0.clear()
    }
}
