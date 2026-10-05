use super::der::{DerError, Reader};
use crate::{
    curve25519::ed25519,
    p256::{PUBLIC_KEY_UNCOMPRESSED_SIZE, SECRET_KEY_SIZE, SecretKey},
    p384,
};

const PKCS8_DER_LEN: usize = 138;

const EC_PUBLIC_KEY_OID: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
const SECP256R1_OID: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pkcs8Error {
    InvalidLength,
    InvalidSequence,
    InvalidVersion,
    InvalidAlgorithmIdentifier,
    InvalidOctetString,
    InvalidEcSecretKey,
    InvalidEcVersion,
    InvalidSecretKeyOctet,
    InvalidPublicKeyExplicit,
    InvalidPublicKeyBitString,
    InvalidPublicKeyPrefix,
    InvalidPublicKeyMismatch,
    KeyDerivationFailed,
    InvalidDer(DerError),
    UnsupportedAlgorithm,
    UnsupportedCurve,
    InvalidEd25519PrivateKey,
    InvalidMlDsaPrivateKey,
    MlDsaExpandedKeyNotSupported,
}

impl From<DerError> for Pkcs8Error {
    fn from(err: DerError) -> Self {
        Pkcs8Error::InvalidDer(err)
    }
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for Pkcs8Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Pkcs8Error::InvalidLength => write!(f, "invalid DER length"),
            Pkcs8Error::InvalidSequence => write!(f, "invalid outer SEQUENCE"),
            Pkcs8Error::InvalidVersion => write!(f, "invalid version"),
            Pkcs8Error::InvalidAlgorithmIdentifier => write!(f, "invalid AlgorithmIdentifier"),
            Pkcs8Error::InvalidOctetString => write!(f, "invalid OCTET STRING wrapping"),
            Pkcs8Error::InvalidEcSecretKey => write!(f, "invalid ECSecretKey SEQUENCE"),
            Pkcs8Error::InvalidEcVersion => write!(f, "invalid EC version"),
            Pkcs8Error::InvalidSecretKeyOctet => write!(f, "invalid private key OCTET STRING"),
            Pkcs8Error::InvalidPublicKeyExplicit => write!(f, "invalid public key [1] EXPLICIT"),
            Pkcs8Error::InvalidPublicKeyBitString => write!(f, "invalid public key BIT STRING"),
            Pkcs8Error::InvalidPublicKeyPrefix => write!(f, "invalid public key prefix"),
            Pkcs8Error::InvalidPublicKeyMismatch => write!(f, "embedded public key does not match private key"),
            Pkcs8Error::KeyDerivationFailed => write!(f, "key derivation failed"),
            Pkcs8Error::InvalidDer(err) => write!(f, "invalid DER: {err}"),
            Pkcs8Error::UnsupportedAlgorithm => write!(f, "unsupported private key algorithm"),
            Pkcs8Error::UnsupportedCurve => write!(f, "unsupported or missing named curve"),
            Pkcs8Error::InvalidEd25519PrivateKey => write!(f, "invalid Ed25519 private key"),
            Pkcs8Error::InvalidMlDsaPrivateKey => write!(f, "invalid ML-DSA private key"),
            Pkcs8Error::MlDsaExpandedKeyNotSupported => write!(
                f,
                "ML-DSA private keys stored in expanded form cannot be loaded; \
                 provide the key in seed form (RFC 9881)"
            ),
        }
    }
}

fn validate_fixed_prefix(der: &[u8]) -> Result<(), Pkcs8Error> {
    if der.len() != PKCS8_DER_LEN {
        return Err(Pkcs8Error::InvalidLength);
    }

    // Outer SEQUENCE: 30 81 87
    if der[0] != 0x30 || der[1] != 0x81 || der[2] != 0x87 {
        return Err(Pkcs8Error::InvalidSequence);
    }
    // INTEGER version=0: 02 01 00
    if der[3] != 0x02 || der[4] != 0x01 || der[5] != 0x00 {
        return Err(Pkcs8Error::InvalidVersion);
    }
    // AlgorithmIdentifier SEQUENCE: 30 13
    if der[6] != 0x30 || der[7] != 0x13 {
        return Err(Pkcs8Error::InvalidAlgorithmIdentifier);
    }
    // ecPublicKey OID: 06 07 <7 bytes>
    if der[8] != 0x06 || der[9] != 0x07 || der[10..17] != *EC_PUBLIC_KEY_OID {
        return Err(Pkcs8Error::InvalidAlgorithmIdentifier);
    }
    // secp256r1 OID: 06 08 <8 bytes>
    if der[17] != 0x06 || der[18] != 0x08 || der[19..27] != *SECP256R1_OID {
        return Err(Pkcs8Error::InvalidAlgorithmIdentifier);
    }
    // OCTET STRING: 04 6d (109 bytes)
    if der[27] != 0x04 || der[28] != 0x6d {
        return Err(Pkcs8Error::InvalidOctetString);
    }
    // ECSecretKey SEQUENCE: 30 6b
    if der[29] != 0x30 || der[30] != 0x6b {
        return Err(Pkcs8Error::InvalidEcSecretKey);
    }
    // EC version: 02 01 01
    if der[31] != 0x02 || der[32] != 0x01 || der[33] != 0x01 {
        return Err(Pkcs8Error::InvalidEcVersion);
    }
    // private key OCTET STRING: 04 20
    if der[34] != 0x04 || der[35] != 0x20 {
        return Err(Pkcs8Error::InvalidSecretKeyOctet);
    }
    // [1] EXPLICIT: a1 44
    if der[68] != 0xa1 || der[69] != 0x44 {
        return Err(Pkcs8Error::InvalidPublicKeyExplicit);
    }
    // BIT STRING: 03 42 00
    if der[70] != 0x03 || der[71] != 0x42 || der[72] != 0x00 {
        return Err(Pkcs8Error::InvalidPublicKeyBitString);
    }
    // public key must start with 0x04 (uncompressed)
    if der[73] != 0x04 {
        return Err(Pkcs8Error::InvalidPublicKeyPrefix);
    }

    Ok(())
}

