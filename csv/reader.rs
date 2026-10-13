#[cfg(feature = "serde")]
use alloc::collections::BTreeMap;
#[cfg(feature = "serde")]
use alloc::sync::Arc;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

use crate::error::{ReadError, ReadErrorKind};

/// Custom read abstraction that allows `Reader` to work without `std::io::Read`.
///
/// This trait is automatically implemented for `&[u8]` and, when the `std`
/// feature is enabled, for any `std::io::Read` type.
pub trait Read {
    /// Read data into `buf`, appending to its contents.
    ///
    /// Returns `Ok(true)` when at least one byte was appended to `buf`, or
    /// `Ok(false)` to signal end-of-input.
    ///
    /// Implementations **must not** return `Ok(true)` without appending at
    /// least one byte. Doing so is a contract violation; rather than spinning
    /// or panicking, the [`Reader`] reports [`ReadErrorKind::NoProgress`] and
    /// stops reading.
    fn read(&mut self, buf: &mut Vec<u8>) -> Result<bool, ReadError>;
}

#[cfg(not(feature = "std"))]
impl Read for &[u8] {
    fn read(&mut self, buf: &mut Vec<u8>) -> Result<bool, ReadError> {
        let chunk = 16384;
        let to_copy = chunk.min(self.len());
        if to_copy == 0 {
            return Ok(false);
        }
        buf.extend_from_slice(&self[..to_copy]);
        *self = &self[to_copy..];
        Ok(true)
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> Read for R {
    fn read(&mut self, buf: &mut Vec<u8>) -> Result<bool, ReadError> {
        let mut tmp = [0u8; 16384];
        let n = std::io::Read::read(self, &mut tmp)?;
        if n == 0 {
            return Ok(false);
        }
        buf.extend_from_slice(&tmp[..n]);
        Ok(true)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    StartOfField,
    InUnquoted,
    InQuoted,
    AfterQuote,
}

#[derive(Clone, Copy, Debug)]
enum Src {
    Buf,
    Scratch,
}

#[derive(Clone, Copy, Debug)]
struct RawField {
    start: usize,
    end: usize,
    src: Src,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldRange {
    pub start: usize,
    pub end: usize,
}

/// A pull-based CSV reader that parses records from any [`Read`] source.
///
/// The reader supports both in-memory slices (`&[u8]`) and streaming I/O sources
/// (`std::io::Read`) via the custom [`Read`] trait. Rows are parsed on-demand
/// by calling [`rows`](Self::rows) or [`rows_bytes`](Self::rows_bytes).
///
/// # Configuration
///
/// Use builder methods for a fluent setup:
/// - [`set_delimiter`](Self::set_delimiter) — change the field separator (default `,`)
/// - [`set_flexible`](Self::set_flexible) — allow variable-length rows (default `false`)
/// - [`set_headers`](Self::set_headers) — supply column names without parsing
/// - [`set_max_record_bytes`](Self::set_max_record_bytes),
///   [`set_max_field_bytes`](Self::set_max_field_bytes) and
///   [`set_max_columns`](Self::set_max_columns) — opt-in resource limits
///
/// # Headers
///
/// Call [`parse_headers`](Self::parse_headers) to treat the first row as column
/// names. After doing so, [`headers`](Self::headers) returns a `Some` slice and
/// serde deserialization (with the `serde` feature) can match by field name.
///
/// # Errors
///
/// Parsing errors (unterminated quotes, trailing content after a quoted field,
/// inconsistent field counts) are stored on the returned row and surfaced when
/// the row's fields are accessed. I/O errors are also stored on the row. This
/// allows iteration to continue past malformed rows: when a record is
/// malformed, the reader skips to the next record boundary before parsing the
/// following record.
///
/// Errors are reported against the line on which the offending record started.
///
/// # Resource limits
///
/// By default the reader imposes no size limits, so a single enormous record
/// can consume an unbounded amount of memory. Applications that parse
/// untrusted input should set [`set_max_record_bytes`](Self::set_max_record_bytes)
/// (and optionally [`set_max_field_bytes`](Self::set_max_field_bytes) /
/// [`set_max_columns`](Self::set_max_columns)); exceeding a limit produces an
/// error on the row instead of exhausting memory.
///
/// # Iteration
///
/// Produces owned [`Row`] or [`BytesRow`] values. A [`Row`] validates UTF-8
/// on field access; [`BytesRow`] does not. Rows can outlive the reader.
///
/// # Examples
///
/// ```rust
/// use csv::Reader;
///
/// let data = b"name,age\nAlice,30\nBob,25\n";
/// let mut reader = Reader::new(&data[..]);
///
/// // Parse the first row as headers.
/// let headers = reader.parse_headers().unwrap();
/// assert_eq!(headers, &["name", "age"]);
///
/// // Iterate over data rows.
/// for row in reader.rows() {
///     for field in row.iter() {
///         let _: &str = field.unwrap();
///     }
/// }
/// ```
///
/// ```no_run
/// # #[cfg(feature = "std")] {
/// # use csv::Reader;
/// let file = std::fs::File::open("data.csv").unwrap();
/// let mut reader = Reader::new(file).set_delimiter(b';');
///
/// for row in reader.rows() {
///     let name = row.get(0).unwrap().unwrap();
///     println!("{name}");
/// }
/// # }
/// ```
pub struct Reader<R: Read> {
    /// Raw input data read from source.
    buf: Vec<u8>,
    /// Temporary storage for quoted field content (unescaped).
    scratch: Vec<u8>,
    /// Field ranges into `buf` or `scratch` for the current row.
    ranges: Vec<RawField>,
    /// Start of unconsumed data in `buf`.
    start: usize,
    /// End of valid data in `buf`.
    end: usize,
    /// Number of bytes dropped from the front of `buf` by [`Reader::compact`].
    ///
    /// The absolute stream offset of `buf[start]` is `discarded + start`.
    discarded: usize,
    /// Absolute stream offset at which the current record started.
    record_start: usize,
    /// Physical line number (1-based) of the byte at `buf[start]`.
    line: usize,
    /// Physical line number on which the current record started.
    record_line: usize,
    /// Start of current unquoted field within `buf`.
    field_start: usize,
    /// Start of current quoted field within `scratch`.
    scratch_field_start: usize,
    /// Current parser state machine position.
    state: State,
    /// Field delimiter byte (default `,`).
    delimiter: u8,
    /// If true, rows may have varying field counts.
    flexible: bool,
    /// Expected field count per row, inferred from the first data row.
    num_fields: Option<usize>,
    /// True once `parse_headers` or `set_headers` has been called.
    headers_parsed: bool,
    /// A bare `\r` was the last byte of the previous buffer; next `\n` should be skipped.
    pending_cr: bool,
    /// The previous quoted-content slice ended with a bare `\r`; a leading `\n`
    /// in the next slice continues that line break and must not be counted twice.
    pending_lf_quoted: bool,
    /// True once the leading byte-order mark has been considered.
    bom_checked: bool,
    /// If true, a UTF-8 byte-order mark at the very start of the input is skipped.
    strip_bom: bool,
    /// Maximum accepted size of a single record, in bytes (opt-in).
    max_record_bytes: Option<usize>,
    /// Maximum accepted size of a single field, in bytes (opt-in).
    max_field_bytes: Option<usize>,
    /// Maximum accepted number of columns per record (opt-in).
    max_columns: Option<usize>,
    /// Column names, populated by `parse_headers` or `set_headers`.
    headers: Vec<String>,
    #[cfg(feature = "serde")]
    /// Map from header name -> column index for serde field lookup.
    header_map: Option<Arc<BTreeMap<String, usize>>>,
    #[cfg(feature = "serde")]
    /// True when `headers` contains duplicate names, which makes name-based
    /// deserialization ambiguous.
    duplicate_headers: bool,
    /// The underlying data source.
    source: R,
    /// True when the source has signalled end-of-input.
    eof: bool,
}

// ── Common methods (always available) ─────────────────────────────────

impl<R: Read> Reader<R> {
    /// Creates a new `Reader` from any [`Read`] source.
    ///
    /// This works with `&[u8]`, `std::io::Cursor`, `std::fs::File`, etc.
    pub fn new(source: R) -> Self {
        Reader {
            buf: Vec::with_capacity(65536),
            scratch: Vec::new(),
            ranges: Vec::new(),
            start: 0,
            end: 0,
            discarded: 0,
            record_start: 0,
            line: 1,
            record_line: 1,
            field_start: 0,
            scratch_field_start: 0,
            state: State::StartOfField,
            delimiter: b',',
            flexible: false,
            num_fields: None,
            headers_parsed: false,
            pending_cr: false,
            pending_lf_quoted: false,
            bom_checked: false,
            strip_bom: true,
            max_record_bytes: None,
            max_field_bytes: None,
            max_columns: None,
            headers: Vec::new(),
            #[cfg(feature = "serde")]
            header_map: None,
            #[cfg(feature = "serde")]
            duplicate_headers: false,
            source,
            eof: false,
        }
    }

    /// Sets the field delimiter byte (default is `,`).
    ///
    /// # Panics
    ///
    /// Panics if `byte` is `"`, `\r` or `\n`. These bytes carry structural
    /// meaning in CSV and cannot be used as a delimiter: `"` would disable
    /// quoting, and `\r`/`\n` would make record boundaries unrepresentable,
    /// causing the whole input to be read as a single record.
    pub fn set_delimiter(mut self, byte: u8) -> Self {
        assert!(
            byte != b'"' && byte != b'\r' && byte != b'\n',
            "invalid CSV delimiter {byte:#04x}: `\"`, `\\r` and `\\n` are not allowed"
        );
        self.delimiter = byte;
        self
    }

    /// Sets whether variable field counts are allowed (default is `false`).
    pub fn set_flexible(mut self, yes: bool) -> Self {
        self.flexible = yes;
        self
    }

    /// Controls whether a UTF-8 byte-order mark (`U+FEFF`, the bytes
    /// `EF BB BF`) at the very start of the input is skipped (default is
    /// `true`).
    ///
    /// This is what makes files produced by Excel, which writes UTF-8 CSV with
    /// a BOM by default, work with [`parse_headers`](Self::parse_headers) and
    /// serde: without it the first header would be `"\u{feff}name"` and would
    /// match no struct field.
    ///
    /// Only the first three bytes of the stream are considered, and only once.
    /// A BOM anywhere else in the input is always treated as data.
    ///
    /// Pass `false` to keep the mark as part of the first field.
    pub fn set_strip_bom(mut self, yes: bool) -> Self {
        self.strip_bom = yes;
        self
    }

    /// Rejects any record longer than `max` bytes.
    ///
    /// This is the main defence against a single crafted record exhausting
    /// memory. When a record exceeds the limit the row carries a
    /// [`ReadErrorKind::RecordTooLarge`] error and reading stops, because the
    /// remainder of the oversized record cannot be skipped safely.
    ///
    /// By default no limit is applied.
    pub fn set_max_record_bytes(mut self, max: usize) -> Self {
        self.max_record_bytes = Some(max);
        self
    }

    /// Rejects any field longer than `max` bytes.
    ///
    /// When a field exceeds the limit the row carries a
    /// [`ReadErrorKind::FieldTooLarge`] error and **reading stops**, because the
    /// remainder of the oversized record cannot be skipped without reading it.
    ///
    /// By default no limit is applied.
    pub fn set_max_field_bytes(mut self, max: usize) -> Self {
        self.max_field_bytes = Some(max);
        self
    }

    /// Rejects any record with more than `max` columns.
    ///
    /// Each column costs at least one byte of input, so this is a cheap way to
    /// bound the bookkeeping memory used by a record made of millions of empty
    /// fields. When a record exceeds the limit the row carries a
    /// [`ReadErrorKind::TooManyColumns`] error and **reading stops**, because
    /// the remainder of the oversized record cannot be skipped without reading
    /// it.
    ///
    /// By default no limit is applied.
    pub fn set_max_columns(mut self, max: usize) -> Self {
        self.max_columns = Some(max);
        self
    }

    /// Sets the column names for header-based serde deserialization.
    ///
    /// Calling this marks headers as parsed so `reader.headers()` returns `Some`,
    /// and pins the expected field count to the number of names supplied (unless
    /// [`set_flexible`](Self::set_flexible) is enabled), exactly like
    /// [`parse_headers`](Self::parse_headers).
    pub fn set_headers(mut self, headers: Vec<String>) -> Self {
        self.headers_parsed = true;
        if !headers.is_empty() {
            self.num_fields = Some(headers.len());
        }
        self.headers = headers;
        self.rebuild_header_map();
        self
    }

    #[cfg(feature = "serde")]
    fn rebuild_header_map(&mut self) {
        let map: BTreeMap<String, usize> = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), i))
            .collect();
        self.duplicate_headers = map.len() != self.headers.len();
        self.header_map = Some(Arc::new(map));
    }

    #[cfg(not(feature = "serde"))]
    fn rebuild_header_map(&mut self) {}

    /// Returns `Some(&[String])` if headers were parsed or set, `None` otherwise.
    pub fn headers(&self) -> Option<&[String]> {
        if self.headers_parsed { Some(&self.headers) } else { None }
    }

    /// Parses the first row as column headers and stores them internally.
    ///
    /// Returns the header strings as a slice, and pins the expected field count
    /// to the number of headers (unless [`set_flexible`](Self::set_flexible) is
    /// enabled).
    ///
    /// If the input is empty this returns an empty slice and leaves
    /// [`headers`](Self::headers) as `None`, since no header row was seen.
    pub fn parse_headers(&mut self) -> Result<&[String], ReadError> {
        let row = match self.read_row() {
            Some(row) => row,
            None => return Ok(&[]),
        };
        let headers: Vec<String> = row.to_vec()?.iter().map(|s| s.to_string()).collect();
        self.headers_parsed = true;
        self.num_fields = Some(headers.len());
        self.headers = headers;
        self.rebuild_header_map();
        Ok(&self.headers)
    }

    /// Returns an iterator over `Row` values (validated as UTF-8).
    pub fn rows(&mut self) -> Rows<'_, R> {
        Rows {
            reader: self,
        }
    }

