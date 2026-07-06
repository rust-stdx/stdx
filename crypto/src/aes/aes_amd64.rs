#![allow(unsafe_op_in_unsafe_fn)]

/// x86-64 AES-256 block cipher using AES-NI intrinsics.
use core::arch::x86_64::*;

#[inline]
pub(crate) fn expand_key_x86_64<const N: usize>(round_keys_software: [[u8; 16]; N]) -> [__m128i; N] {
    const { assert!(N == 11 || N == 15) };

    let mut round_keys = unsafe { [_mm_setzero_si128(); N] };
    for i in 0..N {
        round_keys[i] = unsafe { _mm_loadu_si128(round_keys_software[i].as_ptr().cast()) };
    }
    round_keys
}

/// Const generic AES encrypt block for AES-128 (N=11) and AES-256 (N=15).
#[target_feature(enable = "aes,sse2")]
#[inline]
pub(crate) unsafe fn aes_encrypt_block<const N: usize>(rk: &[__m128i; N], block: __m128i) -> __m128i {
    const { assert!(N == 11 || N == 15) };

    let mut b = _mm_xor_si128(block, rk[0]);
    b = _mm_aesenc_si128(b, rk[1]);
    b = _mm_aesenc_si128(b, rk[2]);
    b = _mm_aesenc_si128(b, rk[3]);
    b = _mm_aesenc_si128(b, rk[4]);
    b = _mm_aesenc_si128(b, rk[5]);
    b = _mm_aesenc_si128(b, rk[6]);
    b = _mm_aesenc_si128(b, rk[7]);
    b = _mm_aesenc_si128(b, rk[8]);
    b = _mm_aesenc_si128(b, rk[9]);

    if N == 11 {
        _mm_aesenclast_si128(b, rk[10])
    } else {
        let p = rk.as_ptr();
        b = _mm_aesenc_si128(b, *p.add(10));
        b = _mm_aesenc_si128(b, *p.add(11));
        b = _mm_aesenc_si128(b, *p.add(12));
        b = _mm_aesenc_si128(b, *p.add(13));
        _mm_aesenclast_si128(b, *p.add(14))
    }
}

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

    #[test]
    fn aesni_ecb_vectors() {
        skip_unless_aesni!();

        let key: [u8; 32] =
            hex::decode_array::<32>(b"603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4").unwrap();
        let rk = make_rk(&key);

        let vectors: &[([u8; 16], [u8; 16])] = &[
            (
                hex::decode_array::<16>(b"6bc1bee22e409f96e93d7e117393172a").unwrap(),
                hex::decode_array::<16>(b"f3eed1bdb5d2a03c064b5a7e3db181f8").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"ae2d8a571e03ac9c9eb76fac45af8e51").unwrap(),
                hex::decode_array::<16>(b"591ccb10d410ed26dc5ba74a31362870").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"30c81c46a35ce411e5fbc1191a0a52ef").unwrap(),
                hex::decode_array::<16>(b"b6ed21b99ca6f4f9f153e7b1beafed1d").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"f69f2445df4f9b17ad2b417be66c3710").unwrap(),
                hex::decode_array::<16>(b"23304b7a39f9f3ff067d8d8f9e24ecc7").unwrap(),
            ),
        ];

        for (pt, ct_exp) in vectors {
            let pt_xmm = unsafe { _mm_loadu_si128(pt.as_ptr().cast()) };
            let ct_xmm = unsafe { aes_encrypt_block(&rk, pt_xmm) };
            let mut ni_ct = [0u8; 16];
            unsafe { _mm_storeu_si128(ni_ct.as_mut_ptr().cast(), ct_xmm) };
            assert_eq!(ni_ct, *ct_exp);
        }
    }
}
