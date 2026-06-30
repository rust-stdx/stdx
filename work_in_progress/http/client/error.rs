use std::fmt;

use crate::common::Version;

#[derive(Debug)]
pub enum Error {
    Dns(String),
    Connect(String),
    #[cfg(feature = "tls")]
    Tls(String),
    Io(String),
    #[cfg(feature = "http1")]
    H1(String),
    #[cfg(feature = "http2")]
    H2(String),
    #[cfg(feature = "http3")]
    H3(String),
    UnsupportedScheme(String),
    UnsupportedVersion(Version),
    PoolClosed,
    ConnectionClosed,
    DriverTerminated,
    BodyError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Dns(s) => write!(f, "DNS resolution failed: {s}"),
            Error::Connect(s) => write!(f, "connection failed: {s}"),
            #[cfg(feature = "tls")]
            Error::Tls(s) => write!(f, "TLS error: {s}"),
            Error::Io(s) => write!(f, "I/O error: {s}"),
            #[cfg(feature = "http1")]
            Error::H1(s) => write!(f, "HTTP/1 error: {s}"),
            #[cfg(feature = "http2")]
            Error::H2(s) => write!(f, "HTTP/2 error: {s}"),
            #[cfg(feature = "http3")]
            Error::H3(s) => write!(f, "HTTP/3 error: {s}"),
            Error::UnsupportedScheme(s) => write!(f, "unsupported URI scheme: {s}"),
            Error::UnsupportedVersion(v) => write!(f, "unsupported HTTP version: {v:?}"),
            Error::PoolClosed => write!(f, "connection pool is closed"),
            Error::ConnectionClosed => write!(f, "connection closed before response completed"),
            Error::DriverTerminated => write!(f, "connection driver task terminated unexpectedly"),
            Error::BodyError(s) => write!(f, "body error: {s}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}
