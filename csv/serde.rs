#![cfg(feature = "serde")]

extern crate alloc;

use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

use serde::{
    de::{self, DeserializeSeed, Deserializer, Error as _, SeqAccess, Visitor},
    ser,
};

use crate::{
    error::{ReadError, ReadErrorKind, WriteError},
    reader::{FieldRange, Row},
    writer::{Write, Writer, write_csv_field},
};

impl Row {
    /// Deserialize this row into a `T`.
    ///
    /// Headers must have been set on the parent [`Reader`](crate::Reader)
    /// via [`parse_headers`](crate::Reader::parse_headers) or
    /// [`set_headers`](crate::Reader::set_headers) before calling this method.
    /// Struct fields are matched by column name.
    ///
    /// # Errors
    ///
    /// Returns [`ReadError`] with kind [`Deserialize`](crate::ReadErrorKind::Deserialize)
    /// if headers have not been set, or if deserialization fails (type mismatch,
    /// unknown field, etc.).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use csv::Reader;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Record { name: String, age: u32 }
    ///
    /// let mut reader = Reader::new(std::io::Cursor::new(b"name,age\nAlice,30\n"));
    /// reader.parse_headers()?;
    /// for row in reader.rows() {
    ///     let rec: Record = row.deserialize()?;
    ///     println!("{} is {}", rec.name, rec.age);
    /// }
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn deserialize<T>(&self) -> Result<T, ReadError>
    where
        T: serde::de::DeserializeOwned,
    {
        if self.inner.error.is_some() {
            return Err(self.inner.error.clone().unwrap());
        }

        let header_map = self
            .header_map
            .as_ref()
            .ok_or_else(|| ReadError::new(ReadErrorKind::Deserialize("headers not set".into()), 0, 0))?;
        let mut deser = HeaderRow {
            buf: &self.inner.buf,
            ranges: &self.inner.ranges,
            header_map,
            struct_fields: &[],
            index: 0,
        };
        T::deserialize(&mut deser).map_err(|e| ReadError::new(ReadErrorKind::Deserialize(e.msg), 0, 0))
    }
}

struct HeaderRow<'de> {
    buf: &'de [u8],
    ranges: &'de [FieldRange],
    header_map: &'de BTreeMap<String, usize>,
    struct_fields: &'static [&'static str],
    index: usize,
}

impl<'de> HeaderRow<'de> {
    fn field(&self, idx: usize) -> Result<&'de str, CsvError> {
        let range = self
            .ranges
            .get(idx)
            .ok_or_else(|| CsvError::custom("field index out of bounds"))?;
        core::str::from_utf8(&self.buf[range.start..range.end]).map_err(|_| CsvError::custom("invalid UTF-8 in field"))
    }
}

impl<'de> Deserializer<'de> for &mut HeaderRow<'de> {
    type Error = CsvError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_struct("", &[], visitor)
    }

    fn deserialize_struct<V>(
        mut self,
        _name: &'static str,
        struct_fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.struct_fields = struct_fields;
        self.index = 0;
        visitor.visit_seq(&mut self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

impl<'de> SeqAccess<'de> for HeaderRow<'de> {
    type Error = CsvError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.index >= self.struct_fields.len() {
            return Ok(None);
        }
        let field_name = self.struct_fields[self.index];
        self.index += 1;
        let val = match self.header_map.get(field_name) {
            Some(&idx) => self.field(idx).unwrap_or(""),
            None => "",
        };
        seed.deserialize(FieldDeserializer(val)).map(Some)
    }
}

/// Deserializes a single CSV field value with proper type coercion.
struct FieldDeserializer<'a>(&'a str);

macro_rules! forward_parse {
    ($($method:ident => $visit:ident :: $ty:ty),*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where V: Visitor<'de>,
            {
                let v: $ty = self.0.parse().map_err(|_| {
                    CsvError::custom(concat!("invalid ", stringify!($ty)))
                })?;
                visitor.$visit(v)
            }
        )*
    };
}

