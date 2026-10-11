#![cfg_attr(docsrs, feature(doc_cfg))]

//! Loads environment variables from `.env` files.
//!
//! # Quick start
//!
//! ```rust,no_run
//! fn main() -> Result<(), dotenv::Error> {
//!     dotenv::load()?;
//!     Ok(())
//! }
//! ```
//!
//! Call [`load`] near the start of your program to load a `.env` file
//! from the current working directory.
//!
//! # Precedence
//!
//! - **Existing environment variables are never overwritten.** A variable
//!   already set in the environment takes priority over anything in `.env`.
//! - **First declaration wins in `.env`.** If the same key appears multiple
//!   times, only the first is used.
//! - **Loading is atomic.** If the file contains a parse error, no environment
//!   variable is modified, even for lines that appear before the error.
//!
//! # Supported syntax
//!
//! ```env
//! HELLO=world
//! HELLO="world"
//! HELLO='world'
//! HELLO='"nested"'
//! HELLO=world  # inline comment
//! # full-line comment
//! ```
//!
//! ## Key names
//!
//! Keys may only contain ASCII letters, digits, `_`, `.`, and `-`. A key may
//! start with any of those characters (including a digit, `.` or `-`).
//!
//! ## Comments
//!
//! A `#` starts an inline comment only when it is preceded by whitespace
//! (`K=v # comment` strips the comment, but `K=v#comment` keeps the `#`).
//!
//! ## Whitespace
//!
//! Leading whitespace on a line and whitespace around `=` are ignored. An
//! unquoted value is trimmed; whitespace inside an unquoted value is kept
//! (`K=hello world` yields `hello world`).
//!
//! ## Quotes and escapes
//!
//! - Single-quoted values (`'...'`) are literal: no escapes are processed.
//! - Double-quoted values (`"..."`) process the escape sequences `\\`, `\'`,
//!   `\"`, `\$`, `\n`, `\t`, and `\r`. Any other backslash sequence is a
//!   [`ParseErrorKind::InvalidEscape`] error.
//! - Backslashes in unquoted values are literal (`K=C:\path` yields `C:\path`).
//! - A UTF-8 byte-order mark (BOM) at the very start of the file is ignored.
//!
//! ## Limitations
//!
//! - Multi-line values are not supported.
//! - Variable substitution (e.g. `${FOO}`) is not supported.
//! - Export syntax (`export KEY=value`) is not supported.
//! - Only `\n` and `\r\n` are recognised as line endings; a lone `\r` is not a
//!   line break.
//! - Values may not contain a NUL byte (`\0`), which cannot be placed in the
//!   process environment.
//!
//! # Thread safety
//!
//! [`load`] and [`from_path`] call [`std::env::set_var`] internally, which is
//! unsound to call while other threads are running or reading the environment.
//! **Call them before spawning any threads** (typically at the very start of
//! `main`). If you only need to inspect `.env` content, use [`parse`], which
//! does not touch the process environment.
//!
//! # Performance
//!
//! `.env` files are configuration-sized. Because the platform `setenv` costs
//! `O(n)` per variable once `n` variables are set, loading an extremely large
//! number of variables is quadratic overall. Do not feed untrusted,
//! arbitrarily large files to [`load`]; validate them with [`parse`] first.
//!
//! # Deserializing into structs with `FromEnv`
//!
//! This crate provides a [`FromEnv`] trait (and a [`#[derive(FromEnv)]`](FromEnv)
//! proc-macro) for constructing typed structs directly from environment
//! variables.
//!
//! ## Basic usage
//!
//! ```rust,ignore
//! use dotenv::FromEnv;
//!
//! #[derive(FromEnv)]
//! struct Config {
//!     #[env(rename = "MY_HOST")]
//!     host: String,
//!     #[env(rename = "MY_PORT")]
//!     port: u16,
//! }
//!
//! let cfg = Config::from_env().unwrap();
//! ```
//!
//! ## Default values
//!
//! Use `#[env(default)]` for any type that implements [`Default`] (yields the
//! default when the variable is not set) or `#[env(default = expr)]` for any
//! type. The environment variable is always read first; the default is only
//! used when it is not set.
//!
//! ```rust,ignore
//! #[derive(FromEnv)]
//! struct Config {
//!     #[env(rename = "MY_HOST")]
//!     host: String,
//!     #[env(rename = "MY_VERBOSE", default)]
//!     verbose: Option<bool>,
//!     #[env(default)]
//!     timeout: u64,             // reads TIMEOUT, falls back to 0
//!     #[env(rename = "MY_PORT", default = 8080)]
//!     port: u16,
//! }
//! ```
//!
//! `Option<T>` fields are always optional: an unset (or empty) variable yields
//! `None`, otherwise the value is parsed and wrapped in `Some`.
//!
//! ```rust,ignore
//! #[derive(FromEnv)]
//! struct Config {
//!     #[env(rename = "MY_PORT")]
//!     port: Option<u16>,        // None when MY_PORT is unset or empty
//! }
//! ```
//!
//! ## Custom parsers
//!
//! When the standard [`FromStr`] parsing is insufficient, provide a custom
//! parser via `#[env(with = "func")]`. The function may be a full path
//! (`module::func`) and must have the signature `fn(&str, &str) -> Result<T, FromEnvError>`:
//!
//! ```rust,ignore
//! use dotenv::{FromEnv, FromEnvError};
//!
//! fn parse_port(_var: &str, val: &str) -> Result<u16, FromEnvError> {
//!     val.parse().map_err(|e| FromEnvError::invalid("MY_PORT", val, e))
//! }
//!
//! #[derive(FromEnv)]
//! struct Config {
//!     #[env(rename = "MY_PORT", with = "parse_port")]
//!     port: u16,
//! }
//! ```
//!
//! ## Nested structs
//!
//! Fields whose type also implements [`FromEnv`] are automatically detected
//! and populated as nested structs. The parent's field name (in
//! `SCREAMING_SNAKE_CASE`) is used as a prefix so that child fields are read
//! from prefixed variable names:
//!
//! ```rust,ignore
//! #[derive(FromEnv)]
//! struct AppConfig {
//!     database: Database,
//!     debug: bool,       // reads DEBUG
//! }
//!
//! #[derive(FromEnv)]
//! struct Database {
//!     url: String,       // reads DATABASE_URL
//!     pool_size: u32,    // reads DATABASE_POOL_SIZE
//! }
//! ```
//!
//! ## Custom `FromEnvValue` implementations
//!
//! For leaf types that need special parsing logic, implement [`FromEnvValue`]
//! directly on your type. Types that implement [`FromStr`] get a blanket impl
//! automatically.
//!
//! ## Compile-time diagnostics
//!
//! Invalid usage is reported as an ordinary compile error rather than a macro
//! panic:
//!
//! ```compile_fail
//! use dotenv::FromEnv;
//!
//! // Tuple structs are not supported.
//! #[derive(FromEnv)]
//! struct Config(u16);
//! ```
//!
//! ```compile_fail
//! use dotenv::FromEnv;
//!
//! // Multiple `#[env(...)]` attributes on one field are rejected.
//! #[derive(FromEnv)]
//! struct Config {
//!     #[env(rename = "A")]
//!     #[env(rename = "B")]
//!     value: u16,
//! }
//! ```
//!
//! [`#[env(default)]`](FromEnv) without a `rename` reads the variable derived
//! from the field name and requires the field type to implement
//! [`FromEnvAuto`] (a built-in leaf type, or a nested [`FromEnv`] struct);
//! `#[env(rename = "...", default)]` reads the named variable via
//! [`FromEnvValue`].
//!
//! ## Convenience function
//!
//! The function [`from_env`] lets you avoid importing the trait:
//!
//! ```rust,ignore
//! let cfg: Config = dotenv::from_env().unwrap();
//! ```

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    env,
    ffi::{OsStr, OsString},
    fmt,
    fmt::Display,
    fs, io,
    path::Path,
    str::FromStr,
};

