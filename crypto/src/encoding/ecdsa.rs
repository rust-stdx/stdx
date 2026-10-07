//! DER encoding and decoding of ECDSA signatures.
//!
//! TLS and X.509 carry ECDSA signatures as the ASN.1 `ECDSA-Sig-Value`
//! structure:
//!
//! ```text
//! ECDSA-Sig-Value ::= SEQUENCE {
//!     r  INTEGER,
//!     s  INTEGER
//! }
//! ```
//!
//! The curve implementations in this crate operate on the fixed-width
//! `r || s` form instead (`[u8; 64]` for P-256, `[u8; 96]` for P-384). These
//! helpers convert between the two.

use alloc::vec::Vec;

use super::der::{DerError, Reader};

const TAG_SEQUENCE: u8 = 0x30;
const TAG_INTEGER: u8 = 0x02;

/// Errors returned while converting ECDSA signatures to or from DER.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcdsaError {
    /// The DER input was malformed.
    Der(DerError),
    /// The output buffer length was zero or not a multiple of two.
    InvalidOutputLength,
    /// An `INTEGER` encoded a negative value.
    Negative,
    /// An `INTEGER` did not fit in the fixed-width output.
    IntegerTooLarge,
}

impl core::fmt::Display for EcdsaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EcdsaError::Der(err) => write!(f, "{err}"),
            EcdsaError::InvalidOutputLength => write!(f, "output length must be a non-zero multiple of two"),
            EcdsaError::Negative => write!(f, "ECDSA integer is negative"),
            EcdsaError::IntegerTooLarge => write!(f, "ECDSA integer does not fit in the output"),
        }
    }
}

impl From<DerError> for EcdsaError {
    fn from(err: DerError) -> Self {
        EcdsaError::Der(err)
    }
}

impl core::error::Error for EcdsaError {}

fn push_length(out: &mut Vec<u8>, len: usize) {
    if len < 0x80 {
        out.push(len as u8);
        return;
    }
    let bytes = len.to_be_bytes();
    let first = bytes.iter().position(|b| *b != 0).unwrap_or(bytes.len() - 1);
    let significant = &bytes[first..];
    out.push(0x80 | significant.len() as u8);
    out.extend_from_slice(significant);
}

fn encode_integer(value: &[u8], out: &mut Vec<u8>) {
    // Strip leading zero bytes, keeping at least one byte for the value zero.
    let mut start = 0;
    while start + 1 < value.len() && value[start] == 0 {
        start += 1;
    }
    let value = &value[start..];

    // DER integers are signed, so a leading 0x00 is required when the most
    // significant bit is set.
    let pad = value[0] & 0x80 != 0;

    out.push(TAG_INTEGER);
    push_length(out, value.len() + pad as usize);
    if pad {
        out.push(0x00);
    }
    out.extend_from_slice(value);
}

/// Encodes the ECDSA values `r` and `s` (each big-endian, optionally with
/// leading zero bytes) as an `ECDSA-Sig-Value` DER structure.
pub fn encode_signature_der(r: &[u8], s: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(r.len() + s.len() + 8);
    encode_integer(r, &mut body);
    encode_integer(s, &mut body);

    let mut out = Vec::with_capacity(body.len() + 4);
    out.push(TAG_SEQUENCE);
    push_length(&mut out, body.len());
    out.extend_from_slice(&body);
    out
}

fn write_fixed(value: &[u8], out: &mut [u8]) -> Result<(), EcdsaError> {
    // DER integers are signed: a leading 0x00 is padding added to keep a
    // positive value's high bit clear. Strip it before checking the sign.
    let (negative, value) = match value {
        [0x00, rest @ ..] => (false, rest),
        [first, ..] => (first & 0x80 != 0, value),
        [] => (false, value),
    };
    if negative {
        return Err(EcdsaError::Negative);
    }
    if value.is_empty() {
        // Encodes zero; leave the output as all zeros.
        return Ok(());
    }
    if value.len() > out.len() {
        return Err(EcdsaError::IntegerTooLarge);
    }
    let start = out.len() - value.len();
    out[start..].copy_from_slice(value);
    Ok(())
}

/// Decodes an `ECDSA-Sig-Value` DER structure into `raw_out`.
///
/// `raw_out` receives `r || s`, each zero-padded to half of its length. It
/// must be non-empty and even.
///
/// Returns [`EcdsaError`] when the DER is malformed, when an integer is
/// negative or too large for its half of the output, or when the lengths are
/// inconsistent.
pub fn decode_signature_der(der: &[u8], raw_out: &mut [u8]) -> Result<(), EcdsaError> {
    if raw_out.is_empty() || raw_out.len() % 2 != 0 {
        return Err(EcdsaError::InvalidOutputLength);
    }

    let mut outer = Reader::new(der);
    let mut seq = outer.read_sequence()?;
    outer.finish()?;

    let r = seq.read_integer()?;
    let s = seq.read_integer()?;
    seq.finish()?;

    let (r_out, s_out) = raw_out.split_at_mut(raw_out.len() / 2);
    r_out.fill(0);
    s_out.fill(0);
    write_fixed(r, r_out)?;
    write_fixed(s, s_out)?;
    Ok(())
}

