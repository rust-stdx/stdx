//! Error types for parse, syntax, render, and type errors with source positions.

#[cfg(any(feature = "std", test))]
use alloc::string::ToString;
use alloc::{boxed::Box, string::String};
use core::fmt;

#[cfg(feature = "std")]
use serde::ser;

/// A source position (line and column) in a template.
#[derive(Clone, Debug, PartialEq)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

impl SourcePosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    Parse {
        message: String,
    },
    Syntax {
        message: String,
        position: SourcePosition,
    },
    UndefinedVariable {
        name: String,
        position: SourcePosition,
    },
    UndefinedFilter {
        name: String,
        position: SourcePosition,
    },
    UndefinedTemplate {
        name: String,
    },
    Render {
        message: String,
    },
    Type {
        message: String,
    },
    #[cfg(feature = "std")]
    Io(std::io::Error),
}

/// An error returned by the template engine.
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorInner>,
}

#[derive(Debug)]
struct ErrorInner {
    kind: ErrorKind,
    source: Option<String>,
}

impl Error {
    fn new(kind: ErrorKind) -> Self {
        Self {
            inner: Box::new(ErrorInner {
                kind,
                source: None,
            }),
        }
    }

    /// Create a generic parse error.
    pub fn parse(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Parse {
            message: message.into(),
        })
    }

    /// Create a syntax error at a specific position.
    pub fn syntax(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::new(ErrorKind::Syntax {
            message: message.into(),
            position: SourcePosition::new(line, column),
        })
    }

    /// Create an error for an undefined variable.
    pub fn undefined_variable(name: impl Into<String>, line: usize, column: usize) -> Self {
        Self::new(ErrorKind::UndefinedVariable {
            name: name.into(),
            position: SourcePosition::new(line, column),
        })
    }

    /// Create an error for an undefined filter.
    pub fn undefined_filter(name: impl Into<String>, line: usize, column: usize) -> Self {
        Self::new(ErrorKind::UndefinedFilter {
            name: name.into(),
            position: SourcePosition::new(line, column),
        })
    }

    /// Create an error for an undefined template name.
    pub fn undefined_template(name: impl Into<String>) -> Self {
        Self::new(ErrorKind::UndefinedTemplate {
            name: name.into(),
        })
    }

    /// Create a generic render error.
    pub fn render(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Render {
            message: message.into(),
        })
    }

    /// Create a type error.
    pub fn r#type(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Type {
            message: message.into(),
        })
    }

    /// Attach source information to the error.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.inner.source = Some(source.into());
        self
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::new(ErrorKind::Io(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner.kind {
            ErrorKind::Parse {
                message,
            } => {
                write!(f, "parse error: {message}")
            }
            ErrorKind::Syntax {
                message,
                position,
            } => {
                write!(f, "syntax error at {}:{}: {message}", position.line, position.column)
            }
            ErrorKind::UndefinedVariable {
                name,
                position,
            } => {
                write!(f, "undefined variable `{name}` at {}:{}", position.line, position.column)
            }
            ErrorKind::UndefinedFilter {
                name,
                position,
            } => {
                write!(f, "undefined filter `{name}` at {}:{}", position.line, position.column)
            }
            ErrorKind::UndefinedTemplate {
                name,
            } => {
                write!(f, "undefined template `{name}`")
            }
            ErrorKind::Render {
                message,
            } => {
                write!(f, "render error: {message}")
            }
            ErrorKind::Type {
                message,
            } => {
                write!(f, "type error: {message}")
            }
            #[cfg(feature = "std")]
            ErrorKind::Io(err) => {
                write!(f, "I/O error: {err}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.inner.kind {
            ErrorKind::Io(err) => Some(err),
            _ => None,
        }
    }
}

/// Error returned when serializing a value fails.
#[derive(Debug)]
pub struct SerdeError(pub String);

impl fmt::Display for SerdeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SerdeError {}

#[cfg(feature = "std")]
impl ser::Error for SerdeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeError(msg.to_string())
    }
}
