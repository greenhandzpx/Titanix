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
    device: Mutex<Option<TitanixNetDevice>>,
    loopback: Mutex<Option<TitanixLoopback>>,
    sockets_loop: Mutex<Option<SocketSet<'a>>>,
    sockets_dev: Mutex<Option<SocketSet<'a>>>,
}

pub struct TitanixNetDevice {
    pub iface: Interface,
}

pub struct TitanixLoopback {
    pub device: Loopback,
    pub iface: Interface,
}

impl TitanixLoopback {
    fn new() -> Self {
        let mut device = Loopback::new(Medium::Ip);
        let iface = {
            let config = match device.capabilities().medium {
                Medium::Ethernet => {
                    Config::new(EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]).into())
                }
                Medium::Ip => Config::new(smoltcp::wire::HardwareAddress::Ip),
            };

            let mut iface = Interface::new(
                config,
                &mut device,
                Instant::from_millis(current_time_duration().as_millis() as i64),
            );

            iface.update_ip_addrs(|ip_addrs| {
                ip_addrs
                    .push(IpCidr::new(IpAddress::v4(127, 0, 0, 1), 8))
                    .unwrap();
            });

            iface
        };
        Self { device, iface }
    }
}

impl TitanixNetDevice {
    fn new() -> Self {
        let mut device_lock = NET_DEVICE.lock();
        let device = device_lock.as_mut().unwrap();
        let iface = {
            let config = match device.capabilities().medium {
                Medium::Ethernet => {
                    Config::new(EthernetAddress([0x03, 0x00, 0x00, 0x00, 0x00, 0x01]).into())
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
        Self { iface }
    }
}

impl<'a> TitanixNetInterface<'a> {
    pub fn init(&self) {
        *self.device.lock() = Some(TitanixNetDevice::new());
        *self.loopback.lock() = Some(TitanixLoopback::new());
        *self.sockets_loop.lock() = Some(SocketSet::new(vec![]));
        *self.sockets_dev.lock() = Some(SocketSet::new(vec![]));
    }
    pub const fn new() -> Self {
        Self {
            device: Mutex::new(None),
            loopback: Mutex::new(None),
            sockets_loop: Mutex::new(None),
            sockets_dev: Mutex::new(None),
        }
    }
    pub fn add_socket<T>(&self, socket_loop: T, socket_dev: T) -> (SocketHandle, SocketHandle)
    where
        T: AnySocket<'a>,
    {
        let loop_handler = self.sockets_loop.lock().as_mut().unwrap().add(socket_loop);
        let dev_handler = self.sockets_dev.lock().as_mut().unwrap().add(socket_dev);
        (loop_handler, dev_handler)
    }

    pub fn tcp_socket_loop<T>(
        &self,
        handler: SocketHandle,
        f: impl FnOnce(&mut tcp::Socket) -> T,
    ) -> T {
        f(self
            .sockets_loop
            .lock()
            .as_mut()
            .unwrap()
            .get_mut::<tcp::Socket>(handler))
    }

    pub fn tcp_socket_dev<T>(
        &self,
        handler: SocketHandle,
        f: impl FnOnce(&mut tcp::Socket) -> T,
    ) -> T {
        f(self
            .sockets_dev
            .lock()
            .as_mut()
            .unwrap()
            .get_mut::<tcp::Socket>(handler))
    }

    pub fn udp_socket_loop<T>(
        &self,
        handler: SocketHandle,
        f: impl FnOnce(&mut udp::Socket) -> T,
    ) -> T {
        f(self
            .sockets_loop
            .lock()
            .as_mut()
            .unwrap()
            .get_mut::<udp::Socket>(handler))
    }

    pub fn udp_socket_dev<T>(
        &self,
        handler: SocketHandle,
        f: impl FnOnce(&mut udp::Socket) -> T,
    ) -> T {
        f(self
            .sockets_dev
            .lock()
            .as_mut()
            .unwrap()
            .get_mut::<udp::Socket>(handler))
    }

    pub fn loopback<T>(&self, f: impl FnOnce(&mut TitanixLoopback) -> T) -> T {
        f(&mut self.loopback.lock().as_mut().unwrap())
    }

    pub fn device<T>(&self, f: impl FnOnce(&mut TitanixNetDevice) -> T) -> T {
        f(&mut self.device.lock().as_mut().unwrap())
    }

    fn poll_loopback(&self) {
        // log::debug!("[TitanixNetInterface::poll] poll loopback...");
        self.loopback(|inner| {
            inner.iface.poll(
                Instant::from_millis(current_time_duration().as_millis() as i64),
                &mut inner.device,
                &mut self.sockets_loop.lock().as_mut().unwrap(),
            );
        });
    }

    fn poll_device(&self) {
        // log::debug!("[TitanixNetInterface::poll] poll device...");
        self.device(|inner| {
            inner.iface.poll(
                Instant::from_millis(current_time_duration().as_millis() as i64),
                NET_DEVICE.lock().as_mut().unwrap(),
                &mut self.sockets_dev.lock().as_mut().unwrap(),
            );
        });
    }

    pub fn poll(&self, is_local: bool) {
        if is_local {
            self.poll_loopback();
        } else {
            self.poll_device();
        }
    }

    pub fn poll_all(&self) {
        // log::debug!("[TitanixNetInterface::poll] poll all...");
        self.poll_loopback();
        self.poll_device();
    }

    pub fn remove(&self, handler_loop: SocketHandle, handler_dev: SocketHandle) {
        self.sockets_loop
            .lock()
            .as_mut()
            .unwrap()
            .remove(handler_loop);
        self.sockets_dev
            .lock()
            .as_mut()
            .unwrap()
            .remove(handler_dev);
    }
}
