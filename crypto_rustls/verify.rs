//! Signature verification algorithms for rustls / webpki.
//!
//! These are used both to verify the signatures on certificate chains and to
//! verify the handshake signature in `CertificateVerify`.
//!
//! Supported:
//!
//! * ECDSA on P-256 with SHA-256, and P-384 with SHA-384.
//! * Ed25519.
//! * RSA PKCS#1 v1.5 and RSA-PSS, with SHA-256/384/512.
//! * ML-DSA-44/65/87 (FIPS 204), pure mode with an empty context string, as
//!   required by RFC 9881 and the TLS 1.3 ML-DSA profile.
//!
//! Note that the crate's curve implementations bind each curve to a single
//! hash (P-256/SHA-256, P-384/SHA-384), so the uncommon cross combinations
//! (for example a P-256 key with SHA-384) are not registered.
//!
//! Each supported [`crypto`] public-key type implements the local [`Verifier`]
//! trait, and [`Algorithm`] adapts any such type to rustls's
//! [`SignatureVerificationAlgorithm`]. RSA is handled by [`Rsa`] instead,
//! because a bare key does not encode its hash or padding scheme.

use core::marker::PhantomData;

use crypto::{
    curve25519::ed25519,
    encoding::ecdsa::{decode_p256_signature_der, decode_p384_signature_der},
};
use rustls::{
    SignatureScheme,
    crypto::WebPkiSupportedAlgorithms,
    pki_types::{AlgorithmIdentifier, InvalidSignature, SignatureVerificationAlgorithm, alg_id},
};

/// The verification algorithms supported by this provider, and the TLS
/// signature schemes they satisfy.
pub(crate) static ALGORITHMS: WebPkiSupportedAlgorithms = WebPkiSupportedAlgorithms {
    all: &[
        ECDSA_P256_SHA256,
        ECDSA_P384_SHA384,
        ED25519,
        RSA_PSS_SHA512,
        RSA_PSS_SHA384,
        RSA_PSS_SHA256,
        RSA_PKCS1_SHA512,
        RSA_PKCS1_SHA384,
        RSA_PKCS1_SHA256,
        RSA_PKCS1_SHA512_ABSENT_PARAMS,
        RSA_PKCS1_SHA384_ABSENT_PARAMS,
        RSA_PKCS1_SHA256_ABSENT_PARAMS,
        ML_DSA_44,
        ML_DSA_65,
        ML_DSA_87,
    ],
    mapping: &[
        (SignatureScheme::ECDSA_NISTP256_SHA256, &[ECDSA_P256_SHA256]),
        (SignatureScheme::ECDSA_NISTP384_SHA384, &[ECDSA_P384_SHA384]),
        (SignatureScheme::ED25519, &[ED25519]),
        (SignatureScheme::RSA_PSS_SHA512, &[RSA_PSS_SHA512]),
        (SignatureScheme::RSA_PSS_SHA384, &[RSA_PSS_SHA384]),
        (SignatureScheme::RSA_PSS_SHA256, &[RSA_PSS_SHA256]),
        (
            SignatureScheme::RSA_PKCS1_SHA512,
            &[RSA_PKCS1_SHA512, RSA_PKCS1_SHA512_ABSENT_PARAMS],
        ),
        (
            SignatureScheme::RSA_PKCS1_SHA384,
            &[RSA_PKCS1_SHA384, RSA_PKCS1_SHA384_ABSENT_PARAMS],
        ),
        (
            SignatureScheme::RSA_PKCS1_SHA256,
            &[RSA_PKCS1_SHA256, RSA_PKCS1_SHA256_ABSENT_PARAMS],
        ),
        (SignatureScheme::ML_DSA_44, &[ML_DSA_44]),
        (SignatureScheme::ML_DSA_65, &[ML_DSA_65]),
        (SignatureScheme::ML_DSA_87, &[ML_DSA_87]),
    ],
};

