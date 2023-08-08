use core::any::Any;

use alloc::{format, string::String, sync::Arc};
use smoltcp::{
    phy::{self, DeviceCapabilities},
    time::Instant,
    wire::{EthernetAddress, Ipv4Address},
};
use virtio_drivers::{DeviceType, VirtIOHeader, VirtIONet};

use crate::{
    driver::NetDevice,
    sync::mutex::{SpinLock, SpinNoIrqLock},
    utils::error::GeneralRet,
};

use super::VirtioHal;

type Mutex<T> = SpinLock<T>;

#[derive(Clone)]
pub struct VirtIONetDriver(Arc<Mutex<VirtIONet<'static, VirtioHal>>>);

impl NetDevice for VirtIONetDriver {
    fn get_mac(&self) -> EthernetAddress {
        EthernetAddress(self.0.lock().mac())
    }

    fn poll(&self) {
        unimplemented!()
    }
}

impl smoltcp::phy::Device for VirtIONetDriver {
    type RxToken<'a> = VirtIONetDriver;
    type TxToken<'a> = VirtIONetDriver;

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

impl phy::RxToken for VirtIONetDriver {
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

impl phy::TxToken for VirtIONetDriver {
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

pub fn init(header: &'static mut VirtIOHeader) {
    let net = VirtIONet::new(header).expect("failed to create net driver");
    let driver = Arc::new(VirtIONetDriver(Arc::new(Mutex::new(net))));
}
