//! Typed and dynamic JSON decoding.

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::collections::{BTreeMap, VecDeque};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::{
    error::{Error, ErrorKind, Result},
    number::{BigNumberPolicy, Number},
    string,
};

const IS_WS: [bool; 256] = {
    let mut table = [false; 256];
    table[b' ' as usize] = true;
    table[b'\t' as usize] = true;
    table[b'\n' as usize] = true;
    table[b'\r' as usize] = true;
    table
};

/// How duplicate object keys are handled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DuplicateKeys {
    /// The last value wins. This matches `serde_json` and PostgreSQL `jsonb`.
    #[default]
    LastWins,
    /// The first value wins.
    FirstWins,
    /// Reject duplicate keys with [`ErrorKind::DuplicateKey`].
    Reject,
}

/// Decoding options.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Maximum nesting depth. Defaults to 128.
    pub max_depth: usize,
    /// Duplicate-key policy. Defaults to [`DuplicateKeys::LastWins`].
    pub duplicate_keys: DuplicateKeys,
    /// Policy for integers that do not fit in `u64`/`i64`.
    pub big_numbers: BigNumberPolicy,
    /// Skip a UTF-8 byte-order mark at the beginning of the input.
    pub allow_bom: bool,
    /// Optional limit on the number of entries in a single array or object.
    ///
    /// Entry counts are tracked up to 128 levels of nesting. Documents deeper
    /// than that are rejected when this limit is enabled.
    pub max_container_entries: Option<usize>,
    /// Optional limit on the raw length of a single string body, including
    /// escape sequences.
    pub max_string_len: Option<usize>,
    /// Enables the experimental bitmask-ring stage-1 parser.
    ///
    /// The ring is allocation-free and bounded, but currently loses to the
    /// scalar parser on common workloads because the scalar parser already
    /// uses SIMD string scanning. It is exposed for benchmarking and future
    /// stage-2 work.
    pub use_simd_masks: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            max_depth: 128,
            duplicate_keys: DuplicateKeys::LastWins,
            big_numbers: BigNumberPolicy::Float,
            allow_bom: true,
            max_container_entries: None,
            max_string_len: None,
            use_simd_masks: false,
        }
    }
}

/// An object key. Borrowed keys avoid allocation; escaped keys are owned when
/// the `alloc` feature is enabled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key<'a> {
    /// A key without escapes, borrowed from the input.
    Borrowed(&'a str),
    /// A key that required unescaping.
    #[cfg(feature = "alloc")]
    Escaped(String),
}

impl<'a> Key<'a> {
    /// Returns the decoded key.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            Key::Borrowed(value) => value,
            #[cfg(feature = "alloc")]
            Key::Escaped(value) => value,
        }
    }

    /// Converts the key into a `Cow`.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn into_cow(self) -> Cow<'a, str> {
        match self {
            Key::Borrowed(value) => Cow::Borrowed(value),
            Key::Escaped(value) => Cow::Owned(value),
        }
    }
}

/// A JSON decoder over a byte slice.
pub struct Decoder<'a> {
    input: &'a [u8],
    pos: usize,
    options: Options,
    depth: usize,
    counts: [u32; 128],
    input_is_utf8: bool,
    mask: crate::mask::MaskRing,
}

impl<'a> Decoder<'a> {
    /// Creates a decoder with default options.
    pub fn new(input: &'a [u8]) -> Self {
        Self::with_options(input, Options::default())
    }

    /// Creates a decoder over a `&str`.
    ///
    /// String bodies are known to be valid UTF-8 and skip per-string
    /// validation.
    pub fn from_str_input(input: &'a str) -> Self {
        Self::from_str_input_with_options(input, Options::default())
    }

    /// Creates a decoder over a `&str` with explicit options.
    pub fn from_str_input_with_options(input: &'a str, options: Options) -> Self {
        let mut decoder = Self::with_options(input.as_bytes(), options);
        decoder.input_is_utf8 = true;
        decoder
    }

    /// Creates a decoder whose input has already been validated as UTF-8.
    ///
    /// The caller must guarantee that `input` is valid UTF-8. This is used by
    /// the owned entry points after a single SIMD validation pass, and by
    /// `Document`, which validates its input before exposing elements.
    pub(crate) fn from_validated_utf8(input: &'a [u8], options: Options) -> Self {
        let mut decoder = Self::with_options(input, options);
        decoder.input_is_utf8 = true;
        decoder
    }

    /// Creates a decoder with the given options.
    pub fn with_options(input: &'a [u8], options: Options) -> Self {
        let mut pos = 0;
        if options.allow_bom && input.starts_with(&[0xEF, 0xBB, 0xBF]) {
            pos = 3;
        }
        Self {
            input,
            pos,
            options,
            depth: 0,
            counts: [0; 128],
            input_is_utf8: false,
            mask: crate::mask::MaskRing::new(input.len(), options.use_simd_masks),
        }
    }

