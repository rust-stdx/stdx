#![allow(unsafe_op_in_unsafe_fn)]
use core::arch::aarch64::*;

/// aarch64 GHASH using ARMv8 Crypto extension PMULL instructions.
///
/// Uses `vmull_p64` intrinsics via inline asm for carry-less multiplication,
/// and `vrbitq_u8` for per-byte bit reversal (mapping between GCM's big-endian
/// polynomial representation and the little-endian PMULL domain).
#[cfg(test)]
use random::{RngExt, SeedableRng, rngs::ChaChaRng};

/// Single-block GHASH feed (tail processing).
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn ghash_update(mut state: uint8x16_t, h: uint8x16_t, data: &[u8]) -> uint8x16_t {
    let n = data.len();
    let mut i = 0usize;

    while i + 16 <= n {
        let block = vrbitq_u8(vld1q_u8(data.as_ptr().add(i)));
        state = clmul_gcm_pmull(veorq_u8(state, block), h);
        i += 16;
    }

    if i < n {
        let mut padded = [0u8; 16];
        padded[..n - i].copy_from_slice(&data[i..]);
        let block = vrbitq_u8(vld1q_u8(padded.as_ptr()));
        state = clmul_gcm_pmull(veorq_u8(state, block), h);
    }

    state
}

/// 4-block aggregated GHASH.
///
///   state' = state·H⁴ ⊕ B₁·H⁴ ⊕ B₂·H³ ⊕ B₃·H² ⊕ B₄·H
#[inline]
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn ghash_4blocks(
    state: uint8x16_t,
    b1: uint8x16_t,
    b2: uint8x16_t,
    b3: uint8x16_t,
    b4: uint8x16_t,
    h_powers: &[uint8x16_t; 8],
) -> uint8x16_t {
    let b1 = vrbitq_u8(b1);
    let b2 = vrbitq_u8(b2);
    let b3 = vrbitq_u8(b3);
    let b4 = vrbitq_u8(b4);

    let h1 = h_powers[0];
    let h2 = h_powers[1];
    let h3 = h_powers[2];
    let h4 = h_powers[3];

    let t0 = clmul_gcm_pmull(state, h4);
    let t1 = clmul_gcm_pmull(b1, h4);
    let t2 = clmul_gcm_pmull(b2, h3);
    let t3 = clmul_gcm_pmull(b3, h2);
    let t4 = clmul_gcm_pmull(b4, h1);

    let left = veorq_u8(t0, t1);
    let right = veorq_u8(veorq_u8(t2, t3), t4);
    veorq_u8(left, right)
}

/// 8-block aggregated GHASH.
///
///   state' = state·H⁸ ⊕ B₁·H⁸ ⊕ B₂·H⁷ ⊕ ... ⊕ B₈·H
#[inline]
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn ghash_8blocks(
    state: uint8x16_t,
    b1: uint8x16_t,
    b2: uint8x16_t,
    b3: uint8x16_t,
    b4: uint8x16_t,
    b5: uint8x16_t,
    b6: uint8x16_t,
    b7: uint8x16_t,
    b8: uint8x16_t,
    h_powers: &[uint8x16_t; 8],
) -> uint8x16_t {
    let b1 = vrbitq_u8(b1);
    let b2 = vrbitq_u8(b2);
    let b3 = vrbitq_u8(b3);
    let b4 = vrbitq_u8(b4);
    let b5 = vrbitq_u8(b5);
    let b6 = vrbitq_u8(b6);
    let b7 = vrbitq_u8(b7);
    let b8 = vrbitq_u8(b8);

    let h8 = h_powers[7];
    let h7 = h_powers[6];
    let h6 = h_powers[5];
    let h5 = h_powers[4];
    let h4 = h_powers[3];
    let h3 = h_powers[2];
    let h2 = h_powers[1];
    let h1 = h_powers[0];

    let t0 = clmul_gcm_pmull(state, h8);
    let t1 = clmul_gcm_pmull(b1, h8);
    let t2 = clmul_gcm_pmull(b2, h7);
    let t3 = clmul_gcm_pmull(b3, h6);
    let t4 = clmul_gcm_pmull(b4, h5);
    let t5 = clmul_gcm_pmull(b5, h4);
    let t6 = clmul_gcm_pmull(b6, h3);
    let t7 = clmul_gcm_pmull(b7, h2);
    let t8 = clmul_gcm_pmull(b8, h1);

    let l0 = veorq_u8(t0, t1);
    let l1 = veorq_u8(t2, t3);
    let l2 = veorq_u8(t4, t5);
    let l3 = veorq_u8(t6, t7);
    let m0 = veorq_u8(l0, l1);
    let m1 = veorq_u8(l2, l3);
    veorq_u8(m0, veorq_u8(m1, t8))
}

