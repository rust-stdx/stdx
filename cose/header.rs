//! COSE header maps: protected and unprotected buckets.

use core::fmt;

use cbor::{Decoder, Encoder, Type};
use heapless::Vec as HeaplessVec;

use crate::{
    Algorithm, ContentType, Error, Label, Result,
    bytes::Bytes,
    codec::{
        Cursor, MAX_MAP_ENTRIES, cbor_len_array, cbor_len_bytes, cbor_len_int, cbor_len_str, cbor_len_uint,
        decode_bytes, decode_i64, decoder, find_value, label_len, labels_equal, options, take_raw, validate_map,
        value_len, write_label, write_value,
    },
};

/// The role of a header map.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HeaderRole {
    /// The cryptographically protected bucket.
    Protected,
    /// The unprotected bucket.
    Unprotected,
}

/// One COSE header map, encoded as CBOR.
///
/// The map is kept in its encoded form. This preserves unknown and private
/// header parameters exactly, which is required for the protected bucket: its
/// bytes are fed verbatim into `Sig_structure`/`MAC_structure`.
///
/// With the `alloc` feature the map may own its bytes; without it, the map
/// always borrows from the caller.
#[derive(Clone)]
pub struct Header<'a> {
    raw: Bytes<'a>,
}

impl fmt::Debug for Header<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header").field("len", &self.as_bytes().len()).finish()
    }
}

