#![allow(unsafe_op_in_unsafe_fn)]
/// x86-64 GHASH using PCLMULQDQ / VPCLMULQDQ + SSSE3/AVX2/AVX-512 intrinsics.
///
/// The 128-bit single-block helpers are used for AAD and short tails. The
/// 256-bit (8-block) and 512-bit (16-block) functions aggregate several
/// products before a single reduction.
///
/// GCM uses a big-endian polynomial ring; PCLMULQDQ uses little-endian.
/// To bridge this, every GCM block is transformed by reversing bits in each byte
/// with `_mm_shuffle_epi8` nibble lookups before carry-less multiplication.
use core::arch::x86_64::*;

/// Reverses bits in each byte of a 128-bit SSE value using SSSE3 `pshufb`.
#[target_feature(enable = "ssse3")]
#[inline]
pub(crate) unsafe fn bitreverse_per_byte(x: __m128i) -> __m128i {
    let reverse4 = _mm_setr_epi8(0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15);
    let low_nibble_mask = _mm_set1_epi8(0x0f_u8 as i8);

    let lo = _mm_and_si128(x, low_nibble_mask);
    let hi = _mm_and_si128(_mm_srli_epi16(x, 4), low_nibble_mask);

    let rev_lo = _mm_shuffle_epi8(reverse4, lo);
    let rev_hi = _mm_shuffle_epi8(reverse4, hi);
    _mm_or_si128(_mm_slli_epi16(rev_lo, 4), rev_hi)
}

/// Reverses bits in each byte of both 128-bit lanes using AVX2 `vpshufb`.
#[target_feature(enable = "ssse3,avx2")]
#[inline]
pub(crate) unsafe fn bitreverse_per_byte_256(x: __m256i) -> __m256i {
    let reverse4 = _mm256_setr_epi8(
        0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15, 0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15,
    );
    let low_nibble_mask = _mm256_set1_epi8(0x0f_u8 as i8);

    let lo = _mm256_and_si256(x, low_nibble_mask);
    let hi = _mm256_and_si256(_mm256_srli_epi16(x, 4), low_nibble_mask);

    let rev_lo = _mm256_shuffle_epi8(reverse4, lo);
    let rev_hi = _mm256_shuffle_epi8(reverse4, hi);
    _mm256_or_si256(_mm256_slli_epi16(rev_lo, 4), rev_hi)
}

/// Reverses bits in each byte of all four 128-bit lanes using AVX-512BW `vpshufb`.
#[target_feature(enable = "ssse3,avx512f,avx512bw,avx2")]
#[inline]
pub(crate) unsafe fn bitreverse_per_byte_512(x: __m512i) -> __m512i {
    let reverse4_128 = _mm_setr_epi8(0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15);
    let reverse4 = _mm512_broadcast_i32x4(reverse4_128);
    let low_nibble_mask = _mm512_set1_epi8(0x0f_u8 as i8);

    let lo = _mm512_and_si512(x, low_nibble_mask);
    let hi = _mm512_and_si512(_mm512_srli_epi16(x, 4), low_nibble_mask);

    let rev_lo = _mm512_shuffle_epi8(reverse4, lo);
    let rev_hi = _mm512_shuffle_epi8(reverse4, hi);
    _mm512_or_si512(_mm512_slli_epi16(rev_lo, 4), rev_hi)
}

/// Reduce a 256-bit carry-less product `(lo, hi)` modulo P = x^128+x^7+x^2+x+1.
#[target_feature(enable = "pclmulqdq,sse2")]
#[inline]
pub(crate) unsafe fn gcm_reduce(lo: __m128i, hi: __m128i) -> __m128i {
    let poly = _mm_set_epi64x(0, 0x87_i64);
    let t1 = _mm_clmulepi64_si128(hi, poly, 0x00);
    let t2 = _mm_clmulepi64_si128(_mm_shuffle_epi32(hi, 0x4e), poly, 0x00);
    let t2_lo = _mm_slli_si128(t2, 8);
    let t2_hi = _mm_srli_si128(t2, 8);
    let t3 = _mm_clmulepi64_si128(t2_hi, poly, 0x00);
    _mm_xor_si128(_mm_xor_si128(lo, t1), _mm_xor_si128(t2_lo, t3))
}