/// Multiply two GCM elements using PMULL + 3-step reduction.
#[inline]
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn clmul_gcm_pmull(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    let poly = vld1q_u8([0x87, 0, 0, 0, 0, 0, 0, 0, 0x87, 0, 0, 0, 0, 0, 0, 0].as_ptr());
    let zero = vdupq_n_u8(0);

    let a_p128 = vreinterpretq_p128_u8(a);
    let b_p128 = vreinterpretq_p128_u8(b);
    let poly_p128 = vreinterpretq_p128_u8(poly);
    let poly_p64x2 = vreinterpretq_p64_p128(poly_p128);

    let a_p64x2 = vreinterpretq_p64_p128(a_p128);
    let b_p64x2 = vreinterpretq_p64_p128(b_p128);

    // lo = pmull   v18.1q, a.d[0], b.d[0]
    let lo_p128 = vmull_p64(vgetq_lane_p64::<0>(a_p64x2), vgetq_lane_p64::<0>(b_p64x2));
    // hi = pmull2  v19.1q, a.2d, b.2d
    let hi_p128 = vmull_high_p64(a_p64x2, b_p64x2);

    // a_swap = ext a, a, #8; b_swap = ext b, b, #8
    let a_swap = vextq_u8(a, a, 8);
    let b_swap = vextq_u8(b, b, 8);
    // a_xor = a_swap ^ a; b_xor = b_swap ^ b
    let a_xor = veorq_u8(a_swap, a);
    let b_xor = veorq_u8(b_swap, b);
    // mid = pmull a_xor.d[0], b_xor.d[0]
    let mid_p128 = vmull_p64(
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(a_xor))),
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(b_xor))),
    );

    // mid ^= lo ^ hi
    let mut lo = vreinterpretq_u8_p128(lo_p128);
    let mut hi = vreinterpretq_u8_p128(hi_p128);
    let mid = vreinterpretq_u8_p128(mid_p128);
    let mid = veorq_u8(veorq_u8(mid, lo), hi);

    // ext zero, mid, #8 → lo ^= (mid shifted right by 8)
    lo = veorq_u8(lo, vextq_u8(zero, mid, 8));
    // ext mid, zero, #8 → hi ^= (mid shifted left by 8)
    hi = veorq_u8(hi, vextq_u8(mid, zero, 8));

    // Reduction step
    // r1 = pmull hi.d[0], poly.d[0]
    let r1_p128 = vmull_p64(
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(hi))),
        vgetq_lane_p64::<0>(poly_p64x2),
    );
    // hi_swap = ext hi, hi, #8
    let hi_swap = vextq_u8(hi, hi, 8);
    // r2 = pmull hi_swap.d[0], poly.d[0]
    let r2_p128 = vmull_p64(
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(hi_swap))),
        vgetq_lane_p64::<0>(poly_p64x2),
    );

    let r2 = vreinterpretq_u8_p128(r2_p128);
    // ext zero, r2, #8 → r2_lo
    let r2_lo = vextq_u8(zero, r2, 8);
    // ext r2, zero, #8 → r2_hi
    let r2_hi = vextq_u8(r2, zero, 8);
    // r3 = pmull r2_hi.d[0], poly.d[0]
    let r3_p128 = vmull_p64(
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(r2_hi))),
        vgetq_lane_p64::<0>(poly_p64x2),
    );

    // result = lo ^ r1 ^ r2_lo ^ r3
    veorq_u8(
        veorq_u8(veorq_u8(lo, vreinterpretq_u8_p128(r1_p128)), r2_lo),
        vreinterpretq_u8_p128(r3_p128),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aes::ghash::{bitreverse_bytes, gf128_mul, precompute_ghash_powers_8};

    fn have_armv8_crypto() -> bool {
        std::arch::is_aarch64_feature_detected!("aes") && std::arch::is_aarch64_feature_detected!("neon")
    }

    fn load(bytes: &[u8; 16]) -> uint8x16_t {
        unsafe { vld1q_u8(bytes.as_ptr()) }
    }

    fn store(value: uint8x16_t) -> [u8; 16] {
        let mut out = [0u8; 16];
        unsafe { vst1q_u8(out.as_mut_ptr(), value) };
        out
    }

    /// Deterministic pseudo-random block sequence (seeded RNG).
    fn blocks(count: usize, seed: u64) -> Vec<[u8; 16]> {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            out.push(rng.random::<[u8; 16]>());
        }
        out
    }

    /// The natural (big-endian) GHASH state after feeding `data`.
    fn ghash_natural(h: &[u8; 16], state: &[u8; 16], data: &[u8]) -> [u8; 16] {
        let mut state = *state;
        for chunk in data.chunks(16) {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            for i in 0..16 {
                state[i] ^= block[i];
            }
            state = gf128_mul(&state, h);
        }
        state
    }

    fn keys() -> [[u8; 32]; 4] {
        [[0u8; 32], [0xffu8; 32], [0x5au8; 32], {
            let mut k = [0u8; 32];
            for (i, b) in k.iter_mut().enumerate() {
                *b = (i as u8).wrapping_mul(37).wrapping_add(11);
            }
            k
        }]
    }

    /// `clmul_gcm_pmull` must equal the scalar GF(2^128) multiply, in the
    /// bit-reversed-per-byte domain the kernel operates in.
    #[test]
    fn clmul_matches_scalar() {
        if !have_armv8_crypto() {
            eprintln!("skipping clmul_matches_scalar: ARMv8 crypto extensions not available");
            return;
        }

        let inputs = blocks(64, 0x1234_5678_9abc_def0);
        for a in &inputs {
            for b in inputs.iter().take(16) {
                let want = bitreverse_bytes(&gf128_mul(a, b));
                let got = store(unsafe { clmul_gcm_pmull(load(&bitreverse_bytes(a)), load(&bitreverse_bytes(b))) });
                assert_eq!(got, want, "a={a:02x?} b={b:02x?}");
            }
        }
    }

    /// Zero, multiplicative identity (`0x80 00 …` in GCM order) and the
    /// irreducible-polynomial reduction case where the product's high bit is
    /// set must all behave like the scalar multiplier.
    #[test]
    fn clmul_edge_cases() {
        if !have_armv8_crypto() {
            eprintln!("skipping clmul_edge_cases: ARMv8 crypto extensions not available");
            return;
        }

        let zero = [0u8; 16];
        let one = {
            let mut b = [0u8; 16];
            b[0] = 0x80;
            b
        };
        let high = [0x80u8; 16];
        let ones = [0xffu8; 16];

        for x in [zero, one, high, ones, blocks(1, 0xdead_beef_cafe_1234)[0]] {
            for h in [zero, one, high, ones] {
                let want = bitreverse_bytes(&gf128_mul(&x, &h));
                let got = store(unsafe { clmul_gcm_pmull(load(&bitreverse_bytes(&x)), load(&bitreverse_bytes(&h))) });
                assert_eq!(got, want, "x={x:02x?} h={h:02x?}");
            }
            // Identity: x·1 = x.
            let identity =
                store(unsafe { clmul_gcm_pmull(load(&bitreverse_bytes(&x)), load(&bitreverse_bytes(&one))) });
            assert_eq!(identity, bitreverse_bytes(&x));
        }
    }

    /// `ghash_update` over every length 0..=129, including empty input, exact
    /// blocks and partial final blocks.
    #[test]
    fn ghash_update_matches_scalar_all_lengths() {
        if !have_armv8_crypto() {
            eprintln!("skipping ghash_update_matches_scalar_all_lengths: ARMv8 crypto extensions not available");
            return;
        }

        let data: Vec<u8> = (0..130u16)
            .map(|i| (i as u8).wrapping_mul(29).wrapping_add(3))
            .collect();
        for key in keys() {
            let (powers, h) = precompute_ghash_powers_8::<15>(&key);
            for len in 0..=129usize {
                let input = &data[..len];
                let state = load(&bitreverse_bytes(&blocks(1, 0x3141_5926_5358_9793)[0]));
                let got = store(unsafe { ghash_update(state, load(&powers[0]), input) });
                // `state` is in the bit-reversed domain: convert to natural before
                // running the scalar reference, then convert the result back.
                let state_natural = bitreverse_bytes(&store(state));
                let want = bitreverse_bytes(&ghash_natural(&h, &state_natural, input));
                assert_eq!(got, want, "len={len}");
            }
        }
    }

    /// Feeding the same bytes through a slice placed at a non-zero offset must
    /// give the same result (unaligned loads must not over-read).
    #[test]
    fn ghash_update_unaligned_offset() {
        if !have_armv8_crypto() {
            eprintln!("skipping ghash_update_unaligned_offset: ARMv8 crypto extensions not available");
            return;
        }

        let (powers, h) = precompute_ghash_powers_8::<15>(&[0x42u8; 32]);
        for len in [0usize, 1, 15, 16, 17, 31, 32, 33, 64, 65] {
            let mut backing = vec![0xa5u8; len + 3];
            for (i, byte) in backing[2..2 + len].iter_mut().enumerate() {
                *byte = (i as u8).wrapping_mul(7).wrapping_add(1);
            }
            let data = &backing[2..2 + len];

            let got = store(unsafe { ghash_update(vdupq_n_u8(0), load(&powers[0]), data) });
            let want = bitreverse_bytes(&ghash_natural(&h, &[0u8; 16], data));
            assert_eq!(got, want, "len={len}");
            // Canaries untouched (read-only kernel).
            assert_eq!(backing[0], 0xa5);
            assert_eq!(backing[1], 0xa5);
            assert_eq!(backing[2 + len], 0xa5);
        }
    }

    /// The 4-block aggregate must equal four sequential scalar GHASH steps,
    /// for arbitrary (including all-zero) state and blocks.
    #[test]
    fn ghash_4blocks_matches_sequential() {
        if !have_armv8_crypto() {
            eprintln!("skipping ghash_4blocks_matches_sequential: ARMv8 crypto extensions not available");
            return;
        }

        let (powers, h) = precompute_ghash_powers_8::<15>(&[0x37u8; 32]);
        let mut hp = [unsafe { vdupq_n_u8(0) }; 8];
        for (slot, power) in hp.iter_mut().zip(powers.iter()) {
            *slot = load(power);
        }

        let states = [[0u8; 16], [0xffu8; 16], blocks(1, 0x0f0f_0f0f_0f0f_0f0f)[0]];
        let zero = [0u8; 16];
        let data_blocks = blocks(8, 0xabcd_ef01_2345_6789);

        for state in states {
            // All-zero blocks: S' = S·H⁴.
            let got = store(unsafe {
                ghash_4blocks(
                    load(&bitreverse_bytes(&state)),
                    load(&bitreverse_bytes(&zero)),
                    load(&bitreverse_bytes(&zero)),
                    load(&bitreverse_bytes(&zero)),
                    load(&bitreverse_bytes(&zero)),
                    &hp,
                )
            });
            let mut want_natural = state;
            for _ in 0..4 {
                want_natural = gf128_mul(&want_natural, &h);
            }
            assert_eq!(got, bitreverse_bytes(&want_natural), "zero-block state={state:?}");

            // Arbitrary blocks. `ghash_4blocks` bit-reverses `b1..b4` itself.
            let got = store(unsafe {
                ghash_4blocks(
                    load(&bitreverse_bytes(&state)),
                    load(&data_blocks[0]),
                    load(&data_blocks[1]),
                    load(&data_blocks[2]),
                    load(&data_blocks[3]),
                    &hp,
                )
            });
            let mut concat = Vec::with_capacity(64);
            for b in &data_blocks[..4] {
                concat.extend_from_slice(b);
            }
            let want = bitreverse_bytes(&ghash_natural(&h, &state, &concat));
            assert_eq!(got, want, "state={state:?}");
        }
    }

    /// The 8-block aggregate indexes `h_powers[7]` (H⁸), the last slot of the
    /// fixed-size table; verify that pairing and that no larger index is used.
    #[test]
    fn ghash_8blocks_matches_sequential() {
        if !have_armv8_crypto() {
            eprintln!("skipping ghash_8blocks_matches_sequential: ARMv8 crypto extensions not available");
            return;
        }

        let (powers, h) = precompute_ghash_powers_8::<15>(&[0x91u8; 32]);
        let mut hp = [unsafe { vdupq_n_u8(0) }; 8];
        for (slot, power) in hp.iter_mut().zip(powers.iter()) {
            *slot = load(power);
        }

        let states = [[0u8; 16], [0xffu8; 16], blocks(1, 0x5555_aaaa_3333_cccc)[0]];
        let data_blocks = blocks(8, 0x1111_2222_3333_4444);

        for state in states {
            let s = load(&bitreverse_bytes(&state));
            // `ghash_8blocks` bit-reverses `b1..b8` itself.
            let b: Vec<uint8x16_t> = data_blocks.iter().map(load).collect();
            let got = store(unsafe { ghash_8blocks(s, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], &hp) });

            let mut concat = Vec::with_capacity(128);
            for block in &data_blocks {
                concat.extend_from_slice(block);
            }
            let want = bitreverse_bytes(&ghash_natural(&h, &state, &concat));
            assert_eq!(got, want, "state={state:02x?}");
        }

        // H⁸ is the highest power the kernel may touch; powers[7] must be H⁸.
        let mut h8_natural = h;
        for _ in 0..7 {
            h8_natural = gf128_mul(&h8_natural, &h);
        }
        assert_eq!(store(hp[7]), bitreverse_bytes(&h8_natural));
    }

    /// Precomputed powers must be H¹..H⁸ in bit-reversed-per-byte form.
    #[test]
    fn precomputed_powers_match_scalar() {
        for key in keys() {
            let (powers, h) = precompute_ghash_powers_8::<15>(&key);
            let mut power = h;
            for (i, expected) in powers.iter().enumerate() {
                if i != 0 {
                    power = gf128_mul(&power, &h);
                }
                assert_eq!(*expected, bitreverse_bytes(&power), "power {i}");
            }
        }
    }
}