impl<'a> Header<'a> {
    /// Parses and structurally validates a header map.
    ///
    /// An empty slice is accepted and means "empty protected header". Any
    /// non-empty input must be a definite-length CBOR map with unique labels.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Decode`] for malformed CBOR,
    /// [`Error::IndefiniteLength`] for an indefinite-length map,
    /// [`Error::DuplicateLabel`] for duplicate labels,
    /// [`Error::TooManyEntries`] for more than 32 entries, and
    /// [`Error::TrailingBytes`] if extra bytes follow the map.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self> {
        validate_map(bytes)?;
        Ok(Self {
            raw: Bytes::borrowed(bytes),
        })
    }

    /// Parses a header map and takes ownership of its bytes.
    #[cfg(feature = "alloc")]
    pub fn from_owned(bytes: alloc::vec::Vec<u8>) -> Result<Self> {
        validate_map(&bytes)?;
        Ok(Self {
            raw: Bytes::owned(bytes),
        })
    }

    /// An empty protected header (`h''`).
    #[inline]
    pub const fn empty_protected() -> Header<'static> {
        Header {
            raw: Bytes::Borrowed(&[]),
        }
    }

    /// An empty unprotected header (`h'a0'`).
    #[inline]
    pub const fn empty_unprotected() -> Header<'static> {
        Header {
            raw: Bytes::Borrowed(&[0xa0]),
        }
    }

    /// Starts building a header map.
    ///
    /// The builder's lifetime is inferred from borrowed arguments such as
    /// [`HeaderBuilder::key_id`].
    #[inline]
    pub fn builder<'b>() -> HeaderBuilder<'b> {
        HeaderBuilder::new()
    }

    /// Returns the encoded header map.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.raw.as_slice()
    }

    /// Returns true if the header is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        let bytes = self.as_bytes();
        bytes.is_empty() || bytes == [0xa0]
    }

    /// Copies the encoded header map into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        self.as_bytes().to_vec()
    }

    /// Returns the `alg` header parameter.
    ///
    /// Returns `None` if the parameter is absent or malformed. Use
    /// [`Header::validate`] for strict checking.
    pub fn algorithm(&self) -> Option<Algorithm> {
        let raw = self.raw_value(Label::Int(1))?;
        decode_i64(raw).ok().map(Algorithm::from_i64)
    }

    /// Returns the `kid` header parameter.
    ///
    /// Returns `None` if the parameter is absent or malformed.
    pub fn kid(&self) -> Option<&[u8]> {
        let raw = self.raw_value(Label::Int(4))?;
        crate::codec::decode_bytes(raw).ok()
    }

    /// Returns the `content type` header parameter.
    pub fn content_type(&self) -> Result<Option<ContentType<'_>>> {
        let Some(raw) = self.raw_value(Label::Int(3)) else {
            return Ok(None);
        };
        let mut d = decoder(raw);
        let content_type = match d.datatype()? {
            Type::Text => ContentType::Text(d.text()?),
            Type::U8 | Type::U16 | Type::U32 | Type::U64 => ContentType::Uint(d.u64()?),
            _ => return Err(Error::InvalidHeader),
        };
        if d.position() != raw.len() {
            return Err(Error::TrailingBytes);
        }
        Ok(Some(content_type))
    }

    /// Returns the `crit` header parameters.
    pub fn crit(&self) -> Result<Crit<'_>> {
        let Some(raw) = self.raw_value(Label::Int(2)) else {
            return Ok(Crit {
                d: decoder(&[]),
                remaining: 0,
            });
        };
        let mut d = decoder(raw);
        let len = d.array()?.ok_or(Error::IndefiniteLength)?;
        if len == 0 || len > MAX_MAP_ENTRIES {
            return Err(Error::InvalidHeader);
        }
        Ok(Crit {
            d,
            remaining: len,
        })
    }

    /// Returns the raw encoding of a header parameter value.
    pub fn raw<'b>(&'b self, label: impl Into<Label<'b>>) -> Option<&'b [u8]> {
        find_value(self.as_bytes(), label.into()).ok().flatten()
    }

    /// Decodes a header parameter value with `cbor`.
    ///
    /// Nested maps inside private parameter values are decoded with
    /// duplicate keys allowed; the application owns their interpretation.
    pub fn decode<'b, T>(&'b self, label: impl Into<Label<'b>>) -> Result<Option<T>>
    where
        T: cbor::Decode<'b>,
    {
        let Some(raw) = self.raw(label) else {
            return Ok(None);
        };
        Ok(Some(cbor::from_slice_with_options(raw, options())?))
    }

    /// Iterates over all header parameters.
    pub fn iter(&self) -> HeaderIter<'_> {
        let input = self.as_bytes();
        if input.is_empty() {
            return HeaderIter {
                input,
                d: decoder(input),
                remaining: 0,
            };
        }

        let mut d = decoder(input);
        let remaining = match d.map() {
            Ok(Some(len)) => len,
            _ => 0,
        };
        HeaderIter {
            input,
            d,
            remaining,
        }
    }

    /// Structurally validates the header map for the given role.
    ///
    /// This checks value types, duplicate `IV`/`Partial IV`, `crit` placement,
    /// and that every label listed in `crit` is present in this map. It does not
    /// decide whether the application understands a critical label; see
    /// [`Headers::validate_with`].
    ///
    /// Empty input is accepted for [`HeaderRole::Protected`]. It is rejected
    /// for [`HeaderRole::Unprotected`], which must be an encoded map.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidHeader`] for malformed known parameters,
    /// empty `kid`/`IV`/`Partial IV`, invalid content-type syntax, an
    /// out-of-range `alg`, duplicate `crit` entries, both `IV` and
    /// `Partial IV`, or a `crit` label that is not present in this map.
    /// Returns [`Error::CriticalHeaderNotProtected`] if `crit` appears in an
    /// unprotected header.
    pub fn validate(&self, role: HeaderRole) -> Result<()> {
        self.scan(role).map(|_| ())
    }

    /// Single-pass structural validation that also returns the map labels and
    /// the IV flags for cross-bucket checks.
    fn scan<'s>(&'s self, role: HeaderRole) -> Result<HeaderScan<'s>> {
        let bytes = self.as_bytes();
        if bytes.is_empty() {
            if role == HeaderRole::Unprotected {
                return Err(Error::InvalidHeader);
            }
            return Ok(HeaderScan {
                labels: HeaplessVec::new(),
                has_iv: false,
                has_partial_iv: false,
            });
        }

        let mut d = decoder(bytes);
        let len = d.map()?.ok_or(Error::IndefiniteLength)?;
        if len > MAX_MAP_ENTRIES {
            return Err(Error::TooManyEntries);
        }

        let mut labels: HeaplessVec<Label<'s>, MAX_MAP_ENTRIES> = HeaplessVec::new();
        let mut crit_labels: HeaplessVec<Label<'s>, MAX_MAP_ENTRIES> = HeaplessVec::new();
        let mut has_iv = false;
        let mut has_partial_iv = false;

        for _ in 0..len {
            let label = crate::codec::decode_label(&mut d)?;
            if labels.iter().any(|seen| labels_equal(*seen, label)) {
                return Err(Error::DuplicateLabel);
            }
            labels.push(label).map_err(|_| Error::TooManyEntries)?;

            let start = d.position();
            d.skip()?;
            let raw = &bytes[start..d.position()];

            match label {
                Label::Int(1) => {
                    if decode_i64(raw).is_err() {
                        return Err(Error::InvalidHeader);
                    }
                }
                Label::Int(2) => {
                    if role == HeaderRole::Unprotected {
                        return Err(Error::CriticalHeaderNotProtected);
                    }
                    let mut crit = decoder(raw);
                    let count = crit.array()?.ok_or(Error::IndefiniteLength)?;
                    if count == 0 || count > MAX_MAP_ENTRIES {
                        return Err(Error::InvalidHeader);
                    }
                    for _ in 0..count {
                        let crit_label = crate::codec::decode_label(&mut crit)?;
                        if crit_labels.iter().any(|entry| labels_equal(*entry, crit_label)) {
                            return Err(Error::InvalidHeader);
                        }
                        crit_labels.push(crit_label).map_err(|_| Error::TooManyEntries)?;
                    }
                    if crit.position() != raw.len() {
                        return Err(Error::TrailingBytes);
                    }
                }
                Label::Int(3) => validate_content_type(raw)?,
                Label::Int(4) => {
                    let kid = decode_bytes(raw).map_err(|_| Error::InvalidHeader)?;
                    if kid.is_empty() {
                        return Err(Error::InvalidHeader);
                    }
                }
                Label::Int(5) => {
                    let iv = decode_bytes(raw).map_err(|_| Error::InvalidHeader)?;
                    if iv.is_empty() {
                        return Err(Error::InvalidHeader);
                    }
                    has_iv = true;
                }
                Label::Int(6) => {
                    let partial_iv = decode_bytes(raw).map_err(|_| Error::InvalidHeader)?;
                    if partial_iv.is_empty() {
                        return Err(Error::InvalidHeader);
                    }
                    has_partial_iv = true;
                }
                _ => {}
            }
        }

        if d.position() != bytes.len() {
            return Err(Error::TrailingBytes);
        }
        if has_iv && has_partial_iv {
            return Err(Error::InvalidHeader);
        }
        for crit_label in &crit_labels {
            if !labels.iter().any(|label| labels_equal(*label, *crit_label)) {
                return Err(Error::InvalidHeader);
            }
        }

        Ok(HeaderScan {
            labels,
            has_iv,
            has_partial_iv,
        })
    }

    fn raw_value(&self, label: Label<'_>) -> Option<&[u8]> {
        find_value(self.as_bytes(), label).ok().flatten()
    }
}

