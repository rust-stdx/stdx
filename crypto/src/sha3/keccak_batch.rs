//! Batched Keccak-p[1600] permutations shared by the tree-hash constructions
//! and the ML-KEM / ML-DSA matrix, noise, and mask samplers.
//!
//! `W` independent Keccak states are processed in lockstep, one state per SIMD
//! lane. `ROUNDS` selects the Keccak-p round count: 24 for SHAKE-based
//! constructions, 12 for the last twelve rounds used by TurboSHAKE and
//! KangarooTwelve, and 16 for TurboHash.
//!
//! The batch width follows the widest backend the CPU supports:
//!
//! | Target        | Backend              | States per permutation |
//! |---------------|----------------------|------------------------|
//! | `x86_64`      | AVX-512F (runtime)   | 8                      |
//! | `x86_64`      | AVX2 (runtime)       | 4                      |
//! | `aarch64`     | Armv8.4-A SHA-3      | 2                      |
//! | `aarch64`     | NEON                 | 2                      |
//! | `wasm32`      | `simd128` (compile-time) | 2                  |
//! | anything else | portable scalar      | 2                      |
//!
//! On `x86_64`, runtime detection requires the `std` feature; without it the
//! backend is chosen at compile time from `target_feature`. WebAssembly has no
//! runtime feature detection, so `simd128` is used only when it is enabled at
//! build time (`-C target-feature=+simd128`). A backend that is narrower than
//! the requested batch width falls back to the portable code.

#![allow(unsafe_op_in_unsafe_fn)]

use core::cmp::min;

#[cfg(target_arch = "aarch64")]
use super::keccak::ROUND_CONSTANTS;

/// The batched permutation backend, detected once when a hasher is created.
///
/// Detecting here (instead of inside the permutation) keeps the hot loop free
/// of the runtime feature probe.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
pub(crate) enum Backend {
    Scalar,
    #[cfg(target_arch = "aarch64")]
    Neon,
    #[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
    Sha3,
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    Avx2,
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    Avx512,
    /// WebAssembly `simd128`, two states per 128-bit vector.
    ///
    /// Wasm has no runtime feature detection: this variant only exists when
    /// `-C target-feature=+simd128` was passed at build time.
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    WasmSimd128,
}

impl Backend {
    /// Detect the widest batched backend available.
    ///
    /// A statically enabled `target_feature` wins over the runtime probe: a
    /// binary built with `-C target-cpu=native` already requires the feature
    /// everywhere, and the probe can be masked by a hypervisor or an old
    /// kernel even when the CPU supports the instructions.
    #[allow(unreachable_code)]
    #[inline]
    pub(crate) fn detect() -> Backend {
        #[cfg(all(target_arch = "x86_64", feature = "std"))]
        {
            #[cfg(target_feature = "avx512f")]
            return Backend::Avx512;

            #[cfg(target_feature = "avx2")]
            return Backend::Avx2;

            if std::arch::is_x86_feature_detected!("avx512f") {
                return Backend::Avx512;
            }
            if std::arch::is_x86_feature_detected!("avx2") {
                return Backend::Avx2;
            }
        }

        #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx512f"))]
        return Backend::Avx512;

        #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx2"))]
        return Backend::Avx2;

        #[cfg(target_arch = "aarch64")]
        {
            #[cfg(target_feature = "sha3")]
            {
                return Backend::Sha3;
            }

            #[cfg(feature = "std")]
            if std::arch::is_aarch64_feature_detected!("sha3") {
                return Backend::Sha3;
            }

            return Backend::Neon;
        }

        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        return Backend::WasmSimd128;

        Backend::Scalar
    }

    /// Human-readable name of this backend, for diagnostics.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Backend::Scalar => "scalar",
            #[cfg(target_arch = "aarch64")]
            Backend::Neon => "neon",
            #[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
            Backend::Sha3 => "armv8.4-sha3",
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Backend::Avx2 => "avx2",
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Backend::Avx512 => "avx512",
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            Backend::WasmSimd128 => "wasm-simd128",
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// Batched Keccak-p[1600]
//////////////////////////////////////////////////////////////////////////////////////////////////

/// Largest batch width any target compiles in.
///
/// The backing array is sized for this so the struct type does not depend on
/// `W` through an arithmetic expression (which stable Rust does not allow in
/// array lengths). Narrower batches simply leave the tail of the array unused.
///
/// `x86_64` needs eight slots for the AVX-512 kernel; other targets top out at
/// four, which is what the `SpongeX<4, ROUNDS>` tests instantiate.
#[cfg(target_arch = "x86_64")]
pub(crate) const MAX_BATCH: usize = 8;
#[cfg(not(target_arch = "x86_64"))]
pub(crate) const MAX_BATCH: usize = 4;
/// Number of 64-bit lanes in a `MAX_BATCH`-wide state.
const MAX_LANES: usize = 25 * MAX_BATCH;

/// `W` Keccak-p[1600, `ROUNDS`] states processed in lockstep.
///
/// The state is stored lane-interleaved: `lanes[W * i + w]` is lane `i` of
/// state `w`. With `W = 2` that is exactly what a 128-bit NEON register wants
/// (states 0 and 1 in the low and high half), with `W = 4` a 256-bit AVX2
/// register, and with `W = 8` a 512-bit AVX-512 register. The array is 64-byte
/// aligned so every backend can load it with aligned accesses.
#[repr(C, align(64))]
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct KeccakX<const W: usize, const ROUNDS: usize> {
    lanes: [u64; MAX_LANES],
}

const _: () = assert!(
    core::mem::offset_of!(KeccakX<2, 24>, lanes) % 64 == 0,
    "the batched Keccak state must be 64-byte aligned for the SIMD views"
);
#[cfg(target_arch = "x86_64")]
const _: () = assert!(
    core::mem::offset_of!(KeccakX<8, 24>, lanes) % 64 == 0,
    "the x86-64 batched Keccak state must be 64-byte aligned for the SIMD views"
);

impl<const W: usize, const ROUNDS: usize> KeccakX<W, ROUNDS> {
    #[inline]
    const fn zero() -> Self {
        return KeccakX {
            lanes: [0; MAX_LANES],
        };
    }

    /// Apply Keccak-p[1600, `ROUNDS`] to all `W` states using the cached
    /// `backend`.
    ///
    /// If `backend` is wider than `W` (for example an eight-wide backend on a
    /// two-wide sponge, which only happens in tests), this falls back to scalar
    /// code for the active states.
    #[inline]
    fn permute(&mut self, backend: Backend) {
        const {
            assert!(
                ROUNDS == 12 || ROUNDS == 16 || ROUNDS == 24,
                "only Keccak-p[1600, 12], [1600, 16] and [1600, 24] are supported"
            );
        }

        match backend {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Backend::Avx512 => {
                if W == 8 {
                    // SAFETY: AVX-512F was confirmed at construction;
                    // `KeccakX` is 64-byte aligned and the array is
                    // `MAX_LANES` words.
                    unsafe { avx512::permute::<ROUNDS>(&mut self.lanes) };
                    return;
                }
            }
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Backend::Avx2 => {
                if W == 4 {
                    // SAFETY: AVX2 was confirmed at construction; `KeccakX` is
                    // 64-byte aligned and the array is `MAX_LANES` words.
                    unsafe { avx2::permute::<ROUNDS>(&mut self.lanes) };
                    return;
                }
            }
            #[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
            Backend::Sha3 => {
                if W == 2 {
                    let state: &mut [u64; 50] = (&mut self.lanes[..50]).try_into().unwrap();
                    // SAFETY: FEAT_SHA3 was confirmed at construction.
                    unsafe { sha3_ext::permute::<ROUNDS>(state) };
                    return;
                }
            }
            #[cfg(target_arch = "aarch64")]
            Backend::Neon => {
                if W == 2 {
                    let state: &mut [u64; 50] = (&mut self.lanes[..50]).try_into().unwrap();
                    // SAFETY: NEON (ASIMD) is mandatory on AArch64.
                    unsafe { neon::permute::<ROUNDS>(state) };
                    return;
                }
            }
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            Backend::WasmSimd128 => {
                if W == 2 {
                    let state: &mut [u64; 50] = (&mut self.lanes[..50]).try_into().unwrap();
                    // SAFETY: the module is only compiled with `simd128` and
                    // `KeccakX` is 64-byte aligned.
                    unsafe { super::keccak_wasm32_simd128::permute::<ROUNDS>(state) };
                    return;
                }
            }
            Backend::Scalar => {}
        }

        scalar::permute::<W, ROUNDS>(&mut self.lanes);
    }
}

/// Portable fallback: run the scalar permutation on each state in turn.
mod scalar {
    use crate::sha3::keccak::p1600;

    #[inline]
    pub(super) fn permute<const W: usize, const ROUNDS: usize>(lanes: &mut [u64]) {
        let mut state = [0u64; 25];
        for w in 0..W {
            for i in 0..25 {
                state[i] = lanes[W * i + w];
            }
            p1600::<ROUNDS>(&mut state);
            for i in 0..25 {
                lanes[W * i + w] = state[i];
            }
        }
    }
}

/// AVX2 4-way Keccak-p[1600].
///
/// Each `__m256i` holds the same lane of four states. The round is written as a
/// straight-line, by-value function and fully unrolled, like the AVX-512 kernel
/// above: a loop over `RHO`/`PI` keeps the state in memory and turns every
/// rotation into a broadcast of the shift count plus `vpsllvq`/`vpsrlvq`, which
/// costs about three times as many instructions as immediate shifts.
///
/// AVX2 has no per-64-bit-lane rotate, so `rotl` is two immediate shifts and an
/// `or`. Rotations by 8 and 56 could use `vpshufb`, but a byte-shuffle mask is a
/// memory operand and does not measurably reduce the round's port pressure.
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
mod avx2 {
    use core::arch::x86_64::*;

    use super::MAX_LANES;
    use crate::sha3::keccak::ROUND_CONSTANTS;

