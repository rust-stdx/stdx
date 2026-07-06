#![allow(unsafe_op_in_unsafe_fn)]

/// x86-64 AES-256-GCM (GCM path) using AES-NI and PCLMULQDQ intrinsics.
///
/// ## Representations
///
/// GCM uses a big-endian polynomial ring: bit 0 of byte 0 = coefficient of x^127.
/// PCLMULQDQ uses little-endian polynomial order: bit 0 of SSE lane = coefficient of x^0.
///
/// To bridge this, every GCM block is transformed by **reversing bits in each byte**
/// with `_mm_shuffle_epi8` nibble lookups before carry-less multiplication.
/// In that transformed domain, PCLMULQDQ directly computes GHASH multiplication with
/// reduction polynomial P(x) = x^128 + x^7 + x^2 + x + 1.
///
/// ## Precomputed keys
///
/// The caller (via `gcm_encrypt_aesni` / `gcm_decrypt_aesni`) supplies the precomputed
/// AES round keys (`rk: &[__m128i; 15]`) and precomputed GHASH powers
/// (`h_powers: &[__m128i; 4]` where index 0 = H¹, 1 = H², 2 = H³, 3 = H⁴,
/// all in bit-reversed-per-byte form). This eliminates key expansion and
/// GHASH-subkey derivation from every encrypt/decrypt call.
///
/// ## 4-block parallel CTR
///
/// The counter is maintained in a byte-swapped (little-endian counter domain)
/// to allow cheap `_mm_add_epi32` increments without per-block byte-reversal
/// round-trips. Four successive counters are unswapped via `pshufb`, encrypted
/// in an interleaved 4-way AES pipeline, then XOR'd with the plaintext.
///
/// ## 4-block aggregated GHASH
///
/// For four successive ciphertext blocks B₁…B₄, the aggregate update is:
///   state' = state·H⁴ ⊕ B₁·H⁴ ⊕ B₂·H³ ⊕ B₃·H² ⊕ B₄·H
/// where · is GF(2¹²⁸) multiplication and Hⁱ are the precomputed powers.
/// This reduces the reduction critical path compared to four serial
/// multiply-reduce chains.
#[allow(clippy::many_single_char_names)]
use core::arch::x86_64::*;

use super::{
    aes::GCM_MAX_LEN,
    aes_amd64::aes_encrypt_block,
    aes_ctr_amd64::{SWAP_BYTES, increment_counter},
    ghash_amd64::{bitreverse_per_byte, clmul_gcm, ghash_4blocks, ghash_8blocks, ghash_update},
};
use crate::{AeadError, Hash, bytes::Bytes};

// ── AES-256-GCM encrypt (hardware) ────────────────────────────────────────────

