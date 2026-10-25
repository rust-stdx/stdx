//! Keccak-p[1600] for a single state on x86-64.
//!
//! Two kernels live here, selected once per sponge by `PermuteBackend`:
//!
//! | Backend    | State layout                              | Instructions/round |
//! |------------|-------------------------------------------|--------------------|
//! | AVX2       | 25 lanes in 7 YMM, CRYPTOGAMS order       | 108                |
//! | AVX-512F   | 5 rows in 5 ZMM, 5 lanes each             | ~40                |
//!
//! The two are ports of the CRYPTOGAMS/OpenSSL AVX2 kernel and the Keccak
//! team's two-rounds-per-iteration AVX-512 kernel. They both apply
//! Keccak-p[1600, `ROUNDS`] to the same plain `[u64; 25]` state; only the AVX2
//! submodule reshuffles it internally, and it undoes that on entry and exit, so
//! callers never see the difference.
//!
//! Neither instruction set is baseline on x86-64, so each function may only be
//! called after the matching `std::arch::is_x86_feature_detected!` check, or
//! when the feature is enabled at compile time.
#![allow(unsafe_op_in_unsafe_fn)]

/// The Keccak-p[`ROUNDS`] permutation of one 1600-bit state, single-state AVX2.
///
/// # Safety
/// The CPU must implement AVX2.
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
pub(crate) use avx2::p1600_avx2;
/// The Keccak-p[`ROUNDS`] permutation of one 1600-bit state, single-state
/// AVX-512F.
///
/// # Safety
/// The CPU must implement AVX-512F.
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
pub(crate) use avx512::p1600_avx512;

#[cfg(feature = "std")]
use super::keccak::PermuteBackend;
use super::keccak::ROUND_CONSTANTS;

/// Process-wide cache of the backend selected for this x86-64 CPU.
#[cfg(feature = "std")]
static BACKEND: std::sync::OnceLock<PermuteBackend> = std::sync::OnceLock::new();

/// Detects the single-state backend once per process.
///
/// The expensive vendor/family `cpuid` probe therefore runs at most once
/// instead of once per sponge.
#[cfg(feature = "std")]
#[inline]
pub(crate) fn detect_amd64() -> PermuteBackend {
    return *BACKEND.get_or_init(detect_amd64_uncached);
}

#[cfg(feature = "std")]
fn detect_amd64_uncached() -> PermuteBackend {
    if std::arch::is_x86_feature_detected!("avx512f") {
        return PermuteBackend::Avx512;
    }
    if !std::arch::is_x86_feature_detected!("avx2") {
        return PermuteBackend::Soft;
    }

    let (vendor, family) = cpu_vendor_and_family();
    return select_amd64_backend(vendor, family, false, true);
}

#[cfg(feature = "std")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Vendor {
    Intel,
    Amd,
    Other,
}

/// Chooses the single-state backend for an x86-64 CPU.
///
/// The single-state AVX2 kernel is the CRYPTOGAMS/OpenSSL kernel. Measured in
/// October 2026:
///
/// * AMD EPYC 7543 (Zen 3): the portable implementation and BMI were roughly
///   equal, with the portable one slightly faster; AVX2 was slower than both.
/// * AMD Zen 5: AVX-512F was about 20% faster than AVX2, AVX2 was 20-40% faster
///   than BMI, and BMI was 20-40% faster than the portable implementation. The
///   exception was 12-round TurboSHAKE/KT256, where the portable implementation
///   was about 20% faster than BMI.
/// * On Intel, the CRYPTOGAMS kernel is the reference implementation and wins.
///
/// AVX2 is therefore only selected on Intel and AMD family 0x1A and later
/// (Zen 5+). AMD Zen 1-4 (families 0x17 and 0x19) and other vendors, including
/// Hygon (family 0x18), use the portable implementation. Zen 4 normally exposes
/// AVX-512 and takes that path; if AVX-512 is masked, family 0x19 conservatively
/// falls back to the portable implementation.
///
/// The BMI kernel was removed after these measurements: it was never the winner
/// when AVX2 was available, and it lost to the portable implementation for
/// 12-round permutations.
///
/// `avx512f` and `avx2` are passed explicitly so the policy can be tested with
/// synthetic CPUID data.
#[cfg(feature = "std")]
fn select_amd64_backend(vendor: Vendor, family: u32, avx512f: bool, avx2: bool) -> PermuteBackend {
    if avx512f {
        return PermuteBackend::Avx512;
    }

    if avx2 {
        match vendor {
            Vendor::Intel => return PermuteBackend::Avx2,
            Vendor::Amd if family >= 0x1A => return PermuteBackend::Avx2,
            Vendor::Amd | Vendor::Other => {}
        }
    }

    return PermuteBackend::Soft;
}