    /// # Safety
    /// The caller must have verified that the CPU implements AVX2, and `state`
    /// must be 32-byte aligned (guaranteed by `KeccakX`'s `repr(align(64))`) and
    /// exactly `MAX_LANES` words.
    #[inline]
    pub(super) unsafe fn permute<const ROUNDS: usize>(state: &mut [u64; MAX_LANES]) {
        // SAFETY: invariant documented on the function.
        let s: &mut [__m256i; 25] = unsafe { &mut *(state.as_mut_ptr().cast()) };
        keccak_p1600_x4::<ROUNDS>(s);
    }

    macro_rules! rotl {
        ($a:expr, $n:literal) => {
            _mm256_or_si256(_mm256_slli_epi64::<$n>($a), _mm256_srli_epi64::<{ 64 - $n }>($a))
        };
    }
    macro_rules! xor {
        ($a:expr, $b:expr) => {
            _mm256_xor_si256($a, $b)
        };
    }
    /// `_mm256_andnot_si256(a, b)` is `!a & b`, which is the `!b & c` of chi.
    macro_rules! andnot {
        ($a:expr, $b:expr) => {
            _mm256_andnot_si256($a, $b)
        };
    }
    macro_rules! xor4 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
            xor!(xor!(xor!($a, $b), xor!($c, $d)), $e)
        };
    }

    /// One Keccak-p[1600] round for four states.
    ///
    /// The state goes in and comes out by value so that [`keccak_p1600_x4`]
    /// expands the selected rounds into straight-line code. Chi writes a fresh
    /// set of names instead of overwriting its inputs, so the round needs no
    /// register-to-register copies of its own.
    #[inline(always)]
    unsafe fn round(s: [__m256i; 25], rc: u64) -> [__m256i; 25] {
        let [
            s0,
            s1,
            s2,
            s3,
            s4,
            s5,
            s6,
            s7,
            s8,
            s9,
            s10,
            s11,
            s12,
            s13,
            s14,
            s15,
            s16,
            s17,
            s18,
            s19,
            s20,
            s21,
            s22,
            s23,
            s24,
        ] = s;
        let rc = _mm256_set1_epi64x(rc as i64);

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
        let t0 = xor!(s0, d0);
        let t1 = xor!(s1, d1);
        let t2 = xor!(s2, d2);
        let t3 = xor!(s3, d3);
        let t4 = xor!(s4, d4);
        let t5 = xor!(s5, d0);
        let t6 = xor!(s6, d1);
        let t7 = xor!(s7, d2);
        let t8 = xor!(s8, d3);
        let t9 = xor!(s9, d4);
        let t10 = xor!(s10, d0);
        let t11 = xor!(s11, d1);
        let t12 = xor!(s12, d2);
        let t13 = xor!(s13, d3);
        let t14 = xor!(s14, d4);
        let t15 = xor!(s15, d0);
        let t16 = xor!(s16, d1);
        let t17 = xor!(s17, d2);
        let t18 = xor!(s18, d3);
        let t19 = xor!(s19, d4);
        let t20 = xor!(s20, d0);
        let t21 = xor!(s21, d1);
        let t22 = xor!(s22, d2);
        let t23 = xor!(s23, d3);
        let t24 = xor!(s24, d4);
        let n10 = rotl!(t1, 1);
        let n7 = rotl!(t10, 3);
        let n11 = rotl!(t7, 6);
        let n17 = rotl!(t11, 10);
        let n18 = rotl!(t17, 15);
        let n3 = rotl!(t18, 21);
        let n5 = rotl!(t3, 28);
        let n16 = rotl!(t5, 36);
        let n8 = rotl!(t16, 45);
        let n21 = rotl!(t8, 55);
        let n24 = rotl!(t21, 2);
        let n4 = rotl!(t24, 14);
        let n15 = rotl!(t4, 27);
        let n23 = rotl!(t15, 41);
        let n19 = rotl!(t23, 56);
        let n13 = rotl!(t19, 8);
        let n12 = rotl!(t13, 25);
        let n2 = rotl!(t12, 43);
        let n20 = rotl!(t2, 62);
        let n14 = rotl!(t20, 18);
        let n22 = rotl!(t14, 39);
        let n9 = rotl!(t22, 61);
        let n6 = rotl!(t9, 20);
        let n1 = rotl!(t6, 44);
        let (p0, p1, p2, p3, p4) = (t0, n1, n2, n3, n4);
        let o0 = xor!(p0, andnot!(p1, p2));
        let o1 = xor!(p1, andnot!(p2, p3));
        let o2 = xor!(p2, andnot!(p3, p4));
        let o3 = xor!(p3, andnot!(p4, p0));
        let o4 = xor!(p4, andnot!(p0, p1));
        let (p0, p1, p2, p3, p4) = (n5, n6, n7, n8, n9);
        let o5 = xor!(p0, andnot!(p1, p2));
        let o6 = xor!(p1, andnot!(p2, p3));
        let o7 = xor!(p2, andnot!(p3, p4));
        let o8 = xor!(p3, andnot!(p4, p0));
        let o9 = xor!(p4, andnot!(p0, p1));
        let (p0, p1, p2, p3, p4) = (n10, n11, n12, n13, n14);
        let o10 = xor!(p0, andnot!(p1, p2));
        let o11 = xor!(p1, andnot!(p2, p3));
        let o12 = xor!(p2, andnot!(p3, p4));
        let o13 = xor!(p3, andnot!(p4, p0));
        let o14 = xor!(p4, andnot!(p0, p1));
        let (p0, p1, p2, p3, p4) = (n15, n16, n17, n18, n19);
        let o15 = xor!(p0, andnot!(p1, p2));
        let o16 = xor!(p1, andnot!(p2, p3));
        let o17 = xor!(p2, andnot!(p3, p4));
        let o18 = xor!(p3, andnot!(p4, p0));
        let o19 = xor!(p4, andnot!(p0, p1));
        let (p0, p1, p2, p3, p4) = (n20, n21, n22, n23, n24);
        let o20 = xor!(p0, andnot!(p1, p2));
        let o21 = xor!(p1, andnot!(p2, p3));
        let o22 = xor!(p2, andnot!(p3, p4));
        let o23 = xor!(p3, andnot!(p4, p0));
        let o24 = xor!(p4, andnot!(p0, p1));
        let o0 = xor!(o0, rc);

        return [
            o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15, o16, o17, o18, o19, o20, o21, o22,
            o23, o24,
        ];
    }

    /// # Safety
    /// AVX2 must be available.
    #[target_feature(enable = "avx2")]
    unsafe fn keccak_p1600_x4<const ROUNDS: usize>(st: &mut [__m256i; 25]) {
        let mut s = *st;
        match ROUNDS {
            12 => {
                s = round(s, ROUND_CONSTANTS[12]);
                s = round(s, ROUND_CONSTANTS[13]);
                s = round(s, ROUND_CONSTANTS[14]);
                s = round(s, ROUND_CONSTANTS[15]);
                s = round(s, ROUND_CONSTANTS[16]);
                s = round(s, ROUND_CONSTANTS[17]);
                s = round(s, ROUND_CONSTANTS[18]);
                s = round(s, ROUND_CONSTANTS[19]);
                s = round(s, ROUND_CONSTANTS[20]);
                s = round(s, ROUND_CONSTANTS[21]);
                s = round(s, ROUND_CONSTANTS[22]);
                s = round(s, ROUND_CONSTANTS[23]);
            }
            24 => {
                s = round(s, ROUND_CONSTANTS[0]);
                s = round(s, ROUND_CONSTANTS[1]);
                s = round(s, ROUND_CONSTANTS[2]);
                s = round(s, ROUND_CONSTANTS[3]);
                s = round(s, ROUND_CONSTANTS[4]);
                s = round(s, ROUND_CONSTANTS[5]);
                s = round(s, ROUND_CONSTANTS[6]);
                s = round(s, ROUND_CONSTANTS[7]);
                s = round(s, ROUND_CONSTANTS[8]);
                s = round(s, ROUND_CONSTANTS[9]);
                s = round(s, ROUND_CONSTANTS[10]);
                s = round(s, ROUND_CONSTANTS[11]);
                s = round(s, ROUND_CONSTANTS[12]);
                s = round(s, ROUND_CONSTANTS[13]);
                s = round(s, ROUND_CONSTANTS[14]);
                s = round(s, ROUND_CONSTANTS[15]);
                s = round(s, ROUND_CONSTANTS[16]);
                s = round(s, ROUND_CONSTANTS[17]);
                s = round(s, ROUND_CONSTANTS[18]);
                s = round(s, ROUND_CONSTANTS[19]);
                s = round(s, ROUND_CONSTANTS[20]);
                s = round(s, ROUND_CONSTANTS[21]);
                s = round(s, ROUND_CONSTANTS[22]);
                s = round(s, ROUND_CONSTANTS[23]);
            }
            16 => {
                s = round(s, ROUND_CONSTANTS[8]);
                s = round(s, ROUND_CONSTANTS[9]);
                s = round(s, ROUND_CONSTANTS[10]);
                s = round(s, ROUND_CONSTANTS[11]);
                s = round(s, ROUND_CONSTANTS[12]);
                s = round(s, ROUND_CONSTANTS[13]);
                s = round(s, ROUND_CONSTANTS[14]);
                s = round(s, ROUND_CONSTANTS[15]);
                s = round(s, ROUND_CONSTANTS[16]);
                s = round(s, ROUND_CONSTANTS[17]);
                s = round(s, ROUND_CONSTANTS[18]);
                s = round(s, ROUND_CONSTANTS[19]);
                s = round(s, ROUND_CONSTANTS[20]);
                s = round(s, ROUND_CONSTANTS[21]);
                s = round(s, ROUND_CONSTANTS[22]);
                s = round(s, ROUND_CONSTANTS[23]);
            }
            _ => unreachable!(),
        }
        *st = s;
    }

    /// AVX2 counterpart of [`super::avx512::xor_lanes`]: a 4x4 transpose of
    /// 64-bit words for four messages.
    ///
    /// # Safety
    /// `dst` must be 32-byte aligned and hold at least 16 words, and every
    /// `msgs[w]` must have at least `off + 32` bytes.
    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn xor_lanes(dst: &mut [u64], msgs: &[&[u8]; 4], off: usize) {
        // SAFETY: `dst` is aligned and long enough, as documented above.
        let dst: &mut [__m256i; 4] = unsafe { &mut *(dst.as_mut_ptr().cast()) };

        // SAFETY: every message has at least `off + 32` bytes.
        let r0 = unsafe { _mm256_loadu_si256(msgs[0].as_ptr().add(off).cast()) };
        let r1 = unsafe { _mm256_loadu_si256(msgs[1].as_ptr().add(off).cast()) };
        let r2 = unsafe { _mm256_loadu_si256(msgs[2].as_ptr().add(off).cast()) };
        let r3 = unsafe { _mm256_loadu_si256(msgs[3].as_ptr().add(off).cast()) };

        // 4x4 transpose of 64-bit words.
        let t0 = _mm256_unpacklo_epi64(r0, r1);
        let t1 = _mm256_unpackhi_epi64(r0, r1);
        let t2 = _mm256_unpacklo_epi64(r2, r3);
        let t3 = _mm256_unpackhi_epi64(r2, r3);

        dst[0] = _mm256_xor_si256(dst[0], _mm256_permute2x128_si256::<0x20>(t0, t2));
        dst[1] = _mm256_xor_si256(dst[1], _mm256_permute2x128_si256::<0x20>(t1, t3));
        dst[2] = _mm256_xor_si256(dst[2], _mm256_permute2x128_si256::<0x31>(t0, t2));
        dst[3] = _mm256_xor_si256(dst[3], _mm256_permute2x128_si256::<0x31>(t1, t3));
    }
}

