// SAFETY: all NEON intrinsics are only called from functions that have
// #[target_feature(enable = "neon")], so the CPU supports the instructions.

use core::arch::aarch64::*;

use crate::xxh3::{ACC_NB, PRIME32_1, SECRET_CONSUME_RATE, STRIPE_LEN};

#[allow(clippy::similar_names)]
#[target_feature(enable = "neon")]
pub unsafe fn accumulate_loop(
    acc: &mut [u64; ACC_NB],
    input: &[u8],
    input_off: usize,
    secret: &[u8],
    secret_off: usize,
    nb_stripes: usize,
) {
    let mut i = 0;
    while i < nb_stripes {
        // SAFETY: callers guarantee `nb_stripes` stripes fit in `input`/`secret`.
        unsafe {
            accumulate_512(
                acc,
                input,
                input_off + i * STRIPE_LEN,
                secret,
                secret_off + i * SECRET_CONSUME_RATE,
            );
        }
        i += 1;
    }
}

#[allow(clippy::similar_names)]
#[target_feature(enable = "neon")]
pub unsafe fn accumulate_512(
    acc: &mut [u64; ACC_NB],
    input: &[u8],
    input_off: usize,
    secret: &[u8],
    secret_off: usize,
) {
    unsafe {
        let acc_ptr = acc.as_mut_ptr();
        let input_ptr = input.as_ptr().add(input_off);
        let secret_ptr = secret.as_ptr().add(secret_off);

        // Process four 64-bit lanes at a time (two 128-bit vectors). The low and
        // high 32-bit halves of both `data_key` vectors are de-interleaved in
        // one `vuzp` pair, so each pair of lanes needs a single widening
        // multiply-accumulate instead of a narrow + shift per vector.
        //
        // SAFETY: the accumulator is read/written with unaligned NEON loads/stores.
        let mut i = 0;
        while i < 4 {
            let acc_vec_1: uint64x2_t = vld1q_u64(acc_ptr.add(i * 2));
            let acc_vec_2: uint64x2_t = vld1q_u64(acc_ptr.add(i * 2 + 2));
            let data_vec_1: uint64x2_t = vld1q_u64(input_ptr.add(i * 16) as *const u64);
            let data_vec_2: uint64x2_t = vld1q_u64(input_ptr.add(i * 16 + 16) as *const u64);
            let key_vec_1: uint64x2_t = vld1q_u64(secret_ptr.add(i * 16) as *const u64);
            let key_vec_2: uint64x2_t = vld1q_u64(secret_ptr.add(i * 16 + 16) as *const u64);

            let data_swap_1 = vextq_u64::<1>(data_vec_1, data_vec_1);
            let data_swap_2 = vextq_u64::<1>(data_vec_2, data_vec_2);
            let data_key_1 = veorq_u64(data_vec_1, key_vec_1);
            let data_key_2 = veorq_u64(data_vec_2, key_vec_2);

            let data_key_lo = vuzp1q_u32(vreinterpretq_u32_u64(data_key_1), vreinterpretq_u32_u64(data_key_2));
            let data_key_hi = vuzp2q_u32(vreinterpretq_u32_u64(data_key_1), vreinterpretq_u32_u64(data_key_2));

            let sum_1 = vmlal_u32(data_swap_1, vget_low_u32(data_key_lo), vget_low_u32(data_key_hi));
            let sum_2 = vmlal_u32(data_swap_2, vget_high_u32(data_key_lo), vget_high_u32(data_key_hi));

            vst1q_u64(acc_ptr.add(i * 2), vaddq_u64(acc_vec_1, sum_1));
            vst1q_u64(acc_ptr.add(i * 2 + 2), vaddq_u64(acc_vec_2, sum_2));
            i += 2;
        }
    }
}

#[allow(clippy::similar_names)]
#[target_feature(enable = "neon")]
pub unsafe fn scramble_acc(acc: &mut [u64; ACC_NB], secret: &[u8], secret_off: usize) {
    unsafe {
        let k_prime_lo = vdup_n_u32(PRIME32_1 as u32);
        let k_prime_hi = vreinterpretq_u32_u64(vdupq_n_u64(PRIME32_1 << 32));
        let acc_ptr = acc.as_mut_ptr();
        let secret_ptr = secret.as_ptr().add(secret_off);

        let mut i = 0;
        while i < 4 {
            let acc_vec: uint64x2_t = vld1q_u64(acc_ptr.add(i * 2));
            let shifted = vshrq_n_u64::<47>(acc_vec);
            let data_vec = veorq_u64(acc_vec, shifted);
            let key_vec: uint64x2_t = vld1q_u64(secret_ptr.add(i * 16) as *const u64);
            let data_key = veorq_u64(data_vec, key_vec);
            let prod_hi = vmulq_u32(vreinterpretq_u32_u64(data_key), k_prime_hi);
            let data_key_lo = vmovn_u64(data_key);
            let result = vmlal_u32(vreinterpretq_u64_u32(prod_hi), data_key_lo, k_prime_lo);
            vst1q_u64(acc_ptr.add(i * 2), result);
            i += 1;
        }
    }
}
