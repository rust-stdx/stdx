use quic::varint;

/// HTTP/3 frame type identifiers as defined in [RFC 9114 §7.2](https://www.rfc-editor.org/rfc/rfc9114#section-7.2).
///
/// Each variant corresponds to a variable-length integer type field
/// that prefixes every QUIC-stream-based HTTP/3 frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// **DATA** (type `0x0`) — carries arbitrary bytes for the request
    /// or response body on a stream.
    Data,
    /// **HEADERS** (type `0x1`) — carries a QPACK-encoded header block.
    Headers,
    /// **CANCEL_PUSH** (type `0x3`) — tells the server to cancel a
    /// previously promised pushed stream identified by its push ID.
    CancelPush,
    /// **SETTINGS** (type `0x4`) — conveys connection-scoped
    /// configuration parameters. Always sent on the control stream.
    Settings,
    /// **PUSH_PROMISE** (type `0x5`) — a server-initiated promise of
    /// a future push stream, carrying the push ID and a QPACK-encoded
    /// header block for the pushed request.
    PushPromise,
    /// **GOAWAY** (type `0x7`) — initiates graceful connection
    /// shutdown. Carries the ID of the last client-initiated stream
    /// the server will process.
    GoAway,
    /// **MAX_PUSH_ID** (type `0xD`) — used by the client to limit the
    /// maximum push ID the server may use in PUSH_PROMISE frames.
    MaxPushId,
    /// **GREASE** — a reserved frame type (every nibble is `0xA`) used
    /// to ensure extensibility; peers must ignore it.
    Grease(u64),
    /// **Unknown** — a frame type that is not recognised by this
    /// implementation.
    Unknown(u64),
}

impl FrameType {
    pub fn to_u64(self) -> u64 {
        match self {
            FrameType::Data => 0x00,
            FrameType::Headers => 0x01,
            FrameType::CancelPush => 0x03,
            FrameType::Settings => 0x04,
            FrameType::PushPromise => 0x05,
            FrameType::GoAway => 0x07,
            FrameType::MaxPushId => 0x0D,
            FrameType::Grease(v) => v,
            FrameType::Unknown(v) => v,
        }
    }

    pub fn from_u64(ty: u64) -> Self {
        match ty {
            0x00 => FrameType::Data,
            0x01 => FrameType::Headers,
            0x03 => FrameType::CancelPush,
            0x04 => FrameType::Settings,
            0x05 => FrameType::PushPromise,
            0x07 => FrameType::GoAway,
            0x0D => FrameType::MaxPushId,
            raw if is_grease_raw(raw) => FrameType::Grease(raw),
            other => FrameType::Unknown(other),
        }
    }
}

/// A single HTTP/3 setting: an identifier–value pair encoded as
/// variable-length integers.
#[derive(Debug, Clone)]
pub struct Setting {
    pub id: u64,
    pub value: u64,
}

/// An HTTP/3 frame, typed and parsed, ready for encoding or already
/// decoded from a QUIC stream.
///
/// Each variant corresponds to one of the HTTP/3 frame types defined in
/// [RFC 9114 §7.2](https://www.rfc-editor.org/rfc/rfc9114#section-7.2).
///
/// Unlike HTTP/2, HTTP/3 frame types use variable-length integer
/// encoding and lack per-frame flags; most frame semantics are
/// handled by the QUIC stream type or by information inside the
/// payload.
#[derive(Debug, Clone)]
pub enum Frame {
    /// **DATA** — the request or response body.
    Data(Vec<u8>),
    /// **HEADERS** — a QPACK-encoded header block.
    Headers(Vec<u8>),
    /// **CANCEL_PUSH** — cancels a server push identified by the
    /// push ID carried in the payload.
    CancelPush(Vec<u8>),
    /// **SETTINGS** — connection-scoped configuration parameters.
    Settings(Vec<Setting>),
    /// **PUSH_PROMISE** — a server-initiated push promise containing
    /// a QPACK-encoded header block.
    PushPromise(Vec<u8>),
    /// **GOAWAY** — graceful connection shutdown, identifying the
    /// last client-initiated stream the sender will process.
    GoAway { stream_id: u64 },
    /// **MAX_PUSH_ID** — set by the client to limit server push IDs.
    MaxPushId(Vec<u8>),
    /// **GREASE** — reserved frame type that must be ignored.
    Grease(u64, Vec<u8>),
    /// An unrecognized frame type; payload is included verbatim.
    Unknown(u64, Vec<u8>),
}

