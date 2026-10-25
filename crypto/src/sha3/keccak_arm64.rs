//! Keccak-p[1600] on Armv8.4-A with FEAT_SHA3.
//!
//! [`p1600_armv8`] is the single-state permutation used by the SHA-3 and SHAKE
//! sponges. [`permute_lanes`] holds the round itself and is shared with the
//! batched tree-hash kernels in `keccak_batch`, whose two 64-bit lanes carry
//! two independent states.
//!
//! The round is written with the intrinsics for `EOR3`, `RAX1`, `XAR` and
//! `BCAX`, which is what makes it competitive with hand-written assembly:
//! `EOR3` and `BCAX` fuse the three-input XORs of theta and chi, and `XAR`
//! fuses the theta `D`-value XOR with the rho rotation.
//!
//! FEAT_SHA3 is optional on AArch64, so these functions may only be called
//! after `std::arch::is_aarch64_feature_detected!("sha3")` returned `true`, or
//! when the feature is enabled at compile time.
#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::aarch64::*;

use super::keccak::ROUND_CONSTANTS;

/// The Keccak-p[`ROUNDS`] permutation of one 1600-bit state.
///
/// Only the lower 64-bit lane of each 128-bit register carries state; the
/// upper lane is zero and is discarded. The two lanes are independent, so the
/// result does not depend on it. A CPU without FEAT_SHA3 cannot run this
/// function. Instantiating it with `ROUNDS > 24` fails to compile.
///
/// # Safety
///
/// The CPU must implement FEAT_SHA3.
#[target_feature(enable = "sha3")]
#[inline]
pub(crate) unsafe fn p1600_armv8<const ROUNDS: usize>(state: &mut [u64; 25]) {
    const {
        assert!(ROUNDS <= 24, "A round count greater than 24 is not supported.");
    }

    let mut lanes: [uint64x2_t; 25] = core::array::from_fn(|i| {
        // `vld1q_lane` is a plain `ldr d`; `vdupq_n_u64` would let LLVM merge
        // two adjacent words into one 16-byte load plus two lane extracts,
        // which costs an extra vector register for the whole round.
        vld1q_lane_u64::<0>(state.as_ptr().add(i), vdupq_n_u64(0))
    });
    permute_lanes::<ROUNDS>(&mut lanes);

    for (word, &lane) in state.iter_mut().zip(lanes.iter()) {
        vst1q_lane_u64::<0>(word, lane);
    }
}

