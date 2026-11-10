//! Lazy dynamic access to a validated JSON document.
//!
//! Ordinary borrowed access does not allocate. Object keys that contain
//! escapes require an owned `String` when the `alloc` feature is enabled; use
//! [`Element::get_pointer`] or compare through [`crate::Value`] if that
//! allocation is a problem.

use crate::{
    decode::{Decoder, Key},
    error::{Error, ErrorKind, Result},
};

/// A validated JSON document.
///
/// Access is lazy: elements are parsed on demand and strings borrow from the
/// original input whenever possible.
#[derive(Clone, Copy, Debug)]
pub struct Document<'a> {
    bytes: &'a [u8],
}

impl<'a> Document<'a> {
    /// Validates `input` and creates a document with default options.
    pub fn parse(input: &'a [u8]) -> Result<Self> {
        Self::parse_with_options(input, crate::decode::Options::default())
    }

    /// Validates `input` with explicit options and creates a document.
    pub fn parse_with_options(input: &'a [u8], options: crate::decode::Options) -> Result<Self> {
        crate::scan::validate_with_options(input, options)?;
        let bytes = if options.allow_bom && input.starts_with(&[0xEF, 0xBB, 0xBF]) {
            &input[3..]
        } else {
            input
        };
        Ok(Self {
            bytes,
        })
    }

    /// Returns the root element.
    #[inline]
    pub fn root(&self) -> Element<'a> {
        Element {
            bytes: self.bytes,
        }
    }

    /// Returns the raw document bytes.
    #[inline]
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Looks up an RFC 6901 JSON Pointer.
    pub fn get(&self, pointer: &str) -> Result<Option<Element<'a>>> {
        self.root().get_pointer(pointer)
    }
}

/// A lazily parsed JSON value.
#[derive(Clone, Copy, Debug)]
pub struct Element<'a> {
    bytes: &'a [u8],
}

impl<'a> Element<'a> {
    /// Creates an element from bytes that have already been validated.
    #[inline]
    pub(crate) fn from_validated(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
        }
    }

    /// Returns the raw JSON bytes.
    #[inline]
    pub const fn raw(&self) -> &'a [u8] {
        self.bytes
    }

    fn decoder(&self) -> Decoder<'a> {
        Decoder::from_validated_utf8(self.bytes, crate::decode::Options::default())
    }

    /// Returns `true` if this is `null`.
    pub fn is_null(&self) -> Result<bool> {
        let mut decoder = self.decoder();
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'n') => {
                decoder.decode_null()?;
                Ok(true)
            }
            None => Err(Error::new(ErrorKind::UnexpectedEof, 0)),
            Some(_) => Ok(false),
        }
    }

    /// Returns the boolean value, if any.
    pub fn as_bool(&self) -> Result<Option<bool>> {
        let mut decoder = self.decoder();
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b't') | Some(b'f') => Ok(Some(decoder.decode_bool()?)),
            None => Err(Error::new(ErrorKind::UnexpectedEof, 0)),
            Some(_) => Ok(None),
        }
    }

    /// Returns a borrowed string. Escaped strings require
    /// [`Element::string_into`].
    pub fn as_str(&self) -> Result<Option<&'a str>> {
        let mut decoder = self.decoder();
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'"') => decoder.decode_str().map(Some),
            None => Err(Error::new(ErrorKind::UnexpectedEof, 0)),
            Some(_) => Ok(None),
        }
    }

    /// Decodes a string into a caller-provided buffer.
    pub fn string_into<'o>(&self, out: &'o mut [u8]) -> Result<&'o str> {
        let mut decoder = self.decoder();
        decoder.decode_str_into(out)
    }

    /// Returns the value as an `f64`.
    pub fn as_f64(&self) -> Result<Option<f64>> {
        let mut decoder = self.decoder();
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'-' | b'0'..=b'9') => decoder.decode_f64().map(Some),
            None => Err(Error::new(ErrorKind::UnexpectedEof, 0)),
            Some(_) => Ok(None),
        }
    }

    /// Returns the value as an `i64`.
    pub fn as_i64(&self) -> Result<Option<i64>> {
        let mut decoder = self.decoder();
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'-' | b'0'..=b'9') => decoder.decode_i64().map(Some),
            None => Err(Error::new(ErrorKind::UnexpectedEof, 0)),
            Some(_) => Ok(None),
        }
    }

    /// Returns the value as a `u64`.
    pub fn as_u64(&self) -> Result<Option<u64>> {
        let mut decoder = self.decoder();
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'0'..=b'9') => decoder.decode_u64().map(Some),
            None => Err(Error::new(ErrorKind::UnexpectedEof, 0)),
            Some(_) => Ok(None),
        }
    }

    /// Returns an iterator over the array elements.
    pub fn array(&self) -> Result<ArrayIter<'a>> {
        let mut decoder = self.decoder();
        decoder.begin_array()?;
        Ok(ArrayIter {
            decoder,
            first: true,
            done: false,
        })
    }

    /// Returns an iterator over the object entries.
    pub fn object(&self) -> Result<ObjectIter<'a>> {
        let mut decoder = self.decoder();
        decoder.begin_object()?;
        Ok(ObjectIter {
            decoder,
            first: true,
            done: false,
        })
    }

    /// Looks up an object key.
    ///
    /// When an object contains duplicate keys, the last occurrence wins, which
    /// matches the default [`crate::decode::DuplicateKeys`] policy and the
    /// behaviour of [`crate::Value`].
    pub fn get(&self, key: &str) -> Result<Option<Element<'a>>> {
        let iter = self.object()?;
        let mut found = None;
        for entry in iter {
            let (candidate, value) = entry?;
            if candidate.as_str() == key {
                found = Some(value);
            }
        }
        Ok(found)
    }

    /// Looks up an array index.
    pub fn index(&self, index: usize) -> Result<Option<Element<'a>>> {
        let iter = self.array()?;
        for (i, value) in iter.enumerate() {
            if i == index {
                return value.map(Some);
            }
        }
        Ok(None)
    }

    /// Resolves an RFC 6901 JSON Pointer.
    pub fn get_pointer(&self, pointer: &str) -> Result<Option<Element<'a>>> {
        if pointer.is_empty() {
            return Ok(Some(*self));
        }
        let Some(rest) = pointer.strip_prefix('/') else {
            return Ok(None);
        };
        let mut current = *self;
        for token in rest.split('/') {
            if current.as_array().is_some() {
                let Some(index) = parse_index(token) else {
                    return Ok(None);
                };
                match current.index(index)? {
                    Some(value) => current = value,
                    None => return Ok(None),
                }
            } else if current.as_object().is_some() {
                let mut found = None;
                for entry in current.object()? {
                    let (key, value) = entry?;
                    if pointer_token_eq(key.as_str(), token) {
                        found = Some(value);
                    }
                }
                match found {
                    Some(value) => current = value,
                    None => return Ok(None),
                }
            } else {
                return Ok(None);
            }
        }
        Ok(Some(current))
    }

    /// Converts the element into an owned [`crate::Value`].
    #[cfg(feature = "alloc")]
    pub fn to_value(&self) -> Result<crate::Value<'static>> {
        let value: crate::Value<'a> = crate::from_slice(self.bytes)?;
        Ok(value.into_owned())
    }

    fn as_array(&self) -> Option<()> {
        let mut i = 0;
        while i < self.bytes.len() && matches!(self.bytes[i], b' ' | b'\t' | b'\n' | b'\r') {
            i += 1;
        }
        self.bytes.get(i).filter(|&&byte| byte == b'[').map(|_| ())
    }

    fn as_object(&self) -> Option<()> {
        let mut i = 0;
        while i < self.bytes.len() && matches!(self.bytes[i], b' ' | b'\t' | b'\n' | b'\r') {
            i += 1;
        }
        self.bytes.get(i).filter(|&&byte| byte == b'{').map(|_| ())
    }
}

