//! The panic handler
use crate::{driver::shutdown, processor::local_hart};
use core::panic::PanicInfo;
use log::{error, warn};

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
    warn!("backtrace:");
    local_hart().env().stack_tracker.print_stacks();
    shutdown()
}
