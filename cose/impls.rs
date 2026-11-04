//! Implementations of the COSE traits for the stdx `crypto` key types.

use core::fmt;

use constant_time_eq::constant_time_eq;
#[cfg(feature = "random")]
use crypto::mldsa::{MlDsa44SecretKey, MlDsa65SecretKey, MlDsa87SecretKey};
use crypto::{
    Hasher,
    curve25519::ed25519,
    hmac::Hmac,
    mldsa::{
        ML_DSA_44_SIGNATURE_SIZE, ML_DSA_65_SIGNATURE_SIZE, ML_DSA_87_SIGNATURE_SIZE, MlDsa44PublicKey,
        MlDsa65PublicKey, MlDsa87PublicKey,
    },
    p256, p384, p521, rsa,
    sha2::{Sha256, Sha384, Sha512},
};

use crate::{Algorithm, Error, Macer, Result, Signer, Verifier};

////////////////////////////////////////////////////////////////////////////////
// Symmetric HMAC keys
////////////////////////////////////////////////////////////////////////////////

/// A symmetric key used with the COSE HMAC algorithms.
///
/// The key is borrowed. `new` rejects algorithms that are not HMAC and keys
/// shorter than 16 bytes.
pub struct SymmetricKey<'a> {
    key: &'a [u8],
    algorithm: Algorithm,
}

impl<'a> SymmetricKey<'a> {
    /// Creates an HMAC key.
    pub fn new(algorithm: Algorithm, key: &'a [u8]) -> Result<Self> {
        if !algorithm.is_mac() || key.len() < 16 {
            return Err(Error::InvalidKey);
        }
        Ok(Self {
            key,
            algorithm,
        })
    }

    /// Returns the algorithm.
    #[inline]
    pub fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    /// Returns the raw key bytes.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.key
    }
}

impl fmt::Debug for SymmetricKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SymmetricKey")
            .field("algorithm", &self.algorithm)
            .field("key", &"<redacted>")
            .finish()
    }
}

impl Macer for SymmetricKey<'_> {
    #[inline]
    fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    #[inline]
    fn tag_len(&self) -> usize {
        self.algorithm.tag_len().unwrap_or(0)
    }

    fn mac_into(&self, message: &[u8], out: &mut [u8]) -> Result<usize> {
        match self.algorithm {
            Algorithm::Hmac256_64 => {
                let tag = Hmac::<Sha256>::mac(self.key, message);
                copy_tag(&tag, 8, out)
            }
            Algorithm::Hmac256_256 => {
                let tag = Hmac::<Sha256>::mac(self.key, message);
                copy_tag(&tag, 32, out)
            }
            Algorithm::Hmac384_384 => {
                let tag = Hmac::<Sha384>::mac(self.key, message);
                copy_tag(&tag, 48, out)
            }
            Algorithm::Hmac512_512 => {
                let tag = Hmac::<Sha512>::mac(self.key, message);
                copy_tag(&tag, 64, out)
            }
            _ => Err(Error::InvalidKey),
        }
    }

    fn verify(&self, message: &[u8], tag: &[u8]) -> Result<()> {
        let expected_len = self.tag_len();
        if tag.len() != expected_len {
            return Err(Error::InvalidTag);
        }
        let mut computed = [0u8; 64];
        let len = self.mac_into(message, &mut computed[..expected_len])?;
        if constant_time_eq(&computed[..len], tag) {
            Ok(())
        } else {
            Err(Error::InvalidTag)
        }
    }
}

fn copy_tag(tag: &crypto::Hash, len: usize, out: &mut [u8]) -> Result<usize> {
    if out.len() < len {
        return Err(Error::BufferTooSmall {
            required: len,
            available: out.len(),
        });
    }
    out[..len].copy_from_slice(&tag.as_ref()[..len]);
    Ok(len)
}

////////////////////////////////////////////////////////////////////////////////
// Ed25519
////////////////////////////////////////////////////////////////////////////////

impl Signer for ed25519::SecretKey {
    #[inline]
    fn algorithm(&self) -> Algorithm {
        Algorithm::Ed25519
    }

    #[inline]
    fn signature_len(&self) -> usize {
        64
    }