/// AVX-512F 8-way Keccak-p[1600].
///
/// One `zmm` register holds the same lane of eight states, so a single
/// permutation absorbs eight blocks at once. AVX-512F provides 32 vector
/// registers, which is enough to keep the whole 25-lane state resident, and
/// `vprolq` rotates a lane in one instruction (AVX2 needs two shifts and an
/// `or`). `vpternlogq` folds the three-input boolean steps of theta and chi
/// into one instruction as well. The kernel uses only the AVX-512 Foundation
/// subset (`vprolq`, `vpternlogq`, `vpandnq`, `vpxorq`) so it runs on every
/// CPU that advertises `avx512f`.
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
mod avx512 {
    use core::arch::x86_64::*;

    use super::MAX_LANES;
    use crate::sha3::keccak::ROUND_CONSTANTS;

    /// # Safety
    /// The caller must have verified that the CPU implements AVX-512F, and
    /// `state` must be 64-byte aligned and exactly `MAX_LANES` words.
    #[inline]
    pub(super) unsafe fn permute<const ROUNDS: usize>(state: &mut [u64; MAX_LANES]) {
        // SAFETY: invariant documented on the function.
        let s: &mut [__m512i; 25] = unsafe { &mut *(state.as_mut_ptr().cast()) };
        keccak_p1600_x8::<ROUNDS>(s);
    }

