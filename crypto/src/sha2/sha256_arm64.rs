#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::aarch64::*;

use super::sha256::SHA256_K;

#[target_feature(enable = "sha2")]
pub(crate) unsafe fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) {
    let mut abcd = vld1q_u32(state[0..4].as_ptr());
    let mut efgh = vld1q_u32(state[4..8].as_ptr());

    for block in blocks {
        let abcd_orig = abcd;
        let efgh_orig = efgh;

        let mut s0 = vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block[0..16].as_ptr())));
        let mut s1 = vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block[16..32].as_ptr())));
        let mut s2 = vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block[32..48].as_ptr())));
        let mut s3 = vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block[48..64].as_ptr())));

        let mut tmp = vaddq_u32(s0, vld1q_u32(&SHA256_K[0]));
        let mut abcd_prev = abcd;
        abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
        efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

        tmp = vaddq_u32(s1, vld1q_u32(&SHA256_K[4]));
        abcd_prev = abcd;
        abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
        efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

        tmp = vaddq_u32(s2, vld1q_u32(&SHA256_K[8]));
        abcd_prev = abcd;
        abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
        efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

        tmp = vaddq_u32(s3, vld1q_u32(&SHA256_K[12]));
        abcd_prev = abcd;
        abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
        efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

        for t in (16..64).step_by(16) {
            s0 = vsha256su1q_u32(vsha256su0q_u32(s0, s1), s2, s3);
            tmp = vaddq_u32(s0, vld1q_u32(&SHA256_K[t]));
            abcd_prev = abcd;
            abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
            efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

            s1 = vsha256su1q_u32(vsha256su0q_u32(s1, s2), s3, s0);
            tmp = vaddq_u32(s1, vld1q_u32(&SHA256_K[t + 4]));
            abcd_prev = abcd;
            abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
            efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

            s2 = vsha256su1q_u32(vsha256su0q_u32(s2, s3), s0, s1);
            tmp = vaddq_u32(s2, vld1q_u32(&SHA256_K[t + 8]));
            abcd_prev = abcd;
            abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
            efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

            s3 = vsha256su1q_u32(vsha256su0q_u32(s3, s0), s1, s2);
            tmp = vaddq_u32(s3, vld1q_u32(&SHA256_K[t + 12]));
            abcd_prev = abcd;
            abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
            efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);
        }

        abcd = vaddq_u32(abcd, abcd_orig);
        efgh = vaddq_u32(efgh, efgh_orig);
    }

    vst1q_u32(state[0..4].as_mut_ptr(), abcd);
    vst1q_u32(state[4..8].as_mut_ptr(), efgh);
}