/// Returns the x86-64 vendor and, for AMD, its family.
///
/// This executes `cpuid`, which is serializing, so it must only be called from
/// [`detect_amd64_uncached`], whose result is cached for the lifetime of the
/// process. Intel and other vendors need one leaf; AMD needs a second one for
/// the family.
#[cfg(feature = "std")]
fn cpu_vendor_and_family() -> (Vendor, u32) {
    let leaf0 = core::arch::x86_64::__cpuid(0);
    let vendor = if leaf0.ebx == 0x756E_6547 && leaf0.edx == 0x4965_6E69 && leaf0.ecx == 0x6C65_746E {
        Vendor::Intel
    } else if leaf0.ebx == 0x6874_7541 && leaf0.edx == 0x6974_6E65 && leaf0.ecx == 0x444D_4163 {
        Vendor::Amd
    } else {
        return (Vendor::Other, 0);
    };

    if vendor != Vendor::Amd {
        return (vendor, 0);
    }

    let leaf1 = core::arch::x86_64::__cpuid(1);
    let base_family = (leaf1.eax >> 8) & 0xF;
    let ext_family = (leaf1.eax >> 20) & 0xFF;
    let family = if base_family == 0xF {
        base_family + ext_family
    } else {
        base_family
    };

    return (vendor, family);
}

/// Single-state AVX2 kernel, in the CRYPTOGAMS "magic" lane order.
///
/// Each register holds four *different* lanes of the *same* state, so one
/// variable shift rotates four lanes by four different amounts at once. The
/// state is permuted internally so that theta is one XOR of the six state
/// registers and rho+pi reduces to a handful of in-register permutes; `INTERNAL`
/// maps the plain lane order onto that layout on entry and exit.
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
mod avx2 {
    #![allow(unsafe_op_in_unsafe_fn)]

    use core::arch::x86_64::*;

    use super::ROUND_CONSTANTS;

    /// Rotation amounts used before pi, one vector per state register.
    #[repr(align(32))]
    struct Rotations([u64; 4]);

    static RHO_LEFT: [Rotations; 6] = [
        Rotations([3, 18, 36, 41]),
        Rotations([1, 62, 28, 27]),
        Rotations([45, 6, 56, 39]),
        Rotations([10, 61, 55, 8]),
        Rotations([2, 15, 25, 20]),
        Rotations([44, 43, 21, 14]),
    ];

    static RHO_RIGHT: [Rotations; 6] = [
        Rotations([64 - 3, 64 - 18, 64 - 36, 64 - 41]),
        Rotations([64 - 1, 64 - 62, 64 - 28, 64 - 27]),
        Rotations([64 - 45, 64 - 6, 64 - 56, 64 - 39]),
        Rotations([64 - 10, 64 - 61, 64 - 55, 64 - 8]),
        Rotations([64 - 2, 64 - 15, 64 - 25, 64 - 20]),
        Rotations([64 - 44, 64 - 43, 64 - 21, 64 - 14]),
    ];

    /// Position of each plain state lane in the kernel's register file.
    ///
    /// The inverse of the reference implementation's `mapState`: lane `k` of the
    /// register file (`4 * register + slot`) holds plain lane `INTERNAL[k]`.
    const INTERNAL: [usize; 25] = [
        0, 1, 2, 3, 4, 10, 20, 5, 15, 16, 7, 23, 14, 11, 22, 8, 19, 21, 17, 13, 9, 6, 12, 18, 24,
    ];