static TEMPLATE: [u8; PKCS8_DER_LEN] = [
    // SecretKeyInfo SEQUENCE (135 bytes content)
    0x30, 0x81, 0x87, // version INTEGER 0
    0x02, 0x01, 0x00, // AlgorithmIdentifier SEQUENCE (19 bytes)
    0x30, 0x13, // ecPublicKey OID (1.2.840.10045.2.1)
    0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, // secp256r1 OID (1.2.840.10045.3.1.7)
    0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07,
    // OCTET STRING wrapping ECSecretKey (109 bytes)
    0x04, 0x6d, // ECSecretKey SEQUENCE (107 bytes)
    0x30, 0x6b, // EC version INTEGER 1
    0x02, 0x01, 0x01, // private key OCTET STRING (32 bytes) -- placeholder zeros
    0x04, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // [1] EXPLICIT public key wrapper (68 bytes)
    0xa1, 0x44, // BIT STRING with unused_bits=0, length=66 (65 bytes data + 1 byte unused_bits)
    0x03, 0x42, 0x00,
    // uncompressed public key (65 bytes = 0x04 || 32-byte X || 32-byte Y) -- placeholder zeros
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const PRIVATE_KEY_OFFSET: usize = 36;
const PUBLIC_KEY_OFFSET: usize = 73;

/// Encodes a P-256 secret key as PKCS#8 DER (the fixed 138-byte
/// `PrivateKeyInfo` template used by OpenSSL for `secp256r1`).
///
/// The embedded public key is derived from `key`, so the output always
/// contains a consistent private/public key pair.
pub fn encode_p256_pkcs8_der(key: &SecretKey) -> Result<[u8; PKCS8_DER_LEN], Pkcs8Error> {
    let public_key = key.public_key();
    let pub_bytes = public_key.to_bytes();
    let priv_bytes = key.to_bytes();

    let mut der = TEMPLATE;
    der[PUBLIC_KEY_OFFSET..PUBLIC_KEY_OFFSET + PUBLIC_KEY_UNCOMPRESSED_SIZE].copy_from_slice(&pub_bytes);
    der[PRIVATE_KEY_OFFSET..PRIVATE_KEY_OFFSET + SECRET_KEY_SIZE].copy_from_slice(&priv_bytes);

    Ok(der)
}

/// Decodes a P-256 secret key from PKCS#8 DER in the fixed 138-byte
/// `PrivateKeyInfo` template produced by [`encode_p256_pkcs8_der`].
///
/// The embedded public key is checked against the one derived from the
/// private scalar.
///
/// Returns [`Pkcs8Error`] when the input is not exactly the expected
/// template, when the private scalar is invalid, or when the embedded
/// public key does not correspond to the private key
/// ([`Pkcs8Error::InvalidPublicKeyMismatch`]).
pub fn decode_p256_pkcs8_der(der: &[u8]) -> Result<SecretKey, Pkcs8Error> {
    validate_fixed_prefix(der)?;

    let mut private_key = [0u8; SECRET_KEY_SIZE];
    private_key.copy_from_slice(&der[PRIVATE_KEY_OFFSET..PRIVATE_KEY_OFFSET + SECRET_KEY_SIZE]);

    let mut public_key = [0u8; PUBLIC_KEY_UNCOMPRESSED_SIZE];
    public_key.copy_from_slice(&der[PUBLIC_KEY_OFFSET..PUBLIC_KEY_OFFSET + PUBLIC_KEY_UNCOMPRESSED_SIZE]);

    let key = SecretKey::from_bytes(&private_key).map_err(|_| Pkcs8Error::KeyDerivationFailed)?;

    if key.public_key().to_bytes() != public_key {
        return Err(Pkcs8Error::InvalidPublicKeyMismatch);
    }

    Ok(key)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic EC and Ed25519 private key parsing
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Object identifier for `id-ecPublicKey` (1.2.840.10045.2.1).
pub const OID_EC_PUBLIC_KEY: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
/// Object identifier for `secp256r1` / NIST P-256 (1.2.840.10045.3.1.7).
pub const OID_SECP256R1: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];
/// Object identifier for `secp384r1` / NIST P-384 (1.3.132.0.34).
pub const OID_SECP384R1: &[u8] = &[0x2b, 0x81, 0x04, 0x00, 0x22];
/// Object identifier for Ed25519 (1.3.101.112).
pub const OID_ED25519: &[u8] = &[0x2b, 0x65, 0x70];
/// Object identifier for `rsaEncryption` (1.2.840.113549.1.1.1).
pub const OID_RSA_ENCRYPTION: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01];

