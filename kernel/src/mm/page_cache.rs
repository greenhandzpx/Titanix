use alloc::sync::{Arc, Weak};
use log::trace;

use crate::{
    config::mm::PAGE_SIZE_BITS, fs::Inode, sync::mutex::SpinNoIrqLock, utils::error::GeneralRet,
};

use super::{
    page::{Page, PageBuilder},
    radix_tree::RadixTree,
    MapPermission,
};

type Mutex<T> = SpinNoIrqLock<T>;

/// i.e. linux's `address_space`
/// TODO: add lru policy?
pub struct PageCache {
    inode: Option<Weak<dyn Inode>>,
    pages: Mutex<RadixTree<Arc<Page>>>,
}

impl PageCache {
    /// Create a new page cache
    pub fn new(inode: Arc<dyn Inode>, level_num: usize) -> Self {
        Self {
            inode: Some(Arc::downgrade(&inode)),
            pages: Mutex::new(RadixTree::new(level_num)),
        }
    }
    /// Lookup a page according to the given file offset
    pub fn lookup(&self, offset: usize) -> Option<Arc<Page>> {
        self.pages.lock().lookup(offset >> PAGE_SIZE_BITS)
    }
    /// Insert a new page
    pub fn insert(&self, offset: usize, page: Page) {
        self.pages
            .lock()
            .insert(offset >> PAGE_SIZE_BITS, Arc::new(page))
    }
    /// Get a page according to the given file offset
    pub fn get_page(
        &self,
        offset: usize,
        map_perm: Option<MapPermission>,
    ) -> GeneralRet<Arc<Page>> {
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
    /// Flush all pages to disk
    pub fn sync(&self) {
        todo!()
    }
}

/// Page cache test
pub fn page_cache_test() {
    // info!("page_cache_test start...");
    // let mut page_cache = PageCache {
    //     inode: None,
    //     pages: RadixTree::new(3),
    // };
    // page_cache.insert(0x123 << PAGE_SIZE_BITS, Page::new(None));
    // page_cache.insert(0x124 << PAGE_SIZE_BITS, Page::new(None));
    // page_cache.insert(0x125 << PAGE_SIZE_BITS, Page::new(None));
    // assert!(page_cache.lookup(0x123 << PAGE_SIZE_BITS).is_some());
    // assert!(page_cache.lookup(0x124 << PAGE_SIZE_BITS).is_some());
    // assert!(page_cache.lookup(0x125 << PAGE_SIZE_BITS).is_some());
    // page_cache.insert(0x1123 << PAGE_SIZE_BITS, Page::new(None));
    // assert!(page_cache.lookup(0x123 << PAGE_SIZE_BITS).is_none());
    // assert!(page_cache.lookup(0x1123 << PAGE_SIZE_BITS).is_some());
    // info!("page_cache_test passed!");
}
