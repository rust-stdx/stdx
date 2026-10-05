//! Private key loading and signing for rustls.
//!
//! Implements [`rustls::crypto::KeyProvider`] for the private key types this
//! provider supports:
//!
//! * ECDSA on P-256 (PKCS#8 or SEC1) with `ECDSA_NISTP256_SHA256`.
//! * ECDSA on P-384 (PKCS#8 or SEC1) with `ECDSA_NISTP384_SHA384`.
//! * Ed25519 (PKCS#8) with `ED25519`.
//! * ML-DSA-44/65/87 (PKCS#8, RFC 9881 seed form) with `ML_DSA_44/65/87`.
//!
//! RSA private keys are intentionally **not** supported: the `crypto` crate
//! implements RSA verification only. Serving a certificate with an RSA key
//! returns an error; use an ECDSA, Ed25519 or ML-DSA certificate instead.

use core::fmt;
use std::sync::Arc;

use crypto::{
    curve25519::ed25519,
    encoding::pkcs8::{
        EcPrivateKey, MlDsaPrivateKey, Pkcs8Error, decode_ec_pkcs8_der, decode_ec_sec1_der, decode_ed25519_pkcs8_der,
        decode_mldsa_pkcs8_der, is_rsa_pkcs8_der,
    },
    mldsa::{MlDsa44SecretKey, MlDsa65SecretKey, MlDsa87SecretKey},
};
use rustls::{
    Error, SignatureAlgorithm, SignatureScheme,
    pki_types::{AlgorithmIdentifier, PrivateKeyDer, SubjectPublicKeyInfoDer, alg_id},
    sign::{Signer, SigningKey, public_key_to_spki},
};

/// The [`rustls::crypto::KeyProvider`] used by this provider.
#[derive(Debug)]
pub(crate) struct Provider;

impl rustls::crypto::KeyProvider for Provider {
    fn load_private_key(&self, key_der: PrivateKeyDer<'static>) -> Result<Arc<dyn SigningKey>, Error> {
        match key_der {
            PrivateKeyDer::Pkcs8(der) => load_pkcs8(der.secret_pkcs8_der()),
            PrivateKeyDer::Sec1(der) => load_sec1(der.secret_sec1_der()),
            PrivateKeyDer::Pkcs1(_) => Err(Error::General(
                "RSA private keys are not supported by crypto_rustls; use an ECDSA, Ed25519 or ML-DSA key".into(),
            )),
            _ => Err(Error::General("unsupported private key format".into())),
        }
    }
}

fn load_pkcs8(der: &[u8]) -> Result<Arc<dyn SigningKey>, Error> {
    if let Ok(key) = decode_ed25519_pkcs8_der(der) {
        return Ok(Arc::new(Ed25519SigningKey {
            key,
        }));
    }

    if let Ok(key) = decode_ec_pkcs8_der(der) {
        return Ok(match key {
            EcPrivateKey::P256(key) => Arc::new(EcdsaP256SigningKey {
                key,
            }),
            EcPrivateKey::P384(key) => Arc::new(EcdsaP384SigningKey {
                key,
            }),
        });
    }

    if is_rsa_pkcs8_der(der) {
        return Err(Error::General(
            "RSA private keys are not supported by crypto_rustls; use an ECDSA, Ed25519 or ML-DSA key".into(),
        ));
    }

    match decode_mldsa_pkcs8_der(der) {
        Ok(key) => Ok(Arc::new(MlDsaSigningKey::from(key))),
        Err(Pkcs8Error::MlDsaExpandedKeyNotSupported) => Err(Error::General(
            "ML-DSA private keys stored in expanded form are not supported; \
             provide a seed-form key (RFC 9881)"
                .into(),
        )),
        Err(_) => Err(Error::General(
            "could not parse private key as Ed25519, ECDSA P-256/P-384 or ML-DSA".into(),
        )),
    }
}

