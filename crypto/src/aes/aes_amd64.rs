#![allow(unsafe_op_in_unsafe_fn)]

//! x86-64 AES kernels.
//!
//! The AES-NI path is always used when available. On top of it, VAES provides
//! vector AES rounds over 256-bit AVX2 registers (two blocks per register) and
//! 512-bit AVX-512 registers (four blocks per register). The VAES kernels
//! broadcast one 128-bit round key at a time instead of storing a second,
//! larger key schedule.
use core::arch::x86_64::*;

/// Which x86-64 SIMD backend a cipher object was created with.
///
/// This is selected once in the constructor. It is intentionally separate from
/// [`RoundKeys`](super::aes::RoundKeys): the round keys stay in their compact
/// 128-bit form and the VAES kernels broadcast one round key at a time. Storing
/// pre-broadcast 256/512-bit schedules would make every cipher object at least
/// 960 bytes, even when the software or AES-NI backend is selected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum X86Backend {
    /// AES-NI (128-bit) instructions.
    AesNi,
    /// VAES with 256-bit AVX2 registers.
    Vaes256,
    /// VAES with 512-bit AVX-512 registers.
    Vaes512,
}

impl X86Backend {
    /// Detect the best available backend.
    ///
    /// VAES-512 requires AVX-512F and AVX-512BW in addition to VAES. BW is
    /// needed for the lane-local byte shuffles used to swap GCM counters and
    /// reverse GHASH bits. AVX2 is required by the 256-bit fallback and the
    /// GCM/GHASH helpers.
    #[cfg(feature = "std")]
    pub(crate) fn detect() -> Self {
        if std::arch::is_x86_feature_detected!("vaes")
            && std::arch::is_x86_feature_detected!("avx512f")
            && std::arch::is_x86_feature_detected!("avx512bw")
            && std::arch::is_x86_feature_detected!("avx2")
        {
            Self::Vaes512
        } else if std::arch::is_x86_feature_detected!("vaes") && std::arch::is_x86_feature_detected!("avx2") {
            Self::Vaes256
        } else {
            Self::AesNi
        }
    }

    /// Compile-time backend selection for `no_std` builds.
    #[cfg(not(feature = "std"))]
    pub(crate) const fn detect() -> Self {
        if cfg!(all(
            target_feature = "vaes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx2"
        )) {
            Self::Vaes512
        } else if cfg!(all(target_feature = "vaes", target_feature = "avx2")) {
            Self::Vaes256
        } else {
            Self::AesNi
        }
    }
}

#[inline]
pub(crate) fn expand_key_x86_64<const N: usize>(round_keys_software: [[u8; 16]; N]) -> [__m128i; N] {
    const { assert!(N == 11 || N == 15) };

    let mut round_keys = unsafe { [_mm_setzero_si128(); N] };
    for i in 0..N {
        round_keys[i] = unsafe { _mm_loadu_si128(round_keys_software[i].as_ptr().cast()) };
    }
    round_keys
}

/// Const generic AES encrypt block for AES-128 (N=11) and AES-256 (N=15).
#[target_feature(enable = "aes,sse2")]
#[inline]
pub(crate) unsafe fn aes_encrypt_block<const N: usize>(rk: &[__m128i; N], block: __m128i) -> __m128i {
    const { assert!(N == 11 || N == 15) };

    let mut b = _mm_xor_si128(block, rk[0]);
    b = _mm_aesenc_si128(b, rk[1]);
    b = _mm_aesenc_si128(b, rk[2]);
    b = _mm_aesenc_si128(b, rk[3]);
    b = _mm_aesenc_si128(b, rk[4]);
    b = _mm_aesenc_si128(b, rk[5]);
    b = _mm_aesenc_si128(b, rk[6]);
    b = _mm_aesenc_si128(b, rk[7]);
    b = _mm_aesenc_si128(b, rk[8]);
    b = _mm_aesenc_si128(b, rk[9]);

    if N == 11 {
        _mm_aesenclast_si128(b, rk[10])
    } else {
        let p = rk.as_ptr();
        b = _mm_aesenc_si128(b, *p.add(10));
        b = _mm_aesenc_si128(b, *p.add(11));
        b = _mm_aesenc_si128(b, *p.add(12));
        b = _mm_aesenc_si128(b, *p.add(13));
        _mm_aesenclast_si128(b, *p.add(14))
    }
}

