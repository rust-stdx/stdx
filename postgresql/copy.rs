//! Typed binary `COPY ... FROM STDIN` / `TO STDOUT`.
//!
//! Rows are written and read positionally, in the order of the `COPY` column
//! list. [`ToRow`] is implemented for tuples and — via
//! `#[derive(ToRow)]` — for structs, whose fields are encoded in declaration
//! order. [`FromCopyRow`] is implemented for scalars and tuples; a struct is
//! read field by field with [`FromCopyRow::from_copy_row`] or by decoding into
//! a tuple first.

use std::marker::PhantomData;

use bytes::{Buf, Bytes, BytesMut};
use futures_util::StreamExt;
use postgresql_protocol::oid::Oid;

use crate::{
    connection::{COPY_HEADER, CopyInWriter, CopyOutStream, encode_copy_row},
    decode::FromSql,
    encode::ToSql,
    error::Error,
};

/// A value that can be written as one binary `COPY` row.
pub trait ToRow {
    /// Appends the row (field count and framed fields) to `buf`.
    fn encode_row(&self, buf: &mut BytesMut) -> Result<(), Error>;
}

impl<T: ToSql> ToRow for T {
    fn encode_row(&self, buf: &mut BytesMut) -> Result<(), Error> {
        encode_copy_row(&[self], buf)
    }
}

macro_rules! to_row_tuple {
    ($($idx:tt $name:ident),+) => {
        impl<$($name: ToSql),+> ToRow for ($($name,)+) {
            fn encode_row(&self, buf: &mut BytesMut) -> Result<(), Error> {
                encode_copy_row(&[$( &self.$idx as &dyn ToSql ),+], buf)
            }
        }
    };
}

to_row_tuple!(0 A);
to_row_tuple!(0 A, 1 B);
to_row_tuple!(0 A, 1 B, 2 C);
to_row_tuple!(0 A, 1 B, 2 C, 3 D);
to_row_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
to_row_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
to_row_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
to_row_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);

/// A value that can be decoded from one binary `COPY` row.
pub trait FromCopyRow: Sized {
    /// Decodes a row from its fields and their column OIDs.
    fn from_copy_row(oids: &[Oid], fields: &[Option<Bytes>]) -> Result<Self, Error>;
}

fn decode_field<T: FromSql>(oids: &[Oid], fields: &[Option<Bytes>], index: usize) -> Result<T, Error> {
    let oid = oids.get(index).copied().unwrap_or(0);
    if oid != 0 && !T::accepts(oid) {
        return Err(Error::Decode(format!(
            "COPY column {index} has type OID {oid} which cannot be decoded as the requested type"
        )));
    }
    match fields.get(index) {
        None => Err(Error::Decode(format!("COPY row is missing field {index}"))),
        Some(None) => T::from_sql_null(),
        Some(Some(bytes)) => T::from_sql(oid, bytes),
    }
}

impl<T: FromSql> FromCopyRow for T {
    fn from_copy_row(oids: &[Oid], fields: &[Option<Bytes>]) -> Result<Self, Error> {
        decode_field(oids, fields, 0)
    }
}

macro_rules! from_copy_row_tuple {
    ($($idx:tt $name:ident),+) => {
        impl<$($name: FromSql),+> FromCopyRow for ($($name,)+) {
            fn from_copy_row(oids: &[Oid], fields: &[Option<Bytes>]) -> Result<Self, Error> {
                Ok(($( decode_field::<$name>(oids, fields, $idx)?, )+))
            }
        }
    };
}

from_copy_row_tuple!(0 A);
from_copy_row_tuple!(0 A, 1 B);
from_copy_row_tuple!(0 A, 1 B, 2 C);
from_copy_row_tuple!(0 A, 1 B, 2 C, 3 D);
from_copy_row_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
from_copy_row_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
from_copy_row_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
from_copy_row_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);

/// A writer for a binary `COPY ... FROM STDIN` decoded from `T`.
pub struct CopyIn<T> {
    inner: CopyInWriter,
    _marker: PhantomData<fn() -> T>,
}

