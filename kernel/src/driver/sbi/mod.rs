//! SBI call wrappers
#![allow(unused)]

use core::arch::asm;

use super::CharDevice;

// EID, FID
const SBI_HART_START: (usize, usize) = (0x48534d, 0);
const SBI_HART_STOP: (usize, usize) = (0x48534d, 1);
const SBI_HART_GET_STATUS: (usize, usize) = (0x48534d, 2);
const SBI_HART_SUSPEND: (usize, usize) = (0x48534d, 3);

const SBI_SET_TIMER: (usize, usize) = (0, 0);
const SBI_CONSOLE_PUTCHAR: (usize, usize) = (1, 0);
const SBI_CONSOLE_GETCHAR: (usize, usize) = (2, 0);
const SBI_CLEAR_IPI: (usize, usize) = (3, 0);
const SBI_SEND_IPI: (usize, usize) = (4, 0);
const SBI_REMOTE_FENCE_I: (usize, usize) = (5, 0);
const SBI_REMOTE_SFENCE_VMA: (usize, usize) = (6, 0);
const SBI_REMOTE_SFENCE_VMA_ASID: (usize, usize) = (7, 0);
const SBI_SHUTDOWN: (usize, usize) = (8, 0);

/// general sbi call
#[inline(always)]
fn sbi_call(eid_fid: (usize, usize), arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            // "li x16, 0",
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") eid_fid.1,
            in("x17") eid_fid.0,
        );
    }
    ret
}
/// use sbi call to set timer
pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, timer, 0, 0);
}
/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}
/// use sbi call to getchar from console (qemu uart handler)
pub fn console_getchar() -> u8 {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0) as u8
}
/// use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

/// use sbi call to start the specific core
pub fn hart_start(hart_id: usize, start_addr: usize) -> usize {
    sbi_call(SBI_HART_START, hart_id, start_addr, 0)
}

pub struct SbiChar;

impl SbiChar {
    pub fn new() -> Self {
        Self {}
    }
}

impl CharDevice for SbiChar {
    fn getchar(&self) -> u8 {
        console_getchar()
    }
    fn puts(&self, str: &[u8]) {
        for s in str {
            console_putchar(*s as usize);
        }
    }
    fn handle_irq(&self) {
        todo!()
    }
}