    /// Returns an iterator over `BytesRow` values (no UTF-8 validation).
    pub fn rows_bytes(&mut self) -> BytesRows<'_, R> {
        BytesRows {
            reader: self,
        }
    }

    /// Reads more data from the source into `buf`, updating `self.end`.
    ///
    /// Returns `Ok(true)` if new data was loaded, `Ok(false)` at EOF, or `Err` on
    /// I/O error or when the source violates the [`Read`] contract by reporting
    /// progress without appending any bytes.
    ///
    /// Once EOF is reached, subsequent calls return `Ok(false)` without re-reading.
    fn fill_buf(&mut self) -> Result<bool, ReadError> {
        if self.eof {
            return Ok(false);
        }
        let before = self.buf.len();
        if self.source.read(&mut self.buf)? {
            if self.buf.len() == before {
                // Contract violation: `Ok(true)` without progress. Report it
                // instead of looping forever or indexing out of bounds.
                self.eof = true;
                return Err(ReadError::new(ReadErrorKind::NoProgress, self.record_line, 0));
            }
            self.end = self.buf.len();
            Ok(true)
        } else {
            self.eof = true;
            Ok(false)
        }
    }

    /// Skips a UTF-8 byte-order mark at the very start of the input.
    ///
    /// The decision needs up to three bytes, so this may read ahead before the
    /// first record is parsed. That is necessary: with a byte-at-a-time source
    /// the leading `0xEF` would otherwise be consumed as field content before
    /// the mark could be recognised.
    fn strip_leading_bom(&mut self) -> Result<(), ReadError> {
        if self.bom_checked {
            return Ok(());
        }
        self.bom_checked = true;
        if !self.strip_bom {
            return Ok(());
        }
        while self.end - self.start < 3 && !self.eof {
            self.fill_buf()?;
        }
        if self.buf[self.start..].starts_with(&crate::BOM) {
            self.start += 3;
        }
        Ok(())
    }

    /// Parses one CSV row from the input, or returns `None` at EOF.
    ///
    /// The row's fields are assembled into an owned `Row`. Errors (I/O,
    /// unterminated quotes, trailing content, inconsistent field counts,
    /// configured size limits) are stored on the `Row` and deferred until
    /// access.
    fn read_row(&mut self) -> Option<Row> {
        self.compact();
        self.ranges.clear();
        self.scratch.clear();
        self.state = State::StartOfField;
        self.pending_lf_quoted = false;
        self.record_line = self.line;

        if let Err(e) = self.strip_leading_bom() {
            self.eof = true;
            return Some(self.build_row(Some(e)));
        }

        self.record_start = self.discarded + self.start;

        loop {
            if self.start >= self.end {
                match self.fill_buf() {
                    Err(e) => {
                        self.eof = true;
                        return match self.state {
                            State::InQuoted => Some(self.build_row(Some(e))),
                            _ => {
                                self.finalize_current_field();
                                Some(self.build_row(Some(e)))
                            }
                        };
                    }
                    Ok(false) => {
                        if self.ranges.is_empty() && self.state == State::StartOfField {
                            return None;
                        }
                        return match self.state {
                            State::InQuoted => Some(self.build_row(Some(ReadError::new(
                                ReadErrorKind::UnterminatedQuote,
                                self.record_line,
                                0,
                            )))),
                            _ => {
                                self.finalize_current_field();
                                Some(self.build_row(None))
                            }
                        };
                    }
                    Ok(true) => {}
                }
            }

            // Enforce the opt-in resource limits before touching the byte, so a
            // malicious record cannot grow the buffers any further.
            if let Some(kind) = self.limit_exceeded() {
                return Some(self.abandon_record(kind));
            }

            if self.pending_cr {
                if self.buf[self.start] == b'\n' {
                    self.start += 1;
                }
                self.pending_cr = false;
                continue;
            }

            let byte = self.buf[self.start];

            match self.state {
                State::StartOfField => {
                    if byte == b'\r' || byte == b'\n' {
                        if !self.ranges.is_empty() {
                            self.ranges.push(RawField {
                                start: self.start,
                                end: self.start,
                                src: Src::Buf,
                            });
                        }
                        self.consume_newline();
                        if self.ranges.is_empty() {
                            continue;
                        }
                        return Some(self.build_row(None));
                    }
                    self.field_start = self.start;
                    if byte == self.delimiter {
                        self.ranges.push(RawField {
                            start: self.start,
                            end: self.start,
                            src: Src::Buf,
                        });
                        self.start += 1;
                        continue;
                    }
                    if byte == b'"' {
                        self.scratch_field_start = self.scratch.len();
                        self.start += 1;
                        self.state = State::InQuoted;
                    } else {
                        self.start += 1;
                        self.state = State::InUnquoted;
                    }
                }

                State::InUnquoted => {
                    let haystack = &self.buf[self.start..self.end];
                    match memchr::memchr3(self.delimiter, b'\r', b'\n', haystack) {
                        Some(offset) => {
                            let pos = self.start + offset;
                            if let Some(kind) = self.field_limit_exceeded(pos - self.field_start) {
                                return Some(self.abandon_record(kind));
                            }
                            self.ranges.push(RawField {
                                start: self.field_start,
                                end: pos,
                                src: Src::Buf,
                            });
                            let b = self.buf[pos];
                            if b == self.delimiter {
                                self.start = pos + 1;
                                self.state = State::StartOfField;
                            } else {
                                self.start = pos;
                                self.consume_newline();
                                return Some(self.build_row(None));
                            }
                        }
                        None => {
                            self.start = self.end;
                            if let Some(kind) = self.field_limit_exceeded(self.end - self.field_start) {
                                return Some(self.abandon_record(kind));
                            }
                        }
                    }
                }

                State::InQuoted => {
                    let haystack = &self.buf[self.start..self.end];
                    match memchr::memchr(b'"', haystack) {
                        Some(offset) => {
                            let quote_pos = self.start + offset;
                            let segment = &self.buf[self.start..quote_pos];
                            // Check before copying into `scratch`, so the limit
                            // bounds the scratch buffer too.
                            if let Some(kind) =
                                self.field_limit_exceeded(self.scratch.len() - self.scratch_field_start + segment.len())
                            {
                                return Some(self.abandon_record(kind));
                            }
                            self.line += count_line_breaks(segment, &mut self.pending_lf_quoted);
                            // A `"` follows, so a trailing `\r` cannot pair with
                            // anything and has already been counted.
                            self.pending_lf_quoted = false;
                            self.scratch.extend_from_slice(segment);
                            let after_quote = quote_pos + 1;
                            if after_quote < self.end && self.buf[after_quote] == b'"' {
                                self.scratch.push(b'"');
                                self.start = after_quote + 1;
                            } else if after_quote < self.end {
                                self.ranges.push(RawField {
                                    start: self.scratch_field_start,
                                    end: self.scratch.len(),
                                    src: Src::Scratch,
                                });
                                self.start = after_quote;
                                self.state = State::AfterQuote;
                            } else {
                                match self.fill_buf() {
                                    Ok(true) if self.buf.get(after_quote) == Some(&b'"') => {
                                        self.scratch.push(b'"');
                                        self.start = after_quote + 1;
                                    }
                                    Err(e) => {
                                        self.ranges.push(RawField {
                                            start: self.scratch_field_start,
                                            end: self.scratch.len(),
                                            src: Src::Scratch,
                                        });
                                        self.start = after_quote;
                                        self.state = State::AfterQuote;
                                        return Some(self.build_row(Some(e)));
                                    }
                                    _ => {
                                        self.ranges.push(RawField {
                                            start: self.scratch_field_start,
                                            end: self.scratch.len(),
                                            src: Src::Scratch,
                                        });
                                        self.start = after_quote;
                                        self.state = State::AfterQuote;
                                    }
                                }
                            }
                        }
                        None => {
                            self.line +=
                                count_line_breaks(&self.buf[self.start..self.end], &mut self.pending_lf_quoted);
                            self.scratch.extend_from_slice(&self.buf[self.start..self.end]);
                            self.start = self.end;
                            if let Some(kind) = self.field_limit_exceeded(self.scratch.len() - self.scratch_field_start)
                            {
                                return Some(self.abandon_record(kind));
                            }
                        }
                    }
                }

                State::AfterQuote => {
                    if byte == self.delimiter {
                        self.start += 1;
                        self.state = State::StartOfField;
                    } else if is_newline(byte) {
                        self.consume_newline();
                        return Some(self.build_row(None));
                    } else {
                        return Some(self.abandon_record(ReadErrorKind::TrailingContent));
                    }
                }
            }
        }
    }

    /// Returns the first configured limit the current record has exceeded.
    ///
    /// Field sizes are checked where a field is resolved (see
    /// [`field_limit_exceeded`](Self::field_limit_exceeded)); this covers the
    /// record and column limits, which grow monotonically as parsing advances.
    fn limit_exceeded(&self) -> Option<ReadErrorKind> {
        if let Some(limit) = self.max_record_bytes {
            let consumed = self.discarded + self.start - self.record_start;
            if consumed > limit {
                return Some(ReadErrorKind::RecordTooLarge {
                    limit,
                });
            }
        }
        if let Some(limit) = self.max_columns
            && self.ranges.len() > limit
        {
            return Some(ReadErrorKind::TooManyColumns {
                limit,
            });
        }
        None
    }

    /// Returns [`ReadErrorKind::FieldTooLarge`] if a field of `len` bytes
    /// exceeds the configured field limit.
    fn field_limit_exceeded(&self, len: usize) -> Option<ReadErrorKind> {
        match self.max_field_bytes {
            Some(limit) if len > limit => Some(ReadErrorKind::FieldTooLarge {
                limit,
            }),
            _ => None,
        }
    }

    /// Records the fields parsed so far and abandons the current record,
    /// positioning the reader at the start of the next record where possible.
    fn abandon_record(&mut self, kind: ReadErrorKind) -> Row {
        let error = ReadError::new(kind, self.record_line, 0);
        match error.kind {
            // A configured limit was exceeded. Reading further would defeat it
            // (the remainder of the record could be arbitrarily long), so drop
            // the rest of the buffer and stop.
            ReadErrorKind::RecordTooLarge {
                ..
            }
            | ReadErrorKind::FieldTooLarge {
                ..
            }
            | ReadErrorKind::TooManyColumns {
                ..
            } => {
                self.start = self.end;
                self.eof = true;
            }
            _ => {
                // Keep the fields parsed before the offending byte, then skip
                // the junk so the next record starts at a record boundary.
                self.finalize_current_field();
                if !self.skip_record_tail() {
                    self.eof = true;
                }
            }
        }
        self.build_row(Some(error))
    }

    /// Consumes input up to and including the next record terminator.
    ///
    /// Returns `false` if the skip could not complete (end of input, I/O error,
    /// or the configured record-size limit was reached), in which case the
    /// reader is left at end-of-input.
    fn skip_record_tail(&mut self) -> bool {
        loop {
            if self.start >= self.end {
                match self.fill_buf() {
                    Ok(true) => {}
                    _ => return false,
                }
            }
            if self.pending_cr {
                if self.buf[self.start] == b'\n' {
                    self.start += 1;
                }
                self.pending_cr = false;
                return true;
            }
            let haystack = &self.buf[self.start..self.end];
            match memchr::memchr2(b'\r', b'\n', haystack) {
                Some(offset) => {
                    self.start += offset;
                    self.consume_newline();
                    return true;
                }
                None => {
                    self.start = self.end;
                    if let Some(limit) = self.max_record_bytes
                        && self.discarded + self.start - self.record_start > limit
                    {
                        return false;
                    }
                }
            }
        }
    }

    /// Moves unconsumed data to the front of `buf` and truncates.
    ///
    /// Compaction is a full copy of the unconsumed tail, so it is only done
    /// once a meaningful amount of data has been consumed. Compacting on every
    /// record would memmove the whole remaining buffer per record, which
    /// dominates runtime for inputs made of small records.
    ///
    /// Only ever called at a record boundary, where no parsed field ranges are
    /// live.
    fn compact(&mut self) {
        if self.start == 0 {
            return;
        }
        if self.start < 4096 && self.start * 2 < self.buf.len() {
            return;
        }
        let shift = self.start;
        let remaining = self.end - shift;
        if remaining > 0 {
            self.buf.copy_within(shift..self.end, 0);
        }
        self.buf.truncate(remaining);
        self.end = remaining;
        self.start = 0;
        self.discarded += shift;

        // Give back memory retained from an unusually large record. This only
        // triggers once the buffer has grown far beyond its initial capacity,
        // so it never affects steady-state throughput.
        const RETAINED_CAPACITY: usize = 1 << 20;
        if self.buf.capacity() > RETAINED_CAPACITY && self.end <= RETAINED_CAPACITY {
            self.buf.shrink_to_fit();
        }
    }

    /// Consumes a `\r\n`, `\n`, or `\r` line ending, incrementing the line counter.
    ///
    /// If a bare `\r` is at the end of the buffer, sets `pending_cr` so the
    /// following `\n` is skipped on the next read.
    fn consume_newline(&mut self) {
        if self.start < self.end && self.buf[self.start] == b'\r' {
            self.start += 1;
            if self.start >= self.end {
                self.pending_cr = true;
            }
        }
        if self.start < self.end && self.buf[self.start] == b'\n' {
            self.start += 1;
        }
        self.line += 1;
    }

    /// Pushes a `RawField` entry for the current unquoted or empty field.
    ///
    /// Does nothing in the `AfterQuote` state (the field is already recorded).
    fn finalize_current_field(&mut self) {
        match self.state {
            State::InUnquoted => {
                self.ranges.push(RawField {
                    start: self.field_start,
                    end: self.start,
                    src: Src::Buf,
                });
            }
            State::InQuoted => {
                self.ranges.push(RawField {
                    start: self.scratch_field_start,
                    end: self.scratch.len(),
                    src: Src::Scratch,
                });
            }
            State::AfterQuote => {}
            State::StartOfField => {
                if !self.ranges.is_empty() {
                    self.ranges.push(RawField {
                        start: self.start,
                        end: self.start,
                        src: Src::Buf,
                    });
                }
            }
        }
    }

    /// Wraps the bytes row into a `Row`, attaching the header map if available.
    fn build_row(&mut self, error: Option<ReadError>) -> Row {
        let bytes_row = self.build_bytes_row(error);
        Row {
            inner: bytes_row,
            #[cfg(feature = "serde")]
            header_map: self.header_map.clone(),
            #[cfg(feature = "serde")]
            duplicate_headers: self.duplicate_headers,
        }
    }

    /// Assembles the parsed fields into an owned `BytesRow`, checking field-count consistency.
    fn build_bytes_row(&mut self, error: Option<ReadError>) -> BytesRow {
        let mut total: usize = 0;
        for r in &self.ranges {
            total += r.end - r.start;
        }

        let mut row_buf = Vec::with_capacity(total);
        let mut row_ranges = Vec::with_capacity(self.ranges.len());

        for r in &self.ranges {
            let slice = match r.src {
                Src::Buf => &self.buf[r.start..r.end],
                Src::Scratch => &self.scratch[r.start..r.end],
            };
            let start = row_buf.len();
            row_buf.extend_from_slice(slice);
            let end = row_buf.len();
            row_ranges.push(FieldRange {
                start,
                end,
            });
        }

        let field_count = row_ranges.len();

        let error = if !self.flexible {
            error.or_else(|| match self.num_fields {
                Some(expected) if field_count != expected => Some(ReadError::new(
                    ReadErrorKind::InconsistentFieldCount {
                        expected,
                        found: field_count,
                    },
                    self.record_line,
                    0,
                )),
                _ => None,
            })
        } else {
            error
        };

        // Only a well-formed record may establish the expected field count.
        // Otherwise one malformed record would cascade errors over the rest of
        // the input.
        if self.num_fields.is_none() && field_count > 0 && error.is_none() {
            self.num_fields = Some(field_count);
        }

        BytesRow {
            buf: row_buf,
            ranges: row_ranges,
            error,
            line: self.record_line,
        }
    }
}

