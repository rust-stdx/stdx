//! Backend (server → client) message decoding.
//!
//! Row payloads are returned as zero-copy [`Bytes`] slices of the receive
//! buffer, so a `DataRow` never copies its column data.

use bytes::{Buf, Bytes, BytesMut};

use crate::{
    error::{DbError, Error},
    oid::Oid,
};

/// Description of one result column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescription {
    /// Column name.
    pub name: String,
    /// OID of the source table, or `0` for an expression.
    pub table_oid: u32,
    /// Attribute number within the source table, or `0`.
    pub column_attr: i16,
    /// Column type OID.
    pub type_oid: Oid,
    /// Type size in bytes, or negative for variable length.
    pub type_size: i16,
    /// Type modifier (e.g. numeric precision/scale), or `-1`.
    pub type_mod: i32,
    /// Wire format (`0` text, `1` binary).
    pub format: i16,
}

impl FieldDescription {
    /// `true` when this column originates from a real table column.
    pub fn is_table_column(&self) -> bool {
        self.table_oid != 0 && self.column_attr > 0
    }
}

/// A decoded backend message.
#[derive(Debug)]
pub enum BackendMessage {
    /// Authentication succeeded.
    AuthenticationOk,
    /// Server requests a cleartext password.
    AuthenticationCleartextPassword,
    /// Server requests an MD5 password, with a 4-byte salt.
    AuthenticationMd5Password([u8; 4]),
    /// Server offers SASL mechanisms.
    AuthenticationSasl(Vec<String>),
    /// SASL challenge.
    AuthenticationSaslContinue(Bytes),
    /// SASL completion.
    AuthenticationSaslFinal(Bytes),
    /// Backend process id and secret key, for query cancellation.
    BackendKeyData {
        /// Backend process ID.
        pid: i32,
        /// Cancellation secret key.
        secret_key: i32,
    },
    /// A run-time parameter (e.g. `server_version`, `integer_datetimes`).
    ParameterStatus {
        /// Parameter name.
        name: String,
        /// Parameter value.
        value: String,
    },
    /// The connection is ready for a new query.
    ReadyForQuery(u8),
    /// Description of the columns a statement or portal will return.
    RowDescription(Vec<FieldDescription>),
    /// The parameter types a prepared statement expects.
    ParameterDescription(Vec<Oid>),
    /// One result row; the payload starts with the column count.
    DataRow(Bytes),
    /// A statement completed, with its command tag.
    CommandComplete(Bytes),
    /// A `Parse` completed.
    ParseComplete,
    /// A `Bind` completed.
    BindComplete,
    /// A `Close` completed.
    CloseComplete,
    /// The statement/portal returns no rows.
    NoData,
    /// A portal was suspended by the row limit.
    PortalSuspended,
    /// An empty query string produced no command.
    EmptyQueryResponse,
    /// The server reported an error for the current request.
    ErrorResponse(DbError),
    /// A non-fatal notice, to be logged.
    NoticeResponse(DbError),
    /// An asynchronous `NOTIFY`.
    NotificationResponse {
        /// Notifying backend process ID.
        pid: i32,
        /// Channel name.
        channel: String,
        /// Notification payload.
        payload: String,
    },
    /// The server is ready to receive `COPY ... FROM STDIN` data.
    CopyInResponse {
        /// Overall format (`0` text, `1` binary).
        format: u8,
        /// Per-column formats.
        column_formats: Vec<i16>,
    },
    /// The server will stream `COPY ... TO STDOUT` data.
    CopyOutResponse {
        /// Overall format (`0` text, `1` binary).
        format: u8,
        /// Per-column formats.
        column_formats: Vec<i16>,
    },
    /// A chunk of `COPY` data.
    CopyData(Bytes),
    /// The server finished streaming `COPY` data.
    CopyDone,
    /// The server advertises protocol version negotiation.
    NegotiateProtocolVersion {
        /// Newest supported minor protocol version.
        newest_minor: i32,
        /// Unrecognized startup options.
        unsupported: Vec<String>,
    },
}

