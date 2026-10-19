//! ParallelHash256 (NIST SP 800-185, section 6).
//!
//! ParallelHash splits a message into `B`-byte blocks, hashes every block
//! independently with SHAKE, and feeds the resulting chaining values to a final
//! cSHAKE invocation. The blocks are independent, so this module hashes several
//! blocks at a time with a batched Keccak-p[1600] core that keeps one state per
//! SIMD lane. The batch width follows the widest backend the CPU supports:
//!
//! | Target        | Backend              | Blocks per permutation |
//! |---------------|----------------------|------------------------|
//! | `x86_64`      | AVX-512F (runtime)   | 8                      |
//! | `x86_64`      | AVX2 (runtime)       | 4                      |
//! | `aarch64`     | Armv8.4-A SHA-3      | 2                      |
//! | `aarch64`     | NEON                 | 2                      |
//! | anything else | portable scalar      | 2                      |
//!
//! On `x86_64`, runtime detection requires the `std` feature; without it the
//! backend is chosen at compile time from `target_feature`. A backend that is
//! narrower than the requested batch width falls back to the portable code.
//!
//! # Throughput
//!
//! Both the width and the cost of the permutation bound this module: it
//! absorbs 136 bytes per permutation (the SHAKE256 rate), so with a healthy
//! backend the leaf phase runs at roughly `width` times the single-stream
//! SHAKE256 throughput, minus the per-block finalization.
//!
//! That ceiling is worth keeping in mind when comparing against other hashes:
//! BLAKE3 compresses 64 bytes with 7 rounds and runs 16 chunks in parallel on
//! AVX-512, while Keccak-p[1600] needs 24 rounds for 136 bytes. A parallel
//! SHAKE construction is therefore expected to trail BLAKE3 on wide SIMD
//! machines even when both are well implemented; the useful reference is
//! "a multiple of SHAKE256", not parity with BLAKE3.
//!
//! # Allocation-free, no staging buffer
//!
//! The batched path consumes whole blocks straight from the
//! caller's slice; there is no `N * B` staging buffer. A single [`update`]
//! call whose input spans at least two whole blocks hashes them in one or more
//! batched permutations: a short final batch reuses a real block for its unused
//! lanes and discards their chaining values, so no scratch buffer is needed. A
//! lone whole block and the final partial block are hashed with the ordinary
//! one-way SHAKE. This keeps the type small (a fixed-size Keccak state,
//! regardless of `B`) and makes the whole module usable without `alloc`, at the
//! cost of falling back to sequential hashing when the caller feeds small
//! chunks.
//!
//! The permutation backend (AVX-512, AVX2, Armv8.4-A SHA-3 or plain NEON) is
//! detected once, when the hasher is created; [`backend_name`] reports which
//! one this CPU uses.
//!
//! # Block size
//!
//! `B` is a protocol parameter, not fixed by NIST: two implementations only
//! interoperate if they agree on it. It must be greater than zero. The default
//! used by XKCP and BouncyCastle is 8192 bytes; a smaller value such as 1024
//! exposes parallelism for shorter messages but adds one finalization
//! permutation and one chaining value per block.
//!
//! [`update`]: ParallelHash256::update

#![allow(unsafe_op_in_unsafe_fn)]

use core::cmp::min;

#[cfg(target_arch = "aarch64")]
use super::keccak::ROUND_CONSTANTS;
use super::{
    keccak::KeccakSponge,
    shake256::{Shake256, left_encode, right_encode},
};
use crate::Xof;

/// Rate of SHAKE256/cSHAKE256, in bytes.
const PH256_RATE: usize = 136;
/// Chaining-value length of ParallelHash256, in bytes (512 bits).
const PH256_CV_LEN: usize = 64;
/// Largest Keccak rate the final cSHAKE node can need.
///
/// Only ParallelHash256 (rate 136) is implemented, but `cshake_node` stays
/// generic so the SHAKE128 rate (168) still fits if it is added back.
const MAX_RATE: usize = 168;
/// Domain-separation byte of SHAKE (`cSHAKE` with empty name and customization).
const SHAKE_PAD: u8 = 0x1f;
/// Domain-separation byte of cSHAKE.
const CSHAKE_PAD: u8 = 0x04;
/// Fixed function name absorbed by the final cSHAKE node.
const PARALLEL_HASH_NAME: &[u8] = b"ParallelHash";

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
    fn detect() -> Backend {
        #[cfg(all(target_arch = "x86_64", feature = "std"))]
        {
            #[cfg(target_feature = "avx512f")]
            {
                return Backend::Avx512;
            }

            #[cfg(target_feature = "avx2")]
            {
                return Backend::Avx2;
            }

            if std::arch::is_x86_feature_detected!("avx512f") {
                return Backend::Avx512;
            }
            if std::arch::is_x86_feature_detected!("avx2") {
                return Backend::Avx2;
            }
        }

        #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx512f"))]
        {
            return Backend::Avx512;
        }

        #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx2"))]
        {
            return Backend::Avx2;
        }

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

        Backend::Scalar
    }

    /// Human-readable name of this backend, for diagnostics.
    const fn name(self) -> &'static str {
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
        }
    }
}

/// Returns the name of the batched Keccak backend currently in use.
///
/// Useful in benchmarks and CI logs to tell which code path
/// [`ParallelHash256`] takes. The value is detected once per call; the standard
/// library caches the underlying CPU probe, so this is cheap.
///
/// ```
/// use crypto::sha3::parallel_hash_backend_name;
/// println!("ParallelHash backend: {}", parallel_hash_backend_name());
/// ```
pub fn backend_name() -> &'static str {
    Backend::detect().name()
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
/// four, which is what the `SpongeX<4>` tests instantiate.
#[cfg(target_arch = "x86_64")]
const MAX_BATCH: usize = 8;
#[cfg(not(target_arch = "x86_64"))]
const MAX_BATCH: usize = 4;
/// Number of 64-bit lanes in a `MAX_BATCH`-wide state.
const MAX_LANES: usize = 25 * MAX_BATCH;

/// `W` Keccak-p[1600] states processed in lockstep.
///
/// The state is stored lane-interleaved: `lanes[W * i + w]` is lane `i` of
/// state `w`. With `W = 2` that is exactly what a 128-bit NEON register wants
/// (states 0 and 1 in the low and high half), with `W = 4` a 256-bit AVX2
/// register, and with `W = 8` a 512-bit AVX-512 register. The array is 64-byte
/// aligned so every backend can load it with aligned accesses.
#[repr(align(64))]
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct KeccakX<const W: usize> {
    lanes: [u64; MAX_LANES],
}

impl<const W: usize> KeccakX<W> {
    #[inline]
    const fn zero() -> Self {
        return KeccakX {
            lanes: [0; MAX_LANES],
        };
    }

    /// Apply Keccak-p[1600, 24] to all `W` states using the cached `backend`.
    ///
    /// If `backend` is wider than `W` (for example an eight-wide backend on a
    /// two-wide sponge, which only happens in tests), this falls back to scalar
    /// code for the active states.
    #[inline]
    fn permute(&mut self, backend: Backend) {
        match backend {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Backend::Avx512 => {
                if W == 8 {
                    // SAFETY: AVX-512F was confirmed at construction;
                    // `KeccakX` is 64-byte aligned and the array is
                    // `MAX_LANES` words.
                    unsafe { avx512::permute(&mut self.lanes) };
                    return;
                }
            }
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Backend::Avx2 => {
                if W == 4 {
                    // SAFETY: AVX2 was confirmed at construction; `KeccakX` is
                    // 64-byte aligned and the array is `MAX_LANES` words.
                    unsafe { avx2::permute(&mut self.lanes) };
                    return;
                }
            }
            #[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
            Backend::Sha3 => {
                if W == 2 {
                    let state: &mut [u64; 50] = (&mut self.lanes[..50]).try_into().unwrap();
                    // SAFETY: FEAT_SHA3 was confirmed at construction.
                    unsafe { sha3_ext::permute(state) };
                    return;
                }
            }
            #[cfg(target_arch = "aarch64")]
            Backend::Neon => {
                if W == 2 {
                    let state: &mut [u64; 50] = (&mut self.lanes[..50]).try_into().unwrap();
                    // SAFETY: NEON (ASIMD) is mandatory on AArch64.
                    unsafe { neon::permute(state) };
                    return;
                }
            }
            Backend::Scalar => {}
        }

        scalar::permute::<W>(&mut self.lanes);
    }
}

/// Portable fallback: run the scalar permutation on each state in turn.
mod scalar {
    use crate::sha3::keccak::p1600;

    #[inline]
    pub(super) fn permute<const W: usize>(lanes: &mut [u64]) {
        let mut state = [0u64; 25];
        for w in 0..W {
            for i in 0..25 {
                state[i] = lanes[W * i + w];
            }
            p1600::<24>(&mut state);
            for i in 0..25 {
                lanes[W * i + w] = state[i];
            }
        }
    }
}

/// AVX2 4-way Keccak-p[1600].
///
/// Same structure as the NEON 2-way kernel below, but each `__m256i` holds the
/// same lane of four states. AVX2 has no per-64-bit-lane rotate, so `rotl` uses
/// the variable-shift instructions `vpsllvq`/`vpsrlvq`; a shift count of 64
/// yields zero, which makes `rotl(x, 0) == x` fall out naturally.
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
mod avx2 {
    use core::arch::x86_64::*;

    use super::MAX_LANES;
    use crate::sha3::keccak::{PI, RHO, ROUND_CONSTANTS};

    /// # Safety
    /// The caller must have verified that the CPU implements AVX2, and `state`
    /// must be 32-byte aligned (guaranteed by `KeccakX`'s `repr(align(64))`) and
    /// exactly `MAX_LANES` words.
    #[inline]
    pub(super) unsafe fn permute(state: &mut [u64; MAX_LANES]) {
        // SAFETY: invariant documented on the function.
        let s: &mut [__m256i; 25] = unsafe { &mut *(state.as_mut_ptr().cast()) };
        keccak_p1600_x4(s);
    }

    #[inline(always)]
    unsafe fn rotl(a: __m256i, n: u32) -> __m256i {
        let l = _mm256_sllv_epi64(a, _mm256_set1_epi64x(n as i64));
        let r = _mm256_srlv_epi64(a, _mm256_set1_epi64x((64 - n) as i64));
        return _mm256_or_si256(l, r);
    }