/// Counts line breaks in a slice of quoted-field content.
///
/// A `\r` immediately followed by `\n` counts as a single break, even when the
/// pair is split across two slices: `pending_cr` is set when a slice ends on a
/// bare `\r` and consulted by the following call.
///
/// Quoted content usually contains no line breaks at all, so a `memchr2`
/// pre-scan (which is vectorised) keeps this off the hot path.
fn count_line_breaks(bytes: &[u8], pending_cr: &mut bool) -> usize {
    if memchr::memchr2(b'\r', b'\n', bytes).is_none() {
        // No break in this slice, so a pending `\r` cannot pair with a leading
        // `\n` here; any consumed byte clears it.
        if !bytes.is_empty() {
            *pending_cr = false;
        }
        return 0;
    }

    let mut breaks = 0;
    let mut i = 0;
    if *pending_cr {
        if bytes.first() == Some(&b'\n') {
            i = 1;
        }
        *pending_cr = false;
    }
    while i < bytes.len() {
        match bytes[i] {
            b'\n' => {
                breaks += 1;
                i += 1;
            }
            b'\r' => {
                breaks += 1;
                i += 1;
                if i < bytes.len() {
                    if bytes[i] == b'\n' {
                        i += 1;
                    }
                } else {
                    *pending_cr = true;
                }
            }
            _ => i += 1,
        }
    }
    breaks
}