/// Smallest RSA modulus accepted by the RSA verification algorithms, in bytes
/// (2048 bits).
const RSA_MIN_MODULUS_BYTES: usize = 256;

/// A signature-verification algorithm for one `crypto` public-key type.
///
/// This is a local trait, so it can be implemented directly on the (foreign)
/// `crypto` key types; [`Algorithm`] bridges an implementor to rustls's
/// [`SignatureVerificationAlgorithm`].
pub(crate) trait Verifier: Send + Sync + 'static {
    /// The `AlgorithmIdentifier` of the public key.
    const PUBLIC_KEY_ALG_ID: AlgorithmIdentifier;
    /// The `AlgorithmIdentifier` of the signature.
    const SIGNATURE_ALG_ID: AlgorithmIdentifier;

    /// Parses the untrusted `subjectPublicKey` bytes.
    ///
    /// Returns [`InvalidSignature`] if the encoding is invalid.
    fn parse(public_key: &[u8]) -> Result<Self, InvalidSignature>
    where
        Self: Sized;

    /// Verifies `signature` over `message`.
    ///
    /// Returns [`InvalidSignature`] if the signature is invalid.
    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature>;
}

/// Zero-sized adapter from a [`Verifier`] key type to rustls.
struct Algorithm<K>(PhantomData<K>);

impl<K: Verifier> SignatureVerificationAlgorithm for Algorithm<K> {
    fn public_key_alg_id(&self) -> AlgorithmIdentifier {
        K::PUBLIC_KEY_ALG_ID
    }

    fn signature_alg_id(&self) -> AlgorithmIdentifier {
        K::SIGNATURE_ALG_ID
    }

    fn verify_signature(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        K::parse(public_key)?.verify_signature(message, signature)
    }
}

impl<K> core::fmt::Debug for Algorithm<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::any::type_name::<K>())
    }
}

impl Verifier for crypto::p256::PublicKey {
    const PUBLIC_KEY_ALG_ID: AlgorithmIdentifier = alg_id::ECDSA_P256;
    const SIGNATURE_ALG_ID: AlgorithmIdentifier = alg_id::ECDSA_SHA256;

    fn parse(public_key: &[u8]) -> Result<Self, InvalidSignature> {
        Self::from_bytes(public_key).map_err(|_| InvalidSignature)
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        let signature = decode_p256_signature_der(signature).map_err(|_| InvalidSignature)?;
        self.verify(message, &signature).map_err(|_| InvalidSignature)
    }
}

impl Verifier for crypto::p384::PublicKey {
    const PUBLIC_KEY_ALG_ID: AlgorithmIdentifier = alg_id::ECDSA_P384;
    const SIGNATURE_ALG_ID: AlgorithmIdentifier = alg_id::ECDSA_SHA384;

    fn parse(public_key: &[u8]) -> Result<Self, InvalidSignature> {
        Self::from_bytes(public_key).map_err(|_| InvalidSignature)
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        let signature = decode_p384_signature_der(signature).map_err(|_| InvalidSignature)?;
        self.verify(message, &signature).map_err(|_| InvalidSignature)
    }
}

/// ECDSA over P-256 with SHA-256.
pub(crate) static ECDSA_P256_SHA256: &dyn SignatureVerificationAlgorithm =
    &Algorithm::<crypto::p256::PublicKey>(PhantomData);
/// ECDSA over P-384 with SHA-384.
pub(crate) static ECDSA_P384_SHA384: &dyn SignatureVerificationAlgorithm =
    &Algorithm::<crypto::p384::PublicKey>(PhantomData);

impl Verifier for ed25519::PublicKey {
    const PUBLIC_KEY_ALG_ID: AlgorithmIdentifier = alg_id::ED25519;
    const SIGNATURE_ALG_ID: AlgorithmIdentifier = alg_id::ED25519;

