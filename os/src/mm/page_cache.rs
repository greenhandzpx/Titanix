use alloc::{
    sync::{Arc, Weak},
};
use log::{info, trace};

use crate::{config::mm::PAGE_SIZE_BITS, fs::Inode, utils::error::GeneralRet};

use super::{
    page::{Page, PageBuilder},
    radix_tree::RadixTree,
};

/// i.e. linux's `address_space`
/// TODO: add lru policy?
pub struct PageCache {
    inode: Option<Weak<dyn Inode>>,
    pages: RadixTree<Arc<Page>>,
}

impl PageCache {
    /// Create a new page cache
    pub fn new(inode: Arc<dyn Inode>, level_num: usize) -> Self {
        Self {
            inode: Some(Arc::downgrade(&inode)),
            pages: RadixTree::new(level_num),
        }
    }
    /// Lookup a page according to the given file offset
    pub fn lookup(&self, offset: usize) -> Option<Arc<Page>> {
        self.pages.lookup(offset >> PAGE_SIZE_BITS)
    }
    /// Insert a new page
    pub fn insert(&mut self, offset: usize, page: Page) {
        self.pages.insert(offset >> PAGE_SIZE_BITS, Arc::new(page))
    }
    /// Get a page according to the given file offset
    pub fn get_page(&mut self, offset: usize) -> GeneralRet<Arc<Page>> {
        trace!("[PageCache]: get page at file offset {:#x}", offset);
        if let Some(page) = self.lookup(offset) {
            Ok(page)
        } else {
            // TODO add evict policy
            let page = Arc::new(
                PageBuilder::new()
                    .offset(offset)
                    .inode(self.inode.clone().unwrap())
                    .build(),
            );
            self.pages.insert(offset >> PAGE_SIZE_BITS, page.clone());
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
