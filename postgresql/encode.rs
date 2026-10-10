//! Encoding Rust values into the PostgreSQL binary wire format.

use bytes::{BufMut, BytesMut};
use postgresql_protocol::oid::{
    BOOLOID, BYTEAOID, DATEOID, FLOAT4OID, FLOAT8OID, INETOID, INT2OID, INT4OID, INT8OID, INTERVALOID, JSONBOID, Oid,
    TEXTOID, TIMESTAMPOID, TIMESTAMPTZOID, UUIDOID, array_oid, array_oid_checked,
};

use crate::{error::Error, interval::Interval, json::Json};

/// Whether an encoded value is SQL `NULL`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsNull {
    /// The value is SQL `NULL`.
    Null,
    /// The value is not null.
    NotNull,
}

/// A value that can be bound as a query parameter in the binary format.
///
/// This trait is object-safe, so `&[&dyn ToSql]` is the parameter list the
/// macros build. See [`StaticType`] for the compile-time type OID.
pub trait ToSql: Send + Sync {
    /// Appends the binary representation of this value to `buf`, without a
    /// length prefix.
    fn encode(&self, buf: &mut BytesMut) -> Result<IsNull, Error>;

    /// The PostgreSQL type OID of this value.
    fn oid(&self) -> Oid;
}

/// A value with a statically known PostgreSQL type OID.
///
/// Used by the `query!` / `query_as!` macros to check parameter types at
/// compile time. This trait is deliberately separate from [`ToSql`] because an
/// associated constant would make `ToSql` not object-safe.
pub trait StaticType {
    /// The PostgreSQL type OID of this Rust type.
    const OID: Oid;
    /// Whether this Rust type can represent SQL `NULL` (i.e. it is `Option`).
    const NULLABLE: bool = false;
}

macro_rules! impl_pg_type {
    ($ty:ty, $oid:expr, |$this:ident, $buf:ident| $body:block) => {
        impl StaticType for $ty {
            const OID: Oid = $oid;
        }

        impl ToSql for $ty {
            fn encode(&self, buf: &mut BytesMut) -> Result<IsNull, Error> {
                let $this = self;
                let $buf = buf;
                $body
            }

            fn oid(&self) -> Oid {
                $oid
            }
        }
    };
}

/// Appends a complete one-dimensional PostgreSQL array.
pub(crate) fn encode_array<T: ToSql + StaticType>(
    iter: impl Iterator<Item = T>,
    element_oid: Oid,
    buf: &mut BytesMut,
) -> Result<IsNull, Error> {
    buf.put_i32(1); // number of dimensions
    let nulls_pos = buf.len();
    buf.put_i32(0); // has-nulls flag, patched below
    buf.put_u32(element_oid);
    let dim_pos = buf.len();
    buf.put_i32(0); // element count, patched below
    buf.put_i32(1); // lower bound

    let mut count = 0i32;
    let mut has_nulls = false;
    for item in iter {
        let len_pos = buf.len();
        buf.put_i32(0);
        let start = buf.len();
        match item.encode(buf)? {
            IsNull::Null => {
                buf.truncate(len_pos);
                buf.put_i32(-1);
                has_nulls = true;
            }
            IsNull::NotNull => {
                let n = i32::try_from(buf.len() - start)
                    .map_err(|_| Error::Encode("array element is larger than 2 GiB".into()))?;
                buf[len_pos..len_pos + 4].copy_from_slice(&n.to_be_bytes());
            }
        }
        count = count
            .checked_add(1)
            .ok_or_else(|| Error::Encode("array has too many elements".into()))?;
    }

    buf[nulls_pos..nulls_pos + 4].copy_from_slice(&(has_nulls as i32).to_be_bytes());
    buf[dim_pos..dim_pos + 4].copy_from_slice(&count.to_be_bytes());
    Ok(IsNull::NotNull)
}

