use core::time::Duration;

use log::info;

use crate::timer::current_time_duration;

/// Used for sys_getrusage
///                                                  -- user --
/// ---kernel---(switch to other thread) ---kernel---          --- kernel --- (switch)
pub struct ThreadTimeInfo {
    pub start_ts: Duration,
    pub user_time: Duration,
    pub sys_time: Duration,
    pub last_switch_ts: Duration,
    pub last_user_ret_ts: Duration,
}

impl ThreadTimeInfo {
    pub fn new() -> Self {
        let current_ts = current_time_duration();
        Self {
            start_ts: current_ts,
            user_time: Duration::ZERO,
            sys_time: Duration::ZERO,
            last_switch_ts: current_ts,
            last_user_ret_ts: current_ts,
        }
    }

    /// Switch to this task
    pub fn when_entering(&mut self) {
        self.last_switch_ts = current_time_duration(); 
    }

    /// Switch to other task
    pub fn when_leaving(&mut self) {
        let current_ts = current_time_duration();
        self.sys_time += current_ts - self.last_switch_ts;
        self.last_switch_ts = current_ts;
        // info!("update sys time {:?}", self.sys_time);
    }

    /// Trap return to user
    pub fn when_trap_ret(&mut self) {
        self.last_user_ret_ts = current_time_duration();
    }

    /// Trap from user
    pub fn when_trap_in(&mut self) {
        let current_ts = current_time_duration();
        self.user_time += current_ts - self.last_user_ret_ts;
    }

}