/// Multiply two GCM elements (both already byte-swapped) using Karatsuba + PCLMULQDQ.
#[target_feature(enable = "pclmulqdq,sse2")]
#[inline]
pub(crate) unsafe fn clmul_gcm(a: __m128i, b: __m128i) -> __m128i {
    let lo = _mm_clmulepi64_si128(a, b, 0x00);
    let hi = _mm_clmulepi64_si128(a, b, 0x11);
    let a_swap = _mm_shuffle_epi32(a, 0x4e);
    let b_swap = _mm_shuffle_epi32(b, 0x4e);
    let mid = _mm_clmulepi64_si128(_mm_xor_si128(a, a_swap), _mm_xor_si128(b, b_swap), 0x00);
    let mid_true = _mm_xor_si128(mid, _mm_xor_si128(lo, hi));
    let product_lo = _mm_xor_si128(lo, _mm_slli_si128(mid_true, 8));
    let product_hi = _mm_xor_si128(hi, _mm_srli_si128(mid_true, 8));
    gcm_reduce(product_lo, product_hi)
}

/// Feed a byte slice into the running GHASH state (single-block loop).
#[target_feature(enable = "pclmulqdq,ssse3,sse2")]
pub(crate) unsafe fn ghash_update(mut state: __m128i, h: __m128i, data: &[u8]) -> __m128i {
    let n = data.len();
    let mut i = 0;
    while i + 16 <= n {
        let block = bitreverse_per_byte(_mm_loadu_si128(data.as_ptr().add(i).cast()));
        state = clmul_gcm(_mm_xor_si128(state, block), h);
        i += 16;
    }
    if i < n {
        let mut buf = [0u8; 16];
        buf[..n - i].copy_from_slice(&data[i..]);
        let block = bitreverse_per_byte(_mm_loadu_si128(buf.as_ptr().cast()));
        state = clmul_gcm(_mm_xor_si128(state, block), h);
    }
    state
}

/// Process 4 successive GHASH blocks in one aggregated step.
///
///   S' = S·H⁴ ⊕ B₁·H⁴ ⊕ B₂·H³ ⊕ B₃·H² ⊕ B₄·H
#[target_feature(enable = "pclmulqdq,ssse3,sse2")]
#[inline]
pub(crate) unsafe fn ghash_4blocks(
    state: __m128i,
    b1: __m128i,
    b2: __m128i,
    b3: __m128i,
    b4: __m128i,
    h_powers: &[__m128i; 16],
) -> __m128i {
    let b1 = bitreverse_per_byte(b1);
    let b2 = bitreverse_per_byte(b2);
    let b3 = bitreverse_per_byte(b3);
    let b4 = bitreverse_per_byte(b4);

    let h4 = h_powers[3];
    let h3 = h_powers[2];
    let h2 = h_powers[1];
    let h1 = h_powers[0];

    let t0 = clmul_gcm(state, h4);
    let t1 = clmul_gcm(b1, h4);
    let t2 = clmul_gcm(b2, h3);
    let t3 = clmul_gcm(b3, h2);
    let t4 = clmul_gcm(b4, h1);

    let left = _mm_xor_si128(t0, t1);
    let right = _mm_xor_si128(_mm_xor_si128(t2, t3), t4);
    _mm_xor_si128(left, right)
}

