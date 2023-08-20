use crate::{
    config::mm::USER_STACK_SIZE, processor::current_process, stack_trace, utils::error::SyscallRet,
};

/// Infinity for RLimit
pub const RLIM_INFINITY: usize = usize::MAX;

#[allow(unused)]
const RLIMIT_CPU: u32 = 0;
#[allow(unused)]
const RLIMIT_FSIZE: u32 = 1;
#[allow(unused)]
const RLIMIT_DATA: u32 = 2;
#[allow(unused)]
const RLIMIT_STACK: u32 = 3;
#[allow(unused)]
const RLIMIT_CORE: u32 = 4;
#[allow(unused)]
const RLIMIT_RSS: u32 = 5;
#[allow(unused)]
const RLIMIT_NPROC: u32 = 6;
#[allow(unused)]
const RLIMIT_NOFILE: u32 = 7;
#[allow(unused)]
const RLIMIT_MEMLOCK: u32 = 8;
#[allow(unused)]
const RLIMIT_AS: u32 = 9;
#[allow(unused)]
const RLIMIT_LOCKS: u32 = 10;
#[allow(unused)]
const RLIMIT_SIGPENDING: u32 = 11;
#[allow(unused)]
const RLIMIT_MSGQUEUE: u32 = 12;
#[allow(unused)]
const RLIMIT_NICE: u32 = 13;
#[allow(unused)]
const RLIMIT_RTPRIO: u32 = 14;
#[allow(unused)]
const RLIMIT_RTTIME: u32 = 15;

/// Resource Limit
#[derive(Debug, Clone, Copy)]
pub struct RLimit {
    /// Soft limit
    pub rlim_cur: usize,
    /// Hard limit (ceiling for rlim_cur)
    pub rlim_max: usize,
}

impl RLimit {
    /// New a RLimit
    pub fn new(cur: usize, max: usize) -> Self {
        stack_trace!();
        Self {
            rlim_cur: cur,
            rlim_max: max,
        }
    }
    /// Set RLimit
    pub fn set_rlimit(resource: u32, rlimit: &RLimit) -> SyscallRet {
        stack_trace!();
        log::info!("[set_rlimit] try to set limit: {:?}", resource);
        match resource {
            RLIMIT_NOFILE => {
                current_process().inner_handler(|proc| proc.fd_table.set_rlimit(*rlimit))
            }
            _ => {}
        }
        Ok(0)
    }
    /// Get RLimit
    pub fn get_rlimit(resource: u32) -> Self {
        stack_trace!();
        match resource {
            RLIMIT_STACK => Self::new(USER_STACK_SIZE, RLIM_INFINITY),
            RLIMIT_NOFILE => current_process().inner_handler(|proc| proc.fd_table.rlimit()),
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
        stack_trace!();
        Self {
            set: (1 << cpus - 1),
            dummy: [0; 15],
        }
    }
}