    /// Returns the current byte offset.
    #[inline]
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Sets the current byte offset.
    #[inline]
    pub fn set_position(&mut self, position: usize) {
        self.pos = position.min(self.input.len());
        // The ring carries quote/backslash state from the beginning of the
        // input; seeking invalidates it. Fall back to the scalar scanner.
        self.mask.disable();
    }

    /// Returns the remaining input.
    #[inline]
    pub fn remaining(&self) -> &'a [u8] {
        &self.input[self.pos..]
    }

    /// Returns a small initial capacity hint for an array.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn array_hint(&mut self) -> Result<usize> {
        self.skip_ws()?;
        Ok(match self.peek_byte() {
            Some(b']') | None => 0,
            Some(b'[' | b'{') => 2,
            _ => 4,
        })
    }

    /// Returns a small initial capacity hint for an object.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn object_hint(&mut self) -> Result<usize> {
        self.skip_ws()?;
        Ok(match self.peek_byte() {
            Some(b'}') | None => 0,
            _ => 4,
        })
    }

    /// Returns the decoding options.
    #[inline]
    pub fn options(&self) -> Options {
        self.options
    }

    /// Returns the duplicate-key policy without copying the whole options.
    #[inline]
    pub fn duplicate_keys(&self) -> DuplicateKeys {
        self.options.duplicate_keys
    }

    /// Returns whether duplicate fields need to be tracked.
    #[inline]
    pub fn tracks_duplicates(&self) -> bool {
        !matches!(self.options.duplicate_keys, DuplicateKeys::LastWins)
    }

    /// Applies the duplicate-key policy for one object field.
    ///
    /// Returns `true` when the value should be decoded, and `false` when it
    /// should be skipped (first-wins). Returns an error for the reject policy.
    #[inline]
    pub fn duplicate_allows(&self, seen: &mut u128, mask: u128, always_reject: bool) -> Result<bool> {
        match (always_reject, self.options.duplicate_keys) {
            (true, _) | (false, DuplicateKeys::Reject) => {
                if *seen & mask != 0 {
                    Err(self.error(ErrorKind::DuplicateKey))
                } else {
                    *seen |= mask;
                    Ok(true)
                }
            }
            (false, DuplicateKeys::FirstWins) => {
                if *seen & mask != 0 {
                    Ok(false)
                } else {
                    *seen |= mask;
                    Ok(true)
                }
            }
            (false, DuplicateKeys::LastWins) => Ok(true),
        }
    }

    /// Applies the duplicate-key policy when the caller already knows whether
    /// the key has been seen.
    ///
    /// Returns `true` when the value should be decoded, and `false` when it
    /// should be skipped (first-wins). Returns an error for the reject policy.
    #[inline]
    pub fn duplicate_action(&self, seen: bool, always_reject: bool) -> Result<bool> {
        match (always_reject, self.options.duplicate_keys) {
            (true, _) | (false, DuplicateKeys::Reject) => {
                if seen {
                    Err(self.error(ErrorKind::DuplicateKey))
                } else {
                    Ok(true)
                }
            }
            (false, DuplicateKeys::FirstWins) => Ok(!seen),
            (false, DuplicateKeys::LastWins) => Ok(true),
        }
    }

    /// Returns the current nesting depth.
    #[inline]
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Creates an error at the current position.
    #[inline]
    pub fn error(&self, kind: ErrorKind) -> Error {
        Error::at(kind, self.input, self.pos)
    }

    /// Creates an error at an explicit position.
    #[inline]
    pub fn error_at(&self, kind: ErrorKind, offset: usize) -> Error {
        Error::at(kind, self.input, offset)
    }

    #[inline]
    #[allow(unsafe_code)]
    fn body_str(&self, body: &'a [u8]) -> Result<&'a str> {
        if self.input_is_utf8 {
            // SAFETY: the input came from `&str`; `body` starts and ends on
            // ASCII quote boundaries, so it is a valid UTF-8 substring.
            Ok(unsafe { core::str::from_utf8_unchecked(body) })
        } else {
            string::str_from_utf8(body, self.pos)
        }
    }

    pub(crate) fn locate_error(&self, error: Error) -> Error {
        if error.line() == 0 {
            Error::at(error.kind(), self.input, error.offset())
        } else {
            error
        }
    }

    /// Returns the next byte without consuming it.
    #[inline]
    pub fn peek_byte(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    /// Returns the next byte after skipping whitespace, without consuming it.
    #[inline]
    pub fn peek_non_ws(&mut self) -> Result<Option<u8>> {
        self.skip_ws()?;
        Ok(self.peek_byte())
    }

    #[inline]
    fn bump(&mut self) {
        self.pos += 1;
    }

    /// Skips JSON whitespace.
    #[inline]
    pub fn skip_ws(&mut self) -> Result<()> {
        // Most calls start on a non-whitespace byte. Keep that path to a
        // single table lookup before touching the mask ring.
        if let Some(&byte) = self.input.get(self.pos)
            && !IS_WS[byte as usize]
        {
            return Ok(());
        }
        if self.mask.active {
            self.mask.skip_ws(self.input, &mut self.pos)?;
            if self.mask.active {
                return Ok(());
            }
        }
        while let Some(&byte) = self.input.get(self.pos) {
            if !IS_WS[byte as usize] {
                break;
            }
            self.pos += 1;
        }
        Ok(())
    }

    fn expect_byte(&mut self, expected: u8, kind: ErrorKind) -> Result<()> {
        self.skip_ws()?;
        match self.peek_byte() {
            Some(byte) if byte == expected => {
                self.bump();
                Ok(())
            }
            None => Err(self.error(ErrorKind::UnexpectedEof)),
            Some(_) => Err(self.error(kind)),
        }
    }

    fn enter_depth(&mut self) -> Result<()> {
        if self.depth >= self.options.max_depth {
            return Err(self.error(ErrorKind::DepthLimitExceeded));
        }
        self.depth += 1;
        Ok(())
    }

    fn leave_depth(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    /// Decodes a value of type `T`.
    pub fn decode<T: Decode<'a>>(&mut self) -> Result<T> {
        self.skip_ws()?;
        T::decode(self).map_err(|error| self.locate_error(error))
    }

    /// Decodes at the current position without skipping leading whitespace.
    ///
    /// Primitive decoders still skip whitespace themselves; this avoids a
    /// redundant check in hot container loops.
    #[inline]
    pub fn decode_inline<T: Decode<'a>>(&mut self) -> Result<T> {
        T::decode(self).map_err(|error| self.locate_error(error))
    }

    /// Consumes a `null`.
    #[inline]
    pub fn decode_null(&mut self) -> Result<()> {
        self.decode_literal(b"null", ErrorKind::ExpectedValue)
    }

    /// Consumes `true`.
    #[inline]
    pub fn decode_true(&mut self) -> Result<()> {
        self.decode_literal(b"true", ErrorKind::ExpectedValue)
    }

    /// Consumes `false`.
    #[inline]
    pub fn decode_false(&mut self) -> Result<()> {
        self.decode_literal(b"false", ErrorKind::ExpectedValue)
    }

    fn decode_literal(&mut self, literal: &[u8], kind: ErrorKind) -> Result<()> {
        self.skip_ws()?;
        if self.input[self.pos..].starts_with(literal) {
            self.pos += literal.len();
            Ok(())
        } else if self.pos >= self.input.len() {
            Err(self.error(ErrorKind::UnexpectedEof))
        } else {
            Err(self.error(kind))
        }
    }

    /// Decodes a boolean.
    #[inline]
    pub fn decode_bool(&mut self) -> Result<bool> {
        self.skip_ws()?;
        match self.peek_byte() {
            Some(b't') => {
                self.decode_true()?;
                Ok(true)
            }
            Some(b'f') => {
                self.decode_false()?;
                Ok(false)
            }
            None => Err(self.error(ErrorKind::UnexpectedEof)),
            Some(_) => Err(self.error(ErrorKind::ExpectedValue)),
        }
    }

    fn string_body(&mut self) -> Result<(&'a [u8], bool)> {
        self.skip_ws()?;
        match self.peek_byte() {
            Some(b'"') => self.bump(),
            None => return Err(self.error(ErrorKind::UnexpectedEof)),
            Some(_) => return Err(self.error(ErrorKind::ExpectedValue)),
        }
        let start = self.pos;
        let (end, escaped) = if self.mask.active {
            if let Some((close, escaped)) = self.mask.find_string_end(self.input, start)? {
                if let Some(max) = self.options.max_string_len
                    && close - start > max
                {
                    return Err(self.error_at(ErrorKind::StringLimitExceeded, start));
                }
                (close, escaped)
            } else {
                // Either EOF or the mask path was disabled mid-scan. The
                // scalar scanner produces the precise error.
                string::scan_body(self.input, start, self.options.max_string_len)?
            }
        } else {
            string::scan_body(self.input, start, self.options.max_string_len)?
        };
        self.pos = end + 1;
        self.mask.advance(self.pos);
        Ok((&self.input[start..end], escaped))
    }

    /// Decodes a string borrowed from the input.
    ///
    /// # Errors
    /// Returns an error if the string contains escape sequences, because the
    /// decoded form would not be a subslice of the input. Use
    /// [`Decoder::decode_cow_str`] or [`Decoder::decode_string`] instead.
    #[inline]
    pub fn decode_str(&mut self) -> Result<&'a str> {
        let (body, escaped) = self.string_body()?;
        if escaped {
            return Err(self.error(ErrorKind::Message("escaped string cannot be borrowed; use Cow or String")));
        }
        string::str_from_utf8(body, self.pos)
    }

    /// Decodes a string, borrowing it when possible.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn decode_cow_str(&mut self) -> Result<Cow<'a, str>> {
        let (body, escaped) = self.string_body()?;
        if escaped {
            let mut out = String::new();
            if self.input_is_utf8 {
                string::unescape_into_unchecked(body, &mut out, self.pos)?;
            } else {
                string::unescape_into(body, &mut out, self.pos)?;
            }
            Ok(Cow::Owned(out))
        } else {
            Ok(Cow::Borrowed(self.body_str(body)?))
        }
    }

    /// Decodes a string into an owned `String`.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn decode_string(&mut self) -> Result<String> {
        let (body, escaped) = self.string_body()?;
        let offset = self.pos.saturating_sub(body.len() + 1);
        let mut out = String::new();
        out.reserve(body.len());
        if escaped {
            if self.input_is_utf8 {
                string::unescape_into_unchecked(body, &mut out, offset)?;
            } else {
                string::unescape_into(body, &mut out, offset)?;
            }
        } else {
            out.push_str(self.body_str(body)?);
        }
        Ok(out)
    }

    /// Decodes a string into an existing `String`.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn decode_string_into(&mut self, out: &mut String) -> Result<()> {
        let (body, escaped) = self.string_body()?;
        let offset = self.pos.saturating_sub(body.len() + 1);
        out.clear();
        out.reserve(body.len());
        if escaped {
            if self.input_is_utf8 {
                string::unescape_into_unchecked(body, &mut *out, offset)?;
            } else {
                string::unescape_into(body, &mut *out, offset)?;
            }
        } else {
            out.push_str(self.body_str(body)?);
        }
        Ok(())
    }

    /// Decodes a string into a caller-provided buffer and returns it as a
    /// string. This is the no-alloc counterpart of [`Decoder::decode_str`].
    pub fn decode_str_into<'o>(&mut self, out: &'o mut [u8]) -> Result<&'o str> {
        let (body, escaped) = self.string_body()?;
        if escaped {
            let written = string::unescape_slice(body, out, self.pos)?;
            core::str::from_utf8(&out[..written]).map_err(|_| self.error(ErrorKind::InvalidUtf8))
        } else {
            let text = self.body_str(body)?;
            if text.len() > out.len() {
                return Err(Error::buffer_too_small());
            }
            out[..text.len()].copy_from_slice(text.as_bytes());
            core::str::from_utf8(&out[..text.len()]).map_err(|_| self.error(ErrorKind::InvalidUtf8))
        }
    }

    /// Decodes an object key and consumes the following `:`.
    #[inline]
    pub fn key(&mut self) -> Result<Key<'a>> {
        let (body, escaped) = self.string_body()?;
        if escaped {
            #[cfg(feature = "alloc")]
            {
                let mut out = String::new();
                if self.input_is_utf8 {
                    string::unescape_into_unchecked(body, &mut out, self.pos)?;
                } else {
                    string::unescape_into(body, &mut out, self.pos)?;
                }
                self.expect_byte(b':', ErrorKind::ExpectedColon)?;
                return Ok(Key::Escaped(out));
            }
            #[cfg(not(feature = "alloc"))]
            {
                return Err(self.error(ErrorKind::Message("escaped object key requires the alloc feature")));
            }
        }
        let key = self.body_str(body)?;
        self.expect_byte(b':', ErrorKind::ExpectedColon)?;
        Ok(Key::Borrowed(key))
    }

    fn number_text(&mut self) -> Result<(&'a str, bool)> {
        self.skip_ws()?;
        let start = self.pos;
        let (end, is_float) = crate::number::scan(self.input, start)?;
        self.pos = end;
        // The scanner only accepts ASCII digits, signs, dots and exponent
        // markers, so this slice is always valid UTF-8.
        #[allow(unsafe_code)]
        let text = unsafe { core::str::from_utf8_unchecked(&self.input[start..end]) };
        Ok((text, is_float))
    }

    /// Decodes a `u64`.
    #[inline]
    pub fn decode_u64(&mut self) -> Result<u64> {
        self.skip_ws()?;
        let start = self.pos;
        let (value, end) = crate::number::parse_u64_fast(self.input, start)?;
        self.pos = end;
        Ok(value)
    }

    /// Decodes an `i64`.
    #[inline]
    pub fn decode_i64(&mut self) -> Result<i64> {
        self.skip_ws()?;
        let start = self.pos;
        let (value, end) = crate::number::parse_i64_fast(self.input, start)?;
        self.pos = end;
        Ok(value)
    }

    /// Decodes a `u128`.
    #[inline]
    pub fn decode_u128(&mut self) -> Result<u128> {
        let (text, is_float) = self.number_text()?;
        if is_float {
            return Err(self.error(ErrorKind::NumberOutOfRange));
        }
        crate::number::parse_u128(text, self.pos)
    }

    /// Decodes an `i128`.
    #[inline]
    pub fn decode_i128(&mut self) -> Result<i128> {
        let (text, is_float) = self.number_text()?;
        if is_float {
            return Err(self.error(ErrorKind::NumberOutOfRange));
        }
        crate::number::parse_i128(text, self.pos)
    }

    /// Decodes an `f64`.
    #[inline]
    pub fn decode_f64(&mut self) -> Result<f64> {
        self.skip_ws()?;
        let start = self.pos;
        let (value, end) = crate::number::parse_f64_at(self.input, start)?;
        self.pos = end;
        Ok(value)
    }

    /// Decodes an `f32`.
    #[inline]
    pub fn decode_f32(&mut self) -> Result<f32> {
        self.skip_ws()?;
        let start = self.pos;
        let (value, end) = crate::number::parse_f64_at(self.input, start)?;
        self.pos = end;
        let value = value as f32;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(self.error(ErrorKind::NumberOutOfRange))
        }
    }

    /// Decodes a [`Number`].
    #[inline]
    pub fn decode_number(&mut self) -> Result<Number> {
        self.skip_ws()?;
        let start = self.pos;
        let (value, end) = crate::number::parse_number_at(self.input, start, self.options.big_numbers)?;
        self.pos = end;
        Ok(value)
    }

    /// Captures one number without converting it, preserving the exact lexeme.
    #[inline]
    pub fn decode_raw_number(&mut self) -> Result<crate::RawNumber<'a>> {
        self.skip_ws()?;
        let start = self.pos;
        let (end, _) = crate::number::scan(self.input, start)?;
        self.pos = end;
        let text = core::str::from_utf8(&self.input[start..end]).map_err(|_| self.error(ErrorKind::InvalidNumber))?;
        Ok(crate::RawNumber::from_slice(text))
    }

    /// Starts decoding an array. The matching close bracket is consumed by
    /// [`Decoder::array_done`].
    #[inline]
    pub fn begin_array(&mut self) -> Result<()> {
        self.expect_byte(b'[', ErrorKind::ExpectedValue)?;
        self.enter_depth()?;
        if self.depth != 0 && self.depth <= self.counts.len() {
            self.counts[self.depth - 1] = 0;
        }
        Ok(())
    }

    /// Advances to the next array element.
    ///
    /// `first` must be `true` for the first call after [`Decoder::begin_array`].
    /// Returns `false` after consuming the closing `]`.
    #[inline]
    fn count_entry(&mut self) -> Result<()> {
        if self.options.max_container_entries.is_none() {
            return Ok(());
        }
        if self.depth == 0 {
            return Ok(());
        }
        if self.depth > self.counts.len() {
            // Entry limits are tracked only up to the fixed counter array.
            // Silently ignoring the limit at deeper levels would make the
            // option unreliable, so reject instead.
            return Err(self.error(ErrorKind::ContainerLimitExceeded));
        }
        let count = &mut self.counts[self.depth - 1];
        *count = count.saturating_add(1);
        if let Some(max) = self.options.max_container_entries
            && *count as usize > max
        {
            return Err(self.error(ErrorKind::ContainerLimitExceeded));
        }
        Ok(())
    }

    /// Advances to the next array element.
    ///
    /// `first` must be `true` for the first call after [`Decoder::begin_array`].
    /// Returns `false` after consuming the closing `]`.
    #[inline]
    pub fn array_next(&mut self, first: bool) -> Result<bool> {
        self.skip_ws()?;
        if first {
            match self.peek_byte() {
                Some(b']') => {
                    self.bump();
                    self.leave_depth();
                    Ok(false)
                }
                None => Err(self.error(ErrorKind::UnexpectedEof)),
                Some(_) => {
                    self.count_entry()?;
                    Ok(true)
                }
            }
        } else {
            match self.peek_byte() {
                Some(b',') => {
                    self.bump();
                    self.skip_ws()?;
                    match self.peek_byte() {
                        Some(b']') => Err(self.error(ErrorKind::ExpectedValue)),
                        None => Err(self.error(ErrorKind::UnexpectedEof)),
                        Some(_) => {
                            self.count_entry()?;
                            Ok(true)
                        }
                    }
                }
                Some(b']') => {
                    self.bump();
                    self.leave_depth();
                    Ok(false)
                }
                None => Err(self.error(ErrorKind::UnexpectedEof)),
                Some(_) => Err(self.error(ErrorKind::ExpectedComma)),
            }
        }
    }

    /// Advances to the next object entry.
    ///
    /// `first` must be `true` for the first call after [`Decoder::begin_object`].
    /// Returns `false` after consuming the closing `}`.
    #[inline]
    pub fn object_next(&mut self, first: bool) -> Result<bool> {
        self.skip_ws()?;
        if first {
            match self.peek_byte() {
                Some(b'}') => {
                    self.bump();
                    self.leave_depth();
                    Ok(false)
                }
                None => Err(self.error(ErrorKind::UnexpectedEof)),
                Some(_) => {
                    self.count_entry()?;
                    Ok(true)
                }
            }
        } else {
            match self.peek_byte() {
                Some(b',') => {
                    self.bump();
                    self.skip_ws()?;
                    match self.peek_byte() {
                        Some(b'}') => Err(self.error(ErrorKind::ExpectedValue)),
                        None => Err(self.error(ErrorKind::UnexpectedEof)),
                        Some(_) => {
                            self.count_entry()?;
                            Ok(true)
                        }
                    }
                }
                Some(b'}') => {
                    self.bump();
                    self.leave_depth();
                    Ok(false)
                }
                None => Err(self.error(ErrorKind::UnexpectedEof)),
                Some(_) => Err(self.error(ErrorKind::ExpectedComma)),
            }
        }
    }

    /// Returns `true` and consumes `]` when the current array is finished.
    #[inline]
    pub fn array_done(&mut self) -> Result<bool> {
        self.skip_ws()?;
        match self.peek_byte() {
            Some(b']') => {
                self.bump();
                self.leave_depth();
                Ok(true)
            }
            None => Err(self.error(ErrorKind::UnexpectedEof)),
            Some(_) => Ok(false),
        }
    }

    /// Starts decoding an object. The matching close brace is consumed by
    /// [`Decoder::object_done`].
    #[inline]
    pub fn begin_object(&mut self) -> Result<()> {
        self.expect_byte(b'{', ErrorKind::ExpectedValue)?;
        self.enter_depth()?;
        if self.depth != 0 && self.depth <= self.counts.len() {
            self.counts[self.depth - 1] = 0;
        }
        Ok(())
    }

    /// Returns `true` and consumes `}` when the current object is finished.
    #[inline]
    pub fn object_done(&mut self) -> Result<bool> {
        self.skip_ws()?;
        match self.peek_byte() {
            Some(b'}') => {
                self.bump();
                self.leave_depth();
                Ok(true)
            }
            None => Err(self.error(ErrorKind::UnexpectedEof)),
            Some(_) => Ok(false),
        }
    }

    /// Consumes a `,` and rejects a trailing comma before a closing bracket.
    #[inline]
    pub fn expect_comma(&mut self) -> Result<()> {
        self.expect_byte(b',', ErrorKind::ExpectedComma)?;
        self.skip_ws()?;
        match self.peek_byte() {
            Some(b']') | Some(b'}') => Err(self.error(ErrorKind::ExpectedValue)),
            None => Err(self.error(ErrorKind::UnexpectedEof)),
            Some(_) => Ok(()),
        }
    }

    /// Validates and skips one JSON value.
    #[inline]
    pub fn skip_value(&mut self) -> Result<()> {
        self.skip_ws()?;
        match self.peek_byte() {
            None => Err(self.error(ErrorKind::UnexpectedEof)),
            Some(b'n') => self.decode_null(),
            Some(b't') => self.decode_true(),
            Some(b'f') => self.decode_false(),
            Some(b'"') => {
                self.bump();
                let start = self.pos;
                let end = string::skip(self.input, start, self.options.max_string_len, self.input_is_utf8)?;
                self.pos = end + 1;
                Ok(())
            }
            Some(b'[') => {
                self.begin_array()?;
                let mut first = true;
                while self.array_next(first)? {
                    first = false;
                    self.skip_value()?;
                }
                Ok(())
            }
            Some(b'{') => {
                self.begin_object()?;
                let mut first = true;
                while self.object_next(first)? {
                    first = false;
                    self.key()?;
                    self.skip_value()?;
                }
                Ok(())
            }
            Some(b'-' | b'0'..=b'9') => {
                self.number_text()?;
                Ok(())
            }
            Some(_) => Err(self.error(ErrorKind::ExpectedValue)),
        }
    }

    /// Captures one JSON value as a [`crate::RawValue`].
    #[inline]
    pub fn raw_value(&mut self) -> Result<crate::RawValue<'a>> {
        self.skip_ws()?;
        let start = self.pos;
        self.skip_value()?;
        Ok(crate::RawValue::from_validated(&self.input[start..self.pos]))
    }

    /// Checks that the remaining input is only whitespace.
    #[inline]
    pub fn finish(&mut self) -> Result<()> {
        self.skip_ws()?;
        if self.pos == self.input.len() {
            Ok(())
        } else {
            Err(self.error(ErrorKind::TrailingData))
        }
    }
}