    macro_rules! rotl {
        ($a:expr, $n:literal) => {
            _mm512_rol_epi64::<$n>($a)
        };
    }
    macro_rules! xor {
        ($a:expr, $b:expr) => {
            _mm512_xor_si512($a, $b)
        };
    }
    macro_rules! andnot {
        ($a:expr, $b:expr) => {
            _mm512_andnot_si512($a, $b)
        };
    }
    macro_rules! xor4 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
            _mm512_xor_si512(_mm512_xor_si512(_mm512_xor_si512($a, $b), _mm512_xor_si512($c, $d)), $e)
        };
    }

    /// One Keccak-p[1600] round.
    ///
    /// The state goes in and comes out by value so that [`keccak_p1600_x8`]
    /// expands the selected rounds into straight-line code. With a `for` loop over
    /// [`ROUND_CONSTANTS`] the backend keeps the array in memory and re-loads
    /// all 25 lanes every round, which costs about 30% more instructions.
    /// Chi writes a fresh set of names instead of overwriting its inputs, so
    /// the round needs no register-to-register copies.
    #[inline(always)]
    unsafe fn round(s: [__m512i; 25], rc: u64) -> [__m512i; 25] {
        let [
            s0,
            s1,
            s2,
            s3,
            s4,
            s5,
            s6,
            s7,
            s8,
            s9,
            s10,
            s11,
            s12,
            s13,
            s14,
            s15,
            s16,
            s17,
            s18,
            s19,
            s20,
            s21,
            s22,
            s23,
            s24,
        ] = s;
        let rc = _mm512_set1_epi64(rc as i64);

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
        let t0 = xor!(s0, d0);
        let t1 = xor!(s1, d1);
        let t2 = xor!(s2, d2);
        let t3 = xor!(s3, d3);
        let t4 = xor!(s4, d4);
        let t5 = xor!(s5, d0);
        let t6 = xor!(s6, d1);
        let t7 = xor!(s7, d2);
        let t8 = xor!(s8, d3);
        let t9 = xor!(s9, d4);
        let t10 = xor!(s10, d0);
        let t11 = xor!(s11, d1);
        let t12 = xor!(s12, d2);
        let t13 = xor!(s13, d3);
        let t14 = xor!(s14, d4);
        let t15 = xor!(s15, d0);
        let t16 = xor!(s16, d1);
        let t17 = xor!(s17, d2);
        let t18 = xor!(s18, d3);
        let t19 = xor!(s19, d4);
        let t20 = xor!(s20, d0);
        let t21 = xor!(s21, d1);
        let t22 = xor!(s22, d2);
        let t23 = xor!(s23, d3);
        let t24 = xor!(s24, d4);
        let n10 = rotl!(t1, 1);
        let n7 = rotl!(t10, 3);
        let n11 = rotl!(t7, 6);
        let n17 = rotl!(t11, 10);
        let n18 = rotl!(t17, 15);
        let n3 = rotl!(t18, 21);
        let n5 = rotl!(t3, 28);
        let n16 = rotl!(t5, 36);
        let n8 = rotl!(t16, 45);
        let n21 = rotl!(t8, 55);
        let n24 = rotl!(t21, 2);
        let n4 = rotl!(t24, 14);
        let n15 = rotl!(t4, 27);
        let n23 = rotl!(t15, 41);
        let n19 = rotl!(t23, 56);
        let n13 = rotl!(t19, 8);
        let n12 = rotl!(t13, 25);
        let n2 = rotl!(t12, 43);
        let n20 = rotl!(t2, 62);
        let n14 = rotl!(t20, 18);
        let n22 = rotl!(t14, 39);
        let n9 = rotl!(t22, 61);
        let n6 = rotl!(t9, 20);
        let n1 = rotl!(t6, 44);
        let (p0, p1, p2, p3, p4) = (t0, n1, n2, n3, n4);
        let o0 = xor!(p0, andnot!(p1, p2));
        let o1 = xor!(p1, andnot!(p2, p3));
        let o2 = xor!(p2, andnot!(p3, p4));
        let o3 = xor!(p3, andnot!(p4, p0));
        let o4 = xor!(p4, andnot!(p0, p1));
        let (p0, p1, p2, p3, p4) = (n5, n6, n7, n8, n9);
        let o5 = xor!(p0, andnot!(p1, p2));
        let o6 = xor!(p1, andnot!(p2, p3));
        let o7 = xor!(p2, andnot!(p3, p4));
        let o8 = xor!(p3, andnot!(p4, p0));
        let o9 = xor!(p4, andnot!(p0, p1));
        let (p0, p1, p2, p3, p4) = (n10, n11, n12, n13, n14);
        let o10 = xor!(p0, andnot!(p1, p2));
        let o11 = xor!(p1, andnot!(p2, p3));
        let o12 = xor!(p2, andnot!(p3, p4));
        let o13 = xor!(p3, andnot!(p4, p0));
        let o14 = xor!(p4, andnot!(p0, p1));
        let (p0, p1, p2, p3, p4) = (n15, n16, n17, n18, n19);
        let o15 = xor!(p0, andnot!(p1, p2));
        let o16 = xor!(p1, andnot!(p2, p3));
        let o17 = xor!(p2, andnot!(p3, p4));
        let o18 = xor!(p3, andnot!(p4, p0));
        let o19 = xor!(p4, andnot!(p0, p1));
        let (p0, p1, p2, p3, p4) = (n20, n21, n22, n23, n24);
        let o20 = xor!(p0, andnot!(p1, p2));
        let o21 = xor!(p1, andnot!(p2, p3));
        let o22 = xor!(p2, andnot!(p3, p4));
        let o23 = xor!(p3, andnot!(p4, p0));
        let o24 = xor!(p4, andnot!(p0, p1));
        let o0 = xor!(o0, rc);

        return [
            o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15, o16, o17, o18, o19, o20, o21, o22,
            o23, o24,
        ];
    }

    /// # Safety
    /// AVX-512F must be available.
    #[target_feature(enable = "avx512f")]
    unsafe fn keccak_p1600_x8<const ROUNDS: usize>(st: &mut [__m512i; 25]) {
        let mut s = *st;
        match ROUNDS {
            12 => {
                s = round(s, ROUND_CONSTANTS[12]);
                s = round(s, ROUND_CONSTANTS[13]);
                s = round(s, ROUND_CONSTANTS[14]);
                s = round(s, ROUND_CONSTANTS[15]);
                s = round(s, ROUND_CONSTANTS[16]);
                s = round(s, ROUND_CONSTANTS[17]);
                s = round(s, ROUND_CONSTANTS[18]);
                s = round(s, ROUND_CONSTANTS[19]);
                s = round(s, ROUND_CONSTANTS[20]);
                s = round(s, ROUND_CONSTANTS[21]);
                s = round(s, ROUND_CONSTANTS[22]);
                s = round(s, ROUND_CONSTANTS[23]);
            }
            24 => {
                s = round(s, ROUND_CONSTANTS[0]);
                s = round(s, ROUND_CONSTANTS[1]);
                s = round(s, ROUND_CONSTANTS[2]);
                s = round(s, ROUND_CONSTANTS[3]);
                s = round(s, ROUND_CONSTANTS[4]);
                s = round(s, ROUND_CONSTANTS[5]);
                s = round(s, ROUND_CONSTANTS[6]);
                s = round(s, ROUND_CONSTANTS[7]);
                s = round(s, ROUND_CONSTANTS[8]);
                s = round(s, ROUND_CONSTANTS[9]);
                s = round(s, ROUND_CONSTANTS[10]);
                s = round(s, ROUND_CONSTANTS[11]);
                s = round(s, ROUND_CONSTANTS[12]);
                s = round(s, ROUND_CONSTANTS[13]);
                s = round(s, ROUND_CONSTANTS[14]);
                s = round(s, ROUND_CONSTANTS[15]);
                s = round(s, ROUND_CONSTANTS[16]);
                s = round(s, ROUND_CONSTANTS[17]);
                s = round(s, ROUND_CONSTANTS[18]);
                s = round(s, ROUND_CONSTANTS[19]);
                s = round(s, ROUND_CONSTANTS[20]);
                s = round(s, ROUND_CONSTANTS[21]);
                s = round(s, ROUND_CONSTANTS[22]);
                s = round(s, ROUND_CONSTANTS[23]);
            }
            16 => {
                s = round(s, ROUND_CONSTANTS[8]);
                s = round(s, ROUND_CONSTANTS[9]);
                s = round(s, ROUND_CONSTANTS[10]);
                s = round(s, ROUND_CONSTANTS[11]);
                s = round(s, ROUND_CONSTANTS[12]);
                s = round(s, ROUND_CONSTANTS[13]);
                s = round(s, ROUND_CONSTANTS[14]);
                s = round(s, ROUND_CONSTANTS[15]);
                s = round(s, ROUND_CONSTANTS[16]);
                s = round(s, ROUND_CONSTANTS[17]);
                s = round(s, ROUND_CONSTANTS[18]);
                s = round(s, ROUND_CONSTANTS[19]);
                s = round(s, ROUND_CONSTANTS[20]);
                s = round(s, ROUND_CONSTANTS[21]);
                s = round(s, ROUND_CONSTANTS[22]);
                s = round(s, ROUND_CONSTANTS[23]);
            }
            _ => unreachable!(),
        }
        *st = s;
    }

    /// XORs eight whole lanes of eight messages into the state.
    ///
    /// Lane `L` of the state holds the same lane of each of the eight
    /// messages, so absorbing `W` lanes at a time turns into an 8x8 transpose
    /// of 64-bit words followed by one XOR per lane. The transpose is what
    /// makes this worthwhile: the scalar alternative loads and stores one
    /// 64-bit word at a time, eight times per lane.
    ///
    /// # Safety
    /// `dst` must be 64-byte aligned and hold at least 64 words, and every
    /// `msgs[w]` must have at least `off + 64` bytes.
    #[target_feature(enable = "avx512f")]
    pub(super) unsafe fn xor_lanes(dst: &mut [u64], msgs: &[&[u8]; 8], off: usize) {
        // SAFETY: `dst` is aligned and long enough, as documented above.
        let dst: &mut [__m512i; 8] = unsafe { &mut *(dst.as_mut_ptr().cast()) };

        // SAFETY: every message has at least `off + 64` bytes.
        let r0 = unsafe { _mm512_loadu_si512(msgs[0].as_ptr().add(off).cast()) };
        let r1 = unsafe { _mm512_loadu_si512(msgs[1].as_ptr().add(off).cast()) };
        let r2 = unsafe { _mm512_loadu_si512(msgs[2].as_ptr().add(off).cast()) };
        let r3 = unsafe { _mm512_loadu_si512(msgs[3].as_ptr().add(off).cast()) };
        let r4 = unsafe { _mm512_loadu_si512(msgs[4].as_ptr().add(off).cast()) };
        let r5 = unsafe { _mm512_loadu_si512(msgs[5].as_ptr().add(off).cast()) };
        let r6 = unsafe { _mm512_loadu_si512(msgs[6].as_ptr().add(off).cast()) };
        let r7 = unsafe { _mm512_loadu_si512(msgs[7].as_ptr().add(off).cast()) };

        // 8x8 transpose of 64-bit words, three shuffle stages.
        let t0 = _mm512_unpacklo_epi64(r0, r1);
        let t1 = _mm512_unpackhi_epi64(r0, r1);
        let t2 = _mm512_unpacklo_epi64(r2, r3);
        let t3 = _mm512_unpackhi_epi64(r2, r3);
        let t4 = _mm512_unpacklo_epi64(r4, r5);
        let t5 = _mm512_unpackhi_epi64(r4, r5);
        let t6 = _mm512_unpacklo_epi64(r6, r7);
        let t7 = _mm512_unpackhi_epi64(r6, r7);

        let u0 = _mm512_shuffle_i64x2::<0x88>(t0, t2);
        let u1 = _mm512_shuffle_i64x2::<0xdd>(t0, t2);
        let u2 = _mm512_shuffle_i64x2::<0x88>(t1, t3);
        let u3 = _mm512_shuffle_i64x2::<0xdd>(t1, t3);
        let u4 = _mm512_shuffle_i64x2::<0x88>(t4, t6);
        let u5 = _mm512_shuffle_i64x2::<0xdd>(t4, t6);
        let u6 = _mm512_shuffle_i64x2::<0x88>(t5, t7);
        let u7 = _mm512_shuffle_i64x2::<0xdd>(t5, t7);

        // The three shuffle stages leave the columns in bit-reversed order
        // (0, 4, 2, 6, 1, 5, 3, 7); write them back in natural order.
        dst[0] = _mm512_xor_si512(dst[0], _mm512_shuffle_i64x2::<0x88>(u0, u4));
        dst[1] = _mm512_xor_si512(dst[1], _mm512_shuffle_i64x2::<0x88>(u2, u6));
        dst[2] = _mm512_xor_si512(dst[2], _mm512_shuffle_i64x2::<0x88>(u1, u5));
        dst[3] = _mm512_xor_si512(dst[3], _mm512_shuffle_i64x2::<0x88>(u3, u7));
        dst[4] = _mm512_xor_si512(dst[4], _mm512_shuffle_i64x2::<0xdd>(u0, u4));
        dst[5] = _mm512_xor_si512(dst[5], _mm512_shuffle_i64x2::<0xdd>(u2, u6));
        dst[6] = _mm512_xor_si512(dst[6], _mm512_shuffle_i64x2::<0xdd>(u1, u5));
        dst[7] = _mm512_xor_si512(dst[7], _mm512_shuffle_i64x2::<0xdd>(u3, u7));
    }
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use core::arch::aarch64::*;

    use super::ROUND_CONSTANTS;

    /// Plain-NEON 2-way permutation.
    ///
    /// The Rho/Pi step is fully unrolled with constant lane indices and
    /// immediate shifts. Keeping it as a loop over the `PI`/`RHO` tables makes
    /// the state live in memory and turns every rotation into a variable shift,
    /// which costs about twice as much as this form.
    ///
    /// # Safety
    /// Caller must guarantee that `lanes` is 16-byte aligned (guaranteed by
    /// `KeccakX`'s `repr(align(64))`) and exactly 50 words.
    #[inline]
    pub(super) unsafe fn permute<const ROUNDS: usize>(lanes: &mut [u64; 50]) {
        // SAFETY: invariant documented on the function.
        let state: &mut [uint64x2_t; 25] = unsafe { &mut *(lanes.as_mut_ptr().cast()) };
        keccak_p1600_x2::<ROUNDS>(state);
    }

    macro_rules! rotl {
        ($a:expr, $n:literal) => {
            vorrq_u64(vshlq_n_u64::<$n>($a), vshrq_n_u64::<{ 64 - $n }>($a))
        };
    }
    macro_rules! xor {
        ($a:expr, $b:expr) => {
            veorq_u64($a, $b)
        };
    }
    macro_rules! andnot {
        // `vbicq(a, b)` is `a & !b`, i.e. the same operand order as
        // `_mm256_andnot_si256(a, b)` flipped: chi needs `!b & c`.
        ($a:expr, $b:expr) => {
            vbicq_u64($b, $a)
        };
    }
    macro_rules! xor4 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
            veorq_u64(veorq_u64(veorq_u64($a, $b), veorq_u64($c, $d)), $e)
        };
    }

    /// # Safety
    /// NEON is baseline on aarch64.
    unsafe fn keccak_p1600_x2<const ROUNDS: usize>(st: &mut [uint64x2_t; 25]) {
        let (mut s0, mut s1, mut s2, mut s3, mut s4) = (st[0], st[1], st[2], st[3], st[4]);
        let (mut s5, mut s6, mut s7, mut s8, mut s9) = (st[5], st[6], st[7], st[8], st[9]);
        let (mut s10, mut s11, mut s12, mut s13, mut s14) = (st[10], st[11], st[12], st[13], st[14]);
        let (mut s15, mut s16, mut s17, mut s18, mut s19) = (st[15], st[16], st[17], st[18], st[19]);
        let (mut s20, mut s21, mut s22, mut s23, mut s24) = (st[20], st[21], st[22], st[23], st[24]);

        for &rc in ROUND_CONSTANTS[24 - ROUNDS..].iter() {
            let rc = vdupq_n_u64(rc);

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

    /// NEON counterpart of [`super::avx512::xor_lanes`]: interleaves the two
    /// messages into the two state lanes.
    ///
    /// # Safety
    /// `dst` must be 16-byte aligned and hold at least 4 words, and every
    /// `msgs[w]` must have at least `off + 16` bytes.
    #[inline]
    pub(super) unsafe fn xor_lanes(dst: &mut [u64], msgs: &[&[u8]; 2], off: usize) {
        // SAFETY: `dst` is aligned and long enough, as documented above.
        let dst: &mut [uint64x2_t; 2] = unsafe { &mut *(dst.as_mut_ptr().cast()) };
        // SAFETY: every message has at least `off + 16` bytes.
        let a = unsafe { vld1q_u64(msgs[0].as_ptr().add(off).cast()) };
        let b = unsafe { vld1q_u64(msgs[1].as_ptr().add(off).cast()) };

        dst[0] = veorq_u64(dst[0], vzip1q_u64(a, b));
        dst[1] = veorq_u64(dst[1], vzip2q_u64(a, b));
    }
}

#[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
mod sha3_ext {
    use core::arch::aarch64::uint64x2_t;

    /// Armv8.4-A SHA-3 (FEAT_SHA3) 2-way permutation.
    ///
    /// The round is shared with the single-state permutation used by the
    /// SHA-3/SHAKE sponges; see [`crate::sha3::keccak_arm64::permute_lanes`].
    ///
    /// # Safety
    /// The caller must have verified that the CPU implements FEAT_SHA3, and
    /// `lanes` must be 16-byte aligned and exactly 400 bytes.
    #[inline]
    pub(super) unsafe fn permute<const ROUNDS: usize>(lanes: &mut [u64; 50]) {
        // SAFETY: invariant documented on the function.
        let state: &mut [uint64x2_t; 25] = unsafe { &mut *(lanes.as_mut_ptr().cast()) };
        // SAFETY: the backend was detected at construction.
        unsafe { crate::sha3::keccak_arm64::permute_lanes::<ROUNDS>(state) };
    }
}

/// A `W`-way sponge: `W` independent messages absorbed and squeezed in lockstep.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct SpongeX<const W: usize, const ROUNDS: usize> {
    state: KeccakX<W, ROUNDS>,
    rate: usize,
    pad: u8,
    pos: usize,
    backend: Backend,
    squeezing: bool,
}