fn is_newline(b: u8) -> bool {
    b == b'\n' || b == b'\r'
}

// ── BytesRow ──────────────────────────────────────────────────────────

pub struct BytesRow {
    pub(crate) buf: Vec<u8>,
    pub(crate) ranges: Vec<FieldRange>,
    pub(crate) error: Option<ReadError>,
    pub(crate) line: usize,
}

impl BytesRow {
    /// The parse error for this row, if any.
    ///
    /// Unlike [`Row`], a `BytesRow` never fails: the error is stored here and
    /// the raw field bytes remain accessible through [`get`](Self::get) and
    /// [`iter`](Self::iter). [`to_vec`](Self::to_vec) is the accessor that
    /// surfaces the error.
    pub fn error(&self) -> Option<&ReadError> {
        self.error.as_ref()
    }

    pub fn len(&self) -> usize {
        self.ranges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&[u8]> {
        self.ranges.get(index).map(|r| &self.buf[r.start..r.end])
    }

    pub fn to_vec(&self) -> Result<Vec<&[u8]>, ReadError> {
        if let Some(ref e) = self.error {
            return Err(e.clone());
        }
        Ok(self.ranges.iter().map(|r| &self.buf[r.start..r.end]).collect())
    }

    pub fn iter(&self) -> BytesFields<'_> {
        BytesFields {
            buf: &self.buf,
            iter: self.ranges.iter(),
        }
    }
}

