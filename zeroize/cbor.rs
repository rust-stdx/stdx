//! CBOR support for [`Zeroizing`].
//!
//! The inner value is encoded and decoded normally; the temporary copy is
//! zeroized when it is dropped.

use ::cbor::{Decode, Decoder, Encode, Encoder, Error, Write};

use crate::{Zeroize, Zeroizing};

impl<Z> Encode for Zeroizing<Z>
where
    Z: Zeroize + Encode,
{
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        self.0.encode(encoder)
    }
}

impl<'b, Z> Decode<'b> for Zeroizing<Z>
where
    Z: Zeroize + Decode<'b>,
{
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, Error> {
        Ok(Zeroizing::new(Z::decode(decoder)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let value = Zeroizing::new(42u64);
        let bytes = ::cbor::to_vec(&value);
        let decoded: Zeroizing<u64> = ::cbor::from_slice(&bytes).unwrap();
        assert_eq!(*decoded, 42);
    }
}
