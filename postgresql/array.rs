//! Zero-allocation binding of lazy iterators as PostgreSQL arrays.

use std::sync::Mutex;

use bytes::BytesMut;
use postgresql_protocol::oid::{Oid, array_oid, array_oid_checked};

use crate::{
    encode::{IsNull, StaticType, ToSql, encode_array},
    error::Error,
};

/// A single-use adapter that binds an iterator as a PostgreSQL array.
///
/// The iterator is consumed on the first encode. Wrapping a `Vec` or slice is
/// unnecessary — those already implement [`ToSql`] as arrays.
pub struct Array<I> {
    inner: Mutex<Option<I>>,
}

impl<I> Array<I> {
    /// Wraps an iterator.
    pub fn new(iter: I) -> Self {
        Array {
            inner: Mutex::new(Some(iter)),
        }
    }
}

impl<I, T> ToSql for Array<I>
where
    I: Iterator<Item = T> + Send,
    T: ToSql + StaticType,
{
    fn encode(&self, buf: &mut BytesMut) -> Result<IsNull, Error> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| Error::Encode("array iterator mutex was poisoned".into()))?;
        let iter = guard
            .take()
            .ok_or_else(|| Error::Encode("array iterator has already been consumed".into()))?;
        encode_array(iter, T::OID, buf)
    }

    fn oid(&self) -> Oid {
        array_oid(T::OID).unwrap_or(0)
    }
}

impl<I, T> StaticType for Array<I>
where
    I: Iterator<Item = T> + Send,
    T: ToSql + StaticType,
{
    const OID: Oid = array_oid_checked(T::OID);
}

/// Binds a lazy iterator as a one-dimensional PostgreSQL array.
///
/// Use this inside a query macro argument for `UNNEST` / `= ANY`:
///
/// ```ignore
/// # async fn run(db: postgresql::Connection, uuids: Vec<uuid::Uuid>) -> Result<(), postgresql::Error> {
/// # use postgresql::query;
/// let rows = query!(
///     "SELECT id, name FROM users WHERE id = ANY($1::uuid[])",
///     postgresql::array(uuids.iter()))
/// .fetch_all::<_, (uuid::Uuid, String)>(&db)
/// .await?;
/// # Ok(())
/// # }
/// ```
pub fn array<I: IntoIterator>(iter: I) -> Array<I::IntoIter>
where
    I::Item: ToSql + StaticType,
{
    Array::new(iter.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::FromSql;

    fn decode_array(bytes: &[u8]) -> Vec<i32> {
        Vec::<i32>::from_sql(1007, bytes).unwrap()
    }

    #[test]
    fn encodes_like_vec() {
        let values = vec![1i32, 2, 3];
        let mut from_vec = BytesMut::new();
        values.encode(&mut from_vec).unwrap();

        let mut from_iter = BytesMut::new();
        array(values.iter()).encode(&mut from_iter).unwrap();

        assert_eq!(&from_vec[..], &from_iter[..]);
        assert_eq!(decode_array(&from_iter), values);
    }

    #[test]
    fn encodes_empty() {
        let empty: Vec<i32> = Vec::new();
        let mut buf = BytesMut::new();
        array(empty.iter()).encode(&mut buf).unwrap();
        assert_eq!(decode_array(&buf), Vec::<i32>::new());
        assert_eq!(buf.len(), 20);
    }

    #[test]
    fn second_encode_errors() {
        let values = vec![1i32];
        let adapter = array(values.iter());
        let mut buf = BytesMut::new();
        adapter.encode(&mut buf).unwrap();
        assert!(adapter.encode(&mut buf).is_err());
    }
}
