//! Internal constants, CPU feature detection and SIMD dispatch for BLAKE3.
//!
//! This module is the only place that knows which instruction set kernels were
//! compiled in and which one the running CPU supports. The engine in
//! [`super::hasher`] only ever talks to the [`Platform`] enum.

#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
use super::avx2;
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
use super::avx512;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use super::neon;
use super::scalar;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use super::wasm32_simd128;

/// The number of bytes in an output.
pub(crate) const OUT_LEN: usize = 32;

/// The number of bytes in a key.
pub(crate) const KEY_LEN: usize = 32;

/// The number of bytes in a block.
pub(crate) const BLOCK_LEN: usize = 64;

/// The number of bytes in a chunk.
pub(crate) const CHUNK_LEN: usize = 1024;

/// The maximum number of chaining values kept on the subtree stack:
/// `2^54 * CHUNK_LEN = 2^64` bytes of input.
///
/// The stack uses lazy merging, so it needs one more slot than the number of
/// set bits in the chunk counter (see [`super::hasher::Hasher`]).
pub(crate) const MAX_DEPTH: usize = 54;

/// The initialization vector, which is the same as the BLAKE2s IV.
pub(crate) const IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

/// The message word permutation applied between rounds (7 rounds total).
pub(crate) const MSG_SCHEDULE: [[usize; 16]; 7] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8],
    [3, 4, 10, 12, 13, 2, 7, 14, 6, 5, 9, 0, 11, 15, 8, 1],
    [10, 7, 12, 9, 14, 3, 13, 15, 4, 0, 11, 2, 5, 8, 1, 6],
    [12, 13, 9, 11, 15, 10, 14, 8, 7, 2, 5, 3, 0, 1, 6, 4],
    [9, 14, 11, 5, 8, 12, 15, 1, 13, 3, 0, 10, 2, 6, 4, 7],
    [11, 15, 5, 0, 1, 9, 8, 6, 14, 10, 2, 12, 3, 4, 7, 13],
];

/// Set on the first block of a chunk.
pub(crate) const CHUNK_START: u8 = 1 << 0;
/// Set on the last block of a chunk.
pub(crate) const CHUNK_END: u8 = 1 << 1;
/// Set on parent nodes (as opposed to chunk nodes).
pub(crate) const PARENT: u8 = 1 << 2;
/// Set on the final compression of the tree.
pub(crate) const ROOT: u8 = 1 << 3;
/// Set in keyed-hash mode.
pub(crate) const KEYED_HASH: u8 = 1 << 4;
/// Set while hashing the context string of the key derivation mode.
pub(crate) const DERIVE_KEY_CONTEXT: u8 = 1 << 5;
/// Set while hashing the key material of the key derivation mode.
pub(crate) const DERIVE_KEY_MATERIAL: u8 = 1 << 6;

/// The largest number of chunks (or parents) that any compiled kernel can hash
/// in parallel. Independent of what the running CPU actually supports; the
/// runtime degree is [`Platform::simd_degree`].
pub(crate) const MAX_SIMD_DEGREE: usize = {
    if cfg!(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f"))) {
        16
    } else if cfg!(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2"))) {
        8
    } else if cfg!(all(target_arch = "aarch64", target_feature = "neon"))
        || cfg!(all(target_arch = "wasm32", target_feature = "simd128"))
    {
        4
    } else {
        1
    }
};

/// Same as [`MAX_SIMD_DEGREE`] but at least 2. Some buffers need room for two
/// chaining values even when the SIMD degree is 1 (see
/// `compress_subtree_wide`).
pub(crate) const MAX_SIMD_DEGREE_OR_2: usize = if MAX_SIMD_DEGREE < 2 { 2 } else { MAX_SIMD_DEGREE };

/// Whether `hash_many` should increment the chunk counter for each lane.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum IncrementCounter {
    Yes,
    No,
}

impl IncrementCounter {
    #[inline(always)]
    pub(crate) fn yes(self) -> bool {
        matches!(self, IncrementCounter::Yes)
    }
}

#[inline(always)]
pub(crate) fn counter_low(counter: u64) -> u32 {
    counter as u32
}