/// Default largest accepted length for a single backend message body, in
/// bytes (128 MiB).
///
/// A server (or a man in the middle, since `sslmode=require` does not
/// authenticate the server) can otherwise make the client buffer up to 2 GiB
/// while waiting for a message that will never arrive. The limit is checked
/// before anything is buffered, so a hostile length fails immediately.
///
/// This can be raised or lowered per connection with
/// [`decode_with_limit`]; see [`crate::error`] callers. 128 MiB is large enough
/// for any realistic row while keeping the worst-case buffering bounded.
pub const DEFAULT_MAX_MESSAGE_LEN: usize = 128 * 1024 * 1024;

/// Attempts to decode one backend message from `buf` with the default size
/// limit ([`DEFAULT_MAX_MESSAGE_LEN`]).
///
/// Returns `Ok(None)` when the buffer does not yet contain a full message.
///
/// # Errors
///
/// Returns [`Error::Decode`] when the message is malformed: an impossible
/// length, a truncated body, an unterminated string or an invalid count.
pub fn decode(buf: &mut BytesMut) -> Result<Option<BackendMessage>, Error> {
    decode_with_limit(buf, DEFAULT_MAX_MESSAGE_LEN)
}

/// Like [`decode`], but rejects any message whose declared body length exceeds
/// `max` bytes before anything is buffered.
///
/// `max` is clamped to at most `i32::MAX` (the largest length expressible on
/// the wire).
///
/// # Errors
///
/// Returns [`Error::Decode`] when the message is malformed or larger than
/// `max`.
pub fn decode_with_limit(buf: &mut BytesMut, max: usize) -> Result<Option<BackendMessage>, Error> {
    let max = max.min(i32::MAX as usize);
    if buf.len() < 5 {
        return Ok(None);
    }
    let tag = buf[0];
    let len = i32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
    if len < 4 {
        return Err(Error::Decode(format!("invalid message length {len}")));
    }
    // `len` is now known to be positive, so the conversion is exact.
    let len = usize::try_from(len).map_err(|_| Error::Decode(format!("invalid message length {len}")))?;
    if len > max {
        return Err(Error::Decode(format!("message length {len} exceeds the {max} byte limit")));
    }
    let total = 1 + len;
    if buf.len() < total {
        return Ok(None);
    }

    let frame = buf.split_to(total).freeze();
    let payload = frame.slice(5..);
    parse(tag, payload).map(Some)
}

