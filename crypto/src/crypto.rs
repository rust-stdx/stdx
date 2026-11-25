//! Pure Rust cryptography with `no_std` support and hardware / SIMD acceleration
//! for `x86_64`, `aarch64` and WebAssembly.
//!
//! # ⚠️ Warning
//!
//! This crate has **not** undergone a third-party security audit yet. Use at your own risk.
#![allow(clippy::wrong_self_convention, clippy::module_inception)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

mod bytes;

pub mod aegis;
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
#[cfg(feature = "random")]
mod random;
pub mod sha2;
pub mod sha3;
pub mod turbohash;

#[cfg(feature = "alloc")]
pub mod encoding;
mod p_curves;
pub mod pbkdf2;
pub mod rsa;
pub(crate) use bytes::Bytes;
pub use bytes::Hash;
pub use p_curves::{p224, p256, p384, p521};

/// Maximum block size supported by the generic constructions that buffer a
/// hash block on the stack (HMAC and PBKDF2).
///
/// This is the largest `Hasher::BLOCK_SIZE` of any implementation in this
/// crate (SHAKE128, with a 168-byte rate). Implementations with a larger block
/// size are rejected at compile time by those constructions.
const MAX_HASH_BLOCK_SIZE: usize = 168;

/// Maximum output size supported by [`Hash`](struct@Hash).
pub const MAX_HASH_OUTPUT_SIZE: usize = 64;

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
pub enum HashError {
    TooLong,
}

impl core::fmt::Display for HashError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HashError::TooLong => write!(f, "hash is too long"),
        }
    }
}

impl core::error::Error for HashError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AeadError {
    /// The key is not valid for this cipher.
    InvalidKey,
    /// The nonce does not have the length required by this cipher.
    InvalidNonce,
    /// The ciphertext, tag, or associated data failed authentication.
    InvalidCiphertext,
    /// The message exceeds the cipher's length limit.
    MessageTooLong,
    /// The operation is not supported by this cipher.
    Unsupported,
}

impl core::fmt::Display for AeadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AeadError::InvalidKey => write!(f, "key is not valid"),
            AeadError::InvalidNonce => write!(f, "nonce is not valid"),
            AeadError::InvalidCiphertext => write!(f, "ciphertext is not valid"),
            AeadError::MessageTooLong => write!(f, "message is too long"),
            AeadError::Unsupported => write!(f, "operation is not supported"),
        }
    }
}

impl core::error::Error for AeadError {}

/// Errors returned by [`StreamCipher::xor_keystream`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamCipherError {
    /// The counter space for this `(key, nonce)` pair is exhausted. No bytes
    /// were XORed into the output buffer.
    CounterExhausted,
    /// The cipher was zeroized and cannot be reused. Construct a new cipher
    /// with a fresh key instead.
    Zeroized,
}

impl core::fmt::Display for StreamCipherError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StreamCipherError::CounterExhausted => f.write_str("stream cipher counter is exhausted"),
            StreamCipherError::Zeroized => f.write_str("stream cipher was zeroized"),
        }
    }
}

impl core::error::Error for StreamCipherError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EllipticCurveError {
    InvalidKey,
    Unspecified,
    InvalidSignature,
    InvalidSharedSecret,
    Random(RandomError),
}

impl core::fmt::Display for EllipticCurveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EllipticCurveError::InvalidKey => write!(f, "key is not valid"),
            EllipticCurveError::Unspecified => write!(f, "unknown error"),
            EllipticCurveError::InvalidSignature => write!(f, "signature is not valid"),
            EllipticCurveError::InvalidSharedSecret => write!(f, "shared secret is not valid"),
            EllipticCurveError::Random(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for EllipticCurveError {}

impl From<RandomError> for EllipticCurveError {
    fn from(err: RandomError) -> Self {
        EllipticCurveError::Random(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RsaError {
    InvalidKey,
    InvalidSignature,
    NotSupported,
    Unspecified,
}

impl core::fmt::Display for RsaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RsaError::InvalidKey => write!(f, "key is not valid"),
            RsaError::InvalidSignature => write!(f, "signature is not valid"),
            RsaError::NotSupported => write!(f, "key size not supported"),
            RsaError::Unspecified => write!(f, "unknown error"),
        }
    }
}

impl core::error::Error for RsaError {}

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

impl core::error::Error for HkdfError {}

/// Error returned when the operating system's random number generator fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomError {
    /// No random number generator is available on this platform.
    Unsupported,
    /// The operating system's random number generator failed with this OS
    /// error code.
    OsError(i64),
    /// An unexpected failure occurred while generating random data.
    Unspecified,
}

impl core::fmt::Display for RandomError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RandomError::Unsupported => write!(f, "random number generation is not supported on this platform"),
            RandomError::OsError(code) => write!(f, "random number generation failed with OS error {code}"),
            RandomError::Unspecified => write!(f, "random number generation failed"),
        }
    }
}

impl core::error::Error for RandomError {}

