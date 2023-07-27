use smoltcp::{
    iface::{Config, Interface},
    phy::{Device, Loopback, Medium},
    time::Instant,
    wire::{EthernetAddress, IpAddress, IpCidr},
};

use crate::timer::current_time_duration;

pub fn iface() -> Interface {
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
    });
    iface
}