struct HeaderScan<'a> {
    labels: HeaplessVec<Label<'a>, MAX_MAP_ENTRIES>,
    has_iv: bool,
    has_partial_iv: bool,
}

fn validate_content_type(raw: &[u8]) -> Result<()> {
    let mut d = decoder(raw);
    match d.datatype().map_err(|_| Error::InvalidHeader)? {
        Type::Text => {
            let text = d.text().map_err(|_| Error::InvalidHeader)?;
            if text.is_empty() || text.trim() != text || text.matches('/').count() != 1 {
                return Err(Error::InvalidHeader);
            }
        }
        Type::U8 | Type::U16 | Type::U32 | Type::U64 => {
            d.u64().map_err(|_| Error::InvalidHeader)?;
        }
        _ => return Err(Error::InvalidHeader),
    }
    if d.position() != raw.len() {
        return Err(Error::TrailingBytes);
    }
    Ok(())
}

/// A COSE header parameter value.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HeaderValue<'a> {
    /// A negative or positive integer that fits in `i64`.
    Int(i64),
    /// An unsigned integer.
    Uint(u64),
    /// A byte string.
    Bytes(&'a [u8]),
    /// A text string.
    Text(&'a str),
    /// A boolean.
    Bool(bool),
    /// CBOR null.
    Null,
    /// Any array/map/tag/float/simple value, as raw encoded CBOR.
    Raw(&'a [u8]),
}

/// Iterator over the labels in a `crit` array.
pub struct Crit<'a> {
    d: Decoder<'a>,
    remaining: usize,
}

impl<'a> Iterator for Crit<'a> {
    type Item = Result<Label<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let item = crate::codec::decode_label(&mut self.d);
        if item.is_err() {
            self.remaining = 0;
        }
        Some(item)
    }
}

