use constant_time_eq::constant_time_eq;
use crypto::{
    Hash, Hasher,
    blake3::Blake3,
    curve25519::ed25519,
    hmac::Hmac,
    mldsa::{
        ML_DSA_44_SIGNATURE_SIZE, ML_DSA_65_SIGNATURE_SIZE, MlDsa44PublicKey, MlDsa44SecretKey, MlDsa65PublicKey,
        MlDsa65SecretKey,
    },
    p256, rsa,
    sha2::{Sha256, Sha384, Sha512},
};

use crate::{Algorithm, Error};

pub(crate) const SIGNATURE_MAX_SIZE: usize = 3309; // ML-DSA-65

pub trait Signer {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error>;
    fn algorithm(&self) -> Algorithm;
}

pub trait Verifier {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error>;
    fn algorithm(&self) -> Algorithm;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Signature
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Signature {
    value: [u8; SIGNATURE_MAX_SIZE],
    length: usize,
}

impl core::ops::Deref for Signature {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.value[..self.length as usize]
    }
}

impl AsRef<[u8]> for Signature {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.value[..self.length]
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(signature: &[u8]) -> Result<Self, Self::Error> {
        let length = signature.len();
        if length > SIGNATURE_MAX_SIZE {
            return Err(Error::InvalidSignature);
        }

        let mut value = [0u8; SIGNATURE_MAX_SIZE];
        value[..length].copy_from_slice(signature);

        return Ok(Signature {
            value,
            length,
        });
    }
}

impl<const N: usize> TryFrom<[u8; N]> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(signature: [u8; N]) -> Result<Self, Self::Error> {
        signature.as_slice().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(signature: &[u8; N]) -> Result<Self, Self::Error> {
        signature.as_slice().try_into()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Secret key (BLAKE3 / HMAC)
////////////////////////////////////////////////////////////////////////////////////////////////////

/// A symmetric secret key used with the `BLAKE3`, `HS256`, `HS384`, and `HS512` algorithms.
///
/// The key is borrowed, so it must outlive any signing or verification.
///
/// Signing and verification fail with [`Error::InvalidKey`] if `algorithm` is not one of the
/// supported MAC algorithms, if a `BLAKE3` key is not exactly 32 bytes long, or if an HMAC key is
/// shorter than 16 bytes (128 bits).
pub struct SecretKey<'a> {
    pub(crate) key: &'a [u8],
    pub(crate) algorithm: Algorithm,
}

impl<'a> SecretKey<'a> {
    /// Creates a new [`SecretKey`] for `algorithm`.
    #[inline(always)]
    pub fn new(algorithm: Algorithm, key: &'a [u8]) -> Self {
        Self {
            key,
            algorithm,
        }
    }

    /// Computes the message authentication code of `message`.
    fn mac(&self, message: &[u8]) -> Result<Hash, Error> {
        match self.algorithm {
            Algorithm::BLAKE3 => {
                let key: &[u8; 32] = self.key.try_into().map_err(|_| Error::InvalidKey)?;
                Ok(Blake3::keyed_hash(key, message))
            }
            Algorithm::HS256 => self.hmac::<Sha256>(message),
            Algorithm::HS384 => self.hmac::<Sha384>(message),
            Algorithm::HS512 => self.hmac::<Sha512>(message),
            _ => Err(Error::InvalidKey),
        }
    }

    /// Computes the HMAC of `message` with `H`, requiring at least a 128-bit key.
    fn hmac<H: Hasher>(&self, message: &[u8]) -> Result<Hash, Error> {
        if self.key.len() < 16 {
            return Err(Error::InvalidKey);
        }

        Ok(Hmac::<H>::mac(self.key, message))
    }
}

impl Signer for SecretKey<'_> {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return self.mac(message)?.as_ref().try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        self.algorithm
    }
}

impl Verifier for SecretKey<'_> {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let mac = self.mac(message)?;
        return match constant_time_eq(mac.as_ref(), signature) {
            true => Ok(()),
            false => Err(Error::InvalidSignature),
        };
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        self.algorithm
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ed25519
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Signer for ed25519::SecretKey {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return ed25519::SecretKey::sign(self, message).as_ref().try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::EdDSA
    }
}

impl Verifier for ed25519::PublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let signature = signature.try_into().map_err(|_| Error::InvalidSignature)?;
        return ed25519::PublicKey::verify(self, message, &signature).map_err(|_| Error::InvalidSignature);
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::EdDSA
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// P-256
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Signer for p256::SecretKey {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return p256::SecretKey::sign(self, message)
            .map_err(|err| Error::Unspecified(alloc::format!("error signing message: {err:?}")))?
            .as_ref()
            .try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::ES256
    }
}

impl Verifier for p256::PublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let signature = signature.try_into().map_err(|_| Error::InvalidSignature)?;
        return p256::PublicKey::verify(self, message, &signature).map_err(|_| Error::InvalidSignature);
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::ES256
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ML-DSA-44
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Signer for MlDsa44SecretKey {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return MlDsa44SecretKey::sign(self, message, b"")
            .map_err(|_| Error::InvalidSignature)?
            .as_ref()
            .try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::MlDsa44
    }
}

