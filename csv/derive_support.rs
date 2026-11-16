//! Native derive support for CSV records.
//!
//! The traits and helpers in this module are public so that the derive macros
//! can refer to them, but they are not intended to be used directly.

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    ReadError, ReadErrorKind, WriteError,
    reader::Row,
    writer::{Write, write_csv_field},
};

/// Converts a CSV field into a value.
pub trait DecodeField: Sized {
    /// Convert `value` into this type.
    ///
    /// `None` means the column named by the struct field is not present in the
    /// header row. A present but empty field is `Some("")`.
    fn decode_field(value: Option<&str>) -> Result<Self, String>;
}

impl DecodeField for String {
    fn decode_field(value: Option<&str>) -> Result<Self, String> {
        Ok(value.unwrap_or("").to_string())
    }
}

macro_rules! impl_parse_field {
    ($($ty:ty),* $(,)?) => {
        $(
            impl DecodeField for $ty {
                fn decode_field(value: Option<&str>) -> Result<Self, String> {
                    value
                        .unwrap_or("")
                        .parse()
                        .map_err(|_| alloc::format!("invalid {}", stringify!($ty)))
                }
            }
        )*
    };
}

impl_parse_field!(bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl DecodeField for char {
    fn decode_field(value: Option<&str>) -> Result<Self, String> {
        let mut chars = value.unwrap_or("").chars();
        let ch = chars.next().ok_or_else(|| "empty char".to_string())?;
        if chars.next().is_some() {
            return Err("char field contains more than one character".to_string());
        }
        Ok(ch)
    }
}

impl<T: DecodeField> DecodeField for Option<T> {
    fn decode_field(value: Option<&str>) -> Result<Self, String> {
        match value {
            None | Some("") => Ok(None),
            Some(value) => T::decode_field(Some(value)).map(Some),
        }
    }
}

impl<T: DecodeField> DecodeField for Vec<T> {
    fn decode_field(_value: Option<&str>) -> Result<Self, String> {
        Err("cannot decode a sequence from a single CSV field".to_string())
    }
}

/// Row decoder used by `#[derive(Decode)]`.
pub struct Decoder<'a> {
    row: &'a Row,
    header_map: &'a BTreeMap<String, usize>,
}

impl<'a> Decoder<'a> {
    /// Create a decoder for `row`.
    pub fn new(row: &'a Row) -> Result<Self, ReadError> {
        if let Some(error) = row.error() {
            return Err(error.clone());
        }

        let line = row.inner.line;
        let header_map = row
            .header_map
            .as_ref()
            .ok_or_else(|| ReadError::new(ReadErrorKind::Decode("headers not set".into()), line, 0))?;
        if row.duplicate_headers {
            return Err(ReadError::new(
                ReadErrorKind::Decode("headers contain duplicate names".into()),
                line,
                0,
            ));
        }

        Ok(Decoder {
            row,
            header_map,
        })
    }

    /// Decode the field named `name`.
    pub fn field<T: DecodeField>(&self, name: &str) -> Result<T, ReadError> {
        let value = match self.header_map.get(name) {
            Some(&index) => match self.row.get(index) {
                Some(Ok(value)) => Some(value),
                Some(Err(error)) => return Err(error),
                None => {
                    return Err(ReadError::new(
                        ReadErrorKind::Decode("row has fewer fields than the headers require".into()),
                        self.row.inner.line,
                        0,
                    ));
                }
            },
            None => None,
        };

        T::decode_field(value).map_err(|message| ReadError::new(ReadErrorKind::Decode(message), self.row.inner.line, 0))
    }
}

/// Converts a value into a CSV field.
pub trait EncodeField {
    /// Append the encoded field to `buf`.
    fn encode_field(&self, buf: &mut Vec<u8>);
}

impl<T: EncodeField + ?Sized> EncodeField for &T {
    fn encode_field(&self, buf: &mut Vec<u8>) {
        (*self).encode_field(buf);
    }
}

impl EncodeField for str {
    fn encode_field(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }
}

impl EncodeField for String {
    fn encode_field(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }
}

impl EncodeField for bool {
    fn encode_field(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(if *self { b"true" } else { b"false" });
    }
}

