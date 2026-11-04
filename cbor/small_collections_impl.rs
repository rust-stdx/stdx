//! `small_collections` container support.

use small_collections::{SmallString, SmallVec};

use crate::{
    bytes::DecodeBytes,
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, Write},
    error::Result,
};

impl<T: Encode, const N: usize> Encode for SmallVec<T, N> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.array(self.len())?;
        for item in self {
            item.encode(e)?;
        }
        Ok(())
    }
}

impl<'b, T: Decode<'b>, const N: usize> Decode<'b> for SmallVec<T, N> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = SmallVec::new();
        match d.array()? {
            Some(len) => {
                // Do not reserve from the untrusted header: `len` is bounded by
                // the remaining input bytes, not by `len * size_of::<T>()`.
                for _ in 0..len {
                    out.push(T::decode(d)?);
                }
            }
            None => {
                while !d.is_break() {
                    out.push(T::decode(d)?);
                }
                d.read_break()?;
            }
        }
        Ok(out)
    }
}

impl<const N: usize> Encode for SmallString<N> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.text(self.as_str())
    }
}

impl<'b, const N: usize> Decode<'b> for SmallString<N> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = SmallString::new();
        if d.peek_byte()? & 0x1f == 31 {
            let iter = d.text_iter()?;
            for chunk in iter {
                out.push_str(chunk?);
            }
        } else {
            out.push_str(d.text()?);
        }
        Ok(out)
    }
}

impl<'b, const N: usize> DecodeBytes<'b> for SmallVec<u8, N> {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self> {
        let bytes = d.bytes_owned()?;
        Ok(SmallVec::from_slice(&bytes))
    }
}
