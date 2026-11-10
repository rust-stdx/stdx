//! AEGIS authenticated encryption, MACs and stream ciphers (RFC 10032).
//!
//! This module implements the 256-bit AEGIS family:
//!
//! - [`Aegis256`] — the standard 768-bit-state AEAD.
//! - [`Aegis256X2`] — the two-lane parallel mode. It profits most from CPUs
//!   with 256-bit vector AES registers, but also runs on 128-bit AES-NI/NEON.
//! - [`Aegis256X4`] — the four-lane parallel mode. It profits most from CPUs
//!   with 512-bit vector AES registers (VAES-512), and also runs on narrower
//!   AES-NI/NEON implementations at a lower benefit.
//!
//! It also provides the corresponding message authentication codes
//! ([`Aegis256Mac`], [`Aegis256X2Mac`], [`Aegis256X4Mac`]) and stream ciphers
//! ([`Aegis256Stream`], [`Aegis256X2Stream`], [`Aegis256X4Stream`]).
//!
//! # Parameters
//!
//! | Variant | Key | Nonce | Rate | Tag |
//! |---|---|---|---|---|
//! | AEGIS-256 | 32 B | 32 B | 128 bits | 16 or 32 B |
//! | AEGIS-256X2 | 32 B | 32 B | 256 bits | 16 or 32 B |
//! | AEGIS-256X4 | 32 B | 32 B | 512 bits | 16 or 32 B |
//!
//! The tag size is selected at compile time: [`Aegis256::<16>`](Aegis256) for
//! the common 128-bit tag, [`Aegis256::<32>`](Aegis256) for a 256-bit tag.
//!
//! # Security
//!
//! - A nonce must never be reused with the same key, including across tag
//!   lengths.
//! - Keys must be chosen uniformly at random.
//! - AEGIS is key-committing in the receiver-binding game, but not fully
//!   committing when an attacker controls the associated data. Protocols that
//!   require full commitment should hash the associated data with a
//!   collision-resistant hash such as SHA-256 first.
//! - AEGIS-256 and AEGIS-256X2/X4 can safely use random nonces with no
//!   practical message limit.
//! - When decryption runs and the tag does not authenticate, the plaintext
//!   buffer is zeroed before the error is returned. A nonce or tag with the
//!   wrong length is rejected before decryption, so the buffer is untouched in
//!   that case (callers must not use it on any error path).
//!
//! - AEGIS-256 is the default choice. Use a parallel mode only when every
//!   participant agrees on the exact variant and tag size.
//! - Do not use the same key for AEAD encryption and AEGISMAC, or for AEAD
//!   encryption and the stream-cipher mode. Use a dedicated key per purpose.
//!
//! The one-shot AEAD types retain the 32-byte key so they can run `Init` again
//! for every nonce. The MAC and stream-cipher types run `Init` once in their
//! constructor and do not retain the key.
//!
//! # Backends
//!
//! | Target | AEGIS-256 | AEGIS-256X2 | AEGIS-256X4 |
//! |---|---|---|---|
//! | aarch64 | NEON + AES, SHA3 `eor3` when available | same | NEON + AES |
//! | x86-64 | AES-NI | VAES-256, or AES-NI | VAES-512, VAES-256, or AES-NI |
//! | wasm32 + `simd128` | constant-time swizzle AES | lane-major swizzle AES | lane-major swizzle AES |
//! | other | constant-time bitsliced AES | same | same |
//!
//! Runtime detection is used when the `std` feature is enabled. `no_std`
//! builds select a backend from the compile-time `target_feature` settings;
//! without AES features they use the portable constant-time fallback.
//!
//! On aarch64 the SHA3 `eor3` instruction is used for AEGIS-256 and
//! AEGIS-256X2 when the CPU supports it. It mainly helps the two-lane mode
//! (about 13% on long messages); for AEGIS-256 it is neutral. AEGIS-256X4
//! keeps the plain NEON path: for the four-lane AEAD loop `eor3` was about 7%
//! slower in measurements because of register pressure.
//!
//! On wasm32 the parallel modes use a lane-major kernel. wasm `v128` is only
//! 128 bits wide, so X2/X4 would otherwise keep 12 or 24 state vectors live and
//! spill heavily. Processing one lane at a time keeps the working set close to
//! AEGIS-256 and closes most of the long-message gap. Short messages remain
//! slower for X2/X4 because initialization and finalization perform `D` times
//! the AES-round work that cannot be amortized over a small payload.

