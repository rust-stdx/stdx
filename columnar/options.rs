//! Decoding limits and options.

use core::mem::size_of;

use crate::{Error, Result};

/// Limits applied while decoding an envelope from potentially untrusted bytes.
///
/// The defaults are deliberately generous for ordinary data sets but bounded so
/// that a forged header cannot cause unbounded allocation. Use
/// [`DecodeOptions::unlimited`] only for data you already trust.
///
/// Decoders validate a column's metadata before reserving its output, so a
/// short message that merely declares millions of rows or dictionary entries
/// fails before any large allocation. [`DecodeOptions::max_decoded_bytes`]
/// bounds each container (a column, a list, a dictionary, or a row vector)
/// independently; a type with several fields can allocate this much per field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodeOptions {
    /// Maximum number of rows accepted in the envelope header.
    pub max_rows: usize,
    /// Maximum byte length of one string or byte-string value.
    pub max_string_len: usize,
    /// Maximum total number of child values in one list column.
    pub max_list_len: usize,
    /// Maximum nesting depth for skipped CBOR data.
    pub max_depth: usize,
    /// Maximum encoded envelope size, in bytes.
    pub max_total_bytes: usize,
    /// Maximum number of bytes that one decoded container may occupy.
    ///
    /// The bound is applied to the values of one column, list, dictionary or
    /// row vector, using the size of the Rust element type. It is a per
    /// container bound: a type with several fields can allocate this much per
    /// field. The default is 1 GiB.
    pub max_decoded_bytes: usize,
}

impl DecodeOptions {
    /// Strict defaults suitable for untrusted input.
    ///
    /// The defaults allow up to 16,777,216 rows, 1 GiB of string data, 1 GiB
    /// of list elements, 64 levels of nesting, a 1 GiB encoded envelope, and
    /// 1 GiB of decoded data per container.
    pub const fn new() -> Self {
        DecodeOptions {
            max_rows: 1 << 24,
            max_string_len: 1 << 30,
            max_list_len: 1 << 30,
            max_depth: 64,
            max_total_bytes: 1 << 30,
            max_decoded_bytes: 1 << 30,
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
            max_decoded_bytes: usize::MAX,
        }
    }

    /// Check and return the capacity for `count` decoded values of type `T`.
    ///
    /// The declared count must fit [`DecodeOptions::max_decoded_bytes`]. Decoders
    /// call this only after they have validated the payload that describes the
    /// values, so a short message that merely declares a huge count fails before
    /// this allocation hint is used.
    #[doc(hidden)]
    pub fn output_capacity<T>(&self, count: usize) -> Result<usize> {
        self.check_output_bytes::<T>(count)?;
        Ok(count)
    }

    /// Check that `count` values of type `T` fit in
    /// [`DecodeOptions::max_decoded_bytes`].
    ///
    /// Returns [`Error::LengthOverflow`] when the multiplication overflows.
    #[doc(hidden)]
    pub fn check_output_bytes<T>(&self, count: usize) -> Result<()> {
        let bytes = count.checked_mul(size_of::<T>().max(1)).ok_or(Error::LengthOverflow)?;
        // `Vec::reserve` panics above `isize::MAX` even when the byte budget is
        // unlimited.
        if bytes > isize::MAX as usize {
            return Err(Error::LengthOverflow);
        }
        if bytes > self.max_decoded_bytes {
            return Err(Error::LimitExceeded("decoded bytes"));
        }
        Ok(())
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
