//! JSON support for IP networks.
//!
//! Networks use their canonical CIDR string, for example `"192.0.2.0/24"` or
//! `"2001:db8::/32"`.

use ::json::{Decode, Decoder, Encode, Encoder, ErrorKind, Write};

use crate::{IpNetwork, Ipv4Network, Ipv6Network};

impl Encode for Ipv4Network {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.string(&self.to_string())
    }
}

impl<'a> Decode<'a> for Ipv4Network {
    fn decode(decoder: &mut Decoder<'a>) -> ::json::Result<Self> {
        let text = decoder.decode_string()?;
        Ipv4Network::from_str(&text).map_err(|_| decoder.error(ErrorKind::Message("invalid IPv4 network")))
    }
}

impl Encode for Ipv6Network {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.string(&self.to_string())
    }
}

impl<'a> Decode<'a> for Ipv6Network {
    fn decode(decoder: &mut Decoder<'a>) -> ::json::Result<Self> {
        let text = decoder.decode_string()?;
        Ipv6Network::from_str(&text).map_err(|_| decoder.error(ErrorKind::Message("invalid IPv6 network")))
    }
}

impl Encode for IpNetwork {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.string(&self.to_string())
    }
}

impl<'a> Decode<'a> for IpNetwork {
    fn decode(decoder: &mut Decoder<'a>) -> ::json::Result<Self> {
        let text = decoder.decode_string()?;
        IpNetwork::from_str(&text).map_err(|_| decoder.error(ErrorKind::Message("invalid IP network")))
    }
}

use core::str::FromStr as _;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        for text in ["192.0.2.0/24", "2001:db8::/32"] {
            let network: IpNetwork = text.parse().unwrap();
            let json = ::json::to_string(&network);
            assert_eq!(json, format!("\"{text}\""));
            let decoded: IpNetwork = ::json::from_str(&json).unwrap();
            assert_eq!(decoded, network);
        }
    }
}