    /// # Safety
    /// AVX2 must be available.
    #[target_feature(enable = "avx2")]
    unsafe fn keccak_p1600_x4(s: &mut [__m256i; 25]) {
        for &rc in ROUND_CONSTANTS.iter() {
            // theta
            let mut c = [_mm256_setzero_si256(); 5];
            for x in 0..5 {
                c[x] = _mm256_xor_si256(
                    _mm256_xor_si256(_mm256_xor_si256(_mm256_xor_si256(s[x], s[5 + x]), s[10 + x]), s[15 + x]),
                    s[20 + x],
                );
            }
            let mut d = [_mm256_setzero_si256(); 5];
            for x in 0..5 {
                d[x] = _mm256_xor_si256(c[(x + 4) % 5], rotl(c[(x + 1) % 5], 1));
            }
            for y in 0..5 {
                for x in 0..5 {
                    s[5 * y + x] = _mm256_xor_si256(s[5 * y + x], d[x]);
                }
            }

            // rho and pi
            let mut last = s[1];
            for i in 0..24 {
                let j = PI[i];
                let tmp = s[j];
                s[j] = rotl(last, RHO[i]);
                last = tmp;
            }

            // chi: `b[x] ^ (!b[x + 1] & b[x + 2])`; `_mm256_andnot_si256(a, b)`
            // is `!a & b`.
            for y in 0..5 {
                let b = [s[5 * y], s[5 * y + 1], s[5 * y + 2], s[5 * y + 3], s[5 * y + 4]];
                s[5 * y] = _mm256_xor_si256(b[0], _mm256_andnot_si256(b[1], b[2]));
                s[5 * y + 1] = _mm256_xor_si256(b[1], _mm256_andnot_si256(b[2], b[3]));
                s[5 * y + 2] = _mm256_xor_si256(b[2], _mm256_andnot_si256(b[3], b[4]));
                s[5 * y + 3] = _mm256_xor_si256(b[3], _mm256_andnot_si256(b[4], b[0]));
                s[5 * y + 4] = _mm256_xor_si256(b[4], _mm256_andnot_si256(b[0], b[1]));
            }

            // iota
            s[0] = _mm256_xor_si256(s[0], _mm256_set1_epi64x(rc as i64));
        }
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
    pub(super) unsafe fn permute(state: &mut [u64; MAX_LANES]) {
        // SAFETY: invariant documented on the function.
        let s: &mut [__m512i; 25] = unsafe { &mut *(state.as_mut_ptr().cast()) };
        keccak_p1600_x8(s);
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
    /// expands the 24 rounds into straight-line code. With a `for` loop over
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
    unsafe fn keccak_p1600_x8(st: &mut [__m512i; 25]) {
        let mut s = *st;
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

        dst[0] = _mm512_xor_si512(dst[0], _mm512_shuffle_i64x2::<0x88>(u0, u4));
        dst[1] = _mm512_xor_si512(dst[1], _mm512_shuffle_i64x2::<0xdd>(u0, u4));
        dst[2] = _mm512_xor_si512(dst[2], _mm512_shuffle_i64x2::<0x88>(u1, u5));
        dst[3] = _mm512_xor_si512(dst[3], _mm512_shuffle_i64x2::<0xdd>(u1, u5));
        dst[4] = _mm512_xor_si512(dst[4], _mm512_shuffle_i64x2::<0x88>(u2, u6));
        dst[5] = _mm512_xor_si512(dst[5], _mm512_shuffle_i64x2::<0xdd>(u2, u6));
        dst[6] = _mm512_xor_si512(dst[6], _mm512_shuffle_i64x2::<0x88>(u3, u7));
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
    pub(super) unsafe fn permute(lanes: &mut [u64; 50]) {
        // SAFETY: invariant documented on the function.
        let state: &mut [uint64x2_t; 25] = unsafe { &mut *(lanes.as_mut_ptr().cast()) };
        keccak_p1600_x2(state);
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
    unsafe fn keccak_p1600_x2(st: &mut [uint64x2_t; 25]) {
        let (mut s0, mut s1, mut s2, mut s3, mut s4) = (st[0], st[1], st[2], st[3], st[4]);
        let (mut s5, mut s6, mut s7, mut s8, mut s9) = (st[5], st[6], st[7], st[8], st[9]);
        let (mut s10, mut s11, mut s12, mut s13, mut s14) = (st[10], st[11], st[12], st[13], st[14]);
        let (mut s15, mut s16, mut s17, mut s18, mut s19) = (st[15], st[16], st[17], st[18], st[19]);
        let (mut s20, mut s21, mut s22, mut s23, mut s24) = (st[20], st[21], st[22], st[23], st[24]);

        for &rc in ROUND_CONSTANTS.iter() {
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
    use core::arch::aarch64::*;

    use super::ROUND_CONSTANTS;

    /// Armv8.4-A SHA-3 (FEAT_SHA3) 2-way permutation.
    ///
    /// # Safety
    /// The caller must have verified that the CPU implements FEAT_SHA3, and
    /// `lanes` must be 16-byte aligned and exactly 400 bytes.
    #[inline]
    pub(super) unsafe fn permute(lanes: &mut [u64; 50]) {
        // SAFETY: invariant documented on the function.
        let state: &mut [uint64x2_t; 25] = unsafe { &mut *(lanes.as_mut_ptr().cast()) };
        keccak_p1600_x2_sha3(state);
    }

    /// This is the round of `keccak_arm64::p1600_armv8`, expressed with the
    /// Rust intrinsics for `EOR3`, `RAX1`, `XAR` and `BCAX`. Those instructions
    /// are 128-bit operations, so they naturally process both batched states.
    #[target_feature(enable = "sha3")]
    unsafe fn keccak_p1600_x2_sha3(s: &mut [uint64x2_t; 25]) {
        for r in 0..24 {
            let rc = ROUND_CONSTANTS[r];

            let mut v0 = s[0];
            let mut v1 = s[1];
            let mut v2 = s[2];
            let mut v3 = s[3];
            let mut v4 = s[4];
            let mut v5 = s[5];
            let mut v6 = s[6];
            let mut v7 = s[7];
            let mut v8 = s[8];
            let mut v9 = s[9];
            let mut v10 = s[10];
            let mut v11 = s[11];
            let mut v12 = s[12];
            let mut v13 = s[13];
            let mut v14 = s[14];
            let mut v15 = s[15];
            let mut v16 = s[16];
            let mut v17 = s[17];
            let mut v18 = s[18];
            let mut v19 = s[19];
            let mut v20 = s[20];
            let mut v21 = s[21];
            let mut v22 = s[22];
            let mut v23 = s[23];
            let mut v24 = s[24];

            // theta
            let mut v25 = veor3q_u64(v20, v15, v10);
            let mut v26 = veor3q_u64(v21, v16, v11);
            let mut v27 = veor3q_u64(v22, v17, v12);
            let mut v28 = veor3q_u64(v23, v18, v13);
            let mut v29 = veor3q_u64(v24, v19, v14);
            v25 = veor3q_u64(v25, v5, v0);
            v26 = veor3q_u64(v26, v6, v1);
            v27 = veor3q_u64(v27, v7, v2);
            v28 = veor3q_u64(v28, v8, v3);
            v29 = veor3q_u64(v29, v9, v4);
            let mut v30 = vrax1q_u64(v25, v27);
            let mut v31 = vrax1q_u64(v26, v28);
            v27 = vrax1q_u64(v27, v29);
            v28 = vrax1q_u64(v28, v25);
            v29 = vrax1q_u64(v29, v26);

            // rho and pi (theta's D is folded into each XAR)
            v0 = veorq_u64(v0, v29);
            v25 = vxarq_u64::<{ 64 - 1 }>(v1, v30);
            v1 = vxarq_u64::<{ 64 - 44 }>(v6, v30);
            v6 = vxarq_u64::<{ 64 - 20 }>(v9, v28);
            v9 = vxarq_u64::<{ 64 - 61 }>(v22, v31);
            v22 = vxarq_u64::<{ 64 - 39 }>(v14, v28);
            v14 = vxarq_u64::<{ 64 - 18 }>(v20, v29);
            v26 = vxarq_u64::<{ 64 - 62 }>(v2, v31);
            v2 = vxarq_u64::<{ 64 - 43 }>(v12, v31);
            v12 = vxarq_u64::<{ 64 - 25 }>(v13, v27);
            v13 = vxarq_u64::<{ 64 - 8 }>(v19, v28);
            v19 = vxarq_u64::<{ 64 - 56 }>(v23, v27);
            v23 = vxarq_u64::<{ 64 - 41 }>(v15, v29);
            v15 = vxarq_u64::<{ 64 - 27 }>(v4, v28);
            v28 = vxarq_u64::<{ 64 - 14 }>(v24, v28);
            v24 = vxarq_u64::<{ 64 - 2 }>(v21, v30);
            v8 = vxarq_u64::<{ 64 - 55 }>(v8, v27);
            v4 = vxarq_u64::<{ 64 - 45 }>(v16, v30);
            v16 = vxarq_u64::<{ 64 - 36 }>(v5, v29);
            v5 = vxarq_u64::<{ 64 - 28 }>(v3, v27);
            v27 = vxarq_u64::<{ 64 - 21 }>(v18, v27);
            v3 = vxarq_u64::<{ 64 - 15 }>(v17, v31);
            v30 = vxarq_u64::<{ 64 - 10 }>(v11, v30);
            v31 = vxarq_u64::<{ 64 - 6 }>(v7, v31);
            v29 = vxarq_u64::<{ 64 - 3 }>(v10, v29);

            // chi
            v20 = vbcaxq_u64(v26, v22, v8);
            v21 = vbcaxq_u64(v8, v23, v22);
            v22 = vbcaxq_u64(v22, v24, v23);
            v23 = vbcaxq_u64(v23, v26, v24);
            v24 = vbcaxq_u64(v24, v8, v26);

            v17 = vbcaxq_u64(v30, v19, v3);
            v18 = vbcaxq_u64(v3, v15, v19);
            v19 = vbcaxq_u64(v19, v16, v15);
            v15 = vbcaxq_u64(v15, v30, v16);
            v16 = vbcaxq_u64(v16, v3, v30);

            v10 = vbcaxq_u64(v25, v12, v31);
            v11 = vbcaxq_u64(v31, v13, v12);
            v12 = vbcaxq_u64(v12, v14, v13);
            v13 = vbcaxq_u64(v13, v25, v14);
            v14 = vbcaxq_u64(v14, v31, v25);

            v7 = vbcaxq_u64(v29, v9, v4);
            v8 = vbcaxq_u64(v4, v5, v9);
            v9 = vbcaxq_u64(v9, v6, v5);
            v5 = vbcaxq_u64(v5, v29, v6);
            v6 = vbcaxq_u64(v6, v4, v29);

            v3 = vbcaxq_u64(v27, v0, v28);
            v4 = vbcaxq_u64(v28, v1, v0);
            v0 = vbcaxq_u64(v0, v2, v1);
            v1 = vbcaxq_u64(v1, v27, v2);
            v2 = vbcaxq_u64(v2, v28, v27);

            // iota
            v0 = veorq_u64(v0, vdupq_n_u64(rc));

            s[0] = v0;
            s[1] = v1;
            s[2] = v2;
            s[3] = v3;
            s[4] = v4;
            s[5] = v5;
            s[6] = v6;
            s[7] = v7;
            s[8] = v8;
            s[9] = v9;
            s[10] = v10;
            s[11] = v11;
            s[12] = v12;
            s[13] = v13;
            s[14] = v14;
            s[15] = v15;
            s[16] = v16;
            s[17] = v17;
            s[18] = v18;
            s[19] = v19;
            s[20] = v20;
            s[21] = v21;
            s[22] = v22;
            s[23] = v23;
            s[24] = v24;
        }
    }
}

/// A `W`-way sponge: `W` independent messages absorbed and squeezed in lockstep.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct SpongeX<const W: usize> {
    state: KeccakX<W>,
    rate: usize,
    pad: u8,
    pos: usize,
    backend: Backend,
}

impl<const W: usize> SpongeX<W> {
    #[inline]
    pub(crate) const fn new(rate: usize, pad: u8, backend: Backend) -> Self {
        return SpongeX {
            state: KeccakX::zero(),
            rate,
            pad,
            pos: 0,
            backend,
        };
    }

    /// Clears all states and returns to the absorbing phase.
    #[inline]
    pub(crate) fn reset(&mut self) {
        // Only the first `25 * W` lanes are in use (the array is sized for
        // `MAX_BATCH` so the type does not depend on an arithmetic expression).
        self.state.lanes[..25 * W].fill(0);
        self.pos = 0;
    }

    /// Absorbs `W` equal-length messages, one per state.
    ///
    /// # Panics
    /// Panics in debug builds if the messages differ in length.
    pub(crate) fn absorb_batch(&mut self, msgs: &[&[u8]; W]) {
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

    /// Pads, applies the final permutation, and writes [`PH256_CV_LEN`] bytes of
    /// each state into the matching entry of `out`.
    ///
    /// The chaining-value length must not exceed the rate, which holds for
    /// ParallelHash256 (64 < 136).
    pub(crate) fn squeeze_cvs(&mut self, out: &mut [[u8; PH256_CV_LEN]; W]) {
        debug_assert!(PH256_CV_LEN <= self.rate);
        debug_assert!(self.pos < self.rate);

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
        self.export(out);
    }

    fn export(&self, out: &mut [[u8; PH256_CV_LEN]; W]) {
        for w in 0..W {
            for lane in 0..PH256_CV_LEN / 8 {
                out[w][lane * 8..lane * 8 + 8].copy_from_slice(&self.state.lanes[W * lane + w].to_le_bytes());
            }
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
/// size of a [`ParallelHash256`], which stays on the stack and never allocates.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
enum Leaves {
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    X8(SpongeX<8>),
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    X4(SpongeX<4>),
    X2(SpongeX<2>),
}

impl Leaves {
    /// Picks the widest sponge the backend can permute.
    fn new(backend: Backend) -> Self {
        match backend {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Backend::Avx512 => return Leaves::X8(SpongeX::new(PH256_RATE, SHAKE_PAD, backend)),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Backend::Avx2 => return Leaves::X4(SpongeX::new(PH256_RATE, SHAKE_PAD, backend)),
            _ => {}
        }

        Leaves::X2(SpongeX::new(PH256_RATE, SHAKE_PAD, backend))
    }

    /// Number of blocks hashed per permutation.
    #[inline]
    fn width(&self) -> usize {
        match self {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Leaves::X8(_) => 8,
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Leaves::X4(_) => 4,
            Leaves::X2(_) => 2,
        }
    }

    #[inline]
    fn reset(&mut self) {
        match self {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Leaves::X8(sponge) => sponge.reset(),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Leaves::X4(sponge) => sponge.reset(),
            Leaves::X2(sponge) => sponge.reset(),
        }
    }

    fn absorb_batch(&mut self, msgs: &[&[u8]]) {
        debug_assert_eq!(msgs.len(), self.width());

        match self {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Leaves::X8(sponge) => sponge.absorb_batch(msgs.try_into().unwrap()),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Leaves::X4(sponge) => sponge.absorb_batch(msgs.try_into().unwrap()),
            Leaves::X2(sponge) => sponge.absorb_batch(msgs.try_into().unwrap()),
        }
    }

    fn squeeze_cvs(&mut self, out: &mut [[u8; PH256_CV_LEN]]) {
        debug_assert_eq!(out.len(), self.width());

        match self {
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Leaves::X8(sponge) => sponge.squeeze_cvs((&mut out[..8]).try_into().unwrap()),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Leaves::X4(sponge) => sponge.squeeze_cvs((&mut out[..4]).try_into().unwrap()),
            Leaves::X2(sponge) => sponge.squeeze_cvs((&mut out[..2]).try_into().unwrap()),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// ParallelHash
//////////////////////////////////////////////////////////////////////////////////////////////////

/// Builds the final cSHAKE node with its `bytepad(encode_string(N) ||
/// encode_string(S), rate)` prefix already absorbed.
fn cshake_node(rate: usize, customization: &[u8]) -> KeccakSponge<24> {
    let mut sponge = KeccakSponge::new(rate, CSHAKE_PAD);

    let enc_w = left_encode(rate);
    let enc_n = left_encode(PARALLEL_HASH_NAME.len() * 8);
    let enc_s = left_encode(customization.len() * 8);

    sponge.absorb(enc_w.as_ref());
    sponge.absorb(enc_n.as_ref());
    sponge.absorb(PARALLEL_HASH_NAME);
    sponge.absorb(enc_s.as_ref());
    sponge.absorb(customization);

    let total = enc_w.len() + enc_n.len() + PARALLEL_HASH_NAME.len() + enc_s.len() + customization.len();
    let pad = (rate - (total % rate)) % rate;
    if pad > 0 {
        let zeros = [0u8; MAX_RATE];
        sponge.absorb(&zeros[..pad]);
    }

    return sponge;
}

/// Reader returned by [`ParallelHash256::finalize_xof`].
///
/// Produces the ParallelHashXOF256 output stream; [`squeeze`] may be called
/// any number of times.
///
/// [`squeeze`]: ParallelHashReader::squeeze
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct ParallelHashReader {
    inner: KeccakSponge<24>,
}

impl ParallelHashReader {
    /// Squeezes the next `out.len()` bytes of the output stream.
    #[inline]
    pub fn squeeze(&mut self, out: &mut [u8]) {
        self.inner.squeeze(out);
    }
}

/// ParallelHash256 (NIST SP 800-185, section 6).
///
/// The block size is a protocol parameter: both sides must use the same
/// value. `block_size` must be greater than zero.
///
/// This type does not allocate and does not keep an `N * B` staging
/// buffer. A single [`update`](Self::update) call spanning at least two whole
/// blocks hashes them in batched permutations (see the module-level
/// documentation); a lone block or a partial block falls back to sequential
/// SHAKE. How many blocks share one permutation depends on the CPU, see
/// [`backend_name`].
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct ParallelHash256 {
    outer: KeccakSponge<24>,
    queue: Shake256,
    queue_len: usize,
    leaves: Leaves,
    block_size: usize,
    blocks: usize,
}

impl ParallelHash256 {
    /// Creates a hasher for the given block size (in bytes) and
    /// customization string.
    ///
    /// # Panics
    /// Panics if `block_size` is zero.
    pub fn new(block_size: usize, customization: &[u8]) -> Self {
        assert!(block_size > 0, "ParallelHash block size must be greater than zero");

        let mut outer = cshake_node(PH256_RATE, customization);
        outer.absorb(left_encode(block_size).as_ref());

        return ParallelHash256 {
            outer,
            queue: Shake256::new(),
            queue_len: 0,
            leaves: Leaves::new(Backend::detect()),
            block_size,
            blocks: 0,
        };
    }

    /// Absorbs input.
    ///
    /// Blocks are taken straight from `data`; nothing is buffered except
    /// the current partial block, which lives inside a sponge state.
    pub fn update(&mut self, mut data: &[u8]) {
        // 1. Finish a partially filled queued block first.
        if self.queue_len > 0 {
            let take = min(self.block_size - self.queue_len, data.len());
            self.queue.absorb(&data[..take]);
            self.queue_len += take;
            data = &data[take..];

            if self.queue_len == self.block_size {
                self.flush_queue();
            }
        }

        // 2. Full batches: `width` whole blocks at a time, straight from the
        //    caller's slice. `saturating_mul` keeps a huge `block_size` from
        //    wrapping the threshold around to a tiny value.
        let width = self.leaves.width();
        while data.len() >= self.block_size.saturating_mul(width) {
            let mut blocks: [&[u8]; MAX_BATCH] = [&[]; MAX_BATCH];
            let mut rest = data;
            for slot in blocks[..width].iter_mut() {
                let (block, tail) = rest.split_at(self.block_size);
                *slot = block;
                rest = tail;
            }
            self.hash_batch(&blocks[..width], width);
            data = rest;
        }

        // 3. Fill a partial batch of `2..width` whole blocks with the unused
        //    lanes aliased to a real block. Only the first `whole` chaining
        //    values are absorbed, so the dummy lanes cannot affect the output,
        //    and no staging buffer is needed. A single leftover block is hashed
        //    sequentially (a wide permutation would not be cheaper).
        let whole = data.len() / self.block_size;
        if whole >= 2 {
            let mut blocks: [&[u8]; MAX_BATCH] = [&[]; MAX_BATCH];
            for (j, slot) in blocks[..width].iter_mut().enumerate() {
                let idx = j.min(whole - 1);
                *slot = &data[idx * self.block_size..(idx + 1) * self.block_size];
            }
            self.hash_batch(&blocks[..width], whole);
            data = &data[whole * self.block_size..];
        }

        // 4. At most one whole block remains; hash it sequentially.
        if data.len() >= self.block_size {
            let (a, rest) = data.split_at(self.block_size);
            self.hash_one(a);
            data = rest;
        }

        // 5. Buffer the partial tail inside the queue sponge.
        if !data.is_empty() {
            debug_assert_eq!(self.queue_len, 0);
            self.queue = Shake256::new();
            self.queue.absorb(data);
            self.queue_len = data.len();
        }
    }

    /// Finalizes with an output length of `out.len()` bytes.
    ///
    /// `out.len()` is bound into the hash, as required by SP 800-185:
    /// changing it produces an unrelated output. Use
    /// [`finalize_xof`](Self::finalize_xof) for the XOF variant.
    pub fn finalize_into(mut self, out: &mut [u8]) {
        self.flush_queue_if_needed();

        let n = right_encode(self.blocks);
        self.outer.absorb(n.as_ref());

        let bits = out.len().checked_mul(8).expect("ParallelHash output length overflow");
        let l = right_encode(bits);
        self.outer.absorb(l.as_ref());

        self.outer.squeeze(out);
    }

    /// Finalizes in XOF mode (`right_encode(0)`) and returns a reader.
    pub fn finalize_xof(mut self) -> ParallelHashReader {
        self.flush_queue_if_needed();

        let n = right_encode(self.blocks);
        self.outer.absorb(n.as_ref());
        let l = right_encode(0);
        self.outer.absorb(l.as_ref());

        // `Self` implements `Drop` when the `zeroize` feature is on, so move
        // the finished sponge out by swapping in a fresh placeholder (which
        // `Drop` then wipes).
        let outer = core::mem::replace(&mut self.outer, KeccakSponge::new(PH256_RATE, CSHAKE_PAD));
        return ParallelHashReader {
            inner: outer,
        };
    }

    /// Hashes `data` in one call and writes `out.len()` bytes.
    ///
    /// # Panics
    /// Panics if `block_size` is zero, or if `out.len() * 8` overflows a
    /// `usize` (which cannot happen for any realistic output length).
    pub fn hash(data: &[u8], block_size: usize, customization: &[u8], out: &mut [u8]) {
        let mut hasher = Self::new(block_size, customization);
        hasher.update(data);
        hasher.finalize_into(out);
    }

    fn flush_queue_if_needed(&mut self) {
        if self.queue_len > 0 {
            self.flush_queue();
        }
    }

    fn flush_queue(&mut self) {
        let mut cv = [0u8; PH256_CV_LEN];
        self.queue.squeeze(&mut cv);
        self.outer.absorb(&cv);

        self.queue = Shake256::new();
        self.queue_len = 0;
        self.blocks += 1;
    }

    fn hash_one(&mut self, block: &[u8]) {
        let mut cv = [0u8; PH256_CV_LEN];
        Shake256::hash(block, &mut cv);
        self.outer.absorb(&cv);
        self.blocks += 1;
    }

    fn hash_batch(&mut self, blocks: &[&[u8]], active: usize) {
        debug_assert_eq!(blocks.len(), self.leaves.width());
        debug_assert!((1..=blocks.len()).contains(&active));

        let mut cvs = [[0u8; PH256_CV_LEN]; MAX_BATCH];

        self.leaves.reset();
        self.leaves.absorb_batch(blocks);
        self.leaves.squeeze_cvs(&mut cvs[..blocks.len()]);

        for cv in &cvs[..active] {
            self.outer.absorb(cv);
        }
        self.blocks += active;
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// Tests
//////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sha3::keccak::{PI, RHO, p1600};

    /// Official NIST SP 800-185 `ParallelHash_samples` input for samples 1/2.
    const NIST_DATA_24: [u8; 24] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x20, 0x21,
        0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
    ];

    /// Official input for sample 3: `00..0B 10..1B ... 50..5B` (72 bytes).
    const NIST_DATA_72: [u8; 72] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
        0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45,
        0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b,
    ];

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

    fn pack<const W: usize>(states: &[[u64; 25]; W]) -> KeccakX<W> {
        let mut x = KeccakX::<W>::zero();
        for w in 0..W {
            for i in 0..25 {
                x.lanes[W * i + w] = states[w][i];
            }
        }
        return x;
    }

    fn assert_packed<const W: usize>(x: &KeccakX<W>, expected: &[[u64; 25]; W]) {
        for w in 0..W {
            for i in 0..25 {
                assert_eq!(x.lanes[W * i + w], expected[w][i], "state {w} lane {i}");
            }
        }
    }

    fn expected_permuted<const W: usize>(states: &[[u64; 25]; W]) -> [[u64; 25]; W] {
        let mut expected = *states;
        for state in expected.iter_mut() {
            p1600::<24>(state);
        }
        return expected;
    }

    #[test]
    fn scalar_permutation_matches_reference() {
        let mut rng = 0x243f_6a88_85a3_08d3u64;
        for _ in 0..64 {
            let states = random_states::<2>(&mut rng);
            let expected = expected_permuted(&states);
            let mut x = pack(&states);
            scalar::permute::<2>(&mut x.lanes);
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<4>(&mut rng);
            let expected = expected_permuted(&states);
            let mut x = pack(&states);
            scalar::permute::<4>(&mut x.lanes);
            assert_packed(&x, &expected);
        }
    }

    #[test]
    fn dispatched_permutation_matches_reference() {
        let mut rng = 0x1319_8a2e_0370_7344u64;
        for _ in 0..64 {
            let states = random_states::<2>(&mut rng);
            let expected = expected_permuted(&states);
            let mut x = pack(&states);
            x.permute(Backend::detect());
            assert_packed(&x, &expected);
        }
        for _ in 0..64 {
            let states = random_states::<4>(&mut rng);
            let expected = expected_permuted(&states);
            let mut x = pack(&states);
            x.permute(Backend::detect());
            assert_packed(&x, &expected);
        }
        #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
        if avx512_available() {
            for _ in 0..64 {
                let states = random_states::<8>(&mut rng);
                let expected = expected_permuted(&states);
                let mut x = pack(&states);
                x.permute(Backend::Avx512);
                assert_packed(&x, &expected);
            }
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
            let expected = expected_permuted(&states);
            let mut x = pack(&states);
            // SAFETY: NEON is baseline on aarch64.
            let state: &mut [u64; 50] = (&mut x.lanes[..50]).try_into().unwrap();
            unsafe { neon::permute(state) };
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
            let expected = expected_permuted(&states);
            let mut x = pack(&states);
            // SAFETY: the SHA-3 extension was detected above.
            let state: &mut [u64; 50] = (&mut x.lanes[..50]).try_into().unwrap();
            unsafe { sha3_ext::permute(state) };
            assert_packed(&x, &expected);
        }
    }

    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    #[test]
    fn avx2_permutation_matches_reference() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }

        let mut rng = 0x8e4a_1c37_5d09_b2f6u64;
        for _ in 0..64 {
            let states = random_states::<4>(&mut rng);
            let expected = expected_permuted(&states);
            let mut x = pack(&states);
            // SAFETY: AVX2 was detected above, and `KeccakX<4>` is 64-byte
            // aligned with at least 100 lanes.
            unsafe { avx2::permute(&mut x.lanes) };
            assert_packed(&x, &expected);
        }
    }

    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    #[test]
    fn avx512_permutation_matches_reference() {
        if !avx512_available() {
            return;
        }

        let mut rng = 0x5d2c_9f81_7a63_e40bu64;
        for _ in 0..64 {
            let states = random_states::<8>(&mut rng);
            let expected = expected_permuted(&states);
            let mut x = pack(&states);
            // SAFETY: AVX-512F was detected above, and `KeccakX<8>` is 64-byte
            // aligned with exactly `MAX_LANES` lanes.
            unsafe { avx512::permute(&mut x.lanes) };
            assert_packed(&x, &expected);
        }
    }

    fn random_bytes(len: usize, rng: &mut u64) -> std::vec::Vec<u8> {
        let mut out = std::vec![0u8; len];
        for byte in out.iter_mut() {
            *byte = next_u64(rng) as u8;
        }
        return out;
    }

    fn check_batch<const W: usize>(
        msgs: &[std::vec::Vec<u8>],
        expected: &[std::vec::Vec<u8>],
        backend: Backend,
        len: usize,
    ) {
        let refs: [&[u8]; W] = core::array::from_fn(|w| &msgs[w][..]);
        let mut sponge = SpongeX::<W>::new(PH256_RATE, SHAKE_PAD, backend);
        let mut cvs = [[0u8; PH256_CV_LEN]; W];
        sponge.absorb_batch(&refs);
        sponge.squeeze_cvs(&mut cvs);
        for w in 0..W {
            assert_eq!(cvs[w][..], expected[w][..], "width {W}, len {len}, lane {w}");
        }
    }

    #[test]
    fn batched_shake_matches_one_way_shake() {
        let mut rng = 0x4528_21e6_38d0_1377u64;
        for len in [0usize, 1, 7, 8, 135, 136, 137, 168, 169, 272, 1024, 4097] {
            let base: [std::vec::Vec<u8>; 4] = core::array::from_fn(|_| random_bytes(len, &mut rng));
            let msgs: [std::vec::Vec<u8>; 8] = core::array::from_fn(|w| base[w % 4].clone());

            let expected: [std::vec::Vec<u8>; 8] = core::array::from_fn(|w| {
                let mut out = std::vec![0u8; PH256_CV_LEN];
                Shake256::hash(&msgs[w], &mut out);
                out
            });

            // Two-way: the SHA-3 or NEON kernel on aarch64, scalar elsewhere.
            check_batch::<2>(&msgs, &expected, Backend::detect(), len);

            // Wider batches only exist on `x86_64`.
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            {
                let backend = if std::arch::is_x86_feature_detected!("avx2") {
                    Backend::Avx2
                } else {
                    Backend::Scalar
                };
                check_batch::<4>(&msgs, &expected, backend, len);
            }

            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            if avx512_available() {
                check_batch::<8>(&msgs, &expected, Backend::Avx512, len);
            }
        }
    }

    #[test]
    fn nist_parallel_hash_256_samples() {
        let mut out = [0u8; 64];

        ParallelHash256::hash(&NIST_DATA_24, 8, b"", &mut out);
        assert_eq!(
            hex::encode(out),
            "bc1ef124da34495e948ead207dd9842235da432d2bbc54b4c110e64c45110553\
             1b7f2a3e0ce055c02805e7c2de1fb746af97a1dd01f43b824e31b87612410429"
        );

        ParallelHash256::hash(&NIST_DATA_24, 8, b"Parallel Data", &mut out);
        assert_eq!(
            hex::encode(out),
            "cdf15289b54f6212b4bc270528b49526006dd9b54e2b6add1ef6900dda3963bb\
             33a72491f236969ca8afaea29c682d47a393c065b38e29fae651a2091c833110"
        );

        ParallelHash256::hash(&NIST_DATA_72, 12, b"Parallel Data", &mut out);
        assert_eq!(
            hex::encode(out),
            "69d0fcb764ea055dd09334bc6021cb7e4b61348dff375da262671cdec3effa8d\
             1b4568a6cce16b1cad946ddde27f6ce2b8dee4cd1b24851ebf00eb90d43813e9"
        );
    }

    /// One reference vector: `data` is `pattern_data(data_len)`.
    pub(crate) struct PhVec {
        pub block_size: usize,
        pub data_len: usize,
        pub out_len: usize,
        pub customization: &'static [u8],
        pub expected: &'static [u8],
    }

    pub(crate) fn pattern_data(len: usize) -> std::vec::Vec<u8> {
        (0..len)
            .map(|i| (i.wrapping_mul(7).wrapping_add(3) & 0xff) as u8)
            .collect()
    }

    // Kept on one line per vector so the table stays readable.
    #[rustfmt::skip]
    pub(crate) const PH_VECTORS: &[PhVec] = &[
        PhVec { block_size: 1, data_len: 0, out_len: 64, customization: b"", expected: &[0x9f, 0x3e, 0x7a, 0xb5, 0x7e, 0xed, 0x7b, 0xb6, 0xe1, 0x1d, 0x40, 0x38, 0x7c, 0x12, 0x48, 0xfd, 0x8e, 0x4d, 0xe9, 0xb9, 0x46, 0xd8, 0x21, 0xbb, 0x35, 0x15, 0x28, 0x05, 0x61, 0x10, 0xbf, 0x34, 0x3f, 0xc1, 0x11, 0x74, 0xdc, 0x2f, 0x22, 0xb2, 0xce, 0x45, 0x53, 0x87, 0x36, 0x21, 0x97, 0x6e, 0xd0, 0x17, 0x3a, 0xe0, 0x9e, 0x16, 0x49, 0x38, 0xc5, 0x72, 0x23, 0x32, 0xd5, 0xf8, 0xd0, 0x99] },
        PhVec { block_size: 1, data_len: 1, out_len: 64, customization: b"", expected: &[0x8a, 0x2b, 0xda, 0x68, 0x33, 0x80, 0x5f, 0x1c, 0xd4, 0x96, 0x5a, 0x9e, 0x86, 0xbd, 0xf9, 0xc4, 0x19, 0x01, 0xe4, 0x08, 0xef, 0xa2, 0xcd, 0x15, 0x7b, 0x44, 0xbf, 0xc9, 0x3a, 0x16, 0xf8, 0x90, 0x4d, 0x3f, 0xf3, 0x2f, 0x2e, 0x61, 0x6a, 0x3f, 0x69, 0xc3, 0x1d, 0x1d, 0x06, 0x20, 0x6f, 0xd8, 0xf6, 0xa0, 0x4e, 0xaf, 0x02, 0xea, 0x23, 0xf2, 0x80, 0x40, 0xbd, 0xc5, 0x19, 0x7a, 0xd5, 0x86] },
        PhVec { block_size: 1, data_len: 2, out_len: 64, customization: b"", expected: &[0xad, 0xc5, 0x6b, 0x1d, 0xe3, 0x9e, 0xd7, 0x97, 0x56, 0x86, 0xff, 0x68, 0xd8, 0x0f, 0x58, 0xd1, 0x56, 0x42, 0x18, 0x78, 0x2d, 0xd3, 0xdd, 0xdc, 0x27, 0xbb, 0xe3, 0x66, 0x2e, 0x2f, 0x73, 0xd7, 0x30, 0x24, 0x7a, 0xb1, 0x49, 0x66, 0x7e, 0xcf, 0x27, 0xe0, 0xcb, 0x09, 0x53, 0xd6, 0x1a, 0x93, 0x7b, 0xcd, 0xc6, 0xa5, 0x7f, 0x30, 0x99, 0x4a, 0x24, 0x03, 0x83, 0xf5, 0x94, 0x1a, 0x21, 0xe0] },
        PhVec { block_size: 1, data_len: 3, out_len: 64, customization: b"", expected: &[0x25, 0x0b, 0x63, 0xfb, 0x65, 0x2e, 0x5b, 0xdf, 0x21, 0xff, 0xd6, 0x0e, 0x8d, 0x12, 0x26, 0xb5, 0xd3, 0xd9, 0xbd, 0x37, 0x71, 0x6a, 0xf5, 0xe9, 0x9b, 0xc4, 0x05, 0x42, 0xaa, 0x0d, 0xf0, 0x7e, 0x25, 0xd3, 0x78, 0x8a, 0x85, 0xfc, 0xc6, 0x71, 0xe0, 0xd1, 0xa4, 0xbf, 0xf8, 0x58, 0x23, 0xda, 0x9f, 0xb3, 0x3c, 0x5d, 0xf6, 0x2c, 0xf1, 0x66, 0x61, 0x2c, 0x73, 0x9e, 0xdc, 0x64, 0xe6, 0x83] },
        PhVec { block_size: 1, data_len: 10, out_len: 64, customization: b"", expected: &[0x25, 0xe9, 0x86, 0x38, 0x3d, 0xd5, 0x3c, 0xff, 0xdb, 0x05, 0x64, 0x26, 0x0d, 0x2a, 0x09, 0x66, 0x2b, 0xd3, 0x43, 0x0a, 0x54, 0x12, 0x25, 0x0e, 0x33, 0x11, 0x1a, 0x6e, 0x24, 0x1e, 0x6a, 0x1f, 0xf4, 0x79, 0xdb, 0xe1, 0x68, 0xbe, 0xef, 0x2c, 0x5e, 0x1a, 0x6b, 0x7b, 0xbc, 0xbb, 0xb2, 0xb7, 0x3e, 0xb2, 0xf2, 0xbe, 0xa7, 0x25, 0x0d, 0x1c, 0xdd, 0x53, 0x18, 0xd3, 0x3d, 0x68, 0xeb, 0x20] },
        PhVec { block_size: 1, data_len: 3, out_len: 64, customization: b"Parallel Data", expected: &[0x13, 0x79, 0x3d, 0x5c, 0xa6, 0x95, 0x5c, 0x49, 0x68, 0x40, 0x55, 0xec, 0x5f, 0xdb, 0x1a, 0xdf, 0xdc, 0xa6, 0x24, 0x1b, 0x65, 0xea, 0x38, 0xa8, 0x5d, 0xd0, 0x16, 0xe8, 0xf4, 0x62, 0x31, 0x62, 0x11, 0xeb, 0x17, 0x8e, 0x9b, 0x38, 0xdd, 0x19, 0xe9, 0x51, 0xc8, 0x08, 0xeb, 0x69, 0xe1, 0x10, 0xa0, 0xfd, 0x91, 0xba, 0xb7, 0x66, 0x4a, 0xdc, 0x8a, 0x85, 0xc1, 0xf6, 0xd0, 0x08, 0x9f, 0x46] },
        PhVec { block_size: 8, data_len: 0, out_len: 64, customization: b"", expected: &[0x0f, 0x86, 0x63, 0x26, 0xa0, 0x8d, 0x47, 0x16, 0xbe, 0x9a, 0x36, 0x43, 0xbc, 0x1e, 0x12, 0x54, 0x95, 0xea, 0x63, 0xde, 0xd6, 0xd7, 0x9c, 0xf4, 0x80, 0x8b, 0x88, 0xec, 0x86, 0xc5, 0xcd, 0xeb, 0x33, 0xb0, 0x4c, 0xad, 0xe8, 0xa4, 0x34, 0xd2, 0x47, 0xad, 0x3e, 0xe9, 0xe7, 0xce, 0x9c, 0xf0, 0xb7, 0xe5, 0xe6, 0xd7, 0x4d, 0xa6, 0x6c, 0x9d, 0x52, 0x97, 0x42, 0xaf, 0x0d, 0x17, 0xe7, 0x39] },
        PhVec { block_size: 8, data_len: 1, out_len: 64, customization: b"", expected: &[0xe1, 0x0f, 0x55, 0x6f, 0x28, 0x69, 0x92, 0x2a, 0x61, 0x51, 0x58, 0x9e, 0x71, 0xc6, 0xad, 0xc1, 0x81, 0x2b, 0x80, 0x7a, 0xae, 0x03, 0x5b, 0x16, 0x48, 0x09, 0x58, 0xde, 0xf4, 0xe2, 0x42, 0xe1, 0x87, 0x92, 0x75, 0x36, 0x52, 0x23, 0x2a, 0xa4, 0xa7, 0x4b, 0xb5, 0xbc, 0x3e, 0xe5, 0xd5, 0x53, 0x77, 0x38, 0xc9, 0x4f, 0xc8, 0xaf, 0xe4, 0xc0, 0x18, 0x86, 0x7a, 0x5c, 0xa7, 0x9a, 0x7f, 0x39] },
        PhVec { block_size: 8, data_len: 7, out_len: 64, customization: b"", expected: &[0xea, 0x79, 0x7b, 0x28, 0x8f, 0xc8, 0x76, 0x71, 0xdb, 0xda, 0xa9, 0x94, 0x6f, 0x4c, 0xae, 0xec, 0x20, 0xe3, 0xfc, 0x8e, 0x29, 0xca, 0x7c, 0x53, 0xbb, 0xbe, 0x9e, 0x4e, 0xf3, 0xd6, 0x70, 0xd4, 0xae, 0xf5, 0xb0, 0x81, 0xd8, 0x10, 0x49, 0x39, 0x0b, 0xcc, 0xb8, 0x54, 0x80, 0x71, 0xe3, 0x12, 0xbb, 0x92, 0x01, 0x76, 0xe5, 0xa1, 0xf5, 0x22, 0x9b, 0x43, 0xae, 0xd3, 0x97, 0x24, 0xbb, 0xa2] },
        PhVec { block_size: 8, data_len: 8, out_len: 64, customization: b"", expected: &[0x51, 0x30, 0xc6, 0x6a, 0x78, 0xd7, 0xc9, 0x86, 0xe0, 0x95, 0x6a, 0xb9, 0x66, 0xdf, 0x84, 0x1a, 0x15, 0x32, 0x66, 0xd4, 0x59, 0x57, 0x25, 0x00, 0x85, 0x34, 0xa5, 0x32, 0xac, 0xcf, 0x8d, 0xda, 0x57, 0x34, 0xc6, 0xd7, 0xe2, 0x3b, 0x74, 0x39, 0xb3, 0xed, 0x8e, 0x45, 0xb7, 0xe9, 0x4d, 0xcc, 0xd2, 0xa5, 0x03, 0xef, 0x25, 0x53, 0x96, 0xc8, 0xaa, 0x1a, 0xd5, 0xde, 0xbd, 0xad, 0xce, 0xa9] },
        PhVec { block_size: 8, data_len: 9, out_len: 64, customization: b"", expected: &[0xcd, 0x6c, 0x77, 0x47, 0xd1, 0x43, 0xf1, 0x4a, 0x46, 0xe6, 0x45, 0x4f, 0x63, 0xdd, 0x6c, 0xad, 0x74, 0xd2, 0x8a, 0x62, 0x50, 0xd8, 0xf8, 0x58, 0x55, 0xb6, 0xfe, 0x73, 0xb3, 0xf3, 0xbb, 0xdc, 0x66, 0xae, 0x68, 0x31, 0x0a, 0xe5, 0x0b, 0xf4, 0xb6, 0x45, 0x73, 0x5c, 0x97, 0x90, 0x8c, 0xfd, 0xd6, 0x6f, 0x5d, 0x8d, 0x8e, 0xa5, 0x2f, 0xa9, 0x21, 0x7e, 0x76, 0xc1, 0x0c, 0xb8, 0xf7, 0xc4] },
        PhVec { block_size: 8, data_len: 16, out_len: 64, customization: b"", expected: &[0xee, 0x4d, 0x20, 0x63, 0x78, 0x2e, 0xd3, 0x56, 0x14, 0xdf, 0xa5, 0x7f, 0x17, 0x82, 0x1b, 0x08, 0x66, 0x82, 0x46, 0x06, 0x4c, 0xe1, 0x5a, 0xf6, 0x6b, 0x55, 0xd5, 0x38, 0x92, 0x1f, 0x7d, 0x22, 0xd6, 0xab, 0xbe, 0xe3, 0xcc, 0xc1, 0x7f, 0x45, 0x1f, 0xa0, 0x25, 0x61, 0x5c, 0x49, 0xc6, 0x18, 0xc0, 0x79, 0xf8, 0x87, 0x32, 0x16, 0x88, 0x57, 0x1d, 0xb6, 0xac, 0x07, 0x13, 0x82, 0xb1, 0x73] },
        PhVec { block_size: 8, data_len: 17, out_len: 64, customization: b"", expected: &[0x72, 0x21, 0xb5, 0xa5, 0x84, 0xfe, 0xe2, 0xc2, 0xf5, 0xf1, 0xea, 0x4e, 0x4d, 0x7f, 0x56, 0xc4, 0x9d, 0x68, 0x0c, 0xd6, 0xd2, 0x5c, 0x9b, 0xe2, 0xe0, 0x4c, 0x99, 0x43, 0xc8, 0x9a, 0x0e, 0xd7, 0x3d, 0x6d, 0x77, 0xf9, 0x23, 0xdb, 0xbf, 0xb6, 0xdc, 0x22, 0x5c, 0x7d, 0x64, 0xcc, 0x00, 0xad, 0x35, 0x8d, 0x54, 0x11, 0x4e, 0x5b, 0xac, 0x44, 0x9b, 0x05, 0xb7, 0x6e, 0x79, 0x8a, 0xbf, 0xf2] },
        PhVec { block_size: 8, data_len: 24, out_len: 64, customization: b"", expected: &[0x54, 0x06, 0x8c, 0xfd, 0x3c, 0xf2, 0xb8, 0x4e, 0x94, 0x34, 0xb9, 0x55, 0x86, 0x15, 0xbc, 0xb8, 0x4a, 0x8c, 0xf8, 0x7e, 0x4a, 0x61, 0x6a, 0x56, 0x4e, 0x68, 0xe5, 0x57, 0x73, 0x2c, 0xc7, 0x83, 0x95, 0xb3, 0x4a, 0x91, 0x4a, 0x67, 0xaf, 0x70, 0xf5, 0x4c, 0x83, 0x8c, 0x22, 0x33, 0x2c, 0x58, 0x9b, 0x49, 0xfd, 0x3a, 0x5c, 0xf6, 0x49, 0xec, 0x05, 0x08, 0x83, 0xf0, 0x70, 0xef, 0x4c, 0x32] },
        PhVec { block_size: 8, data_len: 45, out_len: 64, customization: b"", expected: &[0x58, 0x3d, 0x66, 0xb8, 0x60, 0xfe, 0xbf, 0x53, 0xdd, 0x7c, 0xc3, 0x74, 0xeb, 0xa1, 0x93, 0x87, 0x6e, 0x3b, 0x05, 0xcb, 0x68, 0xd1, 0x91, 0xa7, 0x01, 0x88, 0xd9, 0xd9, 0x14, 0x26, 0x04, 0x9c, 0x83, 0x42, 0xf0, 0x85, 0xf5, 0x82, 0x37, 0x0f, 0x72, 0x50, 0xee, 0xea, 0x1b, 0x7c, 0xce, 0x31, 0xfc, 0x02, 0xbf, 0xe4, 0x45, 0x37, 0x1d, 0x63, 0xaa, 0x16, 0x17, 0x90, 0x71, 0x27, 0x02, 0x77] },
        PhVec { block_size: 8, data_len: 17, out_len: 64, customization: b"Parallel Data", expected: &[0x3d, 0xea, 0x65, 0xd4, 0xed, 0xa6, 0x8d, 0xac, 0xff, 0x66, 0xc5, 0xbc, 0x0e, 0x32, 0x39, 0x80, 0x55, 0x44, 0x01, 0xc9, 0x95, 0xb8, 0xd7, 0xce, 0xe3, 0x86, 0x0e, 0x0f, 0x7b, 0x3d, 0x37, 0x6a, 0xa1, 0xbf, 0xae, 0xc2, 0x0c, 0xe0, 0xde, 0x64, 0xfa, 0xfe, 0xb3, 0xf8, 0xd3, 0xae, 0x19, 0x9f, 0x6f, 0x8c, 0xba, 0xde, 0xbd, 0x9b, 0xd7, 0xe9, 0x41, 0x29, 0x0a, 0xd0, 0x8b, 0x30, 0xca, 0x2f] },
        PhVec { block_size: 12, data_len: 0, out_len: 64, customization: b"", expected: &[0xa5, 0xce, 0x3f, 0x84, 0x23, 0x6a, 0x85, 0x72, 0x86, 0x26, 0xfc, 0x2a, 0x4e, 0x1c, 0x32, 0x86, 0xee, 0x36, 0xd6, 0xde, 0x2b, 0xaf, 0x18, 0xba, 0xe1, 0x9c, 0x35, 0x67, 0xa4, 0x22, 0x6e, 0xb7, 0x70, 0x37, 0xb0, 0xfd, 0x7f, 0xfc, 0xdb, 0x50, 0x62, 0x45, 0xb5, 0x7c, 0x2f, 0x21, 0x7a, 0xe7, 0x30, 0xbb, 0x06, 0x30, 0x00, 0xf2, 0x34, 0xb5, 0x2c, 0x28, 0x15, 0x44, 0xc4, 0xb6, 0xf1, 0xf9] },
        PhVec { block_size: 12, data_len: 1, out_len: 64, customization: b"", expected: &[0x53, 0xa5, 0xc2, 0x26, 0x17, 0x84, 0xba, 0xd3, 0x74, 0x7d, 0x2e, 0x96, 0x5d, 0x88, 0x0e, 0x11, 0xe6, 0xcf, 0x70, 0x41, 0x63, 0xc2, 0x94, 0x11, 0x02, 0x82, 0x6f, 0x1e, 0x32, 0x7a, 0x46, 0xf2, 0xc0, 0x83, 0x22, 0x9c, 0xb7, 0xe2, 0xcb, 0x2b, 0xa4, 0x80, 0x53, 0xc0, 0x13, 0x93, 0xc2, 0x19, 0xe4, 0x08, 0x58, 0xf4, 0x53, 0x53, 0x44, 0x35, 0xbb, 0x9c, 0x54, 0xb0, 0x3c, 0xca, 0x57, 0xc7] },
        PhVec { block_size: 12, data_len: 11, out_len: 64, customization: b"", expected: &[0x20, 0x3c, 0x00, 0x1d, 0x87, 0x75, 0x8a, 0xf7, 0x24, 0x0c, 0xff, 0xd8, 0x8e, 0xbe, 0xdc, 0x16, 0xfb, 0x6d, 0x90, 0x92, 0x5c, 0xf4, 0x3f, 0x50, 0xe4, 0xbf, 0x70, 0x96, 0x1a, 0x04, 0xdb, 0xf0, 0x17, 0x9c, 0x12, 0x1e, 0x2f, 0xfb, 0xca, 0xc5, 0x56, 0x2f, 0x20, 0x31, 0xf5, 0x68, 0x5d, 0xa6, 0x72, 0x6c, 0x4b, 0xa5, 0x0c, 0x46, 0x5d, 0xc7, 0x3b, 0x14, 0x31, 0x2c, 0xbc, 0x3b, 0x8a, 0x9f] },
        PhVec { block_size: 12, data_len: 12, out_len: 64, customization: b"", expected: &[0x0a, 0x04, 0x96, 0xbb, 0xa9, 0x20, 0x9c, 0x8a, 0xcd, 0x51, 0xa0, 0x9c, 0x87, 0x16, 0xa3, 0xcf, 0xcd, 0x54, 0xed, 0x6f, 0x1e, 0x8d, 0x78, 0xd9, 0x54, 0x5e, 0xfc, 0x2d, 0x92, 0x1e, 0x3d, 0xf9, 0xf3, 0x66, 0x62, 0x9a, 0x43, 0x62, 0x57, 0xa1, 0x4f, 0x70, 0xf0, 0xed, 0x9c, 0x00, 0x1f, 0x62, 0x40, 0x24, 0x5b, 0xdc, 0x92, 0xb1, 0x03, 0x22, 0x68, 0x85, 0x24, 0x64, 0xc0, 0xe2, 0x60, 0x48] },
        PhVec { block_size: 12, data_len: 13, out_len: 64, customization: b"", expected: &[0xd5, 0x30, 0x02, 0x2e, 0x30, 0x8c, 0xce, 0x74, 0x40, 0x6f, 0x12, 0x32, 0xd8, 0xd9, 0x65, 0xc2, 0xd4, 0x45, 0x02, 0x29, 0xa5, 0xfe, 0x57, 0x88, 0x33, 0x82, 0x3a, 0x96, 0x66, 0xc0, 0xc3, 0xa6, 0xd3, 0xd9, 0x73, 0x9e, 0xe6, 0xd0, 0xac, 0xbb, 0x63, 0xc9, 0x5b, 0xca, 0x36, 0x5d, 0x10, 0x24, 0x27, 0xb8, 0x91, 0x3a, 0x6a, 0x11, 0x7c, 0xa7, 0xa7, 0x51, 0xa7, 0xf9, 0x07, 0xb7, 0xf5, 0x6c] },
        PhVec { block_size: 12, data_len: 24, out_len: 64, customization: b"", expected: &[0x40, 0xdc, 0x7d, 0x3c, 0x0e, 0x85, 0xcb, 0xac, 0xe6, 0xbc, 0x48, 0x51, 0x96, 0x89, 0xae, 0x29, 0x22, 0xc1, 0xbb, 0x71, 0xd4, 0x38, 0x15, 0x02, 0x4b, 0xcb, 0x0d, 0xed, 0x67, 0xc6, 0x56, 0x55, 0x23, 0x6a, 0xb7, 0x8e, 0x8a, 0xfc, 0x22, 0xdf, 0x2b, 0x95, 0xb6, 0x7c, 0x39, 0x38, 0x8f, 0xdd, 0x80, 0x43, 0xd7, 0x48, 0xbb, 0x70, 0x22, 0xc9, 0xe4, 0xbb, 0x70, 0x61, 0xcf, 0x72, 0xd4, 0xc2] },
        PhVec { block_size: 12, data_len: 25, out_len: 64, customization: b"", expected: &[0x75, 0xaf, 0x2f, 0x04, 0x8d, 0x04, 0xcc, 0x95, 0x55, 0x49, 0x22, 0xde, 0x79, 0xb6, 0x50, 0x98, 0x3c, 0xd5, 0x1b, 0xe0, 0x7f, 0xbe, 0xd6, 0xb2, 0x99, 0xe2, 0x13, 0x18, 0x00, 0xcf, 0x79, 0x53, 0x2e, 0x8a, 0x27, 0x7d, 0x8f, 0xbb, 0x58, 0x71, 0xad, 0xdb, 0x62, 0x75, 0x43, 0x2f, 0x0b, 0x7d, 0x33, 0xb8, 0x7f, 0x14, 0x42, 0x49, 0x39, 0x87, 0x74, 0xe4, 0x98, 0x78, 0x76, 0xa7, 0xc6, 0x2e] },
        PhVec { block_size: 12, data_len: 36, out_len: 64, customization: b"", expected: &[0xfe, 0xfa, 0x56, 0x6f, 0x1d, 0x5c, 0x85, 0x2a, 0xdb, 0x3b, 0x7e, 0x3d, 0xfb, 0xbf, 0x01, 0xb8, 0xc7, 0xc4, 0x51, 0x2b, 0xb6, 0xe2, 0xdd, 0xf1, 0xa0, 0x4c, 0x86, 0xc6, 0x93, 0x05, 0x9f, 0x9d, 0xb1, 0x44, 0xee, 0xf8, 0x6b, 0xa4, 0xdf, 0x67, 0xf7, 0xd7, 0xfd, 0x96, 0xd7, 0xcd, 0xe9, 0xfe, 0xa8, 0x41, 0x28, 0x06, 0xf7, 0x84, 0x47, 0xb3, 0xb7, 0xa4, 0x19, 0xbb, 0x18, 0x3f, 0x81, 0xac] },
        PhVec { block_size: 12, data_len: 65, out_len: 64, customization: b"", expected: &[0x5d, 0xcc, 0xc7, 0x54, 0xa7, 0xe4, 0xbb, 0x90, 0x75, 0xff, 0x78, 0x20, 0xa1, 0xd7, 0x7b, 0xbc, 0x1e, 0x99, 0xd2, 0xf3, 0x1a, 0x8e, 0x22, 0xab, 0xef, 0xef, 0xdb, 0x33, 0xe5, 0x3e, 0x0b, 0x1c, 0x54, 0xad, 0x7e, 0x3c, 0xcd, 0x34, 0x90, 0xc5, 0xcd, 0x9e, 0xed, 0x7d, 0x38, 0x96, 0x80, 0x93, 0xb0, 0xab, 0x14, 0xa1, 0xd5, 0xcb, 0xaa, 0xb0, 0xa7, 0xc5, 0x4e, 0xbb, 0x52, 0x8f, 0xb1, 0x45] },
        PhVec { block_size: 12, data_len: 25, out_len: 64, customization: b"Parallel Data", expected: &[0xab, 0xb7, 0x7f, 0xd6, 0xc0, 0x46, 0x81, 0xa9, 0x04, 0x96, 0x82, 0x80, 0xb1, 0xc4, 0xec, 0xcf, 0x2a, 0x14, 0x17, 0x0e, 0x13, 0xbf, 0x0c, 0xc9, 0x55, 0x55, 0x45, 0x2c, 0x3d, 0x48, 0x6b, 0xa5, 0xbc, 0xc5, 0xe7, 0xc7, 0x47, 0xd9, 0x57, 0xa4, 0x61, 0x7d, 0x57, 0x87, 0x21, 0xa4, 0xad, 0x16, 0xca, 0xc9, 0x28, 0x50, 0xfa, 0x0d, 0x97, 0xd6, 0xc3, 0xc7, 0x6d, 0xe2, 0xda, 0x5f, 0x8f, 0x37] },
        PhVec { block_size: 64, data_len: 0, out_len: 64, customization: b"", expected: &[0x3b, 0x42, 0xe3, 0x80, 0xf5, 0x79, 0xe4, 0x9f, 0xe3, 0x4c, 0x3a, 0x25, 0xe9, 0xad, 0x58, 0x01, 0x12, 0xe8, 0xf7, 0xec, 0x28, 0x4e, 0x07, 0x36, 0x02, 0xe0, 0x1d, 0x25, 0x43, 0x7c, 0x93, 0xbc, 0xda, 0x21, 0x7e, 0xeb, 0xe7, 0x02, 0x97, 0x0c, 0x67, 0x81, 0xcb, 0x29, 0xd2, 0x25, 0x08, 0xc8, 0x8b, 0x42, 0xc0, 0xb4, 0x9e, 0x14, 0x55, 0x50, 0xfd, 0x25, 0xc1, 0x7d, 0x28, 0x0c, 0xc4, 0xb2] },
        PhVec { block_size: 64, data_len: 1, out_len: 64, customization: b"", expected: &[0x32, 0xf1, 0x44, 0xbd, 0x06, 0xfb, 0x25, 0x8c, 0xb9, 0x39, 0xbf, 0x53, 0x12, 0xc2, 0xe5, 0xe9, 0x20, 0xef, 0x5d, 0x0a, 0x8b, 0xef, 0x21, 0xc6, 0x80, 0x00, 0xda, 0xaa, 0xa4, 0xd1, 0xbe, 0xe0, 0xa5, 0xfe, 0xd6, 0x2f, 0x69, 0x52, 0xcd, 0xbb, 0x05, 0x22, 0xd6, 0x7a, 0x70, 0x10, 0xfe, 0xec, 0x2e, 0xcb, 0x45, 0xfb, 0xcb, 0x98, 0x03, 0xb3, 0x5b, 0x09, 0xd0, 0x5f, 0xe7, 0xa4, 0xd4, 0xaa] },
        PhVec { block_size: 64, data_len: 63, out_len: 64, customization: b"", expected: &[0x17, 0x64, 0xed, 0x99, 0xf5, 0x61, 0x0a, 0xac, 0x3b, 0xb6, 0xbf, 0x56, 0x0b, 0x63, 0x94, 0x6a, 0x4a, 0x1b, 0xa3, 0x54, 0x32, 0xea, 0xdf, 0xcf, 0xdf, 0x90, 0x76, 0x5a, 0xac, 0xfa, 0x62, 0x91, 0xbc, 0xe8, 0x3d, 0x1e, 0xc5, 0xc1, 0x01, 0xb1, 0xde, 0xfa, 0x51, 0x85, 0x7c, 0x5c, 0x44, 0x65, 0xa8, 0xa4, 0x31, 0xa5, 0x77, 0x92, 0x98, 0xba, 0x94, 0x4d, 0x04, 0x70, 0x51, 0x27, 0x32, 0x0f] },
        PhVec { block_size: 64, data_len: 64, out_len: 64, customization: b"", expected: &[0x79, 0x16, 0xca, 0x9c, 0xd5, 0x3f, 0xe3, 0x6b, 0x67, 0x6f, 0x20, 0xed, 0x64, 0x46, 0xcc, 0x0a, 0x68, 0x87, 0xe1, 0x80, 0xd6, 0x0c, 0xcd, 0xf9, 0x35, 0xb9, 0x50, 0x84, 0x7d, 0xfb, 0x8f, 0x93, 0x5c, 0x93, 0xf4, 0x42, 0xd0, 0x20, 0x49, 0xe6, 0xbf, 0x86, 0x7d, 0xe0, 0x56, 0xf2, 0xa4, 0xbd, 0x44, 0x0d, 0xaf, 0x87, 0x18, 0x7e, 0x02, 0x22, 0x21, 0x4d, 0xc6, 0xfb, 0xf4, 0x1a, 0x9b, 0x57] },
        PhVec { block_size: 64, data_len: 65, out_len: 64, customization: b"", expected: &[0xa4, 0xc6, 0x63, 0xa9, 0x38, 0x1d, 0xff, 0xce, 0xd5, 0x1d, 0xce, 0xad, 0xf5, 0xa8, 0xea, 0x18, 0xee, 0xb3, 0xed, 0x12, 0xa8, 0x49, 0xfa, 0x15, 0x15, 0x24, 0x35, 0xc8, 0xb4, 0xc7, 0x55, 0xb2, 0x3c, 0x09, 0x05, 0x5e, 0x48, 0x91, 0x0d, 0xc3, 0x49, 0xc4, 0x35, 0xb5, 0x15, 0xb7, 0x39, 0xca, 0xd4, 0xcc, 0xb6, 0x1d, 0x44, 0xed, 0xc1, 0x8f, 0x9e, 0xe0, 0x7c, 0xff, 0x88, 0x97, 0x13, 0x45] },
        PhVec { block_size: 64, data_len: 128, out_len: 64, customization: b"", expected: &[0xed, 0xa1, 0x3e, 0xfd, 0x3d, 0x5d, 0xe2, 0xeb, 0x7a, 0x83, 0xd5, 0x5f, 0xec, 0x4b, 0xe4, 0x86, 0xd5, 0x83, 0xab, 0x72, 0xc5, 0xe0, 0xf2, 0x1f, 0x9c, 0x1b, 0xb9, 0x5f, 0xa0, 0x17, 0xe9, 0x57, 0xb0, 0x30, 0xe4, 0xce, 0xc5, 0xd8, 0x37, 0xad, 0xf2, 0xe0, 0x5c, 0x68, 0x78, 0xb3, 0x7a, 0x1a, 0xe9, 0x2d, 0x2b, 0x3c, 0x0e, 0xed, 0x4a, 0x0f, 0xd9, 0xec, 0xbc, 0x3f, 0xae, 0xa0, 0xd7, 0xc2] },
        PhVec { block_size: 64, data_len: 129, out_len: 64, customization: b"", expected: &[0xd2, 0xd1, 0x61, 0x9b, 0x82, 0x4a, 0x6c, 0xe4, 0x01, 0xe9, 0x29, 0xf5, 0x37, 0x08, 0x18, 0xa7, 0xad, 0xa1, 0x3b, 0x5f, 0x98, 0x38, 0x29, 0xaa, 0xb7, 0x91, 0xc6, 0x80, 0x46, 0xdc, 0x57, 0x2c, 0x20, 0x74, 0xde, 0x8c, 0x97, 0x69, 0x3d, 0x3b, 0x7d, 0x68, 0x13, 0x68, 0xcd, 0x7f, 0xdb, 0x45, 0x40, 0x13, 0xb0, 0x1e, 0x09, 0x68, 0xb8, 0xdf, 0xab, 0xad, 0x73, 0x3b, 0xfe, 0x44, 0xdc, 0xca] },
        PhVec { block_size: 64, data_len: 192, out_len: 64, customization: b"", expected: &[0x37, 0x21, 0xdf, 0xf3, 0x89, 0xb1, 0xde, 0xf7, 0x7d, 0xf6, 0x58, 0xbf, 0x9c, 0xad, 0xf2, 0x2d, 0xdd, 0xc6, 0xf6, 0x89, 0xa4, 0x8b, 0x89, 0x17, 0xc2, 0x9d, 0x2d, 0x43, 0xac, 0xa9, 0xcf, 0x79, 0x84, 0xf8, 0x08, 0x68, 0xb8, 0xf2, 0x2b, 0x78, 0xc6, 0xfc, 0x1f, 0xb2, 0xb2, 0x6e, 0x5e, 0xe1, 0xc4, 0x01, 0x55, 0x52, 0x62, 0x39, 0x66, 0xce, 0x3c, 0x5b, 0xbf, 0xa3, 0x06, 0x10, 0xa0, 0xa9] },
        PhVec { block_size: 64, data_len: 325, out_len: 64, customization: b"", expected: &[0x8a, 0x82, 0xa3, 0x0b, 0xfe, 0x19, 0xec, 0x09, 0xc1, 0x87, 0x1d, 0xfd, 0x81, 0x39, 0x24, 0x4a, 0x0d, 0x19, 0x29, 0x8f, 0x25, 0xc2, 0xcc, 0x1e, 0x55, 0x89, 0x13, 0xbc, 0xd5, 0xfe, 0x33, 0x52, 0x3e, 0x00, 0x7b, 0xf6, 0xf4, 0x4f, 0x71, 0xde, 0xda, 0xd1, 0xd4, 0xc7, 0xc4, 0xf1, 0xdf, 0x32, 0x2a, 0x74, 0x83, 0xd9, 0x2e, 0xf1, 0xac, 0xaf, 0xed, 0x25, 0xfa, 0xc7, 0xd1, 0x4f, 0x36, 0xad] },
        PhVec { block_size: 64, data_len: 129, out_len: 64, customization: b"Parallel Data", expected: &[0x47, 0xe7, 0x48, 0xa4, 0xc9, 0xc0, 0x76, 0x1f, 0x9a, 0xa5, 0xa8, 0x90, 0x82, 0x33, 0x83, 0x97, 0xb7, 0x31, 0x19, 0x26, 0x8d, 0x21, 0x57, 0xc3, 0x84, 0xdd, 0x1c, 0x50, 0x58, 0x42, 0xd7, 0xdb, 0xab, 0x60, 0xdc, 0xaa, 0x06, 0xfe, 0xa5, 0x26, 0x05, 0xdf, 0x74, 0xed, 0x04, 0x26, 0xf5, 0xd4, 0x9e, 0x46, 0x46, 0x7d, 0xb6, 0x39, 0xd2, 0x69, 0xc5, 0xe5, 0xbe, 0xe3, 0xb4, 0xca, 0xef, 0xc2] },
        PhVec { block_size: 136, data_len: 0, out_len: 64, customization: b"", expected: &[0xf5, 0x3a, 0xbe, 0x02, 0x82, 0xc5, 0xa0, 0xb2, 0xa0, 0x08, 0x33, 0x6a, 0x9e, 0xb9, 0xe8, 0x6f, 0x50, 0x5d, 0x7b, 0xd5, 0x6c, 0xd4, 0x65, 0xb2, 0xb7, 0x2f, 0xaa, 0x09, 0xc8, 0x38, 0x8f, 0x9d, 0x66, 0x81, 0xeb, 0x29, 0x95, 0x31, 0xf7, 0xff, 0xc7, 0xd3, 0xb7, 0xee, 0xdc, 0xf0, 0x85, 0xcd, 0xa4, 0x8f, 0xab, 0x64, 0x12, 0x92, 0x65, 0x86, 0x52, 0x23, 0x06, 0x52, 0x4b, 0x81, 0xdf, 0xd9] },
        PhVec { block_size: 136, data_len: 1, out_len: 64, customization: b"", expected: &[0x6f, 0x4b, 0xd0, 0xef, 0x22, 0xac, 0x21, 0x98, 0x80, 0xfb, 0x91, 0x8c, 0x63, 0x25, 0xba, 0x33, 0xd4, 0x33, 0xd5, 0xe9, 0x2f, 0xf0, 0x71, 0xb6, 0x8f, 0x2a, 0xe2, 0x2b, 0xbb, 0xba, 0x69, 0x74, 0x33, 0xd4, 0xce, 0xaa, 0x95, 0x95, 0xe6, 0x27, 0x9d, 0x8b, 0x83, 0x41, 0x93, 0xe8, 0xac, 0x0a, 0x03, 0x04, 0x77, 0x33, 0x1e, 0x00, 0xee, 0x94, 0x70, 0x1d, 0x2e, 0xec, 0x97, 0xc0, 0x0e, 0x9c] },
        PhVec { block_size: 136, data_len: 135, out_len: 64, customization: b"", expected: &[0x2d, 0x25, 0xfe, 0x19, 0x71, 0xaf, 0x86, 0x4d, 0xc6, 0x92, 0x8a, 0xd4, 0x96, 0xc2, 0x71, 0x5e, 0xf0, 0xda, 0x00, 0x51, 0x81, 0x9b, 0xd6, 0x89, 0x22, 0xbe, 0xb9, 0xc5, 0xde, 0xbb, 0x01, 0x6f, 0xc1, 0xf8, 0xc7, 0x6a, 0x16, 0x31, 0xd2, 0xc5, 0x40, 0x6e, 0xec, 0xa6, 0x36, 0xd4, 0x85, 0xd9, 0x95, 0x27, 0x86, 0xc8, 0xce, 0xbf, 0x0c, 0xc8, 0x7c, 0xe0, 0xea, 0x20, 0x55, 0x30, 0x49, 0xea] },
        PhVec { block_size: 136, data_len: 136, out_len: 64, customization: b"", expected: &[0x62, 0xf5, 0xac, 0x65, 0xd7, 0x0a, 0x64, 0x35, 0xb6, 0xd9, 0x02, 0xae, 0x4d, 0x63, 0x08, 0xd6, 0xeb, 0xbc, 0x60, 0x0f, 0x7f, 0xf2, 0xc8, 0x3e, 0xc1, 0x39, 0xbe, 0x2c, 0xbf, 0xbd, 0x73, 0x20, 0x0d, 0x2c, 0xf9, 0xb6, 0x48, 0x55, 0x88, 0xaf, 0x97, 0x19, 0xa9, 0x30, 0xb9, 0x8e, 0x95, 0xae, 0x7f, 0x1e, 0x34, 0x03, 0xb1, 0x93, 0x72, 0xd4, 0x25, 0x74, 0x38, 0xec, 0x2f, 0x2b, 0x07, 0x73] },
        PhVec { block_size: 136, data_len: 137, out_len: 64, customization: b"", expected: &[0x85, 0xc0, 0xcf, 0xa5, 0xf5, 0x51, 0x48, 0xfa, 0x2d, 0x9e, 0x4c, 0xd4, 0xfe, 0xf7, 0x3f, 0x31, 0x2f, 0xd0, 0x06, 0x46, 0xce, 0x47, 0xa6, 0x71, 0x53, 0x13, 0xee, 0xd3, 0x7f, 0xc0, 0xcf, 0x4c, 0xf0, 0x8a, 0xde, 0x52, 0x06, 0xda, 0x37, 0x4f, 0x77, 0x90, 0x0d, 0x77, 0xc0, 0xa9, 0xa6, 0x57, 0x56, 0xd5, 0xc8, 0x15, 0x57, 0xd8, 0x04, 0xda, 0x54, 0x2a, 0x2f, 0x9b, 0xf7, 0x95, 0xfb, 0xd7] },
        PhVec { block_size: 136, data_len: 272, out_len: 64, customization: b"", expected: &[0x19, 0xcf, 0x7c, 0x14, 0x78, 0xfe, 0x89, 0x25, 0x9e, 0x1b, 0xd6, 0x90, 0x31, 0x3f, 0xf2, 0x52, 0x9a, 0x0c, 0x94, 0x2e, 0x24, 0xbd, 0xe8, 0x2d, 0xed, 0xa6, 0x26, 0xf2, 0x1c, 0xa8, 0xcf, 0x27, 0x06, 0x33, 0x87, 0x63, 0xe3, 0x91, 0x31, 0x8b, 0x81, 0xf3, 0x86, 0x5d, 0x90, 0xdd, 0x2f, 0x4b, 0x3c, 0xd4, 0x9f, 0xea, 0xd2, 0x54, 0x79, 0x86, 0x55, 0xfb, 0xfc, 0x27, 0x37, 0x83, 0x6a, 0x61] },
        PhVec { block_size: 136, data_len: 273, out_len: 64, customization: b"", expected: &[0x27, 0xd0, 0x34, 0x38, 0xcc, 0x0c, 0x4f, 0xb0, 0x57, 0xbd, 0xae, 0x2c, 0xbb, 0x53, 0x11, 0xb9, 0xc0, 0xb0, 0x9f, 0x58, 0x45, 0xb4, 0x81, 0x53, 0x03, 0x5b, 0xad, 0x48, 0x42, 0x56, 0x09, 0xa9, 0x31, 0x59, 0x9d, 0x07, 0xa7, 0xae, 0x55, 0x3d, 0x6c, 0x87, 0xaa, 0xee, 0x22, 0xe4, 0x54, 0x99, 0xab, 0x43, 0xb8, 0x20, 0x14, 0x66, 0xa3, 0x4f, 0xd7, 0xe7, 0x3e, 0x3f, 0x06, 0x77, 0x5b, 0x8e] },
        PhVec { block_size: 136, data_len: 408, out_len: 64, customization: b"", expected: &[0xc0, 0xdf, 0xb9, 0x78, 0x65, 0x4e, 0xa3, 0x68, 0xd9, 0xf0, 0x7b, 0xed, 0x0d, 0xd6, 0xe6, 0x49, 0x45, 0x90, 0x5d, 0x24, 0xad, 0x3b, 0x9e, 0x8e, 0x81, 0xe9, 0x27, 0x57, 0xff, 0xd8, 0xc5, 0xf2, 0x27, 0x73, 0xc2, 0xc6, 0x57, 0xb6, 0x8a, 0xf3, 0xbf, 0x65, 0x19, 0x8f, 0xf5, 0x7c, 0x72, 0x42, 0x95, 0xe9, 0xf4, 0x85, 0x16, 0x3c, 0x7d, 0x81, 0x9d, 0x4d, 0x3d, 0x4c, 0xe6, 0x3c, 0x39, 0xe2] },
        PhVec { block_size: 136, data_len: 685, out_len: 64, customization: b"", expected: &[0x49, 0xd4, 0xaa, 0xd5, 0x6e, 0xd3, 0x2d, 0x78, 0x86, 0xd2, 0x7b, 0x35, 0x0c, 0x3f, 0xf1, 0x61, 0x79, 0x38, 0x04, 0x0e, 0x18, 0xad, 0x8c, 0xc7, 0x0b, 0xfe, 0x17, 0x1a, 0xcb, 0xc2, 0x3f, 0xbb, 0xc0, 0xdc, 0x67, 0x27, 0xf5, 0x36, 0xb7, 0x4c, 0x5e, 0x0f, 0xae, 0x7c, 0x0f, 0x0d, 0xb3, 0x21, 0xdd, 0x21, 0x1d, 0xd0, 0xd5, 0xea, 0x66, 0x92, 0xc8, 0xcf, 0x6a, 0xec, 0x68, 0x4d, 0xea, 0xbc] },
        PhVec { block_size: 136, data_len: 273, out_len: 64, customization: b"Parallel Data", expected: &[0x14, 0xbf, 0x58, 0x08, 0x1d, 0x45, 0x21, 0x40, 0x06, 0xb9, 0xf6, 0xaf, 0x1e, 0x6d, 0xaf, 0x43, 0x65, 0x99, 0x2b, 0x7e, 0x26, 0x63, 0x83, 0x5c, 0xab, 0xc1, 0x06, 0xbe, 0xab, 0xa5, 0xad, 0x60, 0x05, 0xa2, 0x30, 0x60, 0x41, 0x61, 0x01, 0x81, 0x70, 0x45, 0x28, 0x1d, 0xfa, 0xd2, 0x13, 0x73, 0xdb, 0x77, 0x63, 0xb2, 0x07, 0x97, 0xb0, 0xf3, 0x1e, 0x9a, 0xee, 0xe4, 0xc5, 0xb1, 0xa8, 0xb3] },
        PhVec { block_size: 168, data_len: 0, out_len: 64, customization: b"", expected: &[0xc0, 0xc9, 0x4d, 0x1a, 0x8f, 0x3c, 0xd3, 0x8d, 0xda, 0xab, 0x7d, 0x78, 0x8d, 0xd1, 0x2b, 0xba, 0x48, 0x24, 0xb1, 0xb0, 0x25, 0x8f, 0xc0, 0x24, 0x9d, 0xd6, 0x62, 0xba, 0x68, 0x0f, 0x30, 0x81, 0x33, 0x2a, 0x24, 0x11, 0x6d, 0x3e, 0xa3, 0x17, 0x44, 0x2f, 0x17, 0xa0, 0x65, 0x87, 0x28, 0xd8, 0x8b, 0x13, 0x16, 0xf9, 0x42, 0xa9, 0x36, 0x55, 0xc8, 0x68, 0x1d, 0xfb, 0xe4, 0xac, 0x52, 0x95] },
        PhVec { block_size: 168, data_len: 1, out_len: 64, customization: b"", expected: &[0x73, 0x5a, 0x74, 0x31, 0xa0, 0x73, 0x8c, 0xc2, 0x93, 0x99, 0xbe, 0xdf, 0xf1, 0x51, 0x4b, 0x24, 0xe9, 0x73, 0x92, 0xe4, 0xc7, 0x2d, 0xb0, 0x90, 0x71, 0x58, 0x74, 0x63, 0x56, 0x88, 0x3a, 0xa9, 0x29, 0x27, 0xe7, 0xde, 0xfb, 0xa4, 0x80, 0xa0, 0xed, 0xa5, 0x53, 0x7c, 0xf2, 0xd3, 0x7a, 0x51, 0xf5, 0xbc, 0x85, 0xc1, 0x5b, 0xb5, 0xea, 0x3f, 0x53, 0x5c, 0xed, 0x88, 0x93, 0x7b, 0x2f, 0x43] },
        PhVec { block_size: 168, data_len: 167, out_len: 64, customization: b"", expected: &[0xf1, 0x61, 0xef, 0x48, 0x04, 0x03, 0x5d, 0xe6, 0xa4, 0x95, 0x52, 0x4e, 0x23, 0xd6, 0xc4, 0x38, 0x5b, 0x93, 0x6f, 0xb6, 0x4f, 0xd3, 0xb1, 0x25, 0x38, 0x04, 0x66, 0x50, 0x45, 0x0e, 0x5b, 0xca, 0xc3, 0x54, 0xf6, 0xd4, 0xb8, 0x79, 0xec, 0x42, 0x2a, 0x97, 0x5d, 0xf1, 0xa0, 0x4b, 0x6c, 0x53, 0x5e, 0x29, 0xcb, 0x07, 0xfe, 0x21, 0x61, 0x97, 0xac, 0x72, 0x29, 0x73, 0x4a, 0x3d, 0x38, 0x3c] },
        PhVec { block_size: 168, data_len: 168, out_len: 64, customization: b"", expected: &[0xeb, 0x9e, 0x9f, 0x3a, 0x10, 0x2b, 0x10, 0x5f, 0xb2, 0x74, 0x8c, 0x2a, 0x65, 0x81, 0x1e, 0xe6, 0xba, 0xc5, 0xa3, 0x9a, 0x09, 0x06, 0x7f, 0x07, 0xc0, 0xe9, 0xd0, 0x50, 0x4e, 0x27, 0x1f, 0xbb, 0x0a, 0x1f, 0x08, 0x0b, 0x1b, 0x41, 0x72, 0x64, 0x7c, 0xdd, 0x48, 0xb0, 0x43, 0x4a, 0x40, 0xe1, 0x98, 0x42, 0xc6, 0x54, 0x88, 0x7c, 0xe6, 0x6f, 0x3c, 0x1a, 0x6f, 0x09, 0xc5, 0xa5, 0x9a, 0xff] },
        PhVec { block_size: 168, data_len: 169, out_len: 64, customization: b"", expected: &[0x82, 0xa4, 0x55, 0x6b, 0xed, 0x3c, 0x71, 0x8a, 0x5e, 0x01, 0x55, 0xfc, 0xb3, 0x22, 0xfb, 0x9b, 0xb1, 0x5d, 0x2c, 0x61, 0xab, 0x2c, 0x43, 0xf1, 0xc2, 0x6c, 0x7f, 0x1c, 0x2e, 0xbf, 0xa7, 0x00, 0x38, 0x18, 0xee, 0xd3, 0x3a, 0xac, 0x9a, 0xf4, 0xa7, 0x75, 0xb4, 0x74, 0xa7, 0xde, 0xc9, 0x90, 0x9b, 0x9d, 0xc7, 0x55, 0x5f, 0x5a, 0xf1, 0xe9, 0x0d, 0x0d, 0x93, 0x1a, 0xbc, 0x5d, 0x6e, 0xd1] },
        PhVec { block_size: 168, data_len: 336, out_len: 64, customization: b"", expected: &[0xce, 0x59, 0x1c, 0x41, 0xf6, 0x56, 0x72, 0x87, 0x2d, 0xc1, 0xca, 0xaa, 0x8a, 0x4b, 0xeb, 0x98, 0x33, 0xb7, 0x41, 0xc2, 0xd3, 0x54, 0xd7, 0xc7, 0x2e, 0x0e, 0x12, 0x85, 0x1b, 0x2b, 0x72, 0x62, 0xa1, 0xf9, 0x07, 0xa3, 0xa4, 0x5b, 0x42, 0xe6, 0x25, 0xf8, 0xa6, 0xa6, 0x55, 0x81, 0x10, 0xfc, 0x1e, 0xb2, 0x86, 0xc8, 0x2e, 0x36, 0x48, 0xcc, 0xee, 0xee, 0x4b, 0x83, 0xd6, 0xbc, 0x7c, 0x53] },
        PhVec { block_size: 168, data_len: 337, out_len: 64, customization: b"", expected: &[0xeb, 0xa6, 0x86, 0xf3, 0xc3, 0x09, 0x16, 0x37, 0x16, 0xa4, 0x1a, 0x26, 0xeb, 0xfe, 0xdf, 0xdc, 0x6e, 0x1e, 0x23, 0x65, 0xae, 0x4c, 0x27, 0x8d, 0x47, 0x60, 0x06, 0xdd, 0x5d, 0x43, 0x0d, 0xd1, 0x89, 0xfd, 0x67, 0xc7, 0xf6, 0x8f, 0xf6, 0xa5, 0x82, 0x1d, 0x7d, 0x61, 0xa2, 0x20, 0x8c, 0x5a, 0x6f, 0xba, 0x95, 0xad, 0x1e, 0x24, 0x90, 0x6d, 0x13, 0x9e, 0x03, 0xa2, 0x80, 0xd9, 0x78, 0x99] },
        PhVec { block_size: 168, data_len: 504, out_len: 64, customization: b"", expected: &[0x42, 0x70, 0xcc, 0xc8, 0xb3, 0x3a, 0x9d, 0x04, 0x79, 0x01, 0x81, 0x2a, 0x0a, 0x54, 0x0c, 0xba, 0xf5, 0xf2, 0xb4, 0x24, 0x38, 0xbf, 0x06, 0xae, 0x1f, 0xc3, 0xef, 0x94, 0xe4, 0xea, 0xa9, 0x63, 0xd0, 0xec, 0x77, 0xfa, 0x3c, 0x8c, 0x79, 0x7f, 0x35, 0x7e, 0x3b, 0xd6, 0x9b, 0x06, 0xf5, 0x7d, 0xf9, 0x67, 0x38, 0xc3, 0x37, 0xd5, 0x6e, 0xa6, 0x03, 0xf7, 0x88, 0x1b, 0x8f, 0xa7, 0x19, 0x38] },
        PhVec { block_size: 168, data_len: 845, out_len: 64, customization: b"", expected: &[0xf6, 0xc6, 0x24, 0xe4, 0xfb, 0x33, 0x75, 0xab, 0xac, 0x45, 0x9e, 0xeb, 0xf5, 0xaf, 0x70, 0x1d, 0x40, 0xd5, 0x5e, 0x73, 0xa1, 0xb1, 0x16, 0x69, 0xff, 0x0f, 0x4e, 0x7d, 0x0f, 0x64, 0x99, 0x55, 0x49, 0x5e, 0xb4, 0xcb, 0x35, 0x57, 0xf0, 0x45, 0xf2, 0x26, 0x9d, 0x42, 0xee, 0x94, 0xff, 0x5e, 0x2d, 0x71, 0x43, 0xef, 0x59, 0x34, 0x31, 0x59, 0x2c, 0x39, 0x4c, 0x9a, 0x32, 0x41, 0x6c, 0x23] },
        PhVec { block_size: 168, data_len: 337, out_len: 64, customization: b"Parallel Data", expected: &[0xb6, 0x55, 0xf5, 0x09, 0x69, 0x06, 0xda, 0x1b, 0x6c, 0x32, 0x79, 0x3d, 0xf0, 0xd0, 0xb0, 0x2b, 0x88, 0xff, 0x09, 0x00, 0x52, 0xec, 0x2e, 0x58, 0x29, 0x06, 0x99, 0x08, 0x4c, 0x54, 0xa6, 0x9b, 0xa3, 0xe0, 0x45, 0xb4, 0x41, 0x36, 0x36, 0x4a, 0x1d, 0x3f, 0xcd, 0x31, 0x1a, 0x87, 0x57, 0x14, 0x20, 0xdd, 0x3b, 0x88, 0xf1, 0xd9, 0x79, 0xd7, 0xcb, 0x39, 0x0e, 0x15, 0x0f, 0x5d, 0x25, 0x33] },
        PhVec { block_size: 1024, data_len: 0, out_len: 64, customization: b"", expected: &[0xf3, 0xcd, 0xe9, 0x8b, 0xec, 0xd1, 0xa0, 0xee, 0x2b, 0x26, 0x01, 0xe4, 0xd2, 0xd0, 0x27, 0xd3, 0xef, 0xe0, 0xf8, 0xeb, 0xd5, 0x23, 0x10, 0x41, 0xbd, 0x38, 0xf3, 0x26, 0x03, 0x6d, 0x0d, 0x37, 0xc3, 0x72, 0xea, 0x2e, 0x2e, 0x7a, 0x79, 0x5b, 0x12, 0x5f, 0xcd, 0x4e, 0xe0, 0x56, 0xf3, 0xc6, 0x60, 0xc5, 0x01, 0x57, 0x87, 0x4b, 0x2c, 0xf3, 0xff, 0x4c, 0x0d, 0x4e, 0xb6, 0x03, 0x1e, 0x46] },
        PhVec { block_size: 1024, data_len: 1, out_len: 64, customization: b"", expected: &[0x83, 0x3c, 0xdb, 0xae, 0xae, 0x96, 0x29, 0x24, 0xeb, 0x30, 0x61, 0x6a, 0xd9, 0xe0, 0x8d, 0x9f, 0x05, 0x0a, 0x71, 0xf9, 0x2e, 0x13, 0xab, 0x5a, 0x0e, 0x1f, 0x2a, 0xf1, 0xec, 0xf3, 0x86, 0xb8, 0xef, 0xf3, 0x5d, 0x7b, 0xa9, 0x5c, 0x9b, 0xbe, 0xb2, 0x60, 0x2f, 0xf0, 0x45, 0x52, 0x5f, 0xaa, 0x0d, 0xd2, 0x6b, 0x79, 0x96, 0x6f, 0xd4, 0xeb, 0xf2, 0x51, 0xed, 0x7b, 0x74, 0x83, 0x6f, 0x30] },
        PhVec { block_size: 1024, data_len: 1023, out_len: 64, customization: b"", expected: &[0xf1, 0x12, 0x74, 0x28, 0xf4, 0xc0, 0xd3, 0x31, 0xa8, 0xfb, 0x7e, 0xf8, 0x54, 0x78, 0x68, 0xb0, 0x52, 0xf5, 0x18, 0x86, 0x75, 0xab, 0x31, 0x85, 0xf7, 0xeb, 0xdc, 0x12, 0x76, 0xe7, 0xa3, 0x69, 0x1d, 0xc0, 0xff, 0x23, 0x87, 0x4c, 0x0a, 0x8b, 0xa0, 0x19, 0x80, 0x60, 0x2c, 0x73, 0xb9, 0x7d, 0xb8, 0x12, 0x79, 0x5a, 0x13, 0xed, 0x4c, 0xea, 0xb6, 0xbe, 0x0e, 0x17, 0xf9, 0xb2, 0xe2, 0xf2] },
        PhVec { block_size: 1024, data_len: 1024, out_len: 64, customization: b"", expected: &[0xfe, 0x9d, 0x44, 0xc7, 0x6e, 0x1a, 0x99, 0x0d, 0x10, 0x11, 0x74, 0xfc, 0x5b, 0xd2, 0x71, 0x3b, 0x40, 0xd9, 0x24, 0xfb, 0x55, 0xc1, 0x7a, 0xd7, 0x63, 0xec, 0x91, 0x15, 0x76, 0x35, 0xff, 0x9e, 0xd7, 0x7f, 0xc6, 0x69, 0x2f, 0xbc, 0x44, 0xe5, 0x0a, 0x3a, 0x59, 0x27, 0x10, 0x94, 0xa0, 0x29, 0xe2, 0xce, 0xf5, 0xeb, 0x23, 0x3b, 0x20, 0x9e, 0x4f, 0x33, 0x95, 0x04, 0x58, 0x59, 0x31, 0x0d] },
        PhVec { block_size: 1024, data_len: 1025, out_len: 64, customization: b"", expected: &[0x37, 0x30, 0x8a, 0x59, 0x76, 0x0b, 0x6f, 0xbc, 0x0e, 0xbd, 0x28, 0xb7, 0xfc, 0x10, 0x9a, 0x23, 0x68, 0x28, 0x87, 0xda, 0x7c, 0xb8, 0xe6, 0x9c, 0x25, 0xf5, 0x1c, 0xe2, 0xb7, 0x8f, 0xbe, 0xd9, 0x6a, 0xc8, 0xe8, 0x91, 0xf6, 0x6b, 0x51, 0xf6, 0xb7, 0x02, 0x68, 0x51, 0x9a, 0x0e, 0xf3, 0xc4, 0x43, 0xbd, 0x45, 0x83, 0x20, 0xe9, 0x4b, 0x8e, 0xcf, 0x34, 0x79, 0x50, 0xdc, 0x4d, 0xa4, 0x74] },
        PhVec { block_size: 1024, data_len: 2048, out_len: 64, customization: b"", expected: &[0x2f, 0xa8, 0xed, 0x7e, 0x78, 0x23, 0x52, 0x83, 0xf4, 0x6d, 0x09, 0xe3, 0xa2, 0x4d, 0xda, 0xdd, 0x64, 0x56, 0xc0, 0x3e, 0x19, 0x07, 0x68, 0x49, 0xeb, 0xa6, 0x5e, 0xde, 0xe1, 0xba, 0xf3, 0x8e, 0x73, 0xb2, 0x1c, 0xd6, 0xf0, 0x45, 0xa6, 0x30, 0xb0, 0xac, 0x13, 0x32, 0xc5, 0xde, 0x9f, 0x8e, 0xb0, 0x39, 0xb2, 0xe1, 0xf5, 0x8b, 0x21, 0xb0, 0x52, 0x7c, 0xe2, 0xbd, 0x0a, 0xd3, 0x7f, 0x28] },
        PhVec { block_size: 1024, data_len: 2049, out_len: 64, customization: b"", expected: &[0x1b, 0xe9, 0xbe, 0xd4, 0x49, 0xc8, 0xed, 0x66, 0x55, 0xe1, 0x40, 0x2a, 0x52, 0xac, 0x5b, 0x6d, 0x70, 0x22, 0xa8, 0xab, 0x93, 0x39, 0xef, 0xf1, 0x16, 0xfd, 0x6e, 0x96, 0x03, 0xbe, 0x0b, 0xcc, 0xe3, 0x56, 0xa4, 0xfc, 0x5d, 0x17, 0x64, 0x0f, 0x6a, 0xbf, 0xe2, 0xfd, 0x3d, 0xfa, 0x41, 0x00, 0xe3, 0xbd, 0xa8, 0x54, 0x18, 0x45, 0x5b, 0x9f, 0xad, 0x8f, 0xec, 0xe3, 0x1f, 0xa5, 0xc9, 0x0d] },
        PhVec { block_size: 1024, data_len: 3072, out_len: 64, customization: b"", expected: &[0x7d, 0x8f, 0x2b, 0xaf, 0xb7, 0xd6, 0xa4, 0x5a, 0xb6, 0xe9, 0xe5, 0x42, 0x26, 0xbe, 0x63, 0xbc, 0xf1, 0xc3, 0xfd, 0x11, 0x30, 0x75, 0x15, 0x3a, 0x82, 0x56, 0xb5, 0x56, 0xda, 0x43, 0xaa, 0x90, 0xc2, 0xe6, 0xe0, 0x79, 0x28, 0x74, 0x9b, 0x04, 0x46, 0x30, 0xc7, 0x4a, 0x75, 0x5d, 0x32, 0xd1, 0xcc, 0x4b, 0x23, 0x13, 0xbd, 0xcc, 0xf1, 0x54, 0x97, 0x5a, 0x25, 0xb9, 0xfa, 0xbc, 0x63, 0x9d] },
        PhVec { block_size: 1024, data_len: 5125, out_len: 64, customization: b"", expected: &[0x2f, 0x28, 0xce, 0x37, 0xb2, 0xde, 0x7b, 0xf5, 0x15, 0x6c, 0x1b, 0xcb, 0x00, 0x06, 0xa1, 0xda, 0xda, 0xfd, 0xe8, 0x1f, 0x84, 0x7a, 0xc0, 0xad, 0x38, 0x9d, 0xeb, 0x48, 0x63, 0x3e, 0x9d, 0xac, 0x21, 0x67, 0xf6, 0x83, 0x71, 0x1f, 0x60, 0xa4, 0x95, 0xb7, 0x43, 0xa6, 0x95, 0x69, 0x3d, 0x3f, 0x6d, 0xae, 0x60, 0xdf, 0xc5, 0x3e, 0x05, 0x5d, 0x06, 0xbb, 0x42, 0xcb, 0x0c, 0x69, 0x9b, 0xc3] },
        PhVec { block_size: 1024, data_len: 2049, out_len: 64, customization: b"Parallel Data", expected: &[0x81, 0x09, 0x57, 0xa0, 0xe8, 0x58, 0xdf, 0x29, 0xdf, 0x26, 0x68, 0xc3, 0x5f, 0x3a, 0xd2, 0xb7, 0x5e, 0x7f, 0x53, 0x60, 0x24, 0x25, 0x7d, 0xae, 0xa0, 0x27, 0xba, 0x74, 0x29, 0x22, 0x17, 0xd8, 0xb8, 0x73, 0x1e, 0x7b, 0x71, 0x8c, 0xca, 0x4d, 0x8c, 0x78, 0xba, 0x43, 0x0d, 0xa9, 0xdd, 0x07, 0xa7, 0xa7, 0x49, 0xe9, 0x35, 0x9e, 0xa8, 0x4a, 0xf4, 0x9e, 0xda, 0x99, 0xc1, 0xe3, 0x92, 0x01] },
    ];

    #[rustfmt::skip]
    pub(crate) const PH_XOF_VECTORS: &[PhVec] = &[
        PhVec { block_size: 1, data_len: 3, out_len: 64, customization: b"", expected: &[0x4d, 0xd6, 0x20, 0x42, 0x6a, 0xb8, 0xb2, 0xc9, 0x01, 0x33, 0xce, 0x9b, 0xc4, 0x25, 0x14, 0xb5, 0x32, 0x93, 0x3a, 0x3a, 0x30, 0xdd, 0x36, 0x47, 0xe2, 0x70, 0x2c, 0x06, 0x86, 0x1c, 0xad, 0xf6, 0x11, 0x69, 0xa4, 0x49, 0xa0, 0x60, 0xb8, 0xb1, 0xcb, 0xd5, 0xa8, 0x75, 0x8f, 0x2b, 0x07, 0x42, 0xdb, 0xa5, 0x37, 0x4b, 0xd1, 0x3e, 0xcd, 0x1d, 0x4b, 0xe2, 0x88, 0xb5, 0x55, 0x5c, 0x83, 0xf2] },
        PhVec { block_size: 8, data_len: 24, out_len: 64, customization: b"", expected: &[0xe1, 0x17, 0xe5, 0xb6, 0x01, 0xe5, 0x03, 0x99, 0xec, 0x35, 0x0e, 0x6f, 0x0b, 0xec, 0x94, 0xff, 0xc8, 0xa0, 0xfa, 0xc5, 0xc3, 0x29, 0x43, 0xa1, 0xd3, 0xe0, 0x11, 0x1c, 0x0b, 0x64, 0x76, 0x1b, 0x7c, 0x54, 0xa5, 0x15, 0x05, 0xa1, 0x86, 0xe0, 0x88, 0x1d, 0xc9, 0xec, 0xe2, 0x7b, 0x83, 0xf5, 0x36, 0x1a, 0xe1, 0xb2, 0x1f, 0x96, 0x51, 0x94, 0x06, 0x64, 0x75, 0xe0, 0x84, 0x6c, 0x3b, 0xaa] },
        PhVec { block_size: 16, data_len: 40, out_len: 64, customization: b"Parallel Data", expected: &[0xf5, 0x54, 0xc2, 0x42, 0xbe, 0x4e, 0x55, 0x02, 0xb2, 0x6b, 0x3f, 0xdd, 0x39, 0x7d, 0x43, 0xee, 0x79, 0x64, 0xb6, 0x18, 0xf0, 0x85, 0x55, 0x7f, 0xf9, 0xc4, 0x36, 0xce, 0xea, 0x9a, 0x5d, 0xa8, 0x04, 0x57, 0xef, 0x1d, 0x28, 0x8e, 0xf0, 0xc5, 0x5a, 0x89, 0x6e, 0x9b, 0x91, 0xbf, 0x9f, 0x9f, 0xd7, 0x81, 0x37, 0xee, 0xda, 0x8c, 0x34, 0xff, 0x44, 0xdf, 0x2a, 0xe9, 0xb4, 0x94, 0xdd, 0x5d] },
        PhVec { block_size: 136, data_len: 300, out_len: 64, customization: b"", expected: &[0x4d, 0xdc, 0x9b, 0x80, 0x8e, 0x1a, 0x68, 0xa1, 0x93, 0x8e, 0xcb, 0x18, 0xd0, 0xa1, 0x89, 0xf4, 0x51, 0x53, 0x37, 0x1d, 0x7a, 0x54, 0xc0, 0x28, 0x3a, 0x3d, 0xc5, 0x16, 0x2e, 0x95, 0xe6, 0x41, 0x2c, 0x8d, 0xf4, 0x57, 0x93, 0xd5, 0xdd, 0xa9, 0x7c, 0xc1, 0x90, 0xcb, 0x51, 0x5d, 0x78, 0xbe, 0xb1, 0x10, 0x54, 0x68, 0x1f, 0x9d, 0x1f, 0xf9, 0x70, 0x07, 0x0f, 0xa9, 0x74, 0xd2, 0xf8, 0x58] },
        PhVec { block_size: 1024, data_len: 5000, out_len: 64, customization: b"", expected: &[0xa9, 0xef, 0xc3, 0xa0, 0x91, 0x92, 0xa9, 0xa7, 0x4c, 0xd7, 0x36, 0xc4, 0x2c, 0x97, 0xf8, 0x53, 0x6e, 0xaf, 0xaf, 0xa1, 0x18, 0x6d, 0x47, 0x2f, 0x9b, 0x7b, 0x6a, 0x74, 0xe3, 0xa6, 0xa0, 0xa8, 0xd5, 0x53, 0x2e, 0x31, 0x47, 0xf9, 0x84, 0xcf, 0x3e, 0xcd, 0x3d, 0x1d, 0x88, 0x95, 0xf0, 0x31, 0xa0, 0x66, 0x8f, 0x2f, 0x28, 0x9b, 0x3f, 0x17, 0xac, 0x48, 0xd5, 0x5a, 0x9f, 0x4e, 0x2b, 0x35] },
    ];

    #[test]
    fn reference_vectors() {
        for (i, v) in PH_VECTORS.iter().enumerate() {
            let data = pattern_data(v.data_len);
            let mut out = std::vec![0u8; v.out_len];
            ParallelHash256::hash(&data, v.block_size, v.customization, &mut out);
            assert_eq!(out, v.expected, "vector {i}: B={} len={}", v.block_size, v.data_len);
        }
    }

    #[test]
    fn reference_xof_vectors() {
        for (i, v) in PH_XOF_VECTORS.iter().enumerate() {
            let data = pattern_data(v.data_len);
            let mut out = std::vec![0u8; v.out_len];
            let mut h = ParallelHash256::new(v.block_size, v.customization);
            h.update(&data);
            h.finalize_xof().squeeze(&mut out);
            assert_eq!(out, v.expected, "XOF vector {i}: B={} len={}", v.block_size, v.data_len);
        }
    }

    #[test]
    fn xof_squeeze_is_chunk_independent() {
        let data = pattern_data(3 * 1024 + 7);

        let mut one_shot = [0u8; 96];
        let mut h = ParallelHash256::new(1024, b"xof");
        h.update(&data);
        h.finalize_xof().squeeze(&mut one_shot);

        let mut chunked = [0u8; 96];
        let mut h = ParallelHash256::new(1024, b"xof");
        h.update(&data);
        let mut reader = h.finalize_xof();
        let mut off = 0;
        for chunk in [1usize, 7, 13, 32, 43] {
            reader.squeeze(&mut chunked[off..off + chunk]);
            off += chunk;
        }
        reader.squeeze(&mut chunked[off..]);

        assert_eq!(one_shot, chunked);
    }

    /// Width of the leaf sponge selected on this CPU.
    fn batch_width() -> usize {
        Leaves::new(Backend::detect()).width()
    }

    #[test]
    fn streaming_matches_one_shot() {
        let batch = batch_width();
        for &block_size in &[1usize, 8, 12, 136, 168, 1024] {
            for &len in &[
                0usize,
                1,
                block_size - 1,
                block_size,
                block_size + 1,
                2 * block_size,
                2 * block_size + 1,
                3 * block_size,
                5 * block_size + 5,
                batch * block_size,
                batch * block_size + 1,
            ] {
                let data = pattern_data(len);

                let mut one_shot = [0u8; 64];
                ParallelHash256::hash(&data, block_size, b"stream", &mut one_shot);

                // Feed awkward chunk sizes: `1` and `block_size` force the
                // serial path, `batch * block_size + 1` the batched path, and
                // the final chunk exercises the queue.
                for &chunk_size in &[1usize, 3, 7, block_size, 2 * block_size + 1, batch * block_size + 1] {
                    let mut h = ParallelHash256::new(block_size, b"stream");
                    for chunk in data.chunks(chunk_size) {
                        h.update(chunk);
                    }
                    let mut streamed = [0u8; 64];
                    h.finalize_into(&mut streamed);
                    assert_eq!(one_shot, streamed, "B={block_size} len={len} chunk={chunk_size}");
                }
            }
        }
    }

    /// The batched SIMD path must produce exactly the same result as hashing
    /// one block at a time with the plain one-way SHAKE. The one-shot `hash`
    /// buffers a whole slice in a single `update` (so it takes the batched
    /// path), whereas feeding one block per `update` never has enough data to
    /// batch (so it takes the sequential path).
    #[test]
    fn batched_path_matches_sequential_reference() {
        let mut rng = 0x6f4a_11e2_9d38_b705u64;
        let batch = batch_width();
        for &block_size in &[1usize, 8, 63, 136, 137, 1024] {
            for &len in &[
                0usize,
                1,
                block_size,
                block_size + 1,
                2 * block_size,
                3 * block_size,
                3 * block_size + 5,
                batch * block_size,
                batch * block_size + 7,
            ] {
                let data = random_bytes(len, &mut rng);

                let mut batched = [0u8; 64];
                ParallelHash256::hash(&data, block_size, b"ref", &mut batched);

                let mut sequential = [0u8; 64];
                {
                    let mut h = ParallelHash256::new(block_size, b"ref");
                    for block in data.chunks(block_size) {
                        h.update(block);
                    }
                    h.finalize_into(&mut sequential);
                }

                assert_eq!(batched, sequential, "B={block_size} len={len}");
            }
        }
    }

    /// SP 800-185 single-leaf construction for `data.len() < block_size`:
    /// `cSHAKE(left_encode(B) || SHAKE256(data, 64) || right_encode(1) ||
    /// right_encode(L), L, "ParallelHash", S)`.
    fn spec_single_leaf(data: &[u8], block_size: usize, customization: &[u8]) -> [u8; 64] {
        let mut z = std::vec::Vec::new();
        z.extend_from_slice(left_encode(block_size).as_ref());
        let mut cv = [0u8; 64];
        Shake256::hash(data, &mut cv);
        z.extend_from_slice(&cv);
        z.extend_from_slice(right_encode(1).as_ref());
        z.extend_from_slice(right_encode(64 * 8).as_ref());

        let mut out = [0u8; 64];
        crate::sha3::CShake256::hash(&z, PARALLEL_HASH_NAME, customization, &mut out);
        return out;
    }

    #[test]
    fn block_size_larger_than_input_is_a_single_leaf() {
        let data = pattern_data(100);
        for &block_size in &[101usize, 1024, 1 << 20, usize::MAX / 2, usize::MAX - 1, usize::MAX] {
            let mut out = [0u8; 64];
            ParallelHash256::hash(&data, block_size, b"leaf", &mut out);
            assert_eq!(out, spec_single_leaf(&data, block_size, b"leaf"), "B={block_size}");
        }
    }

    #[test]
    fn huge_block_size_does_not_panic() {
        // Regression test: `2 * block_size` used to overflow for large block
        // sizes, panicking in debug (multiply overflow) and release tests
        // (`split_at`, "mid > len"). These must hash as a single leaf instead.
        for &block_size in &[1usize << 62, 1 << 63, usize::MAX] {
            let mut out = [0u8; 64];
            ParallelHash256::hash(b"hello world", block_size, b"", &mut out);
            assert_ne!(out, [0u8; 64], "B={block_size}");
        }
    }

    #[test]
    fn zero_length_output_is_allowed() {
        // A zero-byte request binds `L = 0` and writes nothing.
        let mut out = [0u8; 0];
        ParallelHash256::hash(b"data", 8, b"", &mut out);

        // The XOF stream must still be well defined and match the same prefix.
        let mut one_byte = [0u8; 1];
        let mut h = ParallelHash256::new(8, b"");
        h.update(b"data");
        h.finalize_xof().squeeze(&mut one_byte);
        assert_ne!(one_byte, [0u8]);
    }

    #[test]
    fn customization_is_bound_into_the_hash() {
        let data = pattern_data(200);
        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        ParallelHash256::hash(&data, 16, b"one", &mut a);
        ParallelHash256::hash(&data, 16, b"two", &mut b);
        assert_ne!(a, b);
    }

    #[test]
    fn clone_mid_stream_matches_linear() {
        // BouncyCastle's `testClone`: cloning a partially fed hasher must give
        // the same result as feeding the remaining input to the original.
        let data = pattern_data(3 * 1024 + 7);

        let mut a = [0u8; 64];
        ParallelHash256::hash(&data, 12, b"Parallel Data", &mut a);

        let mut h = ParallelHash256::new(12, b"Parallel Data");
        h.update(&data[..500]);
        let mut clone = h.clone();
        h.update(&data[500..]);
        clone.update(&data[500..]);

        let mut from_original = [0u8; 64];
        let mut from_clone = [0u8; 64];
        h.finalize_into(&mut from_original);
        clone.finalize_into(&mut from_clone);

        assert_eq!(from_original, a);
        assert_eq!(from_clone, a);
    }

    #[test]
    fn official_xof_vectors() {
        // NIST SP 800-185 XOF sample (also in BouncyCastle's `testEmpty`):
        // empty message, B = 62, an awkward customization string and a 16-bit
        // (2-byte) output. The XOF form binds `L = 0`.
        let customization = b"Ny0LL2tUmt<+kuN5:Z7pZ_7]R; l/i:%pWbo4}";
        let mut h = ParallelHash256::new(62, customization);
        h.update(&[]);
        let mut out = [0u8; 2];
        h.finalize_xof().squeeze(&mut out);
        assert_eq!(hex::encode(out), "13c4");

        // BouncyCastle's long XOF vector for ParallelHash256, B = 12.
        let data = NIST_DATA_72;
        let mut h = ParallelHash256::new(12, b"Parallel Data");
        h.update(&data);
        let mut out = [0u8; 64];
        h.finalize_xof().squeeze(&mut out);
        assert_eq!(
            hex::encode(out),
            "6b3e790b330c889a204c2fbc728d809f19367328d852f4002dc829f73afd6bce\
             fb7fe5b607b13a801c0be5c1170bdb794e339458fdb0e62a6af3d42558970249"
        );
    }

    #[test]
    fn output_length_is_bound_into_the_hash() {
        let data = pattern_data(300);
        let mut short = [0u8; 32];
        let mut long = [0u8; 64];

        ParallelHash256::hash(&data, 16, b"", &mut short);
        ParallelHash256::hash(&data, 16, b"", &mut long);

        // Different requested lengths must give unrelated outputs, not a
        // truncation of one another.
        assert_ne!(short[..], long[..32]);
    }

    #[test]
    fn empty_input_has_zero_blocks() {
        // `n = ceil(0 / B) = 0`, confirmed by XKCP, BouncyCastle and
        // tiny-keccak. The value below comes from the independent
        // PyCryptodome/pure-Python reference and is the n = 0 case
        // `cSHAKE(left_encode(B) || right_encode(0) || right_encode(L), L,
        //        "ParallelHash", S)`.
        let mut out = [0u8; 64];
        ParallelHash256::hash(&[], 1024, b"", &mut out);
        assert_eq!(
            hex::encode(out),
            "f3cde98becd1a0ee2b2601e4d2d027d3efe0f8ebd5231041bd38f326036d0d37\
             c372ea2e2e7a795b125fcd4ee056f3c660c50157874b2cf3ff4c0d4eb6031e46"
        );
    }

    #[test]
    #[should_panic(expected = "block size must be greater than zero")]
    fn zero_block_size_is_rejected() {
        let _ = ParallelHash256::new(0, b"");
    }

    // Keep the imported bytecode-table names used by the NEON fallback in sync
    // with the scalar reference.
    #[test]
    fn rho_pi_tables_are_consistent() {
        assert_eq!(RHO.len(), 24);
        assert_eq!(PI.len(), 24);
    }
}
