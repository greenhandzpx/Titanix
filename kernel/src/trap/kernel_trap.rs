use log::{error, warn};
use riscv::register::{
    scause::{self, Interrupt, Trap},
    sepc, stval,
};

use crate::timer::set_next_trigger;

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
            warn!("timer interrupt!!");
            set_next_trigger();
            // todo!()
        }
        _ => {
            // error!("other exception!!");
            error!(
                "[kernel] {:?}(scause:{}) in application, bad addr = {:#x}, bad instruction = {:#x}, kernel panicked!!",
                scause::read().cause(),
                scause::read().bits(),
                sepc::read(),
                stval::read(),
            );
            panic!(
                "a trap {:?} from kernel! stval {:#x}",
                scause::read().cause(),
                stval::read()
            );
        }
    }
}
