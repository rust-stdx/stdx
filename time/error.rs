//! The single error type returned by fallible operations.

use core::fmt;

/// The category of an [`Error`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The input string could not be parsed.
    Parse,
    /// A field, or the result of an operation, is outside the supported range.
    OutOfRange,
    /// A date or time is not valid (for example `February 30`).
    Invalid,
    /// A civil time is ambiguous in the given time zone (a DST "fold").
    Ambiguous,
    /// The requested time zone name is not known.
    UnknownTimeZone,
    /// An RFC 9557 offset disagrees with the supplied time zone.
    OffsetConflict,
    /// The system time zone could not be determined.
    UnknownSystemTimeZone,
    /// A caller-provided buffer was too small for the formatted value.
    BufferTooSmall,
}

/// An error returned by fallible operations in this crate.
///
/// Errors carry a stable [`ErrorKind`] plus a short human-readable message.
/// They are deliberately small and allocation free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    message: &'static str,
}

impl Error {
    pub(crate) const fn new(kind: ErrorKind, message: &'static str) -> Error {
        Error {
            kind,
            message,
        }
    }

    /// Returns the category of this error.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns a short human-readable description of this error.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message)
    }
}

impl core::error::Error for Error {}

pub(crate) const fn parse(message: &'static str) -> Error {
    Error::new(ErrorKind::Parse, message)
}

pub(crate) const fn out_of_range(message: &'static str) -> Error {
    Error::new(ErrorKind::OutOfRange, message)
}

pub(crate) const fn invalid(message: &'static str) -> Error {
    Error::new(ErrorKind::Invalid, message)
}

pub(crate) const fn unknown_time_zone(message: &'static str) -> Error {
    Error::new(ErrorKind::UnknownTimeZone, message)
}

#[cfg(feature = "timezone-system")]
pub(crate) const fn unknown_system_time_zone(message: &'static str) -> Error {
    Error::new(ErrorKind::UnknownSystemTimeZone, message)
}

pub(crate) const fn offset_conflict(message: &'static str) -> Error {
    Error::new(ErrorKind::OffsetConflict, message)
}

pub(crate) const fn ambiguous(message: &'static str) -> Error {
    Error::new(ErrorKind::Ambiguous, message)
}

pub(crate) const fn buffer_too_small(message: &'static str) -> Error {
    Error::new(ErrorKind::BufferTooSmall, message)
}
