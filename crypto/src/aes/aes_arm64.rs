#![allow(unsafe_op_in_unsafe_fn)]

/// aarch64 AES-256-GCM using ARMv8 Crypto extensions.
///
/// Same 8/4-block parallel CTR and aggregated GHASH strategy as the
/// x86_64 path (see `aes256_amd64.rs` for details). The ARMv8 equivalents:
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

/// Const generic AES encrypt block for AES-128 (N=11) and AES-256 (N=15).
#[target_feature(enable = "aes,neon")]
#[inline]
pub(crate) unsafe fn aes_encrypt_block<const N: usize>(rk: &[uint8x16_t; N], block: uint8x16_t) -> uint8x16_t {
    const { assert!(N == 11 || N == 15) };

    let zero = vdupq_n_u8(0);

    let mut b = veorq_u8(block, rk[0]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[1]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[2]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[3]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[4]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[5]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[6]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[7]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[8]);
    b = vaesmcq_u8(vaeseq_u8(b, zero));
    b = veorq_u8(b, rk[9]);

    if N == 15 {
        let p = rk.as_ptr();
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
    } else {
        b = vaeseq_u8(b, zero);
        veorq_u8(b, rk[10])
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
