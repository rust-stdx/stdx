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
