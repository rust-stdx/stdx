#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Columnar encoding for Rust structs, wrapped in a CBOR envelope.
//!
//! This crate turns a `&[T]` into a compact columnar byte string and back. It
//! does **not** use serde, and it does not require a schema description: the
//! Rust type is the schema.
//!
//! # Quick start
//!
//! ```
//! use columnar::Columnar;
//!
//! #[derive(Columnar)]
//! struct Sale {
//!     #[columnar(key = 1)]
//!     id: u64,
//!     #[columnar(key = 2)]
//!     amount: f64,
//!     #[columnar(key = 3)]
//!     region: String,
//! }
//!
//! let sales = vec![
//!     Sale { id: 1, amount: 19.99, region: "eu".into() },
//!     Sale { id: 2, amount: 5.00, region: "us".into() },
//! ];
//!
//! let bytes = columnar::to_vec(&sales).unwrap();
//! let decoded: Vec<Sale> = columnar::from_slice(&bytes).unwrap();
//! assert_eq!(decoded.len(), sales.len());
//! ```
//!
//! # Borrowed decoding
//!
//! `Cow<'a, [u8]>` and `Cow<'a, str>` fields decode as `Cow::Borrowed` views
//! of the input envelope, with no per-value byte copy:
//!
//! ```
//! use std::borrow::Cow;
//! use columnar::Columnar;
//!
//! #[derive(Columnar)]
//! struct Blob<'a> {
//!     #[columnar(key = 1)]
//!     id: u64,
//!     #[columnar(key = 2)]
//!     data: Cow<'a, [u8]>,
//! }
//!
//! let rows = vec![Blob { id: 1, data: Cow::Borrowed(b"hello") }];
//! let bytes = columnar::to_vec(&rows).unwrap();
//! let decoded: Vec<Blob<'_>> = columnar::from_slice(&bytes).unwrap();
//! assert!(matches!(decoded[0].data, Cow::Borrowed(_)));
//! ```
//!
//! Strings work the same way; `Cow<str>` validates UTF-8 and borrows the
//! text bytes instead of allocating a `String` per value.
//!
//! A type with more than one lifetime needs an explicit decode lifetime:
//!
//! ```rust
//! use std::borrow::Cow;
//! use std::marker::PhantomData;
//! use columnar::Columnar;
//!
//! #[derive(Columnar)]
//! #[columnar(borrow = "'a")]
//! struct Two<'a, 'b> {
//!     #[columnar(key = 1)]
//!     data: Cow<'a, [u8]>,
//!     #[columnar(key = 2)]
//!     marker: PhantomData<&'b ()>,
//! }
//! ```
//!
//! # Streaming rows
//!
//! Mark a type with `#[columnar(iterable)]` to decode rows lazily:
//!
//! ```
//! use columnar::Columnar;
//!
//! #[derive(Columnar)]
//! #[columnar(iterable)]
//! struct Sale {
//!     #[columnar(key = 1)]
//!     id: u64,
//!     #[columnar(key = 2)]
//!     region: String,
//! }
//!
//! let sales = vec![Sale { id: 1, region: "eu".into() }];
//! let bytes = columnar::to_vec(&sales).unwrap();
//! for row in columnar::iter_from_slice::<Sale>(&bytes).unwrap() {
//!     let row = row.unwrap();
//!     assert_eq!(row.id, 1);
//! }
//! ```
//!
//! The row iterator decodes column data on demand. It does not allocate a
//! `Vec` of rows, a `Vec` per column, or intermediate length/offset/tag
//! vectors, with two exceptions: RLE and dictionary columns materialize their
//! run values or dictionary once per column. Per-value allocations still
//! depend on the field type: `String`, `Vec<T>`, `BTreeMap<K, V>` and `Box<T>`
//! allocate when a row is yielded; `Cow<'a, str>` and `Cow<'a, [u8]>` borrow
//! from the input.
//!
//! Every field type must implement [`FieldIter`]. Built-in types and iterable
//! derived types do. A custom type that only implements [`FieldDecode`] can
//! use [`from_slice`] as usual, but needs a small [`FieldIter`] implementation
//! to participate in [`iter_from_slice`].
//!
//! Nested derived types must also be marked iterable:
//!
//! ```compile_fail
//! use columnar::Columnar;
//!
//! #[derive(Columnar)]
//! struct Inner {
//!     #[columnar(key = 1)]
//!     value: u64,
//! }
//!
//! #[derive(Columnar)]
//! #[columnar(iterable)]
//! struct Outer {
//!     #[columnar(key = 1)]
//!     inner: Inner,
//! }
//!
//! fn iterate(bytes: &[u8]) {
//!     let _ = columnar::iter_from_slice::<Outer>(bytes);
//! }
//! ```
//!
//! # Envelope
//!
//! The whole value is one CBOR map:
//!
//! ```text
//! {
//!   0: [version, rows],          // header, always first
//!   1: [encoding, payload],      // field with key 1
//!   2: [encoding, payload],      // field with key 2
//!   ...
//! }
//! ```
//!
//! - Key `0` is reserved for the header. Field keys start at `1`.
//! - `version` is currently `1`.
//! - `rows` is the logical number of values in every top-level column.
//! - There is **no per-column length** and **no per-column type tag**. The
//!   Rust field type tells the decoder how to interpret each payload.
//! - Unknown field keys are skipped by the decoder.
//! - Duplicate keys, a missing header, a misplaced header, trailing bytes and
//!   malformed payloads are rejected.
//!
//! # Default encodings
//!
//! The defaults are chosen per Rust type:
//!
//! | Rust type | Default encoding |
//! |---|---|
//! | `bool` | 1 bit per value, or RLE when Auto finds long runs |
//! | `u8`–`u64`, `i8`–`i64`, `usize`, `isize` | Auto: packed when the range fills the type, otherwise frame-of-reference + bit-packing; switches to RLE or delta encodings when a linear scan shows they are smaller |
//! | `u128`, `i128` | 128-bit little-endian, or RLE when Auto finds runs |
//! | `f32`, `f64` | raw IEEE bits, little-endian, or RLE when Auto finds runs |
//! | `char` | `u32` values with Unicode validation (Auto may use RLE/delta like `u32`) |
//! | `String` | varint lengths + UTF-8 bytes; RLE when Auto finds long runs |
//! | `Cow<'a, [u8]>` | varint lengths + raw bytes; decoded as borrowed slices; RLE when Auto finds long runs |
//! | `Cow<'a, str>` | varint lengths + UTF-8 bytes; decoded as borrowed slices; RLE when Auto finds long runs |
//! | `Vec<T>` | varint element lengths + child column |
//! | `BTreeMap<K, V>` | lengths + sorted key/value child columns |
//! | `Option<T>` | all-null, or validity bitmap + dense inner column |
//! | struct | one child column per field |
//! | enum | bit-packed tags + dense per-variant columns |
//!
//! `Option<Option<T>>` is preserved: nested options are encoded recursively.
//!
//! # Field attributes
//!
//! The derive accepts these field attributes:
//!
//! - `#[columnar(key = N)]` — stable envelope key. `0` is reserved and is a
//!   compile-time error. Duplicate keys are also compile-time errors. Fields
//!   without a key get the next free key starting at `1`.
//! - `#[columnar(encoding = "name")]` — override the default encoding. See
//!   [`Encoding`] for the names.
//! - `#[columnar(skip)]` — do not encode the field; decode it with its default.
//! - `#[columnar(default)]` or `#[columnar(default = "path")]` — default for a
//!   skipped or missing field.
//!
//! The container accepts `#[columnar(borrow = "'a")]` to choose which lifetime
//! is used for borrowed decoding when the type has more than one lifetime.
//!
//! Supported encoding overrides:
//!
//! | Types | Names |
//! |---|---|
//! | integers | `packed`, `for`, `varint`, `rle`, `delta_rle`, `delta_of_delta`, `dictionary` |
//! | `bool` | `bitpack`, `rle` |
//! | floats | `packed`, `rle` |
//! | `char` | same as `u32` |
//! | `String`, `Cow<'a, str>`, `Cow<'a, [u8]>` | `lengths`, `offsets`, `rle`, `dictionary` |
//! | `Vec<T>` | `lengths`, `offsets`, `list_lengths`, `list_offsets` |
//! | `Option<T>` | `nullable` (inner defaults still apply) |
//!
//! [`Encoding::Auto`] also considers `rle`, `delta_rle` and
//! `delta_of_delta` using a cheap statistics scan (first/second differences are sampled on large columns). An alternative is
//! chosen only when its estimated payload is at least 5% smaller than the
//! packed/FOR payload and the column has at least 32 values, so small batches
//! and random data keep the simple encodings. The delta encodings only support
//! integer columns up to 64 bits (`u8`–`u64`, `i8`–`i64`, `usize`, `isize`,
//! and `char` through `u32`); `u128`, `i128` and floats reject the explicit
//! delta hints:
//!
//! - `rle` compresses adjacent equal values.
//! - `delta_rle` compresses repeated first differences. It is the best choice
//!   for identifiers, counters and fixed-step timestamps.
//! - `delta_of_delta` stores second differences as signed varints. It is the
//!   best choice when first differences change often but second differences
//!   stay small, such as jittered timestamps.
//!
//! Unsupported combinations return [`Error::UnsupportedEncoding`].
//!
//! # Binary payloads
//!
//! Each column starts with a small integer encoding id and its payload:
//!
//! | Id | Name | Payload |
//! |---:|---|---|
//! | 1 | struct | array of child columns |
//! | 2 | enum | tag byte string + dense variant columns |
//! | 3 | packed | little-endian fixed-width values |
//! | 4 | FOR | `[base, bit_width, packed_offsets]` |
//! | 5 | bitpack | bit-packed values with a type-defined width |
//! | 6 | varint | unsigned LEB128; signed values are zig-zag encoded |
//! | 7 | RLE | `[run_lengths, values_column]` |
//! | 8 | dictionary | `[dictionary_len, values_column, bit_packed_indices]` |
//! | 9 | lengths | `[lengths, bytes_or_utf8]` |
//! | 10 | offsets | `[offset_width, offsets, bytes_or_utf8]` |
//! | 11 | nullable | `[validity_bitmap, dense_inner_column]` |
//! | 12 | list lengths | `[element_lengths, child_column]` |
//! | 13 | list offsets | `[offset_width, offsets, child_column]` |
//! | 14 | all null | empty array |
//! | 15 | delta RLE | `[run_lengths, varint_deltas]` |
//! | 16 | delta of delta | varint column of `first_value` then delta-of-deltas |
//!
//! All integers in payloads are little-endian unless they are CBOR integers.
//! Bit-packed values are LSB-first. Lengths and offsets use checked arithmetic
//! and are validated against the logical row count.
//!
//! Encodings 15 and 16 are additive extensions. Data that does not use them is
//! still readable by older decoders; data that uses them requires a decoder
//! that knows them.
//!
//! # Limits
//!
//! [`from_slice`] and [`iter_from_slice`] use [`DecodeOptions::default`], which
//! bounds rows, strings, list sizes, nesting depth, encoded envelope size and
//! the decoded size of one container. Use the `*_with_options` variants to
//! tighten or relax the limits. With the default limits, decoding never panics
//! on malformed input; invalid data returns an [`Error`]. With
//! [`DecodeOptions::unlimited`], a forged header can request very large
//! allocations, so only use it for trusted data.
//!
//! With [`iter_from_slice`], errors in a column payload can be returned later as
//! an `Err` item from the row iterator. Envelope-level errors are returned when
//! the iterator is constructed.
//!
//! # Compile-time key rules
//!
//! Key `0` and duplicate keys are rejected by the derive:
//!
//! ```compile_fail
//! use columnar::Columnar;
//!
//! #[derive(Columnar)]
//! struct Bad {
//!     #[columnar(key = 0)]
//!     value: u64,
//! }
//! ```
//!
//! ```compile_fail
//! use columnar::Columnar;
//!
//! #[derive(Columnar)]
//! struct Bad {
//!     #[columnar(key = 1)]
//!     a: u64,
//!     #[columnar(key = 1)]
//!     b: u64,
//! }
//! ```
//!
//! # Features
//!
//! | Feature | Default | Description |
//! |---|---|---|
//! | `std` | yes | Standard library integration and `std::error::Error`. |
//!
//! The crate always requires `alloc`, and `#[derive(Columnar)]` is always
//! available. `no_std` builds should disable default features:
//!
//! ```toml
//! columnar = { git = "https://github.com/rust-stdx/stdx", default-features = false }
//! ```
//!
//! # Limitations
//!
//! - `Cow<'a, [u8]>` and `Cow<'a, str>` are the borrowed types in v1. `String`
//!   decodes as an owned value.
//! - `HashMap`, fixed-size arrays and external container types are not
//!   implemented yet; use `BTreeMap` for maps.
//! - Lazy row iteration requires `#[columnar(iterable)]` on the row type and on
//!   every nested derived type used as a field.
//! - The format is stable for version `1`; adding a new wire version will be
//!   required for incompatible changes.