/// A type that can be decoded from JSON.
pub trait Decode<'a>: Sized {
    /// Decodes a value from `decoder`.
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self>;
}

/// A type that owns all of its decoded data and can be built from any input
/// lifetime.
pub trait DecodeOwned: Sized {
    /// Decodes an owned value from `input`.
    fn decode_owned(input: &[u8]) -> Result<Self>;
}

impl<T: for<'a> Decode<'a>> DecodeOwned for T {
    fn decode_owned(input: &[u8]) -> Result<Self> {
        crate::from_slice(input)
    }
}

#[cfg(feature = "alloc")]
impl DecodeOwned for crate::Value<'static> {
    fn decode_owned(input: &[u8]) -> Result<Self> {
        let value: crate::Value<'_> = crate::from_slice(input)?;
        Ok(value.into_owned())
    }
}

impl<'a> Decode<'a> for bool {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_bool()
    }
}

macro_rules! impl_decode_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<'a> Decode<'a> for $ty {
                fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
                    let value = decoder.decode_u64()?;
                    <$ty>::try_from(value).map_err(|_| decoder.error(ErrorKind::NumberOutOfRange))
                }
            }
        )*
    };
}

impl_decode_unsigned!(u8, u16, u32, u64, usize);

macro_rules! impl_decode_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<'a> Decode<'a> for $ty {
                fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
                    let value = decoder.decode_i64()?;
                    <$ty>::try_from(value).map_err(|_| decoder.error(ErrorKind::NumberOutOfRange))
                }
            }
        )*
    };
}