impl fmt::Debug for BytesRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error {
            Some(e) => write!(f, "BytesRow(Err({e}))"),
            None => f
                .debug_list()
                .entries(self.ranges.iter().map(|r| &self.buf[r.start..r.end]))
                .finish(),
        }
    }
}

pub struct BytesFields<'a> {
    buf: &'a [u8],
    iter: core::slice::Iter<'a, FieldRange>,
}

impl<'a> Iterator for BytesFields<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]> {
        let r = self.iter.next()?;
        Some(&self.buf[r.start..r.end])
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for BytesFields<'a> {}

// ── Row ───────────────────────────────────────────────────────────────

pub struct Row {
    pub(crate) inner: BytesRow,
    #[cfg(feature = "serde")]
    pub(crate) header_map: Option<Arc<BTreeMap<String, usize>>>,
    #[cfg(feature = "serde")]
    pub(crate) duplicate_headers: bool,
}

impl Row {
    /// The parse error for this row, if any.
    pub fn error(&self) -> Option<&ReadError> {
        self.inner.error()
    }

    /// Number of fields parsed for this row.
    ///
    /// Fields are still counted when the row carries an error.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the field at `index`, or `None` if there is no such field.
    ///
    /// Consistent with [`to_vec`](Self::to_vec) and [`iter`](Self::iter): if
    /// the row carries a parse error, every in-range index yields `Err`. Only
    /// an out-of-range index yields `None`.
    pub fn get(&self, index: usize) -> Option<Result<&str, ReadError>> {
        self.inner.get(index).map(|bytes| {
            if let Some(ref e) = self.inner.error {
                return Err(e.clone());
            }
            core::str::from_utf8(bytes).map_err(|_| ReadError::new(ReadErrorKind::InvalidUtf8, self.inner.line, 0))
        })
    }

