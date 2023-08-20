use alloc::{
    collections::BTreeMap,
    sync::{Arc, Weak},
    vec::Vec,
};
use log::trace;

use crate::{
    config::mm::PAGE_SIZE_BITS, fs::Inode, mm::MapPermission, stack_trace,
    sync::mutex::SpinNoIrqLock, utils::error::GeneralRet,
};

use crate::mm::{Page, PageBuilder};

/// i.e. linux's `address_space`
/// TODO: add lru policy?
pub struct PageCache {
    inode: Option<Weak<dyn Inode>>,
    /// TODO: compare the performance between Radix tree and B tree map.
    // pages: Mutex<RadixTree<Arc<Page>>>,
    /// Page number -> Page
    pages: SpinNoIrqLock<BTreeMap<usize, Arc<Page>>>,
}

impl PageCache {
    /// Create a new page cache
    pub fn new(inode: Arc<dyn Inode>, _level_num: usize) -> Self {
        stack_trace!();
        Self {
            inode: Some(Arc::downgrade(&inode)),
            pages: SpinNoIrqLock::new(BTreeMap::new()),
        }
    }
    /// Lookup a page according to the given file offset
    pub fn lookup(&self, offset: usize) -> Option<Arc<Page>> {
        stack_trace!();
        self.pages.lock().get(&(offset >> PAGE_SIZE_BITS)).cloned()
    }
    /// Insert a new page
    #[allow(unused)]
    pub fn insert(&self, offset: usize, page: Page) {
        stack_trace!();
        debug_assert!(self
            .pages
            .lock()
            .insert(offset >> PAGE_SIZE_BITS, Arc::new(page))
            .is_none())
    }
    /// Get a page according to the given file offset
    pub fn get_page(
        &self,
        offset: usize,
        map_perm: Option<MapPermission>,
    ) -> GeneralRet<Arc<Page>> {
        stack_trace!();
        trace!("[PageCache]: get page at file offset {:#x}", offset);
        if let Some(page) = self.lookup(offset) {
            Ok(page)
        } else {
            // TODO add evict policy
            let page = Arc::new(
                PageBuilder::new()
                    .is_file_page()
                    .permission(match map_perm {
                        None => MapPermission::from_bits(0).unwrap(),
                        Some(map_perm) => map_perm,
                    })
                    .offset(offset)
                    .inode(self.inode.clone().unwrap())
                    .build(),
            );
            self.pages
                .lock()
                .insert(offset >> PAGE_SIZE_BITS, page.clone());
            Ok(page)
        }
    }
    /// Flush all pages to disk if needed
    pub async fn sync(&self) -> GeneralRet<()> {
        stack_trace!();
        let mut page_set: Vec<Arc<Page>> = Vec::new();
        for (_, page) in self.pages.lock().iter() {
            page_set.push(page.clone());
        }
        for page in page_set {
            page.sync().await?;
        }
        Ok(())
    }
}

// /// Page cache test
// pub fn page_cache_test() {
//     // info!("page_cache_test start...");
//     // let mut page_cache = PageCache {
//     //     inode: None,
//     //     pages: RadixTree::new(3),
//     // };
//     // page_cache.insert(0x123 << PAGE_SIZE_BITS, Page::new(None));
//     // page_cache.insert(0x124 << PAGE_SIZE_BITS, Page::new(None));
//     // page_cache.insert(0x125 << PAGE_SIZE_BITS, Page::new(None));
//     // assert!(page_cache.lookup(0x123 << PAGE_SIZE_BITS).is_some());
//     // assert!(page_cache.lookup(0x124 << PAGE_SIZE_BITS).is_some());
//     // assert!(page_cache.lookup(0x125 << PAGE_SIZE_BITS).is_some());
//     // page_cache.insert(0x1123 << PAGE_SIZE_BITS, Page::new(None));
//     // assert!(page_cache.lookup(0x123 << PAGE_SIZE_BITS).is_none());
//     // assert!(page_cache.lookup(0x1123 << PAGE_SIZE_BITS).is_some());
//     // info!("page_cache_test passed!");
// }
