use core::any::Any;

use alloc::{format, string::String, sync::Arc};
use log::info;
use smoltcp::{
    phy::{self, DeviceCapabilities},
    time::Instant,
    wire::{EthernetAddress, Ipv4Address},
};
use virtio_drivers::{
    device::net::{RxBuffer, VirtIONet},
    transport::mmio::{MmioTransport, VirtIOHeader},
    Error,
};

use crate::{sync::mutex::SpinLock, utils::error::GeneralRet};

use super::{VirtioHal, VIRTIO8};

type Mutex<T> = SpinLock<T>;

const QUEUE_SIZE: usize = 1 << 10;
const BUF_LEN: usize = 1 << 12;

type NetDevice = VirtIONet<VirtioHal, MmioTransport, QUEUE_SIZE>;
#[derive(Clone)]
pub struct VirtIONetDevice(Arc<Mutex<NetDevice>>);

unsafe impl Send for VirtIONetDevice {}
unsafe impl Sync for VirtIONetDevice {}

impl smoltcp::phy::Device for VirtIONetDevice {
    type RxToken<'a> = VirtioRxToken where Self: 'a;
    type TxToken<'a> = VirtioTxToken where Self: 'a;

    fn receive(&mut self, _ts: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        match self.0.lock().receive() {
            Ok(buf) => Some((
                VirtioRxToken(self.0.clone(), buf),
                VirtioTxToken(self.0.clone()),
            )),
            Err(Error::NotReady) => None,
            Err(err) => panic!("receive failed: {}", err),
        }
    }

    fn transmit(&mut self, _ts: Instant) -> Option<Self::TxToken<'_>> {
        Some(VirtioTxToken(self.0.clone()))
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1536;
        caps.max_burst_size = Some(1);
        caps
    }
}

pub struct VirtioRxToken(Arc<Mutex<NetDevice>>, RxBuffer);
pub struct VirtioTxToken(Arc<Mutex<NetDevice>>);

impl phy::RxToken for VirtioRxToken {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        // let mut buffer = [0u8; 2000];
        // let mut driver = self.0.lock();
        // let len = driver.recv(&mut buffer).expect("failed to recv packet");
        let mut rx_buf = self.1;
        let ret = f(rx_buf.packet_mut());
        self.0.lock().recycle_rx_buffer(rx_buf).unwrap();
        ret
    }
}
impl phy::TxToken for VirtioTxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut dev = self.0.lock();
        let mut tx_buf = dev.new_tx_buffer(len);
        let ret = f(tx_buf.packet_mut());
        dev.send(tx_buf).expect("failed to send packet");
        ret
    }
}

impl VirtIONetDevice {
    pub fn new() -> Self {
        // todo!()
        let ret = unsafe {
            let vaddr = VIRTIO8;
            let header = &mut *(vaddr as *mut VirtIOHeader);
            let net = NetDevice::new(MmioTransport::new(header.into()).unwrap(), BUF_LEN)
                .expect("failed to create net driver");
            log::info!("VirtIONetDevice net header init");
            Self(Arc::new(Mutex::new(net)))
        };
        ret
    }
}
