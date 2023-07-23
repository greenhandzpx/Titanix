use log::error;
use riscv::register::{
    scause::{self, Interrupt, Trap},
    sepc, stval,
};

use crate::timer::{handle_timeout_events, set_next_trigger};

/// Kernel trap handler
#[no_mangle]
pub fn kernel_trap_handler() {
    let scause = scause::read();
    let _stval = stval::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            // error!("external interrrupt!!");
            todo!()
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // warn!("timer interrupt!!");
            handle_timeout_events();
            set_next_trigger();
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
