use core::arch::wasm32::*;

use crate::xxh3::{ACC_NB, PRIME32_1};

#[allow(clippy::similar_names)]
pub fn accumulate_512(acc: &mut [u64; ACC_NB], input: &[u8], input_off: usize, secret: &[u8], secret_off: usize) {
    unsafe {
        let acc_ptr = acc.as_mut_ptr() as *mut u64;
        let input_ptr = input.as_ptr().add(input_off);
        let secret_ptr = secret.as_ptr().add(secret_off);

        let mask = i64x2_splat(0x00000000FFFFFFFFi64);

        for i in 0..4 {
            let acc_vec = v128_load(acc_ptr.add(i * 2) as *const v128);
            let data_vec = v128_load(input_ptr.add(i * 16) as *const v128);
            let key_vec = v128_load(secret_ptr.add(i * 16) as *const v128);

            let data_key = v128_xor(data_vec, key_vec);

            let data_key_high = u64x2_shr(data_key, 32);
            let data_key_low = v128_and(data_key, mask);
            let product = i64x2_mul(data_key_low, data_key_high);

            let data_swap = i64x2_shuffle::<1, 0>(data_vec, data_vec);
            let sum = i64x2_add(acc_vec, data_swap);
            let result = i64x2_add(sum, product);

            v128_store(acc_ptr.add(i * 2) as *mut v128, result);
        }
    }
}

#[allow(clippy::similar_names)]
pub fn scramble_acc(acc: &mut [u64; ACC_NB], secret: &[u8], secret_off: usize) {
    unsafe {
        let acc_ptr = acc.as_mut_ptr() as *mut u64;
        let secret_ptr = secret.as_ptr().add(secret_off);

        let prime_vec = i64x2_splat(PRIME32_1 as i64);
        let mask = i64x2_splat(0x00000000FFFFFFFFi64);

        for i in 0..4 {
            let acc_vec = v128_load(acc_ptr.add(i * 2) as *const v128);

            let shifted = u64x2_shr(acc_vec, 47);
            let data_vec = v128_xor(acc_vec, shifted);

            let key_vec = v128_load(secret_ptr.add(i * 16) as *const v128);
            let data_key = v128_xor(data_vec, key_vec);

            let data_key_hi = u64x2_shr(data_key, 32);
            let data_key_lo = v128_and(data_key, mask);

            let prod_lo = i64x2_mul(data_key_lo, prime_vec);
            let prod_hi = i64x2_mul(data_key_hi, prime_vec);
            let prod_hi_shifted = i64x2_shl(prod_hi, 32);
            let result = i64x2_add(prod_lo, prod_hi_shifted);

            v128_store(acc_ptr.add(i * 2) as *mut v128, result);
        }
    }
}
