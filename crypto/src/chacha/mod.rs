//! The ChaCha family of stream ciphers and AEADs.
//!
//! # Stream ciphers vs AEADs
//!
//! A **stream cipher** (`ChaCha`, `XChaCha`) generates a keystream from a key and nonce and XORs it
//! with plaintext to produce ciphertext. It provides **confidentiality** but *no* integrity or
//! authentication. An attacker can modify the ciphertext and produce garbage on decryption.
//! Use a stream cipher when you only need secrecy and are handling authentication
//! separately.
//!
//! An **AEAD** (Authenticated Encryption with Associated Data) wraps a stream cipher with a MAC to
//! provide **confidentiality + integrity** in a single primitive. This crate offers three AEAD
//! constructions built on ChaCha20:  [`ChaCha20Blake3`] (encrypt-then-MAC with BLAKE3),
//! [`ChaCha20Poly1305`] (RFC 8439), and [`XChaCha20Poly1305`] (extended-nonce variant).
//!
//! # ChaChaDjb vs ChaChaIetf
//!
//! The original ChaCha design by Daniel J. Bernstein uses a **64-bit counter** and a **64-bit nonce**
//! (8 bytes). These are the `Djb` variants (`ChaCha8Djb`, `ChaCha12Djb`, `ChaCha20Djb`).
//!
//! The IETF variant (RFC 8439) uses a **32-bit counter** and a **96-bit nonce** (12 bytes). This is
//! [`ChaCha20Ietf`]. The IETF layout is required by TLS 1.3 and is used as the inner cipher for
//! the [`ChaCha20Poly1305`] AEAD.
//!
//! [`XChaCha20`] extends the nonce to 24 bytes by deriving a subkey with [`hchacha20`], then encrypting
//! with the IETF variant of ChaCha20. This allows random nonces with negligible collision probability.
//!
//! # Examples
//!
//! ## AEAD usage (e.g. [`ChaCha20Blake3`])
//!
//! ```
//! use crypto::{Aead, chacha::ChaCha20Blake3};
//!
//! let key = [0xab; 32]; // WARNING: don't use static values here
//! let nonce = [0xcd; 32];
//! let aad = b"associated data";
//! let plaintext = b"hello world";
//!
//! let cipher = ChaCha20Blake3::new(&key);
//!
//! let mut buf = plaintext.to_vec();
//! let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad);
//!
//! cipher.decrypt_in_place(&mut buf, &nonce, aad, tag.as_ref())
//!     .expect("decryption failed");
//! assert_eq!(&buf, plaintext);
//! ```
//!
//! ## Stream cipher usage (e.g. [`ChaCha20Djb`])
//!
//! ```
//! use crypto::{StreamCipher, chacha::ChaCha20Djb};
//!
//! let key = [0xab; 32];
//! let nonce = [0xcd; 8];
//! let mut plaintext = *b"hello world";
//!
//! let mut cipher = ChaCha20Djb::new(&key, &nonce);
//! cipher.xor_keystream(&mut plaintext);
//! // plaintext is now encrypted
//! cipher.set_counter(0);
//!
//! cipher.xor_keystream(&mut plaintext);
//! // plaintext is back to "hello world" (XOR is its own inverse)
//! assert_eq!(&plaintext, b"hello world");
//! ```

// aarch64 assumes that NEON instructions are always present
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod chacha_neon;

// import if the target runtime supports the feature
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod chacha_wasm_simd128;

// import if runtime CPU features detection is enabled or if the target CPU supports the feature
#[cfg(any(
    all(target_arch = "x86_64", feature = "std"),
    all(target_arch = "x86_64", target_feature = "avx2")
))]
mod chacha_avx2;

// import if runtime CPU features detection is enabled or if the target CPU supports the feature
#[cfg(any(
    all(target_arch = "x86_64", feature = "std"),
    all(target_arch = "x86_64", target_feature = "avx512f")
))]
mod chacha_avx512;

/// ChaCha and XChaCha cipher implementations.
mod chacha;

pub(crate) use chacha::{BLOCK_SIZE, CONSTANT, STATE_WORDS, quarter_round};
pub use chacha::{ChaCha, ChaCha8Djb, ChaCha12Djb, ChaCha20Djb, ChaCha20Ietf, XChaCha, XChaCha20};

mod chacha20_blake3;
pub use chacha20_blake3::ChaCha20Blake3;

/// ChaCha20-Poly1305 AEAD construction (RFC 8439) and XChaCha20-Poly1305.
mod chacha20_poly1305;
pub use chacha20_poly1305::{ChaCha20Poly1305, XChaCha20Poly1305};

/// HChaCha20 hash function.
mod hchacha20;
pub use hchacha20::hchacha20;
