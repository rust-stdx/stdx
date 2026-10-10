//! Result rows and the [`FromRow`] trait.

use std::sync::Arc;

use bytes::Bytes;
use postgresql_protocol::{backend::FieldDescription, oid::Oid};

use crate::{decode::FromSql, error::Error, json::Json};

/// Metadata for one result column.
#[derive(Debug, Clone)]
pub struct Column {
    /// Column name.
    pub name: String,
    /// Column type OID.
    pub type_oid: Oid,
    /// Type size in bytes, or negative for variable length.
    pub type_size: i16,
    /// Type modifier.
    pub type_mod: i32,
    /// Source table OID, or `0`.
    pub table_oid: u32,
    /// Source attribute number, or `0`.
    pub column_attr: i16,
}

/// The column metadata shared by every row of one result set.
#[derive(Debug)]
pub struct Columns {
    columns: Vec<Column>,
}

impl Columns {
    /// Builds shared column metadata from protocol field descriptions.
    pub(crate) fn from_fields(fields: &[FieldDescription]) -> Arc<Self> {
        let columns: Vec<Column> = fields
            .iter()
            .map(|f| Column {
                name: f.name.clone(),
                type_oid: f.type_oid,
                type_size: f.type_size,
                type_mod: f.type_mod,
                table_oid: f.table_oid,
                column_attr: f.column_attr,
            })
            .collect();
        Arc::new(Columns {
            columns,
        })
    }

    /// The columns, in order.
    pub fn as_slice(&self) -> &[Column] {
        &self.columns
    }

    /// The number of columns.
    pub fn len(&self) -> usize {
        self.columns.len()
    }

    /// `true` if the result set has no columns.
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    /// Resolves a column name to its index.
    ///
    /// Uses a linear scan: result sets rarely have enough columns for a hash
    /// map allocation to pay off.
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|column| column.name == name)
    }
}

/// One result row.
///
/// Column values are borrowed slices of the received frame, so a row can be
/// cheaply cloned and kept beyond its originating query.
#[derive(Debug, Clone)]
pub struct Row {
    columns: Arc<Columns>,
    values: Box<[Option<Bytes>]>,
    zone: time::TimeZone,
}

impl Row {
    pub(crate) fn new(columns: Arc<Columns>, values: Box<[Option<Bytes>]>, zone: time::TimeZone) -> Self {
        Row {
            columns,
            values,
            zone,
        }
    }

    /// Parses a `DataRow` payload (starting with the column count).
    ///
    /// The payload comes straight from the server and is validated before it
    /// is trusted: a negative or oversized column count, a column length that
    /// runs past the frame, or a count that does not match the statement's
    /// result description is an error, never a panic.
    pub(crate) fn parse(columns: Arc<Columns>, payload: &Bytes, zone: time::TimeZone) -> Result<Self, Error> {
        if payload.len() < 2 {
            return Err(Error::Decode("data row is truncated".into()));
        }
        let raw_count = i16::from_be_bytes(payload[0..2].try_into().unwrap());
        let count = usize::try_from(raw_count)
            .map_err(|_| Error::Decode(format!("data row has an invalid column count {raw_count}")))?;
        if count != columns.len() {
            return Err(Error::Decode(format!(
                "data row has {count} columns but the statement returns {}",
                columns.len()
            )));
        }
        let mut pos = 2usize;

        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            if pos + 4 > payload.len() {
                return Err(Error::Decode("data row is truncated".into()));
            }
            let raw_len = i32::from_be_bytes(payload[pos..pos + 4].try_into().unwrap());
            pos += 4;
            if raw_len == -1 {
                values.push(None);
            } else {
                let len = usize::try_from(raw_len)
                    .map_err(|_| Error::Decode(format!("data row column has an invalid length {raw_len}")))?;
                if pos + len > payload.len() {
                    return Err(Error::Decode("data row column exceeds frame".into()));
                }
                values.push(Some(payload.slice(pos..pos + len)));
                pos += len;
            }
        }
        Ok(Row::new(columns, values.into_boxed_slice(), zone))
    }

    /// The column metadata.
    pub fn columns(&self) -> &[Column] {
        self.columns.as_slice()
    }

    /// The number of columns.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// `true` if the row has no columns.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns `true` if the column at `index` is `NULL`.
    pub fn is_null(&self, index: usize) -> bool {
        self.values.get(index).map(|v| v.is_none()).unwrap_or(true)
    }

    /// Decodes the column named `name`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ColumnNotFound`] if the name is unknown,
    /// [`Error::UnexpectedNull`] if the column is `NULL` and `T` is not
    /// [`Option`], or a decode error if the bytes are malformed.
    pub fn try_get<T: FromSql>(&self, name: &str) -> Result<T, Error> {
        let index = self
            .columns
            .index_of(name)
            .ok_or_else(|| Error::ColumnNotFound(name.to_string()))?;
        self.try_get_by_index(index)
    }

    /// Decodes the column at `index`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ColumnNotFound`] if the index is out of range,
    /// [`Error::UnexpectedNull`] if the column is `NULL` and `T` is not
    /// [`Option`], [`Error::Decode`] if the column's type cannot be decoded as
    /// `T`, or a decode error if the bytes are malformed.
    pub fn try_get_by_index<T: FromSql>(&self, index: usize) -> Result<T, Error> {
        let column = self
            .columns
            .as_slice()
            .get(index)
            .ok_or_else(|| Error::ColumnNotFound(format!("index {index}")))?;
        if !T::accepts(column.type_oid) {
            return Err(Error::Decode(format!(
                "column `{}` has type OID {} which cannot be decoded as the requested type",
                column.name, column.type_oid
            )));
        }
        match self.values.get(index).and_then(|v| v.as_ref()) {
            None => T::from_sql_null().map_err(|_| Error::UnexpectedNull(column.name.clone())),
            Some(bytes) => T::from_sql_zoned(column.type_oid, bytes, self.zone),
        }
    }

    fn value(&self, name: &str) -> Result<Option<&Bytes>, Error> {
        let index = self
            .columns
            .index_of(name)
            .ok_or_else(|| Error::ColumnNotFound(name.to_string()))?;
        Ok(self.values.get(index).and_then(|v| v.as_ref()))
    }

    /// Returns the named column as a borrowed UTF-8 string, without allocating.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ColumnNotFound`] if the name is unknown,
    /// [`Error::UnexpectedNull`] if the column is `NULL`, or a decode error if
    /// the bytes are not valid UTF-8.
    pub fn get_str<'a>(&'a self, name: &str) -> Result<&'a str, Error> {
        match self.value(name)? {
            None => Err(Error::UnexpectedNull(name.to_string())),
            Some(bytes) => Ok(std::str::from_utf8(bytes)?),
        }
    }

    /// Returns the named column as borrowed bytes, without allocating.
    ///
    /// Returns `None` for SQL `NULL`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ColumnNotFound`] if the name is unknown.
    pub fn get_bytes<'a>(&'a self, name: &str) -> Result<Option<&'a [u8]>, Error> {
        Ok(self.value(name)?.map(|bytes| bytes.as_ref()))
    }
}

