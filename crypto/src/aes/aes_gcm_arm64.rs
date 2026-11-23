#![allow(unsafe_op_in_unsafe_fn)]

/// aarch64 AES-GCM using ARMv8 Crypto extensions (GCM path).
///
/// `N` is the number of round keys: 11 for AES-128, 15 for AES-256.
///
/// Same 8/4-block parallel CTR and aggregated GHASH strategy as the
/// x86_64 path. The ARMv8 equivalents:
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

use constant_time_eq::constant_time_eq;

use super::{
    aes::GCM_MAX_LEN,
    aes_arm64::{aes_encrypt_4blocks, aes_encrypt_8blocks, aes_encrypt_block},
    aes_ctr_arm64::{SWAP_MASK, increment_counter},
    ghash_arm64::{clmul_gcm_pmull, ghash_4blocks, ghash_8blocks, ghash_update},
};
use crate::{AeadError, Hash, bytes::Bytes};

// ── Encrypt ───────────────────────────────────────────────────────────────────

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn gcm_encrypt_armv8<const N: usize>(
    round_keys: &[uint8x16_t; N],
    h_powers: &[uint8x16_t; 8],
    in_out: &mut [u8],
    nonce: &[u8; 12],
    aad: &[u8],
) -> Hash {
    const {
        assert!(N == 11 || N == 15);
    }

    assert!(
        in_out.len() as u64 <= GCM_MAX_LEN,
        "GCM plaintext exceeds maximum allowed length (2^32 - 2 blocks)"
    );

    let mut j0 = [0u8; 16];
    j0[..12].copy_from_slice(nonce);
    j0[15] = 1;

    let ej0 = aes_encrypt_block::<N>(round_keys, vld1q_u8(j0.as_ptr()));
    let mut ej0_bytes = [0u8; 16];
    vst1q_u8(ej0_bytes.as_mut_ptr(), ej0);

    // CTR starts at J0 + 1
    let ctr = increment_counter(vld1q_u8(j0.as_ptr()));

    let swap = vld1q_u8(SWAP_MASK.as_ptr());
    let mut base = vqtbl1q_u8(ctr, swap);
    let zero = vdupq_n_u32(0);
    let one = vsetq_lane_u32(1, vdupq_n_u32(0), 0);
    let two = vsetq_lane_u32(2, vdupq_n_u32(0), 0);
    let three = vsetq_lane_u32(3, vdupq_n_u32(0), 0);
    let four = vsetq_lane_u32(4, vdupq_n_u32(0), 0);
    let five = vsetq_lane_u32(5, vdupq_n_u32(0), 0);
    let six = vsetq_lane_u32(6, vdupq_n_u32(0), 0);
    let seven = vsetq_lane_u32(7, vdupq_n_u32(0), 0);

    let mut state = vdupq_n_u8(0);
    state = ghash_update(state, h_powers[0], aad);

    let n = in_out.len();
    let mut i = 0usize;

    // 8-block fused pipeline
    while i + 128 <= n {
        let c1 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), zero)), swap);
        let c2 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), one)), swap);
        let c3 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), two)), swap);
        let c4 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), three)), swap);
        let c5 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), four)), swap);
        let c6 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), five)), swap);
        let c7 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), six)), swap);
        let c8 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), seven)), swap);

        let (k1, k2, k3, k4, k5, k6, k7, k8) = aes_encrypt_8blocks::<N>(round_keys, c1, c2, c3, c4, c5, c6, c7, c8);

        let p1 = vld1q_u8(in_out.as_ptr().add(i));
        let p2 = vld1q_u8(in_out.as_ptr().add(i + 16));
        let p3 = vld1q_u8(in_out.as_ptr().add(i + 32));
        let p4 = vld1q_u8(in_out.as_ptr().add(i + 48));
        let p5 = vld1q_u8(in_out.as_ptr().add(i + 64));
        let p6 = vld1q_u8(in_out.as_ptr().add(i + 80));
        let p7 = vld1q_u8(in_out.as_ptr().add(i + 96));
        let p8 = vld1q_u8(in_out.as_ptr().add(i + 112));

        let ct1 = veorq_u8(p1, k1);
        let ct2 = veorq_u8(p2, k2);
        let ct3 = veorq_u8(p3, k3);
        let ct4 = veorq_u8(p4, k4);
        let ct5 = veorq_u8(p5, k5);
        let ct6 = veorq_u8(p6, k6);
        let ct7 = veorq_u8(p7, k7);
        let ct8 = veorq_u8(p8, k8);

        vst1q_u8(in_out.as_mut_ptr().add(i), ct1);
        vst1q_u8(in_out.as_mut_ptr().add(i + 16), ct2);
        vst1q_u8(in_out.as_mut_ptr().add(i + 32), ct3);
        vst1q_u8(in_out.as_mut_ptr().add(i + 48), ct4);
        vst1q_u8(in_out.as_mut_ptr().add(i + 64), ct5);
        vst1q_u8(in_out.as_mut_ptr().add(i + 80), ct6);
        vst1q_u8(in_out.as_mut_ptr().add(i + 96), ct7);
        vst1q_u8(in_out.as_mut_ptr().add(i + 112), ct8);

        state = ghash_8blocks(state, ct1, ct2, ct3, ct4, ct5, ct6, ct7, ct8, h_powers);

        base = vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), vsetq_lane_u32(8, vdupq_n_u32(0), 0)));
        i += 128;
    }

    // 4-block fused pipeline
    while i + 64 <= n {
        let c1 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), zero)), swap);
        let c2 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), one)), swap);
        let c3 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), two)), swap);
        let c4 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), three)), swap);

        let (k1, k2, k3, k4) = aes_encrypt_4blocks::<N>(round_keys, c1, c2, c3, c4);

        let p1 = vld1q_u8(in_out.as_ptr().add(i));
        let p2 = vld1q_u8(in_out.as_ptr().add(i + 16));
        let p3 = vld1q_u8(in_out.as_ptr().add(i + 32));
        let p4 = vld1q_u8(in_out.as_ptr().add(i + 48));

        let ct1 = veorq_u8(p1, k1);
        let ct2 = veorq_u8(p2, k2);
        let ct3 = veorq_u8(p3, k3);
        let ct4 = veorq_u8(p4, k4);

        vst1q_u8(in_out.as_mut_ptr().add(i), ct1);
        vst1q_u8(in_out.as_mut_ptr().add(i + 16), ct2);
        vst1q_u8(in_out.as_mut_ptr().add(i + 32), ct3);
        vst1q_u8(in_out.as_mut_ptr().add(i + 48), ct4);

        state = ghash_4blocks(state, ct1, ct2, ct3, ct4, h_powers);

        base = vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), four));
        i += 64;
    }

    // Tail blocks (1-3)
    while i + 16 <= n {
        let k = aes_encrypt_block::<N>(round_keys, vqtbl1q_u8(base, swap));
        let ct = veorq_u8(vld1q_u8(in_out.as_ptr().add(i)), k);
        vst1q_u8(in_out.as_mut_ptr().add(i), ct);

        let block = vrbitq_u8(ct);
        state = clmul_gcm_pmull(veorq_u8(state, block), h_powers[0]);

        base = vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), one));
        i += 16;
    }
    if i < n {
        let k = aes_encrypt_block::<N>(round_keys, vqtbl1q_u8(base, swap));
        let mut buf = [0u8; 16];
        buf[..n - i].copy_from_slice(&in_out[i..]);
        let ct = veorq_u8(vld1q_u8(buf.as_ptr()), k);
        let mut out_buf = [0u8; 16];
        vst1q_u8(out_buf.as_mut_ptr(), ct);
        in_out[i..].copy_from_slice(&out_buf[..n - i]);

        let mut padded = [0u8; 16];
        padded[..n - i].copy_from_slice(&in_out[i..]);
        let block = vrbitq_u8(vld1q_u8(padded.as_ptr()));
        state = clmul_gcm_pmull(veorq_u8(state, block), h_powers[0]);
    }

    // Length block
    let mut len_block = [0u8; 16];
    len_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    len_block[8..].copy_from_slice(&((in_out.len() as u64) * 8).to_be_bytes());
    let len_br = vrbitq_u8(vld1q_u8(len_block.as_ptr()));
    state = clmul_gcm_pmull(veorq_u8(state, len_br), h_powers[0]);

    // Tag
    let mut tag = Bytes::<64>::with_length(16);
    let tag_neon = veorq_u8(vrbitq_u8(state), vld1q_u8(ej0_bytes.as_ptr()));
    vst1q_u8(tag.as_mut().as_mut_ptr(), tag_neon);
    Hash(tag)
}

