use crate::Checksum;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub(crate) const PRIME32_1: u64 = 0x9E3779B1;
pub(crate) const PRIME32_3: u64 = 0xC2B2AE3D;

const PRIME64_1: u64 = 0x9E3779B185EBCA87;
const PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
const PRIME64_5: u64 = 0x27D4EB2F165667C5;

const STRIPE_LEN: usize = 64;
const SECRET_CONSUME_RATE: usize = 8;
pub(crate) const ACC_NB: usize = 8;
const SECRET_MERGEACCS_START: usize = 11;
const SECRET_LASTACC_START: usize = 7;
const MID_SIZE_MAX: usize = 240;
const SECRET_SIZE_MIN: usize = 136;
const DEFAULT_SECRET_SIZE: usize = 192;
const STRIPES_PER_BLOCK: usize = (DEFAULT_SECRET_SIZE - STRIPE_LEN) / SECRET_CONSUME_RATE;

/// The default 192-byte secret used by XXH3.
const DEFAULT_SECRET: [u8; 192] = [
    0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c, 0xde, 0xd4, 0x6d,
    0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f, 0xcb, 0x79, 0xe6, 0x4e, 0xcc, 0xc0,
    0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21, 0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43, 0x24, 0x8e, 0xe0,
    0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c, 0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb, 0x88, 0xd0, 0x65, 0x8b,
    0x1b, 0x53, 0x2e, 0xa3, 0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19, 0xef, 0x46, 0xa9, 0xde, 0xac,
    0xd8, 0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7, 0xc7, 0x0b, 0x4f, 0x1d, 0x8a, 0x51,
    0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78, 0x73, 0x64, 0xea, 0xc5, 0xac, 0x83, 0x34,
    0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb, 0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49,
    0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e, 0x2b, 0x16, 0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8,
    0xd1, 0x7a, 0xd0, 0x31, 0xce, 0x45, 0xcb, 0x3a, 0x8f, 0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b,
    0x40, 0x7e,
];

/// 64-byte aligned 8 × u64 accumulator used by all XXH3 paths.
///
/// Alignment matches AVX-512's zmm register size requirement (64 bytes).
/// On other targets the alignment is free and has no downside.
#[repr(align(64))]
#[derive(Clone, Copy)]
pub(crate) struct Acc(pub(crate) [u64; ACC_NB]);

impl core::ops::Deref for Acc {
    type Target = [u64; ACC_NB];

    #[inline]
    fn deref(&self) -> &[u64; ACC_NB] {
        &self.0
    }
}

impl core::ops::DerefMut for Acc {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u64; ACC_NB] {
        &mut self.0
    }
}

const INITIAL_ACC: Acc = Acc([
    PRIME32_3,
    PRIME64_1,
    PRIME64_2,
    0x165667B19E3779F9, // PRIME64_3
    0x85EBCA77C2B2AE63, // PRIME64_4
    0x85EBCA77,         // PRIME32_2
    PRIME64_5,
    PRIME32_1,
]);

// ---------------------------------------------------------------------------
// Low-level helpers
// ---------------------------------------------------------------------------
#[inline]
const fn read_64le(data: &[u8], offset: usize) -> u64 {
    let (_, rest) = data.split_at(offset);
    let arr = match rest.split_first_chunk::<8>() {
        Some((arr, _)) => arr,
        None => panic!("read_64le: out of bounds"),
    };
    u64::from_le_bytes(*arr)
}

#[inline]
const fn read_32le(data: &[u8], offset: usize) -> u32 {
    let (_, rest) = data.split_at(offset);
    let arr = match rest.split_first_chunk::<4>() {
        Some((arr, _)) => arr,
        None => panic!("read_32le: out of bounds"),
    };
    u32::from_le_bytes(*arr)
}
#[inline]
const fn xorshift64(value: u64, shift: u64) -> u64 {
    value ^ (value >> shift)
}

#[inline]
const fn avalanche(mut h: u64) -> u64 {
    h = xorshift64(h, 37);
    h = h.wrapping_mul(0x165667919E3779F9);
    xorshift64(h, 32)
}

/// XXH64 avalanche, used in the 1to3 and empty 0to16 paths.
const fn xxh64_avalanche(mut h: u64) -> u64 {
    h ^= h >> 33;
    h = h.wrapping_mul(PRIME64_2);
    h ^= h >> 29;
    h = h.wrapping_mul(0x165667B19E3779F9);
    h ^= h >> 32;
    h
}

#[inline]
const fn strong_avalanche(mut value: u64, len: u64) -> u64 {
    value ^= value.rotate_left(49) ^ value.rotate_left(24);
    value = value.wrapping_mul(0x9FB21C651E98DF25);
    value ^= (value >> 35).wrapping_add(len);
    value = value.wrapping_mul(0x9FB21C651E98DF25);
    xorshift64(value, 28)
}

#[inline]
const fn mul128_fold64(l: u64, r: u64) -> u64 {
    let p = (l as u128).wrapping_mul(r as u128);
    (p as u64) ^ ((p >> 64) as u64)
}

#[inline]
const fn mult32_to64(left: u32, right: u32) -> u64 {
    (left as u64).wrapping_mul(right as u64)
}

#[inline]
const fn mix16b(input: &[u8], input_offset: usize, secret: &[u8], secret_offset: usize, seed: u64) -> u64 {
    let mut input_lo = read_64le(input, input_offset);
    let mut input_hi = read_64le(input, input_offset + 8);

    input_lo ^= read_64le(secret, secret_offset).wrapping_add(seed);
    input_hi ^= read_64le(secret, secret_offset + 8).wrapping_sub(seed);

    mul128_fold64(input_lo, input_hi)
}

#[inline]
const fn mix_two_accs(acc: &Acc, offset: usize, secret: &[u8], sec_off: usize) -> u64 {
    mul128_fold64(
        acc.0[offset] ^ read_64le(secret, sec_off),
        acc.0[offset + 1] ^ read_64le(secret, sec_off + 8),
    )
}

#[inline]
const fn merge_accs(acc: &Acc, secret: &[u8], start_offset: usize, mut result: u64) -> u64 {
    let mut i = 0;
    while i < 4 {
        result = result.wrapping_add(mix_two_accs(acc, i * 2, secret, start_offset + i * 16));
        i += 1;
    }
    avalanche(result)
}

// ---------------------------------------------------------------------------
// Core XXH3 accumulate / scramble
// ---------------------------------------------------------------------------