fn load_sec1(der: &[u8]) -> Result<Arc<dyn SigningKey>, Error> {
    match decode_ec_sec1_der(der) {
        Ok(EcPrivateKey::P256(key)) => Ok(Arc::new(EcdsaP256SigningKey {
            key,
        })),
        Ok(EcPrivateKey::P384(key)) => Ok(Arc::new(EcdsaP384SigningKey {
            key,
        })),
        Err(_) => Err(Error::General(
            "could not parse SEC1 private key; only ECDSA P-256/P-384 are supported".into(),
        )),
    }
}

macro_rules! ecdsa_signing_key {
    ($key_struct:ident, $signer_struct:ident, $key:ty, $scalar_len:expr, $scheme:expr, $spki_alg_id:expr) => {
        struct $key_struct {
            key: $key,
        }

        impl fmt::Debug for $key_struct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($key_struct)).finish_non_exhaustive()
            }
        }

        impl SigningKey for $key_struct {
            fn choose_scheme(&self, offered: &[SignatureScheme]) -> Option<Box<dyn Signer>> {
                if offered.contains(&$scheme) {
                    Some(Box::new($signer_struct {
                        key: self.key.clone(),
                    }))
                } else {
                    None
                }
            }

            fn public_key(&self) -> Option<SubjectPublicKeyInfoDer<'_>> {
                Some(public_key_to_spki(&$spki_alg_id, self.key.public_key().to_bytes()))
            }

            fn algorithm(&self) -> SignatureAlgorithm {
                SignatureAlgorithm::ECDSA
            }
        }

        struct $signer_struct {
            key: $key,
        }

        impl fmt::Debug for $signer_struct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($signer_struct)).finish_non_exhaustive()
            }
        }

        impl Signer for $signer_struct {
            fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
                let raw = self
                    .key
                    .sign(message)
                    .map_err(|_| Error::General("ECDSA signing failed".into()))?;
                Ok(crypto::encoding::ecdsa::encode_signature_der(
                    &raw[..$scalar_len],
                    &raw[$scalar_len..],
                ))
            }

            fn scheme(&self) -> SignatureScheme {
                $scheme
            }
        }
    };
}

ecdsa_signing_key!(
    EcdsaP256SigningKey,
    EcdsaP256Signer,
    crypto::p256::SecretKey,
    32,
    SignatureScheme::ECDSA_NISTP256_SHA256,
    alg_id::ECDSA_P256
);
ecdsa_signing_key!(
    EcdsaP384SigningKey,
    EcdsaP384Signer,
    crypto::p384::SecretKey,
    48,
    SignatureScheme::ECDSA_NISTP384_SHA384,
    alg_id::ECDSA_P384
);

struct Ed25519SigningKey {
    key: ed25519::SecretKey,
}

impl fmt::Debug for Ed25519SigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ed25519SigningKey").finish_non_exhaustive()
    }
}

impl SigningKey for Ed25519SigningKey {
    fn choose_scheme(&self, offered: &[SignatureScheme]) -> Option<Box<dyn Signer>> {
        if offered.contains(&SignatureScheme::ED25519) {
            Some(Box::new(Ed25519Signer {
                key: self.key.clone(),
            }))
        } else {
            None
        }
    }

    fn public_key(&self) -> Option<SubjectPublicKeyInfoDer<'_>> {
        Some(public_key_to_spki(&alg_id::ED25519, self.key.public_key().to_bytes()))
    }

    fn algorithm(&self) -> SignatureAlgorithm {
        SignatureAlgorithm::ED25519
    }
}

struct Ed25519Signer {
    key: ed25519::SecretKey,
}

impl fmt::Debug for Ed25519Signer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ed25519Signer").finish_non_exhaustive()
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(self.key.sign(message).to_vec())
    }

    fn scheme(&self) -> SignatureScheme {
        SignatureScheme::ED25519
    }
}

/// The three supported ML-DSA private key types.
///
/// The expanded key material is large, so each variant is boxed; the signing
/// key wraps it in an [`Arc`] so signers can share it.
enum MlDsaKey {
    Dsa44(Box<MlDsa44SecretKey>),
    Dsa65(Box<MlDsa65SecretKey>),
    Dsa87(Box<MlDsa87SecretKey>),
}

