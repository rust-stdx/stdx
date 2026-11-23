#![allow(unsafe_op_in_unsafe_fn)]
/// aarch64 AES-CTR using ARMv8 Crypto and NEON.
use core::arch::aarch64::*;

use super::aes_arm64::aes_encrypt_block;

/// Byte-reversal shuffle mask: maps byte i ↔ byte 15-i (full 16-byte reversal).
pub(crate) const SWAP_MASK: [u8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

/// Increment the 16-byte big-endian counter block by one.
///
/// This is the NIST SP 800-38A §5.1 standard incrementing function applied to
/// the full 128-bit block: the byte-reversed counter is split into two 64-bit
/// lanes and the carry is propagated from the low lane into the high lane.
#[inline]
pub(crate) fn increment_counter(counter: uint8x16_t) -> uint8x16_t {
    unsafe {
        let swap = vld1q_u8(SWAP_MASK.as_ptr());
        let le = vqtbl1q_u8(counter, swap);
        let lanes = vreinterpretq_u64_u8(le);
        let lo = vgetq_lane_u64::<0>(lanes);
        let hi = vgetq_lane_u64::<1>(lanes);
        let (lo, carry) = lo.overflowing_add(1);
        let hi = hi.wrapping_add(carry as u64);
        let inc = vcombine_u64(vcreate_u64(lo), vcreate_u64(hi));
        vqtbl1q_u8(vreinterpretq_u8_u64(inc), swap)
    }
}

/// XOR the keystream over `in_out` using ARMv8 Crypto extensions.
///
/// `N` is the number of round keys: 11 for AES-128, 15 for AES-256.
/// The counter is read from and written back to `counter`; it advances once
/// per complete 16-byte block. If `in_out` ends in a partial block, that block
/// is encrypted without advancing the counter, so a caller that wants to
/// continue mid-block must track the offset itself. `AesCtr` only passes whole
/// blocks (it buffers the tail), so it does not depend on that behaviour.
pub(crate) unsafe fn xor_keystream_armv8<const N: usize>(
    round_keys: &[uint8x16_t; N],
    counter: &mut [u8; 16],
    in_out: &mut [u8],
) {
    const {
        assert!(N == 11 || N == 15);
    }

    let n = in_out.len();
    let mut i = 0;
    let mut ctr = vld1q_u8(counter.as_ptr());

    while i + 16 <= n {
        let ks = aes_encrypt_block::<N>(round_keys, ctr);
        let p = vld1q_u8(in_out.as_ptr().add(i));
        vst1q_u8(in_out.as_mut_ptr().add(i), veorq_u8(p, ks));
        ctr = increment_counter(ctr);
        i += 16;
    }
    if i < n {
        let ks = aes_encrypt_block::<N>(round_keys, ctr);
        let mut ks_bytes = [0u8; 16];
        vst1q_u8(ks_bytes.as_mut_ptr(), ks);
        for k in 0..n - i {
            in_out[i + k] ^= ks_bytes[k];
        }
    }

    vst1q_u8(counter.as_mut_ptr(), ctr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aes::{
        aes::{RoundKeysSoftware, expand_key},
        aes_arm64::expand_key_armv8,
        aes_ct::xor_keystream_soft,
    };

    /// Counters whose carry crosses the low 32-bit word, the low 64-bit lane and
    /// the full 128-bit wrap.
    fn counters() -> [[u8; 16]; 6] {
        [
            0u128,
            0xff,
            0xffff_ffff,
            0xffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff_0000_0000_0000_0000,
            u128::MAX,
        ]
        .map(u128::to_be_bytes)
    }

    fn pattern(len: usize) -> Vec<u8> {
        (0..len).map(|i| (i as u8).wrapping_mul(37).wrapping_add(11)).collect()
    }

    fn to_bytes(v: uint8x16_t) -> [u8; 16] {
        let mut out = [0u8; 16];
        unsafe { vst1q_u8(out.as_mut_ptr(), v) };
        out
    }

    /// The NEON increment must be exactly `counter + 1 mod 2^128` in big-endian
    /// form, including carries out of each 64-bit lane.
    #[test]
    fn increment_counter_is_full_128_bit() {
        let values = [
            0u128,
            1,
            0xfe,
            0xff,
            0xffff_ffff,
            0xffff_ffff_ffff_ffff,
            0x00ff_00ff_00ff_00ff_00ff_00ff_00ff_00ff,
            u128::MAX - 1,
            u128::MAX,
        ];
        for value in values {
            let input = value.to_be_bytes();
            let got = increment_counter(unsafe { vld1q_u8(input.as_ptr()) });
            assert_eq!(to_bytes(got), value.wrapping_add(1).to_be_bytes(), "value={value:#034x}");
        }
    }

    /// Direct kernel check, including non-zero slice offsets and canaries, so
    /// the pointer arithmetic in the multi-block loop is exercised without the
    /// dispatcher.
    fn check_kernel<const N: usize>(soft: &RoundKeysSoftware<N>) {
        let rk = expand_key_armv8(*soft);
        let mut lengths: Vec<usize> = (0..=260).collect();
        lengths.extend([383, 384, 385, 511, 512, 513, 1023, 1024, 1025]);

        for base in counters() {
            for &len in &lengths {
                for offset in [0usize, 1, 15, 17] {
                    let pt = pattern(len);
                    let mut got_buf = vec![0xa5u8; offset + len + 16];
                    got_buf[offset..offset + len].copy_from_slice(&pt);
                    let mut got_ctr = base;

                    let mut want = pt.clone();
                    let mut want_ctr = base;

                    unsafe {
                        xor_keystream_armv8(&rk, &mut got_ctr, &mut got_buf[offset..offset + len]);
                    }
                    xor_keystream_soft(soft, &mut want_ctr, &mut want);

                    assert_eq!(
                        &got_buf[offset..offset + len],
                        &want[..],
                        "ciphertext len={len} offset={offset}"
                    );
                    assert_eq!(got_ctr, want_ctr, "counter len={len} offset={offset}");
                    assert!(
                        got_buf[..offset].iter().all(|&b| b == 0xa5),
                        "prefix clobbered len={len} offset={offset}"
                    );
                    assert!(
                        got_buf[offset + len..].iter().all(|&b| b == 0xa5),
                        "suffix clobbered len={len} offset={offset}"
                    );
                }
            }
        }
    }

    #[test]
    fn kernel_matches_soft_aes128() {
        let key = [0x3cu8; 16];
        check_kernel(&expand_key::<11>(&key));
    }

    #[test]
    fn kernel_matches_soft_aes256() {
        let key = [0x9eu8; 32];
        check_kernel(&expand_key::<15>(&key));
    }
}