use memchr::memchr;

/// Errors that can occur when loading a `.env` file.
#[derive(Debug)]
pub enum Error {
    /// An I/O error (file not found, permissions, invalid UTF-8, etc.).
    Io(io::Error),
    /// A parse error on a specific line.
    Parse(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "dotenv I/O error: {e}"),
            Error::Parse(e) => write!(f, "dotenv parse error at line {}: {}", e.line, e.kind),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Parse(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

/// A parse error with the line number and kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// The 1-indexed line number where the error occurred.
    pub line: usize,
    /// The kind of parse error.
    pub kind: ParseErrorKind,
}

/// The specific kind of parse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// A line without an `=` sign.
    MissingEquals,
    /// A quoted value (`"..."` or `'...'`) without a closing quote.
    UnmatchedQuote,
    /// A line with an empty key before the `=` sign.
    EmptyKey,
    /// A key containing characters outside the allowed set
    /// (alphanumeric, `_`, `.`, `-`).
    InvalidKey,
    /// Extra content found after a closing quote.
    TrailingContent,
    /// A value containing a NUL byte, which cannot be stored in the process
    /// environment.
    InvalidValue,
    /// An unrecognised escape sequence inside a double-quoted value.
    InvalidEscape,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::MissingEquals => f.write_str("missing equals sign"),
            ParseErrorKind::UnmatchedQuote => f.write_str("unmatched quote"),
            ParseErrorKind::EmptyKey => f.write_str("empty key"),
            ParseErrorKind::InvalidKey => f.write_str("invalid key character"),
            ParseErrorKind::TrailingContent => f.write_str("trailing content after closing quote"),
            ParseErrorKind::InvalidValue => f.write_str("value contains a NUL byte"),
            ParseErrorKind::InvalidEscape => f.write_str("invalid escape sequence"),
        }
    }
}

/// Loads the `.env` file from the current working directory.
///
/// Each key-value pair found in the file is set as an environment variable
/// for the current process, subject to these rules:
///
/// 1. A variable already present in the environment is not overwritten.
/// 2. When the same key appears multiple times in `.env`, the first
///    declaration takes effect.
/// 3. If the file is malformed, no variable is modified.
///
/// # Errors
///
/// Returns [`Error`] if the file cannot be read (missing, permissions,
/// invalid UTF-8, etc.) or if the `.env` file is malformed.
///
/// # Thread safety
///
/// This function modifies the process environment. Call it before spawning any
/// threads; see the [crate-level documentation](crate#thread-safety).
///
/// # Example
///
/// ```rust,no_run
/// if let Err(e) = dotenv::load() {
///     eprintln!("Failed to load .env: {e}");
/// }
/// ```
pub fn load() -> Result<(), Error> {
    let mut path = env::current_dir()?;
    path.push(".env");
    from_path(path)
}

/// Loads a `.env` file from an explicit path.
///
/// Behaves like [`load`] but reads from `path` instead of `./.env`. See
/// [`load`] for the precedence rules and the thread-safety requirements.
///
/// # Errors
///
/// Returns [`Error`] if the file cannot be read or is malformed.
///
/// # Example
///
/// ```rust,no_run
/// fn main() -> Result<(), dotenv::Error> {
///     dotenv::from_path("config/.env")?;
///     Ok(())
/// }
/// ```
pub fn from_path(path: impl AsRef<Path>) -> Result<(), Error> {
    let content = fs::read_to_string(path)?;
    // `parse` resolves duplicate keys (first declaration wins) and returns
    // before any environment variable is touched, so loading stays atomic.
    let vars = parse(&content).map_err(Error::Parse)?;

    // Snapshot of the pre-existing variable *names*. `vars_os` is used (rather
    // than `vars`) so that variables whose keys or values are not valid UTF-8
    // do not cause a panic.
    let existing: HashSet<OsString> = env::vars_os().map(|(k, _)| k).collect();

    for (key, value) in &vars {
        if !existing.contains(OsStr::new(*key)) {
            // SAFETY: the caller is required to invoke this before any other
            // thread runs or reads the environment (see the crate docs).
            unsafe { env::set_var(*key, value.as_ref()) };
        }
    }
    Ok(())
}

