use alloc::vec;
use smoltcp::{
    iface::{Config, Interface, SocketHandle, SocketSet},
    phy::{Device, Loopback, Medium},
    socket::{tcp, udp, Socket},
    time::Instant,
    wire::{EthernetAddress, IpAddress, IpCidr},
};

use crate::{sync::mutex::SpinNoIrqLock, timer::current_time_duration};

type Mutex<T> = SpinNoIrqLock<T>;

pub static NET_INTERFACE: TitanixNetInterface = TitanixNetInterface::new();

pub fn init() {
    NET_INTERFACE.init();
}

pub struct TitanixNetInterface<'a> {
    inner: Mutex<Option<TitanixNetInterfaceInner<'a>>>,
}

struct TitanixNetInterfaceInner<'a> {
    pub iface: Interface,
    pub sockets: SocketSet<'a>,
}

impl TitanixNetInterfaceInner<'static> {
    fn iface() -> Interface {
        let mut device = Loopback::new(Medium::Ethernet);
        let config = match device.capabilities().medium {
            Medium::Ethernet => {
                Config::new(EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]).into())
            }
            Medium::Ip => Config::new(smoltcp::wire::HardwareAddress::Ip),
        };

        let mut iface = Interface::new(
            config,
            &mut device,
            Instant::from_secs(current_time_duration().as_secs() as i64),
        );
        iface.update_ip_addrs(|ip_addrs| {
            ip_addrs
                .push(IpCidr::new(IpAddress::v4(127, 0, 0, 1), 8))
                .unwrap();
            ip_addrs
                .push(IpCidr::new(IpAddress::v6(0, 0, 0, 0, 0, 0, 0, 1), 128))
                .unwrap();
        });
        iface
    }
    fn new() -> Self {
        Self {
            iface: Self::iface(),
            sockets: SocketSet::new(vec![]),
        }
    }
}

impl TitanixNetInterface<'static> {
    pub fn init(&self) {
        *self.inner.lock() = Some(TitanixNetInterfaceInner::new());
    }
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }
    // pub fn add_socket(&self, socket: Socket) -> SocketHandle {
    //     match socket {
    //         Socket::Tcp(socket) => self.inner.lock().as_mut().unwrap().sockets.add(socket),
    //         Socket::Udp(socket) => self.inner.lock().as_mut().unwrap().sockets.add(socket),
    //     }
    // }
    // pub fn get_tcp(&self, handler: SocketHandle) -> &mut tcp::Socket {
    //     self.inner
    //         .lock()
    //         .as_ref()
    //         .unwrap()
    //         .sockets
    //         .get_mut::<tcp::Socket>(handler)
    // }
    // pub fn get_udp(&self, handler: SocketHandle) -> &mut udp::Socket {
    //     self.inner
    //         .lock()
    //         .as_ref()
    //         .unwrap()
    //         .sockets
    //         .get_mut::<udp::Socket>(handler)
    // }

    pub fn inner_handler<T>(
        &self,
        f: impl FnOnce(&mut TitanixNetInterfaceInner<'static>) -> T,
    ) -> T {
        f(&mut self.inner.lock().as_mut().unwrap())
    }
}