extern crate alloc;

#[allow(missing_docs)]
mod bitpack;
mod error;
mod field;
#[allow(missing_docs)]
mod format;
#[allow(missing_docs)]
mod iterators;
mod options;

pub use columnar_derive::Columnar;
pub use error::{Error, Result};
pub use field::{Field, FieldDecode};
pub use format::Encoding;
pub use options::DecodeOptions;

/// A Rust type that can be encoded as a columnar batch.
///
/// Implement this with `#[derive(Columnar)]` for a struct or enum. The derive
/// generates [`Columnar`], [`ColumnarDecode`], [`Field`] and [`FieldDecode`], so
/// the type can also be nested inside another derived type. With the
/// `#[columnar(iterable)]` container attribute it also generates
/// [`ColumnarIter`] and [`FieldIter`].
pub trait Columnar: Sized {
    /// Encode a slice of rows into one CBOR envelope.
    fn encode_rows(rows: &[Self]) -> Result<alloc::vec::Vec<u8>>;

    /// Encode a slice of rows into an existing buffer.
    ///
    /// The buffer is cleared first. Reusing a buffer avoids one allocation per
    /// call on hot paths. If encoding fails, the buffer contains an unspecified
    /// partial envelope and should be cleared before it is used again.
    fn encode_rows_into(rows: &[Self], out: &mut alloc::vec::Vec<u8>) -> Result<()> {
        let encoded = Self::encode_rows(rows)?;
        out.clear();
        out.extend_from_slice(&encoded);
        Ok(())
    }
}