/// Parses `.env` content into a map of key/value pairs.
///
/// Duplicate keys follow the same rule as [`load`]: the **first** declaration
/// wins. Iteration order of the returned map is unspecified. Escape sequences
/// in double-quoted values are expanded (see the [crate-level
/// documentation](crate#quotes-and-escapes)).
///
/// Keys and values borrow from `input`. A value that contains escape
/// sequences is materialised and returned as [`Cow::Owned`]; every other value
/// is a [`Cow::Borrowed`] slice of `input`. Use [`Cow::as_ref`] (or `.as_ref()`)
/// to obtain a plain `&str`, and convert with
/// `into_iter().map(|(k, v)| (k.to_owned(), v.into_owned()))` if the map must
/// outlive `input`.
///
/// This function does not touch the process environment, so it is safe to call
/// from any thread and useful for validating or inspecting `.env` content.
///
/// # Errors
///
/// Returns [`ParseError`] if the content is malformed.
///
/// # Example
///
/// ```
/// let vars = dotenv::parse("A=1\nB=two\nA=ignored").unwrap();
/// assert_eq!(vars["A"].as_ref(), "1");
/// assert_eq!(vars["B"].as_ref(), "two");
/// assert_eq!(vars.len(), 2);
/// ```
pub fn parse(input: &str) -> Result<HashMap<&str, Cow<'_, str>>, ParseError> {
    // Ignore a UTF-8 BOM at the very start of the file.
    let input = input.strip_prefix('\u{FEFF}').unwrap_or(input);
    let mut map: HashMap<&str, Cow<'_, str>> = HashMap::new();

    for (line_idx, raw_line) in input.lines().enumerate() {
        let line_no = line_idx + 1;
        let line = raw_line.trim_start();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let eq_pos = memchr(b'=', line.as_bytes()).ok_or(ParseError {
            line: line_no,
            kind: ParseErrorKind::MissingEquals,
        })?;

        let key = line[..eq_pos].trim_end();
        let value_str = &line[eq_pos + 1..];

        if key.is_empty() {
            return Err(ParseError {
                line: line_no,
                kind: ParseErrorKind::EmptyKey,
            });
        }

        if !key
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'.' || c == b'-')
        {
            return Err(ParseError {
                line: line_no,
                kind: ParseErrorKind::InvalidKey,
            });
        }

        let value = parse_value(value_str, line_no)?;
        // First declaration wins: `entry` only inserts when the key is absent.
        map.entry(key).or_insert(value);
    }

    Ok(map)
}

/// Find the first `#` preceded by whitespace (indicating a comment start).
fn find_comment_start(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut offset = 0;
    while let Some(pos) = memchr(b'#', &bytes[offset..]) {
        let abs = offset + pos;
        if abs > 0 && bytes[abs - 1].is_ascii_whitespace() {
            return Some(abs);
        }
        offset = abs + 1;
    }
    None
}

/// Parse a single value string (everything after `=`).
fn parse_value(s: &str, line: usize) -> Result<Cow<'_, str>, ParseError> {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return Ok(Cow::Borrowed(""));
    }

    match trimmed.as_bytes()[0] {
        b'"' => parse_double_quoted(trimmed, line),
        b'\'' => parse_single_quoted(trimmed, line),
        _ => {
            let comment_start = find_comment_start(s);
            let val = match comment_start {
                Some(pos) => &s[..pos],
                None => s,
            };
            let val = val.trim();
            if val.as_bytes().contains(&0) {
                return Err(ParseError {
                    line,
                    kind: ParseErrorKind::InvalidValue,
                });
            }
            Ok(Cow::Borrowed(val))
        }
    }
}

/// Parse a double-quoted value, finding the closing quote while honouring
/// backslash escapes, then expanding the escapes.
fn parse_double_quoted(trimmed: &str, line: usize) -> Result<Cow<'_, str>, ParseError> {
    let content = &trimmed[1..];

    let mut escaped = false;
    let mut close = None;
    for (i, &b) in content.as_bytes().iter().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        match b {
            b'\\' => escaped = true,
            b'"' => {
                close = Some(i);
                break;
            }
            _ => {}
        }
    }
    let close = close.ok_or(ParseError {
        line,
        kind: ParseErrorKind::UnmatchedQuote,
    })?;

    let raw_content = &content[..close];
    let after = content[close + 1..].trim();
    if !after.is_empty() && !after.starts_with('#') {
        return Err(ParseError {
            line,
            kind: ParseErrorKind::TrailingContent,
        });
    }

    let value = expand_escapes(raw_content, line)?;
    if value.as_bytes().contains(&0) {
        return Err(ParseError {
            line,
            kind: ParseErrorKind::InvalidValue,
        });
    }
    Ok(value)
}

/// Parse a single-quoted value. Single quotes are literal: no escapes.
fn parse_single_quoted(trimmed: &str, line: usize) -> Result<Cow<'_, str>, ParseError> {
    let content = &trimmed[1..];
    let close = memchr(b'\'', content.as_bytes()).ok_or(ParseError {
        line,
        kind: ParseErrorKind::UnmatchedQuote,
    })?;

    let raw = &content[..close];
    let after = content[close + 1..].trim();
    if !after.is_empty() && !after.starts_with('#') {
        return Err(ParseError {
            line,
            kind: ParseErrorKind::TrailingContent,
        });
    }

    if raw.as_bytes().contains(&0) {
        return Err(ParseError {
            line,
            kind: ParseErrorKind::InvalidValue,
        });
    }
    Ok(Cow::Borrowed(raw))
}

/// Expand backslash escape sequences inside a double-quoted value.
///
/// Supported: `\\`, `\'`, `\"`, `\$`, `\n`, `\t`, `\r`. Any other sequence is
/// an [`ParseErrorKind::InvalidEscape`] error.
fn expand_escapes(raw: &str, line: usize) -> Result<Cow<'_, str>, ParseError> {
    if !raw.as_bytes().contains(&b'\\') {
        return Ok(Cow::Borrowed(raw));
    }

    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('\'') => out.push('\''),
            Some('"') => out.push('"'),
            Some('$') => out.push('$'),
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            // A trailing backslash cannot occur here (the closing-quote scan
            // would have consumed it), but report it rather than panicking.
            Some(_) | None => {
                return Err(ParseError {
                    line,
                    kind: ParseErrorKind::InvalidEscape,
                });
            }
        }
    }
    Ok(Cow::Owned(out))
}

// ---------------------------------------------------------------------------
// FromEnv trait and derive support
// ---------------------------------------------------------------------------

pub use dotenv_derive::FromEnv;

/// Error returned by [`FromEnv::from_env`].
///
/// # Example
///
/// ```
/// # use dotenv::FromEnvError;
/// let err = FromEnvError::missing("MY_VAR");
/// assert_eq!(err.to_string(), "environment variable `MY_VAR` is not set");
///
/// let err = FromEnvError::invalid("PORT", "abc", "invalid digit");
/// assert_eq!(
///     err.to_string(),
///     "environment variable `PORT` has invalid value `abc`: invalid digit"
/// );
/// ```
#[derive(Debug, Clone)]
pub enum FromEnvError {
    /// An environment variable was not set.
    Missing(String),
    /// An environment variable was set but could not be parsed.
    Invalid {
        /// The name of the environment variable.
        var: String,
        /// The raw value of the environment variable.
        value: String,
        /// A description of why parsing failed.
        message: String,
    },
}