impl_decode_signed!(i8, i16, i32, i64, isize);

impl<'a> Decode<'a> for u128 {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_u128()
    }
}

impl<'a> Decode<'a> for i128 {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_i128()
    }
}

impl<'a> Decode<'a> for f64 {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_f64()
    }
}

impl<'a> Decode<'a> for f32 {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_f32()
    }
}

impl<'a> Decode<'a> for Number {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_number()
    }
}

impl<'a> Decode<'a> for &'a str {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_str()
    }
}

#[cfg(feature = "alloc")]
impl<'a> Decode<'a> for String {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_string()
    }
}

#[cfg(feature = "alloc")]
impl<'a> Decode<'a> for Cow<'a, str> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_cow_str()
    }
}

#[cfg(feature = "alloc")]
impl<'a> Decode<'a> for char {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        let value = decoder.decode_cow_str()?;
        let mut chars = value.chars();
        let ch = chars.next().ok_or_else(|| decoder.error(ErrorKind::ExpectedValue))?;
        if chars.next().is_some() {
            return Err(decoder.error(ErrorKind::ExpectedValue));
        }
        Ok(ch)
    }
}

#[cfg(not(feature = "alloc"))]
impl<'a> Decode<'a> for char {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        let value = decoder.decode_str()?;
        let mut chars = value.chars();
        let ch = chars.next().ok_or_else(|| decoder.error(ErrorKind::ExpectedValue))?;
        if chars.next().is_some() {
            return Err(decoder.error(ErrorKind::ExpectedValue));
        }
        Ok(ch)
    }
}

