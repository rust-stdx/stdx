#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidDer,
    InvalidCertificate,
    InvalidSpki,
    InvalidPublicKey,
    Truncated,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDer => f.write_str("invalid DER encoding"),
            Self::InvalidCertificate => f.write_str("invalid X.509 certificate structure"),
            Self::InvalidSpki => f.write_str("invalid SubjectPublicKeyInfo"),
            Self::InvalidPublicKey => f.write_str("invalid public key BIT STRING"),
            Self::Truncated => f.write_str("truncated DER input"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Debug)]
struct Tlv<'a> {
    tag: u8,
    raw: &'a [u8],
    value: &'a [u8],
}

fn read_tlv(data: &[u8]) -> Result<Tlv<'_>, Error> {
    if data.is_empty() {
        return Err(Error::Truncated);
    }
    let tag = data[0];
    let consumed = 1;

    if data.len() <= consumed {
        return Err(Error::Truncated);
    }
    let len_byte = data[consumed];

    let (len, len_size) = if len_byte & 0x80 == 0 {
        (len_byte as usize, 1)
    } else {
        let num_bytes = (len_byte & 0x7f) as usize;
        if num_bytes == 0 || num_bytes > core::mem::size_of::<usize>() {
            return Err(Error::InvalidDer);
        }
        if data.len() <= consumed + num_bytes {
            return Err(Error::Truncated);
        }
        if num_bytes > 1 && data[consumed + 1] == 0 {
            return Err(Error::InvalidDer);
        }
        let mut l = 0usize;
        for i in 0..num_bytes {
            l = (l << 8) | data[consumed + 1 + i] as usize;
        }
        if l < 128 {
            return Err(Error::InvalidDer);
        }
        (l, 1 + num_bytes)
    };

    let start = consumed + len_size;
    let end = start.checked_add(len).ok_or(Error::InvalidDer)?;
    if end > data.len() {
        return Err(Error::Truncated);
    }

    Ok(Tlv {
        tag,
        raw: &data[..end],
        value: &data[start..end],
    })
}

fn skip_tlv(data: &[u8], offset: &mut usize) -> Result<(), Error> {
    let tlv = read_tlv(&data[*offset..])?;
    *offset += tlv.raw.len();
    Ok(())
}

/// Extract raw public key bytes from a SubjectPublicKeyInfo DER blob.
///
/// The SPKI DER contains a BIT STRING that holds the raw key material
/// (uncompressed point for P-256, 32-byte key for Ed25519, etc.).
/// This walks the DER structure to locate the BIT STRING and returns
/// its content (minus the leading unused-bits byte).
pub fn extract_key_from_spki(spki_der: &[u8]) -> Result<&[u8], Error> {
    let spki = read_tlv(spki_der)?;
    if spki.tag != 0x30 {
        return Err(Error::InvalidSpki);
    }
    let mut offset = 0;
    skip_tlv(spki.value, &mut offset)?;
    let inner = &spki.value[offset..];
    let bs = read_tlv(inner)?;
    if bs.tag != 0x03 {
        return Err(Error::InvalidSpki);
    }
    if bs.value.is_empty() {
        return Err(Error::InvalidPublicKey);
    }
    Ok(&bs.value[1..])
}

/// Extract the SubjectPublicKeyInfo DER from an X.509 certificate.
///
/// Returns the raw DER bytes of the `subjectPublicKeyInfo` field inside the
/// `tbsCertificate` SEQUENCE.
pub fn extract_spki_from_cert<'a>(cert_der: &'a [u8]) -> Result<&'a [u8], Error> {
    let cert = read_tlv(cert_der)?;
    if cert.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let tbs = read_tlv(cert.value)?;
    if tbs.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let inner = tbs.value;
    let mut offset = 0;

    if offset >= inner.len() {
        return Err(Error::InvalidCertificate);
    }

    if inner[offset] == 0xa0 {
        skip_tlv(inner, &mut offset)?;
    }

    skip_tag(inner, &mut offset, 0x02)?;
    skip_tag(inner, &mut offset, 0x30)?;
    skip_tag(inner, &mut offset, 0x30)?;
    skip_tag(inner, &mut offset, 0x30)?;
    skip_tag(inner, &mut offset, 0x30)?;

    if offset >= inner.len() || inner[offset] != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let spki = read_tlv(&inner[offset..])?;
    Ok(&inner[offset..offset + spki.raw.len()])
}