/// Iterator over a header map.
pub struct HeaderIter<'a> {
    input: &'a [u8],
    d: Decoder<'a>,
    remaining: usize,
}

impl<'a> Iterator for HeaderIter<'a> {
    type Item = Result<(Label<'a>, HeaderValue<'a>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;

        let result = (|| -> Result<(Label<'a>, HeaderValue<'a>)> {
            let label = crate::codec::decode_label(&mut self.d)?;
            let value = match self.d.datatype()? {
                Type::U8 | Type::U16 | Type::U32 | Type::U64 => HeaderValue::Uint(self.d.u64()?),
                Type::I8 | Type::I16 | Type::I32 | Type::I64 => HeaderValue::Int(self.d.i64()?),
                Type::Bytes => HeaderValue::Bytes(self.d.bytes()?),
                Type::Text => HeaderValue::Text(self.d.text()?),
                Type::Bool => HeaderValue::Bool(self.d.bool()?),
                Type::Null => {
                    self.d.null()?;
                    HeaderValue::Null
                }
                _ => HeaderValue::Raw(take_raw(self.input, &mut self.d)?),
            };
            Ok((label, value))
        })();

        match result {
            Ok(item) => Some(Ok(item)),
            Err(err) => {
                self.remaining = 0;
                Some(Err(err))
            }
        }
    }
}

/// A protected and unprotected header pair.
pub struct Headers<'a> {
    protected: Header<'a>,
    unprotected: Header<'a>,
}

impl<'a> Headers<'a> {
    /// Validates and combines the two header buckets.
    ///
    /// Both buckets are parsed in a single pass. The unprotected bucket must
    /// be a non-empty encoded map; the protected bucket may be empty.
    /// Duplicate labels across the two buckets, and the simultaneous presence
    /// of `IV` and `Partial IV`, are rejected.
    ///
    /// # Errors
    ///
    /// Propagates structural header errors from [`Header::validate`], and
    /// returns [`Error::DuplicateLabel`] when a label appears in both buckets.
    pub fn new(protected: Header<'a>, unprotected: Header<'a>) -> Result<Self> {
        let protected_scan = protected.scan(HeaderRole::Protected)?;
        let unprotected_scan = unprotected.scan(HeaderRole::Unprotected)?;

        let duplicate = protected_scan.labels.iter().any(|protected_label| {
            unprotected_scan
                .labels
                .iter()
                .any(|label| labels_equal(*protected_label, *label))
        });
        let iv_conflict = (protected_scan.has_iv || unprotected_scan.has_iv)
            && (protected_scan.has_partial_iv || unprotected_scan.has_partial_iv);
        drop(protected_scan);
        drop(unprotected_scan);

        if duplicate {
            return Err(Error::DuplicateLabel);
        }
        if iv_conflict {
            return Err(Error::InvalidHeader);
        }

        Ok(Self {
            protected,
            unprotected,
        })
    }

