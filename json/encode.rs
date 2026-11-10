//! JSON encoding.

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use core::fmt;
use core::marker::PhantomData;

use crate::{
    error::{Error, Result},
    number::Number,
};

const DIGIT_PAIRS: &[u8; 200] = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";

#[inline]
fn format_u64(mut value: u64, buffer: &mut [u8; 20]) -> &[u8] {
    let mut index = buffer.len();
    while value >= 100 {
        let pair = (value % 100) as usize * 2;
        value /= 100;
        index -= 2;
        buffer[index..index + 2].copy_from_slice(&DIGIT_PAIRS[pair..pair + 2]);
    }
    if value < 10 {
        index -= 1;
        buffer[index] = b'0' + value as u8;
    } else {
        let pair = value as usize * 2;
        index -= 2;
        buffer[index..index + 2].copy_from_slice(&DIGIT_PAIRS[pair..pair + 2]);
    }
    &buffer[index..]
}

#[inline]
fn format_u128(mut value: u128, buffer: &mut [u8; 40]) -> &[u8] {
    let mut index = buffer.len();
    while value >= 100 {
        let pair = (value % 100) as usize * 2;
        value /= 100;
        index -= 2;
        buffer[index..index + 2].copy_from_slice(&DIGIT_PAIRS[pair..pair + 2]);
    }
    if value < 10 {
        index -= 1;
        buffer[index] = b'0' + value as u8;
    } else {
        let pair = value as usize * 2;
        index -= 2;
        buffer[index..index + 2].copy_from_slice(&DIGIT_PAIRS[pair..pair + 2]);
    }
    &buffer[index..]
}

/// A byte sink for JSON encoding.
pub trait Write {
    /// The error returned by this writer.
    type Error;

    /// Writes all of `bytes`.
    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error>;

    /// Writes a single byte.
    #[inline]
    fn write_byte(&mut self, byte: u8) -> core::result::Result<(), Self::Error> {
        self.write_all(&[byte])
    }

    /// Hints that at least `additional` more bytes will be written.
    ///
    /// The default implementation does nothing. Growable writers such as
    /// `Vec<u8>` override it to reduce reallocations.
    #[inline]
    fn reserve(&mut self, _additional: usize) {}
}

impl<W: Write + ?Sized> Write for &mut W {
    type Error = W::Error;

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error> {
        (**self).write_all(bytes)
    }

    #[inline]
    fn write_byte(&mut self, byte: u8) -> core::result::Result<(), Self::Error> {
        (**self).write_byte(byte)
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        (**self).reserve(additional);
    }
}

#[cfg(feature = "alloc")]
impl Write for Vec<u8> {
    type Error = core::convert::Infallible;

    #[inline(always)]
    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error> {
        self.extend_from_slice(bytes);
        Ok(())
    }

    #[inline(always)]
    fn write_byte(&mut self, byte: u8) -> core::result::Result<(), Self::Error> {
        self.push(byte);
        Ok(())
    }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional);
    }
}

impl Write for &mut [u8] {
    type Error = Error;

    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error> {
        if self.len() < bytes.len() {
            return Err(Error::buffer_too_small());
        }
        let remaining = core::mem::take(self);
        let (head, tail) = remaining.split_at_mut(bytes.len());
        head.copy_from_slice(bytes);
        *self = tail;
        Ok(())
    }
}

#[cfg(feature = "bytes")]
impl Write for bytes::BytesMut {
    type Error = core::convert::Infallible;

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error> {
        self.extend_from_slice(bytes);
        Ok(())
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        bytes::BytesMut::reserve(self, additional);
    }
}

/// Counts bytes written.
#[derive(Clone, Copy, Debug, Default)]
pub struct Count(pub usize);

impl Write for Count {
    type Error = core::convert::Infallible;

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error> {
        self.0 += bytes.len();
        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, _byte: u8) -> core::result::Result<(), Self::Error> {
        self.0 += 1;
        Ok(())
    }
}

