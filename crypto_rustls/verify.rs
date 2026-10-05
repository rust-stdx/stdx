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

use crypto::{
    curve25519::ed25519,
    encoding::ecdsa::{decode_p256_signature_der, decode_p384_signature_der},
};
use rustls::{
    SignatureScheme,
    crypto::WebPkiSupportedAlgorithms,
    pki_types::{AlgorithmIdentifier, InvalidSignature, SignatureVerificationAlgorithm, alg_id},
};

/// Smallest RSA modulus accepted by the RSA verification algorithms, in bytes
/// (2048 bits).
const RSA_MIN_MODULUS_BYTES: usize = 256;

macro_rules! ecdsa_verify_alg {
    ($name:ident, $public_key_alg:expr, $signature_alg:expr, $public:ty, $sig_len:expr, $decode:expr) => {
        #[derive(Debug)]
        struct $name;

        impl SignatureVerificationAlgorithm for $name {
            fn public_key_alg_id(&self) -> AlgorithmIdentifier {
                $public_key_alg
            }

            fn signature_alg_id(&self) -> AlgorithmIdentifier {
                $signature_alg
            }

            fn verify_signature(
                &self,
                public_key: &[u8],
                message: &[u8],
                signature: &[u8],
            ) -> Result<(), InvalidSignature> {
                let public = <$public>::from_bytes(public_key).map_err(|_| InvalidSignature)?;
                let signature: [u8; $sig_len] = ($decode)(signature).map_err(|_| InvalidSignature)?;
                public.verify(message, &signature).map_err(|_| InvalidSignature)
            }
        }
    };
}

macro_rules! rsa_verify_alg {
    ($name:ident, $signature_alg:expr, $verify:ident) => {
        #[derive(Debug)]
        struct $name;

        impl SignatureVerificationAlgorithm for $name {
            fn public_key_alg_id(&self) -> AlgorithmIdentifier {
                alg_id::RSA_ENCRYPTION
            }

            fn signature_alg_id(&self) -> AlgorithmIdentifier {
                $signature_alg
            }

            fn verify_signature(
                &self,
                public_key: &[u8],
                message: &[u8],
                signature: &[u8],
            ) -> Result<(), InvalidSignature> {
                // Parse the key once and verify through it, rather than parsing
                // in the size check and again inside the free function.
                let key = crypto::rsa::PublicKey::from_pkcs1_der(public_key).map_err(|_| InvalidSignature)?;
                if key.modulus_len_bytes() < RSA_MIN_MODULUS_BYTES {
                    return Err(InvalidSignature);
                }
                key.$verify(signature, message).map_err(|_| InvalidSignature)
            }
        }
    };
}

ecdsa_verify_alg!(
    EcdsaP256Sha256,
    alg_id::ECDSA_P256,
    alg_id::ECDSA_SHA256,
    crypto::p256::PublicKey,
    64,
    decode_p256_signature_der
);
ecdsa_verify_alg!(
    EcdsaP384Sha384,
    alg_id::ECDSA_P384,
    alg_id::ECDSA_SHA384,
    crypto::p384::PublicKey,
    96,
    decode_p384_signature_der
);

/// ECDSA over P-256 with SHA-256.
pub(crate) static ECDSA_P256_SHA256: &dyn SignatureVerificationAlgorithm = &EcdsaP256Sha256;
/// ECDSA over P-384 with SHA-384.
pub(crate) static ECDSA_P384_SHA384: &dyn SignatureVerificationAlgorithm = &EcdsaP384Sha384;

/// Ed25519.
#[derive(Debug)]
struct Ed25519Verify;

impl SignatureVerificationAlgorithm for Ed25519Verify {
    fn public_key_alg_id(&self) -> AlgorithmIdentifier {
        alg_id::ED25519
    }

    fn signature_alg_id(&self) -> AlgorithmIdentifier {
        alg_id::ED25519
    }

