//! Encoding: the [`Write`] sink, [`Encoder`], the [`Encode`] trait and the
//! convenience constructors.

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::collections::{BTreeMap, BTreeSet};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::{
    data::{Integer, Simple, Tag},
    error::{Error, Result},
    float::{f16_to_f64, f64_to_f16, shortest_f64},
};

/// A sink that accepts encoded bytes.
///
/// Implementations are provided for `&mut [u8]`, `Vec<u8>` (with `alloc`),
/// `&mut W`, and `std::io::Write` (with `std`). The error type is the sink's
/// own error; encoding itself does not fail.
pub trait Write {
    /// The error returned by the sink.
    type Error;

    /// Write the whole slice.
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
}

impl<W: Write + ?Sized> Write for &mut W {
    type Error = W::Error;

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        (**self).write_all(bytes)
    }
}

impl Write for &mut [u8] {
    type Error = Error;

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        if bytes.len() > self.len() {
            return Err(Error::buffer_too_small());
        }
        let this = core::mem::take(self);
        let (head, tail) = this.split_at_mut(bytes.len());
        head.copy_from_slice(bytes);
        *self = tail;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl Write for Vec<u8> {
    type Error = core::convert::Infallible;

    #[inline(always)]
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}

/// A writer that only counts bytes. Used by [`encoded_len`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Count(pub usize);

impl Count {
    /// Create a new counter.
    #[inline]
    pub const fn new() -> Self {
        Count(0)
    }

    /// Return the number of bytes written so far.
    #[inline]
    pub const fn get(self) -> usize {
        self.0
    }
}

impl Write for Count {
    type Error = core::convert::Infallible;

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0 += bytes.len();
        Ok(())
    }
}

/// The CBOR encoder.
///
/// The encoder writes directly to the supplied [`Write`] sink. All integer
/// widths are encoded in their shortest form; floats use the shortest form
/// that preserves the value unless a fixed-width method is used.
#[derive(Debug)]
pub struct Encoder<W> {
    writer: W,
    canonical: bool,
}

impl<W: Write> Encoder<W> {
    /// Create an encoder writing to `writer`.
    #[inline]
    pub fn new(writer: W) -> Self {
        Encoder {
            writer,
            canonical: false,
        }
    }

    /// Create an encoder in deterministic mode.
    ///
    /// The mode affects `Value` map ordering, NaN canonicalization and
    /// dynamic map helpers. Derived types already emit map keys in canonical
    /// order at compile time.
    #[inline]
    pub fn canonical(writer: W) -> Self {
        Encoder {
            writer,
            canonical: true,
        }
    }

    /// Set deterministic mode.
    #[inline]
    pub fn set_canonical(&mut self, canonical: bool) {
        self.canonical = canonical;
    }

    /// Return whether deterministic mode is enabled.
    #[inline]
    pub const fn is_canonical(&self) -> bool {
        self.canonical
    }

    /// Access the underlying writer.
    #[inline]
    pub fn writer(&self) -> &W {
        &self.writer
    }