/// Encoding options.
#[derive(Clone, Copy, Debug)]
pub struct EncodeOptions {
    /// Pretty-print with newlines and indentation. Requires `alloc`.
    pub pretty: bool,
    /// Number of spaces per indentation level in pretty mode.
    pub indent: u8,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self {
            pretty: false,
            indent: 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ContainerState {
    first: bool,
    nonempty: bool,
}

const MAX_CONTAINER_DEPTH: usize = 128;

/// A JSON encoder.
pub struct Encoder<W> {
    writer: W,
    options: EncodeOptions,
    stack: [ContainerState; MAX_CONTAINER_DEPTH],
    depth: usize,
    float_buffer: zmij::Buffer,
}

impl<W: Write> Encoder<W> {
    /// Creates a compact encoder.
    #[inline]
    pub fn new(writer: W) -> Self {
        Self::with_options(writer, EncodeOptions::default())
    }

    /// Creates an encoder with explicit options.
    #[inline]
    pub fn with_options(writer: W, options: EncodeOptions) -> Self {
        Self {
            writer,
            options,
            stack: [ContainerState::default(); MAX_CONTAINER_DEPTH],
            depth: 0,
            float_buffer: zmij::Buffer::new(),
        }
    }

    /// Returns the underlying writer.
    #[inline]
    pub fn writer(&self) -> &W {
        &self.writer
    }

    /// Returns the underlying writer mutably.
    #[inline]
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Consumes the encoder and returns the writer.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }

    /// Writes an element separator.
    ///
    /// `first` must be `true` for the first element of the current container.
    /// In compact mode it is the only separator state needed; pretty printing
    /// also tracks container state for indentation.
    #[inline]
    pub fn comma(&mut self, first: bool) -> core::result::Result<(), W::Error> {
        // Compact output is the hot path: the caller already knows whether
        // this is the first element, so no container-state lookup is needed.
        if !self.options.pretty {
            if !first {
                self.writer.write_byte(b',')?;
            }
            return Ok(());
        }
        if self.depth > MAX_CONTAINER_DEPTH {
            if !first {
                self.writer.write_byte(b',')?;
            }
            return Ok(());
        }
        if self.depth != 0 {
            let index = self.depth - 1;
            if index < MAX_CONTAINER_DEPTH {
                let was_first = self.stack[index].first;
                if was_first {
                    self.stack[index].first = false;
                } else {
                    self.writer.write_byte(b',')?;
                }
                if self.options.pretty {
                    self.newline_indent(self.depth)?;
                }
                self.stack[index].nonempty = true;
                return Ok(());
            }
        }
        let _ = first;
        #[cfg(not(feature = "alloc"))]
        {
            if !first {
                self.writer.write_byte(b',')?;
            }
        }
        Ok(())
    }

    #[inline]
    fn newline_indent(&mut self, level: usize) -> core::result::Result<(), W::Error> {
        if !self.options.pretty {
            return Ok(());
        }
        self.writer.write_byte(b'\n')?;
        let spaces = level.saturating_mul(self.options.indent as usize);
        const SPACES: &[u8] = b"                                ";
        let mut left = spaces;
        while left > 0 {
            let take = left.min(SPACES.len());
            self.writer.write_all(&SPACES[..take])?;
            left -= take;
        }
        Ok(())
    }

    /// Starts an array.
    #[inline]
    pub fn begin_array(&mut self) -> core::result::Result<(), W::Error> {
        self.writer.write_byte(b'[')?;
        if self.options.pretty {
            if self.depth < MAX_CONTAINER_DEPTH {
                self.stack[self.depth] = ContainerState {
                    first: true,
                    nonempty: false,
                };
            }
            self.depth += 1;
        }
        Ok(())
    }

    /// Ends an array.
    #[inline]
    pub fn end_array(&mut self) -> core::result::Result<(), W::Error> {
        if self.options.pretty && self.depth != 0 {
            self.depth -= 1;
            if self.depth < MAX_CONTAINER_DEPTH {
                let state = self.stack[self.depth];
                if state.nonempty {
                    self.newline_indent(self.depth)?;
                }
            }
        }
        self.writer.write_byte(b']')
    }

    /// Starts an object.
    #[inline]
    pub fn begin_object(&mut self) -> core::result::Result<(), W::Error> {
        self.writer.write_byte(b'{')?;
        if self.options.pretty {
            if self.depth < MAX_CONTAINER_DEPTH {
                self.stack[self.depth] = ContainerState {
                    first: true,
                    nonempty: false,
                };
            }
            self.depth += 1;
        }
        Ok(())
    }

    /// Ends an object.
    #[inline]
    pub fn end_object(&mut self) -> core::result::Result<(), W::Error> {
        if self.options.pretty && self.depth != 0 {
            self.depth -= 1;
            if self.depth < MAX_CONTAINER_DEPTH {
                let state = self.stack[self.depth];
                if state.nonempty {
                    self.newline_indent(self.depth)?;
                }
            }
        }
        self.writer.write_byte(b'}')
    }

    /// Writes an object key. The caller must call [`Encoder::comma`] first.
    #[inline]
    pub fn key(&mut self, key: &str) -> core::result::Result<(), W::Error> {
        crate::string::escape_into(key, &mut self.writer)?;
        self.writer.write_byte(b':')?;
        if self.options.pretty {
            self.writer.write_byte(b' ')?;
        }
        Ok(())
    }

    /// Writes `null`.
    #[inline]
    pub fn null(&mut self) -> core::result::Result<(), W::Error> {
        self.writer.write_all(b"null")
    }

    /// Writes a boolean.
    #[inline]
    pub fn boolean(&mut self, value: bool) -> core::result::Result<(), W::Error> {
        self.writer.write_all(if value { b"true" } else { b"false" })
    }

    /// Writes a string.
    #[inline]
    pub fn string(&mut self, value: &str) -> core::result::Result<(), W::Error> {
        crate::string::escape_into(value, &mut self.writer)
    }

    /// Writes raw JSON text as a value.
    ///
    /// The caller is responsible for the text being valid JSON. Because the
    /// argument is a `&str`, it is always valid UTF-8.
    #[inline]
    pub fn raw(&mut self, value: &str) -> core::result::Result<(), W::Error> {
        self.writer.write_all(value.as_bytes())
    }

    /// Writes raw bytes that were validated as JSON by this crate.
    #[inline]
    pub(crate) fn raw_validated(&mut self, value: &[u8]) -> core::result::Result<(), W::Error> {
        self.writer.write_all(value)
    }

    /// Writes a number.
    #[inline]
    pub fn number(&mut self, value: Number) -> core::result::Result<(), W::Error> {
        match value {
            Number::PosInt(value) => self.u64(value),
            Number::NegInt(value) => self.i64(value),
            Number::Float(value) => self.f64(value),
        }
    }

    /// Writes a `u64`.
    #[inline]
    pub fn u64(&mut self, value: u64) -> core::result::Result<(), W::Error> {
        let mut buffer = [0u8; 20];
        let digits = format_u64(value, &mut buffer);
        self.writer.write_all(digits)
    }

    /// Writes an `i64`.
    #[inline]
    pub fn i64(&mut self, value: i64) -> core::result::Result<(), W::Error> {
        if value < 0 {
            self.writer.write_all(b"-")?;
            self.u64(value.unsigned_abs())
        } else {
            self.u64(value as u64)
        }
    }

    /// Writes an `u128`.
    #[inline]
    pub fn u128(&mut self, value: u128) -> core::result::Result<(), W::Error> {
        let mut buffer = [0u8; 40];
        let digits = format_u128(value, &mut buffer);
        self.writer.write_all(digits)
    }

    /// Writes an `i128`.
    #[inline]
    pub fn i128(&mut self, value: i128) -> core::result::Result<(), W::Error> {
        if value < 0 {
            self.writer.write_all(b"-")?;
            self.u128(value.unsigned_abs())
        } else {
            self.u128(value as u128)
        }
    }

    /// Writes an `f64`. Non-finite values are written as `null`.
    #[inline(always)]
    pub fn f64(&mut self, value: f64) -> core::result::Result<(), W::Error> {
        if !value.is_finite() {
            return self.null();
        }
        let formatted = self.float_buffer.format_finite(value);
        self.writer.write_all(formatted.as_bytes())
    }

    /// Encodes a value.
    #[inline]
    pub fn encode<T: Encode + ?Sized>(&mut self, value: &T) -> core::result::Result<(), W::Error> {
        value.encode(self)
    }
}

/// A type that can be encoded as JSON.
pub trait Encode {
    /// Encodes this value into `encoder`.
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error>;
}

impl<T: Encode + ?Sized> Encode for &T {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

impl<T: Encode + ?Sized> Encode for &mut T {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode + ?Sized> Encode for Box<T> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode + ?Sized> Encode for alloc::rc::Rc<T> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode + ?Sized> Encode for alloc::sync::Arc<T> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

impl Encode for bool {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.boolean(*self)
    }
}

macro_rules! impl_encode_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Encode for $ty {
                fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
                    encoder.u64(*self as u64)
                }
            }
        )*
    };
}

