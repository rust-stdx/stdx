use alloc::vec::Vec;

use constant_time_eq::constant_time_eq;
use crypto::{
    Hasher,
    blake3::Blake3,
    curve25519::ed25519,
    hmac::Hmac,
    p256, rsa,
    sha2::{Sha256, Sha384, Sha512},
};
use smallvec::SmallVec;

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

impl TryFrom<Vec<u8>> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(signature: Vec<u8>) -> Result<Self, Self::Error> {
        signature.as_slice().try_into()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// BLAKE3
////////////////////////////////////////////////////////////////////////////////////////////////////

/// A `BLAKE3` key.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Blake3Key {
    key: [u8; 32],
}

impl Blake3Key {
    /// Generates a random 256-bit [`Blake3Key`].
    pub fn generate() -> Blake3Key {
        let key = crypto::random_bytes();
        return Blake3Key {
            key,
        };
    }

    #[inline(always)]
    pub fn from_bytes(key: &[u8; 32]) -> Blake3Key {
        return Blake3Key {
            key: *key,
        };
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8; 32] {
        return &self.key;
    }
}

impl Signer for Blake3Key {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        let signature = Blake3::keyed_hash(&self.key, message);
        return signature.as_ref().try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::BLAKE3
    }
}

impl Verifier for Blake3Key {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let expected_signature = Blake3::keyed_hash(&self.key, message);
        return match constant_time_eq(signature.as_ref(), &expected_signature) {
            true => Ok(()),
            false => Err(Error::InvalidSignature),
        };
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::BLAKE3
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// HMAC-SHA-256
////////////////////////////////////////////////////////////////////////////////////////////////////

/// A `HS256` key.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct HmacSha256Key {
    key: SmallVec<u8, 32>,
}

impl HmacSha256Key {
    /// Generates a random 256-bit [`HmacSha256Key`].
    pub fn generate() -> HmacSha256Key {
        let key: [u8; 32] = crypto::random_bytes();
        return HmacSha256Key {
            key: key.into(),
        };
    }

    /// Creates a new [`HmacSha256Key`] from bytes. Returns an error is the key is shorter than
    /// 16 bytes (128 bits).
    pub fn from_bytes(key: &[u8]) -> Result<HmacSha256Key, Error> {
        // require at least a 128-bit key
        if key.len() < 16 {
            return Err(Error::InvalidKey);
        }

        return Ok(HmacSha256Key {
            key: SmallVec::from_slice_copy(key),
        });
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        return &self.key;
    }
}

impl Signer for HmacSha256Key {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        let signature = Hmac::<Sha256>::mac(&self.key, message);
        return signature.as_ref().try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::HS256
    }
}

impl Verifier for HmacSha256Key {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let message_mac = Hmac::<Sha256>::mac(&self.key, message);
        return match constant_time_eq(&message_mac, signature) {
            true => Ok(()),
            false => Err(Error::InvalidSignature),
        };
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::HS256
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// HMAC-SHA-512
////////////////////////////////////////////////////////////////////////////////////////////////////

/// A `HS512` key.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct HmacSha512Key {
    key: SmallVec<u8, 32>,
}

impl HmacSha512Key {
    /// Generates a random 256-bit [`HmacSha512Key`].
    pub fn generate() -> HmacSha256Key {
        let key: [u8; 32] = crypto::random_bytes();
        return HmacSha256Key {
            key: key.into(),
        };
    }

    /// Creates a new [`HmacSha512Key`] from bytes. Returns an error is the key is shorter than
    /// 16 bytes (128 bits).
    pub fn from_bytes(key: &[u8]) -> Result<HmacSha512Key, Error> {
        // require at least a 128-bit key
        if key.len() < 16 {
            return Err(Error::InvalidKey);
        }

        return Ok(HmacSha512Key {
            key: SmallVec::from_slice_copy(key),
        });
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        return &self.key;
    }
}

impl Signer for HmacSha512Key {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        let signature = Hmac::<Sha512>::mac(&self.key, message);
        return signature.as_ref().try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::HS512
    }
}

impl Verifier for HmacSha512Key {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let message_mac = Hmac::<Sha512>::mac(&self.key, message);
        return match constant_time_eq(&message_mac, signature) {
            true => Ok(()),
            false => Err(Error::InvalidSignature),
        };
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::HS512
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ed25519
////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: zeroize?
pub struct Ed25519SecretKey {
    pub(crate) key: ed25519::SecretKey,
}

impl Ed25519SecretKey {
    /// Generates a random [`Ed25519SecretKey`].
    pub fn generate() -> Ed25519SecretKey {
        let key = ed25519::SecretKey::generate();
        return Ed25519SecretKey {
            key,
        };
    }