impl<T: ToRow> CopyIn<T> {
    pub(crate) fn new(inner: CopyInWriter) -> Self {
        CopyIn {
            inner,
            _marker: PhantomData,
        }
    }

    /// Writes one row.
    ///
    /// Rows are encoded into a reusable buffer and coalesced into larger
    /// `CopyData` messages, so a bulk load does not cost one allocation and one
    /// network write per row.
    ///
    /// # Errors
    ///
    /// Returns an error if encoding fails or the connection is closed.
    pub async fn write_row(&self, row: &T) -> Result<(), Error> {
        self.inner.append(|buf| row.encode_row(buf)).await
    }

    /// Completes the COPY and returns the number of rows written.
    ///
    /// # Errors
    ///
    /// Returns an error if the server rejects the COPY.
    pub async fn finish(self) -> Result<u64, Error> {
        self.inner.finish().await
    }
}

/// A reader for a binary `COPY ... TO STDOUT` decoded into `T`.
pub struct CopyOut<T> {
    inner: CopyOutStream,
    oids: Vec<Oid>,
    /// Unconsumed bytes. Chunks are appended (amortized O(1)); a completed row
    /// is split off with `split_to`, so a row straddling many `CopyData`
    /// messages is never re-copied.
    buf: BytesMut,
    /// Largest row this reader will buffer, in bytes, before erroring.
    max_row_len: usize,
    header_skipped: bool,
    trailer_seen: bool,
    done: bool,
    _marker: PhantomData<fn() -> T>,
}

impl<T: FromCopyRow> CopyOut<T> {
    pub(crate) fn new(inner: CopyOutStream, oids: Vec<Oid>, max_row_len: usize) -> Self {
        CopyOut {
            inner,
            oids,
            buf: BytesMut::new(),
            max_row_len,
            header_skipped: false,
            trailer_seen: false,
            done: false,
            _marker: PhantomData,
        }
    }

    /// Reads the next row, or `None` at the end of the COPY stream.
    ///
    /// # Errors
    ///
    /// Returns a decode error for a malformed or truncated stream (a stream
    /// that ends without the binary `COPY` trailer is an error, not a clean
    /// end), a row larger than the configured limit, a server error, or an I/O
    /// error.
    pub async fn next(&mut self) -> Result<Option<T>, Error> {
        loop {
            if let Some(row) = self.try_parse()? {
                return Ok(Some(row));
            }
            if self.done {
                if !self.buf.is_empty() {
                    return Err(Error::Decode(format!(
                        "binary COPY stream has {} trailing bytes after its trailer",
                        self.buf.len()
                    )));
                }
                return Ok(None);
            }
            match self.inner.next().await {
                Some(Ok(chunk)) => {
                    // A single row is bounded by `max_row_len` inside
                    // `try_parse` (which checks the cumulative row offset), and
                    // each `CopyData` message is already bounded by the
                    // connection's message limit, so appending here cannot grow
                    // the buffer without bound.
                    self.buf.extend_from_slice(&chunk);
                }
                Some(Err(err)) => {
                    self.done = true;
                    return Err(err);
                }
                None => {
                    self.done = true;
                    if !self.trailer_seen {
                        return Err(Error::Decode(
                            "binary COPY stream ended before its trailer (truncated or malformed)".into(),
                        ));
                    }
                    if !self.buf.is_empty() {
                        return Err(Error::Decode(format!(
                            "binary COPY stream has {} trailing bytes after its trailer",
                            self.buf.len()
                        )));
                    }
                }
            }
        }
    }

    /// Reads every remaining row.
    ///
    /// # Errors
    ///
    /// Returns a decode, server or I/O error.
    pub async fn collect_all(mut self) -> Result<Vec<T>, Error> {
        let mut out = Vec::new();
        while let Some(row) = self.next().await? {
            out.push(row);
        }
        Ok(out)
    }