/// Interleaved AES-encrypt 8 blocks — each round is applied across all 8 blocks.
#[target_feature(enable = "aes,sse2")]
#[inline]
pub(crate) unsafe fn aes_encrypt_8blocks<const N: usize>(
    rk: &[__m128i; N],
    b1: __m128i,
    b2: __m128i,
    b3: __m128i,
    b4: __m128i,
    b5: __m128i,
    b6: __m128i,
    b7: __m128i,
    b8: __m128i,
) -> (__m128i, __m128i, __m128i, __m128i, __m128i, __m128i, __m128i, __m128i) {
    const { assert!(N == 11 || N == 15) };

    let b1 = _mm_xor_si128(b1, rk[0]);
    let b2 = _mm_xor_si128(b2, rk[0]);
    let b3 = _mm_xor_si128(b3, rk[0]);
    let b4 = _mm_xor_si128(b4, rk[0]);
    let b5 = _mm_xor_si128(b5, rk[0]);
    let b6 = _mm_xor_si128(b6, rk[0]);
    let b7 = _mm_xor_si128(b7, rk[0]);
    let b8 = _mm_xor_si128(b8, rk[0]);

    let b1 = _mm_aesenc_si128(b1, rk[1]);
    let b2 = _mm_aesenc_si128(b2, rk[1]);
    let b3 = _mm_aesenc_si128(b3, rk[1]);
    let b4 = _mm_aesenc_si128(b4, rk[1]);
    let b5 = _mm_aesenc_si128(b5, rk[1]);
    let b6 = _mm_aesenc_si128(b6, rk[1]);
    let b7 = _mm_aesenc_si128(b7, rk[1]);
    let b8 = _mm_aesenc_si128(b8, rk[1]);

    let b1 = _mm_aesenc_si128(b1, rk[2]);
    let b2 = _mm_aesenc_si128(b2, rk[2]);
    let b3 = _mm_aesenc_si128(b3, rk[2]);
    let b4 = _mm_aesenc_si128(b4, rk[2]);
    let b5 = _mm_aesenc_si128(b5, rk[2]);
    let b6 = _mm_aesenc_si128(b6, rk[2]);
    let b7 = _mm_aesenc_si128(b7, rk[2]);
    let b8 = _mm_aesenc_si128(b8, rk[2]);

    let b1 = _mm_aesenc_si128(b1, rk[3]);
    let b2 = _mm_aesenc_si128(b2, rk[3]);
    let b3 = _mm_aesenc_si128(b3, rk[3]);
    let b4 = _mm_aesenc_si128(b4, rk[3]);
    let b5 = _mm_aesenc_si128(b5, rk[3]);
    let b6 = _mm_aesenc_si128(b6, rk[3]);
    let b7 = _mm_aesenc_si128(b7, rk[3]);
    let b8 = _mm_aesenc_si128(b8, rk[3]);

    let b1 = _mm_aesenc_si128(b1, rk[4]);
    let b2 = _mm_aesenc_si128(b2, rk[4]);
    let b3 = _mm_aesenc_si128(b3, rk[4]);
    let b4 = _mm_aesenc_si128(b4, rk[4]);
    let b5 = _mm_aesenc_si128(b5, rk[4]);
    let b6 = _mm_aesenc_si128(b6, rk[4]);
    let b7 = _mm_aesenc_si128(b7, rk[4]);
    let b8 = _mm_aesenc_si128(b8, rk[4]);

    let b1 = _mm_aesenc_si128(b1, rk[5]);
    let b2 = _mm_aesenc_si128(b2, rk[5]);
    let b3 = _mm_aesenc_si128(b3, rk[5]);
    let b4 = _mm_aesenc_si128(b4, rk[5]);
    let b5 = _mm_aesenc_si128(b5, rk[5]);
    let b6 = _mm_aesenc_si128(b6, rk[5]);
    let b7 = _mm_aesenc_si128(b7, rk[5]);
    let b8 = _mm_aesenc_si128(b8, rk[5]);

    let b1 = _mm_aesenc_si128(b1, rk[6]);
    let b2 = _mm_aesenc_si128(b2, rk[6]);
    let b3 = _mm_aesenc_si128(b3, rk[6]);
    let b4 = _mm_aesenc_si128(b4, rk[6]);
    let b5 = _mm_aesenc_si128(b5, rk[6]);
    let b6 = _mm_aesenc_si128(b6, rk[6]);
    let b7 = _mm_aesenc_si128(b7, rk[6]);
    let b8 = _mm_aesenc_si128(b8, rk[6]);

    let b1 = _mm_aesenc_si128(b1, rk[7]);
    let b2 = _mm_aesenc_si128(b2, rk[7]);
    let b3 = _mm_aesenc_si128(b3, rk[7]);
    let b4 = _mm_aesenc_si128(b4, rk[7]);
    let b5 = _mm_aesenc_si128(b5, rk[7]);
    let b6 = _mm_aesenc_si128(b6, rk[7]);
    let b7 = _mm_aesenc_si128(b7, rk[7]);
    let b8 = _mm_aesenc_si128(b8, rk[7]);

    let b1 = _mm_aesenc_si128(b1, rk[8]);
    let b2 = _mm_aesenc_si128(b2, rk[8]);
    let b3 = _mm_aesenc_si128(b3, rk[8]);
    let b4 = _mm_aesenc_si128(b4, rk[8]);
    let b5 = _mm_aesenc_si128(b5, rk[8]);
    let b6 = _mm_aesenc_si128(b6, rk[8]);
    let b7 = _mm_aesenc_si128(b7, rk[8]);
    let b8 = _mm_aesenc_si128(b8, rk[8]);

    let b1 = _mm_aesenc_si128(b1, rk[9]);
    let b2 = _mm_aesenc_si128(b2, rk[9]);
    let b3 = _mm_aesenc_si128(b3, rk[9]);
    let b4 = _mm_aesenc_si128(b4, rk[9]);
    let b5 = _mm_aesenc_si128(b5, rk[9]);
    let b6 = _mm_aesenc_si128(b6, rk[9]);
    let b7 = _mm_aesenc_si128(b7, rk[9]);
    let b8 = _mm_aesenc_si128(b8, rk[9]);

    if N == 11 {
        (
            _mm_aesenclast_si128(b1, rk[10]),
            _mm_aesenclast_si128(b2, rk[10]),
            _mm_aesenclast_si128(b3, rk[10]),
            _mm_aesenclast_si128(b4, rk[10]),
            _mm_aesenclast_si128(b5, rk[10]),
            _mm_aesenclast_si128(b6, rk[10]),
            _mm_aesenclast_si128(b7, rk[10]),
            _mm_aesenclast_si128(b8, rk[10]),
        )
    } else {
        let b1 = _mm_aesenc_si128(b1, rk[10]);
        let b2 = _mm_aesenc_si128(b2, rk[10]);
        let b3 = _mm_aesenc_si128(b3, rk[10]);
        let b4 = _mm_aesenc_si128(b4, rk[10]);
        let b5 = _mm_aesenc_si128(b5, rk[10]);
        let b6 = _mm_aesenc_si128(b6, rk[10]);
        let b7 = _mm_aesenc_si128(b7, rk[10]);
        let b8 = _mm_aesenc_si128(b8, rk[10]);

        let b1 = _mm_aesenc_si128(b1, rk[11]);
        let b2 = _mm_aesenc_si128(b2, rk[11]);
        let b3 = _mm_aesenc_si128(b3, rk[11]);
        let b4 = _mm_aesenc_si128(b4, rk[11]);
        let b5 = _mm_aesenc_si128(b5, rk[11]);
        let b6 = _mm_aesenc_si128(b6, rk[11]);
        let b7 = _mm_aesenc_si128(b7, rk[11]);
        let b8 = _mm_aesenc_si128(b8, rk[11]);

        let b1 = _mm_aesenc_si128(b1, rk[12]);
        let b2 = _mm_aesenc_si128(b2, rk[12]);
        let b3 = _mm_aesenc_si128(b3, rk[12]);
        let b4 = _mm_aesenc_si128(b4, rk[12]);
        let b5 = _mm_aesenc_si128(b5, rk[12]);
        let b6 = _mm_aesenc_si128(b6, rk[12]);
        let b7 = _mm_aesenc_si128(b7, rk[12]);
        let b8 = _mm_aesenc_si128(b8, rk[12]);

        let b1 = _mm_aesenc_si128(b1, rk[13]);
        let b2 = _mm_aesenc_si128(b2, rk[13]);
        let b3 = _mm_aesenc_si128(b3, rk[13]);
        let b4 = _mm_aesenc_si128(b4, rk[13]);
        let b5 = _mm_aesenc_si128(b5, rk[13]);
        let b6 = _mm_aesenc_si128(b6, rk[13]);
        let b7 = _mm_aesenc_si128(b7, rk[13]);
        let b8 = _mm_aesenc_si128(b8, rk[13]);

        (
            _mm_aesenclast_si128(b1, rk[14]),
            _mm_aesenclast_si128(b2, rk[14]),
            _mm_aesenclast_si128(b3, rk[14]),
            _mm_aesenclast_si128(b4, rk[14]),
            _mm_aesenclast_si128(b5, rk[14]),
            _mm_aesenclast_si128(b6, rk[14]),
            _mm_aesenclast_si128(b7, rk[14]),
            _mm_aesenclast_si128(b8, rk[14]),
        )
    }
}