impl MlDsaKey {
    fn scheme(&self) -> SignatureScheme {
        match self {
            MlDsaKey::Dsa44(_) => SignatureScheme::ML_DSA_44,
            MlDsaKey::Dsa65(_) => SignatureScheme::ML_DSA_65,
            MlDsaKey::Dsa87(_) => SignatureScheme::ML_DSA_87,
        }
    }

    fn alg_id(&self) -> AlgorithmIdentifier {
        match self {
            MlDsaKey::Dsa44(_) => alg_id::ML_DSA_44,
            MlDsaKey::Dsa65(_) => alg_id::ML_DSA_65,
            MlDsaKey::Dsa87(_) => alg_id::ML_DSA_87,
        }
    }

    fn public_key(&self) -> Vec<u8> {
        match self {
            MlDsaKey::Dsa44(key) => key.public_key().to_bytes().to_vec(),
            MlDsaKey::Dsa65(key) => key.public_key().to_bytes().to_vec(),
            MlDsaKey::Dsa87(key) => key.public_key().to_bytes().to_vec(),
        }
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        // RFC 9881 and the TLS 1.3 ML-DSA profile require the pure ML-DSA
        // variant with an empty FIPS 204 context string.
        let signature = match self {
            MlDsaKey::Dsa44(key) => key.sign(message, b"").map(|sig| sig.to_vec()),
            MlDsaKey::Dsa65(key) => key.sign(message, b"").map(|sig| sig.to_vec()),
            MlDsaKey::Dsa87(key) => key.sign(message, b"").map(|sig| sig.to_vec()),
        };
        signature.map_err(|_| Error::General("ML-DSA signing failed".into()))
    }
}

struct MlDsaSigningKey {
    key: Arc<MlDsaKey>,
}

impl From<MlDsaPrivateKey> for MlDsaSigningKey {
    fn from(key: MlDsaPrivateKey) -> Self {
        let key = match key {
            MlDsaPrivateKey::Dsa44(key) => MlDsaKey::Dsa44(key),
            MlDsaPrivateKey::Dsa65(key) => MlDsaKey::Dsa65(key),
            MlDsaPrivateKey::Dsa87(key) => MlDsaKey::Dsa87(key),
        };
        Self {
            key: Arc::new(key),
        }
    }
}

impl fmt::Debug for MlDsaSigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MlDsaSigningKey")
            .field("scheme", &self.key.scheme())
            .finish_non_exhaustive()
    }
}

impl SigningKey for MlDsaSigningKey {
    fn choose_scheme(&self, offered: &[SignatureScheme]) -> Option<Box<dyn Signer>> {
        let scheme = self.key.scheme();
        if offered.contains(&scheme) {
            Some(Box::new(MlDsaSigner {
                key: Arc::clone(&self.key),
                scheme,
            }))
        } else {
            None
        }
    }

    fn public_key(&self) -> Option<SubjectPublicKeyInfoDer<'_>> {
        Some(public_key_to_spki(&self.key.alg_id(), self.key.public_key()))
    }

    fn algorithm(&self) -> SignatureAlgorithm {
        // ML-DSA has no dedicated member in the legacy `SignatureAlgorithm`
        // enum; rustls' own provider reports `Unknown(0)` as well.
        SignatureAlgorithm::Unknown(0)
    }
}

struct MlDsaSigner {
    key: Arc<MlDsaKey>,
    scheme: SignatureScheme,
}

impl fmt::Debug for MlDsaSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MlDsaSigner").field("scheme", &self.scheme).finish()
    }
}

impl Signer for MlDsaSigner {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        self.key.sign(message)
    }

    fn scheme(&self) -> SignatureScheme {
        self.scheme
    }
}

#[cfg(test)]
mod tests {
    use crypto::encoding::pkcs8::{encode_ed25519_pkcs8_der, encode_p384_pkcs8_der};
    use rustls::{
        crypto::KeyProvider,
        pki_types::{PrivatePkcs1KeyDer, PrivatePkcs8KeyDer, PrivateSec1KeyDer},
    };