impl<'de, 'a: 'de> Deserializer<'de> for FieldDeserializer<'a> {
    type Error = CsvError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.0)
    }

    forward_parse! {
        deserialize_bool   => visit_bool   :: bool,
        deserialize_i8     => visit_i8     :: i8,
        deserialize_i16    => visit_i16    :: i16,
        deserialize_i32    => visit_i32    :: i32,
        deserialize_i64    => visit_i64    :: i64,
        deserialize_i128   => visit_i128   :: i128,
        deserialize_u8     => visit_u8     :: u8,
        deserialize_u16    => visit_u16    :: u16,
        deserialize_u32    => visit_u32    :: u32,
        deserialize_u64    => visit_u64    :: u64,
        deserialize_u128   => visit_u128   :: u128,
        deserialize_f32    => visit_f32    :: f32,
        deserialize_f64    => visit_f64    :: f64
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut chars = self.0.chars();
        let ch = chars.next().ok_or_else(|| CsvError::custom("empty char"))?;
        if chars.next().is_some() {
            return Err(CsvError::custom("char field contains more than one character"));
        }
        visitor.visit_char(ch)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.0)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.0.to_owned())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.0.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.0.as_bytes().to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.0.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(CsvError::custom("cannot deserialize sequence from a single field"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(CsvError::custom("cannot deserialize tuple from a single field"))
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(CsvError::custom("cannot deserialize tuple struct from a single field"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(CsvError::custom("cannot deserialize map from a single field"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(CsvError::custom("cannot deserialize struct from a single field"))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(CsvError::custom("cannot deserialize enum from a single field"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.0)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

#[derive(Debug)]
pub struct CsvError {
    pub(crate) msg: String,
}

impl de::Error for CsvError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        CsvError {
            msg: msg.to_string(),
        }
    }
}

impl ser::Error for CsvError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        CsvError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CsvError {}

#[cfg(all(not(feature = "std"), feature = "serde"))]
impl core::error::Error for CsvError {}

// ── Serialization ─────────────────────────────────────────────────────

impl<W: Write> Writer<W> {
    /// Serialize a record and write it as a CSV data row.
    ///
    /// Headers must have been set (via [`set_headers`](Self::set_headers) or
    /// [`write_headers`](Self::write_headers)) before calling this method.
    ///
    /// For structs, fields are matched by name against the stored headers and
    /// written in header column order. For sequences and tuples, elements are
    /// written positionally (the field count must equal the header count).
    ///
    /// # Errors
    ///
    /// Returns [`WriteError::Serialize`] if headers have not been set, or if
    /// a serde serialization error occurs (e.g. an unknown field name, or an
    /// unsupported type like a map or enum).
    ///
    /// Returns [`WriteError::InconsistentFieldCount`] if the number of fields
    /// serialized differs from the expected count (unless flexible mode is
    /// enabled).
    ///
    /// Returns [`WriteError::Io`] if the underlying writer fails on flush.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use csv::Writer;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let mut w = Writer::new(Vec::new())
    ///     .set_headers(vec!["name".into(), "age".into()]);
    ///
    /// let alice = Person { name: "Alice".into(), age: 30 };
    /// w.serialize(&alice)?;
    /// let result = String::from_utf8(w.into_inner()?).unwrap();
    /// assert_eq!(result, "Alice,30\r\n");
    /// # Ok::<_, csv::WriteError>(())
    /// ```
    pub fn serialize<T: serde::Serialize>(&mut self, record: &T) -> Result<(), WriteError> {
        if self.headers.is_none() {
            return Err(WriteError::Serialize("headers not set".into()));
        }
        let headers = self.headers.as_ref().unwrap();
        let header_count = headers.len();
        let delimiter = self.delimiter;

        self.ser_values.clear();
        self.ser_values.resize_with(header_count, || None);
        self.ser_capture_buf.clear();
        let mut field_count = 0;

        {
            let mut ser = StructSer {
                headers,
                values: &mut self.ser_values,
                field_count: &mut field_count,
                capture_buf: &mut self.ser_capture_buf,
            };
            record.serialize(&mut ser).map_err(|e| WriteError::Serialize(e.msg))?;
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

        for (i, val) in self.ser_values.iter().enumerate() {
            if i > 0 {
                self.buf.push(delimiter);
            }
            match val {
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

/// Top-level serializer that collects field values from a serde record.
struct StructSer<'a, 'b> {
    headers: &'a [String],
    values: &'b mut Vec<Option<Vec<u8>>>,
    field_count: &'b mut usize,
    capture_buf: &'b mut Vec<u8>,
}

impl<'a, 'b> serde::Serializer for &'b mut StructSer<'a, 'b> {
    type Ok = ();
    type Error = CsvError;

    type SerializeSeq = SeqWriter<'b>;
    type SerializeTuple = SeqWriter<'b>;
    type SerializeTupleStruct = SeqWriter<'b>;
    type SerializeTupleVariant = serde::ser::Impossible<(), CsvError>;
    type SerializeMap = serde::ser::Impossible<(), CsvError>;
    type SerializeStruct = StructCollector<'a, 'b>;
    type SerializeStructVariant = serde::ser::Impossible<(), CsvError>;

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<StructCollector<'a, 'b>, CsvError> {
        Ok(StructCollector {
            headers: self.headers,
            values: self.values,
            field_count: self.field_count,
            capture_buf: self.capture_buf,
        })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<SeqWriter<'b>, CsvError> {
        Ok(SeqWriter {
            values: self.values,
            field_count: self.field_count,
            idx: 0,
            capture_buf: self.capture_buf,
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<SeqWriter<'b>, CsvError> {
        self.serialize_seq(Some(_len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<SeqWriter<'b>, CsvError> {
        self.serialize_seq(Some(_len))
    }

    fn serialize_bool(self, v: bool) -> Result<(), CsvError> {
        let s = if v { "true" } else { "false" };
        self.serialize_str(s)
    }

    fn serialize_i8(self, v: i8) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_i16(self, v: i16) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_i32(self, v: i32) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_i64(self, v: i64) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_i128(self, v: i128) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u8(self, v: u8) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u16(self, v: u16) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u32(self, v: u32) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u64(self, v: u64) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u128(self, v: u128) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_f32(self, v: f32) -> Result<(), CsvError> {
        let mut buf = ryu::Buffer::new();
        self.serialize_str(buf.format(v))
    }
    fn serialize_f64(self, v: f64) -> Result<(), CsvError> {
        let mut buf = ryu::Buffer::new();
        self.serialize_str(buf.format(v))
    }

    fn serialize_char(self, v: char) -> Result<(), CsvError> {
        let mut buf = [0u8; 4];
        let s = v.encode_utf8(&mut buf);
        self.serialize_str(s)
    }

    fn serialize_str(self, v: &str) -> Result<(), CsvError> {
        self.write_field_bytes(v.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), CsvError> {
        self.write_field_bytes(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<(), CsvError> {
        self.write_field_bytes(b"");
        Ok(())
    }

    fn serialize_some<T: ?Sized + serde::Serialize>(self, value: &T) -> Result<(), CsvError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), CsvError> {
        self.write_field_bytes(b"");
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), CsvError> {
        self.write_field_bytes(b"");
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), CsvError> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), CsvError> {
        Err(CsvError::custom("enum variants not supported"))
    }

    fn serialize_unit_variant(self, _name: &'static str, _idx: u32, _variant: &'static str) -> Result<(), CsvError> {
        Err(CsvError::custom("enum variants not supported"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("enum variants not supported"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("enum variants not supported"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("maps not supported"))
    }
}

impl StructSer<'_, '_> {
    fn write_field_bytes(&mut self, bytes: &[u8]) {
        if let Some(slot) = self.values.first_mut() {
            *slot = Some(bytes.to_vec());
        }
        *self.field_count = 1;
    }
}

/// Collects struct fields by name and stores them at header-matching positions.
struct StructCollector<'a, 'b> {
    headers: &'a [String],
    values: &'b mut Vec<Option<Vec<u8>>>,
    field_count: &'b mut usize,
    capture_buf: &'b mut Vec<u8>,
}

impl ser::SerializeStruct for StructCollector<'_, '_> {
    type Ok = ();
    type Error = CsvError;

    fn serialize_field<T: ?Sized + serde::Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), CsvError> {
        let pos = self
            .headers
            .iter()
            .position(|h| h == key)
            .ok_or_else(|| CsvError::custom(alloc::format!("unknown field '{key}'")))?;

        self.capture_buf.clear();
        {
            let mut capture = FieldCapture {
                buf: self.capture_buf,
            };
            value.serialize(&mut capture)?;
        }
        self.values[pos] = Some(core::mem::take(self.capture_buf));
        *self.field_count += 1;
        Ok(())
    }

    fn end(self) -> Result<(), CsvError> {
        Ok(())
    }
}

/// Writes seq/tuple elements positionally into the values array.
struct SeqWriter<'b> {
    values: &'b mut Vec<Option<Vec<u8>>>,
    field_count: &'b mut usize,
    idx: usize,
    capture_buf: &'b mut Vec<u8>,
}

impl ser::SerializeSeq for SeqWriter<'_> {
    type Ok = ();
    type Error = CsvError;

    fn serialize_element<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<(), CsvError> {
        if self.idx >= self.values.len() {
            return Err(CsvError::custom("sequence longer than header count"));
        }
        self.capture_buf.clear();
        {
            let mut capture = FieldCapture {
                buf: self.capture_buf,
            };
            value.serialize(&mut capture)?;
        }
        self.values[self.idx] = Some(core::mem::take(self.capture_buf));
        self.idx += 1;
        *self.field_count += 1;
        Ok(())
    }

    fn end(self) -> Result<(), CsvError> {
        if self.idx != self.values.len() {
            return Err(CsvError::custom("sequence length does not match header count"));
        }
        Ok(())
    }
}

impl ser::SerializeTuple for SeqWriter<'_> {
    type Ok = ();
    type Error = CsvError;

    fn serialize_element<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<(), CsvError> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), CsvError> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SeqWriter<'_> {
    type Ok = ();
    type Error = CsvError;

    fn serialize_field<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<(), CsvError> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), CsvError> {
        ser::SerializeSeq::end(self)
    }
}

/// Captures a single serialized field value into a `Vec<u8>`.
struct FieldCapture<'a> {
    buf: &'a mut Vec<u8>,
}

impl<'a> serde::Serializer for &'a mut FieldCapture<'a> {
    type Ok = ();
    type Error = CsvError;

    type SerializeSeq = serde::ser::Impossible<(), CsvError>;
    type SerializeTuple = serde::ser::Impossible<(), CsvError>;
    type SerializeTupleStruct = serde::ser::Impossible<(), CsvError>;
    type SerializeTupleVariant = serde::ser::Impossible<(), CsvError>;
    type SerializeMap = serde::ser::Impossible<(), CsvError>;
    type SerializeStruct = serde::ser::Impossible<(), CsvError>;
    type SerializeStructVariant = serde::ser::Impossible<(), CsvError>;

    fn serialize_bool(self, v: bool) -> Result<(), CsvError> {
        let s = if v { "true" } else { "false" };
        self.buf.extend_from_slice(s.as_bytes());
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_i16(self, v: i16) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_i32(self, v: i32) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_i64(self, v: i64) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_i128(self, v: i128) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u8(self, v: u8) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u16(self, v: u16) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u32(self, v: u32) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u64(self, v: u64) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_u128(self, v: u128) -> Result<(), CsvError> {
        self.serialize_str(&format_number::format_int(v))
    }
    fn serialize_f32(self, v: f32) -> Result<(), CsvError> {
        let mut buf = ryu::Buffer::new();
        self.serialize_str(buf.format(v))
    }
    fn serialize_f64(self, v: f64) -> Result<(), CsvError> {
        let mut buf = ryu::Buffer::new();
        self.serialize_str(buf.format(v))
    }

    fn serialize_char(self, v: char) -> Result<(), CsvError> {
        let mut buf = [0u8; 4];
        let s = v.encode_utf8(&mut buf);
        self.buf.extend_from_slice(s.as_bytes());
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), CsvError> {
        self.buf.extend_from_slice(v.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), CsvError> {
        self.buf.extend_from_slice(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<(), CsvError> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + serde::Serialize>(self, value: &T) -> Result<(), CsvError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), CsvError> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), CsvError> {
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), CsvError> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), CsvError> {
        Err(CsvError::custom("enum variants not supported in field capture"))
    }

    fn serialize_unit_variant(self, _name: &'static str, _idx: u32, _variant: &'static str) -> Result<(), CsvError> {
        Err(CsvError::custom("enum variants not supported in field capture"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("enum variants not supported in field capture"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("enum variants not supported in field capture"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("sequence inside a single field not supported"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("tuple inside a single field not supported"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("tuple struct inside a single field not supported"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("map inside a single field not supported"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<serde::ser::Impossible<(), CsvError>, CsvError> {
        Err(CsvError::custom("struct inside a single field not supported"))
    }
}
