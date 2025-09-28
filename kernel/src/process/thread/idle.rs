#[cfg(feature = "kernel_preempt")]
use crate::process::thread;

pub fn start_idle_thread() {
    // idle thread
    #[cfg(feature = "kernel_preempt")]
    thread::spawn_time_consuming_kernel_thread(
        async move {
            loop {
                unsafe {
                    core::arch::riscv64::wfi();
                }
            }
        },
        "idle",
    );
}
