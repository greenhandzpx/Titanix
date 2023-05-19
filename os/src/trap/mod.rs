//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].
mod context;

use crate::mm::VirtAddr;
use crate::process::thread::exit_and_terminate_all_threads;
use crate::processor::{current_process, current_trap_cx, local_hart};
use crate::signal::check_signal_for_current_process;
use crate::stack_trace;
// use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::{process, syscall::syscall};
// use crate::process::self;
// use crate::process::{
//     current_trap_cx, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, self,
// };
use crate::timer::set_next_trigger;
use core::arch::global_asm;
use log::{debug, error, info, warn};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

extern "C" {
    fn __trap_from_user();
}

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        // stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
        stvec::write(__trap_from_user as usize, TrapMode::Direct);
    }
}
/// enable timer interrupt in sie CSR
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
/// handle an interrupt, exception, or system call from user space
pub async fn trap_handler() {
    // TODO: modify the trap handout to be async

    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            cx.sepc += 4;
            let old_user_x17 = cx.user_x[17];
            // get system call return value
            let result = syscall(
                cx.user_x[17],
                [
                    cx.user_x[10],
                    cx.user_x[11],
                    cx.user_x[12],
                    cx.user_x[13],
                    cx.user_x[14],
                    cx.user_x[15],
                ],
            )
            .await;
            // cx is changed during sys_exec, so we have to call it again
            cx = current_trap_cx();
            stack_trace!();
            // if old_user_x17 == 220 {
            //     info!("sys_clone sepc {:#x}", cx.sepc);
            // }
            cx.user_x[10] = match result {
                Ok(ret) => ret as usize,
                Err(err) => -(err as isize) as usize,
            }
            // TODO: Change into async syscall
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::InstructionFault)
        | Trap::Exception(Exception::InstructionPageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            match current_process().inner_handler(|proc| {
                let va = VirtAddr::from(stval);
                proc.memory_set.handle_page_fault(va, scause.bits())
            }) {
                Ok(()) => {
                    debug!(
                        "[kernel] [proc {}]handle legal page fault, addr {:#x}, instruction {:#x}",
                        current_process().pid(),
                        stval,
                        current_trap_cx().sepc
                    );
                }
                Err(_) => {
                    warn!(
                        "[kernel] {:?}(scause:{}) in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it. pid: {}",
                        scause.cause(),
                        scause.bits(),
                        stval,
                        current_trap_cx().sepc,
                        current_process().pid()
                    );
                    #[cfg(feature = "stack_trace")]
                    warn!("backtrace:");
                    local_hart()
                        .env()
                        .stack_tracker
                        .as_mut()
                        .unwrap()
                        .print_stacks();
                    exit_and_terminate_all_threads(-2);
                    // current_process().inner_handler(|proc| {
                    //     proc.exit_code = -2;
                    //     proc.is_zombie = true;
                    // });
                }
            }
            // let sstatus = sstatus::read();
            // debug!("sstatus {:#x}", sstatus.bits());
            // There are serveral kinds of page faults:
            // 1. mmap area
            // 2. sbrk area
            // 3. fork cow area
            // 4. user stack
            // 5. execve elf file
            // 6. dynamic link
            // 7. illegal page fault

            // todo!("Exit current process when encounting illegal addr");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            // println!("[kernel] IllegalInstruction in application, kernel killed it.");
            // // illegal instruction exit code
            current_process().set_zombie();
            // exit_current_and_run_next(-3);
            // todo!("Exit current process when encounting illegal instruction");
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // debug!("time interrupt, pid {}", current_process().pid());
            // debug!("sstatus {:#x}", sstatus::read().bits());
            // debug!("timer interrrupt");
            set_next_trigger();
            process::yield_now().await
            // suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }

    // TODO: modify trap ret
    // trap_return();
}

/// #[no_mangle]
/// set the new addr of __restore asm function in TRAMPOLINE page,
/// set the reg a0 = trap_cx_ptr, reg a1 = phy addr of usr page table,
/// finally, jump to new addr of __restore asm function
// pub fn trap_return() -> ! {
//     set_user_trap_entry();
//     let trap_cx_ptr = TRAP_CONTEXT;
//     let user_satp = current_user_token();
//     extern "C" {
//         fn __alltraps();
//         fn __restore();
//     }
//     let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
//     unsafe {
//         asm!(
//             "fence.i",
//             "jr {restore_va}",
//             restore_va = in(reg) restore_va,
//             in("a0") trap_cx_ptr,
//             in("a1") user_satp,
//             options(noreturn)
//         );
//     }
// }

#[no_mangle]
/// Back to user mode.
/// Note that we don't need to flush TLB since user and 
/// kernel use the same pagetable.
pub fn trap_return(trap_context: &mut TrapContext) {
    set_user_trap_entry();
    extern "C" {
        // fn __alltraps();
        fn __return_to_user(cx: *mut TrapContext);
    }

    check_signal_for_current_process();
    // info!("trap return sepc {:#x}", trap_context.sepc);
    unsafe {
        __return_to_user(trap_context);
    }
}

#[no_mangle]
/// Unimplement: traps/interrupts/exceptions from kernel mode
/// Todo: Chapter 9: I/O device
pub fn trap_from_kernel() -> ! {
    error!(
        "[kernel] {:?}(scause:{}) in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it. pid: {}",
        scause::read().cause(),
        scause::read().bits(),
        stval::read(),
        current_trap_cx().sepc,
        current_process().pid()
    );
    panic!(
        "a trap {:?} from kernel! stval {:#x}",
        scause::read().cause(),
        stval::read()
    );
}

pub use context::TrapContext;
pub use context::UserContext;
