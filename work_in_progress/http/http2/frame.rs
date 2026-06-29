use super::error::ErrorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Data,
    Headers,
    Priority,
    RstStream,
    Settings,
    PushPromise,
    Ping,
    GoAway,
    WindowUpdate,
    Continuation,
}

impl FrameType {
    pub fn to_u8(self) -> u8 {
        match self {
            FrameType::Data => 0x00,
            FrameType::Headers => 0x01,
            FrameType::Priority => 0x02,
            FrameType::RstStream => 0x03,
            FrameType::Settings => 0x04,
            FrameType::PushPromise => 0x05,
            FrameType::Ping => 0x06,
            FrameType::GoAway => 0x07,
            FrameType::WindowUpdate => 0x08,
            FrameType::Continuation => 0x09,
        }
    }

    pub fn from_u8(ty: u8) -> Option<Self> {
        match ty {
            0x00 => Some(FrameType::Data),
            0x01 => Some(FrameType::Headers),
            0x02 => Some(FrameType::Priority),
            0x03 => Some(FrameType::RstStream),
            0x04 => Some(FrameType::Settings),
            0x05 => Some(FrameType::PushPromise),
            0x06 => Some(FrameType::Ping),
            0x07 => Some(FrameType::GoAway),
            0x08 => Some(FrameType::WindowUpdate),
            0x09 => Some(FrameType::Continuation),
            _ => None,
        }
    }
}

pub const FLAG_END_STREAM: u8 = 0x01;
pub const FLAG_ACK: u8 = 0x01;
pub const FLAG_END_HEADERS: u8 = 0x04;
pub const FLAG_PADDED: u8 = 0x08;
pub const FLAG_PRIORITY: u8 = 0x20;

#[derive(Debug, Clone)]
pub struct FrameHeader {
    pub length: u32,
    pub frame_type: FrameType,
    pub flags: u8,
    pub stream_id: u32,
}

pub fn encode_frame_header(header: &FrameHeader, buf: &mut Vec<u8>) {
    buf.push((header.length >> 16) as u8);
    buf.push((header.length >> 8) as u8);
    buf.push(header.length as u8);
    buf.push(header.frame_type.to_u8());
    buf.push(header.flags);
    buf.push((header.stream_id >> 24) as u8);
    buf.push((header.stream_id >> 16) as u8);
    buf.push((header.stream_id >> 8) as u8);
    buf.push(header.stream_id as u8);
}

pub fn decode_frame_header(data: &[u8]) -> Result<(FrameHeader, usize), H2FrameError> {
    if data.len() < 9 {
        return Err(H2FrameError::Incomplete);
    }
    let length = ((data[0] as u32) << 16) | ((data[1] as u32) << 8) | (data[2] as u32);
    let frame_type = data[3];
    let flags = data[4];
    let stream_id = ((data[5] as u32) << 24) | ((data[6] as u32) << 16) | ((data[7] as u32) << 8) | (data[8] as u32);
    let stream_id = stream_id & 0x7FFF_FFFF;
    let ft = FrameType::from_u8(frame_type).ok_or(H2FrameError::UnknownFrameType(frame_type))?;
    Ok((
        FrameHeader {
            length,
            frame_type: ft,
            flags,
            stream_id,
        },
        9,
    ))
}

