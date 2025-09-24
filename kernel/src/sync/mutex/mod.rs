use crate::processor::{local_irq_disable, local_irq_enable, local_irq_is_enabled};

use self::{remutex::ReentrantMutex, sleep_mutex::SleepMutex, spin_mutex::SpinMutex};

/// ReentrantMutex
pub mod remutex;
/// SleepMutex
pub mod sleep_mutex;
/// SpinMutex
pub mod spin_mutex;

/// SpinLock
pub type SpinLock<T> = SpinMutex<T, Spin>;
/// SpinNoIrqLock(Cannot be interrupted)
pub type SpinNoIrqLock<T> = SpinMutex<T, SpinNoIrq>;
/// SleepLock
pub type SleepLock<T> = SleepMutex<T, SpinNoIrq>;
/// ReentrantMutex
#[allow(unused)]
pub type ReentrantLock<T> = ReentrantMutex<T, SpinNoIrq>;

/// Low-level support for mutex(spinlock, sleeplock, etc)
pub trait MutexSupport {
    /// Guard data
    type GuardData;
    /// Called before lock() & try_lock()
    fn before_lock() -> Self::GuardData;
    /// Called when MutexGuard dropping
    fn after_unlock(_: &mut Self::GuardData);
}

/// Spin MutexSupport
pub struct Spin;

impl MutexSupport for Spin {
    type GuardData = ();
    #[inline(always)]
    fn before_lock() -> Self::GuardData {}
    #[inline(always)]
    fn after_unlock(_: &mut Self::GuardData) {}
}

/// Sie Guard
pub struct IrqEnableGuard(bool);

impl IrqEnableGuard {
    /// Construct a IrqEnableGuard
    pub fn new() -> Self {
        Self({
            let irq_enable_before = local_irq_is_enabled();
            local_irq_disable();
            irq_enable_before
        })
    }
}
impl Drop for IrqEnableGuard {
    fn drop(&mut self) {
        if self.0 {
            local_irq_enable();
        }
    }
}

/// SpinNoIrq MutexSupport
pub struct SpinNoIrq;

impl MutexSupport for SpinNoIrq {
    type GuardData = IrqEnableGuard;
    #[inline(always)]
    fn before_lock() -> Self::GuardData {
        IrqEnableGuard::new()
    }
    #[inline(always)]
    fn after_unlock(_: &mut Self::GuardData) {}
}
