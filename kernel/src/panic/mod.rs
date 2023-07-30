//! The panic handler
use crate::driver::shutdown;
use core::panic::PanicInfo;
use log::error;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("[kernel] Panicked: {}", info.message().unwrap());
    }
    #[cfg(feature = "stack_trace")]
    {
        log::error!("backtrace:");
        crate::processor::local_hart()
            .env()
            .stack_tracker
            .print_stacks_err();
    }
    shutdown()
}