mod aegis_core;
mod aegis_soft;

#[cfg(target_arch = "x86_64")]
mod aegis_amd64;
#[cfg(target_arch = "aarch64")]
mod aegis_arm64;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod aegis_wasm32_simd128;

use constant_time_eq::constant_time_eq;

use crate::{Aead, AeadError, Hash, StreamCipher};

/// Largest supported plaintext / associated-data length (RFC 10032: 2^61 - 1
/// bytes).
const MAX_INPUT_LEN: u64 = (1u64 << 61) - 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Backend {
    #[cfg(target_arch = "aarch64")]
    Arm,
    /// ARMv8 AES + the SHA3 `eor3` instruction (256 and X2 only).
    #[cfg(target_arch = "aarch64")]
    ArmSha3,
    #[cfg(target_arch = "x86_64")]
    X86AesNi,
    #[cfg(target_arch = "x86_64")]
    X86Vaes256,
    #[cfg(target_arch = "x86_64")]
    X86Vaes512,
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    Wasm,
    /// Lane-major WebAssembly kernel for the parallel modes.
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    WasmLane,
    Soft,
}

#[inline]
#[allow(unreachable_code)]
fn detect<const D: usize>() -> Backend {
    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(feature = "std")]
        {
            if std::arch::is_aarch64_feature_detected!("aes") {
                if D <= 2 && std::arch::is_aarch64_feature_detected!("sha3") {
                    return Backend::ArmSha3;
                }
                return Backend::Arm;
            }
            return Backend::Soft;
        }
        #[cfg(not(feature = "std"))]
        {
            if cfg!(target_feature = "aes") {
                if D <= 2 && cfg!(target_feature = "sha3") {
                    return Backend::ArmSha3;
                }
                return Backend::Arm;
            }
            return Backend::Soft;
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        #[cfg(feature = "std")]
        {
            let aes = std::arch::is_x86_feature_detected!("aes");
            let vaes =
                aes && std::arch::is_x86_feature_detected!("vaes") && std::arch::is_x86_feature_detected!("avx2");
            let vaes512 = vaes
                && std::arch::is_x86_feature_detected!("avx512f")
                && std::arch::is_x86_feature_detected!("avx512bw");
            return x86_select::<D>(aes, vaes, vaes512);
        }
        #[cfg(not(feature = "std"))]
        {
            let aes = cfg!(target_feature = "aes");
            let vaes = cfg!(all(target_feature = "vaes", target_feature = "avx2", target_feature = "aes"));
            let vaes512 = cfg!(all(
                target_feature = "vaes",
                target_feature = "avx2",
                target_feature = "avx512f",
                target_feature = "avx512bw"
            ));
            return x86_select::<D>(aes, vaes, vaes512);
        }
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        if D > 1 {
            return Backend::WasmLane;
        }
        return Backend::Wasm;
    }

    Backend::Soft
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn x86_select<const D: usize>(aes: bool, vaes: bool, vaes512: bool) -> Backend {
    if D == 1 {
        if aes { Backend::X86AesNi } else { Backend::Soft }
    } else if D == 2 {
        if vaes {
            Backend::X86Vaes256
        } else if aes {
            Backend::X86AesNi
        } else {
            Backend::Soft
        }
    } else if vaes512 {
        Backend::X86Vaes512
    } else if vaes {
        Backend::X86Vaes256
    } else if aes {
        Backend::X86AesNi
    } else {
        Backend::Soft
    }
}