#[inline]
const fn accumulate_512_scalar(acc: &mut Acc, input: &[u8], input_off: usize, secret: &[u8], secret_off: usize) {
    let mut i = 0;
    while i < ACC_NB {
        let data_val = read_64le(input, input_off + i * 8);
        let data_key = data_val ^ read_64le(secret, secret_off + i * 8);
        acc.0[i ^ 1] = acc.0[i ^ 1].wrapping_add(data_val);
        acc.0[i] = acc.0[i].wrapping_add(mult32_to64((data_key & 0xFFFFFFFF) as u32, (data_key >> 32) as u32));
        i += 1;
    }
}

#[inline]
const fn accumulate_loop_scalar(
    acc: &mut Acc,
    input: &[u8],
    input_off: usize,
    secret: &[u8],
    secret_off: usize,
    nb_stripes: usize,
) {
    let mut i = 0;
    while i < nb_stripes {
        accumulate_512_scalar(
            acc,
            input,
            input_off + i * STRIPE_LEN,
            secret,
            secret_off + i * SECRET_CONSUME_RATE,
        );
        i += 1;
    }
}

#[inline]
const fn scramble_acc_scalar(acc: &mut Acc, secret: &[u8], secret_off: usize) {
    let mut i = 0;
    while i < ACC_NB {
        let key = read_64le(secret, secret_off + i * 8);
        let mut val = xorshift64(acc.0[i], 47);
        val ^= key;
        acc.0[i] = val.wrapping_mul(PRIME32_1);
        i += 1;
    }
}

#[inline]
const fn hash_long_internal_loop(acc: &mut Acc, input: &[u8], secret: &[u8]) {
    let nb_stripes = STRIPES_PER_BLOCK;
    let block_len = STRIPE_LEN * nb_stripes;
    let nb_blocks = (input.len() - 1) / block_len;

    let mut i = 0;
    while i < nb_blocks {
        accumulate_loop_scalar(acc, input, i * block_len, secret, 0, nb_stripes);
        scramble_acc_scalar(acc, secret, secret.len() - STRIPE_LEN);
        i += 1;
    }

    // Last partial block
    let nb_stripes = ((input.len() - 1) - (block_len * nb_blocks)) / STRIPE_LEN;
    accumulate_loop_scalar(acc, input, nb_blocks * block_len, secret, 0, nb_stripes);

    // Last stripe
    let last_stripe_start = input.len() - STRIPE_LEN;
    let last_secret_off = secret.len() - STRIPE_LEN - SECRET_LASTACC_START;
    accumulate_512_scalar(acc, input, last_stripe_start, secret, last_secret_off);
}

// ---------------------------------------------------------------------------
// Custom default secret generation for seeded hashing
// ---------------------------------------------------------------------------

const fn custom_default_secret_scalar(seed: u64) -> [u8; DEFAULT_SECRET_SIZE] {
    let mut result = [0u8; DEFAULT_SECRET_SIZE];
    let nb_rounds = DEFAULT_SECRET_SIZE / 16;
    let mut i = 0;
    while i < nb_rounds {
        let lo = read_64le(&DEFAULT_SECRET, i * 16).wrapping_add(seed);
        let hi = read_64le(&DEFAULT_SECRET, i * 16 + 8).wrapping_sub(seed);
        let lo_bytes = lo.to_le_bytes();
        let hi_bytes = hi.to_le_bytes();
        let mut j = 0;
        while j < 8 {
            result[i * 16 + j] = lo_bytes[j];
            result[i * 16 + 8 + j] = hi_bytes[j];
            j += 1;
        }
        i += 1;
    }
    result
}

// ---------------------------------------------------------------------------
// Dispatch wrappers — select SIMD or scalar at compile / run time.
// On aarch64  NEON is always available (compile-time dispatch).
// On x86_64   Uses AVX512 when available, otherwise AVX2 (compile-time dispatch).
// ---------------------------------------------------------------------------