fn parse(tag: u8, payload: Bytes) -> Result<BackendMessage, Error> {
    match tag {
        b'R' => {
            let mut r = Reader::new(payload);
            let kind = r.i32()?;
            match kind {
                0 => Ok(BackendMessage::AuthenticationOk),
                3 => Ok(BackendMessage::AuthenticationCleartextPassword),
                5 => {
                    let salt = r.bytes(4)?;
                    Ok(BackendMessage::AuthenticationMd5Password([salt[0], salt[1], salt[2], salt[3]]))
                }
                10 => {
                    // Each mechanism is at least one byte plus its NUL.
                    let mut mechanisms = Vec::with_capacity(r.cap(2, 2));
                    while r.remaining() > 0 {
                        let m = r.cstr()?;
                        if m.is_empty() {
                            break;
                        }
                        mechanisms.push(String::from_utf8_lossy(&m).into_owned());
                    }
                    Ok(BackendMessage::AuthenticationSasl(mechanisms))
                }
                11 => Ok(BackendMessage::AuthenticationSaslContinue(r.rest())),
                12 => Ok(BackendMessage::AuthenticationSaslFinal(r.rest())),
                other => Err(Error::Auth(format!("unsupported authentication method {other}"))),
            }
        }
        b'K' => {
            let mut r = Reader::new(payload);
            Ok(BackendMessage::BackendKeyData {
                pid: r.i32()?,
                secret_key: r.i32()?,
            })
        }
        b'S' => {
            let mut r = Reader::new(payload);
            Ok(BackendMessage::ParameterStatus {
                name: r.string()?,
                value: r.string()?,
            })
        }
        b'Z' => {
            let mut r = Reader::new(payload);
            Ok(BackendMessage::ReadyForQuery(r.u8()?))
        }
        b'T' => {
            let mut r = Reader::new(payload);
            let count = r.count16()?;
            // A field description is at least 19 bytes.
            let mut fields = Vec::with_capacity(r.cap(count, 19));
            for _ in 0..count {
                fields.push(FieldDescription {
                    name: r.string()?,
                    table_oid: r.u32()?,
                    column_attr: r.i16()?,
                    type_oid: r.u32()?,
                    type_size: r.i16()?,
                    type_mod: r.i32()?,
                    format: r.i16()?,
                });
            }
            Ok(BackendMessage::RowDescription(fields))
        }
        b't' => {
            let mut r = Reader::new(payload);
            let count = r.count16()?;
            let mut oids = Vec::with_capacity(r.cap(count, 4));
            for _ in 0..count {
                oids.push(r.u32()?);
            }
            Ok(BackendMessage::ParameterDescription(oids))
        }
        b'D' => Ok(BackendMessage::DataRow(payload)),
        b'C' => {
            let mut r = Reader::new(payload);
            Ok(BackendMessage::CommandComplete(r.cstr()?))
        }
        b'1' => Ok(BackendMessage::ParseComplete),
        b'2' => Ok(BackendMessage::BindComplete),
        b'3' => Ok(BackendMessage::CloseComplete),
        b'n' => Ok(BackendMessage::NoData),
        b's' => Ok(BackendMessage::PortalSuspended),
        b'I' => Ok(BackendMessage::EmptyQueryResponse),
        b'E' => {
            let err = decode_error(payload)?;
            Err(Error::Server(err))
        }
        b'N' => {
            let err = decode_error(payload)?;
            Ok(BackendMessage::NoticeResponse(err))
        }
        b'A' => {
            let mut r = Reader::new(payload);
            Ok(BackendMessage::NotificationResponse {
                pid: r.i32()?,
                channel: r.string()?,
                payload: r.string()?,
            })
        }
        b'G' => {
            let mut r = Reader::new(payload);
            let format = r.u8()?;
            let count = r.count16()?;
            let mut column_formats = Vec::with_capacity(r.cap(count, 2));
            for _ in 0..count {
                column_formats.push(r.i16()?);
            }
            Ok(BackendMessage::CopyInResponse {
                format,
                column_formats,
            })
        }
        b'W' => Err(Error::Protocol(
            "CopyBothResponse (streaming replication) is not supported".into(),
        )),
        b'H' => {
            let mut r = Reader::new(payload);
            let format = r.u8()?;
            let count = r.count16()?;
            let mut column_formats = Vec::with_capacity(r.cap(count, 2));
            for _ in 0..count {
                column_formats.push(r.i16()?);
            }
            Ok(BackendMessage::CopyOutResponse {
                format,
                column_formats,
            })
        }
        b'd' => Ok(BackendMessage::CopyData(payload)),
        b'c' => Ok(BackendMessage::CopyDone),
        b'v' => {
            let mut r = Reader::new(payload);
            let newest_minor = r.i32()?;
            let count = r.count32()?;
            let mut unsupported = Vec::with_capacity(r.cap(count, 1));
            for _ in 0..count {
                unsupported.push(r.string()?);
            }
            Ok(BackendMessage::NegotiateProtocolVersion {
                newest_minor,
                unsupported,
            })
        }
        other => Err(Error::Protocol(format!("unknown message tag {:?}", other as char))),
    }
}

fn decode_error(payload: Bytes) -> Result<DbError, Error> {
    let mut r = Reader::new(payload);
    let mut err = DbError::default();
    while r.remaining() > 0 {
        let field = r.u8()?;
        if field == 0 {
            break;
        }
        let value = r.string_lossy()?;
        match field {
            b'S' | b'V' => err.severity = value,
            b'C' => err.code = value,
            b'M' => err.message = value,
            b'D' => err.detail = Some(value),
            b'H' => err.hint = Some(value),
            b'P' => err.position = value.parse().ok(),
            b'n' => err.constraint = Some(value),
            b't' => err.table = Some(value),
            b'c' => err.column = Some(value),
            b's' => err.schema = Some(value),
            b'd' => err.data_type = Some(value),
            _ => {}
        }
    }
    Ok(err)
}