#[cfg(feature = "random")]
impl From<getrandom::Error> for RandomError {
    fn from(err: getrandom::Error) -> Self {
        if err == getrandom::Error::UNSUPPORTED {
            RandomError::Unsupported
        } else if let Some(code) = err.raw_os_error() {
            RandomError::OsError(code as i64)
        } else {
            RandomError::Unspecified
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait StreamCipher {
    /// XORs the next keystream bytes into `in_out`.
    ///
    /// Returns [`StreamCipherError::CounterExhausted`] instead of wrapping the
    /// counter, or [`StreamCipherError::Zeroized`] if the cipher has been
    /// zeroized. On error, no bytes are written to `in_out`.
    fn xor_keystream(&mut self, in_out: &mut [u8]) -> Result<(), StreamCipherError>;

    /// Overwrites `dst` with the next keystream bytes.
    ///
    /// This is the direct keystream counterpart of
    /// [`StreamCipher::xor_keystream`]. It is useful when the keystream is
    /// needed as-is, for example by a random number generator.
    ///
    /// Returns the same errors as [`StreamCipher::xor_keystream`]. On error,
    /// the contents of `dst` are unspecified.
    fn fill_keystream(&mut self, dst: &mut [u8]) -> Result<(), StreamCipherError> {
        dst.fill(0);
        self.xor_keystream(dst)
    }
}

pub trait Aead {
    const KEY_SIZE: usize;
    const TAG_SIZE: usize;
    const NONCE_SIZE: usize;

    /// Encrypts `in_out` in place and returns the authentication tag.
    ///
    /// # Errors
    ///
    /// Returns [`AeadError::InvalidNonce`] when `nonce.len() != Self::NONCE_SIZE`
    /// and [`AeadError::MessageTooLong`] when the message exceeds the cipher's
    /// length limit. On error, `in_out` is left unchanged.
    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Result<Hash, AeadError>;

    /// Authenticates and decrypts `in_out` in place.
    ///
    /// # Errors
    ///
    /// Returns [`AeadError::InvalidNonce`] when `nonce.len() != Self::NONCE_SIZE`,
    /// [`AeadError::InvalidCiphertext`] when the tag does not authenticate, and
    /// [`AeadError::MessageTooLong`] when the ciphertext exceeds the cipher's
    /// length limit.
    ///
    /// # Buffer contents on failure
    ///
    /// When the tag does not authenticate, this returns an error and the
    /// contents of `in_out` are **unspecified**: some implementations may have
    /// already written plaintext into the buffer before checking the tag, while
    /// others may leave it untouched or zero it. Callers must only use the
    /// buffer when `Ok(())` is returned.
    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError>;

    /// Encrypts `plaintext` and returns `ciphertext || tag`.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`encrypt_in_place`](Self::encrypt_in_place).
    #[cfg(feature = "alloc")]
    fn encrypt(&self, plaintext: &[u8], nonce: &[u8], aad: &[u8]) -> Result<Vec<u8>, AeadError> {
        let capacity = plaintext
            .len()
            .checked_add(Self::TAG_SIZE)
            .ok_or(AeadError::MessageTooLong)?;
        let mut ciphertext = Vec::with_capacity(capacity);
        ciphertext.extend_from_slice(plaintext);

        let tag = match self.encrypt_in_place(&mut ciphertext, nonce, aad) {
            Ok(tag) => tag,
            Err(err) => {
                // The caller still owns the input; wipe the duplicate copy
                // before dropping it.
                #[cfg(feature = "zeroize")]
                zeroize::Zeroize::zeroize(ciphertext.as_mut_slice());
                return Err(err);
            }
        };
        ciphertext.extend_from_slice(tag.as_ref());

        return Ok(ciphertext);
    }

    #[cfg(feature = "alloc")]
    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8], aad: &[u8]) -> Result<Vec<u8>, AeadError> {
        if ciphertext.len() < Self::TAG_SIZE {
            return Err(AeadError::InvalidCiphertext);
        }

        let plaintext_length = ciphertext.len() - Self::TAG_SIZE;
        let mut plaintext = Vec::with_capacity(plaintext_length);
        plaintext.extend_from_slice(&ciphertext[..plaintext_length]);

        if let Err(err) = self.decrypt_in_place(&mut plaintext, nonce, aad, &ciphertext[plaintext_length..]) {
            // A fused backend may have written unauthenticated plaintext here;
            // wipe it before dropping the buffer.
            #[cfg(feature = "zeroize")]
            zeroize::Zeroize::zeroize(plaintext.as_mut_slice());
            return Err(err);
        }

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

/// `Zeroize` is a no-op when the `zeroize` feature is not enabled.
pub trait Hasher: Clone + Zeroize {
    /// The internal block size of the hash function
    const BLOCK_SIZE: usize;
    /// The output size of the hash function.
    ///
    /// Must not exceed [`MAX_HASH_OUTPUT_SIZE`] (64), the fixed capacity of
    /// [`Hash`](struct@Hash); implementors with a larger `OUTPUT_SIZE` will not compile when
    /// used with functions that bound-check against this limit.
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

pub trait Xof: Send + Sync {
    /// Absorbs input bytes.
    fn absorb(&mut self, data: &[u8]);

    /// Squeezes output bytes.
    ///
    /// The first call finalizes the input phase; an empty `out` still performs
    /// that finalization. Subsequent calls continue the output stream.
    fn squeeze(&mut self, out: &mut [u8]);
}
