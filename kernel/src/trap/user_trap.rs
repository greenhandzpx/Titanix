use log::warn;
use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    sepc, stval,
};

use crate::{
    mm::{memory_space, VirtAddr, VA_WIDTH_SV39},
    process::thread::{self, exit_and_terminate_all_threads},
    processor::{
        close_interrupt, current_process, current_task, current_trap_cx, hart::local_hart,
        open_interrupt,
    },
    signal::{check_signal_for_current_process, check_signal_for_current_thread},
    stack_trace,
    syscall::syscall,
    timer::{handle_timeout_events, set_next_trigger},
    trap::set_user_trap_entry,
    utils::error::SyscallErr,
};

use super::{set_kernel_trap_entry, TrapContext};

#[no_mangle]
/// handle an interrupt, exception, or system call from user space
pub async fn trap_handler() {
    set_kernel_trap_entry();

    let stval = stval::read();
    let scause = scause::read();
    // info!(
    //     "trap in, sepc {:#x}, user sp {:#x}, kernel sp {:#x}",
    //     current_trap_cx().sepc,
    //     current_trap_cx().user_x[2],
    //     current_trap_cx().kernel_sp,
    // );

    open_interrupt();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            cx.sepc += 4;
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
            cx.user_x[10] = match result {
                Ok(ret) => ret as usize,
                Err(err) => {
                    log::info!("[trap_handler] syscall return, err {}", err as usize);
                    -(err as isize) as usize
                }
            }
            // TODO: Change into async syscall
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::InstructionFault)
        | Trap::Exception(Exception::InstructionPageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            log::debug!(
                "[kernel] encounter page fault, addr {:#x}, instruction {:#x} scause {:?}",
                stval,
                current_trap_cx().sepc,
                scause.cause()
            );
            stack_trace!();

            let tmp = (stval as isize >> VA_WIDTH_SV39) as isize;
            if tmp != 0 && tmp != -1 {
                log::error!("v {:#x}, tmp {:#x}", stval, tmp);
                local_hart().env().stack_tracker.print_stacks_err();
                exit_and_terminate_all_threads(-2);
            }
            match memory_space::handle_page_fault(VirtAddr::from(stval), scause.bits()).await {
                Ok(()) => {
                    log::trace!(
                        "[kernel] handle legal page fault, addr {:#x}, instruction {:#x}",
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
                    local_hart().env().stack_tracker.print_stacks();

                    exit_and_terminate_all_threads(-2);
                }
            }
            // There are serveral kinds of page faults:
            // 1. mmap area
            // 2. sbrk area
            // 3. fork cow area
            // 4. user stack
            // 5. execve elf file
            // 6. dynamic link
            // 7. illegal page fault
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            warn!(
                "[kernel] IllegalInstruction in application, kernel killed it, stval {:#x}, sepc {:#x}",
                stval,
                sepc::read(),
            );
            #[cfg(feature = "stack_trace")]
            warn!("backtrace:");
            local_hart().env().stack_tracker.print_stacks();
            exit_and_terminate_all_threads(-2);
        }
        Trap::Exception(Exception::Breakpoint) => {
            warn!(
                "[kernel] Breakpoint from application, sepc = {:#x}",
                sepc::read(),
            );
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            cx.sepc += 2;
            // process::yield_now().await
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            handle_timeout_events();
            set_next_trigger();
            thread::yield_now().await;
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!, sepc = {:#x}",
                scause.cause(),
                stval,
                sepc::read(),
            );
        }
    }

    // TODO: modify trap ret
    // trap_return();
}

#[no_mangle]
/// Back to user mode.
/// Note that we don't need to flush TLB since user and
/// kernel use the same pagetable.
pub fn trap_return() {
    // Important!
    close_interrupt();

    set_user_trap_entry();
    extern "C" {
        // fn __alltraps();
        fn __return_to_user(cx: *mut TrapContext);
    }

    // If no pending sig for process, then check for thread.
    // TODO: not sure whether this is the right way
    if !check_signal_for_current_process() {
        check_signal_for_current_thread();
    }

    unsafe {
        (*current_task().inner.get()).time_info.when_trap_ret();

        // error!(
        //     "[trap_return] user sp {:#x}, sepc {:#x}, trap cx addr {:#x}",
        //     current_trap_cx().user_x[2],
        //     current_trap_cx().sepc,
        //     current_trap_cx() as *mut _ as usize
        // );

        __return_to_user(current_trap_cx());

        // Open interrupt in `trap_handler`

        // error!(
        //     "[trap_in] sepc {:#x}, x10 {:#x} x11 {:#x}",
        //     current_trap_cx().sepc,
        //     current_trap_cx().user_x[10],
        //     current_trap_cx().user_x[11],
        // );

        (*current_task().inner.get()).time_info.when_trap_in();
    }
}