    fn sign_into(&self, message: &[u8], out: &mut [u8]) -> Result<usize> {
        let signature = self.sign(message);
        copy_signature(&signature, out)
    }
}

impl Verifier for ed25519::PublicKey {
    #[inline]
    fn algorithm(&self) -> Algorithm {
        Algorithm::Ed25519
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        let signature: &[u8; 64] = signature.try_into().map_err(|_| Error::InvalidSignature)?;
        ed25519::PublicKey::verify(self, message, signature).map_err(|_| Error::InvalidSignature)
    }
}

fn copy_signature<const N: usize>(signature: &[u8; N], out: &mut [u8]) -> Result<usize> {
    if out.len() < N {
        return Err(Error::BufferTooSmall {
            required: N,
            available: out.len(),
        });
    }
    out[..N].copy_from_slice(signature);
    Ok(N)
}

////////////////////////////////////////////////////////////////////////////////
// ECDSA
////////////////////////////////////////////////////////////////////////////////

macro_rules! ecdsa_signer {
    ($secret:ty, $alg:expr, $len:expr) => {
        impl Signer for $secret {
            #[inline]
            fn algorithm(&self) -> Algorithm {
                $alg
            }

            #[inline]
            fn signature_len(&self) -> usize {
                $len
            }

            fn sign_into(&self, message: &[u8], out: &mut [u8]) -> Result<usize> {
                let signature = self.sign(message)?;
                copy_signature(&signature, out)
            }
        }
    };
}

macro_rules! ecdsa_verifier {
    ($public:ty, $alg:expr, $len:expr) => {
        impl Verifier for $public {
            #[inline]
            fn algorithm(&self) -> Algorithm {
                $alg
            }

            fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
                let signature: &[u8; $len] = signature.try_into().map_err(|_| Error::InvalidSignature)?;
                <$public>::verify(self, message, signature).map_err(|_| Error::InvalidSignature)
            }
        }
    };
}

ecdsa_signer!(p256::SecretKey, Algorithm::ESP256, 64);
ecdsa_verifier!(p256::PublicKey, Algorithm::ESP256, 64);
ecdsa_signer!(p384::SecretKey, Algorithm::ESP384, 96);
ecdsa_verifier!(p384::PublicKey, Algorithm::ESP384, 96);
ecdsa_signer!(p521::SecretKey, Algorithm::ESP512, 132);
ecdsa_verifier!(p521::PublicKey, Algorithm::ESP512, 132);

////////////////////////////////////////////////////////////////////////////////
// ML-DSA
////////////////////////////////////////////////////////////////////////////////

macro_rules! mldsa_verifier {
    ($public:ty, $alg:expr, $len:expr) => {
        impl Verifier for $public {
            #[inline]
            fn algorithm(&self) -> Algorithm {
                $alg
            }

            fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
                let signature: &[u8; $len] = signature.try_into().map_err(|_| Error::InvalidSignature)?;
                self.verify(message, signature, b"")
                    .map_err(|_| Error::InvalidSignature)
            }
        }
    };
}

#[cfg(feature = "random")]
macro_rules! mldsa_signer {
    ($secret:ty, $alg:expr, $len:expr) => {
        impl Signer for $secret {
            #[inline]
            fn algorithm(&self) -> Algorithm {
                $alg
            }

            #[inline]
            fn signature_len(&self) -> usize {
                $len
            }

            fn sign_into(&self, message: &[u8], out: &mut [u8]) -> Result<usize> {
                let signature = self.sign(message, b"")?;
                copy_signature(&signature, out)
            }
        }
    };
}

mldsa_verifier!(MlDsa44PublicKey, Algorithm::MlDsa44, ML_DSA_44_SIGNATURE_SIZE);
mldsa_verifier!(MlDsa65PublicKey, Algorithm::MlDsa65, ML_DSA_65_SIGNATURE_SIZE);
mldsa_verifier!(MlDsa87PublicKey, Algorithm::MlDsa87, ML_DSA_87_SIGNATURE_SIZE);

