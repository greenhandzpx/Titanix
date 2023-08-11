use alloc::{collections::BTreeMap, string::String, sync::Arc};
use core::slice;
use device_tree::{DeviceTree, Node};

use crate::sync::mutex::SpinNoIrqLock;

type Mutex<T> = SpinNoIrqLock<T>;

const DEVICE_TREE_MAGIC: u32 = 0xd00dfeed;

/// Compatible lookup
pub static DEVICE_TREE_REGISTRY: Mutex<BTreeMap<&'static str, fn(&Node)>> =
    Mutex::new(BTreeMap::new());

fn walk_dt_node(dt: &Node, level: usize) {
    if let Ok(compatible) = dt.prop_str("compatible") {
        let registry = DEVICE_TREE_REGISTRY.lock();
        if let Some(f) = registry.get(compatible) {
            f(dt);
        }
    }
    for child in dt.children.iter() {
        log::debug!("{} node name {}", level, child.name);

        walk_dt_node(child, level + 1);
    }
}

struct DtbHeader {
    magic: u32,
    size: u32,
}

pub fn init(dtb: usize) {
    let header = unsafe { &*(dtb as *const DtbHeader) };
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        let dtb_data = unsafe { slice::from_raw_parts(dtb as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(dtb_data) {
            // find interrupt controller first
            walk_dt_node(&dt.root, 0);
        }
    }
}
