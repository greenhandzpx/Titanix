use core::str::FromStr;

use crate::{driver::NET_DEVICE, sync::mutex::SpinNoIrqLock, timer::current_time_duration};
use alloc::vec;
use smoltcp::{
    iface::{Config, Interface, SocketHandle, SocketSet},
    phy::{Device, Loopback, Medium},
    socket::{tcp, udp, AnySocket},
    time::Instant,
    wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address},
};

type Mutex<T> = SpinNoIrqLock<T>;

pub static NET_INTERFACE: TitanixNetInterface = TitanixNetInterface::new();
const IP: &str = "10.0.2.15"; // QEMU user networking default IP
const GATEWAY: &str = "10.0.2.2"; // QEMU user networking gateway

pub fn init() {
    NET_INTERFACE.init();
}

pub struct TitanixNetInterface<'a> {
    inner: Mutex<Option<TitanixNetInterfaceInner<'a>>>,
}

pub struct TitanixNetInterfaceInner<'a> {
    // pub device: Loopback,
    pub iface: Interface,
    pub sockets: SocketSet<'a>,
}

impl<'a> TitanixNetInterfaceInner<'a> {
    fn new() -> Self {
        // let mut device = Loopback::new(Medium::Ethernet);
        let mut device_lock = NET_DEVICE.lock();
        let device = device_lock.as_mut().unwrap();
        let iface = {
            let config = match device.capabilities().medium {
                Medium::Ethernet => {
                    Config::new(EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]).into())
                }
                Medium::Ip => Config::new(smoltcp::wire::HardwareAddress::Ip),
            };

            let mut iface = Interface::new(
                config,
                device,
                Instant::from_millis(current_time_duration().as_millis() as i64),
            );

            iface.update_ip_addrs(|ip_addrs| {
                ip_addrs
                    .push(IpCidr::new(IpAddress::from_str(IP).unwrap(), 24))
                    .unwrap();
            });

            iface
                .routes_mut()
                .add_default_ipv4_route(Ipv4Address::from_str(GATEWAY).unwrap())
                .unwrap();

            iface
        };
        Self {
            // device,
            iface,
            sockets: SocketSet::new(vec![]),
        }
    }
}

impl<'a> TitanixNetInterface<'a> {
    pub fn init(&self) {
        *self.inner.lock() = Some(TitanixNetInterfaceInner::new());
    }
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }
    pub fn add_socket<T>(&self, socket: T) -> SocketHandle
    where
        T: AnySocket<'a>,
    {
        self.inner.lock().as_mut().unwrap().sockets.add(socket)
    }

    pub fn tcp_socket<T>(&self, handler: SocketHandle, f: impl FnOnce(&mut tcp::Socket) -> T) -> T {
        f(self
            .inner
            .lock()
            .as_mut()
            .unwrap()
            .sockets
            .get_mut::<tcp::Socket>(handler))
    }

    pub fn udp_socket<T>(&self, handler: SocketHandle, f: impl FnOnce(&mut udp::Socket) -> T) -> T {
        f(self
            .inner
            .lock()
            .as_mut()
            .unwrap()
            .sockets
            .get_mut::<udp::Socket>(handler))
    }

    pub fn inner_handler<T>(&self, f: impl FnOnce(&mut TitanixNetInterfaceInner<'a>) -> T) -> T {
        f(&mut self.inner.lock().as_mut().unwrap())
    }

    pub fn poll(&self) {
        log::debug!("[TitanixNetInterface::poll] poll...");
        self.inner_handler(|inner| {
            inner.iface.poll(
                Instant::from_millis(current_time_duration().as_millis() as i64),
                // &mut inner.device,
                NET_DEVICE.lock().as_mut().unwrap(),
                &mut inner.sockets,
            );
        });
    }
    pub fn remove(&self, handler: SocketHandle) {
        self.inner_handler(|inner| {
            inner.sockets.remove(handler);
        });
    }
}