/// A supported elliptic-curve private key parsed from DER.
pub enum EcPrivateKey {
    /// NIST P-256 (secp256r1).
    P256(crate::p256::SecretKey),
    /// NIST P-384 (secp384r1).
    P384(crate::p384::SecretKey),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Curve {
    P256,
    P384,
}

fn curve_from_oid(oid: &[u8]) -> Result<Curve, Pkcs8Error> {
    if oid == OID_SECP256R1 {
        Ok(Curve::P256)
    } else if oid == OID_SECP384R1 {
        Ok(Curve::P384)
    } else {
        Err(Pkcs8Error::UnsupportedCurve)
    }
}

fn build_ec_key(curve: Curve, scalar: &[u8]) -> Result<EcPrivateKey, Pkcs8Error> {
    match curve {
        Curve::P256 => {
            let scalar: [u8; crate::p256::SECRET_KEY_SIZE] =
                scalar.try_into().map_err(|_| Pkcs8Error::InvalidEcSecretKey)?;
            let key = crate::p256::SecretKey::from_bytes(&scalar).map_err(|_| Pkcs8Error::KeyDerivationFailed)?;
            Ok(EcPrivateKey::P256(key))
        }
        Curve::P384 => {
            let scalar: [u8; crate::p384::SECRET_KEY_SIZE] =
                scalar.try_into().map_err(|_| Pkcs8Error::InvalidEcSecretKey)?;
            let key = crate::p384::SecretKey::from_bytes(&scalar).map_err(|_| Pkcs8Error::KeyDerivationFailed)?;
            Ok(EcPrivateKey::P384(key))
        }
    }
}

/// Parses an RFC 5915 `ECPrivateKey`, returning the named curve (when the
/// optional `[0] parameters` field is present) and the private scalar.
fn parse_sec1(der: &[u8]) -> Result<(Option<Curve>, &[u8]), Pkcs8Error> {
    let mut outer = Reader::new(der);
    let mut seq = outer.read_sequence()?;
    outer.finish()?;

    let version = seq.read_integer()?;
    if version != [0x01] {
        return Err(Pkcs8Error::InvalidEcVersion);
    }
    let scalar = seq.read_octet_string()?;

    let mut curve = None;
    while !seq.is_empty() {
        match seq.peek_tag()? {
            0xa0 => {
                let mut params = seq.read_explicit(0xa0)?;
                let oid = params.read_oid()?;
                params.finish()?;
                let parsed = curve_from_oid(oid)?;
                if let Some(previous) = curve {
                    if previous != parsed {
                        return Err(Pkcs8Error::InvalidAlgorithmIdentifier);
                    }
                }
                curve = Some(parsed);
            }
            0xa1 => {
                let mut public = seq.read_explicit(0xa1)?;
                let _ = public.read_bit_string()?;
                public.finish()?;
            }
            _ => return Err(Pkcs8Error::InvalidEcSecretKey),
        }
    }

    Ok((curve, scalar))
}

/// Decodes a SEC1 / RFC 5915 `ECPrivateKey` DER into a P-256 or P-384 key.
///
/// The named curve must be present in the `[0] parameters` field; the optional
/// embedded public key is ignored.
///
/// Returns [`Pkcs8Error`] when the structure is malformed, the curve is not
/// P-256 or P-384, or the scalar is not a valid private key.
pub fn decode_ec_sec1_der(der: &[u8]) -> Result<EcPrivateKey, Pkcs8Error> {
    let (curve, scalar) = parse_sec1(der)?;
    let curve = curve.ok_or(Pkcs8Error::UnsupportedCurve)?;
    build_ec_key(curve, scalar)
}

/// Decodes a PKCS#8 `PrivateKeyInfo` DER containing a P-256 or P-384 key.
///
/// The curve is taken from the `AlgorithmIdentifier`; when the inner SEC1
/// structure also carries a curve, both must agree.
///
/// Returns [`Pkcs8Error`] when the structure is malformed, the algorithm is
/// not `id-ecPublicKey`, the curve is not P-256 or P-384, or the scalar is
/// not a valid private key.
pub fn decode_ec_pkcs8_der(der: &[u8]) -> Result<EcPrivateKey, Pkcs8Error> {
    let mut outer = Reader::new(der);
    let mut seq = outer.read_sequence()?;
    outer.finish()?;

    let version = seq.read_integer()?;
    if version != [0x00] {
        return Err(Pkcs8Error::InvalidVersion);
    }

    let mut algorithm = seq.read_sequence()?;
    let algorithm_oid = algorithm.read_oid()?;
    if algorithm_oid != OID_EC_PUBLIC_KEY {
        return Err(Pkcs8Error::UnsupportedAlgorithm);
    }
    let curve_oid = algorithm.read_oid()?;
    algorithm.finish()?;
    let outer_curve = curve_from_oid(curve_oid)?;

    let private_key = seq.read_octet_string()?;

    // Skip the optional attributes field.
    while !seq.is_empty() {
        match seq.peek_tag()? {
            0xa0 => {
                let _attributes = seq.read_explicit(0xa0)?;
            }
            _ => return Err(Pkcs8Error::InvalidDer(DerError::UnexpectedTag)),
        }
    }

    let (inner_curve, scalar) = parse_sec1(private_key)?;
    if let Some(inner_curve) = inner_curve {
        if inner_curve != outer_curve {
            return Err(Pkcs8Error::InvalidAlgorithmIdentifier);
        }
    }
    build_ec_key(outer_curve, scalar)
}

/// Decodes a PKCS#8 `PrivateKeyInfo` DER containing an Ed25519 key.
///
/// Returns the key's 32-byte seed via [`ed25519::SecretKey`].
///
/// Both the RFC 8410 nested `OCTET STRING` form and the bare 32-byte form are
/// accepted. An optional `[1]` public key field is ignored.
///
/// Returns [`Pkcs8Error`] when the structure is malformed, the algorithm is
/// not Ed25519, or the seed has the wrong length.
pub fn decode_ed25519_pkcs8_der(der: &[u8]) -> Result<ed25519::SecretKey, Pkcs8Error> {
    let mut outer = Reader::new(der);
    let mut seq = outer.read_sequence()?;
    outer.finish()?;

    let version = seq.read_integer()?;
    if version != [0x00] && version != [0x01] {
        return Err(Pkcs8Error::InvalidVersion);
    }

    let mut algorithm = seq.read_sequence()?;
    let algorithm_oid = algorithm.read_oid()?;
    if algorithm_oid != OID_ED25519 {
        return Err(Pkcs8Error::UnsupportedAlgorithm);
    }
    // Ed25519 AlgorithmIdentifier has no parameters.
    algorithm.finish()?;

    let private_key = seq.read_octet_string()?;

    while !seq.is_empty() {
        match seq.peek_tag()? {
            0xa0 => {
                let _attributes = seq.read_explicit(0xa0)?;
            }
            0xa1 => {
                let mut public = seq.read_explicit(0xa1)?;
                let _ = public.read_bit_string()?;
                public.finish()?;
            }
            _ => return Err(Pkcs8Error::InvalidDer(DerError::UnexpectedTag)),
        }
    }

    let seed = if private_key.len() == ed25519::SECRET_KEY_SIZE {
        private_key
    } else {
        let mut nested = Reader::new(private_key);
        let seed = nested.read_octet_string()?;
        nested.finish()?;
        seed
    };

    let seed: [u8; ed25519::SECRET_KEY_SIZE] = seed.try_into().map_err(|_| Pkcs8Error::InvalidEd25519PrivateKey)?;
    Ok(ed25519::SecretKey::from_bytes(&seed))
}

/// Reports whether `der` is a PKCS#8 `PrivateKeyInfo` whose
/// `AlgorithmIdentifier` is `rsaEncryption` (1.2.840.113549.1.1.1).
///
/// This crate does not implement RSA private keys. The predicate lets callers
/// recognise an RSA key and report a clear "unsupported" error rather than a
/// generic parse failure. It only inspects the algorithm identifier, so it
/// returns `false` for malformed input.
pub fn is_rsa_pkcs8_der(der: &[u8]) -> bool {
    fn inner(der: &[u8]) -> Result<bool, Pkcs8Error> {
        let mut outer = Reader::new(der);
        let mut seq = outer.read_sequence()?;
        outer.finish()?;

        let _version = seq.read_integer()?;
        let mut algorithm = seq.read_sequence()?;
        // RSA's AlgorithmIdentifier carries NULL parameters, so only the OID
        // needs to be read; the rest of the structure is not inspected.
        let algorithm_oid = algorithm.read_oid()?;

        Ok(algorithm_oid == OID_RSA_ENCRYPTION)
    }

    inner(der).unwrap_or(false)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ML-DSA private keys (RFC 9881)
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Object identifier for `id-ml-dsa-44` (2.16.840.1.101.3.4.3.17).
pub const OID_ML_DSA_44: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x03, 0x11];
/// Object identifier for `id-ml-dsa-65` (2.16.840.1.101.3.4.3.18).
pub const OID_ML_DSA_65: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x03, 0x12];
/// Object identifier for `id-ml-dsa-87` (2.16.840.1.101.3.4.3.19).
pub const OID_ML_DSA_87: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x03, 0x13];