/// Interleaved AES-encrypt 4 blocks.
#[target_feature(enable = "aes,sse2")]
#[inline]
pub(crate) unsafe fn aes_encrypt_4blocks<const N: usize>(
    rk: &[__m128i; N],
    b1: __m128i,
    b2: __m128i,
    b3: __m128i,
    b4: __m128i,
) -> (__m128i, __m128i, __m128i, __m128i) {
    const { assert!(N == 11 || N == 15) };

    let b1 = _mm_xor_si128(b1, rk[0]);
    let b2 = _mm_xor_si128(b2, rk[0]);
    let b3 = _mm_xor_si128(b3, rk[0]);
    let b4 = _mm_xor_si128(b4, rk[0]);

    let b1 = _mm_aesenc_si128(b1, rk[1]);
    let b2 = _mm_aesenc_si128(b2, rk[1]);
    let b3 = _mm_aesenc_si128(b3, rk[1]);
    let b4 = _mm_aesenc_si128(b4, rk[1]);

    let b1 = _mm_aesenc_si128(b1, rk[2]);
    let b2 = _mm_aesenc_si128(b2, rk[2]);
    let b3 = _mm_aesenc_si128(b3, rk[2]);
    let b4 = _mm_aesenc_si128(b4, rk[2]);

    let b1 = _mm_aesenc_si128(b1, rk[3]);
    let b2 = _mm_aesenc_si128(b2, rk[3]);
    let b3 = _mm_aesenc_si128(b3, rk[3]);
    let b4 = _mm_aesenc_si128(b4, rk[3]);

    let b1 = _mm_aesenc_si128(b1, rk[4]);
    let b2 = _mm_aesenc_si128(b2, rk[4]);
    let b3 = _mm_aesenc_si128(b3, rk[4]);
    let b4 = _mm_aesenc_si128(b4, rk[4]);

    let b1 = _mm_aesenc_si128(b1, rk[5]);
    let b2 = _mm_aesenc_si128(b2, rk[5]);
    let b3 = _mm_aesenc_si128(b3, rk[5]);
    let b4 = _mm_aesenc_si128(b4, rk[5]);

    let b1 = _mm_aesenc_si128(b1, rk[6]);
    let b2 = _mm_aesenc_si128(b2, rk[6]);
    let b3 = _mm_aesenc_si128(b3, rk[6]);
    let b4 = _mm_aesenc_si128(b4, rk[6]);

    let b1 = _mm_aesenc_si128(b1, rk[7]);
    let b2 = _mm_aesenc_si128(b2, rk[7]);
    let b3 = _mm_aesenc_si128(b3, rk[7]);
    let b4 = _mm_aesenc_si128(b4, rk[7]);

    let b1 = _mm_aesenc_si128(b1, rk[8]);
    let b2 = _mm_aesenc_si128(b2, rk[8]);
    let b3 = _mm_aesenc_si128(b3, rk[8]);
    let b4 = _mm_aesenc_si128(b4, rk[8]);

    let b1 = _mm_aesenc_si128(b1, rk[9]);
    let b2 = _mm_aesenc_si128(b2, rk[9]);
    let b3 = _mm_aesenc_si128(b3, rk[9]);
    let b4 = _mm_aesenc_si128(b4, rk[9]);

    if N == 11 {
        (
            _mm_aesenclast_si128(b1, rk[10]),
            _mm_aesenclast_si128(b2, rk[10]),
            _mm_aesenclast_si128(b3, rk[10]),
            _mm_aesenclast_si128(b4, rk[10]),
        )
    } else {
        let b1 = _mm_aesenc_si128(b1, rk[10]);
        let b2 = _mm_aesenc_si128(b2, rk[10]);
        let b3 = _mm_aesenc_si128(b3, rk[10]);
        let b4 = _mm_aesenc_si128(b4, rk[10]);

        let b1 = _mm_aesenc_si128(b1, rk[11]);
        let b2 = _mm_aesenc_si128(b2, rk[11]);
        let b3 = _mm_aesenc_si128(b3, rk[11]);
        let b4 = _mm_aesenc_si128(b4, rk[11]);

        let b1 = _mm_aesenc_si128(b1, rk[12]);
        let b2 = _mm_aesenc_si128(b2, rk[12]);
        let b3 = _mm_aesenc_si128(b3, rk[12]);
        let b4 = _mm_aesenc_si128(b4, rk[12]);

        let b1 = _mm_aesenc_si128(b1, rk[13]);
        let b2 = _mm_aesenc_si128(b2, rk[13]);
        let b3 = _mm_aesenc_si128(b3, rk[13]);
        let b4 = _mm_aesenc_si128(b4, rk[13]);

        (
            _mm_aesenclast_si128(b1, rk[14]),
            _mm_aesenclast_si128(b2, rk[14]),
            _mm_aesenclast_si128(b3, rk[14]),
            _mm_aesenclast_si128(b4, rk[14]),
        )
    }
}