impl<'a> Decode<'a> for () {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_null()
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Option<T> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            Ok(None)
        } else {
            Some(T::decode(decoder)).transpose()
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: Decode<'a>> Decode<'a> for Box<T> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        T::decode(decoder).map(Box::new)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: Decode<'a>> Decode<'a> for alloc::rc::Rc<T> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        T::decode(decoder).map(alloc::rc::Rc::new)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: Decode<'a>> Decode<'a> for alloc::sync::Arc<T> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        T::decode(decoder).map(alloc::sync::Arc::new)
    }
}

impl<'a, T> Decode<'a> for PhantomData<T> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_null()?;
        Ok(PhantomData)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: Decode<'a>> Decode<'a> for Vec<T> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.begin_array()?;
        let mut values = Vec::with_capacity(decoder.array_hint()?);
        let mut first = true;
        while decoder.array_next(first)? {
            first = false;
            values.push(decoder.decode_inline::<T>()?);
        }
        Ok(values)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: Decode<'a>> Decode<'a> for VecDeque<T> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.begin_array()?;
        let mut values = VecDeque::with_capacity(decoder.array_hint()?);
        let mut first = true;
        while decoder.array_next(first)? {
            first = false;
            values.push_back(decoder.decode_inline::<T>()?);
        }
        Ok(values)
    }
}