    /// Imports the private key from a seed.
    pub fn from_bytes(seed: &[u8; 32]) -> Result<Ed25519SecretKey, Error> {
        let key = ed25519::SecretKey::from_bytes(seed);
        return Ok(Ed25519SecretKey {
            key,
        });
    }

    /// Converts the private key to a byte array.
    #[inline(always)]
    pub(crate) fn to_bytes(&self) -> [u8; 32] {
        return self.key.to_bytes();
    }

    #[inline(always)]
    pub fn public_key(&self) -> Ed25519PublicKey {
        return Ed25519PublicKey {
            key: self.key.public_key(),
        };
    }
}

impl From<ed25519::SecretKey> for Ed25519SecretKey {
    fn from(key: ed25519::SecretKey) -> Self {
        Self {
            key,
        }
    }
}

impl Signer for Ed25519SecretKey {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return self.key.sign(message).as_ref().try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::EdDSA
    }
}

pub struct Ed25519PublicKey {
    key: ed25519::PublicKey,
}

impl Ed25519PublicKey {
    pub fn from_bytes(public_key: &[u8; 32]) -> Result<Ed25519PublicKey, Error> {
        let key = ed25519::PublicKey::from_bytes(public_key).map_err(|_| Error::InvalidKey)?;
        Ok(Ed25519PublicKey {
            key,
        })
    }

    /// Converts the public key to a byte array.
    #[inline(always)]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.key.to_bytes()
    }
}

impl From<ed25519::PublicKey> for Ed25519PublicKey {
    fn from(key: ed25519::PublicKey) -> Self {
        Self {
            key,
        }
    }
}

impl Verifier for Ed25519PublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let signature = signature.try_into().map_err(|_| Error::InvalidSignature)?;
        return self
            .key
            .verify(message, &signature)
            .map_err(|_| Error::InvalidSignature);
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::EdDSA
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// P-256
////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: zeroize
pub struct P256SecretKey {
    key: p256::SecretKey,
}

impl P256SecretKey {
    /// Generates a random [`P256SecretKey`].
    pub fn generate() -> Result<P256SecretKey, Error> {
        let key = p256::SecretKey::generate().map_err(|_| Error::InvalidKey)?;
        return Ok(P256SecretKey {
            key,
        });
    }

    #[inline(always)]
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<P256SecretKey, Error> {
        let key = p256::SecretKey::from_bytes(bytes).map_err(|_| Error::InvalidKey)?;
        return Ok(P256SecretKey {
            key,
        });
    }

    /// Converts the secret key to byte array.
    #[inline(always)]
    pub(crate) fn to_bytes(&self) -> [u8; 32] {
        return self.key.to_bytes();
    }

    #[inline(always)]
    pub fn public_key(&self) -> P256PublicKey {
        return P256PublicKey {
            key: self.key.public_key(),
        };
    }
}

impl From<p256::SecretKey> for P256SecretKey {
    fn from(key: p256::SecretKey) -> Self {
        Self {
            key,
        }
    }
}

impl Signer for P256SecretKey {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return self
            .key
            .sign(message)
            .map_err(|err| Error::Unspecified(alloc::format!("error signing message: {err:?}")))?
            .as_ref()
            .try_into();
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::ES256
    }
}

pub struct P256PublicKey {
    pub(crate) key: p256::PublicKey,
}

impl P256PublicKey {
    pub fn from_x_y(x: &[u8; 32], y: &[u8; 32]) -> Result<P256PublicKey, Error> {
        let key = p256::PublicKey::from_x_y(x, y).map_err(|_| Error::InvalidKey)?;
        Ok(P256PublicKey {
            key,
        })
    }

    pub fn from_bytes(public_key: &[u8]) -> Result<P256PublicKey, Error> {
        let key = p256::PublicKey::from_bytes(public_key).map_err(|_| Error::InvalidKey)?;
        Ok(P256PublicKey {
            key,
        })
    }

    // Converts the public key to a byte array.
    // #[inline(always)]
    // pub(crate) fn to_bytes(&self) -> [u8; 65] {
    //     self.key.to_bytes()
    // }
}

impl From<p256::PublicKey> for P256PublicKey {
    fn from(key: p256::PublicKey) -> Self {
        Self {
            key,
        }
    }
}

impl Verifier for P256PublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let signature = signature.try_into().map_err(|_| Error::InvalidSignature)?;
        return self
            .key
            .verify(message, &signature)
            .map_err(|_| Error::InvalidSignature);
    }

    #[inline(always)]
    fn algorithm(&self) -> Algorithm {
        Algorithm::ES256
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// RSA
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An RSA public key for JWT verification, supporting both PKCS#1 v1.5 and RSA-PSS signatures.
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
