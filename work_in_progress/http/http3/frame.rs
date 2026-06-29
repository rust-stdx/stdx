use quic::varint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Data,
    Headers,
    CancelPush,
    Settings,
    PushPromise,
    GoAway,
    MaxPushId,
    Grease(u64),
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

#[derive(Debug, Clone)]
pub struct Setting {
    pub id: u64,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub enum Frame {
    Data(Vec<u8>),
    Headers(Vec<u8>),
    CancelPush(Vec<u8>),
    Settings(Vec<Setting>),
    PushPromise(Vec<u8>),
    GoAway { stream_id: u64 },
    MaxPushId(Vec<u8>),
    Grease(u64, Vec<u8>),
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

#[derive(Debug)]
pub enum FrameDecodeError {
    Incomplete,
    BadVarint,
    UnknownFrameType(u64),
}

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