#[inline(always)]
fn encrypt_impl<const D: usize>(
    backend: Backend,
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &mut [u8],
) {
    assert!((ad.len() as u64) <= MAX_INPUT_LEN && (msg.len() as u64) <= MAX_INPUT_LEN);
    match backend {
        #[cfg(target_arch = "aarch64")]
        Backend::Arm => unsafe { aegis_arm64::encrypt::<D>(key, nonce, ad, msg, tag) },
        #[cfg(target_arch = "aarch64")]
        Backend::ArmSha3 => unsafe { aegis_arm64::encrypt_sha3::<D>(key, nonce, ad, msg, tag) },
        #[cfg(target_arch = "x86_64")]
        Backend::X86AesNi => {
            if D == 1 {
                unsafe { aegis_amd64::encrypt_aesni_1(key, nonce, ad, msg, tag) }
            } else if D == 2 {
                unsafe { aegis_amd64::encrypt_aesni_2(key, nonce, ad, msg, tag) }
            } else {
                unsafe { aegis_amd64::encrypt_aesni_4(key, nonce, ad, msg, tag) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes256 => {
            if D == 2 {
                unsafe { aegis_amd64::encrypt_vaes256_2(key, nonce, ad, msg, tag) }
            } else {
                unsafe { aegis_amd64::encrypt_vaes256_4(key, nonce, ad, msg, tag) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes512 => unsafe { aegis_amd64::encrypt_vaes512_4(key, nonce, ad, msg, tag) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::Wasm => unsafe { aegis_wasm32_simd128::encrypt::<D>(key, nonce, ad, msg, tag) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::WasmLane => unsafe { aegis_wasm32_simd128::encrypt_lane::<D>(key, nonce, ad, msg, tag) },
        Backend::Soft => aegis_core::encrypt::<aegis_soft::Soft<D>>(key, nonce, ad, msg, tag),
    }
}

#[inline(always)]
fn decrypt_impl<const D: usize>(
    backend: Backend,
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &[u8],
) -> bool {
    assert!((ad.len() as u64) <= MAX_INPUT_LEN && (msg.len() as u64) <= MAX_INPUT_LEN);
    match backend {
        #[cfg(target_arch = "aarch64")]
        Backend::Arm => unsafe { aegis_arm64::decrypt::<D>(key, nonce, ad, msg, tag) },
        #[cfg(target_arch = "aarch64")]
        Backend::ArmSha3 => unsafe { aegis_arm64::decrypt_sha3::<D>(key, nonce, ad, msg, tag) },
        #[cfg(target_arch = "x86_64")]
        Backend::X86AesNi => {
            if D == 1 {
                unsafe { aegis_amd64::decrypt_aesni_1(key, nonce, ad, msg, tag) }
            } else if D == 2 {
                unsafe { aegis_amd64::decrypt_aesni_2(key, nonce, ad, msg, tag) }
            } else {
                unsafe { aegis_amd64::decrypt_aesni_4(key, nonce, ad, msg, tag) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes256 => {
            if D == 2 {
                unsafe { aegis_amd64::decrypt_vaes256_2(key, nonce, ad, msg, tag) }
            } else {
                unsafe { aegis_amd64::decrypt_vaes256_4(key, nonce, ad, msg, tag) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes512 => unsafe { aegis_amd64::decrypt_vaes512_4(key, nonce, ad, msg, tag) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::Wasm => unsafe { aegis_wasm32_simd128::decrypt::<D>(key, nonce, ad, msg, tag) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::WasmLane => unsafe { aegis_wasm32_simd128::decrypt_lane::<D>(key, nonce, ad, msg, tag) },
        Backend::Soft => aegis_core::decrypt::<aegis_soft::Soft<D>>(key, nonce, ad, msg, tag),
    }
}

#[inline(always)]
fn init_state_impl<const D: usize>(backend: Backend, key: &[u8; 32], nonce: &[u8; 32], out: &mut [u8]) {
    match backend {
        #[cfg(target_arch = "aarch64")]
        Backend::Arm => unsafe { aegis_arm64::init_state::<D>(key, nonce, out) },
        #[cfg(target_arch = "aarch64")]
        Backend::ArmSha3 => unsafe { aegis_arm64::init_state_sha3::<D>(key, nonce, out) },
        #[cfg(target_arch = "x86_64")]
        Backend::X86AesNi => {
            if D == 1 {
                unsafe { aegis_amd64::init_aesni_1(key, nonce, out) }
            } else if D == 2 {
                unsafe { aegis_amd64::init_aesni_2(key, nonce, out) }
            } else {
                unsafe { aegis_amd64::init_aesni_4(key, nonce, out) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes256 => {
            if D == 2 {
                unsafe { aegis_amd64::init_vaes256_2(key, nonce, out) }
            } else {
                unsafe { aegis_amd64::init_vaes256_4(key, nonce, out) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes512 => unsafe { aegis_amd64::init_vaes512_4(key, nonce, out) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::Wasm => unsafe { aegis_wasm32_simd128::init_state::<D>(key, nonce, out) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::WasmLane => unsafe { aegis_wasm32_simd128::init_state_lane::<D>(key, nonce, out) },
        Backend::Soft => aegis_core::init_state::<aegis_soft::Soft<D>>(key, nonce, out),
    }
}

#[inline(always)]
fn absorb_state_impl<const D: usize>(backend: Backend, state: &mut [u8], ad: &[u8]) {
    match backend {
        #[cfg(target_arch = "aarch64")]
        Backend::Arm => unsafe { aegis_arm64::absorb_state::<D>(state, ad) },
        #[cfg(target_arch = "aarch64")]
        Backend::ArmSha3 => unsafe { aegis_arm64::absorb_state_sha3::<D>(state, ad) },
        #[cfg(target_arch = "x86_64")]
        Backend::X86AesNi => {
            if D == 1 {
                unsafe { aegis_amd64::absorb_aesni_1(state, ad) }
            } else if D == 2 {
                unsafe { aegis_amd64::absorb_aesni_2(state, ad) }
            } else {
                unsafe { aegis_amd64::absorb_aesni_4(state, ad) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes256 => {
            if D == 2 {
                unsafe { aegis_amd64::absorb_vaes256_2(state, ad) }
            } else {
                unsafe { aegis_amd64::absorb_vaes256_4(state, ad) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes512 => unsafe { aegis_amd64::absorb_vaes512_4(state, ad) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::Wasm => unsafe { aegis_wasm32_simd128::absorb_state::<D>(state, ad) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::WasmLane => unsafe { aegis_wasm32_simd128::absorb_state_lane::<D>(state, ad) },
        Backend::Soft => aegis_core::absorb_state::<aegis_soft::Soft<D>>(state, ad),
    }
}

#[inline(always)]
fn finalize_mac_impl<const D: usize>(backend: Backend, state: &[u8], data_len: u64, tag: &mut [u8]) {
    match backend {
        #[cfg(target_arch = "aarch64")]
        Backend::Arm => unsafe { aegis_arm64::finalize_mac_state::<D>(state, data_len, tag) },
        #[cfg(target_arch = "aarch64")]
        Backend::ArmSha3 => unsafe { aegis_arm64::finalize_mac_state_sha3::<D>(state, data_len, tag) },
        #[cfg(target_arch = "x86_64")]
        Backend::X86AesNi => {
            if D == 1 {
                unsafe { aegis_amd64::finalize_aesni_1(state, data_len, tag) }
            } else if D == 2 {
                unsafe { aegis_amd64::finalize_aesni_2(state, data_len, tag) }
            } else {
                unsafe { aegis_amd64::finalize_aesni_4(state, data_len, tag) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes256 => {
            if D == 2 {
                unsafe { aegis_amd64::finalize_vaes256_2(state, data_len, tag) }
            } else {
                unsafe { aegis_amd64::finalize_vaes256_4(state, data_len, tag) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes512 => unsafe { aegis_amd64::finalize_vaes512_4(state, data_len, tag) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::Wasm => unsafe { aegis_wasm32_simd128::finalize_mac_state::<D>(state, data_len, tag) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::WasmLane => unsafe { aegis_wasm32_simd128::finalize_mac_state_lane::<D>(state, data_len, tag) },
        Backend::Soft => aegis_core::finalize_mac_state::<aegis_soft::Soft<D>>(state, data_len, tag),
    }
}

#[inline(always)]
fn next_keystream_impl<const D: usize>(backend: Backend, state: &mut [u8], out: &mut [u8]) {
    match backend {
        #[cfg(target_arch = "aarch64")]
        Backend::Arm => unsafe { aegis_arm64::next_keystream_state::<D>(state, out) },
        #[cfg(target_arch = "aarch64")]
        Backend::ArmSha3 => unsafe { aegis_arm64::next_keystream_state_sha3::<D>(state, out) },
        #[cfg(target_arch = "x86_64")]
        Backend::X86AesNi => {
            if D == 1 {
                unsafe { aegis_amd64::next_aesni_1(state, out) }
            } else if D == 2 {
                unsafe { aegis_amd64::next_aesni_2(state, out) }
            } else {
                unsafe { aegis_amd64::next_aesni_4(state, out) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes256 => {
            if D == 2 {
                unsafe { aegis_amd64::next_vaes256_2(state, out) }
            } else {
                unsafe { aegis_amd64::next_vaes256_4(state, out) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes512 => unsafe { aegis_amd64::next_vaes512_4(state, out) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::Wasm => unsafe { aegis_wasm32_simd128::next_keystream_state::<D>(state, out) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::WasmLane => unsafe { aegis_wasm32_simd128::next_keystream_state_lane::<D>(state, out) },
        Backend::Soft => aegis_core::next_keystream_state::<aegis_soft::Soft<D>>(state, out),
    }
}

#[inline(always)]
fn stream_impl<const D: usize>(backend: Backend, state: &mut [u8], in_out: &mut [u8]) {
    match backend {
        #[cfg(target_arch = "aarch64")]
        Backend::Arm => unsafe { aegis_arm64::stream_state::<D>(state, in_out) },
        #[cfg(target_arch = "aarch64")]
        Backend::ArmSha3 => unsafe { aegis_arm64::stream_state_sha3::<D>(state, in_out) },
        #[cfg(target_arch = "x86_64")]
        Backend::X86AesNi => {
            if D == 1 {
                unsafe { aegis_amd64::stream_aesni_1(state, in_out) }
            } else if D == 2 {
                unsafe { aegis_amd64::stream_aesni_2(state, in_out) }
            } else {
                unsafe { aegis_amd64::stream_aesni_4(state, in_out) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes256 => {
            if D == 2 {
                unsafe { aegis_amd64::stream_vaes256_2(state, in_out) }
            } else {
                unsafe { aegis_amd64::stream_vaes256_4(state, in_out) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes512 => unsafe { aegis_amd64::stream_vaes512_4(state, in_out) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::Wasm => unsafe { aegis_wasm32_simd128::stream_state::<D>(state, in_out) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::WasmLane => unsafe { aegis_wasm32_simd128::stream_state_lane::<D>(state, in_out) },
        Backend::Soft => aegis_core::stream_xor_state::<aegis_soft::Soft<D>>(state, in_out),
    }
}

#[inline(always)]
fn stream_fill_impl<const D: usize>(backend: Backend, state: &mut [u8], out: &mut [u8]) {
    match backend {
        #[cfg(target_arch = "aarch64")]
        Backend::Arm => unsafe { aegis_arm64::fill_state::<D>(state, out) },
        #[cfg(target_arch = "aarch64")]
        Backend::ArmSha3 => unsafe { aegis_arm64::fill_state_sha3::<D>(state, out) },
        #[cfg(target_arch = "x86_64")]
        Backend::X86AesNi => {
            if D == 1 {
                unsafe { aegis_amd64::fill_aesni_1(state, out) }
            } else if D == 2 {
                unsafe { aegis_amd64::fill_aesni_2(state, out) }
            } else {
                unsafe { aegis_amd64::fill_aesni_4(state, out) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes256 => {
            if D == 2 {
                unsafe { aegis_amd64::fill_vaes256_2(state, out) }
            } else {
                unsafe { aegis_amd64::fill_vaes256_4(state, out) }
            }
        }
        #[cfg(target_arch = "x86_64")]
        Backend::X86Vaes512 => unsafe { aegis_amd64::fill_vaes512_4(state, out) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::Wasm => unsafe { aegis_wasm32_simd128::fill_state::<D>(state, out) },
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        Backend::WasmLane => unsafe { aegis_wasm32_simd128::fill_state_lane::<D>(state, out) },
        Backend::Soft => aegis_core::stream_fill_state::<aegis_soft::Soft<D>>(state, out),
    }
}

macro_rules! define_aead {
    ($name:ident, $d:expr) => {
        /// AEGIS authenticated encryption with associated data.
        ///
        /// The tag size is a const generic parameter and must be 16 or 32
        /// bytes. See the [module documentation](self) for parameter details.
        #[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
        pub struct $name<const TAG: usize = 16> {
            key: [u8; 32],
            #[cfg_attr(feature = "zeroize", zeroize(skip))]
            backend: Backend,
        }

        impl<const TAG: usize> $name<TAG> {
            /// Key size in bytes.
            pub const KEY_SIZE: usize = 32;
            /// Nonce size in bytes.
            pub const NONCE_SIZE: usize = 32;
            /// Tag size in bytes.
            pub const TAG_SIZE: usize = TAG;

            /// Creates a new cipher from a 32-byte key.
            ///
            /// # Panics
            ///
            /// Panics at compile time when `TAG` is not 16 or 32.
            pub fn new(key: &[u8; 32]) -> Self {
                const {
                    assert!(TAG == 16 || TAG == 32, "AEGIS tag size must be 16 or 32 bytes");
                }
                Self {
                    key: *key,
                    backend: detect::<$d>(),
                }
            }

            /// Encrypts `msg` in place and returns a detached tag.
            ///
            /// # Panics
            ///
            /// Panics if `ad.len()` or `msg.len()` exceeds `2^61 - 1` bytes
            /// (the RFC 10032 input limit).
            #[inline]
            #[must_use]
            pub fn encrypt_in_place_detached(&self, msg: &mut [u8], nonce: &[u8; 32], ad: &[u8]) -> [u8; TAG] {
                let mut tag = [0u8; TAG];
                encrypt_impl::<$d>(self.backend, &self.key, nonce, ad, msg, &mut tag);
                tag
            }

            /// Authenticates and decrypts `msg` in place with a detached tag.
            ///
            /// On failure `msg` is zeroed and [`AeadError::InvalidCiphertext`]
            /// is returned.
            ///
            /// # Panics
            ///
            /// Panics if `ad.len()` or `msg.len()` exceeds `2^61 - 1` bytes
            /// (the RFC 10032 input limit).
            #[inline]
            pub fn decrypt_in_place_detached(
                &self,
                msg: &mut [u8],
                nonce: &[u8; 32],
                ad: &[u8],
                tag: &[u8; TAG],
            ) -> Result<(), AeadError> {
                if decrypt_impl::<$d>(self.backend, &self.key, nonce, ad, msg, tag) {
                    Ok(())
                } else {
                    Err(AeadError::InvalidCiphertext)
                }
            }
        }

        impl<const TAG: usize> core::fmt::Debug for $name<TAG> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }

        impl<const TAG: usize> Aead for $name<TAG> {
            const KEY_SIZE: usize = 32;
            const TAG_SIZE: usize = TAG;
            const NONCE_SIZE: usize = 32;

            #[inline]
            fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
                let nonce: &[u8; 32] = nonce
                    .try_into()
                    .unwrap_or_else(|_| panic!("AEGIS nonce must be 32 bytes"));
                Hash::from(self.encrypt_in_place_detached(in_out, nonce, aad))
            }

            #[inline]
            fn decrypt_in_place(
                &self,
                in_out: &mut [u8],
                nonce: &[u8],
                aad: &[u8],
                tag: &[u8],
            ) -> Result<(), AeadError> {
                let nonce: &[u8; 32] = nonce.try_into().map_err(|_| AeadError::InvalidNonce)?;
                let tag: &[u8; TAG] = tag.try_into().map_err(|_| AeadError::InvalidCiphertext)?;
                self.decrypt_in_place_detached(in_out, nonce, aad, tag)
            }
        }
    };
}

define_aead!(Aegis256, 1);
define_aead!(Aegis256X2, 2);
define_aead!(Aegis256X4, 4);

macro_rules! define_mac {
    ($name:ident, $d:expr) => {
        /// AEGIS message authentication code (AEGISMAC).
        ///
        /// The state can be cloned to authenticate multiple messages with the
        /// same key and nonce (which is allowed for AEGISMAC). The tag size
        /// must be 16 or 32 bytes.
        ///
        /// Do not use the same key for this MAC and for AEAD encryption.
        #[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
        #[derive(Clone)]
        pub struct $name<const TAG_SIZE: usize = 16> {
            state: [u8; 96 * $d],
            #[cfg_attr(feature = "zeroize", zeroize(skip))]
            backend: Backend,
            buf: [u8; 16 * $d],
            pos: usize,
            len: u64,
        }

        impl<const TAG_SIZE: usize> $name<TAG_SIZE> {
            /// Creates a MAC with an all-zero nonce.
            pub fn new(key: &[u8; 32]) -> Self {
                Self::new_with_nonce(key, &[0u8; 32])
            }

            /// Creates a MAC with an explicit 32-byte nonce.
            ///
            /// The same key may be reused with the same nonce for the MAC
            /// interface (unlike AEAD encryption).
            pub fn new_with_nonce(key: &[u8; 32], nonce: &[u8; 32]) -> Self {
                const {
                    assert!(
                        TAG_SIZE == 16 || TAG_SIZE == 32,
                        "AEGIS tag size must be 16 or 32 bytes"
                    );
                }
                let backend = detect::<$d>();
                let mut state = [0u8; 96 * $d];
                init_state_impl::<$d>(backend, key, nonce, &mut state);
                Self {
                    state,
                    backend,
                    buf: [0u8; 16 * $d],
                    pos: 0,
                    len: 0,
                }
            }

            /// Feeds data into the MAC.
            ///
            /// For best throughput, feed large chunks: each call round-trips
            /// the internal state.
            ///
            /// # Panics
            ///
            /// Panics if the total authenticated length would exceed
            /// `2^61 - 1` bytes (the RFC 10032 input limit).
            pub fn update(&mut self, data: &[u8]) {
                let rate = 16 * $d;
                let new_len = self
                    .len
                    .checked_add(data.len() as u64)
                    .expect("AEGIS MAC length overflow");
                assert!(new_len <= MAX_INPUT_LEN, "AEGIS MAC input is too long");
                self.len = new_len;

                let mut offset = 0;
                if self.pos != 0 {
                    let needed = rate - self.pos;
                    let take = needed.min(data.len());
                    self.buf[self.pos..self.pos + take].copy_from_slice(&data[..take]);
                    self.pos += take;
                    offset = take;
                    if self.pos < rate {
                        return;
                    }
                    absorb_state_impl::<$d>(self.backend, &mut self.state, &self.buf[..rate]);
                    self.pos = 0;
                }

                let full = (data.len() - offset) / rate * rate;
                if full != 0 {
                    absorb_state_impl::<$d>(self.backend, &mut self.state, &data[offset..offset + full]);
                    offset += full;
                }

                let remaining = data.len() - offset;
                if remaining != 0 {
                    self.buf[..remaining].copy_from_slice(&data[offset..]);
                    self.pos = remaining;
                }
            }

            /// Finalizes the MAC and returns the tag.
            #[must_use]
            pub fn finalize(mut self) -> [u8; TAG_SIZE] {
                let rate = 16 * $d;
                if self.pos != 0 {
                    self.buf[self.pos..rate].fill(0);
                    absorb_state_impl::<$d>(self.backend, &mut self.state, &self.buf[..rate]);
                }
                let mut tag = [0u8; TAG_SIZE];
                finalize_mac_impl::<$d>(self.backend, &self.state, self.len, &mut tag);
                tag
            }

            /// Verifies a tag in constant time.
            ///
            /// Returns [`AeadError::InvalidCiphertext`] when the tag does not
            /// match.
            pub fn verify(self, tag: &[u8; TAG_SIZE]) -> Result<(), AeadError> {
                let expected = self.finalize();
                if constant_time_eq(tag, &expected) {
                    Ok(())
                } else {
                    Err(AeadError::InvalidCiphertext)
                }
            }
        }

        impl<const TAG_SIZE: usize> core::fmt::Debug for $name<TAG_SIZE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
    };
}

define_mac!(Aegis256Mac, 1);
define_mac!(Aegis256X2Mac, 2);
define_mac!(Aegis256X4Mac, 4);

macro_rules! define_stream {
    ($name:ident, $d:expr) => {
        /// AEGIS keystream generator (RFC 10032 §7).
        ///
        /// The chunked interface produces one continuous keystream, so it may
        /// be called with arbitrary sizes. [`Self::fill_keystream`] overwrites
        /// the output buffer; [`Self::xor_keystream_in_place`] XORs it. For
        /// best throughput, use large chunks. The same nonce must not be reused
        /// with the same key, and this mode must not share a key with AEAD or
        /// AEGISMAC.
        #[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
        #[derive(Clone)]
        pub struct $name {
            state: [u8; 96 * $d],
            #[cfg_attr(feature = "zeroize", zeroize(skip))]
            backend: Backend,
            buf: [u8; 16 * $d],
            pos: usize,
            buf_len: usize,
        }

        impl $name {
            /// Creates a new keystream from a 32-byte key and nonce.
            pub fn new(key: &[u8; 32], nonce: &[u8; 32]) -> Self {
                let backend = detect::<$d>();
                let mut state = [0u8; 96 * $d];
                init_state_impl::<$d>(backend, key, nonce, &mut state);
                Self {
                    state,
                    backend,
                    buf: [0u8; 16 * $d],
                    pos: 0,
                    buf_len: 0,
                }
            }

            /// Fills `out` with keystream bytes, overwriting its previous
            /// contents.
            pub fn fill_keystream(&mut self, out: &mut [u8]) {
                let rate = 16 * $d;
                let mut written = 0;

                while written < out.len() {
                    if self.pos < self.buf_len {
                        let take = (self.buf_len - self.pos).min(out.len() - written);
                        out[written..written + take].copy_from_slice(&self.buf[self.pos..self.pos + take]);
                        self.pos += take;
                        written += take;
                        continue;
                    }

                    let remaining = out.len() - written;
                    if remaining >= rate {
                        let full = remaining / rate * rate;
                        stream_fill_impl::<$d>(self.backend, &mut self.state, &mut out[written..written + full]);
                        written += full;
                    } else {
                        self.buf = [0u8; 16 * $d];
                        next_keystream_impl::<$d>(self.backend, &mut self.state, &mut self.buf[..rate]);
                        self.buf_len = rate;
                        self.pos = 0;
                    }
                }
            }

            /// XORs the keystream into `in_out`, encrypting or decrypting.
            pub fn xor_keystream_in_place(&mut self, in_out: &mut [u8]) {
                let rate = 16 * $d;
                let mut done = 0;

                if self.pos < self.buf_len {
                    let take = (self.buf_len - self.pos).min(in_out.len());
                    for i in 0..take {
                        in_out[i] ^= self.buf[self.pos + i];
                    }
                    self.pos += take;
                    done = take;
                }

                let remaining = in_out.len() - done;
                let full = remaining / rate * rate;
                if full != 0 {
                    stream_impl::<$d>(self.backend, &mut self.state, &mut in_out[done..done + full]);
                    done += full;
                }

                if done < in_out.len() {
                    self.buf = [0u8; 16 * $d];
                    next_keystream_impl::<$d>(self.backend, &mut self.state, &mut self.buf[..rate]);
                    self.buf_len = rate;
                    self.pos = in_out.len() - done;
                    for i in 0..self.pos {
                        in_out[done + i] ^= self.buf[i];
                    }
                }
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }

        impl StreamCipher for $name {
            fn xor_keystream(&mut self, in_out: &mut [u8]) {
                self.xor_keystream_in_place(in_out)
            }
        }
    };
}

define_stream!(Aegis256Stream, 1);
define_stream!(Aegis256X2Stream, 2);
define_stream!(Aegis256X4Stream, 4);

#[cfg(test)]
mod tests;
