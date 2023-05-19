use core::arch::global_asm;

use log::{debug, error, warn};
use riscv::register::{stvec, utvec::TrapMode};

use crate::{
    config::{mm::PAGE_SIZE, process::SYSCALL_STR_ARG_MAX_LEN},
    process::thread::exit_and_terminate_all_threads,
    processor::{current_process, local_hart, SumGuard},
    trap::trap_from_kernel,
    utils::error::{GeneralRet, SyscallErr}, stack_trace,
};

use super::VirtAddr;

global_asm!(include_str!("check.S"));
///
pub struct UserCheck {
    _sum_guard: SumGuard,
}

extern "C" {
    fn __try_access_user_error_trap();
    fn __try_read_user_u8(user_addr: usize) -> (usize, usize);
    fn __try_write_user_u8(user_addr: usize) -> (usize, usize);
    fn __trap_from_user();
}

impl Drop for UserCheck {
    fn drop(&mut self) {
        unsafe {
            // stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
            stvec::write(trap_from_kernel as usize, TrapMode::Direct);
        }
    }
}
impl UserCheck {
    /// Create a new UserCheck
    pub fn new() -> Self {
        unsafe {
            stvec::write(__try_access_user_error_trap as usize, TrapMode::Direct);
        }
        let ret = Self {
            _sum_guard: SumGuard::new(),
        };
        ret
    }

    /// Check wether the given user addr is readable or not
    pub fn check_readable_slice(&self, buf: *const u8, len: usize) -> GeneralRet<()> {
        // let _sum_guard = SumGuard::new();
        let buf_start: VirtAddr = VirtAddr::from(buf as usize).floor().into();
        let buf_end: VirtAddr = VirtAddr::from(buf as usize + len).ceil().into();
        let mut va = buf_start;
        // debug!("[proc {}] check read sva {:#x} eva {:#x}", current_process().pid(), buf_start.0, buf_end.0);
        while va < buf_end {
            if let Some(scause) = self.try_read_u8(va.into()) {
                self.handle_page_fault(&mut va, scause)?;
            }
            va.0 += PAGE_SIZE;
        }
        Ok(())
    }

    /// Check wether the given user addr is writable or not
    pub fn check_writable_slice(&self, buf: *mut u8, len: usize) -> GeneralRet<()> {
        // let _sum_guard = SumGuard::new();
        let buf_start: VirtAddr = VirtAddr::from(buf as usize).floor().into();
        let buf_end: VirtAddr = VirtAddr::from(buf as usize + len).ceil().into();
        let mut va = buf_start;
        while va < buf_end {
            if let Some(scause) = self.try_write_u8(va.into()) {
                self.handle_page_fault(&mut va, scause)?
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
                self.handle_page_fault(&mut va, scause)?
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
    fn try_read_u8(&self, user_addr: usize) -> Option<usize> {
        // debug!("satp(2) {:#x}", satp::read().bits());
        // debug!("try read u8, addr {:#x}", user_addr);
        let (a0, scause) = unsafe { __try_read_user_u8(user_addr) };
        match a0 {
            0 => None,
            _ => Some(scause),
        }
    }

    fn try_write_u8(&self, user_addr: usize) -> Option<usize> {
        let (a0, scause) = unsafe { __try_write_user_u8(user_addr) };
        match a0 {
            0 => None,
            // TODO: optimize
            _ => Some(15),
        }
    }

    fn handle_page_fault(&self, va: &mut VirtAddr, scause: usize) -> GeneralRet<()> {
        match current_process().inner_handler(|proc| proc.memory_set.handle_page_fault(*va, scause))
        {
            Ok(()) => {
                debug!(
                    "[kernel] [proc {}]handle legal page fault, addr {:#x}",
                    current_process().pid(),
                    va.0
                );
                va.0 += PAGE_SIZE;
                Ok(())
            }
            Err(_) => {
                warn!(
                    "[kernel] [user check](scause:{}) in application, bad addr = {:#x}, kernel killed it. pid: {}",
                    scause,
                    va.0,
                    current_process().pid()
                );
                warn!("backtrace:");
                local_hart()
                    .env()
                    .stack_tracker
                    .as_ref()
                    .unwrap()
                    .print_stacks();
                exit_and_terminate_all_threads(-2);
                // current_process().inner_handler(|proc| {
                //     proc.exit_code = -2;
                //     proc.is_zombie = true;
                // });
                return Err(SyscallErr::EFAULT);
            }
        }
    }
}
