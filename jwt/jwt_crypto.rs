use alloc::vec::Vec;

use constant_time_eq::constant_time_eq;
use crypto::{
    blake3::Blake3,
    curve25519::ed25519,
    hmac::Hmac,
    p256,
    sha2::{Sha256, Sha512},
};
use smallvec::SmallVec;

use crate::{Algorithm, Error};

pub(crate) const SIGNATURE_MAX_SIZE: usize = 132;

pub trait Signer {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error>;
    fn algorithm(&self) -> Algorithm;
}

pub trait Verifier {
    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), Error>;
    fn algorithm(&self) -> Algorithm;
}

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
    pub fn new(key: &[u8; 32]) -> Blake3Key {
        return Blake3Key {
            key: *key,
        };
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        return &self.key;
    }
}

impl Signer for Blake3Key {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        let signature = Blake3::keyed_hash(&self.key, message);
        return signature.as_ref().try_into();
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::BLAKE3
    }
}

impl Verifier for Blake3Key {
    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), Error> {
        let expected_signature = Blake3::keyed_hash(&self.key, message);
        return match constant_time_eq(signature.as_ref(), &expected_signature) {
            true => Ok(()),
            false => Err(Error::InvalidSignature),
        };
    }

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
    pub fn new(key: &[u8]) -> Result<HmacSha256Key, Error> {
        // require at least a 128-bit key
        if key.len() < 16 {
            return Err(Error::InvalidKey);
        }

        return Ok(HmacSha256Key {
            key: SmallVec::from_slice_copy(key),
        });
    }

    pub fn as_bytes(&self) -> &[u8] {
        return &self.key;
    }
}

impl Signer for HmacSha256Key {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        let signature = Hmac::<Sha256>::mac(&self.key, message);
        return signature.as_ref().try_into();
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::HS256
    }
}

impl Verifier for HmacSha256Key {
    fn verify(&self, message: &[u8], expected_signature: &Signature) -> Result<(), Error> {
        let message_mac = Hmac::<Sha256>::mac(&self.key, message);
        return match constant_time_eq(&message_mac, &expected_signature) {
            true => Ok(()),
            false => Err(Error::InvalidSignature),
        };
    }

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
    pub fn new(key: &[u8]) -> Result<HmacSha512Key, Error> {
        // require at least a 128-bit key
        if key.len() < 16 {
            return Err(Error::InvalidKey);
        }

        return Ok(HmacSha512Key {
            key: SmallVec::from_slice_copy(key),
        });
    }

    pub fn as_bytes(&self) -> &[u8] {
        return &self.key;
    }
}

impl Signer for HmacSha512Key {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        let signature = Hmac::<Sha512>::mac(&self.key, message);
        return signature.as_ref().try_into();
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::HS512
    }
}

impl Verifier for HmacSha512Key {
    fn verify(&self, message: &[u8], expected_signature: &Signature) -> Result<(), Error> {
        let message_mac = Hmac::<Sha512>::mac(&self.key, message);
        return match constant_time_eq(&message_mac, &expected_signature) {
            true => Ok(()),
            false => Err(Error::InvalidSignature),
        };
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::HS512
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ed25519
////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: zeroize?
pub struct Ed25519SecretKey {
    key: ed25519::SecretKey,
}

impl Ed25519SecretKey {
    pub fn generate() -> Ed25519SecretKey {
        let key = ed25519::SecretKey::generate();
        return Ed25519SecretKey {
            key,
        };
    }

    pub fn from_bytes(seed: &[u8; 32]) -> Result<Ed25519SecretKey, Error> {
        let key = ed25519::SecretKey::from_bytes(seed);
        return Ok(Ed25519SecretKey {
            key,
        });
    }

    /// Converts the private key to byte array.
    pub fn to_bytes(&self) -> [u8; 32] {
        return self.key.to_bytes();
    }

    pub fn public_key(&self) -> Ed25519PublicKey {
        return Ed25519PublicKey {
            key: self.key.public_key(),
        };
    }
}

impl Signer for Ed25519SecretKey {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return self.key.sign(message).as_ref().try_into();
    }

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
    pub fn to_bytes(&self) -> [u8; 32] {
        self.key.to_bytes()
    }
}

impl Verifier for Ed25519PublicKey {
    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), Error> {
        let signature = signature.as_ref().try_into().map_err(|_| Error::InvalidSignature)?;
        return self
            .key
            .verify(message, &signature)
            .map_err(|_| Error::InvalidSignature);
    }

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
    pub fn generate() -> Result<P256SecretKey, Error> {
        let key = p256::SecretKey::generate().map_err(|_| Error::InvalidKey)?;
        return Ok(P256SecretKey {
            key,
        });
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Result<P256SecretKey, Error> {
        let key = p256::SecretKey::from_bytes(bytes).map_err(|_| Error::InvalidKey)?;
        return Ok(P256SecretKey {
            key,
        });
    }

    /// Converts the secret key to byte array.
    pub fn to_bytes(&self) -> [u8; 32] {
        return self.key.to_bytes();
    }

    pub fn public_key(&self) -> P256PublicKey {
        return P256PublicKey {
            key: self.key.public_key(),
        };
    }
}

impl Signer for P256SecretKey {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        return self
            .key
            .sign(message)
            .map_err(|err| Error::Unspecified(err.to_string()))?
            .as_ref()
            .try_into();
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::ES256
    }
}

pub struct P256PublicKey {
    key: p256::PublicKey,
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

    /// Converts the public key to a byte array.
    pub fn to_bytes(&self) -> [u8; 65] {
        self.key.to_bytes()
    }
}

impl Verifier for P256PublicKey {
    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), Error> {
        let signature = signature.as_ref().try_into().map_err(|_| Error::InvalidSignature)?;
        return self
            .key
            .verify(message, &signature)
            .map_err(|_| Error::InvalidSignature);
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::ES256
    }
}