    fn parse(public_key: &[u8]) -> Result<Self, InvalidSignature> {
        let public_key: &[u8; 32] = public_key.try_into().map_err(|_| InvalidSignature)?;
        ed25519::PublicKey::from_bytes(public_key).map_err(|_| InvalidSignature)
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        let signature: &[u8; 64] = signature.try_into().map_err(|_| InvalidSignature)?;
        self.verify(message, signature).map_err(|_| InvalidSignature)
    }
}

/// Ed25519.
pub(crate) static ED25519: &dyn SignatureVerificationAlgorithm = &Algorithm::<ed25519::PublicKey>(PhantomData);

// RFC 4055 section 2.1 requires implementations to accept `sha*WithRSAEncryption`
// when the (optional) `NULL` parameters are absent. The `alg_id::RSA_PKCS1_*`
// constants encode the parameters as present, so distinct algorithm objects are
// registered for the absent-parameter encodings. These only affect X.509
// signature-algorithm matching; TLS handshake signatures are selected by
// `SignatureScheme`.
const ALG_ID_RSA_PKCS1_SHA256_ABSENT_PARAMS: AlgorithmIdentifier =
    AlgorithmIdentifier::from_slice(&[0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b]);
const ALG_ID_RSA_PKCS1_SHA384_ABSENT_PARAMS: AlgorithmIdentifier =
    AlgorithmIdentifier::from_slice(&[0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0c]);
const ALG_ID_RSA_PKCS1_SHA512_ABSENT_PARAMS: AlgorithmIdentifier =
    AlgorithmIdentifier::from_slice(&[0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0d]);

/// ML-DSA is used in "pure" mode with an empty FIPS 204 context string, as
/// required by RFC 9881 (X.509) and the TLS 1.3 ML-DSA profile. The algorithm
/// identifier is the same for the public key and the signature.
impl Verifier for crypto::mldsa::MlDsa44PublicKey {
    const PUBLIC_KEY_ALG_ID: AlgorithmIdentifier = alg_id::ML_DSA_44;
    const SIGNATURE_ALG_ID: AlgorithmIdentifier = alg_id::ML_DSA_44;

    fn parse(public_key: &[u8]) -> Result<Self, InvalidSignature> {
        let public_key: &[u8; crypto::mldsa::ML_DSA_44_PUBLIC_KEY_SIZE] =
            public_key.try_into().map_err(|_| InvalidSignature)?;
        Ok(Self::from_bytes(public_key))
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        let signature: &[u8; crypto::mldsa::ML_DSA_44_SIGNATURE_SIZE] =
            signature.try_into().map_err(|_| InvalidSignature)?;
        self.verify(message, signature, b"").map_err(|_| InvalidSignature)
    }
}

impl Verifier for crypto::mldsa::MlDsa65PublicKey {
    const PUBLIC_KEY_ALG_ID: AlgorithmIdentifier = alg_id::ML_DSA_65;
    const SIGNATURE_ALG_ID: AlgorithmIdentifier = alg_id::ML_DSA_65;

    fn parse(public_key: &[u8]) -> Result<Self, InvalidSignature> {
        let public_key: &[u8; crypto::mldsa::ML_DSA_65_PUBLIC_KEY_SIZE] =
            public_key.try_into().map_err(|_| InvalidSignature)?;
        Ok(Self::from_bytes(public_key))
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        let signature: &[u8; crypto::mldsa::ML_DSA_65_SIGNATURE_SIZE] =
            signature.try_into().map_err(|_| InvalidSignature)?;
        self.verify(message, signature, b"").map_err(|_| InvalidSignature)
    }
}

impl Verifier for crypto::mldsa::MlDsa87PublicKey {
    const PUBLIC_KEY_ALG_ID: AlgorithmIdentifier = alg_id::ML_DSA_87;
    const SIGNATURE_ALG_ID: AlgorithmIdentifier = alg_id::ML_DSA_87;