/// A supported ML-DSA private key parsed from DER.
///
/// Only the seed is retained: the expanded key material is recomputed from it,
/// so keys stored purely in `expandedKey` form cannot be represented.
///
/// The keys are large, so each variant is boxed.
pub enum MlDsaPrivateKey {
    /// ML-DSA-44.
    Dsa44(Box<crate::mldsa::MlDsa44SecretKey>),
    /// ML-DSA-65.
    Dsa65(Box<crate::mldsa::MlDsa65SecretKey>),
    /// ML-DSA-87.
    Dsa87(Box<crate::mldsa::MlDsa87SecretKey>),
}

/// Extracts the 32-byte seed from the RFC 9881 `ML-DSA-XX-PrivateKey` CHOICE.
///
/// The choice is dispatched by tag, as required by RFC 9881 section 6:
/// `[0]` (`0x80`) is the seed, `OCTET STRING` (`0x04`) is the expanded key,
/// and `SEQUENCE` (`0x30`) is the `both` form (seed followed by the expanded
/// key). An explicit `[0]` (`0xa0`) wrapping is also accepted.
fn parse_mldsa_private_key_seed(der: &[u8]) -> Result<[u8; 32], Pkcs8Error> {
    let mut reader = Reader::new(der);
    match reader.peek_tag()? {
        // seed [0] OCTET STRING, implicitly tagged.
        0x80 => {
            let seed = reader.read_expected(0x80)?;
            reader.finish()?;
            seed.try_into().map_err(|_| Pkcs8Error::InvalidMlDsaPrivateKey)
        }
        // seed [0] OCTET STRING, explicitly tagged (lenient interoperability).
        0xa0 => {
            let mut explicit = reader.read_explicit(0xa0)?;
            reader.finish()?;
            let seed = explicit.read_octet_string()?;
            explicit.finish()?;
            seed.try_into().map_err(|_| Pkcs8Error::InvalidMlDsaPrivateKey)
        }
        // expandedKey only: the seed is not recoverable.
        0x04 => Err(Pkcs8Error::MlDsaExpandedKeyNotSupported),
        // both: SEQUENCE { seed OCTET STRING (32), expandedKey OCTET STRING }.
        0x30 => {
            let mut both = reader.read_sequence()?;
            reader.finish()?;
            let seed = both.read_octet_string()?;
            let _expanded = both.read_octet_string()?;
            both.finish()?;
            seed.try_into().map_err(|_| Pkcs8Error::InvalidMlDsaPrivateKey)
        }
        _ => Err(Pkcs8Error::InvalidMlDsaPrivateKey),
    }
}

/// Decodes a PKCS#8 `PrivateKeyInfo` DER containing an ML-DSA private key.
///
/// Supports ML-DSA-44, ML-DSA-65 and ML-DSA-87 in the seed and `both` forms
/// defined by RFC 9881. If the optional `[1]` public key is present it is
/// checked against the key derived from the seed. When the `both` form is
/// used, the seed is retained and the embedded expanded key is not compared
/// against it (RFC 9881's seed-consistency check is optional).
///
/// Returns [`Pkcs8Error::MlDsaExpandedKeyNotSupported`] when the key is stored
/// purely in expanded form (the seed cannot be recovered), and other
/// [`Pkcs8Error`] variants when the structure is malformed or the algorithm is
/// not ML-DSA.
pub fn decode_mldsa_pkcs8_der(der: &[u8]) -> Result<MlDsaPrivateKey, Pkcs8Error> {
    let mut outer = Reader::new(der);
    let mut seq = outer.read_sequence()?;
    outer.finish()?;

    let version = seq.read_integer()?;
    if version != [0x00] && version != [0x01] {
        return Err(Pkcs8Error::InvalidVersion);
    }

    let mut algorithm = seq.read_sequence()?;
    let oid = algorithm.read_oid()?;
    // ML-DSA AlgorithmIdentifiers carry no parameters.
    algorithm.finish()?;

    let private_key = seq.read_octet_string()?;

    let mut public_key = None;
    while !seq.is_empty() {
        match seq.peek_tag()? {
            0xa0 => {
                let _attributes = seq.read_explicit(0xa0)?;
            }
            0xa1 => {
                let mut public = seq.read_explicit(0xa1)?;
                public_key = Some(public.read_bit_string()?);
                public.finish()?;
            }
            _ => return Err(Pkcs8Error::InvalidDer(DerError::UnexpectedTag)),
        }
    }

    let seed = parse_mldsa_private_key_seed(private_key)?;

    match oid {
        OID_ML_DSA_44 => {
            let key = crate::mldsa::MlDsa44SecretKey::new(&seed);
            check_mldsa_public_key(public_key, &key.public_key().to_bytes())?;
            Ok(MlDsaPrivateKey::Dsa44(Box::new(key)))
        }
        OID_ML_DSA_65 => {
            let key = crate::mldsa::MlDsa65SecretKey::new(&seed);
            check_mldsa_public_key(public_key, &key.public_key().to_bytes())?;
            Ok(MlDsaPrivateKey::Dsa65(Box::new(key)))
        }
        OID_ML_DSA_87 => {
            let key = crate::mldsa::MlDsa87SecretKey::new(&seed);
            check_mldsa_public_key(public_key, &key.public_key().to_bytes())?;
            Ok(MlDsaPrivateKey::Dsa87(Box::new(key)))
        }
        _ => Err(Pkcs8Error::UnsupportedAlgorithm),
    }
}

fn check_mldsa_public_key(public_key: Option<&[u8]>, derived: &[u8]) -> Result<(), Pkcs8Error> {
    match public_key {
        Some(public_key) if public_key != derived => Err(Pkcs8Error::InvalidPublicKeyMismatch),
        _ => Ok(()),
    }
}

