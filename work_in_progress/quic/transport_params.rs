//! QUIC transport parameters (RFC 9000 §18).

use alloc::vec::Vec;

use crate::{error::Error, varint};

/// Known transport parameter type codes (RFC 9000 §18.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum ParamType {
    OriginalDestinationConnectionId = 0x00,
    MaxIdleTimeout = 0x01,
    StatelessResetToken = 0x02,
    MaxUdpPayloadSize = 0x03,
    InitialMaxData = 0x04,
    InitialMaxStreamDataBidiLocal = 0x05,
    InitialMaxStreamDataBidiRemote = 0x06,
    InitialMaxStreamDataUni = 0x07,
    InitialMaxStreamsBidi = 0x08,
    InitialMaxStreamsUni = 0x09,
    AckDelayExponent = 0x0a,
    MaxAckDelay = 0x0b,
    DisableActiveMigration = 0x0c,
    PreferredAddress = 0x0d,
    ActiveConnectionIdLimit = 0x0e,
    InitialSourceConnectionId = 0x0f,
    RetrySourceConnectionId = 0x10,
    VersionInformation = 0x11,
    MaxDatagramFrameSize = 0x20,
}

/// A single transport parameter (type, value).
#[derive(Debug, Clone)]
pub struct Param {
    pub ty: u64,
    pub value: Vec<u8>,
}

/// Encode a list of transport parameters into a byte buffer.
pub fn encode(params: &[Param]) -> Vec<u8> {
    let mut buf = Vec::new();
    for p in params {
        varint::encode(p.ty, &mut buf);
        varint::encode(p.value.len() as u64, &mut buf);
        buf.extend_from_slice(&p.value);
    }
    buf
}

/// Decode transport parameters from a buffer.
pub fn decode(data: &[u8]) -> Result<Vec<Param>, Error> {
    let mut params = Vec::new();
    let mut off = 0usize;
    while off < data.len() {
        let (ty, c) = varint::decode(&data[off..]).map_err(|_| Error::TransportParam("bad param type".into()))?;
        off += c;
        let (len, c) = varint::decode(&data[off..]).map_err(|_| Error::TransportParam("bad param len".into()))?;
        off += c;
        if off + len as usize > data.len() {
            return Err(Error::TransportParam("param value truncated".into()));
        }
        let value = data[off..off + len as usize].to_vec();
        params.push(Param {
            ty,
            value,
        });
        off += len as usize;
    }
    Ok(params)
}

/// Find a parameter by type.
pub fn find_param<'a>(params: &'a [Param], ty: ParamType) -> Option<&'a Param> {
    params.iter().find(|p| p.ty == ty as u64)
}

/// Extract a varint value from a transport parameter.
pub fn param_varint(param: &Param) -> Result<u64, Error> {
    let (v, _) = varint::decode(&param.value).map_err(|_| Error::TransportParam("bad varint param".into()))?;
    Ok(v)
}

/// Extract a byte string value from a transport parameter.
pub fn param_bytes(param: &Param) -> &[u8] {
    &param.value
}