    fn parse(public_key: &[u8]) -> Result<Self, InvalidSignature> {
        let public_key: &[u8; crypto::mldsa::ML_DSA_87_PUBLIC_KEY_SIZE] =
            public_key.try_into().map_err(|_| InvalidSignature)?;
        Ok(Self::from_bytes(public_key))
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        let signature: &[u8; crypto::mldsa::ML_DSA_87_SIGNATURE_SIZE] =
            signature.try_into().map_err(|_| InvalidSignature)?;
        self.verify(message, signature, b"").map_err(|_| InvalidSignature)
    }
}

/// ML-DSA-44 (FIPS 204 category 2).
pub(crate) static ML_DSA_44: &dyn SignatureVerificationAlgorithm =
    &Algorithm::<crypto::mldsa::MlDsa44PublicKey>(PhantomData);
/// ML-DSA-65 (FIPS 204 category 3).
pub(crate) static ML_DSA_65: &dyn SignatureVerificationAlgorithm =
    &Algorithm::<crypto::mldsa::MlDsa65PublicKey>(PhantomData);
/// ML-DSA-87 (FIPS 204 category 5).
pub(crate) static ML_DSA_87: &dyn SignatureVerificationAlgorithm =
    &Algorithm::<crypto::mldsa::MlDsa87PublicKey>(PhantomData);

/// RSA verification algorithm.
///
/// The scheme is stored alongside the verify function: a bare
/// [`crypto::rsa::PublicKey`] does not encode whether to use PKCS#1 v1.5 or
/// PSS, nor which hash. The absent-parameter `AlgorithmIdentifier` variants
/// reuse the same verify function as their present-parameter counterparts.
/// A `crypto` RSA verification method: `(key, signature, message)`.
type RsaVerify = fn(&crypto::rsa::PublicKey, &[u8], &[u8]) -> Result<(), crypto::RsaError>;

#[derive(Debug)]
struct Rsa {
    signature_alg: AlgorithmIdentifier,
    verify: RsaVerify,
}

impl SignatureVerificationAlgorithm for Rsa {
    fn public_key_alg_id(&self) -> AlgorithmIdentifier {
        alg_id::RSA_ENCRYPTION
    }

    fn signature_alg_id(&self) -> AlgorithmIdentifier {
        self.signature_alg
    }

    fn verify_signature(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        // Parse the key once and verify through it, rather than parsing in the
        // size check and again inside the free function.
        let key = crypto::rsa::PublicKey::from_pkcs1_der(public_key).map_err(|_| InvalidSignature)?;
        if key.modulus_len_bytes() < RSA_MIN_MODULUS_BYTES {
            return Err(InvalidSignature);
        }
        (self.verify)(&key, signature, message).map_err(|_| InvalidSignature)
    }
}