/// Process 8 successive GHASH blocks in one aggregated step.
///
///   S' = S·H⁸ ⊕ B₁·H⁸ ⊕ B₂·H⁷ ⊕ B₃·H⁶ ⊕ B₄·H⁵
///      ⊕ B₅·H⁴ ⊕ B₆·H³ ⊕ B₇·H² ⊕ B₈·H
#[target_feature(enable = "pclmulqdq,ssse3,sse2")]
#[inline]
pub(crate) unsafe fn ghash_8blocks(
    state: __m128i,
    b1: __m128i,
    b2: __m128i,
    b3: __m128i,
    b4: __m128i,
    b5: __m128i,
    b6: __m128i,
    b7: __m128i,
    b8: __m128i,
    h_powers: &[__m128i; 16],
) -> __m128i {
    let b1 = bitreverse_per_byte(b1);
    let b2 = bitreverse_per_byte(b2);
    let b3 = bitreverse_per_byte(b3);
    let b4 = bitreverse_per_byte(b4);
    let b5 = bitreverse_per_byte(b5);
    let b6 = bitreverse_per_byte(b6);
    let b7 = bitreverse_per_byte(b7);
    let b8 = bitreverse_per_byte(b8);

    let h8 = h_powers[7];
    let h7 = h_powers[6];
    let h6 = h_powers[5];
    let h5 = h_powers[4];
    let h4 = h_powers[3];
    let h3 = h_powers[2];
    let h2 = h_powers[1];
    let h1 = h_powers[0];

    let t0 = clmul_gcm(state, h8);
    let t1 = clmul_gcm(b1, h8);
    let t2 = clmul_gcm(b2, h7);
    let t3 = clmul_gcm(b3, h6);
    let t4 = clmul_gcm(b4, h5);
    let t5 = clmul_gcm(b5, h4);
    let t6 = clmul_gcm(b6, h3);
    let t7 = clmul_gcm(b7, h2);
    let t8 = clmul_gcm(b8, h1);

    let l0 = _mm_xor_si128(t0, t1);
    let l1 = _mm_xor_si128(t2, t3);
    let l2 = _mm_xor_si128(t4, t5);
    let l3 = _mm_xor_si128(t6, t7);
    let m0 = _mm_xor_si128(l0, l1);
    let m1 = _mm_xor_si128(l2, l3);
    _mm_xor_si128(m0, _mm_xor_si128(m1, t8))
}

// ── VPCLMULQDQ (256-bit) ──────────────────────────────────────────────────────

/// Carry-less product components of two 128-bit lanes, without reduction.
///
/// Returns `(lo, mid, hi)` where `mid` is the Karatsuba middle term with the
/// `lo`/`hi` correction already applied. XOR-ing these components across many
/// products is valid because the decomposition is linear over GF(2).
#[target_feature(enable = "vpclmulqdq,avx2")]
#[inline]
unsafe fn clmul_noreduce_256(a: __m256i, b: __m256i) -> (__m256i, __m256i, __m256i) {
    let lo = _mm256_clmulepi64_epi128::<0x00>(a, b);
    let hi = _mm256_clmulepi64_epi128::<0x11>(a, b);
    let a_swap = _mm256_shuffle_epi32(a, 0x4e);
    let b_swap = _mm256_shuffle_epi32(b, 0x4e);
    let mid = _mm256_clmulepi64_epi128::<0x00>(_mm256_xor_si256(a, a_swap), _mm256_xor_si256(b, b_swap));
    let mid = _mm256_xor_si256(mid, _mm256_xor_si256(lo, hi));
    (lo, mid, hi)
}

/// Fold the two 128-bit lanes of `x` together.
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn fold_lanes_256(x: __m256i) -> __m128i {
    _mm_xor_si128(_mm256_castsi256_si128(x), _mm256_extracti128_si256::<1>(x))
}

/// Process eight successive GHASH blocks (in four 256-bit vectors) using
/// VPCLMULQDQ.
///
/// Uses the `(S ⊕ B₁)·H⁸` identity so that the state and the first block share
/// one lane:
///
///   S' = (S·H⁸ ⊕ B₁·H⁸) ⊕ B₂·H⁷ ⊕ … ⊕ B₈·H¹
///
/// The four vector products are accumulated unreduced, their lanes are folded,
/// and a single reduction is performed.
#[target_feature(enable = "vpclmulqdq,avx2,ssse3,sse2")]
#[inline]
pub(crate) unsafe fn ghash_8blocks_vpclmul(
    state: __m128i,
    c0: __m256i,
    c1: __m256i,
    c2: __m256i,
    c3: __m256i,
    h_powers: &[__m128i; 16],
) -> __m128i {
    let c0 = bitreverse_per_byte_256(c0);
    let c1 = bitreverse_per_byte_256(c1);
    let c2 = bitreverse_per_byte_256(c2);
    let c3 = bitreverse_per_byte_256(c3);

    // Lane 0 = state xor B1, lane 1 = B2.
    let c0_lo = _mm_xor_si128(_mm256_castsi256_si128(c0), state);
    let c0 = _mm256_inserti128_si256::<0>(c0, c0_lo);

    // Powers in descending order: [H8, H7], [H6, H5], [H4, H3], [H2, H1].
    let p0 = _mm256_set_m128i(h_powers[6], h_powers[7]);
    let p1 = _mm256_set_m128i(h_powers[4], h_powers[5]);
    let p2 = _mm256_set_m128i(h_powers[2], h_powers[3]);
    let p3 = _mm256_set_m128i(h_powers[0], h_powers[1]);

    let (lo0, mi0, hi0) = clmul_noreduce_256(c0, p0);
    let (lo1, mi1, hi1) = clmul_noreduce_256(c1, p1);
    let (lo2, mi2, hi2) = clmul_noreduce_256(c2, p2);
    let (lo3, mi3, hi3) = clmul_noreduce_256(c3, p3);

    let lo = fold_lanes_256(_mm256_xor_si256(_mm256_xor_si256(lo0, lo1), _mm256_xor_si256(lo2, lo3)));
    let mid = fold_lanes_256(_mm256_xor_si256(_mm256_xor_si256(mi0, mi1), _mm256_xor_si256(mi2, mi3)));
    let hi = fold_lanes_256(_mm256_xor_si256(_mm256_xor_si256(hi0, hi1), _mm256_xor_si256(hi2, hi3)));

    let product_lo = _mm_xor_si128(lo, _mm_slli_si128::<8>(mid));
    let product_hi = _mm_xor_si128(hi, _mm_srli_si128::<8>(mid));
    gcm_reduce(product_lo, product_hi)
}