impl Verifier for MlDsa44PublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let signature: &[u8; ML_DSA_44_SIGNATURE_SIZE] = signature.try_into().map_err(|_| Error::InvalidSignature)?;
        return MlDsa44PublicKey::verify(self, message, signature, b"").map_err(|_| Error::InvalidSignature);
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::MlDsa44
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ML-DSA-65
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Signer for MlDsa65SecretKey {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return MlDsa65SecretKey::sign(self, message, b"")
            .map_err(|_| Error::InvalidSignature)?
            .as_ref()
            .try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::MlDsa65
    }
}

impl Verifier for MlDsa65PublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let signature: &[u8; ML_DSA_65_SIGNATURE_SIZE] = signature.try_into().map_err(|_| Error::InvalidSignature)?;
        return MlDsa65PublicKey::verify(self, message, signature, b"").map_err(|_| Error::InvalidSignature);
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::MlDsa65
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// RSA
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An RSA public key for JWT verification, supporting both PKCS#1 v1.5 and RSA-PSS signatures.
///
/// The algorithm is stored alongside the key because a bare [`rsa::PublicKey`] cannot tell
/// whether it must verify `RS*` (PKCS#1 v1.5) or `PS*` (RSA-PSS) signatures, nor which hash to use.
///
/// # Algorithms
///
/// | Variant   | JWT Algorithm | Scheme          | Hash     |
/// |-----------|---------------|-----------------|----------|
/// | RS256     | `RS256`       | PKCS#1 v1.5     | SHA-256  |
/// | RS384     | `RS384`       | PKCS#1 v1.5     | SHA-384  |
/// | RS512     | `RS512`       | PKCS#1 v1.5     | SHA-512  |
/// | PS256     | `PS256`       | RSA-PSS         | SHA-256  |
/// | PS384     | `PS384`       | RSA-PSS         | SHA-384  |
/// | PS512     | `PS512`       | RSA-PSS         | SHA-512  |
///
/// Signing is not supported — this key type is verification-only.
///
/// # Constructors
///
/// * [`RsaPublicKey::from_n_e`] — build from raw modulus and exponent bytes (useful with JWK)
/// * [`RsaPublicKey::from_pkcs1_der`] — parse from PKCS#1 DER `SEQUENCE { INTEGER n, INTEGER e }`
///
/// # Errors
///
/// Returns [`Error::InvalidKey`] if the algorithm is not an RSA variant or
/// if the underlying RSA key parsing fails. Returns [`Error::InvalidSignature`]
/// on verification failures.
pub struct RsaPublicKey {
    pub(crate) key: rsa::PublicKey,
    pub(crate) alg: Algorithm,
}

impl RsaPublicKey {
    /// Build an RSA public key from raw modulus `n` and public exponent `e`
    /// (both big-endian byte slices).
    ///
    /// This is useful when importing keys from JWK format where `n` and `e`
    /// are base64url-encoded big-endian byte values.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidKey`] if `alg` is not an RSA algorithm
    /// or if the modulus/exponent bytes describe an invalid RSA key.
    pub(crate) fn from_n_e(alg: Algorithm, n: &[u8], e: &[u8]) -> Result<Self, Error> {
        if !matches!(
            alg,
            Algorithm::RS256
                | Algorithm::RS384
                | Algorithm::RS512
                | Algorithm::PS256
                | Algorithm::PS384
                | Algorithm::PS512
        ) {
            return Err(Error::InvalidKey);
        }
        let key = rsa::PublicKey::from_n_e(n, e).map_err(|_| Error::InvalidKey)?;
        Ok(RsaPublicKey {
            key,
            alg,
        })
    }

    /// Parse an RSA public key from PKCS#1 DER bytes.
    ///
    /// The input is the raw `SEQUENCE { INTEGER n, INTEGER e }` inside the
    /// `SubjectPublicKeyInfo` BIT STRING.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidKey`] if `alg` is not an RSA algorithm
    /// or if the DER bytes do not encode a valid RSA public key.
    pub fn from_pkcs1_der(pkcs1_der: &[u8], alg: Algorithm) -> Result<Self, Error> {
        if !matches!(
            alg,
            Algorithm::RS256
                | Algorithm::RS384
                | Algorithm::RS512
                | Algorithm::PS256
                | Algorithm::PS384
                | Algorithm::PS512
        ) {
            return Err(Error::InvalidKey);
        }
        let key = rsa::PublicKey::from_pkcs1_der(pkcs1_der).map_err(|_| Error::InvalidKey)?;
        Ok(RsaPublicKey {
            key,
            alg,
        })
    }
}

impl Verifier for RsaPublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        match self.alg {
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
            _ => return Err(Error::InvalidKey),
        }
        .map_err(|_| Error::InvalidSignature)
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        self.alg
    }
}