/// Decode a columnar envelope while borrowing from `bytes`.
///
/// Implement this with `#[derive(Columnar)]`. Owned types can implement it for
/// any `'de`; types containing borrowed fields such as `Cow<'de, [u8]>` or
/// `Cow<'de, str>` implement it for the matching lifetime.
pub trait ColumnarDecode<'de>: Sized {
    /// Decode one CBOR envelope into rows.
    fn decode_rows(bytes: &'de [u8], options: DecodeOptions) -> Result<alloc::vec::Vec<Self>>;
}

/// A field type that can be decoded lazily, one value at a time.
///
/// This is the iterator counterpart of [`FieldDecode`]. Implement it for a
/// custom field type to use that type with [`iter_from_slice`]. A simple
/// materializing implementation can forward to [`FieldDecode::decode_column`]:
///
/// ```
/// # use columnar::{DecodeOptions, FieldDecode, FieldIter, Result};
/// # struct MyType;
/// # impl<'de> FieldDecode<'de> for MyType {
/// #     fn decode_column(
/// #         _d: &mut columnar::__private::Decoder<'de>,
/// #         _rows: usize,
/// #         _options: &DecodeOptions,
/// #     ) -> Result<Vec<Self>> {
/// #         Ok(Vec::new())
/// #     }
/// # }
/// impl<'de> FieldIter<'de> for MyType {
///     type Iter = columnar::__private::Materialized<Self>;
///
///     fn decode_column_iter(
///         d: &mut columnar::__private::Decoder<'de>,
///         rows: usize,
///         options: &DecodeOptions,
///     ) -> Result<Self::Iter> {
///         Ok(columnar::__private::Materialized::new(
///             Self::decode_column(d, rows, options)?,
///         ))
///     }
/// }
/// ```
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not support lazy column iteration",
    label = "this field type only implements `FieldDecode`",
    note = "implement `FieldIter` for `{Self}`, or decode the whole batch with `from_slice`"
)]
pub trait FieldIter<'de>: field::FieldDecode<'de> {
    /// Iterator over exactly the requested number of column values.
    type Iter: Iterator<Item = Result<Self>> + 'de;