#[inline]
#[allow(unreachable_code)]
fn accumulate_512(acc: &mut Acc, input: &[u8], input_off: usize, secret: &[u8], secret_off: usize) {
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        crate::xxh3_wasm_simd128::accumulate_512(acc, input, input_off, secret, secret_off);
        return;
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        unsafe { crate::xxh3_neon::accumulate_512(acc, input, input_off, secret, secret_off) };
        return;
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
    {
        unsafe { crate::xxh3_avx512::accumulate_512(acc, input, input_off, secret, secret_off) };
        return;
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
    {
        unsafe { crate::xxh3_avx2::accumulate_512(acc, input, input_off, secret, secret_off) };
        return;
    }

    accumulate_512_scalar(acc, input, input_off, secret, secret_off);
}

#[inline]
#[allow(unreachable_code)]
fn accumulate_loop(acc: &mut Acc, input: &[u8], input_off: usize, secret: &[u8], secret_off: usize, nb_stripes: usize) {
    let mut idx = 0;
    while idx < nb_stripes {
        accumulate_512(
            acc,
            input,
            input_off + idx * STRIPE_LEN,
            secret,
            secret_off + idx * SECRET_CONSUME_RATE,
        );
        idx += 1;
    }
}

#[inline]
#[allow(unreachable_code)]
fn scramble_acc(acc: &mut Acc, secret: &[u8], secret_off: usize) {
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        crate::xxh3_wasm_simd128::scramble_acc(acc, secret, secret_off);
        return;
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        unsafe { crate::xxh3_neon::scramble_acc(acc, secret, secret_off) };
        return;
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
    {
        unsafe { crate::xxh3_avx512::scramble_acc(acc, secret, secret_off) };
        return;
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
    {
        unsafe { crate::xxh3_avx2::scramble_acc(acc, secret, secret_off) };
        return;
    }

    scramble_acc_scalar(acc, secret, secret_off);
}

#[inline]
fn custom_default_secret(seed: u64) -> [u8; DEFAULT_SECRET_SIZE] {
    custom_default_secret_scalar(seed)
}

// ---------------------------------------------------------------------------
// One-shot: 0..16 bytes
// ---------------------------------------------------------------------------

const fn xxh3_64_1to3(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    let len = input.len();
    let c1 = input[0] as u32;
    let c2 = input[len >> 1] as u32;
    let c3 = input[len - 1] as u32;
    let combined = (c1 << 16) | (c2 << 24) | (c3) | ((len as u32) << 8);
    let flip = ((read_32le(secret, 0) ^ read_32le(secret, 4)) as u64).wrapping_add(seed);
    xxh64_avalanche((combined as u64) ^ flip)
}

const fn xxh3_64_4to8(input: &[u8], mut seed: u64, secret: &[u8]) -> u64 {
    seed ^= ((seed as u32).swap_bytes() as u64) << 32;
    let len = input.len();
    let input1 = read_32le(input, 0);
    let input2 = read_32le(input, len - 4);
    let flip = (read_64le(secret, 8) ^ read_64le(secret, 16)).wrapping_sub(seed);
    let input64 = (input2 as u64).wrapping_add((input1 as u64) << 32);
    strong_avalanche(input64 ^ flip, len as u64)
}

const fn xxh3_64_9to16(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    let len = input.len();
    let flip1 = (read_64le(secret, 24) ^ read_64le(secret, 32)).wrapping_add(seed);
    let flip2 = (read_64le(secret, 40) ^ read_64le(secret, 48)).wrapping_sub(seed);
    let input_lo = read_64le(input, 0) ^ flip1;
    let input_hi = read_64le(input, len - 8) ^ flip2;
    let acc = (len as u64)
        .wrapping_add(input_lo.swap_bytes())
        .wrapping_add(input_hi)
        .wrapping_add(mul128_fold64(input_lo, input_hi));
    avalanche(acc)
}

const fn xxh3_64_0to16(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    if input.len() > 8 {
        xxh3_64_9to16(input, seed, secret)
    } else if input.len() >= 4 {
        xxh3_64_4to8(input, seed, secret)
    } else if !input.is_empty() {
        xxh3_64_1to3(input, seed, secret)
    } else {
        xxh64_avalanche(seed ^ read_64le(secret, 56) ^ read_64le(secret, 64))
    }
}

// ---------------------------------------------------------------------------
// One-shot: 17..128 bytes
// ---------------------------------------------------------------------------

const fn xxh3_64_7to128(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    let len = input.len();
    let mut acc = (len as u64).wrapping_mul(PRIME64_1);

    if len > 32 {
        if len > 64 {
            if len > 96 {
                acc = acc.wrapping_add(mix16b(input, 48, secret, 96, seed));
                acc = acc.wrapping_add(mix16b(input, len - 64, secret, 112, seed));
            }
            acc = acc.wrapping_add(mix16b(input, 32, secret, 64, seed));
            acc = acc.wrapping_add(mix16b(input, len - 48, secret, 80, seed));
        }
        acc = acc.wrapping_add(mix16b(input, 16, secret, 32, seed));
        acc = acc.wrapping_add(mix16b(input, len - 32, secret, 48, seed));
    }

    acc = acc.wrapping_add(mix16b(input, 0, secret, 0, seed));
    acc = acc.wrapping_add(mix16b(input, len - 16, secret, 16, seed));

    avalanche(acc)
}

// ---------------------------------------------------------------------------
// One-shot: 129..240 bytes
// ---------------------------------------------------------------------------

const fn xxh3_64_129to240(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    let len = input.len();
    let mut acc = (len as u64).wrapping_mul(PRIME64_1);
    let nb_rounds = len / 16;

    let mut i = 0;
    while i < 8 {
        acc = acc.wrapping_add(mix16b(input, i * 16, secret, i * 16, seed));
        i += 1;
    }
    acc = avalanche(acc);

    i = 8;
    while i < nb_rounds {
        acc = acc.wrapping_add(mix16b(input, i * 16, secret, (i - 8) * 16 + 3, seed));
        i += 1;
    }

    acc = acc.wrapping_add(mix16b(input, len - 16, secret, SECRET_SIZE_MIN - 17, seed));

    avalanche(acc)
}

// ---------------------------------------------------------------------------
// Long path: >240 bytes (one-shot)
// ---------------------------------------------------------------------------

const fn xxh3_64_long_impl(input: &[u8], secret: &[u8]) -> u64 {
    let mut acc = INITIAL_ACC;
    hash_long_internal_loop(&mut acc, input, secret);
    merge_accs(
        &acc,
        secret,
        SECRET_MERGEACCS_START,
        (input.len() as u64).wrapping_mul(PRIME64_1),
    )
}

const fn xxh3_64_long_with_seed(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    if seed == 0 {
        xxh3_64_long_impl(input, secret)
    } else {
        xxh3_64_long_impl(input, &custom_default_secret_scalar(seed))
    }
}

const fn xxh3_64_one_shot(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    if input.len() <= 16 {
        xxh3_64_0to16(input, seed, secret)
    } else if input.len() <= 128 {
        xxh3_64_7to128(input, seed, secret)
    } else if input.len() <= MID_SIZE_MAX {
        xxh3_64_129to240(input, seed, secret)
    } else {
        xxh3_64_long_with_seed(input, seed, secret)
    }
}

// ---------------------------------------------------------------------------
// 128-bit: helper
// ---------------------------------------------------------------------------

#[inline(always)]
#[allow(clippy::too_many_arguments)]
const fn mix32_b(
    lo: &mut u64,
    hi: &mut u64,
    input: &[u8],
    input1_off: usize,
    input2_off: usize,
    secret: &[u8],
    secret_off: usize,
    seed: u64,
) {
    *lo = lo.wrapping_add(mix16b(input, input1_off, secret, secret_off, seed));
    *lo ^= read_64le(input, input2_off).wrapping_add(read_64le(input, input2_off + 8));
    *hi = hi.wrapping_add(mix16b(input, input2_off, secret, secret_off + 16, seed));
    *hi ^= read_64le(input, input1_off).wrapping_add(read_64le(input, input1_off + 8));
}

// ---------------------------------------------------------------------------
// 128-bit: short paths (return u128)
// ---------------------------------------------------------------------------

const fn xxh3_128_1to3(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    let len = input.len();
    let c1 = input[0] as u32;
    let c2 = input[len >> 1] as u32;
    let c3 = input[len - 1] as u32;
    let combinedl = ((c1) << 16) | (c2 << 24) | (c3) | ((len as u32) << 8);
    let combinedh = combinedl.swap_bytes().rotate_left(13);
    let bitflipl = ((read_32le(secret, 0) ^ read_32le(secret, 4)) as u64).wrapping_add(seed);
    let bitfliph = ((read_32le(secret, 8) ^ read_32le(secret, 12)) as u64).wrapping_sub(seed);
    let keyed_lo = (combinedl as u64) ^ bitflipl;
    let keyed_hi = (combinedh as u64) ^ bitfliph;
    ((xxh64_avalanche(keyed_hi) as u128) << 64) | (xxh64_avalanche(keyed_lo) as u128)
}

const fn xxh3_128_9to16(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    let len = input.len();
    let bitflipl = (read_64le(secret, 32) ^ read_64le(secret, 40)).wrapping_sub(seed);
    let bitfliph = (read_64le(secret, 48) ^ read_64le(secret, 56)).wrapping_add(seed);
    let input_lo = read_64le(input, 0);
    let mut input_hi = read_64le(input, len - 8);

    let m128_full = (input_lo ^ input_hi ^ bitflipl) as u128 * PRIME64_1 as u128;
    let mut m128_lo = m128_full as u64;
    let mut m128_hi = (m128_full >> 64) as u64;

    m128_lo = m128_lo.wrapping_add(((len - 1) as u64) << 54);
    input_hi ^= bitfliph;
    m128_hi = m128_hi
        .wrapping_add(input_hi)
        .wrapping_add(mult32_to64(input_hi as u32, 0x85EBCA76u32));

    m128_lo ^= m128_hi.swap_bytes();

    let h128_full = (m128_lo as u128).wrapping_mul(PRIME64_2 as u128);
    let h128_lo = h128_full as u64;
    let h128_hi = ((h128_full >> 64) as u64).wrapping_add(m128_hi.wrapping_mul(PRIME64_2));

    ((avalanche(h128_hi) as u128) << 64) | (avalanche(h128_lo) as u128)
}

const fn xxh3_128_4to8_return(input: &[u8], mut seed: u64, secret: &[u8]) -> u128 {
    seed ^= ((seed as u32).swap_bytes() as u64) << 32;
    let len = input.len();
    let input_lo = read_32le(input, 0);
    let input_hi = read_32le(input, len - 4);
    let input_64 = (input_lo as u64).wrapping_add((input_hi as u64) << 32);
    let bitflip = (read_64le(secret, 16) ^ read_64le(secret, 24)).wrapping_add(seed);
    let keyed = input_64 ^ bitflip;
    let m128 = (keyed as u128).wrapping_mul(PRIME64_1.wrapping_add((len as u64) << 2) as u128);
    let mut m128_lo = m128 as u64;
    let mut m128_hi = (m128 >> 64) as u64;
    m128_hi = m128_hi.wrapping_add(m128_lo << 1);
    m128_lo ^= m128_hi >> 3;
    m128_lo = xorshift64(m128_lo, 35);
    m128_lo = m128_lo.wrapping_mul(0x9FB21C651E98DF25);
    m128_lo = xorshift64(m128_lo, 28);
    m128_hi = avalanche(m128_hi);
    ((m128_hi as u128) << 64) | (m128_lo as u128)
}

const fn xxh3_128_0to16(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    if input.len() > 8 {
        xxh3_128_9to16(input, seed, secret)
    } else if input.len() >= 4 {
        xxh3_128_4to8_return(input, seed, secret)
    } else if !input.is_empty() {
        xxh3_128_1to3(input, seed, secret)
    } else {
        let flip_lo = read_64le(secret, 64) ^ read_64le(secret, 72);
        let flip_hi = read_64le(secret, 80) ^ read_64le(secret, 88);
        (xxh64_avalanche(seed ^ flip_lo) as u128) | ((xxh64_avalanche(seed ^ flip_hi) as u128) << 64)
    }
}

// ---------------------------------------------------------------------------
// 128-bit: medium paths (17..128), (129..240) — return u128
// ---------------------------------------------------------------------------

const fn xxh3_128_7to128(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    let len = input.len();
    let mut lo = (len as u64).wrapping_mul(PRIME64_1);
    let mut hi: u64 = 0;

    if len > 32 {
        if len > 64 {
            if len > 96 {
                mix32_b(&mut lo, &mut hi, input, 48, len - 64, secret, 96, seed);
            }
            mix32_b(&mut lo, &mut hi, input, 32, len - 48, secret, 64, seed);
        }
        mix32_b(&mut lo, &mut hi, input, 16, len - 32, secret, 32, seed);
    }

    mix32_b(&mut lo, &mut hi, input, 0, len - 16, secret, 0, seed);

    (avalanche(lo.wrapping_add(hi)) as u128)
        | ((0u64.wrapping_sub(avalanche(
            lo.wrapping_mul(PRIME64_1)
                .wrapping_add(hi.wrapping_mul(0x85EBCA77C2B2AE63))
                .wrapping_add((len as u64).wrapping_sub(seed).wrapping_mul(PRIME64_2)),
        )) as u128)
            << 64)
}

const fn xxh3_128_129to240(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    let len = input.len();
    let nb_rounds = len / 32;
    let mut lo = (len as u64).wrapping_mul(PRIME64_1);
    let mut hi: u64 = 0;

    let mut i = 0;
    while i < 4 {
        let offset = 32 * i;
        mix32_b(&mut lo, &mut hi, input, offset, offset + 16, secret, offset, seed);
        i += 1;
    }

    lo = avalanche(lo);
    hi = avalanche(hi);

    i = 4;
    while i < nb_rounds {
        mix32_b(&mut lo, &mut hi, input, 32 * i, 32 * i + 16, secret, 3 + 32 * (i - 4), seed);
        i += 1;
    }

    mix32_b(
        &mut lo,
        &mut hi,
        input,
        len - 16,
        len - 32,
        secret,
        SECRET_SIZE_MIN - 17 - 16,
        0u64.wrapping_sub(seed),
    );

    (avalanche(lo.wrapping_add(hi)) as u128)
        | ((0u64.wrapping_sub(avalanche(
            lo.wrapping_mul(PRIME64_1)
                .wrapping_add(hi.wrapping_mul(0x85EBCA77C2B2AE63))
                .wrapping_add((len as u64).wrapping_sub(seed).wrapping_mul(PRIME64_2)),
        )) as u128)
            << 64)
}

// ---------------------------------------------------------------------------
// Long path: 128-bit (>240 bytes)
// ---------------------------------------------------------------------------

const fn xxh3_128_long_impl(input: &[u8], secret: &[u8]) -> u128 {
    let mut acc = INITIAL_ACC;
    hash_long_internal_loop(&mut acc, input, secret);

    let lo = merge_accs(
        &acc,
        secret,
        SECRET_MERGEACCS_START,
        (input.len() as u64).wrapping_mul(PRIME64_1),
    );
    let hi = merge_accs(
        &acc,
        secret,
        secret.len() - ACC_NB * 8 - SECRET_MERGEACCS_START,
        !(input.len() as u64).wrapping_mul(PRIME64_2),
    );

    (lo as u128) | ((hi as u128) << 64)
}

const fn xxh3_128_long_with_seed(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    if seed == 0 {
        xxh3_128_long_impl(input, secret)
    } else {
        xxh3_128_long_impl(input, &custom_default_secret_scalar(seed))
    }
}

const fn xxh3_128_one_shot(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    if input.len() <= 16 {
        xxh3_128_0to16(input, seed, secret)
    } else if input.len() <= 128 {
        xxh3_128_7to128(input, seed, secret)
    } else if input.len() <= MID_SIZE_MAX {
        xxh3_128_129to240(input, seed, secret)
    } else {
        xxh3_128_long_with_seed(input, seed, secret)
    }
}

// ---------------------------------------------------------------------------
// Streaming state for long inputs (>240 bytes)
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct LongState {
    acc: Acc,
    secret: [u8; DEFAULT_SECRET_SIZE],
    buf: [u8; STRIPE_LEN],
    buf_len: u8,
    nb_stripes_acc: usize,
    total_len: u64,
}

impl LongState {
    fn new(secret: &[u8; DEFAULT_SECRET_SIZE]) -> Self {
        LongState {
            acc: INITIAL_ACC,
            secret: *secret,
            buf: [0u8; STRIPE_LEN],
            buf_len: 0,
            nb_stripes_acc: 0,
            total_len: 0,
        }
    }

    fn update(&mut self, mut data: &[u8]) {
        self.total_len += data.len() as u64;

        if self.buf_len > 0 {
            let take = (STRIPE_LEN - self.buf_len as usize).min(data.len());
            self.buf[self.buf_len as usize..self.buf_len as usize + take].copy_from_slice(&data[..take]);
            self.buf_len += take as u8;
            data = &data[take..];
            if self.buf_len as usize == STRIPE_LEN {
                accumulate_512(
                    &mut self.acc,
                    &self.buf,
                    0,
                    &self.secret,
                    self.nb_stripes_acc * SECRET_CONSUME_RATE,
                );
                self.nb_stripes_acc += 1;
                self.buf_len = 0;
            }
        }

        // Process full stripes, leaving enough room for the last stripe
        // in the buffer (matches C reference's (len - 1) / STRIPE_LEN logic)
        while data.len() > STRIPE_LEN {
            let nb_stripes = (data.len() - 1) / STRIPE_LEN;
            let nb = nb_stripes.min(STRIPES_PER_BLOCK - self.nb_stripes_acc);
            if nb == 0 {
                break;
            }
            accumulate_loop(
                &mut self.acc,
                data,
                0,
                &self.secret,
                self.nb_stripes_acc * SECRET_CONSUME_RATE,
                nb,
            );
            self.nb_stripes_acc += nb;
            if self.nb_stripes_acc >= STRIPES_PER_BLOCK {
                scramble_acc(&mut self.acc, &self.secret, DEFAULT_SECRET_SIZE - STRIPE_LEN);
                self.nb_stripes_acc = 0;
            }
            data = &data[nb * STRIPE_LEN..];
        }

        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len() as u8;
        }
    }

    fn sum_64b(self) -> u64 {
        let mut acc = self.acc;

        if self.buf_len > 0 {
            let mut last_stripe = [0u8; STRIPE_LEN];
            last_stripe[..self.buf_len as usize].copy_from_slice(&self.buf[..self.buf_len as usize]);
            let copy_len = (SECRET_LASTACC_START).min(STRIPE_LEN - self.buf_len as usize);
            let secret_tail_start = DEFAULT_SECRET_SIZE - SECRET_LASTACC_START;
            last_stripe[STRIPE_LEN - SECRET_LASTACC_START..STRIPE_LEN - SECRET_LASTACC_START + copy_len]
                .copy_from_slice(&self.secret[secret_tail_start..secret_tail_start + copy_len]);
            let sec_off = DEFAULT_SECRET_SIZE - STRIPE_LEN - SECRET_LASTACC_START;
            accumulate_512(&mut acc, &last_stripe, 0, &self.secret, sec_off);
        }

        merge_accs(
            &acc,
            &self.secret,
            SECRET_MERGEACCS_START,
            (self.total_len).wrapping_mul(PRIME64_1),
        )
    }

    fn sum_128b(self) -> u128 {
        let mut acc = self.acc;

        if self.buf_len > 0 {
            let mut last_stripe = [0u8; STRIPE_LEN];
            last_stripe[..self.buf_len as usize].copy_from_slice(&self.buf[..self.buf_len as usize]);
            let copy_len = (SECRET_LASTACC_START).min(STRIPE_LEN - self.buf_len as usize);
            let secret_tail_start = DEFAULT_SECRET_SIZE - SECRET_LASTACC_START;
            last_stripe[STRIPE_LEN - SECRET_LASTACC_START..STRIPE_LEN - SECRET_LASTACC_START + copy_len]
                .copy_from_slice(&self.secret[secret_tail_start..secret_tail_start + copy_len]);
            let sec_off = DEFAULT_SECRET_SIZE - STRIPE_LEN - SECRET_LASTACC_START;
            accumulate_512(&mut acc, &last_stripe, 0, &self.secret, sec_off);
        }

        let lo = merge_accs(
            &acc,
            &self.secret,
            SECRET_MERGEACCS_START,
            (self.total_len).wrapping_mul(PRIME64_1),
        );
        let hi = merge_accs(
            &acc,
            &self.secret,
            DEFAULT_SECRET_SIZE - ACC_NB * 8 - SECRET_MERGEACCS_START,
            !(self.total_len).wrapping_mul(PRIME64_2),
        );

        (lo as u128) | ((hi as u128) << 64)
    }
}

// ---------------------------------------------------------------------------
// Public struct: Xxh3_64
// ---------------------------------------------------------------------------

/// XXH3 64-bit hash.
///
/// A fast non-cryptographic hash using the XXH3 algorithm. Supports an
/// optional `u64` seed and an optional custom 192-byte secret.
///
/// # Example
///
/// ```rust
/// use xxhash::{Xxh3_64, Checksum};
///
/// let hash = Xxh3_64::checksum(b"hello world");
/// ```
#[derive(Clone)]
pub struct Xxh3_64 {
    seed: u64,
    secret: [u8; DEFAULT_SECRET_SIZE],
    buf: [u8; MID_SIZE_MAX],
    buf_len: usize,
    long: Option<LongState>,
}

impl Xxh3_64 {
    /// Create a new XXH3-64 hasher with the given seed.
    #[inline]
    pub const fn with_seed(seed: u64) -> Self {
        Xxh3_64 {
            seed,
            secret: DEFAULT_SECRET,
            buf: [0u8; MID_SIZE_MAX],
            buf_len: 0,
            long: None,
        }
    }

    /// Create a new XXH3-64 hasher with a custom 192-byte secret (seed = 0).
    #[inline]
    pub const fn with_secret(secret: [u8; DEFAULT_SECRET_SIZE]) -> Self {
        Xxh3_64 {
            seed: 0,
            secret,
            buf: [0u8; MID_SIZE_MAX],
            buf_len: 0,
            long: None,
        }
    }

    /// Create a new XXH3-64 hasher with a custom seed and secret.
    #[inline]
    pub const fn with_seed_and_secret(seed: u64, secret: [u8; DEFAULT_SECRET_SIZE]) -> Self {
        Xxh3_64 {
            seed,
            secret,
            buf: [0u8; MID_SIZE_MAX],
            buf_len: 0,
            long: None,
        }
    }
}

impl Checksum for Xxh3_64 {
    type Output = u64;

    fn new() -> Self {
        Self::with_seed(0)
    }

    fn checksum(data: &[u8]) -> Self::Output {
        if data.len() <= MID_SIZE_MAX {
            xxh3_64(data)
        } else {
            let mut hasher = Self::new();
            hasher.update(data);
            hasher.sum()
        }
    }

    fn update(&mut self, data: &[u8]) {
        if let Some(long) = &mut self.long {
            long.update(data);
            return;
        }

        let mut remaining = data;

        if self.buf_len < MID_SIZE_MAX {
            let take = (MID_SIZE_MAX - self.buf_len).min(remaining.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&remaining[..take]);
            self.buf_len += take;
            remaining = &remaining[take..];
        }

        if self.buf_len >= MID_SIZE_MAX && !remaining.is_empty() {
            let long_secret = if self.seed == 0 {
                self.secret
            } else {
                custom_default_secret(self.seed)
            };
            let mut long = LongState::new(&long_secret);
            long.update(&self.buf[..self.buf_len]);
            self.long = Some(long);
            self.buf_len = 0;

            if !remaining.is_empty() {
                self.long.as_mut().unwrap().update(remaining);
            }
        }
    }

    fn sum(self) -> Self::Output {
        if let Some(long) = self.long {
            return long.sum_64b();
        }
        xxh3_64_one_shot(&self.buf[..self.buf_len], self.seed, &self.secret)
    }
}

impl core::fmt::Debug for Xxh3_64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Xxh3_64").finish()
    }
}