/// Unix timestamp of the PostgreSQL epoch (2000-01-01T00:00:00Z).
pub(crate) const PG_EPOCH_UNIX: i64 = 946_684_800;

pub(crate) fn pg_epoch() -> Result<time::DateTime, Error> {
    Ok(time::DateTime::from_unix(PG_EPOCH_UNIX)?)
}

impl_pg_type!(i16, INT2OID, |this, buf| {
    buf.put_slice(&this.to_be_bytes());
    Ok(IsNull::NotNull)
});

impl_pg_type!(i32, INT4OID, |this, buf| {
    buf.put_slice(&this.to_be_bytes());
    Ok(IsNull::NotNull)
});

impl_pg_type!(i64, INT8OID, |this, buf| {
    buf.put_slice(&this.to_be_bytes());
    Ok(IsNull::NotNull)
});

impl_pg_type!(f32, FLOAT4OID, |this, buf| {
    buf.put_slice(&this.to_be_bytes());
    Ok(IsNull::NotNull)
});

impl_pg_type!(f64, FLOAT8OID, |this, buf| {
    buf.put_slice(&this.to_be_bytes());
    Ok(IsNull::NotNull)
});

impl_pg_type!(bool, BOOLOID, |this, buf| {
    buf.put_u8(u8::from(*this));
    Ok(IsNull::NotNull)
});

impl_pg_type!(str, TEXTOID, |this, buf| {
    buf.put_slice(this.as_bytes());
    Ok(IsNull::NotNull)
});

impl_pg_type!(String, TEXTOID, |this, buf| {
    buf.put_slice(this.as_bytes());
    Ok(IsNull::NotNull)
});

impl_pg_type!([u8], BYTEAOID, |this, buf| {
    buf.put_slice(this);
    Ok(IsNull::NotNull)
});

impl_pg_type!(Vec<u8>, BYTEAOID, |this, buf| {
    buf.put_slice(this);
    Ok(IsNull::NotNull)
});

impl_pg_type!(uuid::Uuid, UUIDOID, |this, buf| {
    buf.put_slice(&this.as_bytes()[..]);
    Ok(IsNull::NotNull)
});

impl_pg_type!(time::DateTime, TIMESTAMPTZOID, |this, buf| {
    let (secs, nanos) = this.signed_duration_since(pg_epoch()?);
    let micros = secs
        .checked_mul(1_000_000)
        .and_then(|v| v.checked_add((nanos / 1000) as i64))
        .ok_or_else(|| Error::Encode("timestamp is out of range for timestamptz".into()))?;
    buf.put_i64(micros);
    Ok(IsNull::NotNull)
});

/// Serializes any `serde` value as a `jsonb` payload.
///
/// Returns an error if the value cannot be serialized to JSON.
pub fn json_encode<T: serde::Serialize + ?Sized>(value: &T, buf: &mut BytesMut) -> Result<IsNull, Error> {
    buf.put_u8(1); // jsonb format version
    // Serialize straight into the destination buffer instead of through an
    // intermediate `Vec` (one fewer allocation and copy per parameter).
    serde_json::to_writer(JsonWriter(buf), value)?;
    Ok(IsNull::NotNull)
}

/// Adapts a `BytesMut` to [`std::io::Write`] so `serde_json` can serialize into
/// it directly.
///
/// The `bytes` crate is built here without its `std` feature, so `BytesMut` has
/// no `io::Write` implementation of its own.
struct JsonWriter<'a>(&'a mut BytesMut);

impl std::io::Write for JsonWriter<'_> {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl_pg_type!(serde_json::Value, JSONBOID, |this, buf| { json_encode(this, buf) });

impl<T: serde::Serialize + Send + Sync> ToSql for Json<T> {
    fn encode(&self, buf: &mut BytesMut) -> Result<IsNull, Error> {
        json_encode(&self.0, buf)
    }

    fn oid(&self) -> Oid {
        JSONBOID
    }
}