#[derive(Debug)]
pub enum H2FrameError {
    Incomplete,
    UnknownFrameType(u8),
    BadPayload(&'static str),
    InvalidStreamId,
}

#[derive(Debug, Clone)]
pub enum Frame {
    Data {
        stream_id: u32,
        end_stream: bool,
        padded: bool,
        data: Vec<u8>,
        padding: Vec<u8>,
    },
    Headers {
        stream_id: u32,
        end_stream: bool,
        end_headers: bool,
        padded: bool,
        priority: bool,
        exclusive: bool,
        stream_dependency: u32,
        weight: u8,
        fragment: Vec<u8>,
        padding: Vec<u8>,
    },
    Priority {
        stream_id: u32,
        exclusive: bool,
        stream_dependency: u32,
        weight: u8,
    },
    RstStream {
        stream_id: u32,
        error_code: ErrorCode,
    },
    Settings {
        ack: bool,
        settings: Vec<(u16, u32)>,
    },
    PushPromise {
        stream_id: u32,
        end_headers: bool,
        padded: bool,
        promised_stream_id: u32,
        fragment: Vec<u8>,
        padding: Vec<u8>,
    },
    Ping {
        ack: bool,
        opaque_data: [u8; 8],
    },
    GoAway {
        last_stream_id: u32,
        error_code: ErrorCode,
        debug_data: Vec<u8>,
    },
    WindowUpdate {
        stream_id: u32,
        window_size_increment: u32,
    },
    Continuation {
        stream_id: u32,
        end_headers: bool,
        fragment: Vec<u8>,
    },
}

fn flag_data(end_stream: bool, padded: bool) -> u8 {
    let mut f = 0u8;
    if end_stream {
        f |= FLAG_END_STREAM;
    }
    if padded {
        f |= FLAG_PADDED;
    }
    f
}

fn flag_headers(end_stream: bool, end_headers: bool, padded: bool, priority: bool) -> u8 {
    let mut f = 0u8;
    if end_stream {
        f |= FLAG_END_STREAM;
    }
    if end_headers {
        f |= FLAG_END_HEADERS;
    }
    if padded {
        f |= FLAG_PADDED;
    }
    if priority {
        f |= FLAG_PRIORITY;
    }
    f
}

pub fn encode_frame(frame: &Frame) -> Vec<u8> {
    match frame {
        Frame::Data {
            stream_id,
            end_stream,
            padded,
            data,
            padding,
        } => {
            let flags = flag_data(*end_stream, *padded);
            let mut payload = Vec::new();
            if *padded {
                payload.push(padding.len() as u8);
                payload.extend_from_slice(data);
                payload.extend_from_slice(padding);
            } else {
                payload.extend_from_slice(data);
            }
            let mut buf = Vec::with_capacity(9 + payload.len());
            encode_frame_header(
                &FrameHeader {
                    length: payload.len() as u32,
                    frame_type: FrameType::Data,
                    flags,
                    stream_id: *stream_id,
                },
                &mut buf,
            );
            buf.extend_from_slice(&payload);
            buf
        }
        Frame::Headers {
            stream_id,
            end_stream,
            end_headers,
            padded,
            priority,
            exclusive,
            stream_dependency,
            weight,
            fragment,
            padding,
        } => {
            let flags = flag_headers(*end_stream, *end_headers, *padded, *priority);
            let mut payload = Vec::new();
            if *padded {
                payload.push(padding.len() as u8);
            }
            if *priority {
                let dep = *stream_dependency | if *exclusive { 0x8000_0000 } else { 0 };
                payload.push((dep >> 24) as u8);
                payload.push((dep >> 16) as u8);
                payload.push((dep >> 8) as u8);
                payload.push(dep as u8);
                payload.push(*weight);
            }
            payload.extend_from_slice(fragment);
            if *padded {
                payload.extend_from_slice(padding);
            }
            let mut buf = Vec::with_capacity(9 + payload.len());
            encode_frame_header(
                &FrameHeader {
                    length: payload.len() as u32,
                    frame_type: FrameType::Headers,
                    flags,
                    stream_id: *stream_id,
                },
                &mut buf,
            );
            buf.extend_from_slice(&payload);
            buf
        }
        Frame::Priority {
            stream_id,
            exclusive,
            stream_dependency,
            weight,
        } => {
            let dep = *stream_dependency | if *exclusive { 0x8000_0000 } else { 0 };
            let payload = vec![
                (dep >> 24) as u8,
                (dep >> 16) as u8,
                (dep >> 8) as u8,
                dep as u8,
                *weight,
            ];
            let mut buf = Vec::with_capacity(9 + 5);
            encode_frame_header(
                &FrameHeader {
                    length: 5,
                    frame_type: FrameType::Priority,
                    flags: 0,
                    stream_id: *stream_id,
                },
                &mut buf,
            );
            buf.extend_from_slice(&payload);
            buf
        }
        Frame::RstStream {
            stream_id,
            error_code,
        } => {
            let ec = error_code.to_u32();
            let payload = vec![(ec >> 24) as u8, (ec >> 16) as u8, (ec >> 8) as u8, ec as u8];
            let mut buf = Vec::with_capacity(9 + 4);
            encode_frame_header(
                &FrameHeader {
                    length: 4,
                    frame_type: FrameType::RstStream,
                    flags: 0,
                    stream_id: *stream_id,
                },
                &mut buf,
            );
            buf.extend_from_slice(&payload);
            buf
        }
        Frame::Settings {
            ack,
            settings,
        } => {
            let flags = if *ack { FLAG_ACK } else { 0 };
            let mut payload = Vec::new();
            for &(id, value) in settings {
                payload.push((id >> 8) as u8);
                payload.push(id as u8);
                payload.push((value >> 24) as u8);
                payload.push((value >> 16) as u8);
                payload.push((value >> 8) as u8);
                payload.push(value as u8);
            }
            let mut buf = Vec::with_capacity(9 + payload.len());
            encode_frame_header(
                &FrameHeader {
                    length: payload.len() as u32,
                    frame_type: FrameType::Settings,
                    flags,
                    stream_id: 0,
                },
                &mut buf,
            );
            buf.extend_from_slice(&payload);
            buf
        }
        Frame::PushPromise {
            stream_id,
            end_headers,
            padded,
            promised_stream_id,
            fragment,
            padding,
        } => {
            let mut flags = if *end_headers { FLAG_END_HEADERS } else { 0 };
            if *padded {
                flags |= FLAG_PADDED;
            }
            let mut payload = Vec::new();
            if *padded {
                payload.push(padding.len() as u8);
            }
            let psid = *promised_stream_id & 0x7FFF_FFFF;
            payload.push((psid >> 24) as u8);
            payload.push((psid >> 16) as u8);
            payload.push((psid >> 8) as u8);
            payload.push(psid as u8);
            payload.extend_from_slice(fragment);
            if *padded {
                payload.extend_from_slice(padding);
            }
            let mut buf = Vec::with_capacity(9 + payload.len());
            encode_frame_header(
                &FrameHeader {
                    length: payload.len() as u32,
                    frame_type: FrameType::PushPromise,
                    flags,
                    stream_id: *stream_id,
                },
                &mut buf,
            );
            buf.extend_from_slice(&payload);
            buf
        }
        Frame::Ping {
            ack,
            opaque_data,
        } => {
            let flags = if *ack { FLAG_ACK } else { 0 };
            let mut buf = Vec::with_capacity(9 + 8);
            encode_frame_header(
                &FrameHeader {
                    length: 8,
                    frame_type: FrameType::Ping,
                    flags,
                    stream_id: 0,
                },
                &mut buf,
            );
            buf.extend_from_slice(opaque_data);
            buf
        }
        Frame::GoAway {
            last_stream_id,
            error_code,
            debug_data,
        } => {
            let lsid = *last_stream_id & 0x7FFF_FFFF;
            let ec = error_code.to_u32();
            let mut payload = Vec::with_capacity(8 + debug_data.len());
            payload.push((lsid >> 24) as u8);
            payload.push((lsid >> 16) as u8);
            payload.push((lsid >> 8) as u8);
            payload.push(lsid as u8);
            payload.push((ec >> 24) as u8);
            payload.push((ec >> 16) as u8);
            payload.push((ec >> 8) as u8);
            payload.push(ec as u8);
            payload.extend_from_slice(debug_data);
            let mut buf = Vec::with_capacity(9 + payload.len());
            encode_frame_header(
                &FrameHeader {
                    length: payload.len() as u32,
                    frame_type: FrameType::GoAway,
                    flags: 0,
                    stream_id: 0,
                },
                &mut buf,
            );
            buf.extend_from_slice(&payload);
            buf
        }
        Frame::WindowUpdate {
            stream_id,
            window_size_increment,
        } => {
            let inc = *window_size_increment & 0x7FFF_FFFF;
            let payload = vec![(inc >> 24) as u8, (inc >> 16) as u8, (inc >> 8) as u8, inc as u8];
            let mut buf = Vec::with_capacity(9 + 4);
            encode_frame_header(
                &FrameHeader {
                    length: 4,
                    frame_type: FrameType::WindowUpdate,
                    flags: 0,
                    stream_id: *stream_id,
                },
                &mut buf,
            );
            buf.extend_from_slice(&payload);
            buf
        }
        Frame::Continuation {
            stream_id,
            end_headers,
            fragment,
        } => {
            let flags = if *end_headers { FLAG_END_HEADERS } else { 0 };
            let mut buf = Vec::with_capacity(9 + fragment.len());
            encode_frame_header(
                &FrameHeader {
                    length: fragment.len() as u32,
                    frame_type: FrameType::Continuation,
                    flags,
                    stream_id: *stream_id,
                },
                &mut buf,
            );
            buf.extend_from_slice(fragment);
            buf
        }
    }
}

pub fn decode_frame(data: &[u8]) -> Result<(Frame, usize), H2FrameError> {
    let (header, hdr_len) = decode_frame_header(data)?;
    let total_len = hdr_len + header.length as usize;
    if data.len() < total_len {
        return Err(H2FrameError::Incomplete);
    }
    let payload = &data[hdr_len..total_len];

    let frame = match header.frame_type {
        FrameType::Data => {
            let padded = (header.flags & FLAG_PADDED) != 0;
            let end_stream = (header.flags & FLAG_END_STREAM) != 0;
            let mut offset = 0;
            let pad_len = if padded {
                if payload.is_empty() {
                    return Err(H2FrameError::BadPayload("DATA: missing pad length"));
                }
                let pl = payload[0] as usize;
                offset += 1;
                pl
            } else {
                0
            };
            if offset + pad_len > payload.len() {
                return Err(H2FrameError::BadPayload("DATA: padding exceeds payload"));
            }
            let data_end = payload.len() - pad_len;
            let data = payload[offset..data_end].to_vec();
            let padding = if padded {
                payload[data_end..].to_vec()
            } else {
                Vec::new()
            };
            Frame::Data {
                stream_id: header.stream_id,
                end_stream,
                padded,
                data,
                padding,
            }
        }
        FrameType::Headers => {
            let padded = (header.flags & FLAG_PADDED) != 0;
            let priority = (header.flags & FLAG_PRIORITY) != 0;
            let end_stream = (header.flags & FLAG_END_STREAM) != 0;
            let end_headers = (header.flags & FLAG_END_HEADERS) != 0;
            let mut offset = 0;
            let pad_len = if padded {
                if payload.is_empty() {
                    return Err(H2FrameError::BadPayload("HEADERS: missing pad length"));
                }
                let pl = payload[0] as usize;
                offset += 1;
                pl
            } else {
                0
            };
            let (exclusive, stream_dependency, weight) = if priority {
                if payload.len() < offset + 5 {
                    return Err(H2FrameError::BadPayload("HEADERS: priority truncated"));
                }
                let dep = ((payload[offset] as u32) << 24)
                    | ((payload[offset + 1] as u32) << 16)
                    | ((payload[offset + 2] as u32) << 8)
                    | (payload[offset + 3] as u32);
                let excl = (dep & 0x8000_0000) != 0;
                let dep_id = dep & 0x7FFF_FFFF;
                let w = payload[offset + 4];
                offset += 5;
                (excl, dep_id, w)
            } else {
                (false, 0, 0)
            };
            let fragment_end = payload.len() - pad_len;
            let fragment = payload[offset..fragment_end].to_vec();
            let padding = if padded {
                payload[fragment_end..].to_vec()
            } else {
                Vec::new()
            };
            Frame::Headers {
                stream_id: header.stream_id,
                end_stream,
                end_headers,
                padded,
                priority,
                exclusive,
                stream_dependency,
                weight,
                fragment,
                padding,
            }
        }
        FrameType::Priority => {
            if payload.len() < 5 {
                return Err(H2FrameError::BadPayload("PRIORITY: truncated"));
            }
            let dep = ((payload[0] as u32) << 24)
                | ((payload[1] as u32) << 16)
                | ((payload[2] as u32) << 8)
                | (payload[3] as u32);
            let exclusive = (dep & 0x8000_0000) != 0;
            let stream_dependency = dep & 0x7FFF_FFFF;
            let weight = payload[4];
            Frame::Priority {
                stream_id: header.stream_id,
                exclusive,
                stream_dependency,
                weight,
            }
        }
        FrameType::RstStream => {
            if payload.len() < 4 {
                return Err(H2FrameError::BadPayload("RST_STREAM: truncated"));
            }
            let ec = ((payload[0] as u32) << 24)
                | ((payload[1] as u32) << 16)
                | ((payload[2] as u32) << 8)
                | (payload[3] as u32);
            Frame::RstStream {
                stream_id: header.stream_id,
                error_code: ErrorCode::from_u32(ec),
            }
        }
        FrameType::Settings => {
            let ack = (header.flags & FLAG_ACK) != 0;
            if ack && !payload.is_empty() {
                return Err(H2FrameError::BadPayload("SETTINGS ACK must have empty payload"));
            }
            if payload.len() % 6 != 0 {
                return Err(H2FrameError::BadPayload("SETTINGS: payload must be multiple of 6"));
            }
            let mut settings = Vec::new();
            for chunk in payload.chunks(6) {
                let id = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
                let value = ((chunk[2] as u32) << 24)
                    | ((chunk[3] as u32) << 16)
                    | ((chunk[4] as u32) << 8)
                    | (chunk[5] as u32);
                settings.push((id, value));
            }
            Frame::Settings {
                ack,
                settings,
            }
        }
        FrameType::PushPromise => {
            let padded = (header.flags & FLAG_PADDED) != 0;
            let end_headers = (header.flags & FLAG_END_HEADERS) != 0;
            let mut offset = 0;
            let pad_len = if padded {
                if payload.is_empty() {
                    return Err(H2FrameError::BadPayload("PUSH_PROMISE: missing pad length"));
                }
                let pl = payload[0] as usize;
                offset += 1;
                pl
            } else {
                0
            };
            if payload.len() < offset + 4 {
                return Err(H2FrameError::BadPayload("PUSH_PROMISE: truncated promised stream ID"));
            }
            let psid = ((payload[offset] as u32) << 24)
                | ((payload[offset + 1] as u32) << 16)
                | ((payload[offset + 2] as u32) << 8)
                | (payload[offset + 3] as u32);
            let promised_stream_id = psid & 0x7FFF_FFFF;
            offset += 4;
            let fragment_end = payload.len() - pad_len;
            let fragment = payload[offset..fragment_end].to_vec();
            let padding = if padded {
                payload[fragment_end..].to_vec()
            } else {
                Vec::new()
            };
            Frame::PushPromise {
                stream_id: header.stream_id,
                end_headers,
                padded,
                promised_stream_id,
                fragment,
                padding,
            }
        }
        FrameType::Ping => {
            if payload.len() < 8 {
                return Err(H2FrameError::BadPayload("PING: truncated"));
            }
            let mut opaque_data = [0u8; 8];
            opaque_data.copy_from_slice(&payload[..8]);
            let ack = (header.flags & FLAG_ACK) != 0;
            Frame::Ping {
                ack,
                opaque_data,
            }
        }
        FrameType::GoAway => {
            if payload.len() < 8 {
                return Err(H2FrameError::BadPayload("GOAWAY: truncated"));
            }
            let last_stream_id = ((payload[0] as u32) << 24)
                | ((payload[1] as u32) << 16)
                | ((payload[2] as u32) << 8)
                | (payload[3] as u32);
            let ec = ((payload[4] as u32) << 24)
                | ((payload[5] as u32) << 16)
                | ((payload[6] as u32) << 8)
                | (payload[7] as u32);
            let debug_data = if payload.len() > 8 {
                payload[8..].to_vec()
            } else {
                Vec::new()
            };
            Frame::GoAway {
                last_stream_id: last_stream_id & 0x7FFF_FFFF,
                error_code: ErrorCode::from_u32(ec),
                debug_data,
            }
        }
        FrameType::WindowUpdate => {
            if payload.len() < 4 {
                return Err(H2FrameError::BadPayload("WINDOW_UPDATE: truncated"));
            }
            let inc = ((payload[0] as u32) << 24)
                | ((payload[1] as u32) << 16)
                | ((payload[2] as u32) << 8)
                | (payload[3] as u32);
            Frame::WindowUpdate {
                stream_id: header.stream_id,
                window_size_increment: inc & 0x7FFF_FFFF,
            }
        }
        FrameType::Continuation => {
            let end_headers = (header.flags & FLAG_END_HEADERS) != 0;
            let fragment = payload.to_vec();
            Frame::Continuation {
                stream_id: header.stream_id,
                end_headers,
                fragment,
            }
        }
    };

    Ok((frame, total_len))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(frame: &Frame) {
        let encoded = encode_frame(frame);
        let (decoded, consumed) = decode_frame(&encoded).unwrap();
        assert_eq!(consumed, encoded.len());
        assert_frame_eq(&decoded, frame);
    }

    fn assert_frame_eq(a: &Frame, b: &Frame) {
        match (a, b) {
            (
                Frame::Data {
                    stream_id: asid,
                    end_stream: aes,
                    data: ad,
                    ..
                },
                Frame::Data {
                    stream_id: bsid,
                    end_stream: bes,
                    data: bd,
                    ..
                },
            ) => {
                assert_eq!(asid, bsid);
                assert_eq!(aes, bes);
                assert_eq!(ad, bd);
            }
            (
                Frame::Headers {
                    stream_id: asid,
                    end_stream: aes,
                    end_headers: aeh,
                    fragment: af,
                    ..
                },
                Frame::Headers {
                    stream_id: bsid,
                    end_stream: bes,
                    end_headers: beh,
                    fragment: bf,
                    ..
                },
            ) => {
                assert_eq!(asid, bsid);
                assert_eq!(aes, bes);
                assert_eq!(aeh, beh);
                assert_eq!(af, bf);
            }
            (
                Frame::Settings {
                    ack: aa,
                    settings: as_,
                },
                Frame::Settings {
                    ack: ba,
                    settings: bs_,
                },
            ) => {
                assert_eq!(aa, ba);
                assert_eq!(as_, bs_);
            }
            (
                Frame::GoAway {
                    last_stream_id: a,
                    error_code: ae,
                    debug_data: add,
                },
                Frame::GoAway {
                    last_stream_id: b,
                    error_code: be,
                    debug_data: bdd,
                },
            ) => {
                assert_eq!(a, b);
                assert_eq!(ae, be);
                assert_eq!(add, bdd);
            }
            (a, b) => {
                let encoded_a = encode_frame(a);
                let encoded_b = encode_frame(b);
                assert_eq!(encoded_a, encoded_b);
            }
        }
    }

    #[test]
    fn test_frame_type_roundtrip() {
        for ty in &[
            FrameType::Data,
            FrameType::Headers,
            FrameType::Priority,
            FrameType::RstStream,
            FrameType::Settings,
            FrameType::Ping,
            FrameType::GoAway,
            FrameType::WindowUpdate,
            FrameType::Continuation,
            FrameType::PushPromise,
        ] {
            assert_eq!(FrameType::from_u8(ty.to_u8()), Some(*ty));
        }
    }

    #[test]
    fn test_data_frame() {
        roundtrip(&Frame::Data {
            stream_id: 1,
            end_stream: true,
            padded: false,
            data: b"hello".to_vec(),
            padding: vec![],
        });
    }

    #[test]
    fn test_data_frame_padded() {
        roundtrip(&Frame::Data {
            stream_id: 3,
            end_stream: false,
            padded: true,
            data: b"payload".to_vec(),
            padding: vec![0; 8],
        });
    }

    #[test]
    fn test_headers_frame() {
        roundtrip(&Frame::Headers {
            stream_id: 1,
            end_stream: true,
            end_headers: true,
            padded: false,
            priority: false,
            exclusive: false,
            stream_dependency: 0,
            weight: 0,
            fragment: vec![0x82, 0x84, 0x86],
            padding: vec![],
        });
    }

    #[test]
    fn test_headers_with_priority() {
        roundtrip(&Frame::Headers {
            stream_id: 1,
            end_stream: false,
            end_headers: true,
            padded: false,
            priority: true,
            exclusive: false,
            stream_dependency: 3,
            weight: 128,
            fragment: vec![0x82],
            padding: vec![],
        });
    }

    #[test]
    fn test_priority_frame() {
        roundtrip(&Frame::Priority {
            stream_id: 3,
            exclusive: false,
            stream_dependency: 0,
            weight: 200,
        });
    }

    #[test]
    fn test_rst_stream() {
        roundtrip(&Frame::RstStream {
            stream_id: 1,
            error_code: ErrorCode::NoError,
        });
        roundtrip(&Frame::RstStream {
            stream_id: 5,
            error_code: ErrorCode::ProtocolError,
        });
    }

    #[test]
    fn test_settings() {
        roundtrip(&Frame::Settings {
            ack: false,
            settings: vec![(0x01, 4096), (0x04, 65535)],
        });
        roundtrip(&Frame::Settings {
            ack: true,
            settings: vec![],
        });
    }

    #[test]
    fn test_ping() {
        roundtrip(&Frame::Ping {
            ack: false,
            opaque_data: [0; 8],
        });
        roundtrip(&Frame::Ping {
            ack: true,
            opaque_data: [1, 2, 3, 4, 5, 6, 7, 8],
        });
    }

    #[test]
    fn test_goaway() {
        roundtrip(&Frame::GoAway {
            last_stream_id: 0,
            error_code: ErrorCode::NoError,
            debug_data: vec![],
        });
        roundtrip(&Frame::GoAway {
            last_stream_id: 100,
            error_code: ErrorCode::InternalError,
            debug_data: b"shutdown".to_vec(),
        });
    }

    #[test]
    fn test_window_update() {
        roundtrip(&Frame::WindowUpdate {
            stream_id: 0,
            window_size_increment: 65535,
        });
        roundtrip(&Frame::WindowUpdate {
            stream_id: 3,
            window_size_increment: 16384,
        });
    }

    #[test]
    fn test_continuation() {
        roundtrip(&Frame::Continuation {
            stream_id: 1,
            end_headers: true,
            fragment: vec![0x82, 0x84],
        });
    }

    #[test]
    fn test_push_promise() {
        roundtrip(&Frame::PushPromise {
            stream_id: 1,
            end_headers: true,
            padded: false,
            promised_stream_id: 2,
            fragment: vec![0x82],
            padding: vec![],
        });
    }

    #[test]
    fn test_decode_incomplete_header() {
        assert!(matches!(decode_frame(&[0; 8]), Err(H2FrameError::Incomplete)));
    }

    #[test]
    fn test_decode_incomplete_payload() {
        let frame = Frame::Ping {
            ack: false,
            opaque_data: [0; 8],
        };
        let mut encoded = encode_frame(&frame);
        encoded.truncate(encoded.len() - 2);
        assert!(matches!(decode_frame(&encoded), Err(H2FrameError::Incomplete)));
    }

    #[test]
    fn test_decode_unknown_frame_type() {
        let data = vec![
            0x00, 0x00, 0x00, // length
            0x0A, // unknown type 10
            0x00, // flags
            0x00, 0x00, 0x00, 0x01, // stream_id
        ];
        assert!(matches!(decode_frame(&data), Err(H2FrameError::UnknownFrameType(0x0A))));
    }
}