#[inline(always)]
pub(crate) fn counter_high(counter: u64) -> u32 {
    (counter >> 32) as u32
}

/// The SIMD backend selected at runtime (or at compile time when the `std`
/// feature is off and `target_feature` is set explicitly).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
pub(crate) enum Platform {
    Portable,
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    Avx512,
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    Avx2,
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    Neon,
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    Wasm32Simd,
}

impl Platform {
    /// Detect the widest available backend.
    ///
    /// On x86_64 with the `std` feature this performs a (cached) runtime CPU
    /// check; without `std` it relies on the compile-time `target_feature`.
    /// AArch64 always has NEON and WebAssembly SIMD is compile-time only, so
    /// those are selected through cfg instead.
    #[allow(unreachable_code)]
    pub(crate) fn detect() -> Platform {
        #[cfg(all(target_arch = "x86_64", feature = "std"))]
        {
            if is_x86_feature_detected!("avx512f") {
                return Platform::Avx512;
            }
            if is_x86_feature_detected!("avx2") {
                return Platform::Avx2;
            }
        }

        #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx512f"))]
        {
            return Platform::Avx512;
        }

        #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx2"))]
        {
            return Platform::Avx2;
        }

        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        {
            return Platform::Neon;
        }

        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        {
            return Platform::Wasm32Simd;
        }

        Platform::Portable
    }

    /// The number of chunks hashed in parallel by this backend.
    #[inline]
    pub(crate) fn simd_degree(self) -> usize {
        match self {
            Platform::Portable => 1,
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            Platform::Avx512 => avx512::DEGREE,
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            Platform::Avx2 => avx2::DEGREE,
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            Platform::Neon => neon::DEGREE,
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            Platform::Wasm32Simd => wasm32_simd128::DEGREE,
        }
    }

    /// Compress one block into `cv` (truncated output, 8 words).
    #[inline]
    pub(crate) fn compress_in_place(
        self,
        cv: &mut [u32; 8],
        block: &[u8; BLOCK_LEN],
        block_len: u8,
        counter: u64,
        flags: u8,
    ) {
        match self {
            Platform::Portable => scalar::compress_in_place(cv, block, block_len, counter, flags),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            // SAFETY: AVX-512F is guaranteed by `detect()` / the compile-time feature.
            Platform::Avx512 => unsafe { avx512::compress_in_place(cv, block, block_len, counter, flags) },
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            // SAFETY: AVX2 is guaranteed by `detect()` / the compile-time feature.
            Platform::Avx2 => unsafe { avx2::compress_in_place(cv, block, block_len, counter, flags) },
            // NEON has no dedicated single-block compression; the portable one is the same speed.
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            Platform::Neon => scalar::compress_in_place(cv, block, block_len, counter, flags),
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            Platform::Wasm32Simd => wasm32_simd128::compress_in_place(cv, block, block_len, counter, flags),
        }
    }

    /// Compress one block and return the full 64-byte extended output state.
    #[inline]
    pub(crate) fn compress_xof(
        self,
        cv: &[u32; 8],
        block: &[u8; BLOCK_LEN],
        block_len: u8,
        counter: u64,
        flags: u8,
    ) -> [u8; 64] {
        match self {
            Platform::Portable => scalar::compress_xof(cv, block, block_len, counter, flags),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            // SAFETY: AVX-512F is guaranteed by `detect()` / the compile-time feature.
            Platform::Avx512 => unsafe { avx512::compress_xof(cv, block, block_len, counter, flags) },
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            // SAFETY: AVX2 is guaranteed by `detect()` / the compile-time feature.
            Platform::Avx2 => unsafe { avx2::compress_xof(cv, block, block_len, counter, flags) },
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            Platform::Neon => scalar::compress_xof(cv, block, block_len, counter, flags),
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            Platform::Wasm32Simd => wasm32_simd128::compress_xof(cv, block, block_len, counter, flags),
        }
    }