    /// Mutably access the underlying writer.
    #[inline]
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Return the underlying writer.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), W::Error> {
        self.writer.write_all(bytes)
    }

    #[inline(always)]
    fn head(&mut self, major: u8, n: u64) -> Result<(), W::Error> {
        if n < 24 {
            self.write_all(&[major | n as u8])
        } else if n <= u8::MAX as u64 {
            self.write_all(&[major | 24, n as u8])
        } else if n <= u16::MAX as u64 {
            let mut buf = [major | 25, 0, 0];
            buf[1..].copy_from_slice(&(n as u16).to_be_bytes());
            self.write_all(&buf)
        } else if n <= u32::MAX as u64 {
            let mut buf = [major | 26, 0, 0, 0, 0];
            buf[1..].copy_from_slice(&(n as u32).to_be_bytes());
            self.write_all(&buf)
        } else {
            let mut buf = [major | 27, 0, 0, 0, 0, 0, 0, 0, 0];
            buf[1..].copy_from_slice(&n.to_be_bytes());
            self.write_all(&buf)
        }
    }

    /// Encode an unsigned integer.
    #[inline(always)]
    pub fn u64(&mut self, value: u64) -> Result<(), W::Error> {
        self.head(0x00, value)
    }

    /// Encode a `u8`.
    #[inline]
    pub fn u8(&mut self, value: u8) -> Result<(), W::Error> {
        self.u64(u64::from(value))
    }

    /// Encode a `u16`.
    #[inline]
    pub fn u16(&mut self, value: u16) -> Result<(), W::Error> {
        self.u64(u64::from(value))
    }

    /// Encode a `u32`.
    #[inline]
    pub fn u32(&mut self, value: u32) -> Result<(), W::Error> {
        self.u64(u64::from(value))
    }

    /// Encode a `usize`.
    #[inline]
    pub fn usize(&mut self, value: usize) -> Result<(), W::Error> {
        self.u64(value as u64)
    }

    /// Encode a signed integer.
    #[inline(always)]
    pub fn i64(&mut self, value: i64) -> Result<(), W::Error> {
        if value >= 0 {
            self.head(0x00, value as u64)
        } else {
            self.head(0x20, (-1 - value) as u64)
        }
    }

    /// Encode an `i8`.
    #[inline]
    pub fn i8(&mut self, value: i8) -> Result<(), W::Error> {
        self.i64(i64::from(value))
    }

    /// Encode an `i16`.
    #[inline]
    pub fn i16(&mut self, value: i16) -> Result<(), W::Error> {
        self.i64(i64::from(value))
    }

    /// Encode an `i32`.
    #[inline]
    pub fn i32(&mut self, value: i32) -> Result<(), W::Error> {
        self.i64(i64::from(value))
    }

    /// Encode an `isize`.
    #[inline]
    pub fn isize(&mut self, value: isize) -> Result<(), W::Error> {
        self.i64(value as i64)
    }

    /// Encode a CBOR integer.
    #[inline]
    pub fn int(&mut self, value: Integer) -> Result<(), W::Error> {
        match value {
            Integer::Unsigned(v) => self.u64(v),
            Integer::Negative(n) => self.head(0x20, n),
        }
    }

    /// Encode a `u128`, using tag 2 when the value does not fit in 64 bits.
    pub fn u128(&mut self, value: u128) -> Result<(), W::Error> {
        if value <= u64::MAX as u128 {
            return self.u64(value as u64);
        }
        self.tag(crate::data::tags::BIGPOS)?;
        let bytes = value.to_be_bytes();
        self.bignum_bytes(&bytes)
    }

    /// Encode an `i128`, using tag 3 when the value does not fit in 64 bits.
    pub fn i128(&mut self, value: i128) -> Result<(), W::Error> {
        if value >= 0 {
            return self.u128(value as u128);
        }
        if value >= i64::MIN as i128 {
            return self.i64(value as i64);
        }
        let magnitude = (-(value + 1)) as u128;
        self.tag(crate::data::tags::BIGNEG)?;
        let bytes = magnitude.to_be_bytes();
        self.bignum_bytes(&bytes)
    }

    fn bignum_bytes(&mut self, bytes: &[u8]) -> Result<(), W::Error> {
        let first = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
        self.bytes(&bytes[first..])
    }

    /// Encode a boolean.
    #[inline(always)]
    pub fn bool(&mut self, value: bool) -> Result<(), W::Error> {
        self.write_all(&[0xe0 | if value { 21 } else { 20 }])
    }

    /// Encode `null`.
    #[inline(always)]
    pub fn null(&mut self) -> Result<(), W::Error> {
        self.write_all(&[0xf6])
    }

    /// Encode `undefined`.
    #[inline]
    pub fn undefined(&mut self) -> Result<(), W::Error> {
        self.write_all(&[0xf7])
    }

    /// Encode a simple value.
    #[inline]
    pub fn simple(&mut self, value: Simple) -> Result<(), W::Error> {
        let v = value.get();
        if v < 24 {
            self.write_all(&[0xe0 | v])
        } else {
            self.write_all(&[0xf8, v])
        }
    }

    /// Encode a `char` as a one-character text string.
    #[inline]
    pub fn char(&mut self, value: char) -> Result<(), W::Error> {
        let mut buf = [0u8; 4];
        self.text(value.encode_utf8(&mut buf))
    }

    /// Encode an `f64` using the shortest lossless representation.
    #[inline(always)]
    pub fn f64(&mut self, value: f64) -> Result<(), W::Error> {
        let (tag, bits) = shortest_f64(value);
        match tag {
            0xf9 => {
                let b = (bits as u16).to_be_bytes();
                self.write_all(&[0xf9, b[0], b[1]])
            }
            0xfa => {
                let b = (bits as u32).to_be_bytes();
                self.write_all(&[0xfa, b[0], b[1], b[2], b[3]])
            }
            _ => {
                let b = bits.to_be_bytes();
                self.write_all(&[0xfb, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
            }
        }
    }

    /// Encode an `f64` as a 64-bit float, regardless of value.
    #[inline(always)]
    pub fn f64_always(&mut self, value: f64) -> Result<(), W::Error> {
        let b = value.to_bits().to_be_bytes();
        self.write_all(&[0xfb, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }

    /// Encode an `f32` using the shortest lossless representation.
    #[inline]
    pub fn f32(&mut self, value: f32) -> Result<(), W::Error> {
        self.f64(f64::from(value))
    }

    /// Encode an `f32` as a 32-bit float.
    #[inline]
    pub fn f32_always(&mut self, value: f32) -> Result<(), W::Error> {
        let b = value.to_bits().to_be_bytes();
        self.write_all(&[0xfa, b[0], b[1], b[2], b[3]])
    }

    /// Encode a precomputed binary16 value.
    #[inline]
    pub fn f16_bits(&mut self, bits: u16) -> Result<(), W::Error> {
        let b = bits.to_be_bytes();
        self.write_all(&[0xf9, b[0], b[1]])
    }

    /// Encode a byte string.
    #[inline(always)]
    pub fn bytes(&mut self, bytes: &[u8]) -> Result<(), W::Error> {
        self.head(0x40, bytes.len() as u64)?;
        self.write_all(bytes)
    }

    /// Encode only the head of a definite-length byte string.
    #[inline(always)]
    pub fn bytes_len(&mut self, len: u64) -> Result<(), W::Error> {
        self.head(0x40, len)
    }

    /// Encode a text string.
    #[inline(always)]
    pub fn text(&mut self, text: &str) -> Result<(), W::Error> {
        self.head(0x60, text.len() as u64)?;
        self.write_all(text.as_bytes())
    }

    /// Encode only the head of a definite-length text string.
    #[inline(always)]
    pub fn text_len(&mut self, len: u64) -> Result<(), W::Error> {
        self.head(0x60, len)
    }

    /// Encode an array header.
    #[inline(always)]
    pub fn array(&mut self, len: usize) -> Result<(), W::Error> {
        self.head(0x80, len as u64)
    }

    /// Encode a map header.
    #[inline(always)]
    pub fn map(&mut self, len: usize) -> Result<(), W::Error> {
        self.head(0xa0, len as u64)
    }

    /// Start an indefinite-length array.
    #[inline]
    pub fn begin_array(&mut self) -> Result<(), W::Error> {
        self.write_all(&[0x9f])
    }

    /// Start an indefinite-length map.
    #[inline]
    pub fn begin_map(&mut self) -> Result<(), W::Error> {
        self.write_all(&[0xbf])
    }

    /// Start an indefinite-length byte string.
    #[inline]
    pub fn begin_bytes(&mut self) -> Result<(), W::Error> {
        self.write_all(&[0x5f])
    }

    /// Start an indefinite-length text string.
    #[inline]
    pub fn begin_text(&mut self) -> Result<(), W::Error> {
        self.write_all(&[0x7f])
    }

    /// Write the `break` stop code.
    #[inline]
    pub fn end(&mut self) -> Result<(), W::Error> {
        self.write_all(&[0xff])
    }

    /// Encode a tag number.
    #[inline(always)]
    pub fn tag<T: Into<Tag>>(&mut self, tag: T) -> Result<(), W::Error> {
        self.head(0xc0, tag.into().get())
    }

    /// Write already-validated CBOR bytes unchanged.
    #[inline]
    pub fn raw(&mut self, bytes: &[u8]) -> Result<(), W::Error> {
        self.write_all(bytes)
    }

    /// Encode any value implementing [`Encode`].
    #[inline]
    pub fn encode<T: Encode + ?Sized>(&mut self, value: &T) -> Result<(), W::Error> {
        value.encode(self)
    }

    /// Encode an iterator as a definite-length array.
    ///
    /// The iterator is consumed, so the length must be known up front.
    #[inline]
    pub fn encode_iter<I, T>(&mut self, len: usize, iter: I) -> Result<(), W::Error>
    where
        I: IntoIterator<Item = T>,
        T: Encode,
    {
        self.array(len)?;
        for item in iter {
            item.encode(self)?;
        }
        Ok(())
    }
}

/// Encode a value into a CBOR item.
///
/// Implementations should write exactly one item. Encoding never fails except
/// for the sink's error, which is propagated as [`Write::Error`].
pub trait Encode {
    /// Encode this value.
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error>;
}

impl<T: Encode + ?Sized> Encode for &T {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

impl<T: Encode + ?Sized> Encode for &mut T {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode + ?Sized> Encode for Box<T> {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode + ?Sized> Encode for alloc::rc::Rc<T> {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode + ?Sized> Encode for alloc::sync::Arc<T> {
    #[inline]
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        (**self).encode(encoder)
    }
}

impl Encode for u8 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.u8(*self)
    }
}
impl Encode for u16 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.u16(*self)
    }
}
impl Encode for u32 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.u32(*self)
    }
}
impl Encode for u64 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.u64(*self)
    }
}
impl Encode for usize {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.usize(*self)
    }
}
impl Encode for u128 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.u128(*self)
    }
}
impl Encode for i8 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.i8(*self)
    }
}
impl Encode for i16 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.i16(*self)
    }
}
impl Encode for i32 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.i32(*self)
    }
}
impl Encode for i64 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.i64(*self)
    }
}
impl Encode for isize {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.isize(*self)
    }
}
impl Encode for i128 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.i128(*self)
    }
}
impl Encode for f32 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.f32(*self)
    }
}
impl Encode for f64 {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.f64(*self)
    }
}
impl Encode for bool {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.bool(*self)
    }
}
impl Encode for char {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.char(*self)
    }
}
impl Encode for str {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.text(self)
    }
}
#[cfg(feature = "alloc")]
impl Encode for String {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.text(self)
    }
}
impl Encode for Integer {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.int(*self)
    }
}
impl Encode for Simple {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.simple(*self)
    }
}
impl Encode for () {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.null()
    }
}
impl<T: Encode> Encode for Option<T> {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        match self {
            Some(value) => value.encode(e),
            None => e.null(),
        }
    }
}
impl<T: Encode> Encode for PhantomData<T> {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.null()
    }
}
impl<T: Encode> Encode for [T] {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.array(self.len())?;
        for item in self {
            item.encode(e)?;
        }
        Ok(())
    }
}
#[cfg(feature = "alloc")]
impl<T: Encode> Encode for Vec<T> {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        self.as_slice().encode(e)
    }
}
#[cfg(feature = "alloc")]
impl<T: Encode> Encode for alloc::collections::VecDeque<T> {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.array(self.len())?;
        for item in self {
            item.encode(e)?;
        }
        Ok(())
    }
}
impl<T: Encode, const N: usize> Encode for [T; N] {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.array(N)?;
        for item in self {
            item.encode(e)?;
        }
        Ok(())
    }
}
#[cfg(feature = "alloc")]
impl<K: Encode, V: Encode> Encode for BTreeMap<K, V> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        if e.is_canonical() {
            let mut entries: Vec<(Vec<u8>, &K, &V)> = self
                .iter()
                .map(|(key, value)| (to_canonical_vec(key), key, value))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            e.map(entries.len())?;
            for (key_bytes, _, value) in entries {
                e.raw(&key_bytes)?;
                value.encode(e)?;
            }
            return Ok(());
        }
        e.map(self.len())?;
        for (k, v) in self {
            k.encode(e)?;
            v.encode(e)?;
        }
        Ok(())
    }
}
#[cfg(feature = "alloc")]
impl<T: Encode> Encode for BTreeSet<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.array(self.len())?;
        for item in self {
            item.encode(e)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<K: Encode, V: Encode, S> Encode for std::collections::HashMap<K, V, S> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        if e.is_canonical() {
            let mut entries: Vec<(Vec<u8>, &K, &V)> = self
                .iter()
                .map(|(key, value)| (to_canonical_vec(key), key, value))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            e.map(entries.len())?;
            for (key_bytes, _, value) in entries {
                e.raw(&key_bytes)?;
                value.encode(e)?;
            }
            return Ok(());
        }
        e.map(self.len())?;
        for (k, v) in self {
            k.encode(e)?;
            v.encode(e)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Encode, S> Encode for std::collections::HashSet<T, S> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
        e.array(self.len())?;
        for item in self {
            item.encode(e)?;
        }
        Ok(())
    }
}