/// A type that can be built from a [`Row`].
///
/// Implemented for scalars, tuples, arrays, and — via
/// `#[derive(postgresql::FromRow)]` — structs with named fields.
pub trait FromRow: Sized {
    /// Builds a value from a row.
    fn from_row(row: &Row) -> Result<Self, Error>;
}

macro_rules! impl_from_row_scalar {
    ($($ty:ty),* $(,)?) => {
        $(
            impl FromRow for $ty {
                fn from_row(row: &Row) -> Result<Self, Error> {
                    row.try_get_by_index(0)
                }
            }
        )*
    };
}

impl_from_row_scalar!(
    i16,
    i32,
    i64,
    f32,
    f64,
    bool,
    String,
    Vec<u8>,
    uuid::Uuid,
    time::DateTime,
    serde_json::Value,
    ipnetwork::IpNetwork,
    crate::interval::Interval,
);

impl<T: FromSql> FromRow for Option<T> {
    fn from_row(row: &Row) -> Result<Self, Error> {
        row.try_get_by_index(0)
    }
}

impl FromRow for Row {
    fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(row.clone())
    }
}

impl<T: FromSql> FromRow for Vec<T> {
    fn from_row(row: &Row) -> Result<Self, Error> {
        row.try_get_by_index(0)
    }
}

impl<T: serde::de::DeserializeOwned> FromRow for Json<T> {
    fn from_row(row: &Row) -> Result<Self, Error> {
        row.try_get_by_index(0)
    }
}

macro_rules! impl_from_row_tuple {
    ($($name:ident),+) => {
        #[allow(unused_assignments)]
        impl<$($name: FromSql),+> FromRow for ($($name,)+) {
            fn from_row(row: &Row) -> Result<Self, Error> {
                let mut index = 0usize;
                Ok((
                    $({
                        let value = row.try_get_by_index::<$name>(index)?;
                        index += 1;
                        value
                    },)+
                ))
            }
        }
    };
}