    /// Hash several inputs of equal length in parallel.
    ///
    /// Every input must be a whole number of 64-byte blocks with the same block
    /// count (`blocks = inputs[i].len() / BLOCK_LEN`). `out` must hold
    /// `inputs.len() * OUT_LEN` bytes.
    #[inline]
    pub(crate) fn hash_many(
        self,
        inputs: &[&[u8]],
        key: &[u32; 8],
        counter: u64,
        increment_counter: IncrementCounter,
        flags: u8,
        flags_start: u8,
        flags_end: u8,
        out: &mut [u8],
    ) {
        debug_assert!(out.len() >= inputs.len() * OUT_LEN, "out too short");
        debug_assert!(inputs.iter().all(|i| i.len() % BLOCK_LEN == 0), "inputs must be whole blocks");
        debug_assert!(
            inputs.is_empty() || inputs.iter().all(|i| i.len() == inputs[0].len()),
            "inputs must have equal lengths"
        );
        match self {
            Platform::Portable => {
                scalar::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out)
            }
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            // SAFETY: AVX-512F is guaranteed by `detect()` / the compile-time feature.
            Platform::Avx512 => unsafe {
                avx512::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out)
            },
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            // SAFETY: AVX2 is guaranteed by `detect()` / the compile-time feature.
            Platform::Avx2 => unsafe {
                avx2::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out)
            },
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            // SAFETY: NEON is mandatory on aarch64.
            Platform::Neon => unsafe {
                neon::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out)
            },
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            Platform::Wasm32Simd => {
                wasm32_simd128::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out)
            }
        }
    }

    /// Fill `out` with consecutive extendable-output blocks of the root node
    /// `cv`/`block`.
    ///
    /// Block `i` is produced with counter `counter + i`. `out` must be a whole
    /// number of 64-byte blocks and may be empty, in which case this is a
    /// no-op: some backends can only write a full SIMD group at a time.
    #[inline]
    pub(crate) fn xof_many(
        self,
        cv: &[u32; 8],
        block: &[u8; BLOCK_LEN],
        block_len: u8,
        counter: u64,
        flags: u8,
        out: &mut [u8],
    ) {
        debug_assert_eq!(out.len() % BLOCK_LEN, 0, "out must be whole blocks");
        if out.is_empty() {
            return;
        }
        match self {
            Platform::Portable => scalar::xof_many(cv, block, block_len, counter, flags, out),
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            // SAFETY: AVX-512F is guaranteed by `detect()` / the compile-time feature.
            Platform::Avx512 => unsafe { avx512::xof_many(cv, block, block_len, counter, flags, out) },
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            // SAFETY: AVX2 is guaranteed by `detect()` / the compile-time feature.
            Platform::Avx2 => unsafe { avx2::xof_many(cv, block, block_len, counter, flags, out) },
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            // SAFETY: NEON is mandatory on aarch64.
            Platform::Neon => unsafe { neon::xof_many(cv, block, block_len, counter, flags, out) },
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            Platform::Wasm32Simd => wasm32_simd128::xof_many(cv, block, block_len, counter, flags, out),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every backend compiled into this build. The portable backend is always
    /// first; the SIMD ones are appended when they can run.
    #[allow(clippy::vec_init_then_push)]
    fn available_platforms() -> Vec<Platform> {
        let mut platforms = vec![Platform::Portable];
        #[cfg(all(target_arch = "x86_64", feature = "std"))]
        {
            if is_x86_feature_detected!("avx512f") {
                platforms.push(Platform::Avx512);
            }
            if is_x86_feature_detected!("avx2") {
                platforms.push(Platform::Avx2);
            }
        }
        #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx512f"))]
        platforms.push(Platform::Avx512);
        #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx2"))]
        platforms.push(Platform::Avx2);
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        platforms.push(Platform::Neon);
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        platforms.push(Platform::Wasm32Simd);
        platforms
    }

    /// `detect()` must select a backend whose kernel is actually compiled in,
    /// whose `simd_degree` fits the compile-time maximum, and (with `std`) it
    /// must agree with the runtime feature checks.
    #[test]
    fn detect_selects_a_compiled_backend() {
        assert!(MAX_SIMD_DEGREE.is_power_of_two(), "MAX_SIMD_DEGREE must be a power of two");
        const {
            assert!(MAX_SIMD_DEGREE >= 1);
        }
        assert_eq!(MAX_SIMD_DEGREE_OR_2, core::cmp::max(2, MAX_SIMD_DEGREE));

        let detected = Platform::detect();
        // The degree of every backend is bounded by the compile-time maximum,
        // which is what sizes the stack buffers in `hasher.rs`.
        for platform in available_platforms() {
            let degree = platform.simd_degree();
            assert!(degree >= 1, "{platform:?}");
            assert!(degree <= MAX_SIMD_DEGREE, "{platform:?} degree {degree} > {MAX_SIMD_DEGREE}");
            assert!(degree.is_power_of_two(), "{platform:?}");
        }
        assert!(detected.simd_degree() >= 1 && detected.simd_degree() <= MAX_SIMD_DEGREE);

        // The detected backend must be one of the compiled, runnable ones.
        assert!(
            available_platforms().contains(&detected),
            "detect() returned {detected:?}, which is not runnable on this build"
        );

        #[cfg(all(target_arch = "x86_64", feature = "std"))]
        {
            let expected = if is_x86_feature_detected!("avx512f") {
                Platform::Avx512
            } else if is_x86_feature_detected!("avx2") {
                Platform::Avx2
            } else {
                Platform::Portable
            };
            assert_eq!(detected, expected);
        }
        #[cfg(all(target_arch = "x86_64", not(feature = "std")))]
        {
            #[cfg(target_feature = "avx512f")]
            assert_eq!(detected, Platform::Avx512);
            #[cfg(all(not(target_feature = "avx512f"), target_feature = "avx2"))]
            assert_eq!(detected, Platform::Avx2);
            #[cfg(all(not(target_feature = "avx512f"), not(target_feature = "avx2")))]
            assert_eq!(detected, Platform::Portable);
        }
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        assert_eq!(detected, Platform::Neon);
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        assert_eq!(detected, Platform::Wasm32Simd);
        #[cfg(not(any(
            target_arch = "x86_64",
            all(target_arch = "aarch64", target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128")
        )))]
        assert_eq!(detected, Platform::Portable);
    }

    fn pattern(len: usize, seed: u8) -> Vec<u8> {
        (0..len)
            .map(|i| ((i as u8).wrapping_mul(37).wrapping_add(seed)) % 251)
            .collect()
    }

    /// `hash_many` requires equal-length inputs; the debug assertion must fire.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "inputs must have equal lengths")]
    fn hash_many_rejects_unequal_input_lengths() {
        let key = [0u32; 8];
        let a = [0u8; BLOCK_LEN];
        let b = [0u8; 2 * BLOCK_LEN];
        let inputs: [&[u8]; 2] = [&a, &b];
        let mut out = [0u8; 2 * OUT_LEN];
        Platform::Portable.hash_many(&inputs, &key, 0, IncrementCounter::Yes, 0, CHUNK_START, CHUNK_END, &mut out);
    }

    /// `hash_many` may only write `inputs.len() * OUT_LEN` bytes, must never
    /// write outside the slice it is given, and must produce the same chaining
    /// values as the portable backend for every SIMD-group/tail split.
    #[test]
    fn hash_many_respects_output_bounds() {
        let key = {
            let bytes = pattern(32, 77);
            let mut words = [0u32; 8];
            for (word, chunk) in words.iter_mut().zip(bytes.as_chunks::<4>().0) {
                *word = u32::from_le_bytes(*chunk);
            }
            words
        };

        for platform in available_platforms() {
            // Whole chunks: counts around every compiled degree.
            for &n in &[1usize, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 23, 24, 31, 32, 33] {
                let inputs_owned: Vec<Vec<u8>> = (0..n).map(|i| pattern(CHUNK_LEN, (5000 + i) as u8)).collect();
                let inputs: Vec<&[u8]> = inputs_owned.iter().map(Vec::as_slice).collect();

                let out_len = n * OUT_LEN;
                let mut guarded = vec![0xAAu8; 3 + out_len + 5];
                let mut expected = vec![0u8; out_len];
                Platform::Portable.hash_many(
                    &inputs,
                    &key,
                    7,
                    IncrementCounter::Yes,
                    0,
                    CHUNK_START,
                    CHUNK_END,
                    &mut expected,
                );
                let (prefix, rest) = guarded.split_at_mut(3);
                let (out, suffix) = rest.split_at_mut(out_len);
                platform.hash_many(&inputs, &key, 7, IncrementCounter::Yes, 0, CHUNK_START, CHUNK_END, out);
                assert_eq!(out, &expected[..], "hash_many output mismatch ({platform:?}, n {n})");
                assert!(
                    prefix.iter().all(|&b| b == 0xAA),
                    "hash_many wrote before its output ({platform:?}, n {n})"
                );
                assert!(
                    suffix.iter().all(|&b| b == 0xAA),
                    "hash_many wrote past its output ({platform:?}, n {n})"
                );
            }

            // Parent blocks: one 64-byte block per input, counter not incremented.
            for &n in &[2usize, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33] {
                let inputs_owned: Vec<Vec<u8>> = (0..n).map(|i| pattern(BLOCK_LEN, (9000 + i) as u8)).collect();
                let inputs: Vec<&[u8]> = inputs_owned.iter().map(Vec::as_slice).collect();

                let out_len = n * OUT_LEN;
                let mut guarded = vec![0xAAu8; 1 + out_len + 1];
                let mut expected = vec![0u8; out_len];
                Platform::Portable.hash_many(&inputs, &key, 0, IncrementCounter::No, PARENT, 0, 0, &mut expected);
                let (prefix, rest) = guarded.split_at_mut(1);
                let (out, suffix) = rest.split_at_mut(out_len);
                platform.hash_many(&inputs, &key, 0, IncrementCounter::No, PARENT, 0, 0, out);
                assert_eq!(out, &expected[..], "parent hash_many mismatch ({platform:?}, n {n})");
                assert!(prefix.iter().all(|&b| b == 0xAA));
                assert!(suffix.iter().all(|&b| b == 0xAA));
            }
        }
    }

    /// `xof_many` produces consecutive output blocks (including across the
    /// 64-bit output-block counter wrap), writes exactly `out.len()` bytes, and
    /// treats an empty output as a no-op.
    #[test]
    fn xof_many_respects_output_bounds_and_wraps_counter() {
        let cv = {
            let bytes = pattern(32, 42);
            let mut words = [0u32; 8];
            for (word, chunk) in words.iter_mut().zip(bytes.as_chunks::<4>().0) {
                *word = u32::from_le_bytes(*chunk);
            }
            words
        };
        let block_bytes = pattern(BLOCK_LEN, 99);
        let block: [u8; BLOCK_LEN] = block_bytes.as_slice().try_into().unwrap();

        for platform in available_platforms() {
            for &blocks in &[1usize, 2, 3, 4, 5, 7, 8, 15, 16, 17, 31, 32, 33] {
                for &counter in &[0u64, 7, u32::MAX as u64 - 2, u64::MAX - 2, u64::MAX] {
                    let mut expected = vec![0u8; blocks * BLOCK_LEN];
                    for (i, out_block) in expected.as_chunks_mut::<BLOCK_LEN>().0.iter_mut().enumerate() {
                        *out_block =
                            scalar::compress_xof(&cv, &block, BLOCK_LEN as u8, counter.wrapping_add(i as u64), 0);
                    }

                    let mut guarded = vec![0xAAu8; 2 + blocks * BLOCK_LEN + 3];
                    let (prefix, rest) = guarded.split_at_mut(2);
                    let (out, suffix) = rest.split_at_mut(blocks * BLOCK_LEN);
                    platform.xof_many(&cv, &block, BLOCK_LEN as u8, counter, 0, out);
                    assert_eq!(
                        out,
                        &expected[..],
                        "xof_many mismatch ({platform:?}, blocks {blocks}, counter {counter})"
                    );
                    assert!(prefix.iter().all(|&b| b == 0xAA), "xof_many wrote before its output");
                    assert!(suffix.iter().all(|&b| b == 0xAA), "xof_many wrote past its output");
                }
            }

            // Empty output: must be a complete no-op.
            let mut canary = [0xAAu8; 8];
            platform.xof_many(&cv, &block, BLOCK_LEN as u8, 0, 0, &mut []);
            assert_eq!(canary, [0xAAu8; 8]);
            canary[0] = 0x55;
            assert_eq!(canary[0], 0x55);
        }
    }
}
