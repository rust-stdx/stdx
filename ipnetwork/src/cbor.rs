//! CBOR support for IP networks.
//!
//! Networks use their canonical CIDR string, for example `"192.0.2.0/24"` or
//! `"2001:db8::/32"`.

use ::cbor::{Decode, Decoder, Encode, Encoder, Error, Write};

use crate::{IpNetwork, Ipv4Network, Ipv6Network};

impl Encode for Ipv4Network {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.text(&self.to_string())
    }
}

impl<'b> Decode<'b> for Ipv4Network {
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, Error> {
        let text = decoder.text()?;
        Ipv4Network::from_str(text).map_err(|_| Error::message("invalid IPv4 network").at(decoder.position()))
    }
}

impl Encode for Ipv6Network {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.text(&self.to_string())
    }
}

impl<'b> Decode<'b> for Ipv6Network {
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, Error> {
        let text = decoder.text()?;
        Ipv6Network::from_str(text).map_err(|_| Error::message("invalid IPv6 network").at(decoder.position()))
    }
}

impl Encode for IpNetwork {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.text(&self.to_string())
    }
}

impl<'b> Decode<'b> for IpNetwork {
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, Error> {
        let text = decoder.text()?;
        IpNetwork::from_str(text).map_err(|_| Error::message("invalid IP network").at(decoder.position()))
    }
}

use core::str::FromStr as _;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let network: IpNetwork = "192.0.2.0/24".parse().unwrap();
        let bytes = ::cbor::to_vec(&network);
        let decoded: IpNetwork = ::cbor::from_slice(&bytes).unwrap();
        assert_eq!(decoded, network);
    }
}
