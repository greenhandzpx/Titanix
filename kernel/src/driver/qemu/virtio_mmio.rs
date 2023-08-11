use device_tree::{util::SliceRead, Node};
use virtio_drivers::{DeviceType, VirtIOHeader};

use crate::mm::memory_space::phys_to_virt;

use super::VirtioHal;

#[allow(unused)]
pub fn virtio_probe(node: &Node) {
    let reg = match node.prop_raw("rag") {
        Some(reg) => reg,
        _ => return,
    };
    let paddr = reg.as_slice().read_be_u64(0).unwrap();
    let vaddr = phys_to_virt(paddr as usize);
    let header = unsafe { &mut *(vaddr as *mut VirtIOHeader) };
    if !header.verify() {
        // only support legacy device
        return;
    }
    log::info!(
        "Detected virtio device with vendor id: {:#x}",
        header.vendor_id()
    );
    log::info!("Device tree node {:?}", node);
    // let mut inner = VIRTIODEVICEADDR.0.lock();
    // let addr = inner.as_mut().unwrap();
    // match header.device_type() {
    //     DeviceType::Block => {
    //         addr.insert(header.device_type(), vaddr);
    //     }
    //     DeviceType::Network => {
    //         addr.insert(header.device_type(), vaddr);
    //     }
    //     t => log::warn!("Unrecognized virtio device: {:?}", t),
    // }
}