    /// One Keccak-p[1600] round.
    ///
    /// The six state registers are `y0`..`y6`; `y7`..`y15` are scratch, which is
    /// exactly the sixteen vector registers AVX2 provides. Register numbers are
    /// only assignment order: the kernel renames lanes as it goes, and the values
    /// in `y0`..`y6` on return are the next state in the same layout.
    #[inline(always)]
    unsafe fn round(s: [__m256i; 7], rc: u64) -> [__m256i; 7] {
        let [mut y0, mut y1, mut y2, mut y3, mut y4, mut y5, mut y6] = s;
        let mut y7: __m256i;
        let mut y8: __m256i;
        let mut y9: __m256i;
        let mut y10: __m256i;
        let mut y11: __m256i;
        let mut y12: __m256i;
        let mut y13: __m256i;
        let mut y14: __m256i;
        let mut y15: __m256i;

        // theta: y1..y6 fold into C[1..4]. `vpshufd 0b01001110` swaps the two
        // u64 of each 128-bit half so the broadcast lane-0 register folds in
        // with one more xor instead of a lane shuffle.
        y13 = _mm256_shuffle_epi32::<0b01001110>(y2);
        y12 = _mm256_xor_si256(y3, y5);
        y9 = _mm256_xor_si256(y6, y4);
        y12 = _mm256_xor_si256(y1, y12);
        y12 = _mm256_xor_si256(y9, y12);
        // `vpermq 0b10010011` rotates the lanes by one so lane x reads C[x-1];
        // `0b00111001` does the opposite for rol(C[x+1], 1). Their xor is D.
        y11 = _mm256_permute4x64_epi64::<0b10010011>(y12);
        y13 = _mm256_xor_si256(y2, y13);
        y7 = _mm256_permute4x64_epi64::<0b01001110>(y13);
        y8 = _mm256_srli_epi64::<63>(y12);
        y9 = _mm256_add_epi64(y12, y12);
        y8 = _mm256_or_si256(y9, y8);
        y15 = _mm256_permute4x64_epi64::<0b00111001>(y8);
        y14 = _mm256_xor_si256(y11, y8);
        y14 = _mm256_permute4x64_epi64::<0b00000000>(y14);
        y13 = _mm256_xor_si256(y0, y13);
        y13 = _mm256_xor_si256(y7, y13);
        y7 = _mm256_srli_epi64::<63>(y13);
        y8 = _mm256_add_epi64(y13, y13);
        y8 = _mm256_or_si256(y7, y8);
        y2 = _mm256_xor_si256(y14, y2);
        y0 = _mm256_xor_si256(y14, y0);
        y15 = _mm256_blend_epi32::<0b11000000>(y15, y8);
        y11 = _mm256_blend_epi32::<0b00000011>(y11, y13);
        y15 = _mm256_xor_si256(y11, y15);

        // rho and pi: one variable-shift pair rotates four lanes by four
        // different counts. The `vpermq`s are the part of pi that the magic
        // order cannot express as a register rename.
        y10 = _mm256_sllv_epi64(y2, _mm256_load_si256(RHO_LEFT[0].0.as_ptr().cast()));
        y2 = _mm256_srlv_epi64(y2, _mm256_load_si256(RHO_RIGHT[0].0.as_ptr().cast()));
        y2 = _mm256_or_si256(y10, y2);
        y3 = _mm256_xor_si256(y15, y3);
        y11 = _mm256_sllv_epi64(y3, _mm256_load_si256(RHO_LEFT[2].0.as_ptr().cast()));
        y3 = _mm256_srlv_epi64(y3, _mm256_load_si256(RHO_RIGHT[2].0.as_ptr().cast()));
        y3 = _mm256_or_si256(y11, y3);
        y4 = _mm256_xor_si256(y15, y4);
        y12 = _mm256_sllv_epi64(y4, _mm256_load_si256(RHO_LEFT[3].0.as_ptr().cast()));
        y4 = _mm256_srlv_epi64(y4, _mm256_load_si256(RHO_RIGHT[3].0.as_ptr().cast()));
        y4 = _mm256_or_si256(y12, y4);
        y5 = _mm256_xor_si256(y15, y5);
        y13 = _mm256_sllv_epi64(y5, _mm256_load_si256(RHO_LEFT[4].0.as_ptr().cast()));
        y5 = _mm256_srlv_epi64(y5, _mm256_load_si256(RHO_RIGHT[4].0.as_ptr().cast()));
        y5 = _mm256_or_si256(y13, y5);
        y6 = _mm256_xor_si256(y15, y6);
        y10 = _mm256_permute4x64_epi64::<0b10001101>(y2);
        y11 = _mm256_permute4x64_epi64::<0b10001101>(y3);
        y14 = _mm256_sllv_epi64(y6, _mm256_load_si256(RHO_LEFT[5].0.as_ptr().cast()));
        y8 = _mm256_srlv_epi64(y6, _mm256_load_si256(RHO_RIGHT[5].0.as_ptr().cast()));
        y8 = _mm256_or_si256(y14, y8);
        y1 = _mm256_xor_si256(y15, y1);
        y12 = _mm256_permute4x64_epi64::<0b00011011>(y4);
        y13 = _mm256_permute4x64_epi64::<0b01110010>(y5);
        y15 = _mm256_sllv_epi64(y1, _mm256_load_si256(RHO_LEFT[1].0.as_ptr().cast()));
        y9 = _mm256_srlv_epi64(y1, _mm256_load_si256(RHO_RIGHT[1].0.as_ptr().cast()));
        y9 = _mm256_or_si256(y15, y9);

        // chi: `vpsrldq` and `vpblendd` assemble, for each row, the x+1 and x+2
        // windows of the four-lane groups; the `vpandn`s then produce all five
        // outputs of the ring.
        y14 = _mm256_srli_si256::<8>(y8);
        y7 = _mm256_andnot_si256(y8, y14);
        y3 = _mm256_blend_epi32::<0b00001100>(y9, y13);
        y15 = _mm256_blend_epi32::<0b00001100>(y11, y9);
        y5 = _mm256_blend_epi32::<0b00001100>(y10, y11);
        y14 = _mm256_blend_epi32::<0b00001100>(y9, y10);
        y3 = _mm256_blend_epi32::<0b00110000>(y3, y11);
        y15 = _mm256_blend_epi32::<0b00110000>(y15, y12);
        y5 = _mm256_blend_epi32::<0b00110000>(y5, y9);
        y14 = _mm256_blend_epi32::<0b00110000>(y14, y13);
        y3 = _mm256_blend_epi32::<0b11000000>(y3, y12);
        y15 = _mm256_blend_epi32::<0b11000000>(y15, y13);
        y5 = _mm256_blend_epi32::<0b11000000>(y5, y13);
        y14 = _mm256_blend_epi32::<0b11000000>(y14, y11);
        y3 = _mm256_andnot_si256(y3, y15);
        y5 = _mm256_andnot_si256(y5, y14);
        y6 = _mm256_blend_epi32::<0b00001100>(y12, y9);
        y15 = _mm256_blend_epi32::<0b00001100>(y10, y12);
        y3 = _mm256_xor_si256(y10, y3);
        y6 = _mm256_blend_epi32::<0b00110000>(y6, y10);
        y15 = _mm256_blend_epi32::<0b00110000>(y15, y11);
        y5 = _mm256_xor_si256(y12, y5);
        y6 = _mm256_blend_epi32::<0b11000000>(y6, y11);
        y15 = _mm256_blend_epi32::<0b11000000>(y15, y9);
        y6 = _mm256_andnot_si256(y6, y15);
        y6 = _mm256_xor_si256(y13, y6);
        y4 = _mm256_permute4x64_epi64::<0b00011110>(y8);
        y15 = _mm256_blend_epi32::<0b00110000>(y4, y0);
        y1 = _mm256_permute4x64_epi64::<0b00111001>(y8);
        y1 = _mm256_blend_epi32::<0b11000000>(y1, y0);
        y1 = _mm256_andnot_si256(y1, y15);
        y2 = _mm256_blend_epi32::<0b00001100>(y11, y12);
        y14 = _mm256_blend_epi32::<0b00001100>(y13, y11);
        y2 = _mm256_blend_epi32::<0b00110000>(y2, y13);
        y14 = _mm256_blend_epi32::<0b00110000>(y14, y10);
        y2 = _mm256_blend_epi32::<0b11000000>(y2, y10);
        y14 = _mm256_blend_epi32::<0b11000000>(y14, y12);
        y2 = _mm256_andnot_si256(y2, y14);
        y2 = _mm256_xor_si256(y9, y2);
        y7 = _mm256_permute4x64_epi64::<0b00000000>(y7);
        y3 = _mm256_permute4x64_epi64::<0b00011011>(y3);
        y5 = _mm256_permute4x64_epi64::<0b10001101>(y5);
        y6 = _mm256_permute4x64_epi64::<0b01110010>(y6);
        y4 = _mm256_blend_epi32::<0b00001100>(y13, y10);
        y14 = _mm256_blend_epi32::<0b00001100>(y12, y13);
        y4 = _mm256_blend_epi32::<0b00110000>(y4, y12);
        y14 = _mm256_blend_epi32::<0b00110000>(y14, y9);
        y4 = _mm256_blend_epi32::<0b11000000>(y4, y9);
        y14 = _mm256_blend_epi32::<0b11000000>(y14, y10);
        y4 = _mm256_andnot_si256(y4, y14);
        y0 = _mm256_xor_si256(y7, y0);
        y1 = _mm256_xor_si256(y8, y1);
        y4 = _mm256_xor_si256(y11, y4);

        // iota: the round constant goes into lane 0 only.
        y0 = _mm256_xor_si256(y0, _mm256_set1_epi64x(rc as i64));
        return [y0, y1, y2, y3, y4, y5, y6];
    }