// ── VPCLMULQDQ (512-bit) ──────────────────────────────────────────────────────

/// Carry-less product components of four 128-bit lanes, without reduction.
#[target_feature(enable = "vpclmulqdq,avx512f,avx512bw,avx2")]
#[inline]
unsafe fn clmul_noreduce_512(a: __m512i, b: __m512i) -> (__m512i, __m512i, __m512i) {
    let lo = _mm512_clmulepi64_epi128::<0x00>(a, b);
    let hi = _mm512_clmulepi64_epi128::<0x11>(a, b);
    let a_swap = _mm512_shuffle_epi32::<0x4e>(a);
    let b_swap = _mm512_shuffle_epi32::<0x4e>(b);
    let mid = _mm512_clmulepi64_epi128::<0x00>(_mm512_xor_si512(a, a_swap), _mm512_xor_si512(b, b_swap));
    let mid = _mm512_xor_si512(mid, _mm512_xor_si512(lo, hi));
    (lo, mid, hi)
}

/// Fold the four 128-bit lanes of `x` together.
#[target_feature(enable = "avx512f")]
#[inline]
unsafe fn fold_lanes_512(x: __m512i) -> __m128i {
    let a = _mm512_castsi512_si128(x);
    let b = _mm512_extracti32x4_epi32::<1>(x);
    let c = _mm512_extracti32x4_epi32::<2>(x);
    let d = _mm512_extracti32x4_epi32::<3>(x);
    _mm_xor_si128(_mm_xor_si128(a, b), _mm_xor_si128(c, d))
}

/// Build a 512-bit vector from four 128-bit lanes (lane 0 = `a`).
#[target_feature(enable = "avx512f")]
#[inline]
unsafe fn build_power_512(a: __m128i, b: __m128i, c: __m128i, d: __m128i) -> __m512i {
    let mut p = _mm512_castsi128_si512(a);
    p = _mm512_inserti32x4::<1>(p, b);
    p = _mm512_inserti32x4::<2>(p, c);
    p = _mm512_inserti32x4::<3>(p, d);
    p
}