macro_rules! impl_tuple {
    ($($name:ident),+) => {
        impl<$($name: Encode),+> Encode for ($($name,)+) {
            #[allow(non_snake_case)]
            #[inline]
            fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), W::Error> {
                let ($($name,)+) = self;
                e.array(0 $(+ { let _ = stringify!($name); 1 })+)?;
                $($name.encode(e)?;)+
                Ok(())
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, F);
impl_tuple!(A, B, C, D, F, G);
impl_tuple!(A, B, C, D, F, G, H);
impl_tuple!(A, B, C, D, F, G, H, I);
impl_tuple!(A, B, C, D, F, G, H, I, J);
impl_tuple!(A, B, C, D, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, F, G, H, I, J, K, L);
impl_tuple!(A, B, C, D, F, G, H, I, J, K, L, M);
impl_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N);
impl_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O);
impl_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P);

/// Compute the exact encoded length of a value by encoding it to a counter.
///
/// This is useful for sizing fixed buffers for [`to_slice`]. It performs a
/// full encoding pass and does not allocate.
#[inline]
pub fn encoded_len<T: Encode + ?Sized>(value: &T) -> usize {
    let mut count = Count::new();
    {
        let mut encoder = Encoder::new(&mut count);
        match value.encode(&mut encoder) {
            Ok(()) => {}
            Err(never) => match never {},
        }
    }
    count.get()
}

