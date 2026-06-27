//! QUIC frame types (RFC 9000 §19).

use crate::{error::Error, varint};

/// All frame types used in milestone 1.
#[derive(Debug, Clone)]
pub enum Frame {
    Padding,
    Ping,
    Ack {
        largest_acknowledged: u64,
        ack_delay: u64,
        ack_ranges: Vec<(u64, u64)>, // (gap, length)
    },
    ResetStream {
        stream_id: u64,
        error_code: u64,
        final_size: u64,
    },
    StopSending {
        stream_id: u64,
        error_code: u64,
    },
    Crypto {
        offset: u64,
        data: Vec<u8>,
    },
    NewToken {
        token: Vec<u8>,
    },
    Stream {
        id: u64,
        offset: u64,
        data: Vec<u8>,
        fin: bool,
    },
    MaxData {
        maximum_data: u64,
    },
    MaxStreamData {
        stream_id: u64,
        maximum_stream_data: u64,
    },
    MaxStreams {
        id: u64,
        maximum_streams: u64,
    },
    DataBlocked {
        data_limit: u64,
    },
    StreamDataBlocked {
        stream_id: u64,
        stream_data_limit: u64,
    },
    StreamsBlocked {
        stream_type: bool,
        stream_limit: u64,
    },
    NewConnectionId {
        sequence_number: u64,
        retire_prior_to: u64,
        connection_id: Vec<u8>,
        stateless_reset_token: [u8; 16],
    },
    RetireConnectionId {
        sequence_number: u64,
    },
    PathChallenge {
        data: [u8; 8],
    },
    PathResponse {
        data: [u8; 8],
    },
    ConnectionClose {
        error_code: u64,
        frame_type: Option<u64>,
        reason_phrase: Vec<u8>,
    },
    HandshakeDone,
    Datagram {
        data: Vec<u8>,
    },
}

// ── Encoding ─────────────────────────────────────────────────────────────

pub fn encode(frame: &Frame, buf: &mut Vec<u8>) {
    match frame {
        Frame::Padding => {
            buf.push(0x00);
        }
        Frame::Ping => {
            buf.push(0x01);
        }
        Frame::Ack {
            largest_acknowledged,
            ack_delay,
            ack_ranges,
        } => {
            let ecn = false; // ECN not supported in milestone 1
            let ty: u8 = if ecn { 0x03 } else { 0x02 };
            buf.push(ty);
            varint::encode(*largest_acknowledged, buf);
            varint::encode(*ack_delay, buf);
            varint::encode(ack_ranges.len() as u64, buf);
            for (gap, length) in ack_ranges {
                varint::encode(*gap, buf);
                varint::encode(*length, buf);
            }
            // ECN counts omitted
        }
        Frame::Crypto {
            offset,
            data,
        } => {
            buf.push(0x06);
            varint::encode(*offset, buf);
            varint::encode(data.len() as u64, buf);
            buf.extend_from_slice(data);
        }
        Frame::Stream {
            id,
            offset,
            data,
            fin,
        } => {
            let mut ty = 0x08u8;
            let has_offset = *offset != 0;
            let has_len = true;
            if *fin {
                ty |= 0x01;
            }
            if has_offset {
                ty |= 0x04;
            }
            if has_len {
                ty |= 0x02;
            }
            buf.push(ty);
            varint::encode(*id, buf);
            if has_offset {
                varint::encode(*offset, buf);
            }
            varint::encode(data.len() as u64, buf);
            buf.extend_from_slice(data);
        }
        Frame::MaxData {
            maximum_data,
        } => {
            buf.push(0x10);
            varint::encode(*maximum_data, buf);
        }
        Frame::MaxStreamData {
            stream_id,
            maximum_stream_data,
        } => {
            buf.push(0x11);
            varint::encode(*stream_id, buf);
            varint::encode(*maximum_stream_data, buf);
        }
        Frame::MaxStreams {
            id,
            maximum_streams,
        } => {
            let ty: u8 = if *id & 1 == 0 { 0x12 } else { 0x13 };
            buf.push(ty);
            varint::encode(*maximum_streams, buf);
        }
        Frame::ConnectionClose {
            error_code,
            frame_type,
            reason_phrase,
        } => {
            if let Some(_ft) = frame_type {
                buf.push(0x1d); // CONNECTION_CLOSE with frame type
                varint::encode(*error_code, buf);
                varint::encode(*_ft, buf);
                let rl = reason_phrase.len() as u64;
                varint::encode(rl, buf);
                buf.extend_from_slice(reason_phrase);
            } else {
                buf.push(0x1c);
                varint::encode(*error_code, buf);
                let rl = reason_phrase.len() as u64;
                varint::encode(rl, buf);
                buf.extend_from_slice(reason_phrase);
            }
        }
        Frame::HandshakeDone => {
            buf.push(0x1e);
        }
        Frame::Datagram {
            data,
        } => {
            if data.len() > 0 {
                // If there's data, we include the length
                buf.push(0x31);
                varint::encode(data.len() as u64, buf);
                buf.extend_from_slice(data);
            } else {
                buf.push(0x30);
            }
        }
        _ => unimplemented!("frame type not yet supported"),
    }
}