// ── VAES (vector AES) kernels ─────────────────────────────────────────────────

/// Encrypt `M` vectors of blocks using VAES with 256-bit registers.
///
/// Each `__m256i` holds two independent 128-bit AES blocks. `M` is 2 (four
/// blocks) or 4 (eight blocks). Round keys are broadcast from the compact
/// 128-bit schedule one round at a time, which keeps the key schedule small and
/// avoids a 480-byte per-object key array.
#[target_feature(enable = "vaes,avx2")]
#[inline]
pub(crate) unsafe fn aes_encrypt_vaes256<const N: usize, const M: usize>(
    rk: &[__m128i; N],
    mut blocks: [__m256i; M],
) -> [__m256i; M] {
    const {
        assert!(N == 11 || N == 15);
        assert!(M == 2 || M == 4);
    }

    // `black_box` prevents LLVM from hoisting every broadcast round key out of
    // the caller's block loop. Keeping only one key live at a time avoids
    // spilling the whole 256-bit key schedule to the stack.
    let k = _mm256_broadcastsi128_si256(core::hint::black_box(rk[0]));
    for b in &mut blocks {
        *b = _mm256_xor_si256(*b, k);
    }

    for round in 1..N - 1 {
        let k = _mm256_broadcastsi128_si256(core::hint::black_box(rk[round]));
        for b in &mut blocks {
            *b = _mm256_aesenc_epi128(*b, k);
        }
    }

    let k = _mm256_broadcastsi128_si256(core::hint::black_box(rk[N - 1]));
    for b in &mut blocks {
        *b = _mm256_aesenclast_epi128(*b, k);
    }

    blocks
}