/// RSA PKCS#1 v1.5 with SHA-256.
pub(crate) static RSA_PKCS1_SHA256: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: alg_id::RSA_PKCS1_SHA256,
    verify: crypto::rsa::PublicKey::verify_pkcs1_sha256,
};
/// RSA PKCS#1 v1.5 with SHA-384.
pub(crate) static RSA_PKCS1_SHA384: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: alg_id::RSA_PKCS1_SHA384,
    verify: crypto::rsa::PublicKey::verify_pkcs1_sha384,
};
/// RSA PKCS#1 v1.5 with SHA-512.
pub(crate) static RSA_PKCS1_SHA512: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: alg_id::RSA_PKCS1_SHA512,
    verify: crypto::rsa::PublicKey::verify_pkcs1_sha512,
};
/// RSA-PSS with SHA-256.
pub(crate) static RSA_PSS_SHA256: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: alg_id::RSA_PSS_SHA256,
    verify: crypto::rsa::PublicKey::verify_pss_sha256,
};
/// RSA-PSS with SHA-384.
pub(crate) static RSA_PSS_SHA384: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: alg_id::RSA_PSS_SHA384,
    verify: crypto::rsa::PublicKey::verify_pss_sha384,
};
/// RSA-PSS with SHA-512.
pub(crate) static RSA_PSS_SHA512: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: alg_id::RSA_PSS_SHA512,
    verify: crypto::rsa::PublicKey::verify_pss_sha512,
};
/// RSA PKCS#1 v1.5 with SHA-256, absent `AlgorithmIdentifier` parameters.
pub(crate) static RSA_PKCS1_SHA256_ABSENT_PARAMS: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: ALG_ID_RSA_PKCS1_SHA256_ABSENT_PARAMS,
    verify: crypto::rsa::PublicKey::verify_pkcs1_sha256,
};
/// RSA PKCS#1 v1.5 with SHA-384, absent `AlgorithmIdentifier` parameters.
pub(crate) static RSA_PKCS1_SHA384_ABSENT_PARAMS: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: ALG_ID_RSA_PKCS1_SHA384_ABSENT_PARAMS,
    verify: crypto::rsa::PublicKey::verify_pkcs1_sha384,
};
/// RSA PKCS#1 v1.5 with SHA-512, absent `AlgorithmIdentifier` parameters.
pub(crate) static RSA_PKCS1_SHA512_ABSENT_PARAMS: &dyn SignatureVerificationAlgorithm = &Rsa {
    signature_alg: ALG_ID_RSA_PKCS1_SHA512_ABSENT_PARAMS,
    verify: crypto::rsa::PublicKey::verify_pkcs1_sha512,
};

#[cfg(test)]
mod tests {
    use super::*;

    // Generated with OpenSSL 3.5: an RSA-2048 key, the PKCS#1 public key DER,
    // the message "hello rsa world", and signatures over it.
    const RSA_PUBLIC_KEY: &str = "3082010a0282010100b5fdff68435ed2527e6a4e9e256c63d17439835f83644ef2a7d0e658f30978a1ff1fc4f1b9be1cd5286a2c8f62aaed5d495b455f2e52e3192fad5b445e65986396698371025b687367e2e023c0eb0142b3cf55a0efaf31e850bd23e472c53858928949e1fcce5159aeff9e6d31f1225566dd3f8b34adb1c40031adf95019bfd71d43db1a1d661e35b249abc761a0a568c71a9dd203f54a12c11407a610f265b6694b570ff5ad8bdf970b7e5adae8b89f5eed4b020ccce9304bf675a2f6cd81ceb5ee7e93b050d6b551c2299a60e5780f963993146cf39798d038bdb8cfe69ab1988b17ec9bec0fd5dffad9afae5cd482fbaddd4b6cde3692e8d98a0901d0c7f50203010001";
    const RSA_MESSAGE: &str = "68656c6c6f2072736120776f726c64";
    const RSA_PKCS1_SIGNATURE: &str = "b3282c055f067c1eeb0a838a2ebf84661258153ab2d49befde84d99185f6460dc9b6d0b05671e92b23f0b0143d9f9312ab72e38680470523e06adfc7040cd7e64481760bf63beb39ddb2df522d666e7119b4d8aefc739823b7c90b6ded58afad99cc289f6208c2b1b601b97bb272b4931cc57560b6775df8a2fa7e7653fcbce893d237309c543d7f3c9a985b516ba9f71cc858f289e0fc8ae1f576ddbf0c1224c6abb3fdb66862aa2765cff292755ab75976843b45c6331e1e8b50863e4cbde121a29a6d7074af6d66fe4bbc9b3bebed89dec6065da805f0744ce7b679b8cbbebdbf441ed007ce976dd4ae4b890bec40d8edf7e8805ece87dc3d97d0c6f89f49";
    const RSA_PSS_SIGNATURE: &str = "31b8dd98ebcd5b470e00dba69d76465aac8252d3f67ce50f63606cdddaf8d20685a02d7260e6d4a4731f2899ea36457f68a87050fada7463d5cadd3138f60d35ac42fb042203bfcba36b3a4b47d71da0d2e3155ce00748d30024854f62dc80e0adbad52163f9d53762a3c69be5e5743d86f3d74b7109f2660e945bc25ceab2abc8b83e4a47c1bcc56b69edeee16ba9c53edfc01434ae88719287ec39c70b0abe1f67eeadcaef2741a45c2434c6482def08477fcd56c24b87f829049f1f06fadc67c70c50c4e9a839eaaa01996ebcfa4fc4106a50dbd4379cc8a7fca60f0fa974883fb8d302f1c050513a19d5897ed41fd6f9124157f009e93fd9ec3b5d13b819";