/// Encrypt using AES-NI + PCLMULQDQ with precomputed keys and GHASH powers.
///
/// `N` is the number of round keys: 11 for AES-128, 15 for AES-256.
///
/// CTR and GHASH are **fused** into a single pass over the data to minimise
/// memory bandwidth. Eight blocks are processed per iteration for better
/// pipelining of the `aesenc` instruction chain.
#[target_feature(enable = "aes,pclmulqdq,ssse3,sse4.1,sse2")]
pub(crate) unsafe fn gcm_encrypt_aesni<const N: usize>(
    rk: &[__m128i; N],
    h_powers: &[__m128i; 8],
    in_out: &mut [u8],
    nonce: &[u8; 12],
    aad: &[u8],
) -> Hash {
    const {
        assert!(N == 11 || N == 15);
    }

    assert!(
        in_out.len() <= GCM_MAX_LEN,
        "GCM plaintext exceeds maximum allowed length (2^32 - 2 blocks)"
    );

    // J0 = nonce ∥ 0x00000001
    let mut j0_bytes = [0u8; 16];
    j0_bytes[..12].copy_from_slice(nonce);
    j0_bytes[15] = 0x01;
    let j0 = _mm_loadu_si128(j0_bytes.as_ptr().cast());

    // E(J0) in natural byte order (used to mask the tag)
    let ej0 = aes_encrypt_block::<N>(rk, j0);
    let mut ej0_bytes = [0u8; 16];
    _mm_storeu_si128(ej0_bytes.as_mut_ptr().cast(), ej0);

    // CTR starts at J0 + 1 = nonce ∥ 0x00000002
    let ctr = increment_counter(j0);

    // ── Single-pass fused CTR + GHASH ──────────────────────────────────────
    let swap = _mm_loadu_si128(SWAP_BYTES.as_ptr().cast());
    let mut base = _mm_shuffle_epi8(ctr, swap);
    let zero = _mm_setzero_si128();
    let one = _mm_set_epi32(1, 0, 0, 0);
    let two = _mm_set_epi32(2, 0, 0, 0);
    let three = _mm_set_epi32(3, 0, 0, 0);
    let four = _mm_set_epi32(4, 0, 0, 0);
    let five = _mm_set_epi32(5, 0, 0, 0);
    let six = _mm_set_epi32(6, 0, 0, 0);
    let seven = _mm_set_epi32(7, 0, 0, 0);

    let mut state = _mm_setzero_si128();
    // AAD (single-block GHASH, usually short)
    state = ghash_update(state, h_powers[0], aad);

    let n = in_out.len();
    let mut i = 0;

    // 8-block fused pipeline
    while i + 128 <= n {
        let c1 = _mm_shuffle_epi8(_mm_add_epi32(base, zero), swap);
        let c2 = _mm_shuffle_epi8(_mm_add_epi32(base, one), swap);
        let c3 = _mm_shuffle_epi8(_mm_add_epi32(base, two), swap);
        let c4 = _mm_shuffle_epi8(_mm_add_epi32(base, three), swap);
        let c5 = _mm_shuffle_epi8(_mm_add_epi32(base, four), swap);
        let c6 = _mm_shuffle_epi8(_mm_add_epi32(base, five), swap);
        let c7 = _mm_shuffle_epi8(_mm_add_epi32(base, six), swap);
        let c8 = _mm_shuffle_epi8(_mm_add_epi32(base, seven), swap);

        let k1 = aes_encrypt_block::<N>(rk, c1);
        let k2 = aes_encrypt_block::<N>(rk, c2);
        let k3 = aes_encrypt_block::<N>(rk, c3);
        let k4 = aes_encrypt_block::<N>(rk, c4);
        let k5 = aes_encrypt_block::<N>(rk, c5);
        let k6 = aes_encrypt_block::<N>(rk, c6);
        let k7 = aes_encrypt_block::<N>(rk, c7);
        let k8 = aes_encrypt_block::<N>(rk, c8);

        let p1 = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
        let p2 = _mm_loadu_si128(in_out.as_ptr().add(i + 16).cast());
        let p3 = _mm_loadu_si128(in_out.as_ptr().add(i + 32).cast());
        let p4 = _mm_loadu_si128(in_out.as_ptr().add(i + 48).cast());
        let p5 = _mm_loadu_si128(in_out.as_ptr().add(i + 64).cast());
        let p6 = _mm_loadu_si128(in_out.as_ptr().add(i + 80).cast());
        let p7 = _mm_loadu_si128(in_out.as_ptr().add(i + 96).cast());
        let p8 = _mm_loadu_si128(in_out.as_ptr().add(i + 112).cast());

        let ct1 = _mm_xor_si128(p1, k1);
        let ct2 = _mm_xor_si128(p2, k2);
        let ct3 = _mm_xor_si128(p3, k3);
        let ct4 = _mm_xor_si128(p4, k4);
        let ct5 = _mm_xor_si128(p5, k5);
        let ct6 = _mm_xor_si128(p6, k6);
        let ct7 = _mm_xor_si128(p7, k7);
        let ct8 = _mm_xor_si128(p8, k8);

        _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), ct1);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 16).cast(), ct2);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 32).cast(), ct3);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 48).cast(), ct4);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 64).cast(), ct5);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 80).cast(), ct6);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 96).cast(), ct7);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 112).cast(), ct8);

        state = ghash_8blocks(state, ct1, ct2, ct3, ct4, ct5, ct6, ct7, ct8, h_powers);

        base = _mm_add_epi32(base, _mm_set_epi32(8, 0, 0, 0));
        i += 128;
    }

    // 4-block fused pipeline (tail, when 8 blocks won't fit but 4 will)
    while i + 64 <= n {
        let c1 = _mm_shuffle_epi8(_mm_add_epi32(base, zero), swap);
        let c2 = _mm_shuffle_epi8(_mm_add_epi32(base, one), swap);
        let c3 = _mm_shuffle_epi8(_mm_add_epi32(base, two), swap);
        let c4 = _mm_shuffle_epi8(_mm_add_epi32(base, three), swap);

        let k1 = aes_encrypt_block::<N>(rk, c1);
        let k2 = aes_encrypt_block::<N>(rk, c2);
        let k3 = aes_encrypt_block::<N>(rk, c3);
        let k4 = aes_encrypt_block::<N>(rk, c4);

        let p1 = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
        let p2 = _mm_loadu_si128(in_out.as_ptr().add(i + 16).cast());
        let p3 = _mm_loadu_si128(in_out.as_ptr().add(i + 32).cast());
        let p4 = _mm_loadu_si128(in_out.as_ptr().add(i + 48).cast());

        let ct1 = _mm_xor_si128(p1, k1);
        let ct2 = _mm_xor_si128(p2, k2);
        let ct3 = _mm_xor_si128(p3, k3);
        let ct4 = _mm_xor_si128(p4, k4);

        _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), ct1);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 16).cast(), ct2);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 32).cast(), ct3);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 48).cast(), ct4);

        state = ghash_4blocks(state, ct1, ct2, ct3, ct4, h_powers);

        base = _mm_add_epi32(base, four);
        i += 64;
    }

    // Tail blocks (1-3)
    while i + 16 <= n {
        let k = aes_encrypt_block::<N>(rk, _mm_shuffle_epi8(base, swap));
        let ct = _mm_xor_si128(_mm_loadu_si128(in_out.as_ptr().add(i).cast()), k);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), ct);

        let block = bitreverse_per_byte(ct);
        state = clmul_gcm(_mm_xor_si128(state, block), h_powers[0]);

        base = _mm_add_epi32(base, one);
        i += 16;
    }
    if i < n {
        let k = aes_encrypt_block::<N>(rk, _mm_shuffle_epi8(base, swap));
        let mut buf = [0u8; 16];
        buf[..n - i].copy_from_slice(&in_out[i..]);
        let ct = _mm_xor_si128(_mm_loadu_si128(buf.as_ptr().cast()), k);
        let mut out_buf = [0u8; 16];
        _mm_storeu_si128(out_buf.as_mut_ptr().cast(), ct);
        in_out[i..].copy_from_slice(&out_buf[..n - i]);

        let mut padded = [0u8; 16];
        padded[..n - i].copy_from_slice(&in_out[i..]);
        let block = bitreverse_per_byte(_mm_loadu_si128(padded.as_ptr().cast()));
        state = clmul_gcm(_mm_xor_si128(state, block), h_powers[0]);
    }

    // ── Length block ───────────────────────────────────────────────────────
    let mut len_block = [0u8; 16];
    len_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    len_block[8..].copy_from_slice(&((in_out.len() as u64) * 8).to_be_bytes());
    let len_br = bitreverse_per_byte(_mm_loadu_si128(len_block.as_ptr().cast()));
    state = clmul_gcm(_mm_xor_si128(state, len_br), h_powers[0]);

    // ── Tag ────────────────────────────────────────────────────────────────
    let mut tag = Bytes::<64>::with_length(16);
    let tag_xmm = _mm_xor_si128(bitreverse_per_byte(state), _mm_loadu_si128(ej0_bytes.as_ptr().cast()));
    _mm_storeu_si128(tag.as_mut().as_mut_ptr().cast(), tag_xmm);
    Hash(tag)
}