impl<'a, T: Decode<'a>, const N: usize> Decode<'a> for [T; N] {
    #[allow(unsafe_code)]
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        use core::mem::MaybeUninit;

        decoder.begin_array()?;
        let mut values: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        let mut initialized = 0usize;
        let result = (|| -> Result<()> {
            let mut first = true;
            for slot in &mut values {
                if !decoder.array_next(first)? {
                    return Err(decoder.error(ErrorKind::ExpectedValue));
                }
                first = false;
                slot.write(decoder.decode_inline::<T>()?);
                initialized += 1;
            }
            if decoder.array_next(first)? {
                return Err(decoder.error(ErrorKind::ExpectedValue));
            }
            Ok(())
        })();
        if let Err(error) = result {
            for slot in &mut values[..initialized] {
                // SAFETY: only slots written by the loop above are accessed.
                unsafe { slot.assume_init_drop() };
            }
            return Err(error);
        }
        Ok(values.map(|slot| {
            // SAFETY: every slot was initialized before `result` succeeded.
            unsafe { slot.assume_init() }
        }))
    }
}

impl<'a> Decode<'a> for crate::RawValue<'a> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.raw_value()
    }
}

#[cfg(feature = "alloc")]
impl<'a> Decode<'a> for crate::RawValueBuf {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        let value = decoder.raw_value()?;
        Ok(crate::RawValueBuf::from_validated_slice(value.get()))
    }
}