/// Encodes an ML-DSA-44 secret key as PKCS#8 DER in the RFC 9881 seed format.
pub fn encode_mldsa44_pkcs8_der(key: &crate::mldsa::MlDsa44SecretKey) -> [u8; ML_DSA_SEED_PKCS8_LEN] {
    encode_mldsa_seed_pkcs8(OID_ML_DSA_44, key.seed())
}

/// Encodes an ML-DSA-65 secret key as PKCS#8 DER in the RFC 9881 seed format.
pub fn encode_mldsa65_pkcs8_der(key: &crate::mldsa::MlDsa65SecretKey) -> [u8; ML_DSA_SEED_PKCS8_LEN] {
    encode_mldsa_seed_pkcs8(OID_ML_DSA_65, key.seed())
}

/// Encodes an ML-DSA-87 secret key as PKCS#8 DER in the RFC 9881 seed format.
pub fn encode_mldsa87_pkcs8_der(key: &crate::mldsa::MlDsa87SecretKey) -> [u8; ML_DSA_SEED_PKCS8_LEN] {
    encode_mldsa_seed_pkcs8(OID_ML_DSA_87, key.seed())
}

const ML_DSA_SEED_PKCS8_LEN: usize = 54;

fn encode_mldsa_seed_pkcs8(oid: &[u8], seed: &[u8; 32]) -> [u8; ML_DSA_SEED_PKCS8_LEN] {
    debug_assert_eq!(oid.len(), 9);
    let mut der = [0u8; ML_DSA_SEED_PKCS8_LEN];
    der[0] = 0x30;
    der[1] = 0x34;
    der[2] = 0x02;
    der[3] = 0x01;
    der[4] = 0x00;
    der[5] = 0x30;
    der[6] = 0x0b;
    der[7] = 0x06;
    der[8] = 0x09;
    der[9..18].copy_from_slice(oid);
    der[18] = 0x04;
    der[19] = 0x22;
    der[20] = 0x80;
    der[21] = 0x20;
    der[22..54].copy_from_slice(seed);
    der
}

const PKCS8_P384_DER_LEN: usize = 185;
const PKCS8_ED25519_DER_LEN: usize = 48;

static P384_TEMPLATE: [u8; PKCS8_P384_DER_LEN] = [
    // PrivateKeyInfo SEQUENCE (182 bytes content)
    0x30, 0x81, 0xb6, // version INTEGER 0
    0x02, 0x01, 0x00, // AlgorithmIdentifier SEQUENCE (16 bytes content)
    0x30, 0x10, // ecPublicKey OID (1.2.840.10045.2.1)
    0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, // secp384r1 OID (1.3.132.0.34)
    0x06, 0x05, 0x2b, 0x81, 0x04, 0x00, 0x22, // privateKey OCTET STRING (158 bytes content)
    0x04, 0x81, 0x9e, // ECPrivateKey SEQUENCE (155 bytes content)
    0x30, 0x81, 0x9b, // EC version INTEGER 1
    0x02, 0x01, 0x01, // private key OCTET STRING (48 bytes) -- placeholder zeros
    0x04, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // [1] EXPLICIT public key wrapper
    0xa1, 0x64, // BIT STRING with unused_bits=0, length=98 (97 bytes data + 1 byte unused_bits)
    0x03, 0x62, 0x00, // uncompressed public key (97 bytes = 0x04 || 48-byte X || 48-byte Y) -- placeholder zeros
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00,
];