    /// Decode column metadata and return a lazy value iterator.
    fn decode_column_iter(d: &mut cbor::Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter>;
}

/// A Rust type that can be iterated as a batch of rows.
///
/// Implement this with `#[derive(Columnar)]` and the
/// `#[columnar(iterable)]` container attribute. The row iterator decodes column
/// data lazily and does not allocate a `Vec` of rows.
pub trait ColumnarIter<'de>: Sized {
    /// Iterator over the rows in an envelope.
    type Rows: Iterator<Item = Result<Self>> + ExactSizeIterator + 'de;

    /// Parse an envelope and return a lazy row iterator.
    fn iter_rows(bytes: &'de [u8], options: DecodeOptions) -> Result<Self::Rows>;
}

/// Encode rows using the default options.
///
/// The returned bytes are one CBOR data item. Every top-level field is a
/// column; see the crate documentation for the exact envelope.
pub fn to_vec<T: Columnar>(rows: &[T]) -> Result<alloc::vec::Vec<u8>> {
    T::encode_rows(rows)
}

/// Encode rows into an existing buffer with the default options.
///
/// The buffer is cleared first and its allocation is reused. On error the
/// buffer holds an unspecified partial envelope.
pub fn to_vec_into<T: Columnar>(rows: &[T], out: &mut alloc::vec::Vec<u8>) -> Result<()> {
    T::encode_rows_into(rows, out)
}

/// Decode rows with the default [`DecodeOptions`].
pub fn from_slice<'de, T: ColumnarDecode<'de>>(bytes: &'de [u8]) -> Result<alloc::vec::Vec<T>> {
    T::decode_rows(bytes, DecodeOptions::default())
}

/// Decode rows with explicit limits.
pub fn from_slice_with_options<'de, T: ColumnarDecode<'de>>(
    bytes: &'de [u8],
    options: DecodeOptions,
) -> Result<alloc::vec::Vec<T>> {
    T::decode_rows(bytes, options)
}

/// Iterate over rows without materializing a `Vec` of rows.
///
/// The row type must be marked with `#[columnar(iterable)]`. Column data is
/// decoded lazily, so malformed data later in a column is returned as an
/// [`Error`] item from the iterator.
pub fn iter_from_slice<'de, T: ColumnarIter<'de>>(bytes: &'de [u8]) -> Result<T::Rows> {
    T::iter_rows(bytes, DecodeOptions::default())
}

/// Iterate over rows with explicit limits.
pub fn iter_from_slice_with_options<'de, T: ColumnarIter<'de>>(
    bytes: &'de [u8],
    options: DecodeOptions,
) -> Result<T::Rows> {
    T::iter_rows(bytes, options)
}

#[doc(hidden)]
pub mod __private {
    pub use alloc::{string::String, vec::Vec};

