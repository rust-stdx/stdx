use crate::p256::{PUBLIC_KEY_UNCOMPRESSED_SIZE, SECRET_KEY_SIZE, SecretKey};

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
    InvalidEcPrivateKey,
    InvalidEcVersion,
    InvalidPrivateKeyOctet,
    InvalidPublicKeyExplicit,
    InvalidPublicKeyBitString,
    InvalidPublicKeyPrefix,
    KeyDerivationFailed,
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
            Pkcs8Error::InvalidEcPrivateKey => write!(f, "invalid ECPrivateKey SEQUENCE"),
            Pkcs8Error::InvalidEcVersion => write!(f, "invalid EC version"),
            Pkcs8Error::InvalidPrivateKeyOctet => write!(f, "invalid private key OCTET STRING"),
            Pkcs8Error::InvalidPublicKeyExplicit => write!(f, "invalid public key [1] EXPLICIT"),
            Pkcs8Error::InvalidPublicKeyBitString => write!(f, "invalid public key BIT STRING"),
            Pkcs8Error::InvalidPublicKeyPrefix => write!(f, "invalid public key prefix"),
            Pkcs8Error::KeyDerivationFailed => write!(f, "key derivation failed"),
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
    // ECPrivateKey SEQUENCE: 30 6b
    if der[29] != 0x30 || der[30] != 0x6b {
        return Err(Pkcs8Error::InvalidEcPrivateKey);
    }
    // EC version: 02 01 01
    if der[31] != 0x02 || der[32] != 0x01 || der[33] != 0x01 {
        return Err(Pkcs8Error::InvalidEcVersion);
    }
    // private key OCTET STRING: 04 20
    if der[34] != 0x04 || der[35] != 0x20 {
        return Err(Pkcs8Error::InvalidPrivateKeyOctet);
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
    // PrivateKeyInfo SEQUENCE (135 bytes content)
    0x30, 0x81, 0x87, // version INTEGER 0
    0x02, 0x01, 0x00, // AlgorithmIdentifier SEQUENCE (19 bytes)
    0x30, 0x13, // ecPublicKey OID (1.2.840.10045.2.1)
    0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, // secp256r1 OID (1.2.840.10045.3.1.7)
    0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07,
    // OCTET STRING wrapping ECPrivateKey (109 bytes)
    0x04, 0x6d, // ECPrivateKey SEQUENCE (107 bytes)
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

pub fn encode_p256_pkcs8_der(key: &SecretKey) -> Result<[u8; PKCS8_DER_LEN], Pkcs8Error> {
    let public_key = key.public_key();
    let pub_bytes = public_key.to_bytes();
    let priv_bytes = key.to_bytes();

    let mut der = TEMPLATE;
    der[PUBLIC_KEY_OFFSET..PUBLIC_KEY_OFFSET + PUBLIC_KEY_UNCOMPRESSED_SIZE].copy_from_slice(&pub_bytes);
    der[PRIVATE_KEY_OFFSET..PRIVATE_KEY_OFFSET + SECRET_KEY_SIZE].copy_from_slice(&priv_bytes);

    Ok(der)
}

pub fn decode_p256_pkcs8_der(der: &[u8]) -> Result<SecretKey, Pkcs8Error> {
    validate_fixed_prefix(der)?;

    let mut private_key = [0u8; SECRET_KEY_SIZE];
    private_key.copy_from_slice(&der[PRIVATE_KEY_OFFSET..PRIVATE_KEY_OFFSET + SECRET_KEY_SIZE]);

    let mut public_key = [0u8; PUBLIC_KEY_UNCOMPRESSED_SIZE];
    public_key.copy_from_slice(&der[PUBLIC_KEY_OFFSET..PUBLIC_KEY_OFFSET + PUBLIC_KEY_UNCOMPRESSED_SIZE]);

    SecretKey::from_bytes(&private_key).map_err(|_| Pkcs8Error::KeyDerivationFailed)
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
    fn decode_too_short() {
        assert_eq!(decode_p256_pkcs8_der(&[0u8; 100]), Err(Pkcs8Error::InvalidLength));
    }

    #[test]
    fn decode_too_long() {
        assert_eq!(decode_p256_pkcs8_der(&[0u8; 200]), Err(Pkcs8Error::InvalidLength));
    }

    #[test]
    fn decode_bad_sequence() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[0] = 0x31;
        assert_eq!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidSequence));
    }

    #[test]
    fn decode_bad_version() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[5] = 0x01;
        assert_eq!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidVersion));
    }

    #[test]
    fn decode_bad_algo_identifier() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[10] = 0x00;
        assert_eq!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidAlgorithmIdentifier));
    }

    #[test]
    fn decode_bad_ec_version() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[33] = 0x02;
        assert_eq!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidEcVersion));
    }

    #[test]
    fn decode_missing_public_key() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[68] = 0x00;
        assert_eq!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidPublicKeyExplicit));
    }

    #[test]
    fn decode_bad_public_key_prefix() {
        let mut der = decode_hex(TEST_DER_HEX);
        der[73] = 0x02;
        assert_eq!(decode_p256_pkcs8_der(&der), Err(Pkcs8Error::InvalidPublicKeyPrefix));
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
}