    #[test]
    fn ecdsa_p256_round_trip() {
        let key = crypto::p256::SecretKey::generate().unwrap();
        let public = key.public_key().to_bytes();
        let message = b"a message to be signed";
        let raw = key.sign(message).unwrap();
        let der = crypto::encoding::ecdsa::encode_signature_der(&raw[..32], &raw[32..]);

        assert!(ECDSA_P256_SHA256.verify_signature(&public, message, &der).is_ok());
        assert!(ECDSA_P256_SHA256.verify_signature(&public, b"other", &der).is_err());
    }

    #[test]
    fn ecdsa_p384_round_trip() {
        let key = crypto::p384::SecretKey::generate().unwrap();
        let public = key.public_key().to_bytes();
        let message = b"a message to be signed";
        let raw = key.sign(message).unwrap();
        let der = crypto::encoding::ecdsa::encode_signature_der(&raw[..48], &raw[48..]);

        assert!(ECDSA_P384_SHA384.verify_signature(&public, message, &der).is_ok());
        assert!(ECDSA_P384_SHA384.verify_signature(&public, b"other", &der).is_err());
    }

    #[test]
    fn ed25519_round_trip() {
        let key = ed25519::SecretKey::generate().unwrap();
        let public = key.public_key().to_bytes();
        let message = b"a message to be signed";
        let signature = key.sign(message);

        assert!(ED25519.verify_signature(&public, message, &signature).is_ok());
        assert!(ED25519.verify_signature(&public, b"other", &signature).is_err());
    }

    #[test]
    fn rsa_pkcs1_sha256_openssl_vector() {
        let public = hex::decode(RSA_PUBLIC_KEY).unwrap();
        let message = hex::decode(RSA_MESSAGE).unwrap();
        let signature = hex::decode(RSA_PKCS1_SIGNATURE).unwrap();

        assert!(RSA_PKCS1_SHA256.verify_signature(&public, &message, &signature).is_ok());
        assert!(
            RSA_PKCS1_SHA256
                .verify_signature(&public, b"tampered", &signature)
                .is_err()
        );
    }

    #[test]
    fn rsa_pss_sha256_openssl_vector() {
        let public = hex::decode(RSA_PUBLIC_KEY).unwrap();
        let message = hex::decode(RSA_MESSAGE).unwrap();
        let signature = hex::decode(RSA_PSS_SIGNATURE).unwrap();

        assert!(RSA_PSS_SHA256.verify_signature(&public, &message, &signature).is_ok());
        assert!(
            RSA_PSS_SHA256
                .verify_signature(&public, b"tampered", &signature)
                .is_err()
        );
    }

    #[test]
    fn algorithm_identifiers_match_expectations() {
        assert_eq!(ECDSA_P256_SHA256.public_key_alg_id(), alg_id::ECDSA_P256);
        assert_eq!(ECDSA_P256_SHA256.signature_alg_id(), alg_id::ECDSA_SHA256);
        assert_eq!(ECDSA_P384_SHA384.public_key_alg_id(), alg_id::ECDSA_P384);
        assert_eq!(ED25519.public_key_alg_id(), alg_id::ED25519);
        assert_eq!(ED25519.signature_alg_id(), alg_id::ED25519);
        assert_eq!(RSA_PSS_SHA256.public_key_alg_id(), alg_id::RSA_ENCRYPTION);
        assert_eq!(RSA_PSS_SHA256.signature_alg_id(), alg_id::RSA_PSS_SHA256);
        assert_eq!(RSA_PKCS1_SHA512.signature_alg_id(), alg_id::RSA_PKCS1_SHA512);
    }

