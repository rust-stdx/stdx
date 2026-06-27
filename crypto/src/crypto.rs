//! Pure Rust cryptography with `no_std` support and hardware / SIMD acceleration
//! for `x86_64` and `aarch64` (and sometimes WASM).
//!
//! # ⚠️ Warning
//!
//! This crate has **not** undergone a third-party security audit or formal
//! cryptographic review yet. Use at your own risk.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

mod bytes;
#[cfg(feature = "random")]
mod random;

pub mod aes;
#[cfg(feature = "alloc")]
pub mod argon2;
pub mod ascon;
pub mod blake2;
pub mod blake3;
pub mod chacha;
pub mod curve25519;
pub mod hkdf;
pub mod hmac;
pub mod mldsa;
pub mod mlkem;
pub mod poly1305;
pub mod sha2;
pub mod sha3;
pub mod xwing;

#[cfg(feature = "alloc")]
pub mod encoding;
pub mod p256;
pub mod pbkdf2;
pub mod rsa;
pub(crate) use bytes::Bytes;
pub use bytes::Hash;
#[cfg(feature = "random")]
pub use random::{random_bytes, random_fill};

const MAX_HASH_BLOCK_SIZE: usize = 128;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Errors
////////////////////////////////////////////////////////////////////////////////////////////////////

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Error {
//     Hkdf(HkdfError),
//     Aead(AeadError),
//     EllipticCurve(EllipticCurveError),
// }

// impl core::fmt::Display for Error {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             Error::Hkdf(err) => write!(f, "{err}"),
//             Error::Aead(err) => write!(f, "{err}"),
//             Error::EllipticCurve(err) => write!(f, "{err}"),
//         }
//     }
// }

// #[cfg(feature = "std")]
// impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AeadError {
    InvalidKey,
    InvalidNonce,
    InvalidCiphertext,
}

impl core::fmt::Display for AeadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AeadError::InvalidKey => write!(f, "key is not valid"),
            AeadError::InvalidNonce => write!(f, "nonce is not valid"),
            AeadError::InvalidCiphertext => write!(f, "ciphertext is not valid"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AeadError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EllipticCurveError {
    InvalidKey,
    Unspecified,
    InvalidSignature,
}

impl core::fmt::Display for EllipticCurveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EllipticCurveError::InvalidKey => write!(f, "key is not valid"),
            EllipticCurveError::Unspecified => write!(f, "unknown error"),
            EllipticCurveError::InvalidSignature => write!(f, "signature is not valid"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EllipticCurveError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HkdfError {
    PrkIsTooShort(usize),
    OutputIsTooLong,
}

impl core::fmt::Display for HkdfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HkdfError::PrkIsTooShort(_) => write!(f, "PRK is too short"),
            HkdfError::OutputIsTooLong => {
                write!(f, "HKDF output length exceeds RFC 5869 limit (255 * Hash's output size)")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HkdfError {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Traits
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait StreamCipher: Sized {
    fn xor_keystream(&mut self, in_out: &mut [u8]);
}

pub trait Aead: Sized {
    const TAG_SIZE: usize;
    const NONCE_SIZE: usize;

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash;

    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError>;

    #[cfg(feature = "alloc")]
    fn encrypt(&self, plaintext: &[u8], nonce: &[u8], aad: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len() + Self::TAG_SIZE);
        ciphertext.extend_from_slice(plaintext);

        let tag = self.encrypt_in_place(&mut ciphertext, nonce, aad);
        ciphertext.extend_from_slice(tag.as_ref());

        return ciphertext;
    }

    #[cfg(feature = "alloc")]
    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8], aad: &[u8]) -> Result<Vec<u8>, AeadError> {
        if ciphertext.len() < Self::TAG_SIZE {
            return Err(AeadError::InvalidCiphertext);
        }

        let plaintext_length = ciphertext.len() - Self::TAG_SIZE;
        let mut plaintext = Vec::with_capacity(plaintext_length);
        plaintext.extend_from_slice(&ciphertext[..plaintext_length]);

        self.decrypt_in_place(&mut plaintext, &nonce, aad, &ciphertext[plaintext_length..])?;

        return Ok(plaintext);
    }
}

#[cfg(feature = "zeroize")]
pub trait Zeroize: zeroize::Zeroize {}
#[cfg(feature = "zeroize")]
impl<T: zeroize::Zeroize> Zeroize for T {}

#[cfg(not(feature = "zeroize"))]
pub trait Zeroize {}
#[cfg(not(feature = "zeroize"))]
impl<T> Zeroize for T {}

pub trait Hasher: Sized + Clone + Zeroize {
    /// The internal block size of the hash function
    const BLOCK_SIZE: usize;
    /// The output size of the hash function
    const OUTPUT_SIZE: usize;

    fn new() -> Self;
    fn update(&mut self, data: &[u8]);
    fn sum(self) -> Hash;

    #[inline]
    fn hash(data: &[u8]) -> Hash {
        let mut hasher = Self::new();
        hasher.update(data);
        return hasher.sum();
    }
}

pub trait Xof: Sized + Send + Sync {
    fn absorb(&mut self, data: &[u8]);
    fn squeeze(&mut self, out: &mut [u8]);
}
