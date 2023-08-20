use core::arch::global_asm;

use log::{debug, error, warn};
use riscv::register::{scause::Scause, stvec, utvec::TrapMode};

use crate::{
    config::{mm::PAGE_SIZE, process::SYSCALL_STR_ARG_MAX_LEN},
    processor::{current_process, current_task, SumGuard},
    signal::SIGSEGV,
    stack_trace,
    sync::mutex::SieGuard,
    trap::set_kernel_trap_entry,
    utils::{
        async_utils::block_on,
        error::{GeneralRet, SyscallErr},
    },
};

use super::{memory_space, VirtAddr};

global_asm!(include_str!("check.S"));
///
pub struct UserCheck {
    _sum_guard: SumGuard,
    _sie_guard: SieGuard,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct TryOpRet {
    is_err: usize,
    scause: Scause,
}

// const STORE_PAGE_FAULT: usize = 15;

extern "C" {
    fn __try_access_user_error_trap();
    fn __try_read_user_u8(user_addr: usize) -> TryOpRet;
    fn __try_write_user_u8(user_addr: usize) -> TryOpRet;
    fn __trap_from_user();
    fn __trap_from_kernel();
}

impl Drop for UserCheck {
    fn drop(&mut self) {
        // In Rust, the components drop after the container.
        // So at this time, the _sie_guard is still alive.
        set_kernel_trap_entry();
    }
}

impl UserCheck {
    /// Create a new UserCheck
    pub fn new() -> Self {
        let ret = Self {
            _sum_guard: SumGuard::new(),
            _sie_guard: SieGuard::new(),
        };
        unsafe {
            stvec::write(__try_access_user_error_trap as usize, TrapMode::Direct);
        }
        ret
    }

    /// Check wether the given user addr is readable or not
    pub fn check_readable_slice(&self, buf: *const u8, len: usize) -> GeneralRet<()> {
        stack_trace!();
        let buf_start: VirtAddr = VirtAddr::from(buf as usize).floor().into();
        let mut buf_end: VirtAddr = VirtAddr::from(buf as usize + len).ceil().into();
        if buf_end.0 == 0 && buf_start.0 > 0 {
            buf_end.0 = usize::MAX;
        }
        let mut va = buf_start;
        while va < buf_end {
            if let Some(scause) = self.try_read_u8(va.into()) {
                block_on(self.handle_page_fault(va, scause))?
            }
            va.0 += PAGE_SIZE;
        }
        Ok(())
    }

    /// Check wether the given user addr is writable or not
    pub fn check_writable_slice(&self, buf: *mut u8, len: usize) -> GeneralRet<()> {
        stack_trace!();
        log::debug!(
            "[check_writable_slice] buf addr {:#x} len {:#x}",
            buf as usize,
            len
        );
        let buf_start: VirtAddr = VirtAddr::from(buf as usize).floor().into();
        let buf_end: VirtAddr = VirtAddr::from(buf as usize + len).ceil().into();
        let mut va = buf_start;
        while va < buf_end {
            if let Some(scause) = self.try_write_u8(va.into()) {
                block_on(self.handle_page_fault(va, scause))?
            }
            va.0 += PAGE_SIZE;
        }
        Ok(())
    }

    /// Check wether the given user c string is legal or not.
    pub fn check_c_str(&self, c_str: *const u8) -> GeneralRet<()> {
        debug!("[kernel] check c str");
        stack_trace!();
        let start_addr: VirtAddr = VirtAddr::from(c_str as usize).floor().into();
        let mut va = start_addr;
        let mut first = true;
        loop {
            if let Some(scause) = self.try_read_u8(va.into()) {
                block_on(self.handle_page_fault(va, scause))?
            }
            if first {
                if self.check_c_str_end(VirtAddr::from(c_str as usize)) {
                    return Ok(());
                }
            } else {
                if self.check_c_str_end(va) {
                    return Ok(());
                }
            }
            va.0 += PAGE_SIZE;
            if va.0 - VirtAddr::from(c_str as usize).0 >= SYSCALL_STR_ARG_MAX_LEN {
                error!(
                    "user c str too long!, first {}, va {:#x}, str {:#x}",
                    first, va.0, c_str as usize
                );
                return Err(SyscallErr::EINVAL);
            }
            if first {
                first = false;
            }
        }
    }

    fn check_c_str_end(&self, va: VirtAddr) -> bool {
        let end: VirtAddr = VirtAddr::from(VirtAddr::from(va.floor()).0 + PAGE_SIZE);
        for addr in va.0..end.0 {
            let ch: u8 = unsafe { *(addr as *const u8) };
            if ch == 0 {
                return true;
            }
        }
        false
    }

    /// return `scause` if page fault
    fn try_read_u8(&self, user_addr: usize) -> Option<Scause> {
        // debug!("satp(2) {:#x}", satp::read().bits());
        // debug!("try read u8, addr {:#x}", user_addr);
        let ret = unsafe { __try_read_user_u8(user_addr) };
        match ret.is_err {
            0 => None,
            _ => Some(ret.scause),
        }
    }

    fn try_write_u8(&self, user_addr: usize) -> Option<Scause> {
        let ret = unsafe { __try_write_user_u8(user_addr) };
        match ret.is_err {
            0 => None,
            // TODO: optimize
            _ => Some(ret.scause),
        }
    }

    async fn handle_page_fault(&self, va: VirtAddr, scause: Scause) -> GeneralRet<()> {
        stack_trace!();
        match memory_space::handle_page_fault(va, scause).await {
            Ok(_) => {
                log::trace!(
                    "[kernel] [proc {}]handle legal page fault, addr {:#x}",
                    current_process().pid(),
                    va.0
                );
                // va.0 += PAGE_SIZE;
                Ok(())
            }
            Err(_) => {
                warn!(
                    "[kernel] [UserCheck](scause:{:?}) in application, bad addr = {:#x}, kernel killed it. pid: {}",
                    scause.cause(),
                    va.0,
                    current_process().pid()
                );
                current_task().recv_signal(SIGSEGV);
                #[cfg(feature = "stack_trace")]
                {
                    warn!("backtrace:");
                    crate::processor::local_hart()
                        .env()
                        .stack_tracker
                        .print_stacks();
                }
                // exit_and_terminate_all_threads(0);
                return Err(SyscallErr::EFAULT);
            }
        }
    }
}
