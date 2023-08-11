use core::any::Any;

use alloc::{format, string::String, sync::Arc};
use log::info;
use smoltcp::{
    phy::{self, DeviceCapabilities},
    time::Instant,
    wire::{EthernetAddress, Ipv4Address},
};
use virtio_drivers::{DeviceType, VirtIOHeader, VirtIONet};

use crate::{sync::mutex::SpinLock, utils::error::GeneralRet};

use super::{VirtioHal, VIRTIO8};

type Mutex<T> = SpinLock<T>;

#[derive(Clone)]
pub struct VirtIONetDevice(Arc<Mutex<VirtIONet<'static, VirtioHal>>>);

impl smoltcp::phy::Device for VirtIONetDevice {
    type RxToken<'a> = VirtIONetDevice;
    type TxToken<'a> = VirtIONetDevice;

    fn receive(&mut self, _ts: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let net = self.0.lock();
        if net.can_recv() {
            Some((self.clone(), self.clone()))
        } else {
            None
        }
    }

    fn transmit(&mut self, _ts: Instant) -> Option<Self::TxToken<'_>> {
        let net = self.0.lock();
        if net.can_send() {
            Some(self.clone())
        } else {
            None
        }
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1536;
        caps.max_burst_size = Some(1);
        caps
    }
}

impl phy::RxToken for VirtIONetDevice {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = [0u8; 2000];
        let mut driver = self.0.lock();
        let len = driver.recv(&mut buffer).expect("failed to recv packet");
        f(&mut buffer[..len])
    }
}

impl phy::TxToken for VirtIONetDevice {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = [0u8; 2000];
        let result = f(&mut buffer[..len]);
        let mut driver = self.0.lock();
        driver.send(&buffer).expect("failed to send packet");
        result
    }
}

impl VirtIONetDevice {
    pub fn new() -> Self {
        // todo!()
        let ret = unsafe {
            let vaddr = VIRTIO8;
            let header = &mut *(vaddr as *mut VirtIOHeader);
            let net = VirtIONet::<VirtioHal>::new(header).expect("failed to create net driver");
            log::info!("VirtIONetDevice net header init");
            Self(Arc::new(Mutex::new(net)))
        };
        ret
    }
}
