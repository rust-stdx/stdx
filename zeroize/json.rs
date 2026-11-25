//! JSON support for [`Zeroizing`].
//!
//! The inner value is encoded and decoded normally; the temporary copy is
//! zeroized when it is dropped.

use ::json::{Decode, Decoder, Encode, Encoder, Write};

use crate::{Zeroize, Zeroizing};

impl<Z> Encode for Zeroizing<Z>
where
    Z: Zeroize + Encode,
{
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        self.0.encode(encoder)
    }
}

impl<'a, Z> Decode<'a> for Zeroizing<Z>
where
    Z: Zeroize + Decode<'a>,
{
    fn decode(decoder: &mut Decoder<'a>) -> ::json::Result<Self> {
        Ok(Zeroizing::new(Z::decode(decoder)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let value = Zeroizing::new(42u64);
        let text = ::json::to_string(&value);
        assert_eq!(text, "42");
        let decoded: Zeroizing<u64> = ::json::from_str(&text).unwrap();
        assert_eq!(*decoded, 42);
    }
}