/// The Keccak-p[`ROUNDS`] permutation of two independent states, one per lane.
///
/// Instantiating this function with `ROUNDS > 24` fails to compile.
///
/// # Safety
///
/// The CPU must implement FEAT_SHA3.
///
#[target_feature(enable = "sha3")]
#[inline]
pub(crate) unsafe fn permute_lanes<const ROUNDS: usize>(lanes: &mut [uint64x2_t; 25]) {
    const {
        assert!(ROUNDS <= 24, "A round count greater than 24 is not supported.");
    }

    let mut v0 = lanes[0];
    let mut v1 = lanes[1];
    let mut v2 = lanes[2];
    let mut v3 = lanes[3];
    let mut v4 = lanes[4];
    let mut v5 = lanes[5];
    let mut v6 = lanes[6];
    let mut v7 = lanes[7];
    let mut v8 = lanes[8];
    let mut v9 = lanes[9];
    let mut v10 = lanes[10];
    let mut v11 = lanes[11];
    let mut v12 = lanes[12];
    let mut v13 = lanes[13];
    let mut v14 = lanes[14];
    let mut v15 = lanes[15];
    let mut v16 = lanes[16];
    let mut v17 = lanes[17];
    let mut v18 = lanes[18];
    let mut v19 = lanes[19];
    let mut v20 = lanes[20];
    let mut v21 = lanes[21];
    let mut v22 = lanes[22];
    let mut v23 = lanes[23];
    let mut v24 = lanes[24];

    for r in (24 - ROUNDS)..24 {
        let rc = ROUND_CONSTANTS[r];

        // theta
        let mut v25 = veor3q_u64(v20, v15, v10);
        let mut v26 = veor3q_u64(v21, v16, v11);
        let mut v27 = veor3q_u64(v22, v17, v12);
        let mut v28 = veor3q_u64(v23, v18, v13);
        let mut v29 = veor3q_u64(v24, v19, v14);
        v25 = veor3q_u64(v25, v5, v0);
        v26 = veor3q_u64(v26, v6, v1);
        v27 = veor3q_u64(v27, v7, v2);
        v28 = veor3q_u64(v28, v8, v3);
        v29 = veor3q_u64(v29, v9, v4);
        let mut v30 = vrax1q_u64(v25, v27);
        let mut v31 = vrax1q_u64(v26, v28);
        v27 = vrax1q_u64(v27, v29);
        v28 = vrax1q_u64(v28, v25);
        v29 = vrax1q_u64(v29, v26);

        // rho and pi (theta's D is folded into each XAR)
        v0 = veorq_u64(v0, v29);
        v25 = vxarq_u64::<{ 64 - 1 }>(v1, v30);
        v1 = vxarq_u64::<{ 64 - 44 }>(v6, v30);
        v6 = vxarq_u64::<{ 64 - 20 }>(v9, v28);
        v9 = vxarq_u64::<{ 64 - 61 }>(v22, v31);
        v22 = vxarq_u64::<{ 64 - 39 }>(v14, v28);
        v14 = vxarq_u64::<{ 64 - 18 }>(v20, v29);
        v26 = vxarq_u64::<{ 64 - 62 }>(v2, v31);
        v2 = vxarq_u64::<{ 64 - 43 }>(v12, v31);
        v12 = vxarq_u64::<{ 64 - 25 }>(v13, v27);
        v13 = vxarq_u64::<{ 64 - 8 }>(v19, v28);
        v19 = vxarq_u64::<{ 64 - 56 }>(v23, v27);
        v23 = vxarq_u64::<{ 64 - 41 }>(v15, v29);
        v15 = vxarq_u64::<{ 64 - 27 }>(v4, v28);
        v28 = vxarq_u64::<{ 64 - 14 }>(v24, v28);
        v24 = vxarq_u64::<{ 64 - 2 }>(v21, v30);
        v8 = vxarq_u64::<{ 64 - 55 }>(v8, v27);
        v4 = vxarq_u64::<{ 64 - 45 }>(v16, v30);
        v16 = vxarq_u64::<{ 64 - 36 }>(v5, v29);
        v5 = vxarq_u64::<{ 64 - 28 }>(v3, v27);
        v27 = vxarq_u64::<{ 64 - 21 }>(v18, v27);
        v3 = vxarq_u64::<{ 64 - 15 }>(v17, v31);
        v30 = vxarq_u64::<{ 64 - 10 }>(v11, v30);
        v31 = vxarq_u64::<{ 64 - 6 }>(v7, v31);
        v29 = vxarq_u64::<{ 64 - 3 }>(v10, v29);

        // chi
        v20 = vbcaxq_u64(v26, v22, v8);
        v21 = vbcaxq_u64(v8, v23, v22);
        v22 = vbcaxq_u64(v22, v24, v23);
        v23 = vbcaxq_u64(v23, v26, v24);
        v24 = vbcaxq_u64(v24, v8, v26);

        v17 = vbcaxq_u64(v30, v19, v3);
        v18 = vbcaxq_u64(v3, v15, v19);
        v19 = vbcaxq_u64(v19, v16, v15);
        v15 = vbcaxq_u64(v15, v30, v16);
        v16 = vbcaxq_u64(v16, v3, v30);

        v10 = vbcaxq_u64(v25, v12, v31);
        v11 = vbcaxq_u64(v31, v13, v12);
        v12 = vbcaxq_u64(v12, v14, v13);
        v13 = vbcaxq_u64(v13, v25, v14);
        v14 = vbcaxq_u64(v14, v31, v25);

        v7 = vbcaxq_u64(v29, v9, v4);
        v8 = vbcaxq_u64(v4, v5, v9);
        v9 = vbcaxq_u64(v9, v6, v5);
        v5 = vbcaxq_u64(v5, v29, v6);
        v6 = vbcaxq_u64(v6, v4, v29);

        v3 = vbcaxq_u64(v27, v0, v28);
        v4 = vbcaxq_u64(v28, v1, v0);
        v0 = vbcaxq_u64(v0, v2, v1);
        v1 = vbcaxq_u64(v1, v27, v2);
        v2 = vbcaxq_u64(v2, v28, v27);

        // iota
        v0 = veorq_u64(v0, vdupq_n_u64(rc));
    }

    lanes[0] = v0;
    lanes[1] = v1;
    lanes[2] = v2;
    lanes[3] = v3;
    lanes[4] = v4;
    lanes[5] = v5;
    lanes[6] = v6;
    lanes[7] = v7;
    lanes[8] = v8;
    lanes[9] = v9;
    lanes[10] = v10;
    lanes[11] = v11;
    lanes[12] = v12;
    lanes[13] = v13;
    lanes[14] = v14;
    lanes[15] = v15;
    lanes[16] = v16;
    lanes[17] = v17;
    lanes[18] = v18;
    lanes[19] = v19;
    lanes[20] = v20;
    lanes[21] = v21;
    lanes[22] = v22;
    lanes[23] = v23;
    lanes[24] = v24;
}