/// A checked reader over a message payload.
struct Reader {
    buf: Bytes,
}

impl Reader {
    fn new(buf: Bytes) -> Self {
        Reader {
            buf,
        }
    }

    fn remaining(&self) -> usize {
        self.buf.len()
    }

    fn rest(self) -> Bytes {
        self.buf
    }

    fn u8(&mut self) -> Result<u8, Error> {
        if self.buf.len() < 1 {
            return Err(Error::Decode("unexpected end of message".into()));
        }
        Ok(self.buf.get_u8())
    }

    fn i16(&mut self) -> Result<i16, Error> {
        if self.buf.len() < 2 {
            return Err(Error::Decode("unexpected end of message".into()));
        }
        Ok(self.buf.get_i16())
    }

    fn i32(&mut self) -> Result<i32, Error> {
        if self.buf.len() < 4 {
            return Err(Error::Decode("unexpected end of message".into()));
        }
        Ok(self.buf.get_i32())
    }

    fn u32(&mut self) -> Result<u32, Error> {
        Ok(self.i32()?.cast_unsigned())
    }

    /// Reads a 16-bit element count. Negative counts are rejected instead of
    /// being reinterpreted as huge `usize` values.
    fn count16(&mut self) -> Result<usize, Error> {
        let n = self.i16()?;
        usize::try_from(n).map_err(|_| Error::Decode(format!("invalid element count {n}")))
    }

    /// Reads a 32-bit element count. Negative counts are rejected instead of
    /// being reinterpreted as huge `usize` values.
    fn count32(&mut self) -> Result<usize, Error> {
        let n = self.i32()?;
        usize::try_from(n).map_err(|_| Error::Decode(format!("invalid element count {n}")))
    }

    /// Clamps a wire-supplied element count to what the remaining payload
    /// could possibly hold, so a hostile count cannot request a huge
    /// allocation up front.
    fn cap(&self, count: usize, min_bytes_per_item: usize) -> usize {
        count.min(self.buf.len() / min_bytes_per_item.max(1))
    }

    fn bytes(&mut self, n: usize) -> Result<Bytes, Error> {
        if self.buf.len() < n {
            return Err(Error::Decode("unexpected end of message".into()));
        }
        Ok(self.buf.split_to(n))
    }

    fn cstr(&mut self) -> Result<Bytes, Error> {
        match self.buf.iter().position(|&b| b == 0) {
            Some(pos) => {
                let s = self.buf.split_to(pos);
                self.buf.advance(1);
                Ok(s)
            }
            None => Err(Error::Decode("unterminated string".into())),
        }
    }

    fn string(&mut self) -> Result<String, Error> {
        let bytes = self.cstr()?;
        String::from_utf8(bytes.to_vec()).map_err(|e| Error::Decode(format!("invalid UTF-8: {e}")))
    }

    /// Like [`Reader::string`], but replaces invalid UTF-8 instead of erroring.
    ///
    /// Used for `ErrorResponse` / `NoticeResponse` fields: a server that
    /// mis-encodes (or a hostile one) must not be able to hide its actual error
    /// behind a decode failure in a field the client only reports.
    fn string_lossy(&mut self) -> Result<String, Error> {
        let bytes = self.cstr()?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(tag: u8, payload: &[u8]) -> BytesMut {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&[tag]);
        buf.extend_from_slice(&((payload.len() + 4) as i32).to_be_bytes());
        buf.extend_from_slice(payload);
        buf
    }

    #[test]
    fn incomplete_returns_none() {
        let mut buf = BytesMut::from(&b"R"[..]);
        assert!(decode(&mut buf).unwrap().is_none());
    }

