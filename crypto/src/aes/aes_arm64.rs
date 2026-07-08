#![allow(unsafe_op_in_unsafe_fn)]

/// aarch64 AES-256-GCM using ARMv8 Crypto extensions.
///
/// Same 8/4-block parallel CTR and aggregated GHASH strategy as the
/// x86_64 path (see `aes_amd64.rs` for details). The ARMv8 equivalents:
/// - `vaeseq_u8` / `vaesmcq_u8` for AES
/// - `vmull_p64` intrinsic for carry-less multiplication
/// - `vrbitq_u8` for per-byte bit reversal
/// - `vqtbl1q_u8` for byte permutation (counter swap)
///
/// Round keys are stored in standard form (no pre-transformation).
/// Each AES round is `vaesmcq_u8(vaeseq_u8(b, zero)) ^ rk[i]`,
/// which avoids the need for `vaesimcq_u8` key pre-processing.
///
/// The caller supplies precomputed round keys and GHASH powers,
/// eliminating key expansion and H derivation from every call.
use core::arch::aarch64::*;

pub(crate) fn expand_key_armv8<const N: usize>(round_keys_software: [[u8; 16]; N]) -> [uint8x16_t; N] {
    const { assert!(N == 11 || N == 15) };

    let mut round_keys = [unsafe { vdupq_n_u8(0) }; N];
    for i in 0..N {
        round_keys[i] = unsafe { vld1q_u8(round_keys_software[i].as_ptr()) };
    }
    round_keys
}

/// Const generic AES encrypt block for AES-128 (N=11) and AES-256 (N=15).
#[target_feature(enable = "aes,neon")]
#[inline]
pub(crate) unsafe fn aes_encrypt_block<const N: usize>(round_keys: &[uint8x16_t; N], block: uint8x16_t) -> uint8x16_t {
    const { assert!(N == 11 || N == 15) };

    let zero = vdupq_n_u8(0);

    let mut b = veorq_u8(block, round_keys[0]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[1]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[2]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[3]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[4]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[5]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[6]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[7]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[8]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, round_keys[9]);

    if N == 11 {
        b = vaeseq_u8(b, zero);
        veorq_u8(b, round_keys[10])
    } else {
        let p = round_keys.as_ptr();
        b = vaesmcq_u8(vaeseq_u8(b, zero));
        b = veorq_u8(b, *p.add(10));
        b = vaesmcq_u8(vaeseq_u8(b, zero));
        b = veorq_u8(b, *p.add(11));
        b = vaesmcq_u8(vaeseq_u8(b, zero));
        b = veorq_u8(b, *p.add(12));
        b = vaesmcq_u8(vaeseq_u8(b, zero));
        b = veorq_u8(b, *p.add(13));
        b = vaeseq_u8(b, zero);
        veorq_u8(b, *p.add(14))
    }
}

/// Const generic interleaved AES-encrypt 8 blocks.
///
/// All 8 blocks progress through each AES round together, hiding the 3-4 cycle
/// latency of `vaeseq_u8`/`vaesmcq_u8` across blocks.
#[target_feature(enable = "aes,neon")]
#[inline]
pub(crate) unsafe fn aes_encrypt_8blocks<const N: usize>(
    rk: &[uint8x16_t; N],
    b1: uint8x16_t,
    b2: uint8x16_t,
    b3: uint8x16_t,
    b4: uint8x16_t,
    b5: uint8x16_t,
    b6: uint8x16_t,
    b7: uint8x16_t,
    b8: uint8x16_t,
) -> (
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
) {
    const { assert!(N == 11 || N == 15) };

    let zero = vdupq_n_u8(0);

    // Round 0: AddRoundKey
    let b1 = veorq_u8(b1, rk[0]);
    let b2 = veorq_u8(b2, rk[0]);
    let b3 = veorq_u8(b3, rk[0]);
    let b4 = veorq_u8(b4, rk[0]);
    let b5 = veorq_u8(b5, rk[0]);
    let b6 = veorq_u8(b6, rk[0]);
    let b7 = veorq_u8(b7, rk[0]);
    let b8 = veorq_u8(b8, rk[0]);

    // Rounds 1-9 (SubBytes+ShiftRows, MixColumns, AddRoundKey)
    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[1]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[1]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[1]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[1]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[1]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[1]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[1]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[1]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[2]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[2]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[2]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[2]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[2]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[2]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[2]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[2]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[3]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[3]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[3]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[3]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[3]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[3]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[3]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[3]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[4]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[4]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[4]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[4]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[4]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[4]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[4]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[4]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[5]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[5]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[5]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[5]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[5]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[5]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[5]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[5]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[6]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[6]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[6]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[6]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[6]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[6]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[6]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[6]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[7]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[7]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[7]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[7]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[7]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[7]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[7]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[7]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[8]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[8]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[8]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[8]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[8]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[8]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[8]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[8]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[9]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[9]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[9]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[9]);
    let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[9]);
    let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[9]);
    let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[9]);
    let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[9]);

    if N == 11 {
        (
            veorq_u8(vaeseq_u8(b1, zero), rk[10]),
            veorq_u8(vaeseq_u8(b2, zero), rk[10]),
            veorq_u8(vaeseq_u8(b3, zero), rk[10]),
            veorq_u8(vaeseq_u8(b4, zero), rk[10]),
            veorq_u8(vaeseq_u8(b5, zero), rk[10]),
            veorq_u8(vaeseq_u8(b6, zero), rk[10]),
            veorq_u8(vaeseq_u8(b7, zero), rk[10]),
            veorq_u8(vaeseq_u8(b8, zero), rk[10]),
        )
    } else {
        // AES-256: extra rounds 10-13, then final round 14
        let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[10]);
        let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[10]);
        let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[10]);
        let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[10]);
        let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[10]);
        let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[10]);
        let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[10]);
        let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[10]);

        let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[11]);
        let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[11]);
        let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[11]);
        let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[11]);
        let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[11]);
        let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[11]);
        let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[11]);
        let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[11]);

        let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[12]);
        let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[12]);
        let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[12]);
        let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[12]);
        let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[12]);
        let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[12]);
        let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[12]);
        let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[12]);

        let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[13]);
        let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[13]);
        let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[13]);
        let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[13]);
        let b5 = veorq_u8(vaesmcq_u8(vaeseq_u8(b5, zero)), rk[13]);
        let b6 = veorq_u8(vaesmcq_u8(vaeseq_u8(b6, zero)), rk[13]);
        let b7 = veorq_u8(vaesmcq_u8(vaeseq_u8(b7, zero)), rk[13]);
        let b8 = veorq_u8(vaesmcq_u8(vaeseq_u8(b8, zero)), rk[13]);

        (
            veorq_u8(vaeseq_u8(b1, zero), rk[14]),
            veorq_u8(vaeseq_u8(b2, zero), rk[14]),
            veorq_u8(vaeseq_u8(b3, zero), rk[14]),
            veorq_u8(vaeseq_u8(b4, zero), rk[14]),
            veorq_u8(vaeseq_u8(b5, zero), rk[14]),
            veorq_u8(vaeseq_u8(b6, zero), rk[14]),
            veorq_u8(vaeseq_u8(b7, zero), rk[14]),
            veorq_u8(vaeseq_u8(b8, zero), rk[14]),
        )
    }
}