impl Default for Xxh3_64 {
    #[inline]
    fn default() -> Self {
        Self::with_seed(0)
    }
}

// ---------------------------------------------------------------------------
// Public struct: Xxh3_128
// ---------------------------------------------------------------------------

/// XXH3 128-bit hash.
///
/// A fast non-cryptographic hash using the XXH3-128 algorithm. Supports an
/// optional `u64` seed and an optional custom 192-byte secret.
///
/// # Example
///
/// ```rust
/// use xxhash::{Xxh3_128, Checksum};
///
/// let hash = Xxh3_128::checksum(b"hello world");
/// ```
#[derive(Clone)]
pub struct Xxh3_128 {
    seed: u64,
    secret: [u8; DEFAULT_SECRET_SIZE],
    buf: [u8; MID_SIZE_MAX],
    buf_len: usize,
    long: Option<LongState>,
}

impl Xxh3_128 {
    /// Create a new XXH3-128 hasher with the given seed.
    #[inline]
    pub const fn with_seed(seed: u64) -> Self {
        Xxh3_128 {
            seed,
            secret: DEFAULT_SECRET,
            buf: [0u8; MID_SIZE_MAX],
            buf_len: 0,
            long: None,
        }
    }

    /// Create a new XXH3-128 hasher with a custom 192-byte secret (seed = 0).
    #[inline]
    pub const fn with_secret(secret: [u8; DEFAULT_SECRET_SIZE]) -> Self {
        Xxh3_128 {
            seed: 0,
            secret,
            buf: [0u8; MID_SIZE_MAX],
            buf_len: 0,
            long: None,
        }
    }