    /// The Keccak-p[`ROUNDS`] permutation of one 1600-bit state.
    ///
    /// `ROUNDS` must not exceed 24. A larger value panics in debug builds and is
    /// unsupported otherwise.
    ///
    /// # Safety
    ///
    /// The CPU must implement AVX2.
    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn p1600_avx2<const ROUNDS: usize>(state: &mut [u64; 25]) {
        const {
            assert!(ROUNDS <= 24, "A round count greater than 24 is not supported.");
        }

        // Scatter the plain state into the kernel's register order: register 0 is a
        // broadcast of lane 0, and register `r` (1..=6) holds file positions
        // `4r..4r+3`.
        let mut buf = [[0u64; 4]; 7];
        buf[0] = [state[0]; 4];
        for k in 1..25 {
            buf[1 + (k - 1) / 4][(k - 1) % 4] = state[INTERNAL[k]];
        }

        let mut y: [__m256i; 7] = core::array::from_fn(|i| _mm256_loadu_si256(buf[i].as_ptr().cast()));

        for r in (24 - ROUNDS)..24 {
            y = round(y, ROUND_CONSTANTS[r]);
        }

        for i in 0..7 {
            _mm256_storeu_si256(buf[i].as_mut_ptr().cast(), y[i]);
        }
        state[0] = buf[0][0];
        for k in 1..25 {
            state[INTERNAL[k]] = buf[1 + (k - 1) / 4][(k - 1) % 4];
        }
    }
}