    fn verify_signature(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature> {
        let public_key: [u8; 32] = public_key.try_into().map_err(|_| InvalidSignature)?;
        let public = ed25519::PublicKey::from_bytes(&public_key).map_err(|_| InvalidSignature)?;
        let signature: [u8; 64] = signature.try_into().map_err(|_| InvalidSignature)?;
        public.verify(message, &signature).map_err(|_| InvalidSignature)
    }
}

/// Ed25519.
pub(crate) static ED25519: &dyn SignatureVerificationAlgorithm = &Ed25519Verify;

rsa_verify_alg!(RsaPkcs1Sha256, alg_id::RSA_PKCS1_SHA256, verify_pkcs1_sha256);
rsa_verify_alg!(RsaPkcs1Sha384, alg_id::RSA_PKCS1_SHA384, verify_pkcs1_sha384);
rsa_verify_alg!(RsaPkcs1Sha512, alg_id::RSA_PKCS1_SHA512, verify_pkcs1_sha512);
rsa_verify_alg!(RsaPssSha256, alg_id::RSA_PSS_SHA256, verify_pss_sha256);
rsa_verify_alg!(RsaPssSha384, alg_id::RSA_PSS_SHA384, verify_pss_sha384);
rsa_verify_alg!(RsaPssSha512, alg_id::RSA_PSS_SHA512, verify_pss_sha512);

/// ML-DSA is used in "pure" mode with an empty FIPS 204 context string, as
/// required by RFC 9881 (X.509) and the TLS 1.3 ML-DSA profile. The algorithm
/// identifier is the same for the public key and the signature.
macro_rules! mldsa_verify_alg {
    ($name:ident, $alg_id:expr, $public:ty, $public_key_len:expr, $sig_len:expr) => {
        #[derive(Debug)]
        struct $name;

        impl SignatureVerificationAlgorithm for $name {
            fn public_key_alg_id(&self) -> AlgorithmIdentifier {
                $alg_id
            }

            fn signature_alg_id(&self) -> AlgorithmIdentifier {
                $alg_id
            }

            fn verify_signature(
                &self,
                public_key: &[u8],
                message: &[u8],
                signature: &[u8],
            ) -> Result<(), InvalidSignature> {
                let public_key: [u8; $public_key_len] = public_key.try_into().map_err(|_| InvalidSignature)?;
                let public = <$public>::from_bytes(&public_key);
                let signature: [u8; $sig_len] = signature.try_into().map_err(|_| InvalidSignature)?;
                public
                    .verify(message, &signature, b"")
                    .map_err(|_| InvalidSignature)
            }
        }
    };
}

mldsa_verify_alg!(
    MlDsa44Verify,
    alg_id::ML_DSA_44,
    crypto::mldsa::MlDsa44PublicKey,
    crypto::mldsa::ML_DSA_44_PUBLIC_KEY_SIZE,
    crypto::mldsa::ML_DSA_44_SIGNATURE_SIZE
);
mldsa_verify_alg!(
    MlDsa65Verify,
    alg_id::ML_DSA_65,
    crypto::mldsa::MlDsa65PublicKey,
    crypto::mldsa::ML_DSA_65_PUBLIC_KEY_SIZE,
    crypto::mldsa::ML_DSA_65_SIGNATURE_SIZE
);
mldsa_verify_alg!(
    MlDsa87Verify,
    alg_id::ML_DSA_87,
    crypto::mldsa::MlDsa87PublicKey,
    crypto::mldsa::ML_DSA_87_PUBLIC_KEY_SIZE,
    crypto::mldsa::ML_DSA_87_SIGNATURE_SIZE
);

/// RSA PKCS#1 v1.5 with SHA-256.
pub(crate) static RSA_PKCS1_SHA256: &dyn SignatureVerificationAlgorithm = &RsaPkcs1Sha256;
/// RSA PKCS#1 v1.5 with SHA-384.
pub(crate) static RSA_PKCS1_SHA384: &dyn SignatureVerificationAlgorithm = &RsaPkcs1Sha384;
/// RSA PKCS#1 v1.5 with SHA-512.
pub(crate) static RSA_PKCS1_SHA512: &dyn SignatureVerificationAlgorithm = &RsaPkcs1Sha512;
/// RSA-PSS with SHA-256.
pub(crate) static RSA_PSS_SHA256: &dyn SignatureVerificationAlgorithm = &RsaPssSha256;
/// RSA-PSS with SHA-384.
pub(crate) static RSA_PSS_SHA384: &dyn SignatureVerificationAlgorithm = &RsaPssSha384;
/// RSA-PSS with SHA-512.
pub(crate) static RSA_PSS_SHA512: &dyn SignatureVerificationAlgorithm = &RsaPssSha512;

/// ML-DSA-44 (FIPS 204 category 2).
pub(crate) static ML_DSA_44: &dyn SignatureVerificationAlgorithm = &MlDsa44Verify;
/// ML-DSA-65 (FIPS 204 category 3).
pub(crate) static ML_DSA_65: &dyn SignatureVerificationAlgorithm = &MlDsa65Verify;
/// ML-DSA-87 (FIPS 204 category 5).
pub(crate) static ML_DSA_87: &dyn SignatureVerificationAlgorithm = &MlDsa87Verify;

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
        (SignatureScheme::RSA_PKCS1_SHA512, &[RSA_PKCS1_SHA512]),
        (SignatureScheme::RSA_PKCS1_SHA384, &[RSA_PKCS1_SHA384]),
        (SignatureScheme::RSA_PKCS1_SHA256, &[RSA_PKCS1_SHA256]),
        (SignatureScheme::ML_DSA_44, &[ML_DSA_44]),
        (SignatureScheme::ML_DSA_65, &[ML_DSA_65]),
        (SignatureScheme::ML_DSA_87, &[ML_DSA_87]),
    ],
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
