//! `bytes` crate support.
//!
//! `bytes::Bytes` and `bytes::BytesMut` are unambiguous byte buffers, so they
//! are encoded as CBOR byte strings (major type 2), unlike `Vec<u8>` and
//! `&[u8]`, which are CBOR arrays.

use ::bytes::{Bytes, BytesMut};

use crate::{
    bytes::DecodeBytes,
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, Write},
    error::Result,
};

impl Encode for Bytes {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.bytes(self.as_ref())
    }
}

impl Encode for BytesMut {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.bytes(self.as_ref())
    }
}

impl<'b> Decode<'b> for Bytes {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        Ok(Bytes::from(d.bytes_owned()?))
    }
}

impl<'b> Decode<'b> for BytesMut {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let bytes = d.bytes_owned()?;
        Ok(BytesMut::from(&bytes[..]))
    }
}

impl<'b> DecodeBytes<'b> for Bytes {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self> {
        Ok(Bytes::from(d.bytes_owned()?))
    }
}

impl<'b> DecodeBytes<'b> for BytesMut {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self> {
        let bytes = d.bytes_owned()?;
        Ok(BytesMut::from(&bytes[..]))
    }
}