/// Single-state AVX-512F kernel, two rounds per iteration.
///
/// Five ZMM registers hold the five rows of the state, five lanes each (the top
/// three lanes of every register are unused). Each iteration performs one
/// "even" and one "odd" round. Pi transposes the state between rows and
/// columns, which is what lets chi run across registers; the harmonize step
/// between the two rounds puts the rows back for the next theta.
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
mod avx512 {
    #![allow(unsafe_op_in_unsafe_fn)]

    use core::arch::x86_64::*;

    use super::ROUND_CONSTANTS;

    /// A 512-bit constant vector, aligned so it can be loaded with an aligned move.
    #[repr(align(64))]
    struct V8([u64; 8]);

    /// The state, padded to eight lanes per row.
    ///
    /// Row `y` occupies `0[8 * y..8 * y + 5]`; the remaining lanes stay zero.
    #[repr(align(64))]
    struct Columns([u64; 40]);

    /// Column-rotation indices used by theta.
    static THETA_PERM: [V8; 5] = [
        V8([0, 1, 2, 3, 4, 5, 6, 7]),
        V8([4, 0, 1, 2, 3, 5, 6, 7]),
        V8([3, 4, 0, 1, 2, 5, 6, 7]),
        V8([2, 3, 4, 0, 1, 5, 6, 7]),
        V8([1, 2, 3, 4, 0, 5, 6, 7]),
    ];

    /// Rotation counts applied before pi in the even round.
    static RHO_EVEN: [V8; 5] = [
        V8([0, 1, 62, 28, 27, 0, 0, 0]),
        V8([36, 44, 6, 55, 20, 0, 0, 0]),
        V8([3, 10, 43, 25, 39, 0, 0, 0]),
        V8([41, 45, 15, 21, 8, 0, 0, 0]),
        V8([18, 2, 61, 56, 14, 0, 0, 0]),
    ];

    /// Rotation counts applied before pi in the odd round.
    static RHO_ODD: [V8; 5] = [
        V8([0, 44, 43, 21, 14, 0, 0, 0]),
        V8([18, 1, 6, 25, 8, 0, 0, 0]),
        V8([41, 2, 62, 55, 39, 0, 0, 0]),
        V8([3, 45, 61, 28, 20, 0, 0, 0]),
        V8([36, 10, 15, 56, 27, 0, 0, 0]),
    ];

    /// Lane-rotation indices used by pi.
    static PI0_PERM: [V8; 5] = [
        V8([0, 3, 1, 4, 2, 5, 6, 7]),
        V8([1, 4, 2, 0, 3, 5, 6, 7]),
        V8([2, 0, 3, 1, 4, 5, 6, 7]),
        V8([3, 1, 4, 2, 0, 5, 6, 7]),
        V8([4, 2, 0, 3, 1, 5, 6, 7]),
    ];

