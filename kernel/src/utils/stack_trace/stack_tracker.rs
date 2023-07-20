use core::fmt::Display;

use alloc::vec::Vec;
use log::warn;

use crate::processor::local_hart;

use super::Msg;

pub struct StackTracker {
    stack_info_trace: Vec<StackInfo>,
}

impl StackTracker {
    pub fn new() -> Self {
        Self {
            stack_info_trace: Vec::new(),
        }
    }

    pub fn push_stack_info(&mut self, stack_info: StackInfo) {
        self.stack_info_trace.push(stack_info);
    }
    pub fn pop_stack_info(&mut self) {
        self.stack_info_trace.pop();
    }
    pub fn print_stacks(&self) {
        for stack_info in &self.stack_info_trace {
            log::warn!("{}", stack_info);
        }
    }
    pub fn print_stacks_err(&self) {
        for stack_info in &self.stack_info_trace {
            log::error!("{}", stack_info);
        }
    }
}

pub struct StackInfoGuard {}

impl StackInfoGuard {
    pub fn new(msg: Msg, file: &'static str, line: u32) -> Self {
        local_hart()
            .env_mut()
            .stack_tracker
            .push_stack_info(StackInfo::new(msg, file, line));
        Self {}
    }
}

impl Drop for StackInfoGuard {
    fn drop(&mut self) {
        local_hart().env_mut().stack_tracker.pop_stack_info();
    }
}

pub struct StackInfo {
    msg: Msg,
    file: &'static str,
    line: u32,
}

impl StackInfo {
    pub fn new(msg: Msg, file: &'static str, line: u32) -> Self {
        Self { msg, file, line }
    }

    // pub fn print(&self) {
    //     warn!("[{}:{}]:{}", self.file, self.line, self.msg)
    // }
}

impl Display for StackInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}:{}]:{}", self.file, self.line, self.msg)
    }
}
