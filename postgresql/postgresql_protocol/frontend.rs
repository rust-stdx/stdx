//! Frontend (client → server) message encoding.
//!
//! Messages are appended to a caller-owned [`BytesMut`] so that a full request
//! (`Parse` + `Describe` + `Bind` + `Execute` + `Sync`) can be assembled in one
//! buffer and written with a single syscall.
//!
//! Counts and lengths taken from the caller are range-checked before being
//! written: a silently truncated length prefix makes the server reinterpret
//! the rest of the buffer as new protocol messages.

use bytes::{BufMut, BytesMut};

use crate::{error::Error, oid::Format};

/// Maximum number of values in one `Bind` or `COPY` row.
///
/// The protocol encodes these counts as a 16-bit field that PostgreSQL reads
/// as unsigned, so the limit is 65535 and not 32767.
pub const MAX_VALUE_COUNT: usize = u16::MAX as usize;

/// Appends a raw C string terminated by a NUL byte.
pub fn put_cstr(buf: &mut BytesMut, s: &str) {
    buf.put_slice(s.as_bytes());
    buf.put_u8(0);
}

/// Starts a tagged message, writing the tag and a placeholder length.
///
/// Returns the offset of the length field, to be passed to [`finish`].
pub fn begin(buf: &mut BytesMut, tag: u8) -> usize {
    buf.put_u8(tag);
    let pos = buf.len();
    buf.put_i32(0);
    pos
}

/// Patches the length field written by [`begin`].
///
/// # Errors
///
/// Returns [`Error::Encode`] when the message body is larger than 2 GiB and
/// cannot be expressed in the protocol's 32-bit length prefix. Silently
/// truncating would make the server reinterpret the rest of the buffer as new
/// messages.
pub fn finish(buf: &mut BytesMut, len_pos: usize) -> Result<(), Error> {
    let len = length32(buf.len() - len_pos)?;
    buf[len_pos..len_pos + 4].copy_from_slice(&len.to_be_bytes());
    Ok(())
}

/// Encodes the startup message (no tag).
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn startup(buf: &mut BytesMut, params: &[(&str, &str)]) -> Result<(), Error> {
    // Startup parameters are NUL-delimited on the wire: a NUL inside a key or
    // value would split it and inject extra parameters.
    for (key, value) in params {
        if key.contains('\0') || value.contains('\0') {
            return Err(Error::Encode("startup parameters must not contain a NUL byte".into()));
        }
    }
    let len_pos = buf.len();
    buf.put_i32(0);
    buf.put_i32(196608); // protocol 3.0
    for (k, v) in params {
        put_cstr(buf, k);
        put_cstr(buf, v);
    }
    buf.put_u8(0);
    finish(buf, len_pos)
}

/// Encodes an SSLRequest message.
pub fn ssl_request(buf: &mut BytesMut) {
    buf.put_i32(8);
    buf.put_i32(80877103);
}

/// Encodes a `CancelRequest` message (no tag; sent on a fresh connection).
pub fn cancel_request(buf: &mut BytesMut, process_id: i32, secret_key: i32) {
    buf.put_i32(16);
    buf.put_i32(80877102);
    buf.put_i32(process_id);
    buf.put_i32(secret_key);
}

/// Encodes a cleartext password message (`PasswordMessage`).
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn password(buf: &mut BytesMut, password: &str) -> Result<(), Error> {
    let len_pos = begin(buf, b'p');
    put_cstr(buf, password);
    finish(buf, len_pos)
}

/// Encodes a `SASLInitialResponse`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn sasl_initial_response(buf: &mut BytesMut, mechanism: &str, data: &[u8]) -> Result<(), Error> {
    let len_pos = begin(buf, b'p');
    put_cstr(buf, mechanism);
    buf.put_i32(length32(data.len())?);
    buf.put_slice(data);
    finish(buf, len_pos)
}