/// Encrypt `M` vectors of blocks using VAES with 512-bit registers.
///
/// Each `__m512i` holds four independent 128-bit AES blocks. `M` is 2 (eight
/// blocks) or 4 (sixteen blocks).
#[target_feature(enable = "vaes,avx512f,avx512bw,avx2")]
#[inline]
pub(crate) unsafe fn aes_encrypt_vaes512<const N: usize, const M: usize>(
    rk: &[__m128i; N],
    mut blocks: [__m512i; M],
) -> [__m512i; M] {
    const {
        assert!(N == 11 || N == 15);
        assert!(M == 2 || M == 4);
    }

    // See the 256-bit kernel: keep one broadcast key live at a time rather
    // than letting LLVM hoist and spill all 15 keys.
    let k = _mm512_broadcast_i32x4(core::hint::black_box(rk[0]));
    for b in &mut blocks {
        *b = _mm512_xor_si512(*b, k);
    }

    for round in 1..N - 1 {
        let k = _mm512_broadcast_i32x4(core::hint::black_box(rk[round]));
        for b in &mut blocks {
            *b = _mm512_aesenc_epi128(*b, k);
        }
    }

    let k = _mm512_broadcast_i32x4(core::hint::black_box(rk[N - 1]));
    for b in &mut blocks {
        *b = _mm512_aesenclast_epi128(*b, k);
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn have_features() -> bool {
        std::arch::is_x86_feature_detected!("aes")
            && std::arch::is_x86_feature_detected!("pclmulqdq")
            && std::arch::is_x86_feature_detected!("ssse3")
            && std::arch::is_x86_feature_detected!("sse4.1")
    }

    macro_rules! skip_unless_aesni {
        () => {
            if !have_features() {
                eprintln!("Skipping AES-NI test: CPU features not available");
                return;
            }
        };
    }

    fn make_rk(key: &[u8; 32]) -> [__m128i; 15] {
        let soft = crate::aes::aes::expand_key::<15>(key);
        let mut rk = [unsafe { _mm_setzero_si128() }; 15];
        for i in 0..15 {
            rk[i] = unsafe { _mm_loadu_si128(soft[i].as_ptr().cast()) };
        }
        rk
    }

    #[test]
    fn aesni_ecb_vectors() {
        skip_unless_aesni!();

        let key: [u8; 32] =
            hex::decode_array::<32>(b"603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4").unwrap();
        let rk = make_rk(&key);

        let vectors: &[([u8; 16], [u8; 16])] = &[
            (
                hex::decode_array::<16>(b"6bc1bee22e409f96e93d7e117393172a").unwrap(),
                hex::decode_array::<16>(b"f3eed1bdb5d2a03c064b5a7e3db181f8").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"ae2d8a571e03ac9c9eb76fac45af8e51").unwrap(),
                hex::decode_array::<16>(b"591ccb10d410ed26dc5ba74a31362870").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"30c81c46a35ce411e5fbc1191a0a52ef").unwrap(),
                hex::decode_array::<16>(b"b6ed21b99ca6f4f9f153e7b1beafed1d").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"f69f2445df4f9b17ad2b417be66c3710").unwrap(),
                hex::decode_array::<16>(b"23304b7a39f9f3ff067d8d8f9e24ecc7").unwrap(),
            ),
        ];

        for (pt, ct_exp) in vectors {
            let pt_xmm = unsafe { _mm_loadu_si128(pt.as_ptr().cast()) };
            let ct_xmm = unsafe { aes_encrypt_block(&rk, pt_xmm) };
            let mut ni_ct = [0u8; 16];
            unsafe { _mm_storeu_si128(ni_ct.as_mut_ptr().cast(), ct_xmm) };
            assert_eq!(ni_ct, *ct_exp);
        }
    }

    fn have_vaes256() -> bool {
        std::arch::is_x86_feature_detected!("vaes") && std::arch::is_x86_feature_detected!("avx2")
    }

    fn have_vaes512() -> bool {
        have_vaes256()
            && std::arch::is_x86_feature_detected!("avx512f")
            && std::arch::is_x86_feature_detected!("avx512bw")
    }

    fn rk_from_soft<const N: usize>(soft: [[u8; 16]; N]) -> [__m128i; N] {
        let mut rk = [unsafe { _mm_setzero_si128() }; N];
        for i in 0..N {
            rk[i] = unsafe { _mm_loadu_si128(soft[i].as_ptr().cast()) };
        }
        rk
    }

    fn test_blocks<const N: usize>() -> [[u8; 16]; N] {
        core::array::from_fn(|i| {
            let mut b = [0u8; 16];
            for (j, x) in b.iter_mut().enumerate() {
                *x = (i as u8).wrapping_mul(41).wrapping_add(j as u8).wrapping_add(7);
            }
            b
        })
    }

    #[test]
    fn vaes256_encrypt_matches_software() {
        if !have_vaes256() {
            eprintln!("skipping vaes256_encrypt_matches_software: VAES/AVX2 not available");
            return;
        }

        let key = [0x5au8; 32];
        let soft = crate::aes::aes::expand_key::<15>(&key);
        let rk = rk_from_soft(soft);
        let blocks = test_blocks::<8>();

        let b = [
            unsafe { _mm256_loadu_si256(blocks[0].as_ptr().cast()) },
            unsafe { _mm256_loadu_si256(blocks[2].as_ptr().cast()) },
            unsafe { _mm256_loadu_si256(blocks[4].as_ptr().cast()) },
            unsafe { _mm256_loadu_si256(blocks[6].as_ptr().cast()) },
        ];
        let out = unsafe { aes_encrypt_vaes256::<15, 4>(&rk, b) };
        for (i, y) in out.iter().enumerate() {
            let mut buf = [0u8; 32];
            unsafe { _mm256_storeu_si256(buf.as_mut_ptr().cast(), *y) };
            for lane in 0..2 {
                let blk = i * 2 + lane;
                let expected = crate::aes::aes::encrypt_block(&soft, &blocks[blk]);
                assert_eq!(&buf[lane * 16..lane * 16 + 16], &expected, "block {blk}");
            }
        }
    }

    #[test]
    fn vaes512_encrypt_matches_software() {
        if !have_vaes512() {
            eprintln!("skipping vaes512_encrypt_matches_software: VAES/AVX-512 not available");
            return;
        }

        let key = [0xa5u8; 32];
        let soft = crate::aes::aes::expand_key::<15>(&key);
        let rk = rk_from_soft(soft);
        let blocks = test_blocks::<16>();

        let b = [
            unsafe { _mm512_loadu_si512(blocks[0].as_ptr().cast()) },
            unsafe { _mm512_loadu_si512(blocks[4].as_ptr().cast()) },
            unsafe { _mm512_loadu_si512(blocks[8].as_ptr().cast()) },
            unsafe { _mm512_loadu_si512(blocks[12].as_ptr().cast()) },
        ];
        let out = unsafe { aes_encrypt_vaes512::<15, 4>(&rk, b) };
        for (i, z) in out.iter().enumerate() {
            let mut buf = [0u8; 64];
            unsafe { _mm512_storeu_si512(buf.as_mut_ptr().cast(), *z) };
            for lane in 0..4 {
                let blk = i * 4 + lane;
                let expected = crate::aes::aes::encrypt_block(&soft, &blocks[blk]);
                assert_eq!(&buf[lane * 16..lane * 16 + 16], &expected, "block {blk}");
            }
        }
    }
}