// ── Decrypt ───────────────────────────────────────────────────────────────────

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn gcm_decrypt_armv8<const N: usize>(
    round_keys: &[uint8x16_t; N],
    h_powers: &[uint8x16_t; 8],
    in_out: &mut [u8],
    tag: &[u8; 16],
    nonce: &[u8; 12],
    aad: &[u8],
) -> Result<(), AeadError> {
    const {
        assert!(N == 11 || N == 15);
    }

    if in_out.len() as u64 > GCM_MAX_LEN {
        return Err(AeadError::InvalidCiphertext);
    }

    let mut j0 = [0u8; 16];
    j0[..12].copy_from_slice(nonce);
    j0[15] = 1;

    let ej0 = aes_encrypt_block::<N>(round_keys, vld1q_u8(j0.as_ptr()));
    let mut ej0_bytes = [0u8; 16];
    vst1q_u8(ej0_bytes.as_mut_ptr(), ej0);

    // CTR starts at J0 + 1
    let ctr = increment_counter(vld1q_u8(j0.as_ptr()));

    // ── Single-pass fused GHASH + CTR decrypt ────────────────────────────────
    let swap = vld1q_u8(SWAP_MASK.as_ptr());
    let mut base = vqtbl1q_u8(ctr, swap);
    let zero = vdupq_n_u32(0);
    let one = vsetq_lane_u32(1, vdupq_n_u32(0), 0);
    let two = vsetq_lane_u32(2, vdupq_n_u32(0), 0);
    let three = vsetq_lane_u32(3, vdupq_n_u32(0), 0);
    let four = vsetq_lane_u32(4, vdupq_n_u32(0), 0);
    let five = vsetq_lane_u32(5, vdupq_n_u32(0), 0);
    let six = vsetq_lane_u32(6, vdupq_n_u32(0), 0);
    let seven = vsetq_lane_u32(7, vdupq_n_u32(0), 0);

    let mut state = vdupq_n_u8(0);
    state = ghash_update(state, h_powers[0], aad);

    let n = in_out.len();
    let mut i = 0usize;

    // 8-block fused pipeline
    while i + 128 <= n {
        let c1 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), zero)), swap);
        let c2 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), one)), swap);
        let c3 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), two)), swap);
        let c4 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), three)), swap);
        let c5 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), four)), swap);
        let c6 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), five)), swap);
        let c7 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), six)), swap);
        let c8 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), seven)), swap);

        let (k1, k2, k3, k4, k5, k6, k7, k8) = aes_encrypt_8blocks::<N>(round_keys, c1, c2, c3, c4, c5, c6, c7, c8);

        let ct1 = vld1q_u8(in_out.as_ptr().add(i));
        let ct2 = vld1q_u8(in_out.as_ptr().add(i + 16));
        let ct3 = vld1q_u8(in_out.as_ptr().add(i + 32));
        let ct4 = vld1q_u8(in_out.as_ptr().add(i + 48));
        let ct5 = vld1q_u8(in_out.as_ptr().add(i + 64));
        let ct6 = vld1q_u8(in_out.as_ptr().add(i + 80));
        let ct7 = vld1q_u8(in_out.as_ptr().add(i + 96));
        let ct8 = vld1q_u8(in_out.as_ptr().add(i + 112));

        // GHASH ciphertext first, then decrypt
        state = ghash_8blocks(state, ct1, ct2, ct3, ct4, ct5, ct6, ct7, ct8, h_powers);

        vst1q_u8(in_out.as_mut_ptr().add(i), veorq_u8(ct1, k1));
        vst1q_u8(in_out.as_mut_ptr().add(i + 16), veorq_u8(ct2, k2));
        vst1q_u8(in_out.as_mut_ptr().add(i + 32), veorq_u8(ct3, k3));
        vst1q_u8(in_out.as_mut_ptr().add(i + 48), veorq_u8(ct4, k4));
        vst1q_u8(in_out.as_mut_ptr().add(i + 64), veorq_u8(ct5, k5));
        vst1q_u8(in_out.as_mut_ptr().add(i + 80), veorq_u8(ct6, k6));
        vst1q_u8(in_out.as_mut_ptr().add(i + 96), veorq_u8(ct7, k7));
        vst1q_u8(in_out.as_mut_ptr().add(i + 112), veorq_u8(ct8, k8));

        base = vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), vsetq_lane_u32(8, vdupq_n_u32(0), 0)));
        i += 128;
    }

    // 4-block fused pipeline
    while i + 64 <= n {
        let c1 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), zero)), swap);
        let c2 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), one)), swap);
        let c3 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), two)), swap);
        let c4 = vqtbl1q_u8(vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), three)), swap);

        let (k1, k2, k3, k4) = aes_encrypt_4blocks::<N>(round_keys, c1, c2, c3, c4);

        let ct1 = vld1q_u8(in_out.as_ptr().add(i));
        let ct2 = vld1q_u8(in_out.as_ptr().add(i + 16));
        let ct3 = vld1q_u8(in_out.as_ptr().add(i + 32));
        let ct4 = vld1q_u8(in_out.as_ptr().add(i + 48));

        state = ghash_4blocks(state, ct1, ct2, ct3, ct4, h_powers);

        vst1q_u8(in_out.as_mut_ptr().add(i), veorq_u8(ct1, k1));
        vst1q_u8(in_out.as_mut_ptr().add(i + 16), veorq_u8(ct2, k2));
        vst1q_u8(in_out.as_mut_ptr().add(i + 32), veorq_u8(ct3, k3));
        vst1q_u8(in_out.as_mut_ptr().add(i + 48), veorq_u8(ct4, k4));

        base = vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), four));
        i += 64;
    }

    // Tail blocks (1-3)
    while i + 16 <= n {
        let k = aes_encrypt_block::<N>(round_keys, vqtbl1q_u8(base, swap));
        let ct = vld1q_u8(in_out.as_ptr().add(i));

        let block = vrbitq_u8(ct);
        state = clmul_gcm_pmull(veorq_u8(state, block), h_powers[0]);

        vst1q_u8(in_out.as_mut_ptr().add(i), veorq_u8(ct, k));

        base = vreinterpretq_u8_u32(vaddq_u32(vreinterpretq_u32_u8(base), one));
        i += 16;
    }
    if i < n {
        let k = aes_encrypt_block::<N>(round_keys, vqtbl1q_u8(base, swap));
        let mut buf = [0u8; 16];
        buf[..n - i].copy_from_slice(&in_out[i..]);
        let ct = vld1q_u8(buf.as_ptr());

        let mut padded = [0u8; 16];
        padded[..n - i].copy_from_slice(&in_out[i..]);
        let block = vrbitq_u8(vld1q_u8(padded.as_ptr()));
        state = clmul_gcm_pmull(veorq_u8(state, block), h_powers[0]);

        let pt = veorq_u8(ct, k);
        let mut out_buf = [0u8; 16];
        vst1q_u8(out_buf.as_mut_ptr(), pt);
        in_out[i..].copy_from_slice(&out_buf[..n - i]);
    }

    // Length block
    let mut len_block = [0u8; 16];
    len_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    len_block[8..].copy_from_slice(&((in_out.len() as u64) * 8).to_be_bytes());
    let len_br = vrbitq_u8(vld1q_u8(len_block.as_ptr()));
    state = clmul_gcm_pmull(veorq_u8(state, len_br), h_powers[0]);

    // Tag
    let computed_tag = veorq_u8(vrbitq_u8(state), vld1q_u8(ej0_bytes.as_ptr()));
    let mut computed = [0u8; 16];
    vst1q_u8(computed.as_mut_ptr(), computed_tag);

    if !constant_time_eq(&computed, tag) {
        return Err(AeadError::InvalidCiphertext);
    }

    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use hex;

    use super::*;
    use crate::{Aead, aes::aes::expand_key};

    fn make_round_keys(key: &[u8; 32]) -> [uint8x16_t; 15] {
        let soft = crate::aes::aes::expand_key::<15>(key);
        let mut rk = [unsafe { vdupq_n_u8(0) }; 15];
        for i in 0..15 {
            rk[i] = unsafe { vld1q_u8(soft[i].as_ptr()) };
        }
        rk
    }

    fn make_h_powers(key: &[u8; 32]) -> [uint8x16_t; 8] {
        let (bytes, _) = crate::aes::ghash::precompute_ghash_powers_8::<15>(key);
        let mut hp = [unsafe { vdupq_n_u8(0) }; 8];
        for i in 0..8 {
            hp[i] = unsafe { vld1q_u8(bytes[i].as_ptr()) };
        }
        hp
    }

    #[test]
    fn arm_matches_soft_gcm() {
        let key: [u8; 32] =
            hex::decode_array::<32>(b"feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308").unwrap();
        let nonce: [u8; 12] = hex::decode_array::<12>(b"cafebabefacedbaddecaf888").unwrap();
        let aad = hex::decode("feedfacedeadbeeffeedfacedeadbeef").unwrap();
        let pt: Vec<u8> = (0u8..=255u8).collect();
        let round_keys = expand_key(&key);
        let h = crate::aes::aes::encrypt_block(&round_keys, &[0u8; 16]);

        let cipher = crate::aes::Aes256Gcm::new(&key);
        let mut soft_buf = pt.clone();
        let soft_tag = cipher
            .0
            .encrypt_in_place_soft(&mut soft_buf, &round_keys, &h, &nonce, &aad);

        let rk = make_round_keys(&key);
        let hp = make_h_powers(&key);
        let mut arm_buf = pt.clone();
        let arm_tag = unsafe { gcm_encrypt_armv8(&rk, &hp, &mut arm_buf, &nonce, &aad) };

        assert_eq!(arm_buf, soft_buf);
        assert_eq!(arm_tag.as_ref(), soft_tag.as_ref());
    }

    fn make_round_keys_128(key: &[u8; 16]) -> [uint8x16_t; 11] {
        let soft = crate::aes::aes::expand_key::<11>(key);
        let mut rk = [unsafe { vdupq_n_u8(0) }; 11];
        for (slot, bytes) in rk.iter_mut().zip(soft.iter()) {
            *slot = unsafe { vld1q_u8(bytes.as_ptr()) };
        }
        rk
    }

    fn make_h_powers_128(key: &[u8; 16]) -> [uint8x16_t; 8] {
        let (bytes, _) = crate::aes::ghash::precompute_ghash_powers_8::<11>(key);
        let mut hp = [unsafe { vdupq_n_u8(0) }; 8];
        for (slot, power) in hp.iter_mut().zip(bytes.iter()) {
            *slot = unsafe { vld1q_u8(power.as_ptr()) };
        }
        hp
    }

    fn have_armv8_crypto() -> bool {
        std::arch::is_aarch64_feature_detected!("aes") && std::arch::is_aarch64_feature_detected!("neon")
    }

    fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
        (0..len)
            .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
            .collect()
    }

    /// The `aes,pmull` GCM kernel must agree with the constant-time software
    /// path for AES-128 and AES-256 at every length across the 1/4/8-block loop
    /// boundaries and for partial final blocks. The kernel is called directly so
    /// that both `N = 11` and `N = 15` instantiations are exercised.
    #[test]
    fn arm_kernel_matches_soft_all_lengths() {
        if !have_armv8_crypto() {
            eprintln!("skipping arm_kernel_matches_soft_all_lengths: ARMv8 crypto extensions not available");
            return;
        }

        let nonce = [0x5au8; 12];
        let aad_lengths = [0usize, 1, 15, 16, 17, 32, 33];

        let key128 = [0x3cu8; 16];
        let cipher128 = crate::aes::Aes128Gcm::new(&key128);
        let rk128 = make_round_keys_128(&key128);
        let hp128 = make_h_powers_128(&key128);
        let soft_rk128 = crate::aes::aes::expand_key::<11>(&key128);
        let h128 = crate::aes::aes::encrypt_block(&soft_rk128, &[0u8; 16]);

        let key256 = [0x7eu8; 32];
        let cipher256 = crate::aes::Aes256Gcm::new(&key256);
        let rk256 = make_round_keys(&key256);
        let hp256 = make_h_powers(&key256);
        let soft_rk256 = crate::aes::aes::expand_key::<15>(&key256);
        let h256 = crate::aes::aes::encrypt_block(&soft_rk256, &[0u8; 16]);

        for pt_len in 0..=130usize {
            let pt = pattern(pt_len, 37, 11);
            for &aad_len in &aad_lengths {
                let aad = pattern(aad_len, 53, 7);
                let label = format!("pt_len={pt_len} aad_len={aad_len}");

                // AES-128.
                let mut want = pt.clone();
                let want_tag = cipher128
                    .0
                    .encrypt_in_place_soft(&mut want, &soft_rk128, &h128, &nonce, &aad);
                let mut got = pt.clone();
                let got_tag = unsafe { gcm_encrypt_armv8(&rk128, &hp128, &mut got, &nonce, &aad) };
                assert_eq!(got, want, "AES-128 ciphertext mismatch ({label})");
                assert_eq!(got_tag.as_ref(), want_tag.as_ref(), "AES-128 tag mismatch ({label})");

                let mut back = got.clone();
                unsafe {
                    gcm_decrypt_armv8(&rk128, &hp128, &mut back, want_tag.as_ref().try_into().unwrap(), &nonce, &aad)
                }
                .unwrap_or_else(|e| panic!("AES-128 kernel decrypt failed ({label}): {e}"));
                assert_eq!(back, pt, "AES-128 roundtrip mismatch ({label})");

                // AES-256.
                let mut want = pt.clone();
                let want_tag = cipher256
                    .0
                    .encrypt_in_place_soft(&mut want, &soft_rk256, &h256, &nonce, &aad);
                let mut got = pt.clone();
                let got_tag = unsafe { gcm_encrypt_armv8(&rk256, &hp256, &mut got, &nonce, &aad) };
                assert_eq!(got, want, "AES-256 ciphertext mismatch ({label})");
                assert_eq!(got_tag.as_ref(), want_tag.as_ref(), "AES-256 tag mismatch ({label})");

                let mut back = got.clone();
                unsafe {
                    gcm_decrypt_armv8(&rk256, &hp256, &mut back, want_tag.as_ref().try_into().unwrap(), &nonce, &aad)
                }
                .unwrap_or_else(|e| panic!("AES-256 kernel decrypt failed ({label}): {e}"));
                assert_eq!(back, pt, "AES-256 roundtrip mismatch ({label})");
            }
        }
    }

    /// Non-zero-offset slices and canaries: the kernel must not read or write
    /// outside `in_out`, including for partial final blocks.
    #[test]
    fn arm_kernel_unaligned_slices_preserve_canaries() {
        if !have_armv8_crypto() {
            eprintln!("skipping arm_kernel_unaligned_slices_preserve_canaries: ARMv8 crypto extensions not available");
            return;
        }

        let key = [0x24u8; 32];
        let nonce = [0x68u8; 12];
        let aad = pattern(21, 9, 4);
        let rk = make_round_keys(&key);
        let hp = make_h_powers(&key);

        for len in [0usize, 1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 255, 256, 257] {
            let plaintext = pattern(len, 43, 9);
            for offset in [0usize, 1, 4, 8, 15] {
                let mut storage = vec![0xa5u8; len + offset + 24];
                storage[offset..offset + len].copy_from_slice(&plaintext);

                let tag = unsafe { gcm_encrypt_armv8(&rk, &hp, &mut storage[offset..offset + len], &nonce, &aad) };

                assert!(
                    storage[..offset].iter().all(|&b| b == 0xa5),
                    "prefix canary len={len} off={offset}"
                );
                assert!(
                    storage[offset + len..].iter().all(|&b| b == 0xa5),
                    "suffix canary len={len} off={offset}"
                );

                let mut back = storage[offset..offset + len].to_vec();
                unsafe { gcm_decrypt_armv8(&rk, &hp, &mut back, tag.as_ref().try_into().unwrap(), &nonce, &aad) }
                    .expect("kernel decrypt");
                assert_eq!(back, plaintext);
            }
        }
    }

    /// A bad tag must return `InvalidCiphertext` for every bit position and
    /// every loop shape, and must never return `Ok`.
    #[test]
    fn arm_kernel_rejects_bad_tags() {
        if !have_armv8_crypto() {
            eprintln!("skipping arm_kernel_rejects_bad_tags: ARMv8 crypto extensions not available");
            return;
        }

        let key = [0x31u8; 32];
        let nonce = [0x72u8; 12];
        let aad = pattern(19, 5, 6);
        let rk = make_round_keys(&key);
        let hp = make_h_powers(&key);
        let cipher = crate::aes::Aes256Gcm::new(&key);

        for len in [0usize, 1, 15, 16, 17, 64, 128, 129, 256] {
            let pt = pattern(len, 27, 3);
            let mut buf = pt.clone();
            let tag: [u8; 16] = unsafe { gcm_encrypt_armv8(&rk, &hp, &mut buf, &nonce, &aad) }
                .as_ref()
                .try_into()
                .unwrap();
            for bit in 0..128 {
                let mut bad = tag;
                bad[bit / 8] ^= 1 << (bit % 8);
                let mut tampered = buf.clone();
                assert_eq!(
                    unsafe { gcm_decrypt_armv8(&rk, &hp, &mut tampered, &bad, &nonce, &aad) }.unwrap_err(),
                    crate::AeadError::InvalidCiphertext,
                    "accepted forged tag len={len} bit={bit}"
                );
            }
            let mut good = buf.clone();
            unsafe { gcm_decrypt_armv8(&rk, &hp, &mut good, &tag, &nonce, &aad) }.expect("genuine tag rejected");
            assert_eq!(good, pt);
            // The public wrapper agrees.
            let mut via_dispatch = buf.clone();
            cipher
                .decrypt_in_place(&mut via_dispatch, &nonce, &aad, &tag)
                .expect("dispatch decrypt");
            assert_eq!(via_dispatch, pt);
        }
    }
}