impl fmt::Display for FromEnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FromEnvError::Missing(var) => write!(f, "environment variable `{var}` is not set"),
            FromEnvError::Invalid {
                var,
                value,
                message,
            } => {
                write!(f, "environment variable `{var}` has invalid value `{value}`: {message}")
            }
        }
    }
}

impl core::error::Error for FromEnvError {}

impl FromEnvError {
    /// Construct a [`FromEnvError::Missing`] error.
    pub fn missing(var: impl Into<String>) -> Self {
        FromEnvError::Missing(var.into())
    }

    /// Construct a [`FromEnvError::Invalid`] error.
    pub fn invalid(var: impl Into<String>, value: impl Into<String>, message: impl Into<String>) -> Self {
        FromEnvError::Invalid {
            var: var.into(),
            value: value.into(),
            message: message.into(),
        }
    }
}

/// Trait for types that can be constructed from environment variables.
///
/// Usually derived with [`#[derive(FromEnv)]`](FromEnv).
pub trait FromEnv: Sized {
    /// Load `Self` from environment variables using an empty prefix.
    fn from_env() -> Result<Self, FromEnvError> {
        Self::from_env_with_prefix("")
    }

    /// Load `Self` from environment variables, prepending `prefix` to each
    /// env-var name derived from field names.
    ///
    /// This is used internally to support nested structs. Each parent field
    /// passes its own SCREAMING_SNAKE name plus `_` as the child's prefix.
    fn from_env_with_prefix(prefix: &str) -> Result<Self, FromEnvError>;
}

/// Trait for converting a raw env-var string into a typed value.
///
/// Implementations are provided for all [`FromStr`] types via a blanket impl.
/// You can implement this trait for custom types that need special parsing.
///
/// # Example
///
/// ```
/// use dotenv::FromEnvValue;
///
/// let n = <u16 as FromEnvValue>::from_env_value("42").unwrap();
/// assert_eq!(n, 42);
///
/// let b = <bool as FromEnvValue>::from_env_value("true").unwrap();
/// assert!(b);
///
/// let err = <u16 as FromEnvValue>::from_env_value("abc").unwrap_err();
/// assert!(!err.is_empty());
/// ```
pub trait FromEnvValue: Sized {
    /// Convert the raw string value into `Self`.
    fn from_env_value(s: &str) -> Result<Self, String>;
}

impl<T: FromStr> FromEnvValue for T
where
    T::Err: Display,
{
    fn from_env_value(s: &str) -> Result<Self, String> {
        s.parse::<T>().map_err(|e| e.to_string())
    }
}

/// Auto-dispatch trait for un-attributed `#[derive(FromEnv)]` fields.
///
/// For types that implement [`FromEnv`] (nested structs), calls
/// `from_env_with_prefix`. For leaf types listed in the built-in impls,
/// reads the env var and parses via [`FromStr`].
///
/// You should not need to implement this trait directly.
pub trait FromEnvAuto: Sized {
    /// Read and parse the variable `var_name`, or load a nested struct using
    /// `prefix`.
    fn from_env_auto(prefix: &str, var_name: &str) -> Result<Self, FromEnvError>;
}

impl<T: FromEnv> FromEnvAuto for T {
    fn from_env_auto(prefix: &str, _var_name: &str) -> Result<Self, FromEnvError> {
        Self::from_env_with_prefix(prefix)
    }
}

macro_rules! impl_from_env_auto_leaf {
    ($($t:ty),* $(,)?) => {
        $(impl FromEnvAuto for $t {
            fn from_env_auto(_prefix: &str, var_name: &str) -> Result<Self, FromEnvError> {
                let val = ::std::env::var(var_name)
                    .map_err(|_| FromEnvError::missing(var_name))?;
                <Self as FromEnvValue>::from_env_value(&val)
                    .map_err(|e| FromEnvError::invalid(var_name, val, e))
            }
        })*
    };
}

impl_from_env_auto_leaf!(String, bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,);

