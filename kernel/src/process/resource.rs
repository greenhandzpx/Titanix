use core::mem::size_of;

use log::debug;

use crate::{
    config::{fs::RLIMIT_OFILE, mm::USER_STACK_SIZE},
    fs::MAX_FD,
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
        match resource {
            RLIMIT_NOFILE => unsafe {
                MAX_FD.store(RLIM_INFINITY, core::sync::atomic::Ordering::Relaxed)
            },
            _ => {}
        }
        current_process().inner_handler(|proc| proc.rlimit = self.clone());
        Ok(0)
    }
    /// Get RLimit
    pub fn get_rlimit(resource: u32) -> Self {
        match resource {
            RLIMIT_STACK => Self::new(
                {
                    unsafe {
                        if USER_STACK_SIZE > RLIM_INFINITY {
                            RLIM_INFINITY
                        } else {
                            USER_STACK_SIZE
                        }
                    }
                },
                unsafe { RLIM_INFINITY },
            ),
            RLIMIT_NOFILE => Self {
                rlim_cur: {
                    unsafe {
                        if RLIMIT_OFILE > RLIM_INFINITY {
                            RLIM_INFINITY
                        } else {
                            RLIMIT_OFILE
                        }
                    }
                },
                rlim_max: unsafe { RLIM_INFINITY },
            },
            _ => Self {
                rlim_cur: 0,
                rlim_max: 0,
            },
        }
    }
}

/// A cpu set
/// If you want to set cpu 1, then set = 1 << (1+1) -1
/// Which means set = 1 << (cpu + 1) -1
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CpuSet {
    /// cpu set
    pub set: usize,
    /// for padding
    pub dummy: [usize; 15],
}
impl CpuSet {
    /// alloc a cpu set
    /// you should pass the max number of cpus which you want to set
    pub fn new(cpus: usize) -> Self {
        Self {
            set: (1 << cpus - 1),
            dummy: [0; 15],
        }
    }
}
