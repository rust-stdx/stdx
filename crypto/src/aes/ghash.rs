/// Pure-Rust GHASH (GF(2¹²⁸) multiplication) for AES-GCM.
///
/// Uses a precomputed 16-entry nibble-lookup table to process GF(2¹²⁸)
/// multiplication 4 bits at a time instead of bit-by-bit.
///
/// ## Representations
///
/// A 128-bit GCM element is stored as `u128` where:
///   – bit 127 (MSB of u128) = coefficient of x^0  (= MSB of the first byte)
///   – bit 0   (LSB of u128) = coefficient of x^127 (= LSB of the last byte)
///
/// This matches `u128::from_be_bytes(block)` exactly.
use super::aes::{encrypt_block, key_expand};
use crate::{Hash, bytes::Bytes};

/// Precomputed bit-reversal lookup table for all 256 byte values.
/// Replaces 16 x reverse_bits() calls with simple table lookups.
const BIT_REVERSE: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        t[i] = (i as u8).reverse_bits();
        i += 1;
    }
    t
};

/// Compute the GCM authentication tag (pure Rust).
pub(crate) fn compute_tag(table: &GhashTable, aad: &[u8], ciphertext: &[u8], ej0: &[u8; 16]) -> Hash {
    let mut tag = Hash(Bytes::<64>::with_length(16));
    let state: &mut [u8; 16] = tag.as_mut().try_into().unwrap();

    ghash_update(state, table, aad);
    ghash_update(state, table, ciphertext);

    let mut len_block = [0u8; 16];
    len_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    len_block[8..].copy_from_slice(&((ciphertext.len() as u64) * 8).to_be_bytes());
    ghash_block(state, table, &len_block);

    for i in 0..16 {
        state[i] ^= ej0[i];
    }

    tag
}

/// Feed an arbitrarily-sized byte slice into GHASH, zero-padding the last
/// block if necessary.
fn ghash_update(state: &mut [u8; 16], table: &GhashTable, data: &[u8]) {
    let mut chunks = data.chunks_exact(16);
    for chunk in chunks.by_ref() {
        let block: [u8; 16] = chunk.try_into().unwrap();
        ghash_block(state, table, &block);
    }
    let rem = chunks.remainder();
    if !rem.is_empty() {
        let mut padded = [0u8; 16];
        padded[..rem.len()].copy_from_slice(rem);
        ghash_block(state, table, &padded);
    }
}

/// Update a running GHASH state by XOR-ing one 16-byte block and multiplying
/// by H (NIST SP 800-38D §6.4).
#[inline(always)]
fn ghash_block(state: &mut [u8; 16], table: &GhashTable, block: &[u8; 16]) {
    for i in 0..16 {
        state[i] ^= block[i];
    }
    let new = ghash_mul_table(table, state);
    *state = new;
}

/// Reverse bits in each byte of a 16-byte block.
/// Converts from GCM's big-endian polynomial representation to
/// PCLMULQDQ's little-endian domain (and vice versa).
#[inline(always)]
pub(crate) fn bitreverse_bytes(block: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = BIT_REVERSE[block[i] as usize];
    }
    out
}

// ── GF(2^128) arithmetic for GHASH ───────────────────────────────────────────

#[inline(always)]
fn gf128_mul_x(v: u128) -> u128 {
    let carry = v & 1;
    let shifted = v >> 1;
    if carry != 0 {
        shifted ^ (0xe1u128 << 120)
    } else {
        shifted
    }
}

/// Multiply two GCM elements in GF(2^128).
/// Both operands and the result use big-endian byte layout (NIST SP 800-38D).
pub(crate) fn gf128_mul(x: &[u8; 16], h: &[u8; 16]) -> [u8; 16] {
    let x_val = u128::from_be_bytes(*x);
    let h_val = u128::from_be_bytes(*h);
    let mut z = 0u128;
    let mut v = h_val;

    for k in (0..128u32).rev() {
        if (x_val >> k) & 1 == 1 {
            z ^= v;
        }
        v = gf128_mul_x(v);
    }
    z.to_be_bytes()
}

