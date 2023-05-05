/*
 * @Author: greenhandzpx 893522573@qq.com
 * @Date: 2023-01-31 08:35:53
 * @LastEditors: greenhandzpx 893522573@qq.com
 * @LastEditTime: 2023-01-31 12:53:32
 * @FilePath: /os/src/sync/mutex/mod.rs
 * @Description:
 *
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 */

use riscv::register::sstatus;

use self::spin_mutex::SpinMutex;

/// SleepMutex
pub mod sleep_mutex;
/// SpinMutex
pub mod spin_mutex;

/// SpinLock
pub type SpinLock<T> = SpinMutex<T, Spin>;
/// SpinNoIrqLock(Cannot be interrupted)
pub type SpinNoIrqLock<T> = SpinMutex<T, SpinNoIrq>;

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
pub struct SieGuard(bool);

impl SieGuard {
    fn new() -> Self {
        Self(unsafe {
            let sie_before = sstatus::read().sie();
            sstatus::clear_sie();
            sie_before
        })
    }
}
impl Drop for SieGuard {
    fn drop(&mut self) {
        if self.0 {
            unsafe {
                sstatus::set_sie();
            }
        }
    }
}

/// SpinNoIrq MutexSupport
pub struct SpinNoIrq;

impl MutexSupport for SpinNoIrq {
    type GuardData = SieGuard;
    #[inline(always)]
    fn before_lock() -> Self::GuardData {
        SieGuard::new()
    }
    #[inline(always)]
    fn after_unlock(_: &mut Self::GuardData) {}
}