    /// Parses one row if a complete one is buffered, without copying its
    /// fields.
    fn try_parse(&mut self) -> Result<Option<T>, Error> {
        if !self.header_skipped {
            if self.buf.len() < COPY_HEADER.len() {
                return Ok(None);
            }
            if &self.buf[..COPY_HEADER.len()] != COPY_HEADER {
                return Err(Error::Decode("not a binary COPY stream (bad file header)".into()));
            }
            self.buf.advance(COPY_HEADER.len());
            self.header_skipped = true;
        }
        if self.buf.len() < 2 {
            return Ok(None);
        }
        let raw_count = i16::from_be_bytes([self.buf[0], self.buf[1]]);
        if raw_count == -1 {
            // The end-of-stream trailer.
            self.buf.advance(2);
            self.trailer_seen = true;
            self.done = true;
            return Ok(None);
        }
        let count = usize::try_from(raw_count)
            .map_err(|_| Error::Decode(format!("COPY row has an invalid field count {raw_count}")))?;

        // First pass: find the end of the row, waiting for more data when the
        // row is incomplete. Nothing is consumed until the whole row is here.
        let mut pos = 2usize;
        for _ in 0..count {
            if self.buf.len() < pos + 4 {
                return Ok(None);
            }
            let raw_len = i32::from_be_bytes([self.buf[pos], self.buf[pos + 1], self.buf[pos + 2], self.buf[pos + 3]]);
            pos += 4;
            if raw_len == -1 {
                continue;
            }
            let len = usize::try_from(raw_len)
                .map_err(|_| Error::Decode(format!("COPY field has an invalid length {raw_len}")))?;
            let end = pos
                .checked_add(len)
                .ok_or_else(|| Error::Decode("binary COPY field length overflows the address space".into()))?;
            // Enforce the row limit whether or not the bytes have arrived yet:
            // a fully buffered row must obey it too, not only one being waited
            // for.
            if end > self.max_row_len {
                return Err(Error::Decode(format!(
                    "binary COPY field of {len} bytes exceeds the {} byte row limit",
                    self.max_row_len
                )));
            }
            if self.buf.len() < end {
                return Ok(None);
            }
            pos = end;
        }

        // Second pass: hand out `Bytes` slices of the row, sharing one
        // allocation instead of copying every field.
        let row = self.buf.split_to(pos).freeze();
        let mut fields: Vec<Option<Bytes>> = Vec::with_capacity(count);
        let mut p = 2usize;
        for _ in 0..count {
            let raw_len = i32::from_be_bytes([row[p], row[p + 1], row[p + 2], row[p + 3]]);
            p += 4;
            if raw_len == -1 {
                fields.push(None);
            } else {
                let len = usize::try_from(raw_len)
                    .map_err(|_| Error::Decode(format!("COPY field has an invalid length {raw_len}")))?;
                fields.push(Some(row.slice(p..p + len)));
                p += len;
            }
        }
        Ok(Some(T::from_copy_row(&self.oids, &fields)?))
    }
}

