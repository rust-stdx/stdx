//! WebAssembly `simd128` (2-way) Keccak-p[1600].
//!
//! Two independent states are processed in lockstep, one per 64-bit lane of a
//! `v128`. This is the only useful SIMD width on WebAssembly: the instruction
//! set has no per-lane variable shifts or rotates, so a single Keccak state
//! cannot use the rho step's distinct rotation amounts efficiently.
//!
//! Wasm has no runtime feature detection, so this module is only compiled when
//! `simd128` is enabled at build time (`-C target-feature=+simd128`).
//!
//! Rotations by 8 and 56 are byte shuffles, which is one instruction each.
//! Every other rho offset needs a shift/shift/or sequence because `i64x2_shl`
//! and `u64x2_shr` apply one shift count to both lanes.

use core::arch::wasm32::*;

use super::keccak::ROUND_CONSTANTS;

/// Rotates both 64-bit lanes left by `N` bits.
#[inline(always)]
fn rotl<const N: u32>(a: v128) -> v128 {
    debug_assert!(N > 0 && N < 64, "rotation amount must be in 1..64");
    return v128_or(i64x2_shl(a, N), u64x2_shr(a, 64 - N));
}

/// Rotates both 64-bit lanes left by 8 bits with one byte shuffle.
#[inline(always)]
fn rotl8(a: v128) -> v128 {
    return i8x16_shuffle::<7, 0, 1, 2, 3, 4, 5, 6, 15, 8, 9, 10, 11, 12, 13, 14>(a, a);
}

/// Rotates both 64-bit lanes left by 56 bits with one byte shuffle.
#[inline(always)]
fn rotl56(a: v128) -> v128 {
    return i8x16_shuffle::<1, 2, 3, 4, 5, 6, 7, 0, 9, 10, 11, 12, 13, 14, 15, 8>(a, a);
}

macro_rules! rotl {
    ($a:expr, 8) => {
        rotl8($a)
    };
    ($a:expr, 56) => {
        rotl56($a)
    };
    ($a:expr, $n:literal) => {
        rotl::<$n>($a)
    };
}

macro_rules! xor {
    ($a:expr, $b:expr) => {
        v128_xor($a, $b)
    };
}

/// `!$a & $b`, using the wasm `andnot` operand order.
macro_rules! andnot {
    ($a:expr, $b:expr) => {
        v128_andnot($b, $a)
    };
}

macro_rules! xor4 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        v128_xor(v128_xor(v128_xor($a, $b), v128_xor($c, $d)), $e)
    };
}

/// The Keccak-p[`ROUNDS`] permutation of two states stored as 25 interleaved
/// 128-bit vectors.
///
/// `state[i]` holds lane `i` of both states: lane 0 of the first state in the
/// low 64-bit half, lane 0 of the second state in the high half.
///
/// # Safety
///
/// `lanes` must be 16-byte aligned, and this function may only be called when
/// the `simd128` target feature is enabled (i.e. this module was compiled).
#[inline]
pub(super) unsafe fn permute<const ROUNDS: usize>(lanes: &mut [u64; 50]) {
    const {
        assert!(ROUNDS <= 24, "A round count greater than 24 is not supported.");
    }

    // SAFETY: `lanes` is 16-byte aligned (the caller's `KeccakX` is 64-byte
    // aligned) and holds exactly 50 words, the same size as `[v128; 25]`.
    let state: &mut [v128; 25] = unsafe { &mut *(lanes.as_mut_ptr().cast()) };
    keccak_p1600_x2::<ROUNDS>(state);
}

