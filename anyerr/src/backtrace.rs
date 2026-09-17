#[cfg(std_backtrace)]
pub(crate) use std::backtrace::{Backtrace, BacktraceStatus};

#[cfg(not(std_backtrace))]
pub(crate) enum Backtrace {}

#[cfg(std_backtrace)]
macro_rules! impl_backtrace {
    () => {
        std::backtrace::Backtrace
    };
}

#[cfg(std_backtrace)]
macro_rules! backtrace {
    () => {
        Some(crate::backtrace::Backtrace::capture())
    };
}

#[cfg(not(std_backtrace))]
macro_rules! backtrace {
    () => {
        None
    };
}

#[cfg(error_generic_member_access)]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        match std::error::request_ref::<std::backtrace::Backtrace>($err as &dyn std::error::Error) {
            Some(_) => None,
            None => backtrace!(),
        }
    };
}

#[cfg(all(feature = "std", not(error_generic_member_access), std_backtrace))]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        backtrace!()
    };
}

#[cfg(all(feature = "std", not(std_backtrace)))]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        None
    };
}

fn _assert_send_sync() {
    fn _assert<T: Send + Sync>() {}
    _assert::<Backtrace>();
}