impl<T> std::fmt::Debug for CopyOut<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CopyOut").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use super::*;
    use crate::connection::COPY_HEADER;

    type Row = (i32, Option<String>);

    fn stream() -> (mpsc::Sender<Result<Bytes, Error>>, CopyOut<Row>) {
        // Large enough that the tests can push the whole stream before reading.
        let (tx, rx) = mpsc::channel(256);
        (
            tx,
            CopyOut::new(
                CopyOutStream {
                    rx,
                },
                vec![23, 25],
                128 * 1024 * 1024,
            ),
        )
    }

    fn row(fields: &[Option<&[u8]>]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(fields.len() as i16).to_be_bytes());
        for field in fields {
            match field {
                None => out.extend_from_slice(&(-1i32).to_be_bytes()),
                Some(bytes) => {
                    out.extend_from_slice(&(bytes.len() as i32).to_be_bytes());
                    out.extend_from_slice(bytes);
                }
            }
        }
        out
    }

    fn header() -> Vec<u8> {
        COPY_HEADER.to_vec()
    }

    #[tokio::test]
    async fn reads_rows_and_nulls_across_chunks() {
        let (tx, mut out) = stream();
        let mut data = header();
        data.extend_from_slice(&row(&[Some(&1i32.to_be_bytes()), Some(b"a")]));
        data.extend_from_slice(&row(&[Some(&2i32.to_be_bytes()), None]));
        data.extend_from_slice(&(-1i16).to_be_bytes()); // trailer
        // Feed it one byte at a time: the parser must wait, not truncate.
        for byte in data {
            tx.send(Ok(Bytes::from(vec![byte]))).await.unwrap();
        }
        drop(tx);
        assert_eq!(out.next().await.unwrap().unwrap(), (1, Some("a".to_string())));
        assert_eq!(out.next().await.unwrap().unwrap(), (2, None));
        assert!(out.next().await.unwrap().is_none());
    }

    #[tokio::test]
    async fn truncated_stream_is_an_error_not_a_clean_end() {
        let (tx, mut out) = stream();
        let mut data = header();
        let mut partial = row(&[Some(&1i32.to_be_bytes()), Some(b"abcdefgh")]);
        partial.truncate(partial.len() - 4);
        data.extend_from_slice(&partial);
        tx.send(Ok(Bytes::from(data))).await.unwrap();
        drop(tx); // stream ends without the -1 trailer
        let err = out.next().await.unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "{err}");
    }

    #[tokio::test]
    async fn trailing_garbage_after_the_trailer_is_an_error() {
        let (tx, mut out) = stream();
        let mut data = header();
        data.extend_from_slice(&(-1i16).to_be_bytes());
        data.extend_from_slice(&[0xde, 0xad]);
        tx.send(Ok(Bytes::from(data))).await.unwrap();
        drop(tx);
        let err = out.next().await.unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "{err}");
    }

    #[tokio::test]
    async fn bad_file_header_is_an_error() {
        let (tx, mut out) = stream();
        tx.send(Ok(Bytes::from(b"NOTACOPYFILE\n\xff\r\n\0\0\0\0\0\0\0\0\0".to_vec())))
            .await
            .unwrap();
        drop(tx);
        let err = out.next().await.unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "{err}");
    }

    #[tokio::test]
    async fn invalid_field_counts_and_lengths_are_errors() {
        for payload in [
            // field count -2 (only -1 is the trailer)
            {
                let mut d = header();
                d.extend_from_slice(&(-2i16).to_be_bytes());
                d
            },
            // field length -2 (only -1 is NULL)
            {
                let mut d = header();
                d.extend_from_slice(&row(&[Some(&1i32.to_be_bytes())]));
                d.truncate(d.len() - 4);
                d.extend_from_slice(&(-2i32).to_be_bytes());
                d
            },
        ] {
            let (tx, mut out) = stream();
            tx.send(Ok(Bytes::from(payload))).await.unwrap();
            drop(tx);
            assert!(out.next().await.is_err());
        }
    }

    #[tokio::test]
    async fn a_fully_buffered_row_over_the_limit_is_rejected() {
        // The row limit must be enforced even when the whole row is already
        // buffered, not only while waiting for bytes.
        let (tx, rx) = mpsc::channel(256);
        let mut out = CopyOut::<Row>::new(
            CopyOutStream {
                rx,
            },
            vec![23, 25],
            8,
        );
        let mut data = header();
        data.extend_from_slice(&row(&[Some(&1i32.to_be_bytes()), Some(b"way-too-long")]));
        tx.send(Ok(Bytes::from(data))).await.unwrap();
        drop(tx);
        let err = out.next().await.unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "{err}");
    }

    #[tokio::test]
    async fn fields_borrow_from_the_same_buffer() {
        let (tx, mut out) = stream();
        let mut data = header();
        data.extend_from_slice(&row(&[Some(&7i32.to_be_bytes()), Some(b"xyz")]));
        data.extend_from_slice(&(-1i16).to_be_bytes());
        tx.send(Ok(Bytes::from(data))).await.unwrap();
        drop(tx);
        let row = out.next().await.unwrap().unwrap();
        assert_eq!(row.0, 7);
        assert_eq!(row.1.as_deref(), Some("xyz"));
        assert!(out.next().await.unwrap().is_none());
    }
}