impl<T> StaticType for Json<T> {
    const OID: Oid = JSONBOID;
}

impl_pg_type!(ipnetwork::IpNetwork, INETOID, |this, buf| {
    match this {
        ipnetwork::IpNetwork::V4(net) => {
            buf.put_u8(2);
            buf.put_u8(net.prefix());
            buf.put_u8(0); // not a cidr
            buf.put_u8(4);
            buf.put_slice(&net.ip().octets());
        }
        ipnetwork::IpNetwork::V6(net) => {
            buf.put_u8(3);
            buf.put_u8(net.prefix());
            buf.put_u8(0);
            buf.put_u8(16);
            buf.put_slice(&net.ip().octets());
        }
    }
    Ok(IsNull::NotNull)
});

impl_pg_type!(Interval, INTERVALOID, |this, buf| {
    buf.put_i64(this.micros);
    buf.put_i32(this.days);
    buf.put_i32(this.months);
    Ok(IsNull::NotNull)
});

impl<T: ToSql + StaticType + ?Sized> ToSql for &T {
    fn encode(&self, buf: &mut BytesMut) -> Result<IsNull, Error> {
        (**self).encode(buf)
    }

    fn oid(&self) -> Oid {
        (**self).oid()
    }
}

impl<T: StaticType + ?Sized> StaticType for &T {
    const OID: Oid = T::OID;
    const NULLABLE: bool = T::NULLABLE;
}

impl<T: ToSql + StaticType> ToSql for Option<T> {
    fn encode(&self, buf: &mut BytesMut) -> Result<IsNull, Error> {
        match self {
            Some(value) => value.encode(buf),
            None => Ok(IsNull::Null),
        }
    }

    fn oid(&self) -> Oid {
        T::OID
    }
}

impl<T: StaticType> StaticType for Option<T> {
    const OID: Oid = T::OID;
    const NULLABLE: bool = true;
}

impl<T: ToSql + StaticType> ToSql for [T] {
    fn encode(&self, buf: &mut BytesMut) -> Result<IsNull, Error> {
        encode_array(self.iter(), T::OID, buf)
    }

    fn oid(&self) -> Oid {
        array_oid(T::OID).unwrap_or(0)
    }
}

impl<T: StaticType> StaticType for [T] {
    const OID: Oid = array_oid_checked(T::OID);
}

impl<T: ToSql + StaticType> ToSql for Vec<T> {
    fn encode(&self, buf: &mut BytesMut) -> Result<IsNull, Error> {
        encode_array(self.iter(), T::OID, buf)
    }

    fn oid(&self) -> Oid {
        array_oid(T::OID).unwrap_or(0)
    }
}

impl<T: StaticType> StaticType for Vec<T> {
    const OID: Oid = array_oid_checked(T::OID);
}

/// Encodes a `time::DateTime` as a zone-less `timestamp`.
#[derive(Debug, Clone, Copy)]
pub struct Timestamp(pub time::DateTime);

impl_pg_type!(Timestamp, TIMESTAMPOID, |this, buf| {
    let (secs, nanos) = this.0.signed_duration_since(pg_epoch()?);
    let micros = secs
        .checked_mul(1_000_000)
        .and_then(|v| v.checked_add((nanos / 1000) as i64))
        .ok_or_else(|| Error::Encode("timestamp is out of range".into()))?;
    buf.put_i64(micros);
    Ok(IsNull::NotNull)
});

/// Encodes a `time::DateTime` as a `date` (taken in UTC).
#[derive(Debug, Clone, Copy)]
pub struct Date(pub time::DateTime);

impl_pg_type!(Date, DATEOID, |this, buf| {
    let (secs, _) = this.0.signed_duration_since(pg_epoch()?);
    let days = secs.div_euclid(86_400);
    let days = i32::try_from(days).map_err(|_| Error::Encode("date is out of range".into()))?;
    buf.put_i32(days);
    Ok(IsNull::NotNull)
});