/// The unrolled Keccak round on two interleaved states.
fn keccak_p1600_x2<const ROUNDS: usize>(st: &mut [v128; 25]) {
    const {
        assert!(ROUNDS <= 24, "A round count greater than 24 is not supported.");
    }

    let (mut s0, mut s1, mut s2, mut s3, mut s4) = (st[0], st[1], st[2], st[3], st[4]);
    let (mut s5, mut s6, mut s7, mut s8, mut s9) = (st[5], st[6], st[7], st[8], st[9]);
    let (mut s10, mut s11, mut s12, mut s13, mut s14) = (st[10], st[11], st[12], st[13], st[14]);
    let (mut s15, mut s16, mut s17, mut s18, mut s19) = (st[15], st[16], st[17], st[18], st[19]);
    let (mut s20, mut s21, mut s22, mut s23, mut s24) = (st[20], st[21], st[22], st[23], st[24]);

    for &rc in ROUND_CONSTANTS[24 - ROUNDS..].iter() {
        let rc = u64x2_splat(rc);

        let c0 = xor4!(s0, s5, s10, s15, s20);
        let c1 = xor4!(s1, s6, s11, s16, s21);
        let c2 = xor4!(s2, s7, s12, s17, s22);
        let c3 = xor4!(s3, s8, s13, s18, s23);
        let c4 = xor4!(s4, s9, s14, s19, s24);
        let d0 = xor!(c4, rotl!(c1, 1));
        let d1 = xor!(c0, rotl!(c2, 1));
        let d2 = xor!(c1, rotl!(c3, 1));
        let d3 = xor!(c2, rotl!(c4, 1));
        let d4 = xor!(c3, rotl!(c0, 1));
        s0 = xor!(s0, d0);
        s1 = xor!(s1, d1);
        s2 = xor!(s2, d2);
        s3 = xor!(s3, d3);
        s4 = xor!(s4, d4);
        s5 = xor!(s5, d0);
        s6 = xor!(s6, d1);
        s7 = xor!(s7, d2);
        s8 = xor!(s8, d3);
        s9 = xor!(s9, d4);
        s10 = xor!(s10, d0);
        s11 = xor!(s11, d1);
        s12 = xor!(s12, d2);
        s13 = xor!(s13, d3);
        s14 = xor!(s14, d4);
        s15 = xor!(s15, d0);
        s16 = xor!(s16, d1);
        s17 = xor!(s17, d2);
        s18 = xor!(s18, d3);
        s19 = xor!(s19, d4);
        s20 = xor!(s20, d0);
        s21 = xor!(s21, d1);
        s22 = xor!(s22, d2);
        s23 = xor!(s23, d3);
        s24 = xor!(s24, d4);

        // Rho and pi, fully unrolled so every rotation is an immediate.
        let mut last = s1;
        let t0 = s10;
        s10 = rotl!(last, 1);
        last = t0;
        let t1 = s7;
        s7 = rotl!(last, 3);
        last = t1;
        let t2 = s11;
        s11 = rotl!(last, 6);
        last = t2;
        let t3 = s17;
        s17 = rotl!(last, 10);
        last = t3;
        let t4 = s18;
        s18 = rotl!(last, 15);
        last = t4;
        let t5 = s3;
        s3 = rotl!(last, 21);
        last = t5;
        let t6 = s5;
        s5 = rotl!(last, 28);
        last = t6;
        let t7 = s16;
        s16 = rotl!(last, 36);
        last = t7;
        let t8 = s8;
        s8 = rotl!(last, 45);
        last = t8;
        let t9 = s21;
        s21 = rotl!(last, 55);
        last = t9;
        let t10 = s24;
        s24 = rotl!(last, 2);
        last = t10;
        let t11 = s4;
        s4 = rotl!(last, 14);
        last = t11;
        let t12 = s15;
        s15 = rotl!(last, 27);
        last = t12;
        let t13 = s23;
        s23 = rotl!(last, 41);
        last = t13;
        let t14 = s19;
        s19 = rotl!(last, 56);
        last = t14;
        let t15 = s13;
        s13 = rotl!(last, 8);
        last = t15;
        let t16 = s12;
        s12 = rotl!(last, 25);
        last = t16;
        let t17 = s2;
        s2 = rotl!(last, 43);
        last = t17;
        let t18 = s20;
        s20 = rotl!(last, 62);
        last = t18;
        let t19 = s14;
        s14 = rotl!(last, 18);
        last = t19;
        let t20 = s22;
        s22 = rotl!(last, 39);
        last = t20;
        let t21 = s9;
        s9 = rotl!(last, 61);
        last = t21;
        let t22 = s6;
        s6 = rotl!(last, 20);
        last = t22;
        s1 = rotl!(last, 44);

        // Chi.
        let (b0, b1, b2, b3, b4) = (s0, s1, s2, s3, s4);
        s0 = xor!(b0, andnot!(b1, b2));
        s1 = xor!(b1, andnot!(b2, b3));
        s2 = xor!(b2, andnot!(b3, b4));
        s3 = xor!(b3, andnot!(b4, b0));
        s4 = xor!(b4, andnot!(b0, b1));
        let (b0, b1, b2, b3, b4) = (s5, s6, s7, s8, s9);
        s5 = xor!(b0, andnot!(b1, b2));
        s6 = xor!(b1, andnot!(b2, b3));
        s7 = xor!(b2, andnot!(b3, b4));
        s8 = xor!(b3, andnot!(b4, b0));
        s9 = xor!(b4, andnot!(b0, b1));
        let (b0, b1, b2, b3, b4) = (s10, s11, s12, s13, s14);
        s10 = xor!(b0, andnot!(b1, b2));
        s11 = xor!(b1, andnot!(b2, b3));
        s12 = xor!(b2, andnot!(b3, b4));
        s13 = xor!(b3, andnot!(b4, b0));
        s14 = xor!(b4, andnot!(b0, b1));
        let (b0, b1, b2, b3, b4) = (s15, s16, s17, s18, s19);
        s15 = xor!(b0, andnot!(b1, b2));
        s16 = xor!(b1, andnot!(b2, b3));
        s17 = xor!(b2, andnot!(b3, b4));
        s18 = xor!(b3, andnot!(b4, b0));
        s19 = xor!(b4, andnot!(b0, b1));
        let (b0, b1, b2, b3, b4) = (s20, s21, s22, s23, s24);
        s20 = xor!(b0, andnot!(b1, b2));
        s21 = xor!(b1, andnot!(b2, b3));
        s22 = xor!(b2, andnot!(b3, b4));
        s23 = xor!(b3, andnot!(b4, b0));
        s24 = xor!(b4, andnot!(b0, b1));

        // Iota.
        s0 = xor!(s0, rc);
    }

    st[0] = s0;
    st[1] = s1;
    st[2] = s2;
    st[3] = s3;
    st[4] = s4;
    st[5] = s5;
    st[6] = s6;
    st[7] = s7;
    st[8] = s8;
    st[9] = s9;
    st[10] = s10;
    st[11] = s11;
    st[12] = s12;
    st[13] = s13;
    st[14] = s14;
    st[15] = s15;
    st[16] = s16;
    st[17] = s17;
    st[18] = s18;
    st[19] = s19;
    st[20] = s20;
    st[21] = s21;
    st[22] = s22;
    st[23] = s23;
    st[24] = s24;
}

