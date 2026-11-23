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
//! provide **confidentiality + integrity** in a single primitive. This crate offers two AEAD
//! constructions built on ChaCha20: [`ChaCha20Poly1305`] (RFC 8439) and
//! [`XChaCha20Poly1305`] (extended-nonce variant).
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
//! [`XChaCha20Ietf`] and [`XChaCha20Djb`] extend the nonce to 24 bytes by deriving a subkey with
//! [`hchacha20`], then encrypting with the IETF (32-bit counter) or original DJB (64-bit counter)
//! variant of ChaCha20 respectively. This allows random nonces with negligible collision
//! probability.
//!
//! # Examples
//!
//! ## AEAD usage (e.g. [`XChaCha20Poly1305`])
//!
//! ```
//! use crypto::{Aead, chacha::XChaCha20Poly1305};
//!
//! let key = [0xab; 32]; // WARNING: don't use static values here
//! let nonce = [0xcd; 24];
//! let aad = b"associated data";
//! let plaintext = b"hello world";
//!
//! let cipher = XChaCha20Poly1305::new(&key);
//!
//! let mut buf = plaintext.to_vec();
//! let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad)
//!     .expect("encryption failed");
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
//! cipher.xor_keystream(&mut plaintext).unwrap();
//! // plaintext is now encrypted
//! cipher.set_counter(0);
//!
//! cipher.xor_keystream(&mut plaintext).unwrap();
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
pub use chacha::{
    CHACHA20_IETF_MAX_LEN, ChaCha, ChaCha8Djb, ChaCha12Djb, ChaCha20Djb, ChaCha20Ietf, XChaCha, XChaCha20Djb,
    XChaCha20Ietf,
};

/// ChaCha20-Poly1305 AEAD construction (RFC 8439) and XChaCha20-Poly1305.
mod chacha20_poly1305;
pub use chacha20_poly1305::{ChaCha20Poly1305, XChaCha20Poly1305};

/// HChaCha20 hash function.
mod hchacha20;
pub use hchacha20::hchacha20;

/// Independent, safe reference ChaCha implementation used only by the unit
/// tests of the private SIMD kernels.
///
/// The SIMD kernel modules cannot reach `chacha_generic` (it is private to the
/// `chacha` module) and `#[cfg(test)]` integration tests cannot reach the
/// kernels, so this small helper gives the kernel tests a spec-level oracle for
/// the keystream, the advanced counter and the unconsumed tail of the last
/// block.
#[cfg(all(test, feature = "alloc"))]
pub(crate) mod simd_test_reference {
    use alloc::vec::Vec;

    use super::CONSTANT;

    #[inline(always)]
    fn quarter_round(s: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        s[a] = s[a].wrapping_add(s[b]);
        s[d] = (s[d] ^ s[a]).rotate_left(16);
        s[c] = s[c].wrapping_add(s[d]);
        s[b] = (s[b] ^ s[c]).rotate_left(12);
        s[a] = s[a].wrapping_add(s[b]);
        s[d] = (s[d] ^ s[a]).rotate_left(8);
        s[c] = s[c].wrapping_add(s[d]);
        s[b] = (s[b] ^ s[c]).rotate_left(7);
    }

    /// Computes one 64-byte ChaCha block from the given initial state.
    pub(crate) fn ref_block<const ROUNDS: usize>(initial: [u32; 16]) -> [u8; 64] {
        let mut s = initial;
        for _ in 0..ROUNDS / 2 {
            quarter_round(&mut s, 0, 4, 8, 12);
            quarter_round(&mut s, 1, 5, 9, 13);
            quarter_round(&mut s, 2, 6, 10, 14);
            quarter_round(&mut s, 3, 7, 11, 15);
            quarter_round(&mut s, 0, 5, 10, 15);
            quarter_round(&mut s, 1, 6, 11, 12);
            quarter_round(&mut s, 2, 7, 8, 13);
            quarter_round(&mut s, 3, 4, 9, 14);
        }
        let mut out = [0u8; 64];
        for i in 0..16 {
            out[i * 4..i * 4 + 4].copy_from_slice(&s[i].wrapping_add(initial[i]).to_le_bytes());
        }
        out
    }

    /// Builds an IETF-layout state from a key, a 12-byte nonce and a counter.
    pub(crate) fn ietf_state(key: &[u8; 32], nonce: &[u8; 12], counter: u32) -> [u32; 16] {
        let mut s = [0u32; 16];
        s[..4].copy_from_slice(&CONSTANT);
        for (i, word) in s[4..12].iter_mut().enumerate() {
            *word = u32::from_le_bytes(key[i * 4..i * 4 + 4].try_into().unwrap());
        }
        s[12] = counter;
        s[13] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        s[14] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        s[15] = u32::from_le_bytes(nonce[8..12].try_into().unwrap());
        s
    }

    /// Builds a DJB-layout state from a key, an 8-byte nonce and a counter.
    pub(crate) fn djb_state(key: &[u8; 32], nonce: &[u8; 8], counter: u64) -> [u32; 16] {
        let mut s = [0u32; 16];
        s[..4].copy_from_slice(&CONSTANT);
        for (i, word) in s[4..12].iter_mut().enumerate() {
            *word = u32::from_le_bytes(key[i * 4..i * 4 + 4].try_into().unwrap());
        }
        s[12] = counter as u32;
        s[13] = (counter >> 32) as u32;
        s[14] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        s[15] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        s
    }

    /// Returns the keystream starting at the counter stored in `state` for
    /// `len` bytes.  The counter is advanced block by block exactly like the
    /// kernels do (32-bit for IETF, 64-bit for DJB).
    pub(crate) fn ref_stream_from<const ROUNDS: usize>(state: &[u32; 16], is_ietf: bool, len: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(len.div_ceil(64) * 64);
        for j in 0..len.div_ceil(64) {
            let mut block_state = *state;
            if is_ietf {
                block_state[12] = state[12].wrapping_add(j as u32);
            } else {
                let counter = (((state[13] as u64) << 32) | state[12] as u64).wrapping_add(j as u64);
                block_state[12] = counter as u32;
                block_state[13] = (counter >> 32) as u32;
            }
            out.extend_from_slice(&ref_block::<ROUNDS>(block_state));
        }
        out.truncate(len);
        out
    }
}
