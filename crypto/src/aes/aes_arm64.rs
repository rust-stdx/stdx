#![allow(unsafe_op_in_unsafe_fn)]

//! aarch64 AES-128 / AES-256 block kernels using ARMv8 Crypto extensions.
//!
//! These are the ARMv8 equivalents of the x86-64 kernels in `aes_amd64.rs`:
//! single-block, 4-block and 8-block interleaved AES encryption, built from
//! `vaeseq_u8` (SubBytes + ShiftRows + AddRoundKey) and `vaesmcq_u8`
//! (MixColumns). Round keys are stored in standard form (no pre-transformation);
//! each round is `vaesmcq_u8(vaeseq_u8(b, zero)) ^ rk[i]`, which avoids the
//! `vaesimcq_u8` key pre-processing. The caller supplies the expanded round
//! keys, so key expansion is not repeated per call.
//!
//! GHASH and CTR live in `ghash_arm64.rs` and `aes_ctr_arm64.rs`.
//!
//! # Safety
//!
//! The `unsafe fn`s below are `#[target_feature(enable = "aes,neon")]`. Callers
//! must ensure the AES extension is present, either by checking
//! `is_aarch64_feature_detected!("aes")` or by compiling for a target with
//! `target_feature = "aes"` (NEON is baseline on aarch64). They must also pass
//! a round-key slice with exactly the const-generic count: 11 for AES-128 and
//! 15 for AES-256.
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

// ── Kernel tests: ARMv8 vs constant-time software fallback ───────────────────

#[cfg(test)]
mod kernel_tests {
    use random::{RngExt, SeedableRng, rngs::ChaChaRng};

    use super::*;
    use crate::aes::aes::{decrypt_block, encrypt_block, expand_key};

    /// Deterministic seeded `ChaChaRng` generator.
    fn hardware_available() -> bool {
        std::arch::is_aarch64_feature_detected!("aes")
    }

    fn to_bytes(v: uint8x16_t) -> [u8; 16] {
        let mut out = [0u8; 16];
        unsafe { vst1q_u8(out.as_mut_ptr(), v) };
        out
    }

    /// Run the ARMv8 single-block kernel on a block held at `offset` inside a
    /// larger buffer, so the load/store paths see an unaligned address.
    fn arm_block_at<const N: usize>(rk: &[uint8x16_t; N], buf: &[u8], offset: usize) -> [u8; 16] {
        unsafe {
            let b = vld1q_u8(buf.as_ptr().add(offset));
            to_bytes(aes_encrypt_block::<N>(rk, b))
        }
    }

    #[test]
    fn arm_expand_key_preserves_software_schedule() {
        if !hardware_available() {
            eprintln!("skipping arm_expand_key_preserves_software_schedule: no AES extension");
            return;
        }
        let mut rng = ChaChaRng::seed_from_u64(0x1111_2222_3333_4444);

        for _ in 0..256 {
            let mut key = [0u8; 16];
            rng.fill(&mut key);
            let soft = expand_key::<11>(&key);
            let arm = expand_key_armv8(soft);
            for i in 0..11 {
                assert_eq!(to_bytes(arm[i]), soft[i], "AES-128 rk[{i}]");
            }

            let mut key = [0u8; 32];
            rng.fill(&mut key);
            let soft = expand_key::<15>(&key);
            let arm = expand_key_armv8(soft);
            for i in 0..15 {
                assert_eq!(to_bytes(arm[i]), soft[i], "AES-256 rk[{i}]");
            }
        }
    }