/// XORs two whole message lanes into the matching state vectors.
///
/// `dst` holds two interleaved states and must contain at least four words.
/// Every message must have at least `off + 16` bytes.
///
/// # Safety
///
/// The caller must uphold the length and `simd128` requirements above.
#[inline]
pub(super) unsafe fn xor_lanes(dst: &mut [u64], msgs: &[&[u8]; 2], off: usize) {
    // SAFETY: the caller guarantees that both messages have at least `off + 16`
    // bytes and that `dst` has at least four words. wasm vector loads and
    // stores are unaligned.
    let a = unsafe { v128_load(msgs[0].as_ptr().add(off).cast()) };
    let b = unsafe { v128_load(msgs[1].as_ptr().add(off).cast()) };
    let d0 = unsafe { v128_load(dst.as_ptr().cast()) };
    let d1 = unsafe { v128_load(dst.as_ptr().add(2).cast()) };

    // Transpose 2x2: [a0, a1], [b0, b1] -> [a0, b0], [a1, b1].
    let e0 = i64x2_shuffle::<0, 2>(a, b);
    let e1 = i64x2_shuffle::<1, 3>(a, b);

    // SAFETY: same length invariant as the loads above.
    unsafe {
        v128_store(dst.as_mut_ptr().cast(), v128_xor(d0, e0));
        v128_store(dst.as_mut_ptr().add(2).cast(), v128_xor(d1, e1));
    }
}

