#![cfg(feature = "std")]
#![cfg_attr(thiserror_nightly_testing, feature(error_generic_member_access))]

#[cfg(thiserror_nightly_testing)]
pub mod structs {
    use std::backtrace::Backtrace;

    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct OptSourceNoBacktrace {
        #[source]
        pub source: Option<anyerr::Error>,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct OptSourceAlwaysBacktrace {
        #[source]
        pub source: Option<anyerr::Error>,
        pub backtrace: Backtrace,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct NoSourceOptBacktrace {
        #[backtrace]
        pub backtrace: Option<Backtrace>,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct AlwaysSourceOptBacktrace {
        pub source: anyerr::Error,
        #[backtrace]
        pub backtrace: Option<Backtrace>,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct OptSourceOptBacktrace {
        #[source]
        pub source: Option<anyerr::Error>,
        #[backtrace]
        pub backtrace: Option<Backtrace>,
    }
}

#[cfg(thiserror_nightly_testing)]
pub mod enums {
    use std::backtrace::Backtrace;

    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum OptSourceNoBacktrace {
        #[error("...")]
        Test {
            #[source]
            source: Option<anyerr::Error>,
        },
    }

    #[derive(Error, Debug)]
    pub enum OptSourceAlwaysBacktrace {
        #[error("...")]
        Test {
            #[source]
            source: Option<anyerr::Error>,
            backtrace: Backtrace,
        },
    }

    #[derive(Error, Debug)]
    pub enum NoSourceOptBacktrace {
        #[error("...")]
        Test {
            #[backtrace]
            backtrace: Option<Backtrace>,
        },
    }

    #[derive(Error, Debug)]
    pub enum AlwaysSourceOptBacktrace {
        #[error("...")]
        Test {
            source: anyerr::Error,
            #[backtrace]
            backtrace: Option<Backtrace>,
        },
    }

    #[derive(Error, Debug)]
    pub enum OptSourceOptBacktrace {
        #[error("...")]
        Test {
            #[source]
            source: Option<anyerr::Error>,
            #[backtrace]
            backtrace: Option<Backtrace>,
        },
    }
}

#[test]
#[cfg_attr(
    not(thiserror_nightly_testing),
    ignore = "requires `--cfg=thiserror_nightly_testing`"
)]
fn test_option() {}