impl_encode_unsigned!(u8, u16, u32, u64, usize);

macro_rules! impl_encode_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Encode for $ty {
                fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
                    encoder.i64(*self as i64)
                }
            }
        )*
    };
}

impl_encode_signed!(i8, i16, i32, i64, isize);

impl Encode for u128 {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.u128(*self)
    }
}

impl Encode for i128 {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.i128(*self)
    }
}

impl Encode for f32 {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.f64(*self as f64)
    }
}

impl Encode for f64 {
    #[inline(always)]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.f64(*self)
    }
}

impl Encode for Number {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.number(*self)
    }
}

impl Encode for str {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.string(self)
    }
}

#[cfg(feature = "alloc")]
impl Encode for String {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.string(self)
    }
}

#[cfg(feature = "alloc")]
impl Encode for Cow<'_, str> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.string(self)
    }
}

impl Encode for char {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        let mut buffer = [0u8; 4];
        encoder.string(self.encode_utf8(&mut buffer))
    }
}

impl Encode for () {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.null()
    }
}

impl<T> Encode for PhantomData<T> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.null()
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        match self {
            Some(value) => value.encode(encoder),
            None => encoder.null(),
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode> Encode for Vec<T> {
    #[inline(always)]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encode_slice(self, encoder)
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode> Encode for VecDeque<T> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_array()?;
        for (index, value) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            value.encode(encoder)?;
        }
        encoder.end_array()
    }
}

