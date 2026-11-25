//! CBOR support for [`Version`].
//!
//! A version is encoded as a text string, for example
//! `"1.2.3-alpha+build"`.

use alloc::string::ToString;

use ::cbor::{Decode, Decoder, Encode, Encoder, Error, Write};

use crate::{Version, parse};

impl Encode for Version<'_> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.text(&self.to_string())
    }
}

impl<'b> Decode<'b> for Version<'b> {
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, Error> {
        let text = decoder.text()?;
        parse(text).map_err(|_| Error::message("invalid semantic version").at(decoder.position()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let version = parse("1.2.3").unwrap();
        let bytes = ::cbor::to_vec(&version);
        let decoded: Version<'_> = ::cbor::from_slice(&bytes).unwrap();
        assert_eq!(decoded, version);
    }
}