impl<const W: usize, const ROUNDS: usize> SpongeX<W, ROUNDS> {
    #[inline]
    pub(crate) const fn new(rate: usize, pad: u8, backend: Backend) -> Self {
        assert!(
            rate > 0 && rate < 200 && rate.is_multiple_of(8),
            "the batched Keccak rate must be a positive multiple of 8 bytes below 200"
        );
        return SpongeX {
            state: KeccakX::zero(),
            rate,
            pad,
            pos: 0,
            backend,
            squeezing: false,
        };
    }

    /// Clears all states and returns to the absorbing phase.
    #[inline]
    pub(crate) fn reset(&mut self) {
        // Only the first `25 * W` lanes are in use (the array is sized for
        // `MAX_BATCH` so the type does not depend on an arithmetic expression).
        self.state.lanes[..25 * W].fill(0);
        self.pos = 0;
        self.squeezing = false;
    }

    /// Absorbs `W` equal-length messages, one per state.
    ///
    /// # Panics
    /// Panics in debug builds if the messages differ in length.
    pub(crate) fn absorb_batch(&mut self, msgs: &[&[u8]; W]) {
        debug_assert!(!self.squeezing, "absorb_batch cannot be called after squeezing");
        let len = msgs[0].len();
        debug_assert!(msgs.iter().all(|m| m.len() == len));
        let mut off = 0;

        while off < len {
            let take = min(self.rate - self.pos, len - off);
            let chunk: [&[u8]; W] = core::array::from_fn(|w| &msgs[w][off..off + take]);
            self.xor_batch(&chunk);
            self.pos += take;
            off += take;

            if self.pos == self.rate {
                self.state.permute(self.backend);
                self.pos = 0;
            }
        }
    }

    fn xor_batch(&mut self, msgs: &[&[u8]; W]) {
        let len = msgs[0].len();
        let mut i = 0;

        // Head bytes until the sponge offset is word-aligned.
        while i < len && !(self.pos + i).is_multiple_of(8) {
            self.xor_byte(self.pos + i, msgs, i);
            i += 1;
        }

        // Whole groups: `W` lanes of every message in one vector op, when the
        // backend has a transpose for this width.
        while i + W * 8 <= len {
            let lane = (self.pos + i) / 8;
            if !self.xor_wide(lane, msgs, i) {
                break;
            }
            i += W * 8;
        }

        // Whole 64-bit lanes.
        while i + 8 <= len {
            let lane = (self.pos + i) / 8;
            for w in 0..W {
                let word = u64::from_le_bytes(msgs[w][i..i + 8].try_into().unwrap());
                self.state.lanes[W * lane + w] ^= word;
            }
            i += 8;
        }

        // Tail bytes.
        while i < len {
            self.xor_byte(self.pos + i, msgs, i);
            i += 1;
        }
    }

    /// Absorbs `W` whole lanes of each message with a vector transpose.
    ///
    /// Returns `false` when this width has no vector kernel, leaving the work
    /// to the scalar loop in [`Self::xor_batch`].
    ///
    /// The caller guarantees that `off + W * 8 <= msgs[w].len()` and that
    /// `lane + W` lanes fit in the rate, so the slice below is in bounds.
    #[inline]
    #[allow(unused_variables)]
    fn xor_wide(&mut self, lane: usize, msgs: &[&[u8]; W], off: usize) -> bool {
        let dst = &mut self.state.lanes[W * lane..];

        #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
        if W == 8 && self.backend == Backend::Avx512 {
            // SAFETY: the backend was detected at construction; `dst` holds at
            // least `W * W` words and is 64-byte aligned.
            unsafe { avx512::xor_lanes(dst, msgs.as_slice().try_into().unwrap(), off) };
            return true;
        }

        #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
        if W == 4 && self.backend == Backend::Avx2 {
            // SAFETY: the backend was detected at construction; `dst` holds at
            // least `W * W` words and is 32-byte aligned.
            unsafe { avx2::xor_lanes(dst, msgs.as_slice().try_into().unwrap(), off) };
            return true;
        }

        // NEON is baseline on aarch64, so the bare-NEON and the SHA-3 backends
        // absorb through the same kernel.
        #[cfg(target_arch = "aarch64")]
        if W == 2 {
            // SAFETY: `dst` holds at least `W * W` words and is 16-byte
            // aligned.
            unsafe { neon::xor_lanes(dst, msgs.as_slice().try_into().unwrap(), off) };
            return true;
        }

        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        if W == 2 && self.backend == Backend::WasmSimd128 {
            // SAFETY: `dst` holds at least `W * W` words and is 16-byte
            // aligned; the module is only compiled with `simd128`.
            unsafe { super::keccak_wasm32_simd128::xor_lanes(dst, msgs.as_slice().try_into().unwrap(), off) };
            return true;
        }

        false
    }

    #[inline]
    fn xor_byte(&mut self, p: usize, msgs: &[&[u8]; W], i: usize) {
        let lane = p / 8;
        let shift = (p % 8) * 8;
        for w in 0..W {
            self.state.lanes[W * lane + w] ^= (msgs[w][i] as u64) << shift;
        }
    }

    /// Pads, applies the final permutation, and writes `CV_LEN` bytes of each
    /// state into the matching entry of `out`.
    ///
    /// `CV_LEN` must be a multiple of eight and must not exceed the rate. Both
    /// tree-hash constructions use 64-byte chaining values with a 136-byte
    /// rate.
    pub(crate) fn squeeze_cvs<const CV_LEN: usize>(&mut self, out: &mut [[u8; CV_LEN]; W]) {
        const {
            assert!(
                CV_LEN.is_multiple_of(8),
                "the chaining-value length must be a multiple of 8 bytes"
            );
        }
        debug_assert!(CV_LEN <= self.rate);
        debug_assert!(self.pos < self.rate);

        self.pad_and_permute();
        self.squeezing = true;
        self.export(out);
        self.pos = CV_LEN;
    }

    /// Pads once and squeezes `LEN` bytes from each state, continuing across
    /// as many rate-sized blocks as necessary.
    ///
    /// This is the general counterpart of [`Self::squeeze_cvs`] for samplers
    /// that need more than one block or an output length that is not a
    /// multiple of eight. It must be called after [`Self::absorb_batch`] and
    /// may be called repeatedly; each call continues the output stream.
    pub(crate) fn squeeze_batch<const LEN: usize>(&mut self, out: &mut [[u8; LEN]; W]) {
        debug_assert!(LEN > 0);

        // Fast path for full-rate blocks, which is what the lattice samplers
        // use in their inner loops. `export` copies whole lanes with fewer
        // bounds checks than the general range exporter.
        if LEN == self.rate && (!self.squeezing || self.pos == self.rate) {
            if self.squeezing {
                self.state.permute(self.backend);
            } else {
                self.pad_and_permute();
                self.squeezing = true;
            }
            self.export(out);
            self.pos = self.rate;
            return;
        }

        if !self.squeezing {
            self.pad_and_permute();
            self.squeezing = true;
        }

        let mut offset = 0usize;
        while offset < LEN {
            if self.pos == self.rate {
                self.state.permute(self.backend);
                self.pos = 0;
            }
            let take = min(self.rate - self.pos, LEN - offset);
            for w in 0..W {
                self.export_range(w, self.pos, &mut out[w][offset..offset + take]);
            }
            self.pos += take;
            offset += take;
        }
    }

    fn pad_and_permute(&mut self) {
        // Pad: domain byte at `pos`, 0x80 at the last byte of the rate.
        let p = self.pos;
        let lane = p / 8;
        let shift = (p % 8) * 8;
        for w in 0..W {
            self.state.lanes[W * lane + w] ^= (self.pad as u64) << shift;
        }

        let last = self.rate - 1;
        let lane = last / 8;
        let shift = (last % 8) * 8;
        for w in 0..W {
            self.state.lanes[W * lane + w] ^= 0x80u64 << shift;
        }

        self.state.permute(self.backend);
        self.pos = 0;
    }

    fn export<const CV_LEN: usize>(&self, out: &mut [[u8; CV_LEN]; W]) {
        for w in 0..W {
            for lane in 0..CV_LEN / 8 {
                out[w][lane * 8..lane * 8 + 8].copy_from_slice(&self.state.lanes[W * lane + w].to_le_bytes());
            }
        }
    }