impl<'a> Decode<'a> for crate::RawNumber<'a> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.decode_raw_number()
    }
}

#[cfg(feature = "alloc")]
impl<'a, V: Decode<'a>> Decode<'a> for BTreeMap<String, V> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decode_string_map(decoder)
    }
}

#[cfg(feature = "alloc")]
impl<'a, V: Decode<'a>> Decode<'a> for BTreeMap<&'a str, V> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.begin_object()?;
        let mut map = BTreeMap::new();
        let mut first = true;
        while decoder.object_next(first)? {
            first = false;
            let key = match decoder.key()? {
                Key::Borrowed(key) => key,
                #[cfg(feature = "alloc")]
                Key::Escaped(_) => {
                    return Err(decoder.error(ErrorKind::Message("escaped key cannot be stored as &str")));
                }
            };
            let seen = map.contains_key(key);
            if decoder.duplicate_action(seen, false)? {
                let value = decoder.decode_inline::<V>()?;
                map.insert(key, value);
            } else {
                decoder.skip_value()?;
            }
        }
        Ok(map)
    }
}

#[cfg(feature = "alloc")]
fn decode_string_map<'a, V: Decode<'a>>(decoder: &mut Decoder<'a>) -> Result<BTreeMap<String, V>> {
    decoder.begin_object()?;
    let mut map = BTreeMap::new();
    let mut first = true;
    while decoder.object_next(first)? {
        first = false;
        let key = decoder.key()?.into_cow().into_owned();
        let seen = map.contains_key(&key);
        if decoder.duplicate_action(seen, false)? {
            let value = decoder.decode_inline::<V>()?;
            map.insert(key, value);
        } else {
            decoder.skip_value()?;
        }
    }
    Ok(map)
}

