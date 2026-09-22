/// Pure-Rust GHASH (GF(2¹²⁸) multiplication) for AES-GCM.
///
/// ## Constant-time software multiplication
///
/// The portable multiplier is built from **integer multiplications with
/// "holes"** rather than lookup tables: the operands are split into words
/// with three zero bits between every data bit, multiplied, and recombined,
/// which turns the integer product into a carry-less product. The high half
/// of the 128×128 product is obtained by bit-reversing the operands and the
/// result. No table is indexed by secret data and no branch depends on the
/// key or data, so the classical software-GCM cache-timing attack does not
/// apply.
///
/// This property relies on the target CPU providing a constant-time
/// integer multiply. That is true of all recent 32-bit cores (Cortex-M0/M3/
/// M4/M33, RISC-V RV32IM) and 64-bit cores. On a CPU with a variable-time
/// multiply (some old ARM7/ARM9 designs) this implementation would not be
/// constant-time.
///
/// ## Representation
///
/// A 128-bit GCM element is stored as `[u8; 16]` in big-endian byte order.
/// Multiplications use little-endian 32-bit words internally, matching the
/// bit order expected by the carry-less reduction.
use super::aes::{encrypt_block, expand_key};
use crate::{Hash, aes::aes::RoundKeysSoftware, bytes::Bytes};

/// Compute the GCM authentication tag (pure Rust).
pub(crate) fn compute_tag(h: &[u8; 16], aad: &[u8], ciphertext: &[u8], ej0: &[u8; 16]) -> Hash {
    let mut tag = Hash(Bytes::<64>::with_length(16));
    let state: &mut [u8; 16] = tag.as_mut().try_into().unwrap();

    ghash_update(state, h, aad);
    ghash_update(state, h, ciphertext);

    let mut len_block = [0u8; 16];
    len_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    len_block[8..].copy_from_slice(&((ciphertext.len() as u64) * 8).to_be_bytes());
    ghash_block(state, h, &len_block);

    for i in 0..16 {
        state[i] ^= ej0[i];
    }

    tag
}

/// Feed an arbitrarily-sized byte slice into GHASH, zero-padding the last
/// block if necessary.
fn ghash_update(state: &mut [u8; 16], h: &[u8; 16], data: &[u8]) {
    let mut chunks = data.chunks_exact(16);
    for chunk in chunks.by_ref() {
        let block: [u8; 16] = chunk.try_into().unwrap();
        ghash_block(state, h, &block);
    }
    let rem = chunks.remainder();
    if !rem.is_empty() {
        let mut padded = [0u8; 16];
        padded[..rem.len()].copy_from_slice(rem);
        ghash_block(state, h, &padded);
    }
}

/// Update a running GHASH state by XOR-ing one 16-byte block and multiplying
/// by H (NIST SP 800-38D §6.4).
#[inline(always)]
fn ghash_block(state: &mut [u8; 16], h: &[u8; 16], block: &[u8; 16]) {
    for i in 0..16 {
        state[i] ^= block[i];
    }
    *state = gf128_mul(state, h);
}

/// Reverse bits in each byte of a 16-byte block.
/// Converts from GCM's big-endian polynomial representation to
/// PCLMULQDQ's little-endian domain (and vice versa).
#[inline(always)]
pub(crate) fn bitreverse_bytes(block: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = block[i].reverse_bits();
    }
    out
}

// ── Constant-time carry-less multiplication in GF(2¹²⁸) ──────────────────────

/// Carry-less product of two 32-bit words, truncated to 32 bits.
///
/// Splits each operand into four masked words (one per bit class) so that an
/// ordinary integer multiply propagates carries only into the three "hole"
/// bits between data bits, then masks the holes away. This is the standard
/// "integer multiply with holes" technique for constant-time carry-less
/// multiplication.
#[inline(always)]
fn bmul32(x: u32, y: u32) -> u32 {
    let x0 = x & 0x1111_1111;
    let x1 = x & 0x2222_2222;
    let x2 = x & 0x4444_4444;
    let x3 = x & 0x8888_8888;
    let y0 = y & 0x1111_1111;
    let y1 = y & 0x2222_2222;
    let y2 = y & 0x4444_4444;
    let y3 = y & 0x8888_8888;

    let z0 = x0.wrapping_mul(y0) ^ x1.wrapping_mul(y3) ^ x2.wrapping_mul(y2) ^ x3.wrapping_mul(y1);
    let z1 = x0.wrapping_mul(y1) ^ x1.wrapping_mul(y0) ^ x2.wrapping_mul(y3) ^ x3.wrapping_mul(y2);
    let z2 = x0.wrapping_mul(y2) ^ x1.wrapping_mul(y1) ^ x2.wrapping_mul(y0) ^ x3.wrapping_mul(y3);
    let z3 = x0.wrapping_mul(y3) ^ x1.wrapping_mul(y2) ^ x2.wrapping_mul(y1) ^ x3.wrapping_mul(y0);

    (z0 & 0x1111_1111) | (z1 & 0x2222_2222) | (z2 & 0x4444_4444) | (z3 & 0x8888_8888)
}