#[cfg(test)]
mod tests {
    use super::{permute, xor_lanes};
    use crate::sha3::keccak::{ROUND_CONSTANTS, p1600_soft};

    fn next_u64(state: &mut u64) -> u64 {
        // xorshift64*, adequate for deterministic test data.
        *state ^= *state >> 12;
        *state ^= *state << 25;
        *state ^= *state >> 27;
        return state.wrapping_mul(0x2545_f491_4f6c_dd1d);
    }

    fn random_pair(rng: &mut u64) -> [u64; 50] {
        let mut lanes = [0u64; 50];
        for lane in lanes.iter_mut() {
            *lane = next_u64(rng);
        }
        return lanes;
    }

    /// `permute` requires 16-byte alignment, matching the production
    /// `KeccakX` layout. A bare local array only has 8-byte alignment.
    #[repr(align(16))]
    struct Aligned([u64; 50]);

    /// The x2 kernel must agree with two independent portable permutations for
    /// every supported round count, including the first round (round constant
    /// index 0) and the 12/24 slices used by production.
    #[test]
    fn permutation_matches_soft_all_rounds() {
        let mut rng = 0x0123_4567_89ab_cdefu64;
        macro_rules! check {
            ($($rounds:literal),* $(,)?) => {
                $({
                    let mut lanes = Aligned(random_pair(&mut rng));
                    let expected = lanes.0;
                    let mut a = [0u64; 25];
                    let mut b = [0u64; 25];
                    for i in 0..25 {
                        a[i] = expected[2 * i];
                        b[i] = expected[2 * i + 1];
                    }
                    p1600_soft::<$rounds>(&mut a);
                    p1600_soft::<$rounds>(&mut b);

                    // SAFETY: `lanes` is 16-byte aligned and holds 50 words;
                    // the module is only compiled with `simd128`.
                    unsafe { permute::<$rounds>(&mut lanes.0) };

                    for i in 0..25 {
                        assert_eq!(lanes.0[2 * i], a[i], "ROUNDS={}, lane {i}, state 0", $rounds);
                        assert_eq!(lanes.0[2 * i + 1], b[i], "ROUNDS={}, lane {i}, state 1", $rounds);
                    }
                })*
            };
        }

        check!(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        );
    }

    /// The 12-round slice must be the last twelve FIPS 202 rounds.
    #[test]
    fn twelve_rounds_use_the_last_constants() {
        assert_eq!(
            ROUND_CONSTANTS[24 - 12..],
            [
                0x000000008000808b,
                0x800000000000008b,
                0x8000000000008089,
                0x8000000000008003,
                0x8000000000008002,
                0x8000000000000080,
                0x000000000000800a,
                0x800000008000000a,
                0x8000000080008081,
                0x8000000000008080,
                0x0000000080000001,
                0x8000000080008008,
            ]
        );
    }