/// Convenience function to load a `FromEnv` type from environment variables.
///
/// Equivalent to `<T as FromEnv>::from_env()` but doesn't require importing
/// the `FromEnv` trait.
///
/// # Example
///
/// ```rust,ignore
/// let config: Config = dotenv::from_env().unwrap();
/// ```
pub fn from_env<T: FromEnv>() -> Result<T, FromEnvError> {
    T::from_env()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Serialize tests that mutate the process-wide environment or current
    // directory. `cargo test` runs tests in parallel threads by default.
    #[cfg(not(target_arch = "wasm32"))]
    static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    // `parse` returns an unordered map; sort the resolved pairs so that
    // assertions below are deterministic.
    fn parse_ok(input: &str) -> Vec<(String, String)> {
        let mut pairs: Vec<(String, String)> = parse(input)
            .unwrap()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.into_owned()))
            .collect();
        pairs.sort();
        pairs
    }

    fn parse_kind(input: &str) -> ParseErrorKind {
        parse(input).unwrap_err().kind
    }

    fn parse_line(input: &str) -> usize {
        parse(input).unwrap_err().line
    }

    // Unsafe helpers for tests. Tests that use them hold `TEST_LOCK`.
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn set_env(k: &str, v: &str) {
        unsafe { env::set_var(k, v) };
    }

    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn remove_env(k: &str) {
        unsafe { env::remove_var(k) };
    }

    // ── Basic parsing ──────────────────────────────────────────────────────

    #[test]
    fn simple_key_value() {
        assert_eq!(parse_ok("K=v"), vec![("K".into(), "v".into())]);
    }

    #[test]
    fn multiple_pairs() {
        let pairs = parse_ok("A=1\nB=2\nC=3");
        assert_eq!(
            pairs,
            vec![
                ("A".into(), "1".into()),
                ("B".into(), "2".into()),
                ("C".into(), "3".into()),
            ]
        );
    }

    #[test]
    fn value_with_equals() {
        assert_eq!(parse_ok("K=a=b=c"), vec![("K".into(), "a=b=c".into())]);
    }

    #[test]
    fn key_with_underscore() {
        assert_eq!(parse_ok("MY_KEY=val"), vec![("MY_KEY".into(), "val".into())]);
    }

    #[test]
    fn key_with_dot() {
        assert_eq!(parse_ok("my.key=val"), vec![("my.key".into(), "val".into())]);
    }

    #[test]
    fn key_with_hyphen() {
        assert_eq!(parse_ok("my-key=val"), vec![("my-key".into(), "val".into())]);
    }

    #[test]
    fn key_with_digits() {
        assert_eq!(parse_ok("KEY123=val"), vec![("KEY123".into(), "val".into())]);
    }

    #[test]
    fn key_mixed() {
        assert_eq!(parse_ok("A1.b-C_2=val"), vec![("A1.b-C_2".into(), "val".into())]);
    }

    #[test]
    fn key_starting_with_hyphen() {
        assert_eq!(parse_ok("-KEY=v"), vec![("-KEY".into(), "v".into())]);
    }

    #[test]
    fn key_starting_with_dot() {
        assert_eq!(parse_ok(".KEY=v"), vec![(".KEY".into(), "v".into())]);
    }

    #[test]
    fn key_starting_with_underscore() {
        assert_eq!(parse_ok("_KEY=v"), vec![("_KEY".into(), "v".into())]);
    }

    #[test]
    fn key_only_dots() {
        assert_eq!(parse_ok("...=value"), vec![("...".into(), "value".into())]);
    }

    #[test]
    fn key_only_hyphens() {
        assert_eq!(parse_ok("---=value"), vec![("---".into(), "value".into())]);
    }

    // ── Double-quoted values ───────────────────────────────────────────────

    #[test]
    fn double_quoted_value() {
        assert_eq!(parse_ok("K=\"hello\""), vec![("K".into(), "hello".into())]);
    }

    #[test]
    fn double_quoted_with_spaces() {
        assert_eq!(parse_ok("K=\"hello world\""), vec![("K".into(), "hello world".into())]);
    }

    #[test]
    fn double_quoted_empty() {
        assert_eq!(parse_ok("K=\"\""), vec![("K".into(), "".into())]);
    }

    #[test]
    fn double_quoted_hash_preserved() {
        assert_eq!(parse_ok("K=\"a#b\""), vec![("K".into(), "a#b".into())]);
    }

    #[test]
    fn double_quoted_equals_inside() {
        assert_eq!(parse_ok("K=\"a=b\""), vec![("K".into(), "a=b".into())]);
    }

    #[test]
    fn double_quoted_single_quotes_inside() {
        assert_eq!(parse_ok("K=\"it's ok\""), vec![("K".into(), "it's ok".into())]);
    }

    #[test]
    fn double_quoted_whitespace_preserved() {
        assert_eq!(parse_ok("K=\" hello \""), vec![("K".into(), " hello ".into())]);
    }

    #[test]
    fn double_quoted_trailing_content_error() {
        assert_eq!(parse_kind("K=\"hello\"extra"), ParseErrorKind::TrailingContent);
    }

    #[test]
    fn double_quoted_trailing_comment_allowed() {
        assert_eq!(parse_ok("K=\"hello\" # comment"), vec![("K".into(), "hello".into())]);
    }

    #[test]
    fn double_quoted_trailing_whitespace_allowed() {
        assert_eq!(parse_ok("K=\"hello\"   "), vec![("K".into(), "hello".into())]);
    }

    // ── Escape sequences (double quotes only) ──────────────────────────────

    #[test]
    fn escaped_newline() {
        assert_eq!(parse_ok("K=\"a\\nb\""), vec![("K".into(), "a\nb".into())]);
    }

    #[test]
    fn escaped_tab_and_cr() {
        assert_eq!(parse_ok("K=\"a\\tb\\rc\""), vec![("K".into(), "a\tb\rc".into())]);
    }

    #[test]
    fn escaped_quote() {
        assert_eq!(parse_ok("K=\"a\\\"b\""), vec![("K".into(), "a\"b".into())]);
    }

    #[test]
    fn escaped_backslash() {
        assert_eq!(parse_ok("K=\"a\\\\b\""), vec![("K".into(), "a\\b".into())]);
    }

    #[test]
    fn escaped_single_quote_and_dollar() {
        assert_eq!(parse_ok("K=\"\\'\\$\""), vec![("K".into(), "'$".into())]);
    }

    #[test]
    fn escaped_quote_at_end() {
        assert_eq!(parse_ok("K=\"a\\\"\""), vec![("K".into(), "a\"".into())]);
    }

    #[test]
    fn invalid_escape_is_error() {
        assert_eq!(parse_kind("K=\"a\\qb\""), ParseErrorKind::InvalidEscape);
        assert_eq!(parse_kind("K=\"a\\8\""), ParseErrorKind::InvalidEscape);
    }

    #[test]
    fn backslash_literal_in_single_quotes() {
        assert_eq!(parse_ok("K='C:\\path\\to'"), vec![("K".into(), "C:\\path\\to".into())]);
    }

    #[test]
    fn backslash_literal_unquoted() {
        assert_eq!(parse_ok("K=C:\\path\\to"), vec![("K".into(), "C:\\path\\to".into())]);
    }

    #[test]
    fn escape_error_line_number() {
        assert_eq!(parse_line("A=1\nB=\"x\\q\""), 2);
    }

    // ── Single-quoted values ───────────────────────────────────────────────

    #[test]
    fn single_quoted_value() {
        assert_eq!(parse_ok("K='hello'"), vec![("K".into(), "hello".into())]);
    }

    #[test]
    fn single_quoted_with_spaces() {
        assert_eq!(parse_ok("K='hello world'"), vec![("K".into(), "hello world".into())]);
    }

    #[test]
    fn single_quoted_empty() {
        assert_eq!(parse_ok("K=''"), vec![("K".into(), "".into())]);
    }

    #[test]
    fn single_quoted_hash_preserved() {
        assert_eq!(parse_ok("K='a#b'"), vec![("K".into(), "a#b".into())]);
    }

    #[test]
    fn single_quoted_double_quotes_inside() {
        assert_eq!(parse_ok(r#"K='"hello"'"#), vec![("K".into(), r#""hello""#.into())]);
    }

    #[test]
    fn single_quoted_whitespace_preserved() {
        assert_eq!(parse_ok("K=' hello '"), vec![("K".into(), " hello ".into())]);
    }

    #[test]
    fn single_quoted_trailing_content_error() {
        assert_eq!(parse_kind("K='hello'extra"), ParseErrorKind::TrailingContent);
    }

    #[test]
    fn single_quoted_trailing_comment_allowed() {
        assert_eq!(parse_ok("K='hello' # comment"), vec![("K".into(), "hello".into())]);
    }

    // ── Quoted example from the spec ────────────────────────────────────────

    #[test]
    fn quoted_nested_example() {
        assert_eq!(parse_ok("HELLO='\"hello\"'"), vec![("HELLO".into(), "\"hello\"".into())]);
    }

    // ── Unquoted values ────────────────────────────────────────────────────

    #[test]
    fn unquoted_hash_is_comment() {
        assert_eq!(parse_ok("K=val # comment"), vec![("K".into(), "val".into())]);
    }

    #[test]
    fn unquoted_hash_no_space_not_comment() {
        assert_eq!(parse_ok("K=val#comment"), vec![("K".into(), "val#comment".into())]);
    }

    #[test]
    fn unquoted_trimmed() {
        assert_eq!(parse_ok("K=  val  "), vec![("K".into(), "val".into())]);
    }

    #[test]
    fn unquoted_value_with_internal_spaces() {
        assert_eq!(parse_ok("K=hello world"), vec![("K".into(), "hello world".into())]);
    }

    #[test]
    fn unquoted_trailing_spaces_before_comment() {
        assert_eq!(parse_ok("K=val   # comment"), vec![("K".into(), "val".into())]);
    }

    #[test]
    fn unquoted_value_with_numbers() {
        assert_eq!(parse_ok("PORT=8080"), vec![("PORT".into(), "8080".into())]);
    }

    #[test]
    fn unquoted_value_with_dots() {
        assert_eq!(parse_ok("HOST=192.168.1.1"), vec![("HOST".into(), "192.168.1.1".into())]);
    }

    #[test]
    fn unquoted_value_containing_quote() {
        assert_eq!(parse_ok("K=hello\"there"), vec![("K".into(), "hello\"there".into())]);
    }

    #[test]
    fn unquoted_value_containing_only_hash() {
        assert_eq!(parse_ok("K=#"), vec![("K".into(), "#".into())]);
    }

    #[test]
    fn unquoted_value_hash_without_preceding_space() {
        assert_eq!(parse_ok("K=val#ue"), vec![("K".into(), "val#ue".into())]);
    }

    #[test]
    fn unquoted_hash_with_preceding_space_is_comment() {
        assert_eq!(parse_ok("K=val #ue"), vec![("K".into(), "val".into())]);
    }

    // ── Empty values ───────────────────────────────────────────────────────

    #[test]
    fn empty_value_no_quotes() {
        assert_eq!(parse_ok("K="), vec![("K".into(), "".into())]);
    }

    #[test]
    fn empty_value_trailing_spaces() {
        assert_eq!(parse_ok("K=   "), vec![("K".into(), "".into())]);
    }

    #[test]
    fn empty_value_spaces_before_comment() {
        assert_eq!(parse_ok("K=   # comment"), vec![("K".into(), "".into())]);
    }

    #[test]
    fn empty_double_quoted_value_with_comment() {
        assert_eq!(parse_ok("K=\"\" # comment"), vec![("K".into(), "".into())]);
    }

    // ── Whitespace handling ────────────────────────────────────────────────

    #[test]
    fn leading_whitespace_on_line() {
        assert_eq!(parse_ok("  K=v"), vec![("K".into(), "v".into())]);
    }

    #[test]
    fn trailing_whitespace_before_equals() {
        assert_eq!(parse_ok("K  =v"), vec![("K".into(), "v".into())]);
    }

    #[test]
    fn whitespace_around_equals() {
        assert_eq!(parse_ok("K = v"), vec![("K".into(), "v".into())]);
    }

    #[test]
    fn tabs_as_whitespace() {
        assert_eq!(parse_ok("\tK\t=\tv"), vec![("K".into(), "v".into())]);
    }

    #[test]
    fn tab_after_equals() {
        assert_eq!(parse_ok("K=\tval"), vec![("K".into(), "val".into())]);
    }

    #[test]
    fn double_equals_value() {
        assert_eq!(parse_ok("K==v"), vec![("K".into(), "=v".into())]);
    }

    // ── Comments ───────────────────────────────────────────────────────────

    #[test]
    fn full_line_comment() {
        assert!(parse_ok("# this is a comment").is_empty());
    }

    #[test]
    fn comment_with_leading_spaces() {
        assert!(parse_ok("  # indented comment").is_empty());
    }

    #[test]
    fn empty_lines_skipped() {
        assert!(parse_ok("\n\n\n").is_empty());
    }

    #[test]
    fn mixed_comments_and_values() {
        let pairs = parse_ok("# header\nA=1\n\nB=2 # inline\n");
        assert_eq!(pairs, vec![("A".into(), "1".into()), ("B".into(), "2".into())]);
    }

    // ── Line endings ───────────────────────────────────────────────────────

    #[test]
    fn unix_line_endings() {
        assert_eq!(parse_ok("A=1\nB=2"), vec![("A".into(), "1".into()), ("B".into(), "2".into())]);
    }

    #[test]
    fn windows_line_endings() {
        assert_eq!(parse_ok("A=1\r\nB=2"), vec![("A".into(), "1".into()), ("B".into(), "2".into())]);
    }

    #[test]
    fn no_trailing_newline() {
        assert_eq!(parse_ok("A=1"), vec![("A".into(), "1".into())]);
    }

    #[test]
    fn single_line_no_newline() {
        assert_eq!(parse_ok("K=v"), vec![("K".into(), "v".into())]);
    }

    #[test]
    fn lone_cr_is_not_a_line_break() {
        // Documented limitation: only \n and \r\n split lines.
        assert_eq!(parse_ok("A=1\rB=2"), vec![("A".into(), "1\rB=2".into())]);
    }

    #[test]
    fn cr_cr_lf() {
        assert_eq!(
            parse_ok("A=1\r\r\nB=2"),
            vec![("A".into(), "1".into()), ("B".into(), "2".into())]
        );
    }

    // ── BOM ────────────────────────────────────────────────────────────────

    #[test]
    fn bom_is_stripped() {
        assert_eq!(parse_ok("\u{FEFF}K=v"), vec![("K".into(), "v".into())]);
    }

    #[test]
    fn bom_only_file() {
        assert!(parse_ok("\u{FEFF}").is_empty());
    }

    #[test]
    fn bom_with_comment_first_line() {
        assert_eq!(parse_ok("\u{FEFF}# c\nK=v"), vec![("K".into(), "v".into())]);
    }

    // ── NUL bytes ──────────────────────────────────────────────────────────

    #[test]
    fn nul_in_unquoted_value_is_error() {
        assert_eq!(parse_kind("K=a\0b"), ParseErrorKind::InvalidValue);
    }

    #[test]
    fn nul_in_double_quoted_value_is_error() {
        assert_eq!(parse_kind("K=\"a\0b\""), ParseErrorKind::InvalidValue);
    }

    #[test]
    fn nul_in_single_quoted_value_is_error() {
        assert_eq!(parse_kind("K='a\0b'"), ParseErrorKind::InvalidValue);
    }

    #[test]
    fn nul_in_key_is_invalid_key() {
        assert_eq!(parse_kind("K\0J=v"), ParseErrorKind::InvalidKey);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn load_with_nul_value_errors_without_panicking() {
        let _guard = TEST_LOCK.lock().unwrap();
        let dir = unique_dir("nul");
        fs::create_dir_all(&dir).unwrap();
        let env_path = dir.join(".env");
        fs::write(&env_path, "DOTENV_NUL=a\0b").unwrap();

        let old = env::current_dir().ok();
        env::set_current_dir(&dir).unwrap();
        let result = load();
        if let Some(p) = old {
            let _ = env::set_current_dir(p);
        }
        let _ = fs::remove_dir_all(&dir);

        match result.unwrap_err() {
            Error::Parse(e) => assert_eq!(e.kind, ParseErrorKind::InvalidValue),
            other => panic!("expected parse error, got {other:?}"),
        }
    }

    // ── Edge cases: empty / comment-only files ─────────────────────────────

    #[test]
    fn empty_file() {
        assert!(parse_ok("").is_empty());
    }

    #[test]
    fn only_comments() {
        assert!(parse_ok("# a\n# b\n# c").is_empty());
    }

    #[test]
    fn only_blank_lines() {
        assert!(parse_ok("\n\n \n\t\n").is_empty());
    }

    // ── Error cases ────────────────────────────────────────────────────────

    #[test]
    fn error_missing_equals() {
        assert_eq!(parse_kind("INVALID"), ParseErrorKind::MissingEquals);
    }

    #[test]
    fn error_missing_equals_with_comment() {
        assert_eq!(parse_kind("K # comment"), ParseErrorKind::MissingEquals);
    }

    #[test]
    fn error_empty_key() {
        assert_eq!(parse_kind("=value"), ParseErrorKind::EmptyKey);
    }

    #[test]
    fn error_empty_key_with_spaces() {
        assert_eq!(parse_kind("   =value"), ParseErrorKind::EmptyKey);
    }

    #[test]
    fn error_unmatched_double_quote() {
        assert_eq!(parse_kind("K=\"hello"), ParseErrorKind::UnmatchedQuote);
    }

    #[test]
    fn error_unmatched_single_quote() {
        assert_eq!(parse_kind("K='hello"), ParseErrorKind::UnmatchedQuote);
    }

    #[test]
    fn error_unmatched_double_quote_with_hash() {
        assert_eq!(parse_kind("K=\"hello#more"), ParseErrorKind::UnmatchedQuote);
    }

    #[test]
    fn error_trailing_content_double_quote() {
        assert_eq!(parse_kind("K=\"hello\"extra"), ParseErrorKind::TrailingContent);
    }

    #[test]
    fn error_trailing_content_single_quote() {
        assert_eq!(parse_kind("K='hello'extra"), ParseErrorKind::TrailingContent);
    }

    #[test]
    fn error_trailing_content_line_number() {
        assert_eq!(parse_line("A=1\nK=\"v\"x\nB=2"), 2);
    }

    #[test]
    fn error_invalid_key_exclamation() {
        assert_eq!(parse_kind("K!EY=v"), ParseErrorKind::InvalidKey);
    }

    #[test]
    fn error_invalid_key_dollar() {
        assert_eq!(parse_kind("\u{0024}KEY=v"), ParseErrorKind::InvalidKey);
    }

    #[test]
    fn error_invalid_key_at() {
        assert_eq!(parse_kind("KEY@=v"), ParseErrorKind::InvalidKey);
    }

    #[test]
    fn error_invalid_key_space() {
        assert_eq!(parse_kind("K EY=v"), ParseErrorKind::InvalidKey);
    }

    #[test]
    fn error_invalid_key_slash() {
        assert_eq!(parse_kind("KEY/VAL=v"), ParseErrorKind::InvalidKey);
    }

    #[test]
    fn error_invalid_key_unicode() {
        assert_eq!(parse_kind("K\u{00C9}Y=v"), ParseErrorKind::InvalidKey);
    }

    #[test]
    fn error_line_number_missing_equals() {
        assert_eq!(parse_line("A=1\nINVALID\nB=2"), 2);
    }

    #[test]
    fn error_line_number_invalid_key() {
        assert_eq!(parse_line("A=1\n\"$\"BAD=v\nB=2"), 2);
    }

    #[test]
    fn error_line_number_unmatched_quote() {
        assert_eq!(parse_line("A=1\nK=\"unclosed\nB=2"), 2);
    }

    // ── Unicode values ─────────────────────────────────────────────────────

    #[test]
    fn unicode_value_unquoted() {
        assert_eq!(parse_ok("K=h\u{00E9}llo"), vec![("K".into(), "h\u{00E9}llo".into())]);
    }

    #[test]
    fn unicode_value_double_quoted() {
        assert_eq!(parse_ok("K=\"h\u{00E9}llo\""), vec![("K".into(), "h\u{00E9}llo".into())]);
    }

    #[test]
    fn unicode_value_single_quoted() {
        assert_eq!(parse_ok("K='h\u{00E9}llo'"), vec![("K".into(), "h\u{00E9}llo".into())]);
    }

    // ── Public `parse` (map) ───────────────────────────────────────────────

    #[test]
    fn parse_map_first_wins() {
        let vars = parse("A=1\nA=2\nB=3").unwrap();
        assert_eq!(vars["A"].as_ref(), "1");
        assert_eq!(vars["B"].as_ref(), "3");
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn parse_map_empty() {
        assert!(parse("# c\n\n").unwrap().is_empty());
    }

    #[test]
    fn parse_map_expands_escapes() {
        let vars = parse("K=\"a\\nb\"").unwrap();
        assert_eq!(vars["K"].as_ref(), "a\nb");
    }

    #[test]
    fn parse_borrows_plain_values_and_owns_escaped() {
        let vars = parse("A=plain\nB=\"x\\ny\"").unwrap();
        assert_eq!(vars["A"], Cow::Borrowed("plain"));
        assert!(matches!(&vars["B"], Cow::Owned(_)));
    }

    #[test]
    fn parse_map_error() {
        assert!(parse("=").is_err());
    }

    // ── `load()` / `from_path()` integration tests ─────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    fn unique_dir(tag: &str) -> std::path::PathBuf {
        env::temp_dir().join(format!("dotenv_test_{tag}_{}", std::process::id()))
    }

    // `env::temp_dir()` panics on WASI, so these tests cannot run on wasm32.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn load_sets_vars() {
        let _guard = TEST_LOCK.lock().unwrap();
        let dir = unique_dir("basic");
        let _ = fs::create_dir_all(&dir);
        let env_path = dir.join(".env");
        fs::write(&env_path, "DOTENV_TEST_FOO=bar\nDOTENV_TEST_BAZ=qux").unwrap();

        let old = env::current_dir().ok();
        env::set_current_dir(&dir).unwrap();

        let result = load();

        if let Some(p) = old {
            let _ = env::set_current_dir(p);
        }
        let _ = fs::remove_file(&env_path);
        let _ = fs::remove_dir(&dir);

        assert!(result.is_ok());
        assert_eq!(env::var("DOTENV_TEST_FOO").unwrap(), "bar");
        assert_eq!(env::var("DOTENV_TEST_BAZ").unwrap(), "qux");

        unsafe { remove_env("DOTENV_TEST_FOO") };
        unsafe { remove_env("DOTENV_TEST_BAZ") };
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn from_path_sets_vars() {
        let _guard = TEST_LOCK.lock().unwrap();
        let dir = unique_dir("from_path");
        let _ = fs::create_dir_all(&dir);
        let env_path = dir.join("custom.env");
        fs::write(&env_path, "DOTENV_FROM_PATH=yes").unwrap();

        let result = from_path(&env_path);

        let _ = fs::remove_file(&env_path);
        let _ = fs::remove_dir(&dir);

        assert!(result.is_ok());
        assert_eq!(env::var("DOTENV_FROM_PATH").unwrap(), "yes");
        unsafe { remove_env("DOTENV_FROM_PATH") };
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn load_preserves_existing_env_vars() {
        let _guard = TEST_LOCK.lock().unwrap();
        unsafe { set_env("DOTENV_EXISTING", "original") };

        let dir = unique_dir("existing");
        let _ = fs::create_dir_all(&dir);
        let env_path = dir.join(".env");
        fs::write(&env_path, "DOTENV_EXISTING=from_file").unwrap();

        let old = env::current_dir().ok();
        env::set_current_dir(&dir).unwrap();

        let result = load();

        if let Some(p) = old {
            let _ = env::set_current_dir(p);
        }
        let _ = fs::remove_file(&env_path);
        let _ = fs::remove_dir(&dir);

        assert!(result.is_ok());
        assert_eq!(env::var("DOTENV_EXISTING").unwrap(), "original");

        unsafe { remove_env("DOTENV_EXISTING") };
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn load_first_declaration_wins() {
        let _guard = TEST_LOCK.lock().unwrap();
        let dir = unique_dir("first");
        let _ = fs::create_dir_all(&dir);
        let env_path = dir.join(".env");
        fs::write(&env_path, "DOTENV_DUP=first\nDOTENV_DUP=second").unwrap();

        let old = env::current_dir().ok();
        env::set_current_dir(&dir).unwrap();

        let result = load();

        if let Some(p) = old {
            let _ = env::set_current_dir(p);
        }
        let _ = fs::remove_file(&env_path);
        let _ = fs::remove_dir(&dir);

        assert!(result.is_ok());
        assert_eq!(env::var("DOTENV_DUP").unwrap(), "first");

        unsafe { remove_env("DOTENV_DUP") };
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn load_parse_error_is_atomic() {
        let _guard = TEST_LOCK.lock().unwrap();
        let dir = unique_dir("atomic");
        let _ = fs::create_dir_all(&dir);
        let env_path = dir.join(".env");
        // A valid line precedes a malformed one.
        fs::write(&env_path, "DOTENV_ATOMIC=set\nMALFORMED LINE\n").unwrap();

        let old = env::current_dir().ok();
        env::set_current_dir(&dir).unwrap();
        let result = load();
        if let Some(p) = old {
            let _ = env::set_current_dir(p);
        }
        let _ = fs::remove_file(&env_path);
        let _ = fs::remove_dir(&dir);

        assert!(matches!(result, Err(Error::Parse(_))));
        assert!(
            env::var_os("DOTENV_ATOMIC").is_none(),
            "variable from before the error must not be set"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn load_file_not_found() {
        let _guard = TEST_LOCK.lock().unwrap();
        let dir = unique_dir("missing");
        let _ = fs::create_dir_all(&dir);

        let old = env::current_dir().ok();
        env::set_current_dir(&dir).unwrap();

        let result = load();

        if let Some(p) = old {
            let _ = env::set_current_dir(p);
        }
        let _ = fs::remove_dir(&dir);

        match result.unwrap_err() {
            Error::Io(_) => {}
            _ => panic!("expected Io error"),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn load_parse_error() {
        let _guard = TEST_LOCK.lock().unwrap();
        let dir = unique_dir("parse_err");
        let _ = fs::create_dir_all(&dir);
        let env_path = dir.join(".env");
        fs::write(&env_path, "A=1\nMALFORMED\nB=2").unwrap();

        let old = env::current_dir().ok();
        env::set_current_dir(&dir).unwrap();

        let result = load();

        if let Some(p) = old {
            let _ = env::set_current_dir(p);
        }
        let _ = fs::remove_file(&env_path);
        let _ = fs::remove_dir(&dir);

        match result.unwrap_err() {
            Error::Parse(e) => assert_eq!(e.line, 2),
            other => panic!("expected Parse error, got {other:?}"),
        }
    }

    // A non-UTF-8 variable in the environment must not make `load` panic.
    #[cfg(unix)]
    #[test]
    fn load_survives_non_utf8_env_var() {
        use std::os::unix::ffi::OsStringExt;

        let _guard = TEST_LOCK.lock().unwrap();
        let dir = unique_dir("nonutf8");
        let _ = fs::create_dir_all(&dir);
        let env_path = dir.join(".env");
        fs::write(&env_path, "DOTENV_NONUTF8_OK=1").unwrap();

        let bad_key = OsString::from_vec(b"DOTENV_BAD\xFF".to_vec());
        let bad_val = OsString::from_vec(vec![0xfe, 0xff]);
        unsafe { env::set_var(&bad_key, &bad_val) };

        let old = env::current_dir().ok();
        env::set_current_dir(&dir).unwrap();
        let result = std::panic::catch_unwind(load);
        if let Some(p) = old {
            let _ = env::set_current_dir(p);
        }
        let _ = fs::remove_file(&env_path);
        let _ = fs::remove_dir(&dir);
        unsafe { env::remove_var(&bad_key) };

        assert!(result.is_ok(), "load() panicked with a non-UTF-8 env var present");
        assert_eq!(env::var("DOTENV_NONUTF8_OK").unwrap(), "1");
        unsafe { remove_env("DOTENV_NONUTF8_OK") };
    }
}