macro_rules! impl_int_field {
    ($($ty:ty),* $(,)?) => {
        $(
            impl EncodeField for $ty {
                fn encode_field(&self, buf: &mut Vec<u8>) {
                    let mut number = core::fmt::NumBuffer::new();
                    buf.extend_from_slice(self.format_into(&mut number).as_bytes());
                }
            }
        )*
    };
}

impl_int_field!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl EncodeField for f32 {
    fn encode_field(&self, buf: &mut Vec<u8>) {
        let mut buffer = zmij::Buffer::new();
        buf.extend_from_slice(buffer.format(*self).as_bytes());
    }
}

impl EncodeField for f64 {
    fn encode_field(&self, buf: &mut Vec<u8>) {
        let mut buffer = zmij::Buffer::new();
        buf.extend_from_slice(buffer.format(*self).as_bytes());
    }
}

impl EncodeField for char {
    fn encode_field(&self, buf: &mut Vec<u8>) {
        let mut encoded = [0u8; 4];
        buf.extend_from_slice(self.encode_utf8(&mut encoded).as_bytes());
    }
}

impl<T: EncodeField> EncodeField for Option<T> {
    fn encode_field(&self, buf: &mut Vec<u8>) {
        if let Some(value) = self {
            value.encode_field(buf);
        }
    }
}

/// Record encoder used by `#[derive(Encode)]`.
pub struct Encoder<'a> {
    headers: &'a [String],
    values: &'a mut [Option<Vec<u8>>],
    sequence_index: usize,
}

impl<'a> Encoder<'a> {
    /// Write a named struct field.
    pub fn field<T: EncodeField + ?Sized>(&mut self, name: &str, value: &T) -> Result<(), WriteError> {
        let position = self
            .headers
            .iter()
            .position(|header| header == name)
            .ok_or_else(|| WriteError::Encode(format!("unknown field '{name}'")))?;

        let slot = self.values[position].get_or_insert_with(Vec::new);
        slot.clear();
        value.encode_field(slot);
        Ok(())
    }

    /// Start a positional sequence.
    pub fn start_sequence(&mut self, len: usize) -> Result<(), WriteError> {
        if len != self.headers.len() {
            return Err(WriteError::Encode("sequence length does not match header count".to_string()));
        }
        self.sequence_index = 0;
        Ok(())
    }

    /// Write the next positional sequence element.
    pub fn element<T: EncodeField + ?Sized>(&mut self, value: &T) -> Result<(), WriteError> {
        if self.sequence_index >= self.values.len() {
            return Err(WriteError::Encode("sequence longer than header count".to_string()));
        }
        let position = self.sequence_index;
        self.sequence_index += 1;

        let slot = self.values[position].get_or_insert_with(Vec::new);
        slot.clear();
        value.encode_field(slot);
        Ok(())
    }

    /// Finish a positional sequence.
    pub fn end_sequence(&mut self) -> Result<(), WriteError> {
        if self.sequence_index != self.values.len() {
            return Err(WriteError::Encode("sequence length does not match header count".to_string()));
        }
        Ok(())
    }
}