    /// Returns the protected header map.
    #[inline]
    pub fn protected(&self) -> &Header<'a> {
        &self.protected
    }

    /// Returns the unprotected header map.
    #[inline]
    pub fn unprotected(&self) -> &Header<'a> {
        &self.unprotected
    }

    /// Returns `alg`, preferring the protected bucket.
    pub fn algorithm(&self) -> Option<Algorithm> {
        self.protected.algorithm().or_else(|| self.unprotected.algorithm())
    }

    /// Returns `kid`, preferring the protected bucket.
    pub fn kid(&self) -> Option<&[u8]> {
        self.protected.kid().or_else(|| self.unprotected.kid())
    }

    /// Returns `content type`, preferring the protected bucket.
    pub fn content_type(&self) -> Result<Option<ContentType<'_>>> {
        match self.protected.content_type()? {
            Some(value) => Ok(Some(value)),
            None => self.unprotected.content_type(),
        }
    }

    /// Returns the protected `crit` labels.
    pub fn crit(&self) -> Result<Crit<'_>> {
        self.protected.crit()
    }

    /// Validates critical-header understanding.
    ///
    /// Labels 1 through 6 (`alg`, `crit`, `content type`, `kid`, `IV`,
    /// `Partial IV`) are understood by this crate. Any other protected
    /// critical label must be included in `understood`.
    ///
    /// `Headers` can only be constructed by [`Headers::new`], which already
    /// performed structural validation, so this method only checks the
    /// application-level critical-label policy.
    pub fn validate_with(&self, understood: &[Label<'_>]) -> Result<()> {
        for label in self.protected.crit()? {
            let label = label?;
            if !is_known_header_label(label) && !understood.iter().any(|candidate| labels_equal(*candidate, label)) {
                return Err(Error::UnknownCriticalHeader);
            }
        }

        Ok(())
    }
}

fn is_known_header_label(label: Label<'_>) -> bool {
    matches!(label, Label::Int(1..=6))
}

/// Builder for one header map.
///
/// The builder stores borrowed header values and never allocates. It supports
/// up to eight `crit` labels and eight additional parameters; exceeding those
/// limits makes `build_*` return [`Error::TooManyEntries`].
pub struct HeaderBuilder<'a> {
    alg: Option<Algorithm>,
    crit: HeaplessVec<Label<'a>, 8>,
    content_type: Option<ContentType<'a>>,
    kid: Option<&'a [u8]>,
    rest: HeaplessVec<(Label<'a>, HeaderValue<'a>), 8>,
    overflow: bool,
}

impl<'a> HeaderBuilder<'a> {
    /// Creates an empty builder.
    #[inline]
    pub const fn new() -> Self {
        Self {
            alg: None,
            crit: HeaplessVec::new(),
            content_type: None,
            kid: None,
            rest: HeaplessVec::new(),
            overflow: false,
        }
    }

    /// Sets the `alg` header parameter.
    #[inline]
    pub fn algorithm(mut self, alg: Algorithm) -> Self {
        self.alg = Some(alg);
        self
    }

    /// Adds a `crit` label.
    pub fn critical(mut self, label: Label<'a>) -> Self {
        if self.crit.push(label).is_err() {
            self.overflow = true;
        }
        self
    }