/// An iterator over a JSON array.
pub struct ArrayIter<'a> {
    decoder: Decoder<'a>,
    first: bool,
    done: bool,
}

impl<'a> Iterator for ArrayIter<'a> {
    type Item = Result<Element<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.decoder.array_next(self.first) {
            Ok(true) => {}
            Ok(false) => {
                self.done = true;
                return None;
            }
            Err(error) => {
                self.done = true;
                return Some(Err(error));
            }
        }
        self.first = false;
        match self.decoder.raw_value() {
            Ok(raw) => Some(Ok(Element::from_validated(raw.get()))),
            Err(error) => {
                self.done = true;
                Some(Err(error))
            }
        }
    }
}

/// An iterator over a JSON object.
pub struct ObjectIter<'a> {
    decoder: Decoder<'a>,
    first: bool,
    done: bool,
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = Result<(Key<'a>, Element<'a>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.decoder.object_next(self.first) {
            Ok(true) => {}
            Ok(false) => {
                self.done = true;
                return None;
            }
            Err(error) => {
                self.done = true;
                return Some(Err(error));
            }
        }
        self.first = false;
        let key = match self.decoder.key() {
            Ok(key) => key,
            Err(error) => {
                self.done = true;
                return Some(Err(error));
            }
        };
        match self.decoder.raw_value() {
            Ok(raw) => Some(Ok((key, Element::from_validated(raw.get())))),
            Err(error) => {
                self.done = true;
                Some(Err(error))
            }
        }
    }
}

fn parse_index(token: &str) -> Option<usize> {
    if token.is_empty() || token.starts_with('+') || token.starts_with('-') {
        return None;
    }
    if token.len() > 1 && token.starts_with('0') {
        return None;
    }
    token.parse().ok()
}

fn pointer_token_eq(key: &str, token: &str) -> bool {
    let mut key_chars = key.chars();
    let mut token_chars = token.chars();
    loop {
        match (key_chars.next(), token_chars.next()) {
            (None, None) => return true,
            (Some(key), Some('~')) => match token_chars.next() {
                Some('0') if key == '~' => {}
                Some('1') if key == '/' => {}
                _ => return false,
            },
            (Some(key), Some(token)) if key == token => {}
            _ => return false,
        }
    }
}

impl core::fmt::Display for Element<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(self.bytes).map_err(|_| core::fmt::Error)?)
    }
}