#[cfg(feature = "std")]
impl<'a, V: Decode<'a>, S: core::hash::BuildHasher + Default> Decode<'a> for std::collections::HashMap<String, V, S> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.begin_object()?;
        let mut map = std::collections::HashMap::with_hasher(S::default());
        let mut first = true;
        while decoder.object_next(first)? {
            first = false;
            let key = decoder.key()?.into_cow().into_owned();
            let seen = map.contains_key(&key);
            if decoder.duplicate_action(seen, false)? {
                let value = decoder.decode_inline::<V>()?;
                map.insert(key, value);
            } else {
                decoder.skip_value()?;
            }
        }
        Ok(map)
    }
}

macro_rules! impl_tuple {
    ($($name:ident),+ $(,)?) => {
        impl<'a, $($name: Decode<'a>),+> Decode<'a> for ($($name,)+) {
            #[allow(non_snake_case)]
            #[allow(unused_assignments)]
            fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
                decoder.begin_array()?;
                let mut first = true;
                $(
                    if !decoder.array_next(first)? {
                        return Err(decoder.error(ErrorKind::ExpectedValue));
                    }
                    first = false;
                    let $name = decoder.decode_inline::<$name>()?;
                )+
                if decoder.array_next(first)? {
                    return Err(decoder.error(ErrorKind::ExpectedValue));
                }
                Ok(($($name,)+))
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

/// Decodes a value of type `T` from `input`.
pub fn from_slice<'a, T: Decode<'a>>(input: &'a [u8]) -> Result<T> {
    from_slice_with_options(input, Options::default())
}

/// Decodes a value of type `T` from `input` with explicit options.
///
/// The input is validated as UTF-8 once before parsing. This both enforces
/// RFC 8259 and lets string decoding skip per-string validation.
pub fn from_slice_with_options<'a, T: Decode<'a>>(input: &'a [u8], options: Options) -> Result<T> {
    utf8::from_utf8(input).map_err(|error| Error::at(ErrorKind::InvalidUtf8, input, error.valid_up_to()))?;
    let mut decoder = Decoder::from_validated_utf8(input, options);
    decoder.skip_ws()?;
    let value = decoder.decode_inline::<T>()?;
    decoder.finish()?;
    Ok(value)
}

/// Decodes an owned value of type `T` from a byte slice.
#[allow(dead_code)]
pub fn from_slice_owned<T: DecodeOwned>(input: &[u8]) -> Result<T> {
    T::decode_owned(input)
}

/// Decodes a value of type `T` from a string.
pub fn from_str<'a, T: Decode<'a>>(input: &'a str) -> Result<T> {
    from_str_with_options(input, Options::default())
}

/// Decodes an owned value of type `T` from a string.
#[allow(dead_code)]
pub fn from_str_owned<T: DecodeOwned>(input: &str) -> Result<T> {
    T::decode_owned(input.as_bytes())
}

/// Decodes a value of type `T` from a string with explicit options.
pub fn from_str_with_options<'a, T: Decode<'a>>(input: &'a str, options: Options) -> Result<T> {
    let mut decoder = Decoder::from_str_input_with_options(input, options);
    decoder.skip_ws()?;
    let value = decoder.decode_inline::<T>()?;
    decoder.finish()?;
    Ok(value)
}