    /// Create a new XXH3-128 hasher with a custom seed and secret.
    #[inline]
    pub const fn with_seed_and_secret(seed: u64, secret: [u8; DEFAULT_SECRET_SIZE]) -> Self {
        Xxh3_128 {
            seed,
            secret,
            buf: [0u8; MID_SIZE_MAX],
            buf_len: 0,
            long: None,
        }
    }
}

impl Checksum for Xxh3_128 {
    type Output = u128;

    fn new() -> Self {
        Self::with_seed(0)
    }

    fn checksum(data: &[u8]) -> Self::Output {
        if data.len() <= MID_SIZE_MAX {
            xxh3_128(data)
        } else {
            let mut hasher = Self::new();
            hasher.update(data);
            hasher.sum()
        }
    }

    fn update(&mut self, data: &[u8]) {
        if let Some(long) = &mut self.long {
            long.update(data);
            return;
        }

        let mut remaining = data;

        if self.buf_len < MID_SIZE_MAX {
            let take = (MID_SIZE_MAX - self.buf_len).min(remaining.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&remaining[..take]);
            self.buf_len += take;
            remaining = &remaining[take..];
        }

        if self.buf_len >= MID_SIZE_MAX && !remaining.is_empty() {
            let long_secret = if self.seed == 0 {
                self.secret
            } else {
                custom_default_secret(self.seed)
            };
            let mut long = LongState::new(&long_secret);
            long.update(&self.buf[..self.buf_len]);
            self.long = Some(long);
            self.buf_len = 0;

            if !remaining.is_empty() {
                self.long.as_mut().unwrap().update(remaining);
            }
        }
    }