impl_from_row_tuple!(A);
impl_from_row_tuple!(A, B);
impl_from_row_tuple!(A, B, C);
impl_from_row_tuple!(A, B, C, D);
impl_from_row_tuple!(A, B, C, D, E);
impl_from_row_tuple!(A, B, C, D, E, F);
impl_from_row_tuple!(A, B, C, D, E, F, G);
impl_from_row_tuple!(A, B, C, D, E, F, G, H);

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use postgresql_protocol::backend::FieldDescription;

    use super::*;

    fn columns(names: &[(&str, Oid)]) -> Arc<Columns> {
        let fields: Vec<FieldDescription> = names
            .iter()
            .map(|(name, oid)| FieldDescription {
                name: (*name).to_string(),
                table_oid: 0,
                column_attr: 0,
                type_oid: *oid,
                type_size: -1,
                type_mod: -1,
                format: 1,
            })
            .collect();
        Columns::from_fields(&fields)
    }

    fn frame(parts: &[Option<&[u8]>]) -> Bytes {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&(parts.len() as i16).to_be_bytes());
        for part in parts {
            match part {
                None => buf.extend_from_slice(&(-1i32).to_be_bytes()),
                Some(bytes) => {
                    buf.extend_from_slice(&(bytes.len() as i32).to_be_bytes());
                    buf.extend_from_slice(bytes);
                }
            }
        }
        buf.freeze()
    }

    #[test]
    fn parses_values_and_nulls() {
        let cols = columns(&[("a", 25), ("b", 23)]);
        let payload = frame(&[Some(b"hi"), None]);
        let row = Row::parse(cols, &payload, time::TimeZone::UTC).unwrap();
        assert_eq!(row.get_str("a").unwrap(), "hi");
        assert!(row.is_null(1));
        assert_eq!(row.len(), 2);
    }

    #[test]
    fn negative_column_count_is_rejected() {
        let cols = columns(&[("a", 25)]);
        let mut payload = BytesMut::new();
        payload.extend_from_slice(&(-1i16).to_be_bytes());
        payload.extend_from_slice(&0i32.to_be_bytes());
        let err = Row::parse(cols, &payload.freeze(), time::TimeZone::UTC).unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "{err}");
    }

    #[test]
    fn column_count_must_match_the_statement() {
        // Fewer fields than the description: this is the shape of
        // RUSTSEC-2026-0178 (an out-of-bounds panic in `Row::get`).
        let cols = columns(&[("a", 25), ("b", 25)]);
        let payload = frame(&[Some(b"only one")]);
        let err = Row::parse(cols, &payload, time::TimeZone::UTC).unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "{err}");
    }

    #[test]
    fn truncated_rows_are_rejected() {
        let cols = columns(&[("a", 25)]);
        // Column length runs past the frame.
        let mut payload = BytesMut::new();
        payload.extend_from_slice(&1i16.to_be_bytes());
        payload.extend_from_slice(&100i32.to_be_bytes());
        payload.extend_from_slice(b"short");
        assert!(Row::parse(cols.clone(), &payload.freeze(), time::TimeZone::UTC).is_err());

        // Column length is negative but not the NULL marker.
        let mut payload = BytesMut::new();
        payload.extend_from_slice(&1i16.to_be_bytes());
        payload.extend_from_slice(&(-2i32).to_be_bytes());
        assert!(Row::parse(cols.clone(), &payload.freeze(), time::TimeZone::UTC).is_err());

        // Missing length prefix entirely.
        let mut payload = BytesMut::new();
        payload.extend_from_slice(&1i16.to_be_bytes());
        payload.extend_from_slice(&[1, 2]);
        assert!(Row::parse(cols, &payload.freeze(), time::TimeZone::UTC).is_err());
    }

    #[test]
    fn values_are_zero_copy_slices_of_the_frame() {
        let cols = columns(&[("a", 25)]);
        let payload = frame(&[Some(b"hello")]);
        let row = Row::parse(cols, &payload, time::TimeZone::UTC).unwrap();
        let text = row.get_str("a").unwrap();
        // The accessor borrows from the frame, it does not copy.
        assert!(payload.as_ptr_range().contains(&text.as_ptr()));
    }

    #[derive(serde::Serialize, serde::Deserialize, postgresql_derive::Json, Debug, PartialEq)]
    struct JsonUser {
        name: String,
    }

    #[test]
    fn json_types_decode_a_whole_column() {
        let mut jsonb = Vec::new();
        jsonb.push(1u8); // jsonb format version
        jsonb.extend_from_slice(br#"{"name":"carol"}"#);
        let payload = frame(&[Some(&jsonb)]);

        let cols = columns(&[("user", postgresql_protocol::oid::JSONBOID)]);
        let row = Row::parse(cols, &payload, time::TimeZone::UTC).unwrap();

        let derived = <JsonUser as FromRow>::from_row(&row).unwrap();
        assert_eq!(
            derived,
            JsonUser {
                name: "carol".to_string()
            }
        );

        let wrapped = <Json<JsonUser> as FromRow>::from_row(&row).unwrap();
        assert_eq!(
            wrapped.into_inner(),
            JsonUser {
                name: "carol".to_string()
            }
        );
    }

    #[test]
    fn decoding_into_the_wrong_type_is_an_error() {
        let cols = columns(&[("n", 23)]);
        let payload = frame(&[Some(&1i32.to_be_bytes())]);
        let row = Row::parse(cols, &payload, time::TimeZone::UTC).unwrap();
        // `bool` would otherwise silently reinterpret the first byte.
        assert!(row.try_get::<bool>("n").is_err());
        assert!(row.try_get::<String>("n").is_err());
        assert_eq!(row.try_get::<i32>("n").unwrap(), 1);
    }
}