    fn export_range(&self, w: usize, start: usize, out: &mut [u8]) {
        let mut written = 0usize;
        let mut position = start;
        while written < out.len() {
            let lane = position / 8;
            let lane_offset = position % 8;
            let word = self.state.lanes[W * lane + w].to_le_bytes();
            let take = min(8 - lane_offset, out.len() - written);
            out[written..written + take].copy_from_slice(&word[lane_offset..lane_offset + take]);
            written += take;
            position += take;
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// Runtime-selected leaf width
//////////////////////////////////////////////////////////////////////////////////////////////////

/// The batched leaf sponge, whose width follows the detected [`Backend`].
///
/// Exactly one variant is constructed at run time; the others exist so the
/// compiler can emit the matching kernel. The widest variant determines the
/// size of the owning hasher, which stays on the stack and never allocates.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) enum Leaves<const ROUNDS: usize> {
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    X8(SpongeX<8, ROUNDS>),
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    X4(SpongeX<4, ROUNDS>),
    X2(SpongeX<2, ROUNDS>),
}

impl<const ROUNDS: usize> Leaves<ROUNDS> {
    /// Picks the widest sponge the backend can permute.
    pub(crate) fn new(rate: usize, pad: u8, backend: Backend) -> Self {
        match backend {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Backend::Avx512 => return Leaves::X8(SpongeX::new(rate, pad, backend)),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Backend::Avx2 => return Leaves::X4(SpongeX::new(rate, pad, backend)),
            _ => {}
        }

        Leaves::X2(SpongeX::new(rate, pad, backend))
    }

    /// Number of blocks hashed per permutation.
    #[inline]
    pub(crate) fn width(&self) -> usize {
        match self {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Leaves::X8(_) => 8,
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Leaves::X4(_) => 4,
            Leaves::X2(_) => 2,
        }
    }

    #[inline]
    pub(crate) fn reset(&mut self) {
        match self {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Leaves::X8(sponge) => sponge.reset(),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Leaves::X4(sponge) => sponge.reset(),
            Leaves::X2(sponge) => sponge.reset(),
        }
    }

    pub(crate) fn absorb_batch(&mut self, msgs: &[&[u8]]) {
        debug_assert_eq!(msgs.len(), self.width());

        match self {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Leaves::X8(sponge) => sponge.absorb_batch(msgs.try_into().unwrap()),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Leaves::X4(sponge) => sponge.absorb_batch(msgs.try_into().unwrap()),
            Leaves::X2(sponge) => sponge.absorb_batch(msgs.try_into().unwrap()),
        }
    }

    pub(crate) fn squeeze_cvs<const CV_LEN: usize>(&mut self, out: &mut [[u8; CV_LEN]]) {
        debug_assert_eq!(out.len(), self.width());

        match self {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Leaves::X8(sponge) => sponge.squeeze_cvs::<CV_LEN>((&mut out[..8]).try_into().unwrap()),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Leaves::X4(sponge) => sponge.squeeze_cvs::<CV_LEN>((&mut out[..4]).try_into().unwrap()),
            Leaves::X2(sponge) => sponge.squeeze_cvs::<CV_LEN>((&mut out[..2]).try_into().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sha3::keccak::{ROUND_CONSTANTS, test_util::sponge};

    fn next_u64(state: &mut u64) -> u64 {
        // xorshift64*, adequate for test data.
        *state ^= *state >> 12;
        *state ^= *state << 25;
        *state ^= *state >> 27;
        return state.wrapping_mul(0x2545_f491_4f6c_dd1d);
    }

    fn random_state(rng: &mut u64) -> [u64; 25] {
        let mut out = [0u64; 25];
        for lane in out.iter_mut() {
            *lane = next_u64(rng);
        }
        return out;
    }

    fn random_states<const W: usize>(rng: &mut u64) -> [[u64; 25]; W] {
        core::array::from_fn(|_| random_state(rng))
    }

    fn random_bytes(len: usize, rng: &mut u64) -> std::vec::Vec<u8> {
        let mut out = std::vec![0u8; len];
        for byte in out.iter_mut() {
            *byte = next_u64(rng) as u8;
        }
        return out;
    }

    fn pack<const W: usize, const ROUNDS: usize>(states: &[[u64; 25]; W]) -> KeccakX<W, ROUNDS> {
        let mut x = KeccakX::<W, ROUNDS>::zero();
        for w in 0..W {
            for i in 0..25 {
                x.lanes[W * i + w] = states[w][i];
            }
        }
        return x;
    }

    fn assert_packed<const W: usize, const ROUNDS: usize>(x: &KeccakX<W, ROUNDS>, expected: &[[u64; 25]; W]) {
        for w in 0..W {
            for i in 0..25 {
                assert_eq!(x.lanes[W * i + w], expected[w][i], "state {w} lane {i}");
            }
        }
    }

    fn expected_permuted<const W: usize, const ROUNDS: usize>(states: &[[u64; 25]; W]) -> [[u64; 25]; W] {
        let mut expected = *states;
        for state in expected.iter_mut() {
            // Use the portable implementation so the oracle cannot be poisoned
            // by a broken single-state backend on the machine running the tests.
            crate::sha3::keccak::p1600_soft::<ROUNDS>(state);
        }
        return expected;
    }

    fn check_scalar<const W: usize, const ROUNDS: usize>(seed: u64) {
        let mut rng = seed;
        for _ in 0..64 {
            let states = random_states::<W>(&mut rng);
            let expected = expected_permuted::<W, ROUNDS>(&states);
            let mut x = pack::<W, ROUNDS>(&states);
            scalar::permute::<W, ROUNDS>(&mut x.lanes);
            assert_packed(&x, &expected);
        }
    }

    #[test]
    fn scalar_permutation_matches_reference() {
        check_scalar::<2, 12>(0x243f_6a88_85a3_08d3);
        check_scalar::<4, 12>(0x1319_8a2e_0370_7344);
        check_scalar::<2, 16>(0x4528_21e6_38d0_1377);
        check_scalar::<4, 16>(0xbe54_66cf_34e9_0c6c);
        check_scalar::<2, 24>(0xa409_3822_299f_31d0);
        check_scalar::<4, 24>(0x082e_fa98_ec4e_6c89);
    }

    fn check_dispatched<const W: usize, const ROUNDS: usize>(seed: u64, backend: Backend) {
        let mut rng = seed;
        for _ in 0..64 {
            let states = random_states::<W>(&mut rng);
            let expected = expected_permuted::<W, ROUNDS>(&states);
            let mut x = pack::<W, ROUNDS>(&states);
            x.permute(backend);
            assert_packed(&x, &expected);
        }
    }

    #[test]
    fn dispatched_permutation_matches_reference() {
        let backend = Backend::detect();
        check_dispatched::<2, 12>(0x243f_6a88_85a3_08d3, backend);
        check_dispatched::<4, 12>(0x1319_8a2e_0370_7344, backend);
        check_dispatched::<2, 16>(0x4528_21e6_38d0_1377, backend);
        check_dispatched::<4, 16>(0xbe54_66cf_34e9_0c6c, backend);
        check_dispatched::<2, 24>(0xa409_3822_299f_31d0, backend);
        check_dispatched::<4, 24>(0x082e_fa98_ec4e_6c89, backend);

        #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
        if avx512_available() {
            check_dispatched::<8, 12>(0x5d2c_9f81_7a63_e40b, Backend::Avx512);
            check_dispatched::<8, 16>(0x5d2c_9f81_7a63_e40b, Backend::Avx512);
            check_dispatched::<8, 24>(0x5d2c_9f81_7a63_e40b, Backend::Avx512);
        }
    }

    /// Whether the AVX-512 kernel may be executed on this CPU.
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    fn avx512_available() -> bool {
        #[cfg(feature = "std")]
        {
            return std::arch::is_x86_feature_detected!("avx512f");
        }
        #[cfg(all(not(feature = "std"), target_feature = "avx512f"))]
        {
            return true;
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn neon_permutation_matches_reference() {
        let mut rng = 0xa409_3822_299f_31d0u64;
        for _ in 0..64 {
            let states = random_states::<2>(&mut rng);
            let expected = expected_permuted::<2, 12>(&states);
            let mut x = pack::<2, 12>(&states);
            let state: &mut [u64; 50] = (&mut x.lanes[..50]).try_into().unwrap();
            // SAFETY: NEON is baseline on aarch64.
            unsafe { neon::permute::<12>(state) };
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<2>(&mut rng);
            let expected = expected_permuted::<2, 16>(&states);
            let mut x = pack::<2, 16>(&states);
            let state: &mut [u64; 50] = (&mut x.lanes[..50]).try_into().unwrap();
            // SAFETY: NEON is baseline on aarch64.
            unsafe { neon::permute::<16>(state) };
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<2>(&mut rng);
            let expected = expected_permuted::<2, 24>(&states);
            let mut x = pack::<2, 24>(&states);
            let state: &mut [u64; 50] = (&mut x.lanes[..50]).try_into().unwrap();
            // SAFETY: NEON is baseline on aarch64.
            unsafe { neon::permute::<24>(state) };
            assert_packed(&x, &expected);
        }
    }

    #[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
    #[test]
    fn sha3_permutation_matches_reference() {
        if !std::arch::is_aarch64_feature_detected!("sha3") {
            return;
        }

        let mut rng = 0x082e_fa98_ec4e_6c89u64;
        for _ in 0..64 {
            let states = random_states::<2>(&mut rng);
            let expected = expected_permuted::<2, 12>(&states);
            let mut x = pack::<2, 12>(&states);
            let state: &mut [u64; 50] = (&mut x.lanes[..50]).try_into().unwrap();
            // SAFETY: the SHA-3 extension was detected above.
            unsafe { sha3_ext::permute::<12>(state) };
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<2>(&mut rng);
            let expected = expected_permuted::<2, 16>(&states);
            let mut x = pack::<2, 16>(&states);
            let state: &mut [u64; 50] = (&mut x.lanes[..50]).try_into().unwrap();
            // SAFETY: the SHA-3 extension was detected above.
            unsafe { sha3_ext::permute::<16>(state) };
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<2>(&mut rng);
            let expected = expected_permuted::<2, 24>(&states);
            let mut x = pack::<2, 24>(&states);
            let state: &mut [u64; 50] = (&mut x.lanes[..50]).try_into().unwrap();
            // SAFETY: the SHA-3 extension was detected above.
            unsafe { sha3_ext::permute::<24>(state) };
            assert_packed(&x, &expected);
        }
    }

    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    #[test]
    fn avx2_permutation_matches_reference() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }
        println!("AVX2 kernel test running");

        let mut rng = 0x8e4a_1c37_5d09_b2f6u64;
        for _ in 0..64 {
            let states = random_states::<4>(&mut rng);
            let expected = expected_permuted::<4, 12>(&states);
            let mut x = pack::<4, 12>(&states);
            // SAFETY: AVX2 was detected above, and `KeccakX<4, 12>` is 64-byte
            // aligned with at least `MAX_LANES` lanes.
            unsafe { avx2::permute::<12>(&mut x.lanes) };
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<4>(&mut rng);
            let expected = expected_permuted::<4, 16>(&states);
            let mut x = pack::<4, 16>(&states);
            // SAFETY: AVX2 was detected above, and `KeccakX<4, 16>` is 64-byte
            // aligned with at least `MAX_LANES` lanes.
            unsafe { avx2::permute::<16>(&mut x.lanes) };
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<4>(&mut rng);
            let expected = expected_permuted::<4, 24>(&states);
            let mut x = pack::<4, 24>(&states);
            // SAFETY: AVX2 was detected above, and `KeccakX<4, 24>` is 64-byte
            // aligned with at least `MAX_LANES` lanes.
            unsafe { avx2::permute::<24>(&mut x.lanes) };
            assert_packed(&x, &expected);
        }
    }

    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    #[test]
    fn avx512_permutation_matches_reference() {
        if !avx512_available() {
            return;
        }
        println!("AVX-512 kernel test running");

        let mut rng = 0x5d2c_9f81_7a63_e40bu64;
        for _ in 0..64 {
            let states = random_states::<8>(&mut rng);
            let expected = expected_permuted::<8, 12>(&states);
            let mut x = pack::<8, 12>(&states);
            // SAFETY: AVX-512F was detected above, and `KeccakX<8, 12>` is
            // 64-byte aligned with exactly `MAX_LANES` lanes.
            unsafe { avx512::permute::<12>(&mut x.lanes) };
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<8>(&mut rng);
            let expected = expected_permuted::<8, 16>(&states);
            let mut x = pack::<8, 16>(&states);
            // SAFETY: AVX-512F was detected above, and `KeccakX<8, 16>` is
            // 64-byte aligned with exactly `MAX_LANES` lanes.
            unsafe { avx512::permute::<16>(&mut x.lanes) };
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<8>(&mut rng);
            let expected = expected_permuted::<8, 24>(&states);
            let mut x = pack::<8, 24>(&states);
            // SAFETY: AVX-512F was detected above, and `KeccakX<8, 24>` is
            // 64-byte aligned with exactly `MAX_LANES` lanes.
            unsafe { avx512::permute::<24>(&mut x.lanes) };
            assert_packed(&x, &expected);
        }
    }

    /// Directly checks the AVX-512 8x8 transpose that turns eight consecutive
    /// 64-byte message slices into eight interleaved state lanes. This isolates
    /// the transpose from the permutation and from finalization.
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    #[test]
    fn avx512_xor_lanes_matches_scalar_transpose() {
        if !avx512_available() {
            return;
        }

        let mut rng = 0x9e37_79b9_7f4a_7c15u64;
        let msgs: [std::vec::Vec<u8>; 8] = core::array::from_fn(|_| random_bytes(192, &mut rng));
        let refs: [&[u8]; 8] = core::array::from_fn(|w| &msgs[w][..]);

        for &off in &[0usize, 8, 64] {
            let mut x = KeccakX::<8, 12>::zero();
            for value in &mut x.lanes[..64] {
                *value = next_u64(&mut rng);
            }
            let before = x.lanes;

            // SAFETY: AVX-512F was detected above, `x` is 64-byte aligned and
            // holds `MAX_LANES` words.
            unsafe { avx512::xor_lanes(&mut x.lanes, &refs, off) };

            for lane in 0..8 {
                for w in 0..8 {
                    let bytes = &msgs[w][off + lane * 8..off + lane * 8 + 8];
                    let word = u64::from_le_bytes(bytes.try_into().unwrap());
                    assert_eq!(
                        x.lanes[8 * lane + w],
                        before[8 * lane + w] ^ word,
                        "off {off}, lane {lane}, state {w}"
                    );
                }
            }

            assert_eq!(&x.lanes[64..], &before[64..], "off {off} wrote past the transpose");
        }
    }

    /// Directly checks the AVX2 4x4 transpose used by the batched absorb path.
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    #[test]
    fn avx2_xor_lanes_matches_scalar_transpose() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }

        let mut rng = 0x7f4a_7c15_9e37_79b9u64;
        let msgs: [std::vec::Vec<u8>; 4] = core::array::from_fn(|_| random_bytes(192, &mut rng));
        let refs: [&[u8]; 4] = core::array::from_fn(|w| &msgs[w][..]);

        for &off in &[0usize, 8, 32] {
            let mut x = KeccakX::<4, 12>::zero();
            for value in &mut x.lanes[..32] {
                *value = next_u64(&mut rng);
            }
            let before = x.lanes;

            // SAFETY: AVX2 was detected above, `x` is 64-byte aligned and
            // holds `MAX_LANES` words.
            unsafe { avx2::xor_lanes(&mut x.lanes, &refs, off) };

            for lane in 0..4 {
                for w in 0..4 {
                    let bytes = &msgs[w][off + lane * 8..off + lane * 8 + 8];
                    let word = u64::from_le_bytes(bytes.try_into().unwrap());
                    assert_eq!(
                        x.lanes[4 * lane + w],
                        before[4 * lane + w] ^ word,
                        "off {off}, lane {lane}, state {w}"
                    );
                }
            }

            assert_eq!(&x.lanes[32..], &before[32..], "off {off} wrote past the transpose");
        }
    }

    /// Directly checks the NEON 2x2 transpose used by the batched absorb path.
    #[cfg(target_arch = "aarch64")]
    #[test]
    fn neon_xor_lanes_matches_scalar_transpose() {
        let mut rng = 0x243f_6a88_85a3_08d3u64;
        let msgs: [std::vec::Vec<u8>; 2] = core::array::from_fn(|_| random_bytes(64, &mut rng));
        let refs: [&[u8]; 2] = core::array::from_fn(|w| &msgs[w][..]);

        for &off in &[0usize, 8] {
            let mut x = KeccakX::<2, 12>::zero();
            for value in &mut x.lanes[..4] {
                *value = next_u64(&mut rng);
            }
            let before = x.lanes;

            // SAFETY: NEON is baseline on aarch64, and `x` is 64-byte aligned.
            unsafe { neon::xor_lanes(&mut x.lanes, &refs, off) };

            for lane in 0..2 {
                for w in 0..2 {
                    let bytes = &msgs[w][off + lane * 8..off + lane * 8 + 8];
                    let word = u64::from_le_bytes(bytes.try_into().unwrap());
                    assert_eq!(
                        x.lanes[2 * lane + w],
                        before[2 * lane + w] ^ word,
                        "off {off}, lane {lane}, state {w}"
                    );
                }
            }

            assert_eq!(&x.lanes[4..], &before[4..], "off {off} wrote past the transpose");
        }
    }

    /// The wasm `simd128` backend must agree with two independent portable
    /// permutations, including the four-wide case, which has no wasm kernel and
    /// must fall back to scalar.
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[test]
    fn wasm_simd128_dispatched_permutation_matches_reference() {
        check_dispatched::<2, 12>(0x243f_6a88_85a3_08d3, Backend::WasmSimd128);
        check_dispatched::<2, 24>(0xa409_3822_299f_31d0, Backend::WasmSimd128);
        check_dispatched::<4, 12>(0x1319_8a2e_0370_7344, Backend::WasmSimd128);
        check_dispatched::<4, 24>(0x082e_fa98_ec4e_6c89, Backend::WasmSimd128);
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[test]
    fn wasm_simd128_backend_name() {
        assert_eq!(Backend::WasmSimd128.name(), "wasm-simd128");
        assert_eq!(Backend::detect(), Backend::WasmSimd128);
    }

    /// Exercises the wasm permutation and the 2x2 transpose through the
    /// production `SpongeX` path, including chunk sizes that force the byte
    /// head, wide vector body and scalar tail.
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[test]
    fn sponge_x_wasm_simd128_incremental_absorb_matches_reference() {
        check_incremental_absorb::<2, 24>(136, 0x1f, Backend::WasmSimd128);
        check_incremental_absorb::<2, 12>(136, 0x1f, Backend::WasmSimd128);
        check_incremental_absorb::<2, 24>(168, 0x1f, Backend::WasmSimd128);
    }

    /// Drives the batched absorb path at every starting offset in a word, so
    /// the byte head, the 2x2 wide transpose and the scalar tail are all
    /// exercised for each alignment and each rate.
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[test]
    fn sponge_x_wasm_simd128_absorb_alignment_offsets_match_reference() {
        const LEN: usize = 400;
        let mut rng = 0x3c6e_f372_fe94_f82bu64;
        let msgs: [std::vec::Vec<u8>; 2] = core::array::from_fn(|_| random_bytes(LEN, &mut rng));

        macro_rules! check {
            ($rounds:literal, $rate:literal) => {{
                let mut expected = [[0u8; 64]; 2];
                for w in 0..2 {
                    sponge::<$rounds>(&msgs[w], $rate, 0x1f, &mut expected[w]);
                }

                for prefix in 0..=17usize {
                    let mut sponge = SpongeX::<2, $rounds>::new($rate, 0x1f, Backend::WasmSimd128);
                    sponge.absorb_batch(&[&msgs[0][..prefix], &msgs[1][..prefix]]);
                    sponge.absorb_batch(&[&msgs[0][prefix..], &msgs[1][prefix..]]);
                    let mut cvs = [[0u8; 64]; 2];
                    sponge.squeeze_cvs::<64>(&mut cvs);
                    for w in 0..2 {
                        assert_eq!(
                            cvs[w], expected[w],
                            "rounds={}, rate={}, prefix={prefix}, state={w}",
                            $rounds, $rate
                        );
                    }
                }
            }};
        }

        check!(24, 136);
        check!(12, 136);
        check!(16, 136);
        check!(24, 168);
    }

    /// The scalar fallback must remain bit-exact when `simd128` is compiled in
    /// but a caller explicitly asks for it.
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[test]
    fn sponge_x_wasm_scalar_fallback_matches_reference() {
        check_incremental_absorb::<2, 24>(136, 0x1f, Backend::Scalar);
        check_incremental_absorb::<2, 12>(136, 0x1f, Backend::Scalar);
        check_incremental_absorb::<2, 16>(136, 0x1f, Backend::Scalar);
    }

    fn check_batch<const W: usize, const ROUNDS: usize>(
        msgs: &[std::vec::Vec<u8>],
        expected: &[std::vec::Vec<u8>],
        backend: Backend,
        len: usize,
    ) {
        let refs: [&[u8]; W] = core::array::from_fn(|w| &msgs[w][..]);
        let mut sponge = SpongeX::<W, ROUNDS>::new(136, 0x1f, backend);
        let mut cvs = [[0u8; 64]; W];
        sponge.absorb_batch(&refs);
        sponge.squeeze_cvs::<64>(&mut cvs);
        for w in 0..W {
            assert_eq!(cvs[w][..], expected[w][..], "rounds {ROUNDS}, width {W}, len {len}, lane {w}");
        }
    }

    fn check_batched_squeeze<const LEN: usize>(rate: usize, pad: u8, seed: u64) {
        let mut rng = seed;
        let msgs: [std::vec::Vec<u8>; 2] = core::array::from_fn(|_| random_bytes(34, &mut rng));
        let refs: [&[u8]; 2] = core::array::from_fn(|w| &msgs[w][..]);

        let expected: [std::vec::Vec<u8>; 2] = core::array::from_fn(|w| {
            let mut out = std::vec![0u8; LEN];
            sponge::<24>(&msgs[w], rate, pad, &mut out);
            out
        });

        let mut sponge = SpongeX::<2, 24>::new(rate, pad, Backend::detect());
        sponge.absorb_batch(&refs);
        let mut out = [[0u8; LEN]; 2];
        sponge.squeeze_batch::<LEN>(&mut out);
        for w in 0..2 {
            assert_eq!(out[w][..], expected[w][..], "rate={rate}, LEN={LEN}, state={w}");
        }
    }

    #[test]
    fn batched_squeeze_matches_single_xof() {
        macro_rules! check_lengths {
            ($rate:literal, $pad:literal) => {
                check_batched_squeeze::<1>($rate, $pad, 0x1234);
                check_batched_squeeze::<7>($rate, $pad, 0x1235);
                check_batched_squeeze::<8>($rate, $pad, 0x1236);
                check_batched_squeeze::<127>($rate, $pad, 0x1237);
                check_batched_squeeze::<128>($rate, $pad, 0x1238);
                check_batched_squeeze::<135>($rate, $pad, 0x1239);
                check_batched_squeeze::<136>($rate, $pad, 0x123a);
                check_batched_squeeze::<167>($rate, $pad, 0x123b);
                check_batched_squeeze::<168>($rate, $pad, 0x123c);
                check_batched_squeeze::<169>($rate, $pad, 0x123d);
                check_batched_squeeze::<576>($rate, $pad, 0x123e);
                check_batched_squeeze::<640>($rate, $pad, 0x123f);
                check_batched_squeeze::<1000>($rate, $pad, 0x1240);
            };
        }

        check_lengths!(168, 0x1f);
        check_lengths!(136, 0x1f);
    }

    #[test]
    fn batched_squeeze_continues_across_calls() {
        let mut rng = 0x91e1_0c3d_7f5a_2b48u64;
        let msgs: [std::vec::Vec<u8>; 2] = core::array::from_fn(|_| random_bytes(34, &mut rng));
        let refs: [&[u8]; 2] = core::array::from_fn(|w| &msgs[w][..]);
        let expected: [std::vec::Vec<u8>; 2] = core::array::from_fn(|w| {
            let mut out = std::vec![0u8; 999];
            sponge::<24>(&msgs[w], 168, 0x1f, &mut out);
            out
        });

        let mut sponge = SpongeX::<2, 24>::new(168, 0x1f, Backend::detect());
        sponge.absorb_batch(&refs);
        let mut combined = [[0u8; 999]; 2];
        for offset in (0..999).step_by(37) {
            let mut part = [[0u8; 37]; 2];
            sponge.squeeze_batch::<37>(&mut part);
            for w in 0..2 {
                combined[w][offset..offset + 37].copy_from_slice(&part[w]);
            }
        }
        for w in 0..2 {
            assert_eq!(combined[w][..], expected[w][..], "state={w}");
        }
    }

    fn check_incremental_absorb<const W: usize, const ROUNDS: usize>(rate: usize, pad: u8, backend: Backend) {
        const LEN: usize = 1000;
        let mut rng = 0x3c6e_f372_fe94_f82bu64;
        let msgs: [std::vec::Vec<u8>; W] = core::array::from_fn(|_| random_bytes(LEN, &mut rng));

        let mut expected = [[0u8; 64]; W];
        for w in 0..W {
            sponge::<ROUNDS>(&msgs[w], rate, pad, &mut expected[w]);
        }

        for &chunk in &[1usize, 7, 135, 136, 137, 500, LEN] {
            let mut sponge = SpongeX::<W, ROUNDS>::new(rate, pad, backend);
            for offset in (0..LEN).step_by(chunk) {
                let end = core::cmp::min(offset + chunk, LEN);
                let parts: [&[u8]; W] = core::array::from_fn(|w| &msgs[w][offset..end]);
                sponge.absorb_batch(&parts);
            }
            let mut cvs = [[0u8; 64]; W];
            sponge.squeeze_cvs::<64>(&mut cvs);
            for w in 0..W {
                assert_eq!(cvs[w], expected[w], "rate={rate}, chunk={chunk}, width={W}, state={w}");
            }
        }
    }

    #[test]
    #[should_panic(expected = "the batched Keccak rate must be a positive multiple of 8 bytes below 200")]
    fn sponge_x_rejects_non_word_aligned_rate() {
        let _ = SpongeX::<2, 24>::new(135, 0x1f, Backend::Scalar);
    }

    #[test]
    fn sponge_x_incremental_absorb_matches_reference() {
        check_incremental_absorb::<2, 24>(136, 0x1f, Backend::detect());
        check_incremental_absorb::<2, 12>(136, 0x1f, Backend::detect());
        check_incremental_absorb::<2, 16>(136, 0x1f, Backend::detect());
        check_incremental_absorb::<2, 24>(168, 0x1f, Backend::detect());
    }

    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    #[test]
    fn sponge_x_avx2_incremental_absorb_matches_reference() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }
        check_incremental_absorb::<4, 24>(136, 0x1f, Backend::Avx2);
        check_incremental_absorb::<4, 12>(136, 0x1f, Backend::Avx2);
        check_incremental_absorb::<4, 16>(136, 0x1f, Backend::Avx2);
    }

    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    #[test]
    fn sponge_x_avx512_incremental_absorb_matches_reference() {
        if !std::arch::is_x86_feature_detected!("avx512f") {
            return;
        }
        check_incremental_absorb::<8, 24>(136, 0x1f, Backend::Avx512);
        check_incremental_absorb::<8, 12>(136, 0x1f, Backend::Avx512);
        check_incremental_absorb::<8, 16>(136, 0x1f, Backend::Avx512);
    }

    /// A backend wider than the sponge must fall back to the scalar path
    /// without changing the result.
    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    #[test]
    fn sponge_x_falls_back_to_scalar_for_narrow_widths() {
        if std::arch::is_x86_feature_detected!("avx2") {
            check_incremental_absorb::<2, 24>(136, 0x1f, Backend::Avx2);
        }
        if std::arch::is_x86_feature_detected!("avx512f") {
            check_incremental_absorb::<2, 24>(136, 0x1f, Backend::Avx512);
            check_incremental_absorb::<4, 24>(136, 0x1f, Backend::Avx512);
        }
    }

    #[test]
    fn batched_sponge_matches_one_way_hash() {
        let mut rng = 0x4528_21e6_38d0_1377u64;

        for len in [0usize, 1, 7, 8, 135, 136, 137, 168, 169, 272, 1024, 4097, 8192] {
            let base: [std::vec::Vec<u8>; 4] = core::array::from_fn(|_| random_bytes(len, &mut rng));
            let msgs: [std::vec::Vec<u8>; 8] = core::array::from_fn(|w| base[w % 4].clone());

            let expected_shake: [std::vec::Vec<u8>; 8] = core::array::from_fn(|w| {
                let mut out = std::vec![0u8; 64];
                sponge::<24>(&msgs[w], 136, 0x1f, &mut out);
                out
            });
            let expected_turbo: [std::vec::Vec<u8>; 8] = core::array::from_fn(|w| {
                let mut out = std::vec![0u8; 64];
                sponge::<12>(&msgs[w], 136, 0x1f, &mut out);
                out
            });

            // Two-way: the SHA-3 or NEON kernel on aarch64, scalar elsewhere.
            check_batch::<2, 12>(&msgs, &expected_turbo, Backend::detect(), len);
            check_batch::<2, 24>(&msgs, &expected_shake, Backend::detect(), len);

            // Wider batches only exist on `x86_64`.
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            {
                let backend = if std::arch::is_x86_feature_detected!("avx2") {
                    Backend::Avx2
                } else {
                    Backend::Scalar
                };
                check_batch::<4, 12>(&msgs, &expected_turbo, backend, len);
                check_batch::<4, 24>(&msgs, &expected_shake, backend, len);
            }

            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            if avx512_available() {
                check_batch::<8, 12>(&msgs, &expected_turbo, Backend::Avx512, len);
                check_batch::<8, 24>(&msgs, &expected_shake, Backend::Avx512, len);
            }
        }
    }

    /// The 12-round kernels must select the last twelve round constants, not
    /// the first twelve. This pins the same constants as RFC 9861, appendix A.1.
    #[test]
    fn twelve_round_slice_matches_rfc() {
        let rfc: [u64; 12] = [
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
        ];
        assert_eq!(&ROUND_CONSTANTS[24 - 12..], &rfc);
    }
}