#[cfg(feature = "alloc")]
fn encode_with_capacity<T: Encode + ?Sized>(value: &T, capacity: usize, canonical: bool) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(capacity);
    {
        let mut encoder = if canonical {
            Encoder::canonical(&mut buffer)
        } else {
            Encoder::new(&mut buffer)
        };
        match value.encode(&mut encoder) {
            Ok(()) => {}
            Err(never) => match never {},
        }
    }
    buffer
}

/// Encode a value into a new `Vec<u8>`.
///
/// The vector starts with a modest capacity so small values need only one
/// allocation. Use [`to_vec_with_capacity`] when the size is known.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_vec<T: Encode + ?Sized>(value: &T) -> Vec<u8> {
    encode_with_capacity(value, 128, false)
}

/// Encode a value into a new `Vec<u8>` with the given initial capacity.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_vec_with_capacity<T: Encode + ?Sized>(value: &T, capacity: usize) -> Vec<u8> {
    encode_with_capacity(value, capacity, false)
}

/// Encode a value into a new `Vec<u8>` in deterministic mode.
///
/// Derived maps are emitted in canonical key order by default. The built-in
/// `BTreeMap`, `HashMap`, ordered [`Map`](crate::Map), and [`Value`](crate::Value)
/// maps are sorted in canonical mode, and small [`Bignum`](crate::Bignum)
/// values are emitted as basic integers. Types that use
/// `#[cbor(declaration_order)]` and manual [`Encode`] implementations are
/// responsible for writing their own map keys in canonical order; use
/// [`validate_canonical`](crate::validate_canonical) to check the result.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_canonical_vec<T: Encode + ?Sized>(value: &T) -> Vec<u8> {
    encode_with_capacity(value, 128, true)
}

