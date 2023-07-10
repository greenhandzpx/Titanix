use log::debug;

use crate::utils::error::SyscallRet;

/// domain
const AF_INET: u32 = 2;

/// socket type
const SOCK_DGRAM: u32 = 2;

/// protocol
const IPPROTO_UDP: u32 = 17;

pub fn sys_socket(domain: u32, socket_type: u32, protocol: u32) -> SyscallRet {
    debug!(
        "[sys_socket] domain: {}, type: {}, protocol: {}",
        domain, socket_type, protocol
    );

    todo!()
}