/// Const generic interleaved AES-encrypt 4 blocks.
#[target_feature(enable = "aes,neon")]
#[inline]
pub(crate) unsafe fn aes_encrypt_4blocks<const N: usize>(
    rk: &[uint8x16_t; N],
    b1: uint8x16_t,
    b2: uint8x16_t,
    b3: uint8x16_t,
    b4: uint8x16_t,
) -> (uint8x16_t, uint8x16_t, uint8x16_t, uint8x16_t) {
    const { assert!(N == 11 || N == 15) };

    let zero = vdupq_n_u8(0);

    let b1 = veorq_u8(b1, rk[0]);
    let b2 = veorq_u8(b2, rk[0]);
    let b3 = veorq_u8(b3, rk[0]);
    let b4 = veorq_u8(b4, rk[0]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[1]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[1]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[1]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[1]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[2]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[2]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[2]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[2]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[3]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[3]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[3]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[3]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[4]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[4]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[4]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[4]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[5]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[5]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[5]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[5]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[6]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[6]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[6]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[6]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[7]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[7]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[7]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[7]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[8]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[8]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[8]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[8]);

    let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[9]);
    let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[9]);
    let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[9]);
    let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[9]);

    if N == 11 {
        (
            veorq_u8(vaeseq_u8(b1, zero), rk[10]),
            veorq_u8(vaeseq_u8(b2, zero), rk[10]),
            veorq_u8(vaeseq_u8(b3, zero), rk[10]),
            veorq_u8(vaeseq_u8(b4, zero), rk[10]),
        )
    } else {
        let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[10]);
        let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[10]);
        let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[10]);
        let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[10]);

        let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[11]);
        let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[11]);
        let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[11]);
        let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[11]);

        let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[12]);
        let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[12]);
        let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[12]);
        let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[12]);

        let b1 = veorq_u8(vaesmcq_u8(vaeseq_u8(b1, zero)), rk[13]);
        let b2 = veorq_u8(vaesmcq_u8(vaeseq_u8(b2, zero)), rk[13]);
        let b3 = veorq_u8(vaesmcq_u8(vaeseq_u8(b3, zero)), rk[13]);
        let b4 = veorq_u8(vaesmcq_u8(vaeseq_u8(b4, zero)), rk[13]);

        (
            veorq_u8(vaeseq_u8(b1, zero), rk[14]),
            veorq_u8(vaeseq_u8(b2, zero), rk[14]),
            veorq_u8(vaeseq_u8(b3, zero), rk[14]),
            veorq_u8(vaeseq_u8(b4, zero), rk[14]),
        )
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use hex;

    use super::*;

    fn make_rk(key: &[u8; 32]) -> [uint8x16_t; 15] {
        let soft = crate::aes::aes::expand_key::<15>(key);
        let mut rk = [unsafe { vdupq_n_u8(0) }; 15];
        for i in 0..15 {
            rk[i] = unsafe { vld1q_u8(soft[i].as_ptr()) };
        }
        rk
    }

    #[test]
    fn arm_aes256_ecb_vector() {
        let key: [u8; 32] =
            hex::decode_array::<32>(b"603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4").unwrap();
        let pt: [u8; 16] = hex::decode_array::<16>(b"6bc1bee22e409f96e93d7e117393172a").unwrap();
        let expected: [u8; 16] = hex::decode_array::<16>(b"f3eed1bdb5d2a03c064b5a7e3db181f8").unwrap();

        let rk = make_rk(&key);
        let ct = unsafe { aes_encrypt_block(&rk, vld1q_u8(pt.as_ptr())) };
        let mut out = [0u8; 16];
        unsafe { vst1q_u8(out.as_mut_ptr(), ct) };

        assert_eq!(out, expected);
    }
}