    use super::*;

    fn pkcs8(bytes: Vec<u8>) -> PrivateKeyDer<'static> {
        PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(bytes))
    }

    #[test]
    fn loads_and_signs_with_ecdsa_p256() {
        let key = crypto::p256::SecretKey::generate().unwrap();
        let public = key.public_key().to_bytes();
        let der = crypto::encoding::pkcs8::encode_p256_pkcs8_der(&key).unwrap().to_vec();

        let loaded = Provider.load_private_key(pkcs8(der)).unwrap();
        assert_eq!(loaded.algorithm(), SignatureAlgorithm::ECDSA);

        let signer = loaded.choose_scheme(&[SignatureScheme::ECDSA_NISTP256_SHA256]).unwrap();
        assert_eq!(signer.scheme(), SignatureScheme::ECDSA_NISTP256_SHA256);

        let message = b"handshake signature";
        let signature = signer.sign(message).unwrap();
        assert!(
            crate::verify::ECDSA_P256_SHA256
                .verify_signature(&public, message, &signature)
                .is_ok()
        );
    }

    #[test]
    fn loads_and_signs_with_ecdsa_p384() {
        let key = crypto::p384::SecretKey::generate().unwrap();
        let public = key.public_key().to_bytes();
        let der = encode_p384_pkcs8_der(&key).to_vec();

        let loaded = Provider.load_private_key(pkcs8(der)).unwrap();
        let signer = loaded.choose_scheme(&[SignatureScheme::ECDSA_NISTP384_SHA384]).unwrap();

        let message = b"handshake signature";
        let signature = signer.sign(message).unwrap();
        assert!(
            crate::verify::ECDSA_P384_SHA384
                .verify_signature(&public, message, &signature)
                .is_ok()
        );
    }

    #[test]
    fn loads_and_signs_with_ed25519() {
        let key = ed25519::SecretKey::generate().unwrap();
        let public = key.public_key().to_bytes();
        let der = encode_ed25519_pkcs8_der(&key).to_vec();

        let loaded = Provider.load_private_key(pkcs8(der)).unwrap();
        assert_eq!(loaded.algorithm(), SignatureAlgorithm::ED25519);

        let signer = loaded.choose_scheme(&[SignatureScheme::ED25519]).unwrap();
        let message = b"handshake signature";
        let signature = signer.sign(message).unwrap();
        assert!(
            crate::verify::ED25519
                .verify_signature(&public, message, &signature)
                .is_ok()
        );
    }

    #[test]
    fn loads_sec1_private_key() {
        // OpenSSL-generated P-384 SEC1 (RFC 5915) key from the crypto crate's
        // test corpus.
        let sec1 = hex::decode(
            "3081a4020101043078eff649cf18e8211af8581aa79c9f3271decb2d416a18b96c4efe305ccba836f34b68cb2138798bf3d07ee6ffb306dca00706052b81040022a164036200049fd48843177dcffdded82c7946bc174bbc14e672bb3f8dfad3b27814b686eb366a2ccdd8e898b397355ae71a09bad802c92f5a4d11a844e28dfa763b5450ff0ae67fa555698cb02121d0457acf44d9ee5d09307fdf1b58f0089d6b43e32c9500",
        )
        .unwrap();
        let loaded = Provider
            .load_private_key(PrivateKeyDer::Sec1(PrivateSec1KeyDer::from(sec1)))
            .unwrap();
        assert!(
            loaded
                .choose_scheme(&[SignatureScheme::ECDSA_NISTP384_SHA384])
                .is_some()
        );
    }

    #[test]
    fn rejects_rsa_pkcs1_keys() {
        let der = PrivateKeyDer::Pkcs1(PrivatePkcs1KeyDer::from(vec![0x30, 0x00]));
        assert!(Provider.load_private_key(der).is_err());
    }

    #[test]
    fn rejects_rsa_pkcs8_keys_with_a_clear_error() {
        // A minimal PKCS#8 PrivateKeyInfo carrying the rsaEncryption OID and
        // the customary NULL parameters.
        let der = hex::decode("3016020100300d06092a864886f70d010101050004023000").unwrap();
        let err = Provider.load_private_key(pkcs8(der)).unwrap_err();
        assert!(format!("{err}").contains("RSA"), "{err}");
    }

    #[test]
    fn rejects_unparseable_pkcs8() {
        assert!(Provider.load_private_key(pkcs8(vec![0x30, 0x00])).is_err());
    }

    #[test]
    fn choose_scheme_respects_offers() {
        let key = crypto::p256::SecretKey::generate().unwrap();
        let der = crypto::encoding::pkcs8::encode_p256_pkcs8_der(&key).unwrap().to_vec();
        let loaded = Provider.load_private_key(pkcs8(der)).unwrap();
        assert!(loaded.choose_scheme(&[SignatureScheme::ED25519]).is_none());
        assert!(
            loaded
                .choose_scheme(&[SignatureScheme::ED25519, SignatureScheme::ECDSA_NISTP256_SHA256])
                .is_some()
        );
    }

    #[test]
    fn loads_and_signs_with_mldsa() {
        use crypto::encoding::pkcs8::{encode_mldsa44_pkcs8_der, encode_mldsa65_pkcs8_der, encode_mldsa87_pkcs8_der};

        use crate::verify::{ML_DSA_44, ML_DSA_65, ML_DSA_87};

        let seed = [3u8; 32];
        let message = b"ml-dsa handshake signature";

        let key = crypto::mldsa::MlDsa44SecretKey::new(&seed);
        let public = key.public_key().to_bytes();
        let der = encode_mldsa44_pkcs8_der(&key).to_vec();
        let loaded = Provider.load_private_key(pkcs8(der)).unwrap();
        assert_eq!(loaded.algorithm(), SignatureAlgorithm::Unknown(0));
        let signer = loaded.choose_scheme(&[SignatureScheme::ML_DSA_44]).unwrap();
        assert_eq!(signer.scheme(), SignatureScheme::ML_DSA_44);
        let signature = signer.sign(message).unwrap();
        assert!(ML_DSA_44.verify_signature(&public, message, &signature).is_ok());

        let key = crypto::mldsa::MlDsa65SecretKey::new(&seed);
        let public = key.public_key().to_bytes();
        let der = encode_mldsa65_pkcs8_der(&key).to_vec();
        let loaded = Provider.load_private_key(pkcs8(der)).unwrap();
        let signer = loaded.choose_scheme(&[SignatureScheme::ML_DSA_65]).unwrap();
        let signature = signer.sign(message).unwrap();
        assert!(ML_DSA_65.verify_signature(&public, message, &signature).is_ok());

        let key = crypto::mldsa::MlDsa87SecretKey::new(&seed);
        let public = key.public_key().to_bytes();
        let der = encode_mldsa87_pkcs8_der(&key).to_vec();
        let loaded = Provider.load_private_key(pkcs8(der)).unwrap();
        let signer = loaded.choose_scheme(&[SignatureScheme::ML_DSA_87]).unwrap();
        let signature = signer.sign(message).unwrap();
        assert!(ML_DSA_87.verify_signature(&public, message, &signature).is_ok());
    }

    #[test]
    fn mldsa_choose_scheme_respects_offers() {
        use crypto::encoding::pkcs8::encode_mldsa65_pkcs8_der;

        let key = crypto::mldsa::MlDsa65SecretKey::new(&[3u8; 32]);
        let loaded = Provider
            .load_private_key(pkcs8(encode_mldsa65_pkcs8_der(&key).to_vec()))
            .unwrap();

        assert!(loaded.choose_scheme(&[SignatureScheme::ED25519]).is_none());
        assert!(loaded.choose_scheme(&[SignatureScheme::ML_DSA_44]).is_none());
        assert!(
            loaded
                .choose_scheme(&[SignatureScheme::ED25519, SignatureScheme::ML_DSA_65])
                .is_some()
        );
    }
}