    /// Directly checks the 2x2 transpose used by the batched absorb path.
    #[test]
    fn xor_lanes_matches_scalar_transpose() {
        let mut rng = 0x9e37_79b9_7f4a_7c15u64;
        let msgs: [std::vec::Vec<u8>; 2] = core::array::from_fn(|_| {
            let mut msg = std::vec![0u8; 64];
            for byte in msg.iter_mut() {
                *byte = next_u64(&mut rng) as u8;
            }
            msg
        });
        let refs: [&[u8]; 2] = core::array::from_fn(|w| &msgs[w][..]);

        for &off in &[0usize, 1, 3, 7, 8, 16, 24] {
            let mut lanes = [0u64; 8];
            for value in lanes.iter_mut() {
                *value = next_u64(&mut rng);
            }
            let before = lanes;

            // SAFETY: `lanes` holds 8 words; wasm vector loads and stores are
            // unaligned.
            unsafe { xor_lanes(&mut lanes, &refs, off) };

            for lane in 0..2 {
                for w in 0..2 {
                    let bytes = &msgs[w][off + lane * 8..off + lane * 8 + 8];
                    let word = u64::from_le_bytes(bytes.try_into().unwrap());
                    assert_eq!(
                        lanes[2 * lane + w],
                        before[2 * lane + w] ^ word,
                        "off {off}, lane {lane}, state {w}"
                    );
                }
            }
            assert_eq!(&lanes[4..], &before[4..], "off {off} wrote past the transpose");
        }
    }

    fn median(mut samples: std::vec::Vec<f64>) -> f64 {
        samples.sort_by(f64::total_cmp);
        return samples[samples.len() / 2];
    }

    /// Manual microbenchmark: scalar two-state permutation versus the wasm x2
    /// kernel. Ignored by default; run it on the wasm target with
    /// `--release --ignored`.
    #[cfg(feature = "std")]
    #[test]
    #[ignore = "manual wasm performance benchmark"]
    fn wasm_simd128_benchmark() {
        use std::{hint::black_box, time::Instant};

        const ITERS: usize = 100_000;
        const SAMPLES: usize = 5;

        let mut rng = 0x0123_4567_89ab_cdefu64;

        macro_rules! bench {
            ($rounds:literal) => {{
                let mut scalar_a = [0u64; 25];
                let mut scalar_b = [0u64; 25];
                for value in scalar_a.iter_mut().chain(scalar_b.iter_mut()) {
                    *value = next_u64(&mut rng);
                }
                let mut initial = [0u64; 50];
                for i in 0..25 {
                    initial[2 * i] = scalar_a[i];
                    initial[2 * i + 1] = scalar_b[i];
                }
                let mut simd = Aligned(initial);

                // Warm up both paths.
                for _ in 0..1_000 {
                    p1600_soft::<$rounds>(black_box(&mut scalar_a));
                    p1600_soft::<$rounds>(black_box(&mut scalar_b));
                    // SAFETY: `simd` is 16-byte aligned.
                    unsafe { permute::<$rounds>(black_box(&mut simd.0)) };
                }

                let mut scalar_ns = std::vec::Vec::with_capacity(SAMPLES);
                let mut simd_ns = std::vec::Vec::with_capacity(SAMPLES);

                for _ in 0..SAMPLES {
                    let start = Instant::now();
                    for _ in 0..ITERS {
                        p1600_soft::<$rounds>(black_box(&mut scalar_a));
                        p1600_soft::<$rounds>(black_box(&mut scalar_b));
                    }
                    scalar_ns.push(start.elapsed().as_nanos() as f64 / ITERS as f64);
                }

                for _ in 0..SAMPLES {
                    let start = Instant::now();
                    for _ in 0..ITERS {
                        // SAFETY: `simd` is 16-byte aligned.
                        unsafe { permute::<$rounds>(black_box(&mut simd.0)) };
                    }
                    simd_ns.push(start.elapsed().as_nanos() as f64 / ITERS as f64);
                }

                let scalar_ns = median(scalar_ns);
                let simd_ns = median(simd_ns);
                std::println!(
                    "rounds={} scalar_two_states={scalar_ns:.1} ns simd_two_states={simd_ns:.1} ns \
                     scalar_per_state={:.1} ns simd_per_state={:.1} ns speedup={:.2}x",
                    $rounds,
                    scalar_ns / 2.0,
                    simd_ns / 2.0,
                    scalar_ns / simd_ns,
                );
            }};
        }

        bench!(24);
        bench!(12);
    }
}