impl<T: Encode> Encode for [T] {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encode_slice(self, encoder)
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    #[inline(always)]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encode_slice(self, encoder)
    }
}

#[inline(always)]
fn encode_slice<T: Encode, W: Write>(values: &[T], encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
    encoder.begin_array()?;
    for (index, value) in values.iter().enumerate() {
        encoder.comma(index == 0)?;
        value.encode(encoder)?;
    }
    encoder.end_array()
}

#[cfg(feature = "alloc")]
impl<K: AsRef<str>, V: Encode> Encode for BTreeMap<K, V> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_object()?;
        for (index, (key, value)) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            encoder.key(key.as_ref())?;
            value.encode(encoder)?;
        }
        encoder.end_object()
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode> Encode for BTreeSet<T> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_array()?;
        for (index, value) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            value.encode(encoder)?;
        }
        encoder.end_array()
    }
}

#[cfg(feature = "std")]
impl<K, V, S> Encode for std::collections::HashMap<K, V, S>
where
    K: AsRef<str> + Eq + core::hash::Hash,
    V: Encode,
    S: core::hash::BuildHasher,
{
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_object()?;
        for (index, (key, value)) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            encoder.key(key.as_ref())?;
            value.encode(encoder)?;
        }
        encoder.end_object()
    }
}

#[cfg(feature = "std")]
impl<T, S> Encode for std::collections::HashSet<T, S>
where
    T: Encode + Eq + core::hash::Hash,
    S: core::hash::BuildHasher,
{
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_array()?;
        for (index, value) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            value.encode(encoder)?;
        }
        encoder.end_array()
    }
}

