use log::error;
use riscv::register::{
    scause::{self, Interrupt, Trap},
    sepc, stval,
};

use crate::{
    executor,
    irq_count::IRQ_COUNTER,
    processor::{
        hart::{local_hart_preemptible, set_local_hart_preemptible},
        local_hart, local_irq_disable,
    },
    timer::{handle_timeout_events, set_next_trigger},
};

/// Kernel trap handler
#[no_mangle]
pub fn kernel_trap_handler() {
    let scause = scause::read();
    let _stval = stval::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            // error!("external interrrupt!!");
            crate::driver::intr_handler();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // log::error!("kernel timer interrrupt!!");

            // let sstatus = riscv::register::sstatus::read();
            // log::info!("[kernel_trap_handler] sie {}", sstatus.sie());

            IRQ_COUNTER.add1(1);
            handle_timeout_events();
            set_next_trigger();

            #[cfg(not(feature = "kernel_preempt"))]
            return;

            if !local_hart_preemptible() {
                log::info!("[kernel_trap_handler] cannot preempt");
                return;
            }
            if !executor::has_task() {
                return;
            }

            // log::info!("[kernel_trap_handler] run one task");

            set_local_hart_preemptible(false);

            let mut old_hart = local_hart().enter_preempt_switch();
            executor::run_one_task();
            local_hart().leave_preempt_switch(&mut old_hart);

            // log::info!("[kernel_trap_handler] run one task finished");
            local_irq_disable();
            set_local_hart_preemptible(true);
        }
        _ => {
            // error!("other exception!!");
            error!(
                "[kernel] {:?}(scause:{}) in application, bad addr = {:#x}, bad instruction = {:#x}, kernel panicked!!",
                scause::read().cause(),
                scause::read().bits(),
                stval::read(),
                sepc::read(),
            );
            panic!(
                "a trap {:?} from kernel! stval {:#x}",
                scause::read().cause(),
                stval::read()
            );
        }
    }
}