    #[test]
    fn rsa_pkcs1_absent_params_variants() {
        // The absent-parameter encodings are the bare OID, without the `NULL`.
        assert_eq!(
            RSA_PKCS1_SHA256_ABSENT_PARAMS.signature_alg_id().as_ref(),
            &[0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b]
        );
        assert_ne!(
            RSA_PKCS1_SHA256_ABSENT_PARAMS.signature_alg_id(),
            RSA_PKCS1_SHA256.signature_alg_id()
        );
        assert_eq!(RSA_PKCS1_SHA384_ABSENT_PARAMS.public_key_alg_id(), alg_id::RSA_ENCRYPTION);
        assert_eq!(RSA_PKCS1_SHA512_ABSENT_PARAMS.public_key_alg_id(), alg_id::RSA_ENCRYPTION);
    }

    #[test]
    fn rsa_pkcs1_absent_params_verifies_signatures() {
        let public = hex::decode(RSA_PUBLIC_KEY).unwrap();
        let message = hex::decode(RSA_MESSAGE).unwrap();
        let signature = hex::decode(RSA_PKCS1_SIGNATURE).unwrap();

        assert!(
            RSA_PKCS1_SHA256_ABSENT_PARAMS
                .verify_signature(&public, &message, &signature)
                .is_ok()
        );
        assert!(
            RSA_PKCS1_SHA256_ABSENT_PARAMS
                .verify_signature(&public, b"tampered", &signature)
                .is_err()
        );
    }

    #[test]
    fn mapping_covers_all_listed_algorithms() {
        for algorithm in ALGORITHMS.all {
            let reachable = ALGORITHMS
                .mapping
                .iter()
                .any(|(_, algorithms)| algorithms.iter().any(|a| core::ptr::eq(*a, *algorithm)));
            assert!(reachable, "{:?} is not reachable through the scheme mapping", algorithm);
        }
    }

    #[test]
    fn mldsa_round_trips() {
        let seed = [7u8; 32];
        let message = b"a message to be signed";

        let key = crypto::mldsa::MlDsa44SecretKey::new(&seed);
        let public = key.public_key().to_bytes();
        let signature = key.sign(message, b"").unwrap();
        assert!(ML_DSA_44.verify_signature(&public, message, &signature).is_ok());
        assert!(ML_DSA_44.verify_signature(&public, b"other", &signature).is_err());

        let key = crypto::mldsa::MlDsa65SecretKey::new(&seed);
        let public = key.public_key().to_bytes();
        let signature = key.sign(message, b"").unwrap();
        assert!(ML_DSA_65.verify_signature(&public, message, &signature).is_ok());
        assert!(ML_DSA_65.verify_signature(&public, b"other", &signature).is_err());

        let key = crypto::mldsa::MlDsa87SecretKey::new(&seed);
        let public = key.public_key().to_bytes();
        let signature = key.sign(message, b"").unwrap();
        assert!(ML_DSA_87.verify_signature(&public, message, &signature).is_ok());
        assert!(ML_DSA_87.verify_signature(&public, b"other", &signature).is_err());
    }

    #[test]
    fn mldsa_rejects_wrong_length_public_key() {
        let key = crypto::mldsa::MlDsa65SecretKey::new(&[9u8; 32]);
        let signature = key.sign(b"message", b"").unwrap();

        assert!(ML_DSA_65.verify_signature(&[0u8; 10], b"message", &signature).is_err());
        assert!(
            ML_DSA_65
                .verify_signature(&key.public_key().to_bytes(), b"message", &signature[..10])
                .is_err()
        );
    }
}