#[cfg(feature = "random")]
mldsa_signer!(MlDsa44SecretKey, Algorithm::MlDsa44, ML_DSA_44_SIGNATURE_SIZE);
#[cfg(feature = "random")]
mldsa_signer!(MlDsa65SecretKey, Algorithm::MlDsa65, ML_DSA_65_SIGNATURE_SIZE);
#[cfg(feature = "random")]
mldsa_signer!(MlDsa87SecretKey, Algorithm::MlDsa87, ML_DSA_87_SIGNATURE_SIZE);

////////////////////////////////////////////////////////////////////////////////
// RSA
////////////////////////////////////////////////////////////////////////////////

/// An RSA public key for COSE verification.
///
/// The algorithm is stored alongside the key because a bare
/// [`crypto::rsa::PublicKey`] cannot tell whether it must verify `RS*`
/// (PKCS#1 v1.5) or `PS*` (RSA-PSS) signatures, nor which hash to use.
pub struct RsaPublicKey {
    key: rsa::PublicKey,
    algorithm: Algorithm,
}

impl fmt::Debug for RsaPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RsaPublicKey")
            .field("algorithm", &self.algorithm)
            .finish()
    }
}

impl RsaPublicKey {
    /// Builds an RSA public key from raw big-endian modulus and exponent bytes.
    pub fn from_n_e(algorithm: Algorithm, n: &[u8], e: &[u8]) -> Result<Self> {
        if !matches!(
            algorithm,
            Algorithm::RS256
                | Algorithm::RS384
                | Algorithm::RS512
                | Algorithm::PS256
                | Algorithm::PS384
                | Algorithm::PS512
        ) {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        let key = rsa::PublicKey::from_n_e(n, e)?;
        Ok(Self {
            key,
            algorithm,
        })
    }

    /// Parses a PKCS#1 DER `SEQUENCE { INTEGER n, INTEGER e }`.
    pub fn from_pkcs1_der(algorithm: Algorithm, der: &[u8]) -> Result<Self> {
        if !matches!(
            algorithm,
            Algorithm::RS256
                | Algorithm::RS384
                | Algorithm::RS512
                | Algorithm::PS256
                | Algorithm::PS384
                | Algorithm::PS512
        ) {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        let key = rsa::PublicKey::from_pkcs1_der(der)?;
        Ok(Self {
            key,
            algorithm,
        })
    }

    /// Returns the algorithm.
    #[inline]
    pub fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    /// Returns the modulus as big-endian bytes.
    #[cfg(feature = "alloc")]
    pub fn n_bytes(&self) -> alloc::vec::Vec<u8> {
        self.key.n_bytes()
    }

    /// Returns the public exponent as big-endian bytes.
    #[cfg(feature = "alloc")]
    pub fn e_bytes(&self) -> alloc::vec::Vec<u8> {
        self.key.e_bytes().to_vec()
    }
}

impl Verifier for RsaPublicKey {
    #[inline]
    fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        match self.algorithm {
            Algorithm::RS256 => {
                let digest = Sha256::hash(message);
                self.key
                    .verify_pkcs1_v1_5(signature, digest.as_ref(), rsa::DIGEST_INFO_SHA256_PREFIX)
            }
            Algorithm::RS384 => {
                let digest = Sha384::hash(message);
                self.key
                    .verify_pkcs1_v1_5(signature, digest.as_ref(), rsa::DIGEST_INFO_SHA384_PREFIX)
            }
            Algorithm::RS512 => {
                let digest = Sha512::hash(message);
                self.key
                    .verify_pkcs1_v1_5(signature, digest.as_ref(), rsa::DIGEST_INFO_SHA512_PREFIX)
            }
            Algorithm::PS256 => self.key.verify_pss::<Sha256>(signature, message, Sha256::OUTPUT_SIZE),
            Algorithm::PS384 => self.key.verify_pss::<Sha384>(signature, message, Sha384::OUTPUT_SIZE),
            Algorithm::PS512 => self.key.verify_pss::<Sha512>(signature, message, Sha512::OUTPUT_SIZE),
            _ => return Err(Error::UnsupportedAlgorithm(self.algorithm)),
        }
        .map_err(|_| Error::InvalidSignature)
    }
}
