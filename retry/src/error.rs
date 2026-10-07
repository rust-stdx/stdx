use core::fmt::{self, Display, Formatter};

/// Error returned when all retry attempts are exhausted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<E> {
    /// The last error that occurred.
    pub error: E,
    /// Total number of attempts made (at least 1).
    pub attempts: usize,
}

impl<E> Error<E> {
    /// Creates a new error with the last error and attempt count.
    pub const fn new(error: E, attempts: usize) -> Self {
        Self {
            error,
            attempts,
        }
    }

    /// Consumes the error and returns the inner error.
    pub fn into_inner(self) -> E {
        self.error
    }

    /// Returns a reference to the inner error.
    pub const fn inner(&self) -> &E {
        &self.error
    }

    /// Returns the number of attempts made.
    pub const fn attempts(&self) -> usize {
        self.attempts
    }
}

impl<E: Display> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "retry failed after {} attempts: {}", self.attempts, self.error)
    }
}

impl<E: Display + core::fmt::Debug> core::error::Error for Error<E> {}