/// Decodes an `ECDSA-Sig-Value` DER structure into a fixed-width P-256
/// `r || s` signature.
pub fn decode_p256_signature_der(der: &[u8]) -> Result<[u8; 64], EcdsaError> {
    let mut raw = [0u8; 64];
    decode_signature_der(der, &mut raw)?;
    Ok(raw)
}

/// Decodes an `ECDSA-Sig-Value` DER structure into a fixed-width P-384
/// `r || s` signature.
pub fn decode_p384_signature_der(der: &[u8]) -> Result<[u8; 96], EcdsaError> {
    let mut raw = [0u8; 96];
    decode_signature_der(der, &mut raw)?;
    Ok(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_small_values() {
        let der = encode_signature_der(&[0x01], &[0x01]);
        assert_eq!(der, [0x30, 0x06, 0x02, 0x01, 0x01, 0x02, 0x01, 0x01]);
    }

    #[test]
    fn encodes_with_sign_padding() {
        let r = [0x80];
        let s = [0x7f];
        let der = encode_signature_der(&r, &s);
        assert_eq!(der, [0x30, 0x07, 0x02, 0x02, 0x00, 0x80, 0x02, 0x01, 0x7f]);
    }

    #[test]
    fn trims_leading_zeros() {
        let r = [0x00, 0x00, 0x01];
        let s = [0x00, 0x02];
        let der = encode_signature_der(&r, &s);
        assert_eq!(der, [0x30, 0x06, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02]);
    }

    #[test]
    fn encodes_zero() {
        let der = encode_signature_der(&[0x00], &[0x00]);
        assert_eq!(der, [0x30, 0x06, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00]);
    }

    #[test]
    fn round_trip_p256() {
        let mut raw = [0u8; 64];
        for (i, b) in raw.iter_mut().enumerate() {
            *b = i as u8;
        }
        // Give r and s a high bit so DER adds padding.
        raw[0] = 0xff;
        raw[32] = 0xf0;

        let der = encode_signature_der(&raw[..32], &raw[32..]);
        let decoded = decode_p256_signature_der(&der).unwrap();
        assert_eq!(decoded, raw);
    }

    #[test]
    fn round_trip_p384() {
        let mut raw = [0u8; 96];
        for (i, b) in raw.iter_mut().enumerate() {
            *b = (i * 7) as u8;
        }
        raw[0] = 0x80;
        raw[48] = 0xff;

        let der = encode_signature_der(&raw[..48], &raw[48..]);
        let decoded = decode_p384_signature_der(&der).unwrap();
        assert_eq!(decoded, raw);
    }

    #[test]
    fn decode_rejects_trailing_data() {
        let mut der = encode_signature_der(&[0x01], &[0x01]);
        der.push(0xff);
        assert!(matches!(
            decode_signature_der(&der, &mut [0u8; 64]),
            Err(EcdsaError::Der(DerError::TrailingData))
        ));
    }

    #[test]
    fn decode_rejects_negative_integer() {
        // INTEGER with the high bit set and no sign padding byte.
        let der = [0x30, 0x06, 0x02, 0x01, 0x80, 0x02, 0x01, 0x01];
        assert_eq!(decode_signature_der(&der, &mut [0u8; 64]), Err(EcdsaError::Negative));
    }

    #[test]
    fn decodes_positively_padded_integer() {
        // r = 0x80 padded as 00 80, s = 1.
        let der = [0x30, 0x07, 0x02, 0x02, 0x00, 0x80, 0x02, 0x01, 0x01];
        let mut raw = [0u8; 64];
        decode_signature_der(&der, &mut raw).unwrap();
        assert_eq!(raw[31], 0x80);
        assert_eq!(raw[63], 0x01);
    }

    #[test]
    fn decode_rejects_oversized_integer() {
        let value = [0x01u8; 33];
        let der = encode_signature_der(&value, &[0x01]);
        assert_eq!(decode_signature_der(&der, &mut [0u8; 64]), Err(EcdsaError::IntegerTooLarge));
    }

    #[test]
    fn decode_rejects_bad_output_length() {
        let der = encode_signature_der(&[0x01], &[0x01]);
        assert_eq!(decode_signature_der(&der, &mut [0u8; 63]), Err(EcdsaError::InvalidOutputLength));
    }
}