/// Process sixteen successive GHASH blocks (in four 512-bit vectors) using
/// VPCLMULQDQ.
///
/// The state is folded into the first block's lane, so 16 blocks + state fit in
/// 16 lane products:
///
///   [S⊕B₁, B₂, B₃, B₄] · [H¹⁶, H¹⁵, H¹⁴, H¹³]
///   [B₅..B₈]           · [H¹²..H⁹]
///   [B₉..B₁₂]          · [H⁸..H⁵]
///   [B₁₃..B₁₆]         · [H⁴..H¹]
///
/// The component-wise products are accumulated unreduced, the four lanes are
/// folded, and a single reduction is performed.
#[target_feature(enable = "vpclmulqdq,avx512f,avx512bw,avx2,ssse3")]
#[inline]
pub(crate) unsafe fn ghash_16blocks_vpclmul512(
    state: __m128i,
    c0: __m512i,
    c1: __m512i,
    c2: __m512i,
    c3: __m512i,
    h_powers: &[__m128i; 16],
) -> __m128i {
    let c0 = bitreverse_per_byte_512(c0);
    let c1 = bitreverse_per_byte_512(c1);
    let c2 = bitreverse_per_byte_512(c2);
    let c3 = bitreverse_per_byte_512(c3);

    let c0_lo = _mm_xor_si128(_mm512_castsi512_si128(c0), state);
    let c0 = _mm512_inserti32x4::<0>(c0, c0_lo);

    let p0 = build_power_512(h_powers[15], h_powers[14], h_powers[13], h_powers[12]);
    let p1 = build_power_512(h_powers[11], h_powers[10], h_powers[9], h_powers[8]);
    let p2 = build_power_512(h_powers[7], h_powers[6], h_powers[5], h_powers[4]);
    let p3 = build_power_512(h_powers[3], h_powers[2], h_powers[1], h_powers[0]);

    let (lo0, mi0, hi0) = clmul_noreduce_512(c0, p0);
    let (lo1, mi1, hi1) = clmul_noreduce_512(c1, p1);
    let (lo2, mi2, hi2) = clmul_noreduce_512(c2, p2);
    let (lo3, mi3, hi3) = clmul_noreduce_512(c3, p3);

    let lo = fold_lanes_512(_mm512_xor_si512(_mm512_xor_si512(lo0, lo1), _mm512_xor_si512(lo2, lo3)));
    let mid = fold_lanes_512(_mm512_xor_si512(_mm512_xor_si512(mi0, mi1), _mm512_xor_si512(mi2, mi3)));
    let hi = fold_lanes_512(_mm512_xor_si512(_mm512_xor_si512(hi0, hi1), _mm512_xor_si512(hi2, hi3)));

    let product_lo = _mm_xor_si128(lo, _mm_slli_si128::<8>(mid));
    let product_hi = _mm_xor_si128(hi, _mm_srli_si128::<8>(mid));
    gcm_reduce(product_lo, product_hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn have_vpclmul() -> bool {
        std::arch::is_x86_feature_detected!("vpclmulqdq")
            && std::arch::is_x86_feature_detected!("avx2")
            && std::arch::is_x86_feature_detected!("pclmulqdq")
    }

    fn have_vpclmul512() -> bool {
        have_vpclmul()
            && std::arch::is_x86_feature_detected!("avx512f")
            && std::arch::is_x86_feature_detected!("avx512bw")
    }

    fn make_h_powers() -> [__m128i; 16] {
        let key = [0x5au8; 32];
        let (bytes, _) = crate::aes::ghash::precompute_ghash_powers::<15>(&key);
        let mut hp = [unsafe { _mm_setzero_si128() }; 16];
        for i in 0..16 {
            hp[i] = unsafe { _mm_loadu_si128(bytes[i].as_ptr().cast()) };
        }
        hp
    }

    fn block(i: usize) -> [u8; 16] {
        core::array::from_fn(|j| (i as u8).wrapping_mul(37).wrapping_add(j as u8).wrapping_add(11))
    }

    /// Verify the power/lane pairing used by the 256-bit and 512-bit
    /// aggregators. This uses only 128-bit PCLMULQDQ, so it runs even on CPUs
    /// and emulators without VPCLMULQDQ.
    #[test]
    fn aggregated_power_pairing_matches_sequential_ghash() {
        if !std::arch::is_x86_feature_detected!("pclmulqdq") {
            eprintln!("skipping aggregated_power_pairing_matches_sequential_ghash: PCLMULQDQ not available");
            return;
        }

        let hp = make_h_powers();
        let state = unsafe { _mm_loadu_si128(block(31).as_ptr().cast()) };

        for count in [8usize, 16] {
            let blocks: [[u8; 16]; 16] = core::array::from_fn(block);
            let mut want = state;
            for b in blocks.iter().take(count) {
                let b = unsafe { _mm_loadu_si128(b.as_ptr().cast()) };
                want = unsafe { clmul_gcm(_mm_xor_si128(want, b), hp[0]) };
            }

            // Aggregated form: (S ^ B1) * H^count, then B_i * H^(count-i+1).
            let mut got = unsafe {
                let b0 = _mm_xor_si128(state, _mm_loadu_si128(blocks[0].as_ptr().cast()));
                clmul_gcm(b0, hp[count - 1])
            };
            for i in 1..count {
                let b = unsafe { _mm_loadu_si128(blocks[i].as_ptr().cast()) };
                let product = unsafe { clmul_gcm(b, hp[count - 1 - i]) };
                got = unsafe { _mm_xor_si128(got, product) };
            }

            let mut want_b = [0u8; 16];
            let mut got_b = [0u8; 16];
            unsafe {
                _mm_storeu_si128(want_b.as_mut_ptr().cast(), want);
                _mm_storeu_si128(got_b.as_mut_ptr().cast(), got);
            }
            assert_eq!(got_b, want_b, "power pairing mismatch for {count} blocks");
        }
    }

    #[test]
    fn ghash_8blocks_vpclmul_matches_128bit_path() {
        if !have_vpclmul() {
            eprintln!("skipping ghash_8blocks_vpclmul_matches_128bit_path: VPCLMULQDQ not available");
            return;
        }

        let hp = make_h_powers();
        let state = unsafe { _mm_loadu_si128(block(99).as_ptr().cast()) };
        let b: [[u8; 16]; 8] = core::array::from_fn(block);
        let xmm: [[__m128i; 8]; 1] = [core::array::from_fn(|i| unsafe {
            _mm_loadu_si128(b[i].as_ptr().cast())
        })];
        let want = unsafe {
            ghash_8blocks(
                state, xmm[0][0], xmm[0][1], xmm[0][2], xmm[0][3], xmm[0][4], xmm[0][5], xmm[0][6], xmm[0][7], &hp,
            )
        };
        let c0 = unsafe { _mm256_loadu_si256(b[0].as_ptr().cast()) };
        let c1 = unsafe { _mm256_loadu_si256(b[2].as_ptr().cast()) };
        let c2 = unsafe { _mm256_loadu_si256(b[4].as_ptr().cast()) };
        let c3 = unsafe { _mm256_loadu_si256(b[6].as_ptr().cast()) };
        let got = unsafe { ghash_8blocks_vpclmul(state, c0, c1, c2, c3, &hp) };
        let mut want_b = [0u8; 16];
        let mut got_b = [0u8; 16];
        unsafe {
            _mm_storeu_si128(want_b.as_mut_ptr().cast(), want);
            _mm_storeu_si128(got_b.as_mut_ptr().cast(), got);
        }
        assert_eq!(got_b, want_b);
    }

    #[test]
    fn ghash_16blocks_vpclmul512_matches_128bit_path() {
        if !have_vpclmul512() {
            eprintln!("skipping ghash_16blocks_vpclmul512_matches_128bit_path: AVX-512/VPCLMUL not available");
            return;
        }

        let hp = make_h_powers();
        let state = unsafe { _mm_loadu_si128(block(77).as_ptr().cast()) };
        let b: [[u8; 16]; 16] = core::array::from_fn(block);
        let b128: [[__m128i; 16]; 1] = [core::array::from_fn(|i| unsafe {
            _mm_loadu_si128(b[i].as_ptr().cast())
        })];
        let s1 = unsafe {
            ghash_8blocks(
                state, b128[0][0], b128[0][1], b128[0][2], b128[0][3], b128[0][4], b128[0][5], b128[0][6], b128[0][7],
                &hp,
            )
        };
        let want = unsafe {
            ghash_8blocks(
                s1,
                b128[0][8],
                b128[0][9],
                b128[0][10],
                b128[0][11],
                b128[0][12],
                b128[0][13],
                b128[0][14],
                b128[0][15],
                &hp,
            )
        };
        let c0 = unsafe { _mm512_loadu_si512(b[0].as_ptr().cast()) };
        let c1 = unsafe { _mm512_loadu_si512(b[4].as_ptr().cast()) };
        let c2 = unsafe { _mm512_loadu_si512(b[8].as_ptr().cast()) };
        let c3 = unsafe { _mm512_loadu_si512(b[12].as_ptr().cast()) };
        let got = unsafe { ghash_16blocks_vpclmul512(state, c0, c1, c2, c3, &hp) };
        let mut want_b = [0u8; 16];
        let mut got_b = [0u8; 16];
        unsafe {
            _mm_storeu_si128(want_b.as_mut_ptr().cast(), want);
            _mm_storeu_si128(got_b.as_mut_ptr().cast(), got);
        }
        assert_eq!(got_b, want_b);
    }
}
