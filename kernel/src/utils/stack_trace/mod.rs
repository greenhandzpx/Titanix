use core::fmt::Display;

pub mod stack_tracker;

/// add the current stack info(i.e. line, file) into stack tracer
#[macro_export]
#[cfg(all(feature = "stack_trace", not(feature = "kcov")))]
macro_rules! stack_trace {
    () => {
        let _stack_info_guard = $crate::utils::stack_trace::stack_tracker::StackInfoGuard::new(
            $crate::utils::stack_trace::Msg::None,
            file!(),
            line!(),
        );
    };
    // stack_trace!("message")
    ($msg: literal) => {
        let _stack_info_guard = $crate::utils::stack_trace::stack_tracker::StackInfoGuard::new(
            $crate::utils::stack_trace::Msg::Str($msg),
            file!(),
            line!(),
        );
    };
}
/// add the current stack info(i.e. line, file) into stack tracer
#[macro_export]
#[cfg(feature = "kcov")]
macro_rules! stack_trace {
    () => {
        let pc: usize;
        unsafe {
            core::arch::asm!("auipc {}, 0", out(reg) pc);
        }
        $crate::fs::K_COVERAGE.add(pc);
        let _stack_info_guard = $crate::utils::stack_trace::stack_tracker::StackInfoGuard::new(
            $crate::utils::stack_trace::Msg::None,
            file!(),
            line!(),
        );
    };
    // stack_trace!("message")
    ($msg: literal) => {
        let _stack_info_guard = $crate::utils::stack_trace::stack_tracker::StackInfoGuard::new(
            $crate::utils::stack_trace::Msg::Str($msg),
            file!(),
            line!(),
        );
    };
}

/// add the current stack info(i.e. line, file) into stack tracer
#[macro_export]
#[cfg(all(not(feature = "stack_trace"), not(feature = "kcov")))]
macro_rules! stack_trace {
    () => {};
    ($msg: literal) => {};
}

pub enum Msg {
    #[allow(unused)]
    None,
    #[allow(unused)]
    Str(&'static str),
}

impl Display for Msg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::None => write!(f, "(No msg)"),
            Self::Str(str) => write!(f, "{}", str),
        }
    }
}