// ── Decoding ─────────────────────────────────────────────────────────────

pub fn decode_all<'a>(data: &'a [u8]) -> Result<Vec<Frame>, Error> {
    let mut frames = Vec::new();
    let mut off = 0usize;
    while off < data.len() {
        let (frame, consumed) = decode_one(&data[off..])?;
        frames.push(frame);
        off += consumed;
    }
    Ok(frames)
}

pub fn decode_one(data: &[u8]) -> Result<(Frame, usize), Error> {
    if data.is_empty() {
        return Err(Error::FrameDecode("empty data".into()));
    }
    let (frame_type, type_len) =
        varint::decode(data).map_err(|_| Error::FrameDecode("bad frame type varint".into()))?;
    let mut off = type_len;

    match frame_type {
        0x00 => {
            // Padding: all consecutive 0x00 bytes up to the end
            let count = data.iter().take_while(|&&b| b == 0x00).count();
            Ok((Frame::Padding, count))
        }
        0x01 => Ok((Frame::Ping, 1)),
        0x02 | 0x03 => {
            let (largest, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("ack largest".into()))?;
            off += c;
            let (delay, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("ack delay".into()))?;
            off += c;
            let (range_count, c) =
                varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("ack range cnt".into()))?;
            off += c;
            let mut ranges = Vec::with_capacity(range_count as usize);
            for _ in 0..range_count {
                let (gap, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("ack gap".into()))?;
                off += c;
                let (len, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("ack len".into()))?;
                off += c;
                ranges.push((gap, len));
            }
            Ok((
                Frame::Ack {
                    largest_acknowledged: largest,
                    ack_delay: delay,
                    ack_ranges: ranges,
                },
                off,
            ))
        }
        0x06 => {
            let (offset, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("crypto offset".into()))?;
            off += c;
            let (len, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("crypto len".into()))?;
            off += c;
            if off + len as usize > data.len() {
                return Err(Error::FrameDecode("crypto data truncated".into()));
            }
            let d = data[off..off + len as usize].to_vec();
            off += len as usize;
            Ok((
                Frame::Crypto {
                    offset,
                    data: d,
                },
                off,
            ))
        }
        0x08..=0x0f => {
            let fin = (data[0] & 0x01) != 0;
            let has_len = (data[0] & 0x02) != 0;
            let has_offset = (data[0] & 0x04) != 0;
            let (id, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("stream id".into()))?;
            off += c;
            let offset = if has_offset {
                let (o, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("stream offset".into()))?;
                off += c;
                o
            } else {
                0
            };
            let len = if has_len {
                let (l, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("stream len".into()))?;
                off += c;
                l as usize
            } else {
                data.len() - off
            };
            if off + len > data.len() {
                return Err(Error::FrameDecode("stream data truncated".into()));
            }
            let d = data[off..off + len].to_vec();
            off += len;
            Ok((
                Frame::Stream {
                    id,
                    offset,
                    data: d,
                    fin,
                },
                off,
            ))
        }
        0x10 => {
            let (max, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("max_data".into()))?;
            off += c;
            Ok((
                Frame::MaxData {
                    maximum_data: max,
                },
                off,
            ))
        }
        0x11 => {
            let (sid, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("max_stream_data id".into()))?;
            off += c;
            let (max, c) =
                varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("max_stream_data val".into()))?;
            off += c;
            Ok((
                Frame::MaxStreamData {
                    stream_id: sid,
                    maximum_stream_data: max,
                },
                off,
            ))
        }
        0x12 | 0x13 => {
            let (max, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("max_streams".into()))?;
            off += c;
            Ok((
                Frame::MaxStreams {
                    id: frame_type,
                    maximum_streams: max,
                },
                off,
            ))
        }
        0x1c => {
            let (ec, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("close code".into()))?;
            off += c;
            let (rl, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("close reason len".into()))?;
            off += c;
            let reason = if rl as usize <= data.len() - off {
                let r = data[off..off + rl as usize].to_vec();
                off += rl as usize;
                r
            } else {
                Vec::new()
            };
            Ok((
                Frame::ConnectionClose {
                    error_code: ec,
                    frame_type: None,
                    reason_phrase: reason,
                },
                off,
            ))
        }
        0x1d => {
            let (ec, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("close code".into()))?;
            off += c;
            let (ft, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("close frame type".into()))?;
            off += c;
            let (rl, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("close reason len".into()))?;
            off += c;
            let reason = if rl as usize <= data.len() - off {
                let r = data[off..off + rl as usize].to_vec();
                off += rl as usize;
                r
            } else {
                Vec::new()
            };
            Ok((
                Frame::ConnectionClose {
                    error_code: ec,
                    frame_type: Some(ft),
                    reason_phrase: reason,
                },
                off,
            ))
        }
        0x1e => Ok((Frame::HandshakeDone, 1)),
        0x18 => {
            let (seq, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("new_cid seq".into()))?;
            off += c;
            let (retire, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("new_cid retire".into()))?;
            off += c;
            let (cid_len, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("new_cid len".into()))?;
            off += c;
            let cid_len = cid_len as usize;
            if off + cid_len + 16 > data.len() {
                return Err(Error::FrameDecode("new_cid truncated".into()));
            }
            let cid = data[off..off + cid_len].to_vec();
            off += cid_len;
            let mut token = [0u8; 16];
            token.copy_from_slice(&data[off..off + 16]);
            off += 16;
            Ok((
                Frame::NewConnectionId {
                    sequence_number: seq,
                    retire_prior_to: retire,
                    connection_id: cid,
                    stateless_reset_token: token,
                },
                off,
            ))
        }
        0x19 => {
            let (seq, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("retire_cid seq".into()))?;
            off += c;
            Ok((
                Frame::RetireConnectionId {
                    sequence_number: seq,
                },
                off,
            ))
        }
        0x1a => {
            if off + 8 > data.len() {
                return Err(Error::FrameDecode("path_challenge truncated".into()));
            }
            let mut d = [0u8; 8];
            d.copy_from_slice(&data[off..off + 8]);
            off += 8;
            Ok((
                Frame::PathChallenge {
                    data: d,
                },
                off,
            ))
        }
        0x1b => {
            if off + 8 > data.len() {
                return Err(Error::FrameDecode("path_response truncated".into()));
            }
            let mut d = [0u8; 8];
            d.copy_from_slice(&data[off..off + 8]);
            off += 8;
            Ok((
                Frame::PathResponse {
                    data: d,
                },
                off,
            ))
        }
        0x30 => Ok((
            Frame::Datagram {
                data: Vec::new(),
            },
            1,
        )),
        0x31 => {
            let (len, c) = varint::decode(&data[off..]).map_err(|_| Error::FrameDecode("datagram len".into()))?;
            off += c;
            let d = data[off..off + len as usize].to_vec();
            off += len as usize;
            Ok((
                Frame::Datagram {
                    data: d,
                },
                off,
            ))
        }
        _ => {
            // Unknown frame type: skip the rest of the payload.
            // RFC 9000 §19.21 says this should be a FRAME_ENCODING_ERROR,
            // but many servers send GREASE frames that we should ignore.
            Ok((Frame::Padding, data.len()))
        }
    }
}

/// Number of padding bytes to reach `target_len`.
pub fn pad_to(target_len: usize, current_len: usize, buf: &mut Vec<u8>) {
    for _ in 0..target_len.saturating_sub(current_len) {
        buf.push(0x00);
    }
}