// ── GHASH table-based GF(2¹²⁸) multiplication ────────────────────────────────

/// Table of 16 precomputed GHASH field elements (0×H through 15×H).
pub(crate) type GhashTable = [[u8; 16]; 16];

/// Build a nibble-lookup table for a given GHASH subkey H.
pub(crate) fn precompute_ghash_table(h: &[u8; 16]) -> GhashTable {
    let mut tab = [[0u8; 16]; 16];
    let h_val = u128::from_be_bytes(*h);
    tab[8] = *h;
    tab[4] = gf128_mul_x(h_val).to_be_bytes();
    tab[2] = gf128_mul_x(gf128_mul_x(h_val)).to_be_bytes();
    tab[1] = gf128_mul_x(gf128_mul_x(gf128_mul_x(h_val))).to_be_bytes();

    fn be(x: &[u8; 16]) -> u128 {
        u128::from_be_bytes(*x)
    }
    fn tobe(x: u128) -> [u8; 16] {
        x.to_be_bytes()
    }

    tab[12] = tobe(be(&tab[8]) ^ be(&tab[4]));
    tab[10] = tobe(be(&tab[8]) ^ be(&tab[2]));
    tab[9] = tobe(be(&tab[8]) ^ be(&tab[1]));
    tab[6] = tobe(be(&tab[4]) ^ be(&tab[2]));
    tab[5] = tobe(be(&tab[4]) ^ be(&tab[1]));
    tab[3] = tobe(be(&tab[2]) ^ be(&tab[1]));
    tab[14] = tobe(be(&tab[12]) ^ be(&tab[2]));
    tab[13] = tobe(be(&tab[12]) ^ be(&tab[1]));
    tab[11] = tobe(be(&tab[10]) ^ be(&tab[1]));
    tab[7] = tobe(be(&tab[6]) ^ be(&tab[1]));
    tab[15] = tobe(be(&tab[14]) ^ be(&tab[1]));
    tab
}

/// Multiply X by H using a precomputed nibble-lookup table.
#[inline]
fn ghash_mul_table(table: &GhashTable, x: &[u8; 16]) -> [u8; 16] {
    let mut z = 0u128;
    for &byte in x.iter().rev() {
        let hi = (byte >> 4) as usize;
        let lo = (byte & 0xf) as usize;
        for _ in 0..4 {
            z = gf128_mul_x(z);
        }
        z ^= u128::from_be_bytes(table[lo]);
        for _ in 0..4 {
            z = gf128_mul_x(z);
        }
        z ^= u128::from_be_bytes(table[hi]);
    }
    z.to_be_bytes()
}

// ── Precomputation ───────────────────────────────────────────────────────────

/// Precompute GHASH powers H¹ through H⁸ in bit-reversed-per-byte form.
///
/// Returns `([h1_br..h8_br], h_natural)` where:
/// - `h1_br`..`h8_br` are in the bit-reversed domain used by hardware GHASH
/// - `h_natural` is H in the natural big-endian byte order (for software fallback / E(J0))
pub(crate) fn precompute_ghash_powers(key: &[u8; 32]) -> ([[u8; 16]; 8], [u8; 16]) {
    let rk = key_expand(key);
    let h = encrypt_block(&rk, &[0u8; 16]);
    let h2 = gf128_mul(&h, &h);
    let h3 = gf128_mul(&h2, &h);
    let h4 = gf128_mul(&h3, &h);
    let h5 = gf128_mul(&h4, &h);
    let h6 = gf128_mul(&h5, &h);
    let h7 = gf128_mul(&h6, &h);
    let h8 = gf128_mul(&h7, &h);
    (
        [
            bitreverse_bytes(&h),
            bitreverse_bytes(&h2),
            bitreverse_bytes(&h3),
            bitreverse_bytes(&h4),
            bitreverse_bytes(&h5),
            bitreverse_bytes(&h6),
            bitreverse_bytes(&h7),
            bitreverse_bytes(&h8),
        ],
        h,
    )
}