/// Encode a value into the front of a caller-provided buffer.
///
/// Returns the prefix that was written. If the buffer is too small, an error
/// is returned; the contents of the buffer are unspecified in that case.
#[inline]
pub fn to_slice<'a, T: Encode + ?Sized>(value: &T, buffer: &'a mut [u8]) -> Result<&'a mut [u8]> {
    let total = buffer.len();
    let used = {
        let mut writer = &mut *buffer;
        let mut encoder = Encoder::new(&mut writer);
        value.encode(&mut encoder)?;
        total - writer.len()
    };
    Ok(&mut buffer[..used])
}

/// Encode a value into the front of a caller-provided buffer in deterministic mode.
#[inline]
pub fn to_canonical_slice<'a, T: Encode + ?Sized>(value: &T, buffer: &'a mut [u8]) -> Result<&'a mut [u8]> {
    let total = buffer.len();
    let used = {
        let mut writer = &mut *buffer;
        let mut encoder = Encoder::canonical(&mut writer);
        value.encode(&mut encoder)?;
        total - writer.len()
    };
    Ok(&mut buffer[..used])
}

#[cfg(feature = "std")]
struct IoWriter<W>(W);

#[cfg(feature = "std")]
impl<W: std::io::Write> Write for IoWriter<W> {
    type Error = std::io::Error;

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all(bytes)
    }
}

/// Encode a value into any `std::io::Write`.
#[cfg(feature = "std")]
#[inline]
pub fn to_writer<T: Encode + ?Sized, W: std::io::Write>(value: &T, writer: W) -> std::io::Result<()> {
    let mut writer = IoWriter(writer);
    let mut encoder = Encoder::new(&mut writer);
    value.encode(&mut encoder)
}

/// Encode a value into any `std::io::Write` in deterministic mode.
#[cfg(feature = "std")]
#[inline]
pub fn to_canonical_writer<T: Encode + ?Sized, W: std::io::Write>(value: &T, writer: W) -> std::io::Result<()> {
    let mut writer = IoWriter(writer);
    let mut encoder = Encoder::canonical(&mut writer);
    value.encode(&mut encoder)
}

/// Return the canonical shortest float encoding for a value.
#[inline]
pub fn shortest_float(value: f64) -> (u8, u64) {
    shortest_f64(value)
}

/// Convert binary16 bits to `f64`.
#[inline]
pub fn half_to_f64(bits: u16) -> f64 {
    f16_to_f64(bits)
}

/// Convert an `f64` to binary16 bits (round to nearest even).
#[inline]
pub fn f64_to_half(value: f64) -> u16 {
    f64_to_f16(value)
}