/// Decode a CSV record.
pub trait Decode: Sized {
    /// Decode this record from `decoder`.
    fn decode(decoder: &Decoder<'_>) -> Result<Self, ReadError>;
}

/// Encode a CSV record.
pub trait Encode {
    /// Encode this record into `encoder`.
    fn encode(&self, encoder: &mut Encoder<'_>) -> Result<(), WriteError>;
}

impl Row {
    /// Decode this row into `T`.
    ///
    /// Headers must have been set on the parent [`Reader`](crate::Reader) via
    /// [`parse_headers`](crate::Reader::parse_headers) or
    /// [`set_headers`](crate::Reader::set_headers) before calling this method.
    /// Struct fields are matched by column name.
    ///
    /// # Errors
    ///
    /// Returns [`ReadErrorKind::Decode`] if headers have not been set, if the
    /// headers contain duplicate names, or if a field cannot be converted.
    pub fn decode<T: Decode>(&self) -> Result<T, ReadError> {
        let decoder = Decoder::new(self)?;
        T::decode(&decoder)
    }
}

impl<W: Write> crate::Writer<W> {
    /// Encode a record and write it as a CSV data row.
    ///
    /// Headers must have been set with [`set_headers`](crate::Writer::set_headers)
    /// or [`write_headers`](crate::Writer::write_headers).
    ///
    /// Struct fields are matched by name against the stored headers and written
    /// in header column order. Sequences and tuples are written positionally.
    ///
    /// # Errors
    ///
    /// Returns [`WriteError::Encode`] if headers have not been set, if a struct
    /// field has no matching header, or if a sequence length does not match the
    /// header count. Returns [`WriteError::InconsistentFieldCount`] if the row
    /// conflicts with a previously established field count.
    pub fn encode<T: Encode + ?Sized>(&mut self, record: &T) -> Result<(), WriteError> {
        if self.headers.is_none() {
            return Err(WriteError::Encode("headers not set".to_string()));
        }
        self.write_pending_bom();

        let header_count = self.headers.as_ref().unwrap().len();
        let delimiter = self.delimiter;

        if self.encode_values.len() > header_count {
            self.encode_values.truncate(header_count);
        }
        while self.encode_values.len() < header_count {
            self.encode_values.push(None);
        }
        for bytes in self.encode_values.iter_mut().flatten() {
            bytes.clear();
        }

        {
            let headers = self.headers.as_ref().unwrap();
            let mut encoder = Encoder {
                headers,
                values: &mut self.encode_values,
                sequence_index: 0,
            };
            record.encode(&mut encoder)?;
        }

        match self.num_fields {
            Some(expected) if !self.flexible && header_count != expected => {
                return Err(WriteError::InconsistentFieldCount {
                    expected,
                    found: header_count,
                    row: self.row_count + 1,
                });
            }
            None => {
                self.num_fields = Some(header_count);
            }
            _ => {}
        }

        for (index, value) in self.encode_values.iter().enumerate() {
            if index > 0 {
                self.buf.push(delimiter);
            }
            match value {
                Some(field) => write_csv_field(&mut self.buf, delimiter, field),
                None => write_csv_field(&mut self.buf, delimiter, b""),
            }
        }
        self.buf.extend_from_slice(b"\r\n");
        self.row_count += 1;

        if self.buf.len() >= 8192 {
            self.flush()?;
        }

        Ok(())
    }
}

impl Encode for () {
    fn encode(&self, _encoder: &mut Encoder<'_>) -> Result<(), WriteError> {
        Ok(())
    }
}

impl<T: EncodeField> Encode for [T] {
    fn encode(&self, encoder: &mut Encoder<'_>) -> Result<(), WriteError> {
        encoder.start_sequence(self.len())?;
        for value in self {
            encoder.element(value)?;
        }
        encoder.end_sequence()
    }
}

impl<T: EncodeField, const N: usize> Encode for [T; N] {
    fn encode(&self, encoder: &mut Encoder<'_>) -> Result<(), WriteError> {
        self.as_slice().encode(encoder)
    }
}

impl<T: EncodeField> Encode for Vec<T> {
    fn encode(&self, encoder: &mut Encoder<'_>) -> Result<(), WriteError> {
        self.as_slice().encode(encoder)
    }
}

macro_rules! impl_tuple_encode {
    ($count:expr; $($index:tt => $name:ident),+ $(,)?) => {
        impl<$($name: EncodeField),+> Encode for ($($name,)+) {
            fn encode(&self, encoder: &mut Encoder<'_>) -> Result<(), WriteError> {
                encoder.start_sequence($count)?;
                $(encoder.element(&self.$index)?;)+
                encoder.end_sequence()
            }
        }
    };
}

impl_tuple_encode!(1; 0 => A);
impl_tuple_encode!(2; 0 => A, 1 => B);
impl_tuple_encode!(3; 0 => A, 1 => B, 2 => C);
impl_tuple_encode!(4; 0 => A, 1 => B, 2 => C, 3 => D);
impl_tuple_encode!(5; 0 => A, 1 => B, 2 => C, 3 => D, 4 => E);
impl_tuple_encode!(6; 0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F);
impl_tuple_encode!(7; 0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G);
impl_tuple_encode!(8; 0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H);