static ED25519_TEMPLATE: [u8; PKCS8_ED25519_DER_LEN] = [
    // PrivateKeyInfo SEQUENCE (44 bytes content)
    0x30, 0x2e, // version INTEGER 0
    0x02, 0x01, 0x00, // AlgorithmIdentifier SEQUENCE (5 bytes content)
    0x30, 0x05, // id-Ed25519 OID (1.3.101.112)
    0x06, 0x03, 0x2b, 0x65, 0x70, // privateKey OCTET STRING (34 bytes content)
    0x04, 0x22, // CurvePrivateKey OCTET STRING (32 bytes) -- placeholder zeros
    0x04, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const P384_PRIVATE_KEY_OFFSET: usize = 35;
const P384_PUBLIC_KEY_OFFSET: usize = 88;
const ED25519_SEED_OFFSET: usize = 16;

/// Encodes a P-384 secret key as PKCS#8 DER in the fixed 185-byte
/// `PrivateKeyInfo` form used by OpenSSL for `secp384r1`.
///
/// The embedded public key is derived from `key`, so the output always
/// contains a consistent private/public key pair.
pub fn encode_p384_pkcs8_der(key: &p384::SecretKey) -> [u8; PKCS8_P384_DER_LEN] {
    let mut der = P384_TEMPLATE;
    der[P384_PRIVATE_KEY_OFFSET..P384_PRIVATE_KEY_OFFSET + p384::SECRET_KEY_SIZE].copy_from_slice(&key.to_bytes());
    der[P384_PUBLIC_KEY_OFFSET..P384_PUBLIC_KEY_OFFSET + p384::PUBLIC_KEY_UNCOMPRESSED_SIZE]
        .copy_from_slice(&key.public_key().to_bytes());
    der
}

/// Encodes an Ed25519 secret key as PKCS#8 DER in the fixed 48-byte
/// `PrivateKeyInfo` form defined by RFC 8410.
pub fn encode_ed25519_pkcs8_der(key: &ed25519::SecretKey) -> [u8; PKCS8_ED25519_DER_LEN] {
    let mut der = ED25519_TEMPLATE;
    der[ED25519_SEED_OFFSET..ED25519_SEED_OFFSET + ed25519::SECRET_KEY_SIZE].copy_from_slice(&key.to_bytes());
    der
}

#[cfg(test)]
mod tests {
    use super::*;

    // Known test vector from acme crate:
    // Private key: 255582fd0cce4c24b9bedb09a76206f940dcf1c7437dea0ab71499c5ace733e9
    // Public key:  041e9c23e81a03c54dd3c9ed52cb2ade7a4713e5613d703579c7739e0f132060dcbc4a687d3eb09917d262fc2f23c476c7cbfcecf84f11e458b246ad756d3617c7
    const TEST_PRIVATE_KEY: [u8; 32] = [
        0x25, 0x55, 0x82, 0xfd, 0x0c, 0xce, 0x4c, 0x24, 0xb9, 0xbe, 0xdb, 0x09, 0xa7, 0x62, 0x06, 0xf9, 0x40, 0xdc,
        0xf1, 0xc7, 0x43, 0x7d, 0xea, 0x0a, 0xb7, 0x14, 0x99, 0xc5, 0xac, 0xe7, 0x33, 0xe9,
    ];

    const TEST_PUBLIC_KEY: [u8; 65] = [
        0x04, 0x1e, 0x9c, 0x23, 0xe8, 0x1a, 0x03, 0xc5, 0x4d, 0xd3, 0xc9, 0xed, 0x52, 0xcb, 0x2a, 0xde, 0x7a, 0x47,
        0x13, 0xe5, 0x61, 0x3d, 0x70, 0x35, 0x79, 0xc7, 0x73, 0x9e, 0x0f, 0x13, 0x20, 0x60, 0xdc, 0xbc, 0x4a, 0x68,
        0x7d, 0x3e, 0xb0, 0x99, 0x17, 0xd2, 0x62, 0xfc, 0x2f, 0x23, 0xc4, 0x76, 0xc7, 0xcb, 0xfc, 0xec, 0xf8, 0x4f,
        0x11, 0xe4, 0x58, 0xb2, 0x46, 0xad, 0x75, 0x6d, 0x36, 0x17, 0xc7,
    ];

    fn decode_hex(hex_str: &str) -> Vec<u8> {
        (0..hex_str.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap())
            .collect()
    }

    const TEST_DER_HEX: &str = "308187020100301306072a8648ce3d020106082a8648ce3d030107046d306b0201010420255582fd0cce4c24b9bedb09a76206f940dcf1c7437dea0ab71499c5ace733e9a144034200041e9c23e81a03c54dd3c9ed52cb2ade7a4713e5613d703579c7739e0f132060dcbc4a687d3eb09917d262fc2f23c476c7cbfcecf84f11e458b246ad756d3617c7";

    #[test]
    fn encode_round_trip() {
        let key = SecretKey::from_bytes(&TEST_PRIVATE_KEY).unwrap();
        let der = encode_p256_pkcs8_der(&key).unwrap();
        let decoded = decode_p256_pkcs8_der(&der).unwrap();
        assert_eq!(decoded.to_bytes(), TEST_PRIVATE_KEY);
        assert_eq!(decoded.public_key().to_bytes(), TEST_PUBLIC_KEY);
    }

    #[test]
    fn decode_known_vector() {
        let der_bytes = decode_hex(TEST_DER_HEX);
        let key = decode_p256_pkcs8_der(&der_bytes).unwrap();
        assert_eq!(key.to_bytes(), TEST_PRIVATE_KEY);
        assert_eq!(key.public_key().to_bytes(), TEST_PUBLIC_KEY);
    }

    #[test]
    fn encode_matches_known_der() {
        let key = SecretKey::from_bytes(&TEST_PRIVATE_KEY).unwrap();
        let der = encode_p256_pkcs8_der(&key).unwrap();
        let expected = decode_hex(TEST_DER_HEX);
        assert_eq!(der.as_slice(), expected.as_slice());
    }

    #[test]
    fn detects_rsa_pkcs8() {
        // A minimal PrivateKeyInfo with the rsaEncryption OID and NULL params.
        let rsa = decode_hex("3016020100300d06092a864886f70d010101050004023000");
        assert!(is_rsa_pkcs8_der(&rsa));

        // A P-256 key is not classified as RSA.
        assert!(!is_rsa_pkcs8_der(&decode_hex(TEST_DER_HEX)));

        // Malformed input is not classified as RSA.
        assert!(!is_rsa_pkcs8_der(&[0x30, 0x00]));
    }

    #[test]
    fn decode_too_short() {
        assert!(matches!(decode_p256_pkcs8_der(&[0u8; 100]), Err(Pkcs8Error::InvalidLength)));
    }

    #[test]
    fn decode_too_long() {
        assert!(matches!(decode_p256_pkcs8_der(&[0u8; 200]), Err(Pkcs8Error::InvalidLength)));
    }

    #[test]
    fn decode_bad_sequence() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[0] = 0x31;
        assert!(matches!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidSequence)));
    }

    #[test]
    fn decode_bad_version() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[5] = 0x01;
        assert!(matches!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidVersion)));
    }

    #[test]
    fn decode_bad_algo_identifier() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[10] = 0x00;
        assert!(matches!(
            decode_p256_pkcs8_der(&der),
            Err(Pkcs8Error::InvalidAlgorithmIdentifier)
        ));
    }

    #[test]
    fn decode_bad_ec_version() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[33] = 0x02;
        assert!(matches!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidEcVersion)));
    }

    #[test]
    fn decode_missing_public_key() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[68] = 0x00;
        assert!(matches!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidPublicKeyExplicit)));
    }

    #[test]
    fn decode_bad_public_key_prefix() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[73] = 0x02;
        assert!(matches!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidPublicKeyPrefix)));
    }

    #[test]
    fn decode_mismatched_public_key() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[80] ^= 0xff;
        assert!(matches!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidPublicKeyMismatch)));
    }

    #[test]
    fn encode_round_trip_random_keys() {
        for _ in 0..10 {
            let priv_key: [u8; 32] = rand::random();
            let key = match SecretKey::from_bytes(&priv_key) {
                Ok(k) => k,
                Err(_) => continue,
            };
            let der = encode_p256_pkcs8_der(&key).unwrap();
            let decoded = decode_p256_pkcs8_der(&der).unwrap();
            assert_eq!(decoded.to_bytes(), key.to_bytes());
            assert_eq!(decoded.public_key().to_bytes().len(), 65);
            assert_eq!(decoded.public_key().to_bytes()[0], 0x04);
        }
    }

    #[test]
    fn encode_rejects_invalid_key() {
        let invalid = [0u8; 32];
        assert!(SecretKey::from_bytes(&invalid).is_err());
    }

    // Generated with OpenSSL 3.5 (`genpkey -algorithm EC -pkeyopt
    // ec_paramgen_curve:P-384`).
    const P384_PRIVATE_KEY_HEX: &str =
        "78eff649cf18e8211af8581aa79c9f3271decb2d416a18b96c4efe305ccba836f34b68cb2138798bf3d07ee6ffb306dc";
    const P384_PUBLIC_KEY_HEX: &str = "049fd48843177dcffdded82c7946bc174bbc14e672bb3f8dfad3b27814b686eb366a2ccdd8e898b397355ae71a09bad802c92f5a4d11a844e28dfa763b5450ff0ae67fa555698cb02121d0457acf44d9ee5d09307fdf1b58f0089d6b43e32c9500";
    const P384_PKCS8_DER_HEX: &str = "3081b6020100301006072a8648ce3d020106052b8104002204819e30819b020101043078eff649cf18e8211af8581aa79c9f3271decb2d416a18b96c4efe305ccba836f34b68cb2138798bf3d07ee6ffb306dca164036200049fd48843177dcffdded82c7946bc174bbc14e672bb3f8dfad3b27814b686eb366a2ccdd8e898b397355ae71a09bad802c92f5a4d11a844e28dfa763b5450ff0ae67fa555698cb02121d0457acf44d9ee5d09307fdf1b58f0089d6b43e32c9500";
    const P384_SEC1_DER_HEX: &str = "3081a4020101043078eff649cf18e8211af8581aa79c9f3271decb2d416a18b96c4efe305ccba836f34b68cb2138798bf3d07ee6ffb306dca00706052b81040022a164036200049fd48843177dcffdded82c7946bc174bbc14e672bb3f8dfad3b27814b686eb366a2ccdd8e898b397355ae71a09bad802c92f5a4d11a844e28dfa763b5450ff0ae67fa555698cb02121d0457acf44d9ee5d09307fdf1b58f0089d6b43e32c9500";

    // Generated with OpenSSL 3.5 (`genpkey -algorithm ED25519`).
    const ED25519_SEED_HEX: &str = "cef4aba318713bb028bb60247b458aea67ce12f9fa394afbc185b298ffc98974";
    const ED25519_DER_HEX: &str =
        "302e020100300506032b657004220420cef4aba318713bb028bb60247b458aea67ce12f9fa394afbc185b298ffc98974";

    #[test]
    fn decode_ec_pkcs8_p256_known_vector() {
        let der = decode_hex(TEST_DER_HEX);
        match decode_ec_pkcs8_der(&der).unwrap() {
            EcPrivateKey::P256(key) => {
                assert_eq!(key.to_bytes(), TEST_PRIVATE_KEY);
                assert_eq!(key.public_key().to_bytes(), TEST_PUBLIC_KEY);
            }
            EcPrivateKey::P384(_) => panic!("expected P-256"),
        }
    }

    #[test]
    fn decode_ec_pkcs8_p384_known_vector() {
        let der = decode_hex(P384_PKCS8_DER_HEX);
        match decode_ec_pkcs8_der(&der).unwrap() {
            EcPrivateKey::P384(key) => {
                assert_eq!(key.to_bytes().as_slice(), decode_hex(P384_PRIVATE_KEY_HEX));
                assert_eq!(key.public_key().to_bytes().as_slice(), decode_hex(P384_PUBLIC_KEY_HEX));
            }
            EcPrivateKey::P256(_) => panic!("expected P-384"),
        }
    }

    #[test]
    fn decode_ec_sec1_p384_known_vector() {
        let der = decode_hex(P384_SEC1_DER_HEX);
        match decode_ec_sec1_der(&der).unwrap() {
            EcPrivateKey::P384(key) => {
                assert_eq!(key.to_bytes().as_slice(), decode_hex(P384_PRIVATE_KEY_HEX));
                assert_eq!(key.public_key().to_bytes().as_slice(), decode_hex(P384_PUBLIC_KEY_HEX));
            }
            EcPrivateKey::P256(_) => panic!("expected P-384"),
        }
    }

    #[test]
    fn encode_p384_matches_known_der() {
        let scalar: [u8; 48] = decode_hex(P384_PRIVATE_KEY_HEX).try_into().unwrap();
        let key = p384::SecretKey::from_bytes(&scalar).unwrap();
        let der = encode_p384_pkcs8_der(&key);
        assert_eq!(der.as_slice(), decode_hex(P384_PKCS8_DER_HEX).as_slice());
    }

    #[test]
    fn encode_p384_round_trip() {
        let scalar: [u8; 48] = decode_hex(P384_PRIVATE_KEY_HEX).try_into().unwrap();
        let key = p384::SecretKey::from_bytes(&scalar).unwrap();
        let der = encode_p384_pkcs8_der(&key);
        match decode_ec_pkcs8_der(&der).unwrap() {
            EcPrivateKey::P384(decoded) => assert_eq!(decoded.to_bytes(), scalar),
            EcPrivateKey::P256(_) => panic!("expected P-384"),
        }
    }

    #[test]
    fn decode_ed25519_known_vector() {
        let der = decode_hex(ED25519_DER_HEX);
        let key = decode_ed25519_pkcs8_der(&der).unwrap();
        assert_eq!(key.to_bytes().as_slice(), decode_hex(ED25519_SEED_HEX));
    }

    #[test]
    fn encode_ed25519_matches_known_der() {
        let seed: [u8; 32] = decode_hex(ED25519_SEED_HEX).try_into().unwrap();
        let key = crate::curve25519::ed25519::SecretKey::from_bytes(&seed);
        let der = encode_ed25519_pkcs8_der(&key);
        assert_eq!(der.as_slice(), decode_hex(ED25519_DER_HEX).as_slice());
    }

    #[test]
    fn decode_ec_pkcs8_rejects_unsupported_curve() {
        // Replace the secp384r1 OID with an unsupported one.
        let mut der = decode_hex(P384_PKCS8_DER_HEX);
        der[21] = 0x01;
        assert!(matches!(decode_ec_pkcs8_der(&der), Err(Pkcs8Error::UnsupportedCurve)));
    }

    #[test]
    fn decode_ec_pkcs8_rejects_trailing_data() {
        let mut der = decode_hex(P384_PKCS8_DER_HEX);
        der.push(0x00);
        assert!(matches!(
            decode_ec_pkcs8_der(&der),
            Err(Pkcs8Error::InvalidDer(DerError::TrailingData))
        ));
    }

    #[test]
    fn decode_ed25519_rejects_wrong_algorithm() {
        let mut der = decode_hex(ED25519_DER_HEX);
        der[9] = 0x2c;
        assert!(matches!(decode_ed25519_pkcs8_der(&der), Err(Pkcs8Error::UnsupportedAlgorithm)));
    }

    // RFC 9881, appendix C.1.1: ML-DSA-44 seed-form private key whose seed is
    // 000102...1e1f.
    const ML_DSA_SEED: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11,
        0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    ];
    const ML_DSA_44_SEED_PKCS8_HEX: &str =
        "3034020100300b060960864801650304031104228020000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

    fn der_length(out: &mut Vec<u8>, len: usize) {
        if len < 0x80 {
            out.push(len as u8);
        } else if len < 0x100 {
            out.push(0x81);
            out.push(len as u8);
        } else {
            out.push(0x82);
            out.extend_from_slice(&(len as u16).to_be_bytes());
        }
    }

    fn der_tlv(tag: u8, content: &[u8]) -> Vec<u8> {
        let mut out = vec![tag];
        der_length(&mut out, content.len());
        out.extend_from_slice(content);
        out
    }

    /// Builds a PKCS#8 `PrivateKeyInfo` around an RFC 9881 `ML-DSA-XX-PrivateKey`
    /// CHOICE, with an optional public key field.
    fn mldsa_pkcs8(oid: &[u8], choice: &[u8], public_key: Option<&[u8]>) -> Vec<u8> {
        let algorithm = der_tlv(0x30, &der_tlv(0x06, oid));

        let mut pki = der_tlv(0x02, &[0x00]);
        pki.extend(algorithm);
        pki.extend(der_tlv(0x04, choice));
        if let Some(public_key) = public_key {
            let mut bit_string = vec![0x00];
            bit_string.extend_from_slice(public_key);
            pki.extend(der_tlv(0xa1, &der_tlv(0x03, &bit_string)));
        }
        der_tlv(0x30, &pki)
    }

    fn mldsa_both_choice(seed: &[u8; 32], expanded_len: usize) -> Vec<u8> {
        let mut content = der_tlv(0x04, seed);
        content.extend(der_tlv(0x04, &vec![0xab; expanded_len]));
        der_tlv(0x30, &content)
    }

    #[test]
    fn encode_mldsa44_matches_rfc9881_example() {
        let key = crate::mldsa::MlDsa44SecretKey::new(&ML_DSA_SEED);
        assert_eq!(
            encode_mldsa44_pkcs8_der(&key).as_slice(),
            decode_hex(ML_DSA_44_SEED_PKCS8_HEX).as_slice()
        );
    }

    #[test]
    fn decode_mldsa44_seed_example() {
        let der = decode_hex(ML_DSA_44_SEED_PKCS8_HEX);
        match decode_mldsa_pkcs8_der(&der).unwrap() {
            MlDsaPrivateKey::Dsa44(key) => assert_eq!(key.seed(), &ML_DSA_SEED),
            _ => panic!("expected ML-DSA-44"),
        }
    }

    #[test]
    fn mldsa_seed_round_trip_all_variants() {
        let der = encode_mldsa44_pkcs8_der(&crate::mldsa::MlDsa44SecretKey::new(&ML_DSA_SEED));
        match decode_mldsa_pkcs8_der(&der).unwrap() {
            MlDsaPrivateKey::Dsa44(key) => assert_eq!(key.seed(), &ML_DSA_SEED),
            _ => panic!("expected ML-DSA-44"),
        }

        let der = encode_mldsa65_pkcs8_der(&crate::mldsa::MlDsa65SecretKey::new(&ML_DSA_SEED));
        match decode_mldsa_pkcs8_der(&der).unwrap() {
            MlDsaPrivateKey::Dsa65(key) => assert_eq!(key.seed(), &ML_DSA_SEED),
            _ => panic!("expected ML-DSA-65"),
        }

        let der = encode_mldsa87_pkcs8_der(&crate::mldsa::MlDsa87SecretKey::new(&ML_DSA_SEED));
        match decode_mldsa_pkcs8_der(&der).unwrap() {
            MlDsaPrivateKey::Dsa87(key) => assert_eq!(key.seed(), &ML_DSA_SEED),
            _ => panic!("expected ML-DSA-87"),
        }
    }

    #[test]
    fn decode_mldsa_both_form() {
        // The form OpenSSL emits: SEQUENCE { seed OCTET STRING, expandedKey OCTET STRING }.
        let choice = mldsa_both_choice(&ML_DSA_SEED, 2560);
        let der = mldsa_pkcs8(OID_ML_DSA_44, &choice, None);
        match decode_mldsa_pkcs8_der(&der).unwrap() {
            MlDsaPrivateKey::Dsa44(key) => assert_eq!(key.seed(), &ML_DSA_SEED),
            _ => panic!("expected ML-DSA-44"),
        }
    }

    #[test]
    fn decode_mldsa_rejects_expanded_only() {
        let choice = der_tlv(0x04, &vec![0xcd; 4032]);
        let der = mldsa_pkcs8(OID_ML_DSA_65, &choice, None);
        assert!(matches!(
            decode_mldsa_pkcs8_der(&der),
            Err(Pkcs8Error::MlDsaExpandedKeyNotSupported)
        ));
    }

    #[test]
    fn decode_mldsa_checks_public_key() {
        // Matching public key.
        let mut seed_only = vec![0x80];
        seed_only.push(0x20);
        seed_only.extend_from_slice(&ML_DSA_SEED);
        let key = crate::mldsa::MlDsa44SecretKey::new(&ML_DSA_SEED);
        let der = mldsa_pkcs8(OID_ML_DSA_44, &seed_only, Some(&key.public_key().to_bytes()));
        assert!(decode_mldsa_pkcs8_der(&der).is_ok());

        // Mismatching public key.
        let mut wrong = key.public_key().to_bytes();
        wrong[0] ^= 0xff;
        let der = mldsa_pkcs8(OID_ML_DSA_44, &seed_only, Some(&wrong));
        assert!(matches!(
            decode_mldsa_pkcs8_der(&der),
            Err(Pkcs8Error::InvalidPublicKeyMismatch)
        ));
    }

    #[test]
    fn decode_mldsa_rejects_wrong_algorithm() {
        let mut seed_only = vec![0x80, 0x20];
        seed_only.extend_from_slice(&ML_DSA_SEED);
        let der = mldsa_pkcs8(OID_EC_PUBLIC_KEY, &seed_only, None);
        assert!(matches!(decode_mldsa_pkcs8_der(&der), Err(Pkcs8Error::UnsupportedAlgorithm)));
    }

    #[test]
    fn decode_mldsa_rejects_trailing_data() {
        let mut der = decode_hex(ML_DSA_44_SEED_PKCS8_HEX);
        der.push(0x00);
        assert!(matches!(
            decode_mldsa_pkcs8_der(&der),
            Err(Pkcs8Error::InvalidDer(DerError::TrailingData))
        ));
    }
}