    pub use cbor::{Decoder, Encoder};

    pub use crate::{
        ColumnarDecode, ColumnarIter, DecodeOptions, Encoding, Error, Field, FieldDecode, FieldIter, Result,
        bitpack::{bits_needed, pack_u32, unpack_u32},
        format::{
            ENC_ENUM, ENC_STRUCT, SeenKeys, decoder, encoder, encoder_with_capacity, finish, read_array_len,
            read_column, read_header, read_map_len, write_column, write_header,
        },
        iterators::{Materialized, MaybeDefault, TagIter, count_enum_tags},
    };

    /// Return an allocation hint for `count` decoded values of type `T`.
    #[doc(hidden)]
    pub fn output_capacity<T>(options: &DecodeOptions, count: usize) -> Result<usize> {
        options.output_capacity::<T>(count)
    }

    /// Check that `count` decoded values of type `T` fit the output budget.
    #[doc(hidden)]
    pub fn check_output_bytes<T>(options: &DecodeOptions, count: usize) -> Result<()> {
        options.check_output_bytes::<T>(count)
    }

    /// Take the next value from a decoded column or return an error.
    ///
    /// The derive uses this instead of `unwrap` so that a user-provided
    /// `FieldDecode` that returns fewer values than `rows` is reported as an
    /// error instead of panicking.
    #[doc(hidden)]
    pub fn column_value<T>(iter: &mut impl Iterator<Item = T>, expected: usize) -> Result<T> {
        iter.next().ok_or(Error::LengthMismatch {
            expected,
            actual: 0,
        })
    }
}