/// Encodes a `SASLResponse`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn sasl_response(buf: &mut BytesMut, data: &[u8]) -> Result<(), Error> {
    let len_pos = begin(buf, b'p');
    buf.put_slice(data);
    finish(buf, len_pos)
}

/// Encodes a simple `Query`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn query(buf: &mut BytesMut, sql: &str) -> Result<(), Error> {
    let len_pos = begin(buf, b'Q');
    put_cstr(buf, sql);
    finish(buf, len_pos)
}

/// Encodes a `Parse`.
///
/// # Errors
///
/// Returns [`Error::Encode`] when more than [`MAX_VALUE_COUNT`] parameter
/// types are supplied, or if the message is larger than 2 GiB.
pub fn parse(buf: &mut BytesMut, name: &str, sql: &str, param_oids: &[u32]) -> Result<(), Error> {
    let count = count16(param_oids.len())?;
    let len_pos = begin(buf, b'P');
    put_cstr(buf, name);
    put_cstr(buf, sql);
    buf.put_u16(count);
    for oid in param_oids {
        buf.put_u32(*oid);
    }
    finish(buf, len_pos)
}

/// Starts a `Bind` message; call [`write_param`] for each value, then
/// [`bind_finish`].
///
/// All parameters use the binary format, so one format code is written per
/// parameter. Returns the offset of the length field, to be passed to
/// [`bind_finish`].
///
/// # Errors
///
/// Returns [`Error::Encode`] when more than [`MAX_VALUE_COUNT`] parameters are
/// supplied.
pub fn bind_start(buf: &mut BytesMut, portal: &str, statement: &str, param_count: usize) -> Result<usize, Error> {
    let count = count16(param_count)?;
    let len_pos = begin(buf, b'B');
    put_cstr(buf, portal);
    put_cstr(buf, statement);
    buf.put_u16(count);
    for _ in 0..param_count {
        buf.put_i16(Format::Binary.code());
    }
    buf.put_u16(count);
    Ok(len_pos)
}

/// Writes one `Bind` parameter; `None` is encoded as SQL NULL.
///
/// # Errors
///
/// Returns [`Error::Encode`] when the value is larger than 2 GiB and cannot be
/// expressed in the protocol's 32-bit length prefix.
pub fn write_param(buf: &mut BytesMut, value: Option<&[u8]>) -> Result<(), Error> {
    match value {
        None => buf.put_i32(-1),
        Some(v) => {
            buf.put_i32(length32(v.len())?);
            buf.put_slice(v);
        }
    }
    Ok(())
}

/// Finishes a `Bind` message.
///
/// One result-column format code is written, which the server applies to every
/// result column.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn bind_finish(buf: &mut BytesMut, result_format: Format, len_pos: usize) -> Result<(), Error> {
    buf.put_i16(1);
    buf.put_i16(result_format.code());
    finish(buf, len_pos)
}

/// Encodes a `Describe` (`kind` is `b'S'` for a statement or `b'P'` for a portal).
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn describe(buf: &mut BytesMut, kind: u8, name: &str) -> Result<(), Error> {
    let len_pos = begin(buf, b'D');
    buf.put_u8(kind);
    put_cstr(buf, name);
    finish(buf, len_pos)
}

/// Encodes an `Execute`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn execute(buf: &mut BytesMut, portal: &str, max_rows: i32) -> Result<(), Error> {
    let len_pos = begin(buf, b'E');
    put_cstr(buf, portal);
    buf.put_i32(max_rows);
    finish(buf, len_pos)
}

/// Encodes a `Sync`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn sync(buf: &mut BytesMut) -> Result<(), Error> {
    let len_pos = begin(buf, b'S');
    finish(buf, len_pos)
}

/// Encodes a `Close` (`kind` is `b'S'` for a statement or `b'P'` for a portal).
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn close(buf: &mut BytesMut, kind: u8, name: &str) -> Result<(), Error> {
    let len_pos = begin(buf, b'C');
    buf.put_u8(kind);
    put_cstr(buf, name);
    finish(buf, len_pos)
}