    #[test]
    fn decode_ready_for_query() {
        let mut buf = message(b'Z', &[b'I']);
        match decode(&mut buf).unwrap().unwrap() {
            BackendMessage::ReadyForQuery(s) => assert_eq!(s, b'I'),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn decode_error_response() {
        // S=ERROR\0 C=23505\0 M=dup\0 n=users_pkey\0 \0
        let mut payload = Vec::new();
        payload.extend_from_slice(b"SERROR\0");
        payload.extend_from_slice(b"C23505\0");
        payload.extend_from_slice(b"Mduplicate key\0");
        payload.extend_from_slice(b"nusers_pkey\0");
        payload.push(0);
        let mut buf = message(b'E', &payload);
        match decode(&mut buf).unwrap_err() {
            Error::Server(err) => {
                assert_eq!(err.code, "23505");
                assert!(err.is_unique_violation());
                assert_eq!(err.constraint.as_deref(), Some("users_pkey"));
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn decode_row_description() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&1i16.to_be_bytes());
        payload.extend_from_slice(b"id\0");
        payload.extend_from_slice(&0u32.to_be_bytes());
        payload.extend_from_slice(&0i16.to_be_bytes());
        payload.extend_from_slice(&23u32.to_be_bytes());
        payload.extend_from_slice(&4i16.to_be_bytes());
        payload.extend_from_slice(&(-1i32).to_be_bytes());
        payload.extend_from_slice(&1i16.to_be_bytes());
        let mut buf = message(b'T', &payload);
        match decode(&mut buf).unwrap().unwrap() {
            BackendMessage::RowDescription(fields) => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].name, "id");
                assert_eq!(fields[0].type_oid, 23);
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn decode_parameter_description() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&2i16.to_be_bytes());
        payload.extend_from_slice(&23u32.to_be_bytes());
        payload.extend_from_slice(&25u32.to_be_bytes());
        let mut buf = message(b't', &payload);
        match decode(&mut buf).unwrap().unwrap() {
            BackendMessage::ParameterDescription(oids) => assert_eq!(oids, vec![23, 25]),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn negative_counts_are_rejected_not_panicking() {
        for tag in [b'T', b't'] {
            let mut p = Vec::new();
            p.extend_from_slice(&(-1i16).to_be_bytes());
            let mut buf = message(tag, &p);
            let err = decode(&mut buf).unwrap_err();
            assert!(matches!(err, Error::Decode(_)), "tag {}: {err:?}", tag as char);
        }

        // CopyIn/CopyOut responses.
        let mut p = Vec::new();
        p.push(1u8);
        p.extend_from_slice(&(-3i16).to_be_bytes());
        for tag in [b'G', b'W', b'H'] {
            let mut buf = message(tag, &p);
            assert!(decode(&mut buf).is_err());
        }

        // NegotiateProtocolVersion.
        let mut p = Vec::new();
        p.extend_from_slice(&3i32.to_be_bytes());
        p.extend_from_slice(&(-1i32).to_be_bytes());
        let mut buf = message(b'v', &p);
        assert!(decode(&mut buf).is_err());
    }

    #[test]
    fn huge_counts_do_not_allocate_the_payload_size() {
        // 32767 claimed columns in a 2-byte payload: the decoder must not ask
        // for a 32767-element allocation up front, and must fail cleanly.
        let mut p = Vec::new();
        p.extend_from_slice(&32767i16.to_be_bytes());
        let mut buf = message(b'T', &p);
        assert!(decode(&mut buf).is_err());

        let mut p = Vec::new();
        p.extend_from_slice(&1i32.to_be_bytes());
        p.extend_from_slice(&i32::MAX.to_be_bytes());
        let mut buf = message(b'v', &p);
        assert!(decode(&mut buf).is_err());
    }

    #[test]
    fn oversized_message_length_is_rejected() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&[b'D']);
        buf.extend_from_slice(&i32::MAX.to_be_bytes());
        match decode(&mut buf) {
            Err(Error::Decode(msg)) => assert!(msg.contains("exceeds"), "{msg}"),
            other => panic!("unexpected {other:?}"),
        }
        // The declared length never has to be buffered before it is checked.
        assert_eq!(buf.len(), 5);
    }

    #[test]
    fn custom_limit_is_honoured() {
        // A 1 KiB body is accepted by default but rejected with a small limit.
        let payload = vec![0u8; 1024];
        let mut buf = message(b'D', &payload);
        assert!(decode(&mut buf).unwrap().is_some());

        let mut buf = message(b'D', &payload);
        match decode_with_limit(&mut buf, 100) {
            Err(Error::Decode(msg)) => assert!(msg.contains("exceeds"), "{msg}"),
            other => panic!("unexpected {other:?}"),
        }
        // The over-limit message is rejected before it is split off.
        assert_eq!(buf.len(), 1 + 4 + payload.len());

        // A limit at least as large as the message accepts it.
        let mut buf = message(b'Z', &[b'I']);
        assert!(decode_with_limit(&mut buf, 5).unwrap().is_some());

        // A zero limit rejects every message rather than silently accepting.
        let mut buf = message(b'Z', &[b'I']);
        assert!(decode_with_limit(&mut buf, 0).is_err());
    }

    #[test]
    fn truncated_payloads_error_instead_of_panicking() {
        // Unterminated C strings.
        let mut buf = message(b'S', b"name-without-nul");
        assert!(decode(&mut buf).is_err());
        let mut buf = message(b'C', b"INSERT 0 1");
        assert!(decode(&mut buf).is_err());
        // Not enough bytes for the fixed part.
        let mut buf = message(b'K', &[0, 1]);
        assert!(decode(&mut buf).is_err());
        let mut buf = message(b'Z', &[]);
        assert!(decode(&mut buf).is_err());
        let mut buf = message(b'A', &[0, 0, 0, 1]);
        assert!(decode(&mut buf).is_err());
        // Row description with a field count but no fields.
        let mut p = Vec::new();
        p.extend_from_slice(&1i16.to_be_bytes());
        let mut buf = message(b'T', &p);
        assert!(decode(&mut buf).is_err());
    }

    #[test]
    fn error_fields_may_be_non_utf8_or_non_ascii() {
        // `class()` on the decoded error must not panic on such a code.
        let mut payload = Vec::new();
        payload.extend_from_slice(b"SERROR\0");
        payload.extend_from_slice("C\u{20ac}\0".as_bytes());
        payload.extend_from_slice(b"Mweird\0");
        payload.push(0);
        let mut buf = message(b'E', &payload);
        match decode(&mut buf).unwrap_err() {
            Error::Server(err) => {
                assert_eq!(err.class(), "");
                assert_eq!(err.message, "weird");
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn invalid_utf8_in_identifiers_is_an_error() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&1i16.to_be_bytes());
        payload.extend_from_slice(&[0xff, 0xfe, 0]);
        let mut buf = message(b'T', &payload);
        assert!(decode(&mut buf).is_err());
    }

    #[test]
    fn invalid_utf8_in_error_fields_is_lossy_not_a_decode_error() {
        // A mis-encoded (or hostile) server must not hide its actual error
        // behind a decode failure: the field is reported with replacement
        // characters instead.
        let mut payload = Vec::new();
        payload.extend_from_slice(b"SERROR\0");
        payload.extend_from_slice(b"C42601\0");
        payload.extend_from_slice(&[b'M', 0xff, 0xfe, b'h', 0x00]);
        payload.push(0);
        let mut buf = message(b'E', &payload);
        match decode(&mut buf).unwrap_err() {
            Error::Server(err) => {
                assert_eq!(err.code, "42601");
                assert!(err.message.contains('h'), "message was {:?}", err.message);
                assert!(err.message.contains('\u{fffd}'), "message was {:?}", err.message);
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn copy_both_response_is_rejected() {
        // 'W' is CopyBothResponse (streaming replication), which this client
        // does not support; it must not be mistaken for CopyInResponse.
        let mut payload = Vec::new();
        payload.push(1u8);
        payload.extend_from_slice(&1i16.to_be_bytes());
        payload.extend_from_slice(&1i16.to_be_bytes());
        let mut buf = message(b'W', &payload);
        assert!(matches!(decode(&mut buf), Err(Error::Protocol(_))));
    }

    #[test]
    fn data_row_is_zero_copy_slice_of_frame() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&1i16.to_be_bytes());
        payload.extend_from_slice(&3i32.to_be_bytes());
        payload.extend_from_slice(&[7, 8, 9]);
        let mut buf = message(b'D', &payload);
        match decode(&mut buf).unwrap().unwrap() {
            BackendMessage::DataRow(bytes) => {
                assert_eq!(bytes.len(), payload.len());
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