    /// Sets the `content type` header parameter.
    #[inline]
    pub fn content_type(mut self, content_type: impl Into<ContentType<'a>>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Sets the `kid` header parameter.
    #[inline]
    pub fn key_id(mut self, kid: &'a [u8]) -> Self {
        self.kid = Some(kid);
        self
    }

    /// Adds an arbitrary header parameter.
    pub fn value(mut self, label: Label<'a>, value: HeaderValue<'a>) -> Self {
        if self.rest.push((label, value)).is_err() {
            self.overflow = true;
        }
        self
    }

    /// Adds an arbitrary header parameter whose value is already CBOR-encoded.
    #[inline]
    pub fn raw_value(self, label: Label<'a>, encoded: &'a [u8]) -> Self {
        self.value(label, HeaderValue::Raw(encoded))
    }

    /// Encodes a protected header into `out`.
    ///
    /// If the header has no fields, the result is the empty protected header
    /// (`h''`) and nothing is written.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooManyEntries`] if the builder overflowed, and
    /// [`Error::BufferTooSmall`] if `out` is too small. The encoded map is
    /// validated before it is returned.
    pub fn build_protected_into<'b>(&self, out: &'b mut [u8]) -> Result<Header<'b>> {
        self.build_into(out, HeaderRole::Protected)
    }

    /// Encodes an unprotected header into `out`.
    ///
    /// If the header has no fields, the result is `h'a0'` and nothing is
    /// written.
    pub fn build_unprotected_into<'b>(&self, out: &'b mut [u8]) -> Result<Header<'b>> {
        self.build_into(out, HeaderRole::Unprotected)
    }

    /// Encodes a protected header into a new owned value.
    #[cfg(feature = "alloc")]
    pub fn build_protected(&self) -> Result<Header<'static>> {
        self.build_owned(HeaderRole::Protected)
    }

    /// Encodes an unprotected header into a new owned value.
    #[cfg(feature = "alloc")]
    pub fn build_unprotected(&self) -> Result<Header<'static>> {
        self.build_owned(HeaderRole::Unprotected)
    }

    fn entry_count(&self) -> u64 {
        self.alg.is_some() as u64
            + (!self.crit.is_empty()) as u64
            + self.content_type.is_some() as u64
            + self.kid.is_some() as u64
            + self.rest.len() as u64
    }

    fn encoded_len(&self) -> usize {
        let count = self.entry_count();
        if count == 0 {
            return 0;
        }

        let mut len = cbor_len_uint(count);
        if let Some(alg) = self.alg {
            len += 1 + cbor_len_int(alg.as_i64());
        }
        if !self.crit.is_empty() {
            len += 1 + cbor_len_array(self.crit.len() as u64);
            for label in &self.crit {
                len += label_len(*label);
            }
        }
        if let Some(content_type) = self.content_type {
            len += 1;
            len += match content_type {
                ContentType::Text(value) => cbor_len_str(value.len()),
                ContentType::Uint(value) => cbor_len_uint(value),
            };
        }
        if let Some(kid) = self.kid {
            len += 1 + cbor_len_bytes(kid.len());
        }
        for (label, value) in &self.rest {
            len += label_len(*label) + value_len(*value);
        }
        len
    }

    fn build_into<'b>(&self, out: &'b mut [u8], role: HeaderRole) -> Result<Header<'b>> {
        if self.overflow {
            return Err(Error::TooManyEntries);
        }
        if role == HeaderRole::Unprotected && !self.crit.is_empty() {
            return Err(Error::CriticalHeaderNotProtected);
        }

        let count = self.entry_count();
        if count == 0 {
            return Ok(match role {
                HeaderRole::Protected => Header::empty_protected(),
                HeaderRole::Unprotected => Header::empty_unprotected(),
            });
        }

        let required = self.encoded_len();
        if out.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: out.len(),
            });
        }

        let mut cursor = Cursor::new(&mut out[..required]);
        {
            let mut e = Encoder::new(&mut cursor);
            e.map(count as usize)?;

            if let Some(alg) = self.alg {
                e.i64(1)?;
                e.i64(alg.as_i64())?;
            }
            if !self.crit.is_empty() {
                e.i64(2)?;
                e.array(self.crit.len())?;
                for label in &self.crit {
                    write_label(&mut e, *label)?;
                }
            }
            if let Some(content_type) = self.content_type {
                e.i64(3)?;
                match content_type {
                    ContentType::Text(value) => {
                        e.text(value)?;
                    }
                    ContentType::Uint(value) => {
                        e.u64(value)?;
                    }
                }
            }
            if let Some(kid) = self.kid {
                e.i64(4)?;
                e.bytes(kid)?;
            }
            for (label, value) in &self.rest {
                write_label(&mut e, *label)?;
                write_value(&mut e, *value)?;
            }
        }

        if cursor.position() != required {
            return Err(Error::Encode);
        }

        let header = Header::from_slice(&out[..required])?;
        header.validate(role)?;
        Ok(header)
    }

    #[cfg(feature = "alloc")]
    fn build_owned(&self, role: HeaderRole) -> Result<Header<'static>> {
        if self.overflow {
            return Err(Error::TooManyEntries);
        }
        if role == HeaderRole::Unprotected && !self.crit.is_empty() {
            return Err(Error::CriticalHeaderNotProtected);
        }

        let count = self.entry_count();
        if count == 0 {
            return Ok(match role {
                HeaderRole::Protected => Header::empty_protected(),
                HeaderRole::Unprotected => Header::empty_unprotected(),
            });
        }

        let required = self.encoded_len();
        let mut bytes = alloc::vec![0u8; required];
        self.build_into(&mut bytes, role)?;
        Header::from_owned(bytes)
    }
}

impl Default for HeaderBuilder<'_> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