macro_rules! impl_tuple {
    ($($name:ident),+ $(,)?) => {
        impl<$($name: Encode),+> Encode for ($($name,)+) {
            #[allow(non_snake_case)]
            #[allow(unused_assignments)]
            fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                encoder.begin_array()?;
                let mut first = true;
                $(
                    encoder.comma(first)?;
                    first = false;
                    $name.encode(encoder)?;
                )+
                encoder.end_array()
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

/// Encodes `value` into a new `Vec<u8>`.
#[cfg(feature = "alloc")]
pub fn to_vec<T: Encode + ?Sized>(value: &T) -> Vec<u8> {
    to_vec_with_capacity(value, 256)
}

/// Encodes `value` into a new `Vec<u8>` with the given capacity.
#[cfg(feature = "alloc")]
pub fn to_vec_with_capacity<T: Encode + ?Sized>(value: &T, capacity: usize) -> Vec<u8> {
    let mut encoder = Encoder::new(Vec::with_capacity(capacity));
    match value.encode(&mut encoder) {
        Ok(()) => {}
        Err(never) => match never {},
    }
    encoder.into_inner()
}

/// Encodes `value` into an existing buffer.
#[cfg(feature = "alloc")]
pub fn to_vec_into<T: Encode + ?Sized>(value: &T, out: &mut Vec<u8>) {
    out.clear();
    let mut encoder = Encoder::new(&mut *out);
    match value.encode(&mut encoder) {
        Ok(()) => {}
        Err(never) => match never {},
    }
}

/// Encodes `value` into a new `String`.
///
/// If a custom [`Encode`] implementation writes invalid UTF-8 directly through
/// [`Encoder::writer_mut`], invalid bytes are replaced with the Unicode
/// replacement character instead of panicking. Use [`to_vec`] when byte-exact
/// output is required.
#[cfg(feature = "alloc")]
pub fn to_string<T: Encode + ?Sized>(value: &T) -> String {
    let mut encoder = Encoder::new(Vec::with_capacity(256));
    match value.encode(&mut encoder) {
        Ok(()) => {}
        Err(never) => match never {},
    }
    let out = encoder.into_inner();
    match String::from_utf8(out) {
        Ok(text) => text,
        // A custom `Encode` implementation can bypass `Encoder::raw` by
        // writing directly through `writer_mut`. Never panic in that case.
        Err(error) => String::from_utf8_lossy(error.as_bytes()).into_owned(),
    }
}

/// Encodes `value` into a new pretty-printed `String`.
///
/// Invalid UTF-8 written through [`Encoder::writer_mut`] is replaced lossily;
/// see [`to_string`].
#[cfg(feature = "alloc")]
pub fn to_string_pretty<T: Encode + ?Sized>(value: &T) -> String {
    let mut encoder = Encoder::with_options(
        Vec::with_capacity(256),
        EncodeOptions {
            pretty: true,
            indent: 2,
        },
    );
    match value.encode(&mut encoder) {
        Ok(()) => {}
        Err(never) => match never {},
    }
    let out = encoder.into_inner();
    match String::from_utf8(out) {
        Ok(text) => text,
        // A custom `Encode` implementation can bypass `Encoder::raw` by
        // writing directly through `writer_mut`. Never panic in that case.
        Err(error) => String::from_utf8_lossy(error.as_bytes()).into_owned(),
    }
}

/// Encodes `value` into a fixed-size buffer, returning the written slice.
pub fn to_slice<'a, T: Encode + ?Sized>(value: &T, buffer: &'a mut [u8]) -> Result<&'a mut [u8]> {
    let total = buffer.len();
    let mut writer: &mut [u8] = buffer;
    {
        let mut encoder = Encoder::new(&mut writer);
        value.encode(&mut encoder)?;
    }
    let used = total - writer.len();
    Ok(&mut buffer[..used])
}

/// Returns the number of bytes needed to encode `value`.
pub fn encoded_len<T: Encode + ?Sized>(value: &T) -> usize {
    let mut count = Count(0);
    let mut encoder = Encoder::new(&mut count);
    match value.encode(&mut encoder) {
        Ok(()) => {}
        Err(never) => match never {},
    }
    count.0
}

/// A `fmt::Write` adapter used by `Display`.
#[cfg(feature = "alloc")]
struct FmtWriter<'a, 'b>(&'a mut fmt::Formatter<'b>);

#[cfg(feature = "alloc")]
impl Write for FmtWriter<'_, '_> {
    type Error = fmt::Error;

    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error> {
        let text = core::str::from_utf8(bytes).map_err(|_| fmt::Error)?;
        self.0.write_str(text)
    }
}

/// Formats a value for `Display`.
#[cfg(feature = "alloc")]
pub(crate) fn display<T: Encode + ?Sized>(value: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut writer = FmtWriter(f);
    let mut encoder = Encoder::new(&mut writer);
    value.encode(&mut encoder)
}

/// A `std::io::Write` adapter.
#[cfg(feature = "std")]
pub struct IoWriter<W>(pub W);

#[cfg(feature = "std")]
impl<W: std::io::Write> Write for IoWriter<W> {
    type Error = std::io::Error;

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error> {
        self.0.write_all(bytes)
    }
}

/// Encodes `value` into a `std::io::Write`.
#[cfg(feature = "std")]
pub fn to_io_writer<T: Encode + ?Sized, W: std::io::Write>(value: &T, writer: W) -> std::io::Result<()> {
    let mut adapter = IoWriter(writer);
    let mut encoder = Encoder::new(&mut adapter);
    value.encode(&mut encoder)
}
