// SAFETY: all AVX-512 intrinsics are only called from functions that have
// #[target_feature(enable = "avx512f")], so the CPU supports the instructions.

use core::arch::x86_64::*;

use crate::xxh3::{Acc, PRIME32_1};

#[allow(clippy::similar_names)]
#[target_feature(enable = "avx512f")]
pub unsafe fn accumulate_512(acc: &mut Acc, input: &[u8], input_off: usize, secret: &[u8], secret_off: usize) {
    unsafe {
        let acc_ptr = acc.0.as_mut_ptr() as *mut __m512i;
        let input_ptr = input.as_ptr().add(input_off) as *const __m512i;
        let secret_ptr = secret.as_ptr().add(secret_off) as *const __m512i;

        let acc_vec = _mm512_load_si512(acc_ptr);
        let data_vec = _mm512_loadu_si512(input_ptr);
        let key_vec = _mm512_loadu_si512(secret_ptr);
        let data_key = _mm512_xor_si512(data_vec, key_vec);
        let data_key_lo = _mm512_srli_epi64::<32>(data_key);
        let product = _mm512_mul_epu32(data_key, data_key_lo);
        let data_swap = _mm512_shuffle_epi32::<78>(data_vec);
        let sum = _mm512_add_epi64(acc_vec, data_swap);
        let result = _mm512_add_epi64(product, sum);
        _mm512_store_si512(acc_ptr, result);
    }
}

#[allow(clippy::similar_names)]
#[target_feature(enable = "avx512f")]
pub unsafe fn scramble_acc(acc: &mut Acc, secret: &[u8], secret_off: usize) {
    unsafe {
        let prime32 = _mm512_set1_epi32(PRIME32_1 as i32);
        let acc_ptr = acc.0.as_mut_ptr() as *mut __m512i;
        let secret_ptr = secret.as_ptr().add(secret_off) as *const __m512i;

        let acc_vec = _mm512_load_si512(acc_ptr);
        let shifted = _mm512_srli_epi64::<47>(acc_vec);
        let key_vec = _mm512_loadu_si512(secret_ptr);
        let data_key = _mm512_xor_si512(key_vec, _mm512_xor_si512(acc_vec, shifted));

        let data_key_hi = _mm512_srli_epi64::<32>(data_key);
        let prod_lo = _mm512_mul_epu32(data_key, prime32);
        let prod_hi = _mm512_mul_epu32(data_key_hi, prime32);
        let result = _mm512_add_epi64(prod_lo, _mm512_slli_epi64::<32>(prod_hi));
        _mm512_store_si512(acc_ptr, result);
    }
}
