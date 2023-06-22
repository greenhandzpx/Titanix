use core::mem::size_of;

use crate::utils::string::str_to_array_65;

// const SYSNAME: &str = "Titanix";
// const NODENAME: &str = "Titanix";
// const RELEASE: &str = "Titanix 1.0.0";
// const VERSION: &str = "1.0.0";
const SYSNAME: &str = "Linux";
const NODENAME: &str = "Linux";
const RELEASE: &str = "5.19.0-42-generic";
// const VERSION: &str = "6.4.0";
const VERSION: &str = "#43~22.04.1-Ubuntu SMP PREEMPT_DYNAMIC Fri Apr 21 16:51:08 UTC 2";
const MACHINE: &str = "RISC-V SiFive Freedom U740 SoC";
const DOMAINNAME: &str = "titanix.org";

pub const UTSNAME_SIZE: usize = size_of::<UtsName>() as usize;
pub struct UtsName {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}

impl UtsName {
    pub fn get_utsname() -> Self {
        UtsName {
            sysname: str_to_array_65(SYSNAME),
            nodename: str_to_array_65(NODENAME),
            release: str_to_array_65(RELEASE),
            version: str_to_array_65(VERSION),
            machine: str_to_array_65(MACHINE),
            domainname: str_to_array_65(DOMAINNAME),
        }
    }
}
