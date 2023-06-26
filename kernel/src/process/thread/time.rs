use core::time::Duration;

use log::info;

use crate::timer::current_time_duration;

/// Used for sys_getrusage
///                                                  -- user --                -user- 
/// ---kernel---(switch to other thread) ---kernel---          --- kernel --          --- (switch)
///                                   enter          ret      trap          ret     trap     leave
pub struct ThreadTimeInfo {
    pub start_ts: Duration,
    pub user_time: Duration,
    pub sys_time: Duration,
    pub last_enter_ts: Duration,
    pub last_leave_ts: Duration,
    pub last_user_ret_ts: Option<Duration>,
    pub last_user_trap_ts: Option<Duration>,
}

impl ThreadTimeInfo {
    pub fn new() -> Self {
        let current_ts = current_time_duration();
        Self {
            start_ts: current_ts,
            user_time: Duration::ZERO,
            sys_time: Duration::ZERO,
            last_enter_ts: current_ts,
            last_leave_ts: current_ts,
            last_user_ret_ts: None, 
            last_user_trap_ts: None,
        }
    }

    /// Switch to this task
    pub fn when_entering(&mut self) {
        self.last_enter_ts = current_time_duration();
    }

    /// Switch to other task
    pub fn when_leaving(&mut self) {
        let current_ts = current_time_duration();
        if let Some(last_trap) = self.last_user_trap_ts {
            self.sys_time += current_ts - last_trap;
        } else {
            self.sys_time += current_ts - self.last_enter_ts;
        }
        self.last_leave_ts = current_ts;
        // info!("update sys time {:?}", self.sys_time);
    }

    /// Trap return to user
    pub fn when_trap_ret(&mut self) {
        let current_ts = current_time_duration();
        if let Some(last_trap) = self.last_user_trap_ts {
            self.sys_time += current_ts - last_trap;
        } else {
            self.sys_time += current_ts - self.last_enter_ts;
        }
        self.last_user_ret_ts = Some(current_ts);
    }

    /// Trap from user
    pub fn when_trap_in(&mut self) {
        let current_ts = current_time_duration();
        self.user_time += current_ts - self.last_user_ret_ts.unwrap();
        self.last_user_ret_ts = None;
        self.last_user_trap_ts = Some(current_ts);
    }
}