/// Encodes a `Flush`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn flush(buf: &mut BytesMut) -> Result<(), Error> {
    let len_pos = begin(buf, b'H');
    finish(buf, len_pos)
}

/// Encodes a `Terminate`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn terminate(buf: &mut BytesMut) -> Result<(), Error> {
    let len_pos = begin(buf, b'X');
    finish(buf, len_pos)
}

/// Encodes a `CopyData`.
///
/// # Errors
///
/// Returns [`Error::Encode`] when the payload is larger than 2 GiB.
pub fn copy_data(buf: &mut BytesMut, data: &[u8]) -> Result<(), Error> {
    let len_pos = begin(buf, b'd');
    buf.put_slice(data);
    finish(buf, len_pos)
}

/// Encodes a `CopyDone`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn copy_done(buf: &mut BytesMut) -> Result<(), Error> {
    let len_pos = begin(buf, b'c');
    finish(buf, len_pos)
}

/// Encodes a `CopyFail`.
///
/// # Errors
///
/// Returns [`Error::Encode`] if the message is larger than 2 GiB.
pub fn copy_fail(buf: &mut BytesMut, message: &str) -> Result<(), Error> {
    let len_pos = begin(buf, b'f');
    put_cstr(buf, message);
    finish(buf, len_pos)
}

fn count16(count: usize) -> Result<u16, Error> {
    u16::try_from(count).map_err(|_| Error::Encode(format!("too many values: {count} (max {MAX_VALUE_COUNT})")))
}

fn length32(len: usize) -> Result<i32, Error> {
    i32::try_from(len).map_err(|_| Error::Encode(format!("value is too large: {len} bytes")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_layout() {
        let mut buf = BytesMut::new();
        startup(&mut buf, &[("user", "bob"), ("database", "db")]).unwrap();
        let len = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        assert_eq!(len as usize, buf.len());
        assert_eq!(&buf[4..8], &196608i32.to_be_bytes());
        // body ends with a NUL, keys are NUL-terminated
        assert_eq!(*buf.last().unwrap(), 0);
    }

    #[test]
    fn startup_rejects_nul_bytes() {
        // A NUL would split the parameter and inject extra startup values.
        let mut buf = BytesMut::new();
        assert!(startup(&mut buf, &[("application_name", "a\0user\0admin")]).is_err());
        assert!(startup(&mut buf, &[("us\0er", "x")]).is_err());
        assert!(buf.is_empty());
    }

    #[test]
    fn ssl_request_layout() {
        let mut buf = BytesMut::new();
        ssl_request(&mut buf);
        assert_eq!(buf.len(), 8);
        assert_eq!(&buf[0..4], &8i32.to_be_bytes());
        assert_eq!(&buf[4..8], &80877103i32.to_be_bytes());
    }

    #[test]
    fn bind_length_is_patched() {
        let mut buf = BytesMut::new();
        let pos = bind_start(&mut buf, "", "s1", 1).unwrap();
        write_param(&mut buf, Some(&[1, 2, 3, 4])).unwrap();
        bind_finish(&mut buf, Format::Binary, pos).unwrap();
        let len = i32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
        assert_eq!(len as usize, buf.len() - 1);
    }

    #[test]
    fn parse_encodes_oids() {
        let mut buf = BytesMut::new();
        parse(&mut buf, "", "SELECT $1", &[23]).unwrap();
        let len = i32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
        assert_eq!(len as usize, buf.len() - 1);
    }

    #[test]
    fn rejects_impossible_counts_and_lengths() {
        let mut buf = BytesMut::new();
        assert!(bind_start(&mut buf, "", "s", MAX_VALUE_COUNT + 1).is_err());
        assert!(parse(&mut buf, "", "SELECT $1", &vec![23u32; MAX_VALUE_COUNT + 1]).is_err());
        assert!(write_param(&mut buf, Some(&[0u8; 0])).is_ok());
        assert!(copy_data(&mut buf, &[1, 2, 3]).is_ok());
    }
}
