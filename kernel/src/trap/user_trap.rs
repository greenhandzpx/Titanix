use log::{debug, info, warn};
use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    sepc, stval,
};

use crate::{
    mm::{memory_space, VirtAddr},
    process::{self, thread::exit_and_terminate_all_threads},
    processor::{current_process, current_task, current_trap_cx, hart::local_hart},
    signal::check_signal_for_current_process,
    stack_trace,
    syscall::syscall,
    timer::{handle_timeout_events, set_next_trigger},
    trap::set_user_trap_entry,
    FIRST_HART_ID,
};

use super::{set_kernel_trap_entry, TrapContext};

#[no_mangle]
/// handle an interrupt, exception, or system call from user space
pub async fn trap_handler() {
    set_kernel_trap_entry();

    // if local_hart().hart_id() as u8 != FIRST_HART_ID.load(core::sync::atomic::Ordering::Relaxed) {
    //     info!("other hart trap");
    // }

    unsafe {
        (*current_task().inner.get()).time_info.when_trap_in();
    }
    let scause = scause::read();
    let stval = stval::read();
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
            stack_trace!();
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
            debug!(
                "[kernel] encounter page fault, addr {:#x}, instruction {:#x} scause {:?}",
                stval,
                current_trap_cx().sepc,
                scause.cause()
            );
            match memory_space::handle_page_fault(VirtAddr::from(stval), scause.bits()).await {
                Ok(()) => {
                    debug!(
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
            warn!(
                "[kernel] IllegalInstruction in application, kernel killed it, stval {:#x}",
                stval
            );
            // // illegal instruction exit code
            current_process().set_zombie();
            // exit_current_and_run_next(-3);
            // todo!("Exit current process when encounting illegal instruction");
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
            // debug!("timer interrupt");
            handle_timeout_events();
            set_next_trigger();
            process::yield_now().await;
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
pub fn trap_return(trap_context: &mut TrapContext) {
    set_user_trap_entry();
    extern "C" {
        // fn __alltraps();
        fn __return_to_user(cx: *mut TrapContext);
    }

    check_signal_for_current_process();

    unsafe {
        (*current_task().inner.get()).time_info.when_trap_ret();
    }
    unsafe {
        __return_to_user(trap_context);
    }
}
