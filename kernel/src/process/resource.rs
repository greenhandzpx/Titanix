use core::mem::size_of;

use log::debug;

use crate::{
    config::{fs::RLIMIT_OFILE, mm::USER_STACK_SIZE},
    processor::current_process,
    utils::error::SyscallRet,
};

/// Infinity for RLimit
pub static mut RLIM_INFINITY: usize = usize::MAX;

const RLIMIT_CPU: u32 = 0;
const RLIMIT_FSIZE: u32 = 1;
const RLIMIT_DATA: u32 = 2;
const RLIMIT_STACK: u32 = 3;
const RLIMIT_CORE: u32 = 4;
const RLIMIT_RSS: u32 = 5;
const RLIMIT_NPROC: u32 = 6;
const RLIMIT_NOFILE: u32 = 7;
const RLIMIT_MEMLOCK: u32 = 8;
const RLIMIT_AS: u32 = 9;
const RLIMIT_LOCKS: u32 = 10;
const RLIMIT_SIGPENDING: u32 = 11;
const RLIMIT_MSGQUEUE: u32 = 12;
const RLIMIT_NICE: u32 = 13;
const RLIMIT_RTPRIO: u32 = 14;
const RLIMIT_RTTIME: u32 = 15;

/// RLimit size
pub const RLIMIT_SIZE: usize = size_of::<RLimit>();
/// Resource Limit
#[derive(Debug, Clone)]
pub struct RLimit {
    /// Soft limit
    pub rlim_cur: usize,
    /// Hard limit (ceiling for rlim_cur)
    pub rlim_max: usize,
}

impl RLimit {
    /// New a RLimit
    pub fn new(cur: usize, max: usize) -> Self {
        Self {
            rlim_cur: cur,
            rlim_max: max,
        }
    }
    /// Set RLimit
    pub fn set_rlimit(&self, resource: u32) -> SyscallRet {
        debug!("[set_rlimit] try to set limit: {:?}", self);
        unsafe {
            RLIM_INFINITY = self.rlim_max;
        }
        current_process().inner_handler(|proc| proc.rlimit = self.clone());
        Ok(0)
    }
    /// Get RLimit
    pub fn get_rlimit(resource: u32) -> Self {
        match resource {
            RLIMIT_STACK => Self::new(USER_STACK_SIZE, unsafe { RLIM_INFINITY }),
            RLIMIT_NOFILE => Self {
                rlim_cur: RLIMIT_OFILE,
                rlim_max: unsafe { RLIM_INFINITY },
            },
            _ => Self {
                rlim_cur: 0,
                rlim_max: 0,
            },
        }
    }
}