fn skip_tag(data: &[u8], offset: &mut usize, expected_tag: u8) -> Result<(), Error> {
    if *offset >= data.len() || data[*offset] != expected_tag {
        return Err(Error::InvalidCertificate);
    }
    skip_tlv(data, offset)
}

/// Extract raw public key bytes from an X.509 certificate (DER-encoded).
///
/// Returns the key material (e.g. uncompressed P-256 point, Ed25519 key
/// bytes) stripped of the unused-bits byte from the BIT STRING encoding.
#[cfg(feature = "alloc")]
pub fn extract_public_key_from_cert(cert_der: &[u8]) -> Result<Vec<u8>, Error> {
    let spki = extract_spki_from_cert(cert_der)?;
    let key = extract_key_from_spki(spki)?;
    Ok(key.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn parse_empty() {
        assert_eq!(read_tlv(b"").unwrap_err(), Error::Truncated);
    }

    #[test]
    fn parse_truncated_tag() {
        assert_eq!(read_tlv(b"\x30").unwrap_err(), Error::Truncated);
    }

    #[test]
    fn parse_truncated_value() {
        assert_eq!(read_tlv(b"\x30\x05").unwrap_err(), Error::Truncated);
    }

    #[test]
    fn parse_null() {
        let tlv = read_tlv(b"\x05\x00").unwrap();
        assert_eq!(tlv.tag, 0x05);
        assert!(tlv.value.is_empty());
        assert_eq!(tlv.raw, b"\x05\x00");
    }

    #[test]
    fn parse_integer() {
        let tlv = read_tlv(b"\x02\x03\x01\x00\x01").unwrap();
        assert_eq!(tlv.tag, 0x02);
        assert_eq!(tlv.value, b"\x01\x00\x01");
        assert_eq!(tlv.raw, b"\x02\x03\x01\x00\x01");
    }

    #[test]
    fn parse_long_form_length() {
        let payload = [0x42u8; 256];
        let mut raw = vec![0x04, 0x82, 0x01, 0x00];
        raw.extend_from_slice(&payload);
        let tlv = read_tlv(&raw).unwrap();
        assert_eq!(tlv.tag, 0x04);
        assert_eq!(tlv.value.len(), 256);
    }

    #[test]
    fn parse_sequence() {
        let tlv = read_tlv(b"\x30\x06\x02\x01\x01\x02\x01\x02").unwrap();
        assert_eq!(tlv.tag, 0x30);
        assert_eq!(tlv.value, b"\x02\x01\x01\x02\x01\x02");
    }

    #[test]
    fn spki_roundtrip() {
        let der = decode_hex(
            "3059301306072a8648ce3d020106082a8648ce3d030107034200\
             04deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef\
             deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef\
             deadbeefdeadbeefdeadbeefdeadbeef",
        );
        let key = extract_key_from_spki(&der).unwrap();
        assert_eq!(key.len(), 65);
        assert_eq!(key[0], 0x04);
    }

    #[test]
    fn extract_from_real_cert() {
        let der = include_bytes!("tests/p256_cert.der");
        let spki = extract_spki_from_cert(der).unwrap();
        let key = extract_key_from_spki(spki).unwrap();
        assert_eq!(key.len(), 65);
        assert_eq!(key[0], 0x04);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn extract_pk_alloc() {
        let der = include_bytes!("tests/p256_cert.der");
        let pk = extract_public_key_from_cert(der).unwrap();
        assert_eq!(pk.len(), 65);
        assert_eq!(pk[0], 0x04);
    }

    #[test]
    fn reject_invalid_spki() {
        assert_eq!(extract_key_from_spki(b"\x05\x00").unwrap_err(), Error::InvalidSpki);
    }

    #[test]
    fn reject_truncated_cert() {
        assert_eq!(extract_spki_from_cert(b"\x30\x03\x02\x01").unwrap_err(), Error::Truncated);
    }
}