    /// Returns all fields as `&str`, or the row's parse error.
    pub fn to_vec(&self) -> Result<Vec<&str>, ReadError> {
        if let Some(ref e) = self.inner.error {
            return Err(e.clone());
        }
        self.inner
            .ranges
            .iter()
            .map(|r| {
                core::str::from_utf8(&self.inner.buf[r.start..r.end])
                    .map_err(|_| ReadError::new(ReadErrorKind::InvalidUtf8, self.inner.line, 0))
            })
            .collect()
    }

    /// Iterates over the fields, yielding the row's parse error for each field
    /// when the row is malformed.
    pub fn iter(&self) -> Fields<'_> {
        Fields {
            buf: &self.inner.buf,
            iter: self.inner.ranges.iter(),
            error: self.inner.error.as_ref().map(|e| &e.kind),
            line: self.inner.line,
        }
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner.error {
            Some(e) => write!(f, "Row(Err({e}))"),
            None => {
                f.debug_list()
                    .entries(
                        self.inner.ranges.iter().map(|r| {
                            core::str::from_utf8(&self.inner.buf[r.start..r.end]).unwrap_or("<invalid utf-8>")
                        }),
                    )
                    .finish()
            }
        }
    }
}

pub struct Fields<'a> {
    buf: &'a [u8],
    iter: core::slice::Iter<'a, FieldRange>,
    error: Option<&'a ReadErrorKind>,
    line: usize,
}

impl<'a> Iterator for Fields<'a> {
    type Item = Result<&'a str, ReadError>;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.iter.next()?;
        if let Some(kind) = self.error {
            return Some(Err(ReadError::new(kind.clone(), self.line, 0)));
        }
        Some(
            core::str::from_utf8(&self.buf[r.start..r.end])
                .map_err(|_| ReadError::new(ReadErrorKind::InvalidUtf8, self.line, 0)),
        )
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Fields<'a> {}

// ── Iterators ─────────────────────────────────────────────────────────

pub struct BytesRows<'r, R: Read> {
    reader: &'r mut Reader<R>,
}

impl<'r, R: Read> Iterator for BytesRows<'r, R> {
    type Item = BytesRow;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_row().map(|r| r.inner)
    }
}

pub struct Rows<'r, R: Read> {
    reader: &'r mut Reader<R>,
}

impl<'r, R: Read> Iterator for Rows<'r, R> {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_row()
    }
}