/// Reverse the 32 bits of a word.
#[inline(always)]
fn rev32(x: u32) -> u32 {
    let x = ((x & 0x5555_5555) << 1) | ((x >> 1) & 0x5555_5555);
    let x = ((x & 0x3333_3333) << 2) | ((x >> 2) & 0x3333_3333);
    let x = ((x & 0x0F0F_0F0F) << 4) | ((x >> 4) & 0x0F0F_0F0F);
    let x = ((x & 0x00FF_00FF) << 8) | ((x >> 8) & 0x00FF_00FF);
    x.rotate_right(16)
}

#[inline(always)]
fn be_word(bytes: &[u8]) -> u32 {
    u32::from_be_bytes(bytes.try_into().unwrap())
}

/// Multiply two GCM elements in GF(2¹²⁸) in constant time.
///
/// Both operands and the result use big-endian byte layout (NIST SP 800-38D).
/// The 128×128 carry-less product is computed with Karatsuba over 32-bit
/// `bmul32` products and reduced modulo `x¹²⁸ + x⁷ + x² + x + 1`.
pub(crate) fn gf128_mul(x: &[u8; 16], h: &[u8; 16]) -> [u8; 16] {
    // Little-endian words, most significant word last.
    let yw = [
        be_word(&x[12..16]),
        be_word(&x[8..12]),
        be_word(&x[4..8]),
        be_word(&x[0..4]),
    ];
    let hw = [
        be_word(&h[12..16]),
        be_word(&h[8..12]),
        be_word(&h[4..8]),
        be_word(&h[0..4]),
    ];
    let ywr = [rev32(yw[0]), rev32(yw[1]), rev32(yw[2]), rev32(yw[3])];
    let hwr = [rev32(hw[0]), rev32(hw[1]), rev32(hw[2]), rev32(hw[3])];

    // Karatsuba: three 64×64 products, each split into four 32×32 products.
    let a = [
        yw[0],
        yw[1],
        yw[2],
        yw[3],
        yw[0] ^ yw[1],
        yw[2] ^ yw[3],
        yw[0] ^ yw[2],
        yw[1] ^ yw[3],
        yw[0] ^ yw[1] ^ yw[2] ^ yw[3],
        ywr[0],
        ywr[1],
        ywr[2],
        ywr[3],
        ywr[0] ^ ywr[1],
        ywr[2] ^ ywr[3],
        ywr[0] ^ ywr[2],
        ywr[1] ^ ywr[3],
        ywr[0] ^ ywr[1] ^ ywr[2] ^ ywr[3],
    ];
    let b = [
        hw[0],
        hw[1],
        hw[2],
        hw[3],
        hw[0] ^ hw[1],
        hw[2] ^ hw[3],
        hw[0] ^ hw[2],
        hw[1] ^ hw[3],
        hw[0] ^ hw[1] ^ hw[2] ^ hw[3],
        hwr[0],
        hwr[1],
        hwr[2],
        hwr[3],
        hwr[0] ^ hwr[1],
        hwr[2] ^ hwr[3],
        hwr[0] ^ hwr[2],
        hwr[1] ^ hwr[3],
        hwr[0] ^ hwr[1] ^ hwr[2] ^ hwr[3],
    ];

    let mut c = [0u32; 18];
    for i in 0..18 {
        c[i] = bmul32(a[i], b[i]);
    }
    c[4] ^= c[0] ^ c[1];
    c[5] ^= c[2] ^ c[3];
    c[8] ^= c[6] ^ c[7];
    c[13] ^= c[9] ^ c[10];
    c[14] ^= c[11] ^ c[12];
    c[17] ^= c[15] ^ c[16];

    // Assemble the 256-bit product (high half via bit reversal).
    let d0 = c[0];
    let d1 = c[4] ^ (rev32(c[9]) >> 1);
    let d2 = c[1] ^ c[0] ^ c[2] ^ c[6] ^ (rev32(c[13]) >> 1);
    let d3 = c[4] ^ c[5] ^ c[8] ^ (rev32(c[10] ^ c[9] ^ c[11] ^ c[15]) >> 1);
    let d4 = c[2] ^ c[1] ^ c[3] ^ c[7] ^ (rev32(c[13] ^ c[14] ^ c[17]) >> 1);
    let d5 = c[5] ^ (rev32(c[11] ^ c[10] ^ c[12] ^ c[16]) >> 1);
    let d6 = c[3] ^ (rev32(c[14]) >> 1);
    let d7 = rev32(c[12]) >> 1;

    let mut zw = [0u32; 8];
    zw[0] = d0 << 1;
    zw[1] = (d1 << 1) | (d0 >> 31);
    zw[2] = (d2 << 1) | (d1 >> 31);
    zw[3] = (d3 << 1) | (d2 >> 31);
    zw[4] = (d4 << 1) | (d3 >> 31);
    zw[5] = (d5 << 1) | (d4 >> 31);
    zw[6] = (d6 << 1) | (d5 >> 31);
    zw[7] = (d7 << 1) | (d6 >> 31);

    // Reduce modulo the GCM polynomial.
    for i in 0..4 {
        let lw = zw[i];
        zw[i + 4] ^= lw ^ (lw >> 1) ^ (lw >> 2) ^ (lw >> 7);
        zw[i + 3] ^= (lw << 31) ^ (lw << 30) ^ (lw << 25);
    }

    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&zw[7].to_be_bytes());
    out[4..8].copy_from_slice(&zw[6].to_be_bytes());
    out[8..12].copy_from_slice(&zw[5].to_be_bytes());
    out[12..16].copy_from_slice(&zw[4].to_be_bytes());
    out
}

