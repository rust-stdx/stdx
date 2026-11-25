//! JSON support for [`Version`].
//!
//! A version is encoded as a string, for example `"1.2.3-alpha+build"`.

use alloc::string::ToString;

use ::json::{Decode, Decoder, Encode, Encoder, ErrorKind, Write};

use crate::{Version, parse};

impl Encode for Version<'_> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.string(&self.to_string())
    }
}

impl<'a> Decode<'a> for Version<'a> {
    fn decode(decoder: &mut Decoder<'a>) -> json::Result<Self> {
        let text = decoder.decode_str()?;
        parse(text).map_err(|_| decoder.error(ErrorKind::Message("invalid semantic version")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let version = parse("1.2.3-alpha+build").unwrap();
        let text = ::json::to_string(&version);
        assert_eq!(text, "\"1.2.3-alpha+build\"");
        let decoded: Version<'_> = ::json::from_str(&text).unwrap();
        assert_eq!(decoded, version);
    }
}