    fn sum(self) -> Self::Output {
        if let Some(long) = self.long {
            return long.sum_128b();
        }
        xxh3_128_one_shot(&self.buf[..self.buf_len], self.seed, &self.secret)
    }
}

impl core::fmt::Debug for Xxh3_128 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Xxh3_128").finish()
    }
}

impl Default for Xxh3_128 {
    #[inline]
    fn default() -> Self {
        Self::with_seed(0)
    }
}

// ---------------------------------------------------------------------------
// Standalone const one-shot functions
// ---------------------------------------------------------------------------

/// Compute the XXH3 64-bit hash of `data` in a single call.
///
/// Available as a `const fn` for compile-time hashing.
/// Uses seed=0 and the default XXH3 secret.
///
/// # Example
///
/// ```rust
/// use xxhash::xxh3_64;
///
/// let hash: u64 = xxh3_64(b"hello");
/// assert_eq!(hash, 0x9555E8555C62DCFD);
/// ```
#[inline]
pub const fn xxh3_64(data: &[u8]) -> u64 {
    xxh3_64_one_shot(data, 0, &DEFAULT_SECRET)
}

/// Compute the XXH3 128-bit hash of `data` in a single call.
///
/// Available as a `const fn` for compile-time hashing.
/// Uses seed=0 and the default XXH3 secret.
///
/// # Example
///
/// ```rust
/// use xxhash::xxh3_128;
///
/// let hash: u128 = xxh3_128(b"hello");
/// assert_eq!(hash, 0xB5E9C1AD071B3E7FC779CFAA5E523818);
/// ```
#[inline]
pub const fn xxh3_128(data: &[u8]) -> u128 {
    xxh3_128_one_shot(data, 0, &DEFAULT_SECRET)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Checksum, test_helpers::fill_test_buffer};

    /// PRIME64 used in the C reference sanity check as seed (0x9E3779B185EBCA8D).
    const TEST_SEED: u64 = 0x9E3779B185EBCA8D;
    /// PRIME32 used in the C reference sanity check as seed for XXH128 vectors.
    const TEST_SEED32: u64 = 0x9E3779B1;

    // --- Xxh3_64 tests ---

    #[test]
    fn test_xh3_64_empty() {
        assert_eq!(Xxh3_64::checksum(b""), 0x2D06800538D394C2);
    }

    #[test]
    fn test_xh3_64_hello() {
        assert_eq!(Xxh3_64::checksum(b"hello"), 0x9555E8555C62DCFD);
    }

    #[test]
    fn test_xh3_64_fox() {
        assert_eq!(
            Xxh3_64::checksum(b"The quick brown fox jumps over the lazy dog"),
            0xCE7D19A5418FB365
        );
    }

    /// Official XXH3-64 test vectors from `xsum_sanity_check.c`, covering
    /// all one-shot paths (0to16, 7to128, 129to240) with both seed=0 and
    /// seed=PRIME64.
    #[test]
    fn test_xh3_64_official_vectors() {
        let cases: &[(usize, u64, u64)] = &[
            (0, 0, 0x2D06800538D394C2),
            (0, TEST_SEED, 0xA8A6B918B2F0364A),
            (1, 0, 0xC44BDFF4074EECDB),
            (1, TEST_SEED, 0x032BE332DD766EF8),
            (6, 0, 0x27B56A84CD2D7325),
            (6, TEST_SEED, 0x84589C116AB59AB9),
            (12, 0, 0xA713DAF0DFBB77E7),
            (12, TEST_SEED, 0xE7303E1B2336DE0E),
            (24, 0, 0xA3FE70BF9D3510EB),
            (24, TEST_SEED, 0x850E80FC35BDD690),
            (48, 0, 0x397DA259ECBA1F11),
            (48, TEST_SEED, 0xADC2CBAA44ACC616),
            (80, 0, 0xBCDEFBBB2C47C90A),
            (80, TEST_SEED, 0xC6DD0CB699532E73),
            (195, 0, 0xCD94217EE362EC3A),
            (195, TEST_SEED, 0xBA68003D370CB3D9),
        ];

        for &(len, seed, expected) in cases {
            let buf = fill_test_buffer(len);
            let mut h = Xxh3_64::with_seed(seed);
            h.update(&buf);
            assert_eq!(h.sum(), expected, "XXH3-64 length {len} seed {seed:#x}");
        }
    }

    /// Byte-at-a-time incremental produces the same result as one-shot.
    #[test]
    fn test_xh3_64_byte_at_a_time() {
        let cases: &[(usize, u64, u64)] = &[
            (0, 0, 0x2D06800538D394C2),
            (1, TEST_SEED, 0x032BE332DD766EF8),
            (12, 0, 0xA713DAF0DFBB77E7),
            (80, TEST_SEED, 0xC6DD0CB699532E73),
        ];

        for &(len, seed, expected) in cases {
            let buf = fill_test_buffer(len);
            let mut h = Xxh3_64::with_seed(seed);
            for b in &buf {
                h.update(&[*b]);
            }
            assert_eq!(h.sum(), expected, "XXH3-64 byte-at-a-time length {len} seed {seed:#x}");
        }
    }

    /// Boundary sizes that exercise all one-shot code paths.
    #[test]
    fn test_xh3_64_boundaries() {
        let sizes: &[(usize, u64)] = &[
            // 0to16 path
            (0, 0x2D06800538D394C2),
            (1, 0xC44BDFF4074EECDB),
            (3, 0x54247382A8D6B94D),
            (4, 0xE5DC74BC51848A51),
            (8, 0x24CCC9ACAA9F65E4),
            (16, 0x981B17D36C7498C9),
            // 7to128 path
            (17, 0x796F5ACD3A60F862),
            (32, 0x9FEADDBDBF57EED3),
            (64, 0x9CB48487720EC49D),
            (128, 0xFCFF24126754D861),
            // 129to240 path
            (129, 0x98F1B0A679A2CA29),
            (240, 0x81C3C2B67F568CCF),
        ];

        for &(len, expected) in sizes {
            let buf = fill_test_buffer(len);
            let mut h = Xxh3_64::new();
            h.update(&buf);
            assert_eq!(h.sum(), expected, "XXH3-64 boundary length {len}");
        }
    }

    #[test]
    fn test_xh3_64_incremental() {
        let mut h = Xxh3_64::new();
        h.update(b"The quick brown ");
        h.update(b"fox jumps over ");
        h.update(b"the lazy dog");
        assert_eq!(h.sum(), 0xCE7D19A5418FB365);
    }

    #[test]
    fn test_xh3_64_seeded() {
        let mut h = Xxh3_64::with_seed(42);
        h.update(b"hello");
        assert_eq!(h.sum(), 0xBAFA072F07DB7937);
    }

    #[test]
    fn test_xh3_64_long_incremental() {
        let mut h = Xxh3_64::new();
        let data = [0x55u8; 512];
        h.update(&data[..200]);
        h.update(&data[200..400]);
        h.update(&data[400..]);
        assert_eq!(h.sum(), 0x4C1155EA5825B659);
    }

    // --- Xxh3_128 tests ---

    #[test]
    fn test_xh3_128_empty() {
        assert_eq!(Xxh3_128::checksum(b""), 0x99AA06D3014798D86001C324468D497F);
    }

    #[test]
    fn test_xh3_128_hello() {
        assert_eq!(Xxh3_128::checksum(b"hello"), 0xB5E9C1AD071B3E7FC779CFAA5E523818);
    }

    #[test]
    fn test_xh3_128_fox() {
        assert_eq!(
            Xxh3_128::checksum(b"The quick brown fox jumps over the lazy dog"),
            0xDDD650205CA3E7FA24A1CC2E3A8A7651
        );
    }

    /// Official XXH128 test vectors from `xsum_sanity_check.c`, covering
    /// all one-shot paths with seed=0 and seed=PRIME32.
    #[test]
    fn test_xh3_128_official_vectors() {
        let cases: &[(usize, u64, u128)] = &[
            (0, 0, 0x99AA06D3014798D86001C324468D497F),
            (0, TEST_SEED32, 0x92220AE55E14AB505444F7869C671AB0),
            (1, 0, 0xA6CD5E9392000F6AC44BDFF4074EECDB),
            (1, TEST_SEED32, 0x89B99554BA22467CB53D5557E7F76F8D),
            (6, 0, 0x082AFE0B8162D12A3E7039BDDA43CFC6),
            (6, TEST_SEED32, 0x5A865B5389ABD2B1269D8F70BE98856E),
            (12, 0, 0x6E3EFD8FC7802B18061A192713F69AD9),
            (12, TEST_SEED32, 0xD7E09D518A3405D39BE9F9A67F3C7DFB),
            (24, 0, 0x0CE966E4678D37611E7044D28B1B901D),
            (24, TEST_SEED32, 0x3162026714A6A243D7304C54EBAD40A9),
            (48, 0, 0xA002AC4E5478227EF942219AED80F67B),
            (48, TEST_SEED32, 0x163ADDE36C0722957BA3C3E453A1934E),
            (81, 0, 0x4952F58181AB00425E8BAFB9F95FB803),
            (81, TEST_SEED32, 0x2724EC7ADC750FB6703FBB3D7A5F755C),
            (222, 0, 0x337E09641B948717F1AEBD597CEC6B3A),
            (222, TEST_SEED32, 0x91820016621E97F1AE995BB8AF917A8D),
        ];

        for &(len, seed, expected) in cases {
            let buf = fill_test_buffer(len);
            let mut h = Xxh3_128::with_seed(seed);
            h.update(&buf);
            assert_eq!(h.sum(), expected, "XXH128 length {len} seed {seed:#x}");
        }
    }

    /// Byte-at-a-time incremental produces the same result as one-shot.
    #[test]
    fn test_xh3_128_byte_at_a_time() {
        let cases: &[(usize, u64, u128)] = &[
            (0, 0, 0x99AA06D3014798D86001C324468D497F),
            (1, TEST_SEED32, 0x89B99554BA22467CB53D5557E7F76F8D),
            (12, 0, 0x6E3EFD8FC7802B18061A192713F69AD9),
        ];

        for &(len, seed, expected) in cases {
            let buf = fill_test_buffer(len);
            let mut h = Xxh3_128::with_seed(seed);
            for b in &buf {
                h.update(&[*b]);
            }
            assert_eq!(h.sum(), expected, "XXH128 byte-at-a-time length {len} seed {seed:#x}");
        }
    }

    /// Boundary sizes for XXH128 (all within one-shot paths).
    #[test]
    fn test_xh3_128_boundaries() {
        let sizes: &[(usize, u128)] = &[
            (0, 0x99AA06D3014798D86001C324468D497F),
            (1, 0xA6CD5E9392000F6AC44BDFF4074EECDB),
            (3, 0x20EFC49FF02422EA54247382A8D6B94D),
            (4, 0x970D585AC632BF8E2E7D8D6876A39FE9),
            (8, 0x47A7F080D82BB45664C69CAB4BB21DC5),
            (16, 0xC68C368ECF8A9C05562980258A998629),
            (17, 0x955FA78643ED3669ABBC12D11973D7DB),
            (32, 0x98FC6458710DC2E8278410A17595E3F9),
            (240, 0xAA4202DAA2769DC85C9AAE94C8EBE5A0),
        ];

        for &(len, expected) in sizes {
            let buf = fill_test_buffer(len);
            let mut h = Xxh3_128::new();
            h.update(&buf);
            assert_eq!(h.sum(), expected, "XXH128 boundary length {len}");
        }
    }

    #[test]
    fn test_xh3_128_incremental() {
        let mut h = Xxh3_128::new();
        h.update(b"The quick brown ");
        h.update(b"fox jumps over ");
        h.update(b"the lazy dog");
        assert_eq!(h.sum(), 0xDDD650205CA3E7FA24A1CC2E3A8A7651);
    }

    #[test]
    fn test_xh3_128_long_incremental() {
        let mut h = Xxh3_128::new();
        let data = [0x55u8; 512];
        h.update(&data[..200]);
        h.update(&data[200..400]);
        h.update(&data[400..]);
        assert_eq!(h.sum(), 0x0DD2485B0318DEF24C1155EA5825B659);
    }

    /// The `const fn` one-shot produces the same result as the trait-based
    /// [`Checksum::checksum`] and is usable at compile time.
    #[test]
    fn test_const_fn_xh3_64() {
        assert_eq!(xxh3_64(b""), Xxh3_64::checksum(b""));
        assert_eq!(xxh3_64(b"hello"), Xxh3_64::checksum(b"hello"));
        assert_eq!(
            xxh3_64(b"The quick brown fox jumps over the lazy dog"),
            Xxh3_64::checksum(b"The quick brown fox jumps over the lazy dog")
        );
        // Test up to 240 bytes (below the streaming threshold where both
        // paths agree).
        let buf = &[0x42u8; 200];
        assert_eq!(xxh3_64(buf), Xxh3_64::checksum(buf));
    }

    /// The `const fn` one-shot produces the same result as the trait-based
    /// [`Checksum::checksum`] and is usable at compile time.
    #[test]
    fn test_const_fn_xh3_128() {
        assert_eq!(xxh3_128(b""), Xxh3_128::checksum(b""));
        assert_eq!(xxh3_128(b"hello"), Xxh3_128::checksum(b"hello"));
        assert_eq!(
            xxh3_128(b"The quick brown fox jumps over the lazy dog"),
            Xxh3_128::checksum(b"The quick brown fox jumps over the lazy dog")
        );
        let buf = &[0x42u8; 200];
        assert_eq!(xxh3_128(buf), Xxh3_128::checksum(buf));
    }
}