// ── Precomputation ───────────────────────────────────────────────────────────

/// Precomputed GHASH powers for AES-GCM — hardware-native or software.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub enum GHashPowers {
    #[cfg(target_arch = "x86_64")]
    X86_64([core::arch::x86_64::__m128i; 8]),
    #[cfg(target_arch = "aarch64")]
    Armv8([core::arch::aarch64::uint8x16_t; 8]),
    /// The platform doesn't support hardware GHASH; stores `H` only.
    Software([u8; 16]),
}

/// Precompute GHASH powers H¹ through H⁸ in bit-reversed-per-byte form.
///
/// Returns `([h1_br..h8_br], h_natural)` where:
/// - `h1_br`..`h8_br` are in the bit-reversed domain used by hardware GHASH
/// - `h_natural` is H in the natural big-endian byte order (for software fallback / E(J0))
///
/// `N` is the number of AES round keys: `11` for AES-128, `15` for AES-256.
pub(crate) fn precompute_ghash_powers<const N: usize>(key: &[u8]) -> ([[u8; 16]; 8], [u8; 16]) {
    const {
        assert!(N == 11 || N == 15);
    }
    let rk: RoundKeysSoftware<N> = expand_key::<N>(key);
    let h = encrypt_block::<N>(&rk, &[0u8; 16]);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple branchy bit-serial reference; only used to cross-check the
    /// constant-time multiplier in tests.
    fn gf128_mul_ref(x: &[u8; 16], h: &[u8; 16]) -> [u8; 16] {
        let x_val = u128::from_be_bytes(*x);
        let mut v = u128::from_be_bytes(*h);
        let mut z = 0u128;
        for k in (0..128u32).rev() {
            if (x_val >> k) & 1 == 1 {
                z ^= v;
            }
            let carry = v & 1;
            v >>= 1;
            if carry != 0 {
                v ^= 0xe1u128 << 120;
            }
        }
        z.to_be_bytes()
    }

    #[test]
    fn gf128_mul_matches_reference() {
        // Deterministic pseudo-random inputs plus edge cases.
        let mut x = [0x01u8; 16];
        let mut h = [0x80u8; 16];
        for _ in 0..256 {
            assert_eq!(gf128_mul(&x, &h), gf128_mul_ref(&x, &h));
            for b in x.iter_mut() {
                *b = b.wrapping_mul(31).wrapping_add(17);
            }
            for b in h.iter_mut() {
                *b = b.wrapping_mul(29).wrapping_add(11);
            }
        }
        let zero = [0u8; 16];
        let ones = [0xffu8; 16];
        assert_eq!(gf128_mul(&zero, &ones), zero);
        assert_eq!(gf128_mul(&ones, &zero), zero);
        assert_eq!(gf128_mul(&ones, &ones), gf128_mul_ref(&ones, &ones));
    }
}