    /// The Keccak-p[`ROUNDS`] permutation of one 1600-bit state.
    ///
    /// `ROUNDS` must be at most 24; a larger value aborts in debug builds and is
    /// unsupported otherwise. The body applies rounds in pairs, so an odd count
    /// leaves the final round to the portable implementation.
    ///
    /// # Safety
    ///
    /// The CPU must implement AVX-512F.
    #[target_feature(enable = "avx512f")]
    pub(crate) unsafe fn p1600_avx512<const ROUNDS: usize>(state: &mut [u64; 25]) {
        const {
            assert!(ROUNDS <= 24, "A round count greater than 24 is not supported.");
        }

        // Spread each row of five lanes over eight lanes so a whole row lives in
        // one ZMM register.
        let mut buf = Columns([0u64; 40]);
        for y in 0..5 {
            for x in 0..5 {
                buf.0[8 * y + x] = state[5 * y + x];
            }
        }

        let mut y0 = _mm512_load_si512(buf.0.as_ptr().cast());
        let mut y1 = _mm512_load_si512(buf.0.as_ptr().add(8).cast());
        let mut y2 = _mm512_load_si512(buf.0.as_ptr().add(16).cast());
        let mut y3 = _mm512_load_si512(buf.0.as_ptr().add(24).cast());
        let mut y4 = _mm512_load_si512(buf.0.as_ptr().add(32).cast());

        // The constant names keep the reference implementation's register numbers
        // (`zmm13`..`zmm31`) so the round below can be compared with it directly.
        let c13 = _mm512_load_si512(THETA_PERM[1].0.as_ptr().cast());
        let c14 = _mm512_load_si512(THETA_PERM[2].0.as_ptr().cast());
        let c15 = _mm512_load_si512(THETA_PERM[3].0.as_ptr().cast());
        let c16 = _mm512_load_si512(THETA_PERM[4].0.as_ptr().cast());
        let c17 = _mm512_load_si512(PI0_PERM[0].0.as_ptr().cast());
        let c18 = _mm512_load_si512(PI0_PERM[1].0.as_ptr().cast());
        let c19 = _mm512_load_si512(PI0_PERM[2].0.as_ptr().cast());
        let c20 = _mm512_load_si512(PI0_PERM[3].0.as_ptr().cast());
        let c21 = _mm512_load_si512(PI0_PERM[4].0.as_ptr().cast());
        let c22 = _mm512_load_si512(RHO_EVEN[0].0.as_ptr().cast());
        let c23 = _mm512_load_si512(RHO_EVEN[1].0.as_ptr().cast());
        let c24 = _mm512_load_si512(RHO_EVEN[2].0.as_ptr().cast());
        let c25 = _mm512_load_si512(RHO_EVEN[3].0.as_ptr().cast());
        let c26 = _mm512_load_si512(RHO_EVEN[4].0.as_ptr().cast());
        let c27 = _mm512_load_si512(RHO_ODD[0].0.as_ptr().cast());
        let c28 = _mm512_load_si512(RHO_ODD[1].0.as_ptr().cast());
        let c29 = _mm512_load_si512(RHO_ODD[2].0.as_ptr().cast());
        let c30 = _mm512_load_si512(RHO_ODD[3].0.as_ptr().cast());
        let c31 = _mm512_load_si512(RHO_ODD[4].0.as_ptr().cast());

        for iteration in 0..ROUNDS / 2 {
            let base = 24 - ROUNDS + 2 * iteration;
            let rc0 = ROUND_CONSTANTS[base];
            let rc1 = ROUND_CONSTANTS[base + 1];
            let mut t0: __m512i;
            let mut t1: __m512i;
            let mut t2: __m512i;
            let mut t3: __m512i;
            let mut t4: __m512i;
            // theta: y0..y4 are whole rows, so two ternlogs XOR them into the
            // column parities C. The `vpermq`s then rotate the lane order by one
            // in each direction, and lane x of their sum is
            // D[x] = C[x-1] ^ rol(C[x+1], 1).
            t0 = y0;
            y0 = _mm512_ternarylogic_epi64::<0x96>(y0, y1, y2);
            y0 = _mm512_ternarylogic_epi64::<0x96>(y0, y3, y4);
            t1 = _mm512_rol_epi64::<1>(y0);
            y0 = _mm512_permutexvar_epi64(c13, y0);
            t1 = _mm512_permutexvar_epi64(c16, t1);
            t0 = _mm512_ternarylogic_epi64::<0x96>(t0, t1, y0);
            y1 = _mm512_ternarylogic_epi64::<0x96>(y1, t1, y0);
            y2 = _mm512_ternarylogic_epi64::<0x96>(y2, t1, y0);
            y3 = _mm512_ternarylogic_epi64::<0x96>(y3, t1, y0);
            y4 = _mm512_ternarylogic_epi64::<0x96>(y4, t1, y0);
            // rho: one `vrolv` per row, with a different count in each lane.
            y0 = _mm512_rolv_epi64(t0, c22);
            y1 = _mm512_rolv_epi64(y1, c23);
            y2 = _mm512_rolv_epi64(y2, c24);
            y3 = _mm512_rolv_epi64(y3, c25);
            y4 = _mm512_rolv_epi64(y4, c26);
            // pi: intra-register lane rotations. They leave the parity of the
            // state transposed, which is exactly what chi wants next.
            y0 = _mm512_permutexvar_epi64(c17, y0);
            y1 = _mm512_permutexvar_epi64(c18, y1);
            y2 = _mm512_permutexvar_epi64(c19, y2);
            y3 = _mm512_permutexvar_epi64(c20, y3);
            y4 = _mm512_permutexvar_epi64(c21, y4);
            // chi: after pi each register holds a column, so the five registers
            // combine ring-wise: out[x] = col[x] ^ (~col[x+1] & col[x+2]).
            t0 = y0;
            t1 = y1;
            y0 = _mm512_xor_si512(y0, _mm512_andnot_si512(y1, y2));
            y1 = _mm512_xor_si512(y1, _mm512_andnot_si512(y2, y3));
            y2 = _mm512_xor_si512(y2, _mm512_andnot_si512(y3, y4));
            y3 = _mm512_xor_si512(y3, _mm512_andnot_si512(y4, t0));
            y4 = _mm512_xor_si512(y4, _mm512_andnot_si512(t0, t1));
            // iota: the constant goes into lane 0, leaving the other lanes alone.
            y0 = _mm512_mask_xor_epi64(y0, 0x01, y0, _mm512_set1_epi64(rc0 as i64));

            // harmonize: move lane 1..4 of each register into the row where the
            // odd round expects it, then put the rows back in order. Each mask
            // is a single lane bit (`1 << lane`), matching `k2`..`k5` from the
            // reference implementation's `kshiftlw $1`..`$4`; they are not
            // cumulative.
            t1 = _mm512_mask_blend_epi64(0x02, y1, y2);
            t2 = _mm512_mask_blend_epi64(0x02, y2, y3);
            t3 = _mm512_mask_blend_epi64(0x02, y3, y4);
            t0 = _mm512_mask_blend_epi64(0x02, y0, y1);
            t4 = _mm512_mask_blend_epi64(0x02, y4, y0);
            t1 = _mm512_mask_blend_epi64(0x04, t1, y3);
            t2 = _mm512_mask_blend_epi64(0x04, t2, y4);
            t0 = _mm512_mask_blend_epi64(0x04, t0, y2);
            t3 = _mm512_mask_blend_epi64(0x04, t3, y0);
            t4 = _mm512_mask_blend_epi64(0x04, t4, y1);
            t1 = _mm512_mask_blend_epi64(0x08, t1, y4);
            t0 = _mm512_mask_blend_epi64(0x08, t0, y3);
            t2 = _mm512_mask_blend_epi64(0x08, t2, y0);
            t3 = _mm512_mask_blend_epi64(0x08, t3, y1);
            t4 = _mm512_mask_blend_epi64(0x08, t4, y2);
            t0 = _mm512_mask_blend_epi64(0x10, t0, y4);
            t1 = _mm512_mask_blend_epi64(0x10, t1, y0);
            t2 = _mm512_mask_blend_epi64(0x10, t2, y1);
            t3 = _mm512_mask_blend_epi64(0x10, t3, y2);
            t4 = _mm512_mask_blend_epi64(0x10, t4, y3);
            y1 = _mm512_permutexvar_epi64(c13, t1);
            y2 = _mm512_permutexvar_epi64(c14, t2);
            y3 = _mm512_permutexvar_epi64(c15, t3);
            y4 = _mm512_permutexvar_epi64(c16, t4);
            // The odd round is the same sequence on the rows the harmonize step
            // just produced; only its rotation and permutation tables differ.
            y0 = t0;
            t0 = _mm512_ternarylogic_epi64::<0x96>(t0, y1, y2);
            t0 = _mm512_ternarylogic_epi64::<0x96>(t0, y3, y4);
            t1 = _mm512_rol_epi64::<1>(t0);
            t0 = _mm512_permutexvar_epi64(c13, t0);
            t1 = _mm512_permutexvar_epi64(c16, t1);
            y0 = _mm512_ternarylogic_epi64::<0x96>(y0, t1, t0);
            y3 = _mm512_ternarylogic_epi64::<0x96>(y3, t1, t0);
            y1 = _mm512_ternarylogic_epi64::<0x96>(y1, t1, t0);
            y4 = _mm512_ternarylogic_epi64::<0x96>(y4, t1, t0);
            y2 = _mm512_ternarylogic_epi64::<0x96>(y2, t1, t0);
            y0 = _mm512_rolv_epi64(y0, c27);
            t1 = _mm512_rolv_epi64(y3, c30);
            t2 = _mm512_rolv_epi64(y1, c28);
            t3 = _mm512_rolv_epi64(y4, c31);
            t4 = _mm512_rolv_epi64(y2, c29);
            let t5 = _mm512_permutexvar_epi64(c16, y0);
            let t6 = _mm512_permutexvar_epi64(c15, y0);
            // iota for the odd round.
            y0 = _mm512_mask_xor_epi64(y0, 0x01, y0, _mm512_set1_epi64(rc1 as i64));
            y1 = _mm512_permutexvar_epi64(c14, t1);
            y2 = _mm512_permutexvar_epi64(c16, t2);
            y3 = _mm512_permutexvar_epi64(c13, t3);
            y4 = _mm512_permutexvar_epi64(c15, t4);
            y0 = _mm512_xor_si512(y0, _mm512_andnot_si512(t5, t6));
            let t7 = _mm512_permutexvar_epi64(c13, t1);
            y1 = _mm512_xor_si512(y1, _mm512_andnot_si512(t7, t1));
            t0 = _mm512_permutexvar_epi64(c15, t2);
            t2 = _mm512_permutexvar_epi64(c14, t2);
            y2 = _mm512_xor_si512(y2, _mm512_andnot_si512(t0, t2));
            t1 = _mm512_permutexvar_epi64(c16, t3);
            y3 = _mm512_xor_si512(y3, _mm512_andnot_si512(t3, t1));
            t0 = _mm512_permutexvar_epi64(c14, t4);
            t4 = _mm512_permutexvar_epi64(c13, t4);
            y4 = _mm512_xor_si512(y4, _mm512_andnot_si512(t0, t4));
        }

        _mm512_store_si512(buf.0.as_mut_ptr().cast(), y0);
        _mm512_store_si512(buf.0.as_mut_ptr().add(8).cast(), y1);
        _mm512_store_si512(buf.0.as_mut_ptr().add(16).cast(), y2);
        _mm512_store_si512(buf.0.as_mut_ptr().add(24).cast(), y3);
        _mm512_store_si512(buf.0.as_mut_ptr().add(32).cast(), y4);
        for y in 0..5 {
            for x in 0..5 {
                state[5 * y + x] = buf.0[8 * y + x];
            }
        }

        // The pairs cover rounds `24 - ROUNDS .. 23`, so an odd count is missing
        // round 23. That is exactly the last round of Keccak-f, which
        // Keccak-p[1600, 1] applies, so finish it with the portable one.
        if !ROUNDS.is_multiple_of(2) {
            crate::sha3::keccak::p1600_soft::<1>(state);
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::{Vendor, select_amd64_backend};
    use crate::sha3::keccak::PermuteBackend;

    /// The policy must keep the single-state AVX2 kernel away from CPUs where
    /// it is known to be slower than the portable implementation.
    #[test]
    fn amd64_backend_policy() {
        let cases: [(Vendor, u32, bool, bool, PermuteBackend); 12] = [
            (Vendor::Intel, 0x6, false, true, PermuteBackend::Avx2),
            (Vendor::Intel, 0x6, true, true, PermuteBackend::Avx512),
            (Vendor::Intel, 0x6, false, false, PermuteBackend::Soft),
            (Vendor::Amd, 0x17, false, true, PermuteBackend::Soft),
            (Vendor::Amd, 0x19, false, true, PermuteBackend::Soft),
            (Vendor::Amd, 0x19, false, false, PermuteBackend::Soft),
            (Vendor::Amd, 0x19, true, true, PermuteBackend::Avx512),
            (Vendor::Amd, 0x1A, false, true, PermuteBackend::Avx2),
            (Vendor::Amd, 0x1A, true, true, PermuteBackend::Avx512),
            (Vendor::Other, 0x18, false, true, PermuteBackend::Soft),
            (Vendor::Other, 0x6, false, true, PermuteBackend::Soft),
            (Vendor::Other, 0x6, true, true, PermuteBackend::Avx512),
        ];

        for (vendor, family, avx512f, avx2, expected) in cases {
            assert_eq!(
                select_amd64_backend(vendor, family, avx512f, avx2),
                expected,
                "vendor={vendor:?} family={family:#x} avx512f={avx512f} avx2={avx2}"
            );
        }
    }
}