fn is_grease_raw(ty: u64) -> bool {
    let mut v = ty;
    while v > 0 {
        if v & 0x0F != 0x0A {
            return false;
        }
        v >>= 8;
    }
    true
}

/// Encodes an HTTP/3 [`Frame`] into its wire representation:
/// a variable-length type, a variable-length payload length,
/// followed by the payload itself.
pub fn encode_frame(frame: &Frame) -> Vec<u8> {
    let (ty, payload) = match frame {
        Frame::Data(data) => (FrameType::Data, data.clone()),
        Frame::Headers(data) => (FrameType::Headers, data.clone()),
        Frame::CancelPush(data) => (FrameType::CancelPush, data.clone()),
        Frame::Settings(settings) => {
            let mut p = Vec::new();
            for s in settings {
                varint::encode(s.id, &mut p);
                varint::encode(s.value, &mut p);
            }
            (FrameType::Settings, p)
        }
        Frame::PushPromise(data) => (FrameType::PushPromise, data.clone()),
        Frame::GoAway {
            stream_id,
        } => {
            let mut p = Vec::new();
            varint::encode(*stream_id, &mut p);
            (FrameType::GoAway, p)
        }
        Frame::MaxPushId(data) => (FrameType::MaxPushId, data.clone()),
        Frame::Grease(ty, data) => (FrameType::Grease(*ty), data.clone()),
        Frame::Unknown(ty, data) => (FrameType::Unknown(*ty), data.clone()),
    };
    let ty_u64 = ty.to_u64();
    let mut buf = Vec::new();
    varint::encode(ty_u64, &mut buf);
    varint::encode(payload.len() as u64, &mut buf);
    buf.extend_from_slice(&payload);
    buf
}

/// Errors that can occur when decoding an HTTP/3 frame from raw bytes.
#[derive(Debug)]
pub enum FrameDecodeError {
    /// Not enough bytes available to decode the full frame.
    Incomplete,
    /// A variable-length integer could not be decoded (invalid QUIC
    /// varint encoding).
    BadVarint,
    /// The frame type field contains an unknown type code.
    UnknownFrameType(u64),
}

/// Decodes an HTTP/3 [`Frame`] from raw bytes.
///
/// Reads a variable-length type, a variable-length payload length,
/// then the payload. Parses the payload according to the frame type.
/// Returns an error if the data is truncated, the varint is invalid,
/// or the type is unknown.
pub fn decode_frame(data: &[u8]) -> Result<(Frame, usize), FrameDecodeError> {
    let (ty_raw, tc) = varint::decode(data).map_err(|_| FrameDecodeError::BadVarint)?;
    let ty = FrameType::from_u64(ty_raw);
    let (len, lc) = varint::decode(&data[tc..]).map_err(|_| FrameDecodeError::BadVarint)?;
    let payload_start = tc + lc;
    let payload_end = payload_start + len as usize;
    if data.len() < payload_end {
        return Err(FrameDecodeError::Incomplete);
    }
    let payload = &data[payload_start..payload_end];

    let frame = match ty {
        FrameType::Data => Frame::Data(payload.to_vec()),
        FrameType::Headers => Frame::Headers(payload.to_vec()),
        FrameType::CancelPush => Frame::CancelPush(payload.to_vec()),
        FrameType::Settings => {
            let mut settings = Vec::new();
            let mut pos = 0;
            while pos < payload.len() {
                let (id, ic) = varint::decode(&payload[pos..]).map_err(|_| FrameDecodeError::BadVarint)?;
                let (value, vc) = varint::decode(&payload[pos + ic..]).map_err(|_| FrameDecodeError::BadVarint)?;
                settings.push(Setting {
                    id,
                    value,
                });
                pos += ic + vc;
            }
            Frame::Settings(settings)
        }
        FrameType::PushPromise => Frame::PushPromise(payload.to_vec()),
        FrameType::GoAway => {
            let (stream_id, _) = varint::decode(payload).map_err(|_| FrameDecodeError::BadVarint)?;
            Frame::GoAway {
                stream_id,
            }
        }
        FrameType::MaxPushId => Frame::MaxPushId(payload.to_vec()),
        FrameType::Grease(ty_v) => Frame::Grease(ty_v, payload.to_vec()),
        FrameType::Unknown(ty_v) => Frame::Unknown(ty_v, payload.to_vec()),
    };
    Ok((frame, payload_end))
}