// ── AES-256-GCM decrypt (hardware) ───────────────────────────────────────────

/// Decrypt using AES-NI + PCLMULQDQ with precomputed keys and GHASH powers.
///
/// Authenticate-then-decrypt: GHASH the ciphertext first, verify the tag,
/// then CTR-decrypt. 8-block aggregates are used for large payloads,
/// falling back to 4-block when less than 8 blocks remain.
#[target_feature(enable = "aes,pclmulqdq,ssse3,sse4.1,sse2")]
pub(crate) unsafe fn gcm_decrypt_aesni<const N: usize>(
    rk: &[__m128i; N],
    h_powers: &[__m128i; 8],
    in_out: &mut [u8],
    tag: &[u8; 16],
    nonce: &[u8; 12],
    aad: &[u8],
) -> Result<(), AeadError> {
    const {
        assert!(N == 11 || N == 15);
    }

    if in_out.len() > GCM_MAX_LEN {
        return Err(AeadError::InvalidCiphertext);
    }

    // J0 = nonce ∥ 0x00000001
    let mut j0_bytes = [0u8; 16];
    j0_bytes[..12].copy_from_slice(nonce);
    j0_bytes[15] = 0x01;
    let j0 = _mm_loadu_si128(j0_bytes.as_ptr().cast());

    let ej0 = aes_encrypt_block::<N>(rk, j0);
    let mut ej0_bytes = [0u8; 16];
    _mm_storeu_si128(ej0_bytes.as_mut_ptr().cast(), ej0);

    // ── GHASH the ciphertext ──────────────────────────────────────────────
    let mut state = _mm_setzero_si128();
    state = ghash_update(state, h_powers[0], aad);

    let n = in_out.len();
    let mut i = 0;

    while i + 128 <= n {
        let b1 = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
        let b2 = _mm_loadu_si128(in_out.as_ptr().add(i + 16).cast());
        let b3 = _mm_loadu_si128(in_out.as_ptr().add(i + 32).cast());
        let b4 = _mm_loadu_si128(in_out.as_ptr().add(i + 48).cast());
        let b5 = _mm_loadu_si128(in_out.as_ptr().add(i + 64).cast());
        let b6 = _mm_loadu_si128(in_out.as_ptr().add(i + 80).cast());
        let b7 = _mm_loadu_si128(in_out.as_ptr().add(i + 96).cast());
        let b8 = _mm_loadu_si128(in_out.as_ptr().add(i + 112).cast());
        state = ghash_8blocks(state, b1, b2, b3, b4, b5, b6, b7, b8, h_powers);
        i += 128;
    }

    while i + 64 <= n {
        let b1 = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
        let b2 = _mm_loadu_si128(in_out.as_ptr().add(i + 16).cast());
        let b3 = _mm_loadu_si128(in_out.as_ptr().add(i + 32).cast());
        let b4 = _mm_loadu_si128(in_out.as_ptr().add(i + 48).cast());
        state = ghash_4blocks(state, b1, b2, b3, b4, h_powers);
        i += 64;
    }

    while i + 16 <= n {
        let block = bitreverse_per_byte(_mm_loadu_si128(in_out.as_ptr().add(i).cast()));
        state = clmul_gcm(_mm_xor_si128(state, block), h_powers[0]);
        i += 16;
    }
    if i < n {
        let mut padded = [0u8; 16];
        padded[..n - i].copy_from_slice(&in_out[i..]);
        let block = bitreverse_per_byte(_mm_loadu_si128(padded.as_ptr().cast()));
        state = clmul_gcm(_mm_xor_si128(state, block), h_powers[0]);
    }

    let mut len_block = [0u8; 16];
    len_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    len_block[8..].copy_from_slice(&((in_out.len() as u64) * 8).to_be_bytes());
    let len_br = bitreverse_per_byte(_mm_loadu_si128(len_block.as_ptr().cast()));
    state = clmul_gcm(_mm_xor_si128(state, len_br), h_powers[0]);

    let computed_tag = _mm_xor_si128(bitreverse_per_byte(state), _mm_loadu_si128(ej0_bytes.as_ptr().cast()));
    let mut computed = [0u8; 16];
    _mm_storeu_si128(computed.as_mut_ptr().cast(), computed_tag);

    // Constant-time comparison
    let mut diff = 0u8;
    for k in 0..16 {
        diff |= computed[k] ^ tag[k];
    }
    if diff != 0 {
        return Err(AeadError::InvalidCiphertext);
    }

    // ── CTR decrypt ────────────────────────────────────────────────────────
    let swap = _mm_loadu_si128(SWAP_BYTES.as_ptr().cast());
    let ctr = increment_counter(j0);
    let mut base = _mm_shuffle_epi8(ctr, swap);
    let zero_offset = _mm_setzero_si128();
    let one = _mm_set_epi32(1, 0, 0, 0);
    let two = _mm_set_epi32(2, 0, 0, 0);
    let three = _mm_set_epi32(3, 0, 0, 0);
    let four = _mm_set_epi32(4, 0, 0, 0);
    let five = _mm_set_epi32(5, 0, 0, 0);
    let six = _mm_set_epi32(6, 0, 0, 0);
    let seven = _mm_set_epi32(7, 0, 0, 0);

    let mut i = 0;
    while i + 128 <= n {
        let c1 = _mm_shuffle_epi8(_mm_add_epi32(base, zero_offset), swap);
        let c2 = _mm_shuffle_epi8(_mm_add_epi32(base, one), swap);
        let c3 = _mm_shuffle_epi8(_mm_add_epi32(base, two), swap);
        let c4 = _mm_shuffle_epi8(_mm_add_epi32(base, three), swap);
        let c5 = _mm_shuffle_epi8(_mm_add_epi32(base, four), swap);
        let c6 = _mm_shuffle_epi8(_mm_add_epi32(base, five), swap);
        let c7 = _mm_shuffle_epi8(_mm_add_epi32(base, six), swap);
        let c8 = _mm_shuffle_epi8(_mm_add_epi32(base, seven), swap);

        let k1 = aes_encrypt_block::<N>(rk, c1);
        let k2 = aes_encrypt_block::<N>(rk, c2);
        let k3 = aes_encrypt_block::<N>(rk, c3);
        let k4 = aes_encrypt_block::<N>(rk, c4);
        let k5 = aes_encrypt_block::<N>(rk, c5);
        let k6 = aes_encrypt_block::<N>(rk, c6);
        let k7 = aes_encrypt_block::<N>(rk, c7);
        let k8 = aes_encrypt_block::<N>(rk, c8);

        let ct1 = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
        let ct2 = _mm_loadu_si128(in_out.as_ptr().add(i + 16).cast());
        let ct3 = _mm_loadu_si128(in_out.as_ptr().add(i + 32).cast());
        let ct4 = _mm_loadu_si128(in_out.as_ptr().add(i + 48).cast());
        let ct5 = _mm_loadu_si128(in_out.as_ptr().add(i + 64).cast());
        let ct6 = _mm_loadu_si128(in_out.as_ptr().add(i + 80).cast());
        let ct7 = _mm_loadu_si128(in_out.as_ptr().add(i + 96).cast());
        let ct8 = _mm_loadu_si128(in_out.as_ptr().add(i + 112).cast());

        _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), _mm_xor_si128(ct1, k1));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 16).cast(), _mm_xor_si128(ct2, k2));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 32).cast(), _mm_xor_si128(ct3, k3));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 48).cast(), _mm_xor_si128(ct4, k4));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 64).cast(), _mm_xor_si128(ct5, k5));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 80).cast(), _mm_xor_si128(ct6, k6));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 96).cast(), _mm_xor_si128(ct7, k7));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 112).cast(), _mm_xor_si128(ct8, k8));

        base = _mm_add_epi32(base, _mm_set_epi32(8, 0, 0, 0));
        i += 128;
    }

    while i + 64 <= n {
        let c1 = _mm_shuffle_epi8(_mm_add_epi32(base, zero_offset), swap);
        let c2 = _mm_shuffle_epi8(_mm_add_epi32(base, one), swap);
        let c3 = _mm_shuffle_epi8(_mm_add_epi32(base, two), swap);
        let c4 = _mm_shuffle_epi8(_mm_add_epi32(base, three), swap);

        let k1 = aes_encrypt_block::<N>(rk, c1);
        let k2 = aes_encrypt_block::<N>(rk, c2);
        let k3 = aes_encrypt_block::<N>(rk, c3);
        let k4 = aes_encrypt_block::<N>(rk, c4);

        let ct1 = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
        let ct2 = _mm_loadu_si128(in_out.as_ptr().add(i + 16).cast());
        let ct3 = _mm_loadu_si128(in_out.as_ptr().add(i + 32).cast());
        let ct4 = _mm_loadu_si128(in_out.as_ptr().add(i + 48).cast());

        _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), _mm_xor_si128(ct1, k1));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 16).cast(), _mm_xor_si128(ct2, k2));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 32).cast(), _mm_xor_si128(ct3, k3));
        _mm_storeu_si128(in_out.as_mut_ptr().add(i + 48).cast(), _mm_xor_si128(ct4, k4));

        base = _mm_add_epi32(base, four);
        i += 64;
    }

    while i + 16 <= n {
        let k = aes_encrypt_block::<N>(rk, _mm_shuffle_epi8(base, swap));
        let pt = _mm_xor_si128(_mm_loadu_si128(in_out.as_ptr().add(i).cast()), k);
        _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), pt);
        base = _mm_add_epi32(base, one);
        i += 16;
    }
    if i < n {
        let k = aes_encrypt_block::<N>(rk, _mm_shuffle_epi8(base, swap));
        let mut buf = [0u8; 16];
        buf[..n - i].copy_from_slice(&in_out[i..]);
        let pt = _mm_xor_si128(_mm_loadu_si128(buf.as_ptr().cast()), k);
        let mut out_buf = [0u8; 16];
        _mm_storeu_si128(out_buf.as_mut_ptr().cast(), pt);
        in_out[i..].copy_from_slice(&out_buf[..n - i]);
    }

    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn have_features() -> bool {
        std::arch::is_x86_feature_detected!("aes")
            && std::arch::is_x86_feature_detected!("pclmulqdq")
            && std::arch::is_x86_feature_detected!("ssse3")
            && std::arch::is_x86_feature_detected!("sse4.1")
    }

    macro_rules! skip_unless_aesni {
        () => {
            if !have_features() {
                eprintln!("Skipping AES-NI test: CPU features not available");
                return;
            }
        };
    }

    fn make_rk(key: &[u8; 32]) -> [__m128i; 15] {
        let soft = crate::aes::aes::key_expand_256(key);
        let mut rk = [unsafe { _mm_setzero_si128() }; 15];
        for i in 0..15 {
            rk[i] = unsafe { _mm_loadu_si128(soft[i].as_ptr().cast()) };
        }
        rk
    }

    fn make_h_powers(key: &[u8; 32]) -> [__m128i; 8] {
        let (bytes, _) = crate::aes::ghash::precompute_ghash_powers::<15>(key);
        let mut hp = [unsafe { _mm_setzero_si128() }; 8];
        for i in 0..8 {
            hp[i] = unsafe { _mm_loadu_si128(bytes[i].as_ptr().cast()) };
        }
        hp
    }

    // ── GHASH / GCM test vectors (NIST SP 800-38D + others) ──────────────────
    #[test]
    fn aesni_tag_mismatch() {
        skip_unless_aesni!();

        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let rk = make_rk(&key);
        let hp = make_h_powers(&key);
        let mut buf = b"hello world".to_vec();
        let tag = unsafe { super::gcm_encrypt_aesni(&rk, &hp, &mut buf, &nonce, &[]) };
        let mut bad: [u8; 16] = tag.as_ref().try_into().unwrap();
        bad[7] ^= 0x01;
        assert!(unsafe { super::gcm_decrypt_aesni(&rk, &hp, &mut buf.clone(), &bad, &nonce, &[]) }.is_err());
    }

    #[test]
    fn aesni_matches_soft() {
        skip_unless_aesni!();

        let key: [u8; 32] =
            hex::decode_array::<32>(b"feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308").unwrap();
        let nonce: [u8; 12] = hex::decode_array::<12>(b"cafebabefacedbaddecaf888").unwrap();
        let aad = hex::decode("feedfacedeadbeeffeedfacedeadbeef").unwrap();
        let pt: Vec<u8> = (0u8..=255u8).collect();

        let cipher = crate::aes::aes_gcm::Aes256Gcm::new(&key);
        let mut soft_buf = pt.clone();
        let soft_tag = cipher.encrypt_in_place_soft(&mut soft_buf, &nonce, &aad);

        let rk = make_rk(&key);
        let hp = make_h_powers(&key);
        let mut ni_buf = pt.clone();
        let ni_tag = unsafe { super::gcm_encrypt_aesni(&rk, &hp, &mut ni_buf, &nonce, &aad) };

        assert_eq!(soft_buf, ni_buf, "ciphertext mismatch soft vs AES-NI");
        assert_eq!(soft_tag.as_ref(), ni_tag.as_ref(), "tag mismatch soft vs AES-NI");
    }

    #[test]
    fn aesni_large_roundtrip() {
        skip_unless_aesni!();

        let key = [0x42u8; 32];
        let nonce = [0x99u8; 12];
        let aad = b"some additional data";
        let pt: Vec<u8> = (0u8..=255u8).cycle().take(4096).collect();

        let rk = make_rk(&key);
        let hp = make_h_powers(&key);

        let mut buf = pt.clone();
        let tag = unsafe { super::gcm_encrypt_aesni(&rk, &hp, &mut buf, &nonce, aad) };
        let tag_bytes: [u8; 16] = tag.as_ref().try_into().unwrap();
        unsafe { super::gcm_decrypt_aesni(&rk, &hp, &mut buf, &tag_bytes, &nonce, aad) }
            .expect("large roundtrip decrypt failed");
        assert_eq!(buf, pt);
    }
}
