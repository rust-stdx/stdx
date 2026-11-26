//! Parser options and resource limits.

/// Policy for duplicate mapping keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DuplicateKeys {
    /// Reject the document when a mapping contains the same key twice.
    #[default]
    Error,
    /// Keep the first value for a duplicated key.
    First,
    /// Keep the last value for a duplicated key.
    Last,
}

/// Limits applied while parsing a YAML stream.
///
/// The defaults are deliberately conservative and can be relaxed for trusted
/// documents with [`from_str_with_options`](crate::from_str_with_options).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Options {
    /// How to handle duplicate mapping keys.
    pub duplicate_keys: DuplicateKeys,
    /// Maximum nesting depth for sequences and mappings.
    pub max_depth: usize,
    /// Maximum number of values materialized in one document, or `None` for no
    /// limit.
    ///
    /// This counts nodes produced by events as well as the copies the parser
    /// materializes for anchors, aliases, and merge keys.
    pub max_values: Option<usize>,
    /// Maximum number of alias nodes in one document, or `None` for no limit.
    pub max_aliases: Option<usize>,
    /// Maximum number of documents in a stream, or `None` for no limit.
    pub max_documents: Option<usize>,
    /// Maximum input size in bytes, or `None` for no limit.
    ///
    /// Applies to the slice/string parsers and to
    /// [`from_reader_with_options`](crate::from_reader_with_options).
    pub max_input_bytes: Option<usize>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            duplicate_keys: DuplicateKeys::Error,
            max_depth: 128,
            max_values: Some(1_000_000),
            max_aliases: Some(10_000),
            max_documents: Some(1_024),
            max_input_bytes: Some(64 * 1024 * 1024),
        }
    }
}