    #[test]
    fn arm_aes128_matches_software_and_fips197() {
        if !hardware_available() {
            eprintln!("skipping arm_aes128_matches_software_and_fips197: no AES extension");
            return;
        }
        // FIPS 197 C.1.
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        ];
        let pt = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        ];
        let ct = [
            0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a,
        ];
        let soft = expand_key::<11>(&key);
        let rk = expand_key_armv8(soft);
        assert_eq!(arm_block_at(&rk, &pt, 0), ct);
        assert_eq!(encrypt_block::<11>(&soft, &pt), ct);

        // 10k deterministic vectors against the software fallback.
        let mut rng = ChaChaRng::seed_from_u64(0xdead_beef_0000_0001);
        let mut key = [0u8; 16];
        let mut soft = expand_key::<11>(&key);
        let mut arm = expand_key_armv8(soft);
        for i in 0..10_000 {
            if i % 100 == 0 {
                rng.fill(&mut key);
                soft = expand_key::<11>(&key);
                arm = expand_key_armv8(soft);
            }
            let pt = rng.random::<[u8; 16]>();
            let hw = arm_block_at(&arm, &pt, 0);
            let sw = encrypt_block::<11>(&soft, &pt);
            assert_eq!(hw, sw, "AES-128 hardware/software mismatch at vector {i}");
            // No hardware decrypt path exists; check the software one still
            // inverts the hardware ciphertext.
            assert_eq!(decrypt_block::<11>(&soft, &hw), pt);
        }
    }

    #[test]
    fn arm_aes256_matches_software_and_fips197() {
        if !hardware_available() {
            eprintln!("skipping arm_aes256_matches_software_and_fips197: no AES extension");
            return;
        }
        // FIPS 197 C.3.
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11,
            0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let pt = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        ];
        let ct = [
            0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf, 0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49, 0x60, 0x89,
        ];
        let soft = expand_key::<15>(&key);
        let rk = expand_key_armv8(soft);
        assert_eq!(arm_block_at(&rk, &pt, 0), ct);
        assert_eq!(encrypt_block::<15>(&soft, &pt), ct);

        let mut rng = ChaChaRng::seed_from_u64(0xdead_beef_0000_0002);
        let mut key = [0u8; 32];
        let mut soft = expand_key::<15>(&key);
        let mut arm = expand_key_armv8(soft);
        for i in 0..10_000 {
            if i % 100 == 0 {
                rng.fill(&mut key);
                soft = expand_key::<15>(&key);
                arm = expand_key_armv8(soft);
            }
            let pt = rng.random::<[u8; 16]>();
            let hw = arm_block_at(&arm, &pt, 0);
            let sw = encrypt_block::<15>(&soft, &pt);
            assert_eq!(hw, sw, "AES-256 hardware/software mismatch at vector {i}");
            assert_eq!(decrypt_block::<15>(&soft, &hw), pt);
        }
    }

    #[test]
    fn arm_interleaved_kernels_match_single_block_kernel() {
        if !hardware_available() {
            eprintln!("skipping arm_interleaved_kernels_match_single_block_kernel: no AES extension");
            return;
        }
        let mut rng = ChaChaRng::seed_from_u64(0x0bad_c0de_0000_0007);

        for _ in 0..200 {
            let mut key128 = [0u8; 16];
            rng.fill(&mut key128);
            let soft128 = expand_key::<11>(&key128);
            let rk128 = expand_key_armv8(soft128);

            let mut blocks = [[0u8; 16]; 8];
            for b in blocks.iter_mut() {
                *b = rng.random::<[u8; 16]>();
            }
            let expected: [[u8; 16]; 8] = core::array::from_fn(|i| encrypt_block::<11>(&soft128, &blocks[i]));

            let v = [
                unsafe { vld1q_u8(blocks[0].as_ptr()) },
                unsafe { vld1q_u8(blocks[1].as_ptr()) },
                unsafe { vld1q_u8(blocks[2].as_ptr()) },
                unsafe { vld1q_u8(blocks[3].as_ptr()) },
                unsafe { vld1q_u8(blocks[4].as_ptr()) },
                unsafe { vld1q_u8(blocks[5].as_ptr()) },
                unsafe { vld1q_u8(blocks[6].as_ptr()) },
                unsafe { vld1q_u8(blocks[7].as_ptr()) },
            ];
            let out = unsafe { aes_encrypt_8blocks::<11>(&rk128, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]) };
            let got = [
                to_bytes(out.0),
                to_bytes(out.1),
                to_bytes(out.2),
                to_bytes(out.3),
                to_bytes(out.4),
                to_bytes(out.5),
                to_bytes(out.6),
                to_bytes(out.7),
            ];
            assert_eq!(got, expected, "AES-128 8-block kernel");

            let out = unsafe { aes_encrypt_4blocks::<11>(&rk128, v[0], v[1], v[2], v[3]) };
            let got = [to_bytes(out.0), to_bytes(out.1), to_bytes(out.2), to_bytes(out.3)];
            assert_eq!(got, expected[..4], "AES-128 4-block kernel");

            let mut key256 = [0u8; 32];
            rng.fill(&mut key256);
            let soft256 = expand_key::<15>(&key256);
            let rk256 = expand_key_armv8(soft256);
            let expected: [[u8; 16]; 8] = core::array::from_fn(|i| encrypt_block::<15>(&soft256, &blocks[i]));

            let out = unsafe { aes_encrypt_8blocks::<15>(&rk256, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]) };
            let got = [
                to_bytes(out.0),
                to_bytes(out.1),
                to_bytes(out.2),
                to_bytes(out.3),
                to_bytes(out.4),
                to_bytes(out.5),
                to_bytes(out.6),
                to_bytes(out.7),
            ];
            assert_eq!(got, expected, "AES-256 8-block kernel");

            let out = unsafe { aes_encrypt_4blocks::<15>(&rk256, v[0], v[1], v[2], v[3]) };
            let got = [to_bytes(out.0), to_bytes(out.1), to_bytes(out.2), to_bytes(out.3)];
            assert_eq!(got, expected[..4], "AES-256 4-block kernel");
        }
    }

    /// The kernels load 16 bytes from a `uint8x16_t` and store 16 bytes. Place
    /// the block at every unaligned offset of a canary-filled buffer and check
    /// that no byte outside the 16-byte output slot is touched.
    #[test]
    fn arm_unaligned_offsets_do_not_clobber_neighbours() {
        if !hardware_available() {
            eprintln!("skipping arm_unaligned_offsets_do_not_clobber_neighbours: no AES extension");
            return;
        }
        let key = [0x5au8; 16];
        let soft = expand_key::<11>(&key);
        let rk = expand_key_armv8(soft);
        let pt = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        ];
        let expected = encrypt_block::<11>(&soft, &pt);

        for offset in 0..16usize {
            let mut input = [0xa5u8; 48];
            input[offset..offset + 16].copy_from_slice(&pt);
            let got = arm_block_at(&rk, &input, offset);
            assert_eq!(got, expected, "unaligned kernel result at offset {offset}");
            // The input buffer must not have been written to.
            assert_eq!(&input[offset..offset + 16], &pt);
        }

        // Store into the middle of a canary buffer: only the 16-byte slot may
        // change.
        let mut out = [0x5au8; 48];
        let ct = unsafe { aes_encrypt_block::<11>(&rk, vld1q_u8(pt.as_ptr())) };
        unsafe { vst1q_u8(out.as_mut_ptr().add(16), ct) };
        assert_eq!(&out[..16], &[0x5a; 16][..]);
        assert_eq!(&out[16..32], &expected[..]);
        assert_eq!(&out[32..], &[0x5a; 16][..]);
    }
}
