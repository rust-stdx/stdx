//! Decoding limits and options.

/// Limits applied while decoding an envelope from potentially untrusted bytes.
///
/// The defaults are deliberately generous for ordinary data sets but bounded so
/// that a forged header cannot cause unbounded allocation. Use
/// [`DecodeOptions::unlimited`] only for data you already trust.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodeOptions {
    /// Maximum number of rows accepted in the envelope header.
    pub max_rows: usize,
    /// Maximum byte length of one string or byte-string value.
    pub max_string_len: usize,
    /// Maximum total number of child values in one list column.
    pub max_list_len: usize,
    /// Maximum nesting depth for the CBOR envelope and nested columns.
    pub max_depth: usize,
    /// Maximum encoded envelope size, in bytes.
    pub max_total_bytes: usize,
}

impl DecodeOptions {
    /// Strict defaults suitable for untrusted input.
    ///
    /// The defaults allow up to 16,777,216 rows, 1 GiB of string data, 1 GiB
    /// of list elements, 64 levels of nesting, and a 1 GiB encoded envelope
    /// size.
    pub const fn new() -> Self {
        DecodeOptions {
            max_rows: 1 << 24,
            max_string_len: 1 << 30,
            max_list_len: 1 << 30,
            max_depth: 64,
            max_total_bytes: 1 << 30,
        }
    }

    /// Disable all limits.
    ///
    /// Only use this for data from a trusted source. A malformed header can
    /// make the decoder attempt very large allocations when limits are off.
    pub const fn unlimited() -> Self {
        DecodeOptions {
            max_rows: usize::MAX,
            max_string_len: usize::MAX,
            max_list_len: usize::MAX,
            max_depth: usize::MAX,
            max_total_bytes: usize::MAX,
        }
    }

    pub(crate) fn cbor_options(self) -> cbor::Options {
        cbor::Options {
            max_depth: self.max_depth,
            max_container_entries: Some(16_384),
            duplicate_keys: cbor::DuplicateKeys::Error,
            ..cbor::Options::default()
        }
    }
}

impl Default for DecodeOptions {
    fn default() -> Self {
        DecodeOptions::new()
    }
}
