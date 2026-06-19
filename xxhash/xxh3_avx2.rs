// SAFETY: all AVX2 intrinsics are only called from functions that have
// #[target_feature(enable = "avx2")], so the CPU supports the instructions.

use core::arch::x86_64::*;

use crate::xxh3::{Acc, PRIME32_1};

#[allow(clippy::similar_names)]
#[target_feature(enable = "avx2")]
pub unsafe fn accumulate_512(acc: &mut Acc, input: &[u8], input_off: usize, secret: &[u8], secret_off: usize) {
    unsafe {
        let acc_ptr = acc.0.as_mut_ptr() as *mut u64;
        let input_ptr = input.as_ptr().add(input_off) as *const u64;
        let secret_ptr = secret.as_ptr().add(secret_off) as *const u64;

        let mut i = 0;
        while i < 2 {
            let acc_vec = _mm256_load_si256(acc_ptr.add(i * 4) as *const __m256i);
            let data_vec = _mm256_loadu_si256(input_ptr.add(i * 4) as *const __m256i);
            let key_vec = _mm256_loadu_si256(secret_ptr.add(i * 4) as *const __m256i);
            let data_key = _mm256_xor_si256(data_vec, key_vec);
            let data_key_lo = _mm256_srli_epi64::<32>(data_key);
            let product = _mm256_mul_epu32(data_key, data_key_lo);
            let data_swap = _mm256_shuffle_epi32::<78>(data_vec);
            let sum = _mm256_add_epi64(acc_vec, data_swap);
            let result = _mm256_add_epi64(product, sum);
            _mm256_store_si256(acc_ptr.add(i * 4) as *mut __m256i, result);
            i += 1;
        }
    }
}

#[allow(clippy::similar_names)]
#[target_feature(enable = "avx2")]
pub unsafe fn scramble_acc(acc: &mut Acc, secret: &[u8], secret_off: usize) {
    unsafe {
        let prime32 = _mm256_set1_epi32(PRIME32_1 as i32);
        let acc_ptr = acc.0.as_mut_ptr() as *mut u64;
        let secret_ptr = secret.as_ptr().add(secret_off) as *const u64;

        let mut i = 0;
        while i < 2 {
            let acc_vec = _mm256_load_si256(acc_ptr.add(i * 4) as *const __m256i);
            let shifted = _mm256_srli_epi64::<47>(acc_vec);
            let data_vec = _mm256_xor_si256(acc_vec, shifted);
            let key_vec = _mm256_loadu_si256(secret_ptr.add(i * 4) as *const __m256i);
            let data_key = _mm256_xor_si256(data_vec, key_vec);
            let data_key_hi = _mm256_srli_epi64::<32>(data_key);
            let prod_lo = _mm256_mul_epu32(data_key, prime32);
            let prod_hi = _mm256_mul_epu32(data_key_hi, prime32);
            let result = _mm256_add_epi64(prod_lo, _mm256_slli_epi64::<32>(prod_hi));
            _mm256_store_si256(acc_ptr.add(i * 4) as *mut __m256i, result);
            i += 1;
        }
    }
}
