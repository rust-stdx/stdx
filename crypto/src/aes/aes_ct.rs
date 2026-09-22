//! Constant-time, table-free software AES-128 / AES-256.
//!
//! This is the software fallback used on CPUs without AES hardware
//! acceleration (for example 32-bit microcontrollers) and to derive the GCM
//! subkey `H = E_K(0)`.
//!
//! The state is held in a *bitsliced* representation: eight 32-bit words,
//! each carrying one bit of every byte of **two** 16-byte blocks. `SubBytes`
//! is a straight-line Boolean circuit (Boyar–Peralta) expressed only with
//! `&`, `|`, `^` and `!`, while `ShiftRows` and `MixColumns` are fixed
//! shift/mask/XOR networks. The key schedule uses the same circuit via
//! [`sub_word`].
//!
//! As a result the implementation contains no secret-dependent branches and
//! never indexes memory with key- or data-derived values, which makes it
//! resistant to the cache- and timing-based side channels that affect
//! T-table software AES. Processing two blocks per call also fills a 32-bit
//! register completely, which is what makes it reasonably fast on 32-bit
//! targets.
//!
//! The general technique and the S-box circuit follow Thomas Pornin's
//! constant-time AES implementation in BearSSL (MIT licensed), reimplemented
//! here for this crate.

use super::aes::RoundKeysSoftware;

/// Bitsliced state: eight 32-bit bit-planes covering two 16-byte blocks.
type State = [u32; 8];

/// Maximum number of expanded round-key words (AES-256: 8 × 15).
const MAX_SKEY: usize = 120;
/// Maximum number of compressed round-key words (AES-256: 4 × 15).
const MAX_COMP_SKEY: usize = 60;

/// Apply the AES S-box to all 32 byte-lanes of a bitsliced state.
///
/// This is a direct transcription of the Boyar–Peralta combinational circuit
/// ("A new combinational logic minimization technique with applications to
/// cryptology", <https://eprint.iacr.org/2009/191.pdf>) operating on eight
/// bit-planes at once. It is pure bitwise logic: no branches, no lookups.
#[inline(always)]
fn bitslice_sbox(q: &mut State) {
    // Inputs and outputs are numbered in reverse order (x0 = high bit).
    let (x0, x1, x2, x3, x4, x5, x6, x7) = (q[7], q[6], q[5], q[4], q[3], q[2], q[1], q[0]);

    // Top linear transformation.
    let y14 = x3 ^ x5;
    let y13 = x0 ^ x6;
    let y9 = x0 ^ x3;
    let y8 = x0 ^ x5;
    let t0 = x1 ^ x2;
    let y1 = t0 ^ x7;
    let y4 = y1 ^ x3;
    let y12 = y13 ^ y14;
    let y2 = y1 ^ x0;
    let y5 = y1 ^ x6;
    let y3 = y5 ^ y8;
    let t1 = x4 ^ y12;
    let y15 = t1 ^ x5;
    let y20 = t1 ^ x1;
    let y6 = y15 ^ x7;
    let y10 = y15 ^ t0;
    let y11 = y20 ^ y9;
    let y7 = x7 ^ y11;
    let y17 = y10 ^ y11;
    let y19 = y10 ^ y8;
    let y16 = t0 ^ y11;
    let y21 = y13 ^ y16;
    let y18 = x0 ^ y16;

    // Non-linear section.
    let t2 = y12 & y15;
    let t3 = y3 & y6;
    let t4 = t3 ^ t2;
    let t5 = y4 & x7;
    let t6 = t5 ^ t2;
    let t7 = y13 & y16;
    let t8 = y5 & y1;
    let t9 = t8 ^ t7;
    let t10 = y2 & y7;
    let t11 = t10 ^ t7;
    let t12 = y9 & y11;
    let t13 = y14 & y17;
    let t14 = t13 ^ t12;
    let t15 = y8 & y10;
    let t16 = t15 ^ t12;
    let t17 = t4 ^ t14;
    let t18 = t6 ^ t16;
    let t19 = t9 ^ t14;
    let t20 = t11 ^ t16;
    let t21 = t17 ^ y20;
    let t22 = t18 ^ y19;
    let t23 = t19 ^ y21;
    let t24 = t20 ^ y18;

    let t25 = t21 ^ t22;
    let t26 = t21 & t23;
    let t27 = t24 ^ t26;
    let t28 = t25 & t27;
    let t29 = t28 ^ t22;
    let t30 = t23 ^ t24;
    let t31 = t22 ^ t26;
    let t32 = t31 & t30;
    let t33 = t32 ^ t24;
    let t34 = t23 ^ t33;
    let t35 = t27 ^ t33;
    let t36 = t24 & t35;
    let t37 = t36 ^ t34;
    let t38 = t27 ^ t36;
    let t39 = t29 & t38;
    let t40 = t25 ^ t39;

    let t41 = t40 ^ t37;
    let t42 = t29 ^ t33;
    let t43 = t29 ^ t40;
    let t44 = t33 ^ t37;
    let t45 = t42 ^ t41;
    let z0 = t44 & y15;
    let z1 = t37 & y6;
    let z2 = t33 & x7;
    let z3 = t43 & y16;
    let z4 = t40 & y1;
    let z5 = t29 & y7;
    let z6 = t42 & y11;
    let z7 = t45 & y17;
    let z8 = t41 & y10;
    let z9 = t44 & y12;
    let z10 = t37 & y3;
    let z11 = t33 & y4;
    let z12 = t43 & y13;
    let z13 = t40 & y5;
    let z14 = t29 & y2;
    let z15 = t42 & y9;
    let z16 = t45 & y14;
    let z17 = t41 & y8;

    // Bottom linear transformation.
    let t46 = z15 ^ z16;
    let t47 = z10 ^ z11;
    let t48 = z5 ^ z13;
    let t49 = z9 ^ z10;
    let t50 = z2 ^ z12;
    let t51 = z2 ^ z5;
    let t52 = z7 ^ z8;
    let t53 = z0 ^ z3;
    let t54 = z6 ^ z7;
    let t55 = z16 ^ z17;
    let t56 = z12 ^ t48;
    let t57 = t50 ^ t53;
    let t58 = z4 ^ t46;
    let t59 = z3 ^ t54;
    let t60 = t46 ^ t57;
    let t61 = z14 ^ t57;
    let t62 = t52 ^ t58;
    let t63 = t49 ^ t58;
    let t64 = z4 ^ t59;
    let t65 = t61 ^ t62;
    let t66 = z1 ^ t63;
    let s0 = t59 ^ t63;
    let s6 = t56 ^ !t62;
    let s7 = t48 ^ !t60;
    let t67 = t64 ^ t65;
    let s3 = t53 ^ t66;
    let s4 = t51 ^ t66;
    let s5 = t47 ^ t65;
    let s1 = t64 ^ !s3;
    let s2 = t55 ^ !t67;

    q[7] = s0;
    q[6] = s1;
    q[5] = s2;
    q[4] = s3;
    q[3] = s4;
    q[2] = s5;
    q[1] = s6;
    q[0] = s7;
}

/// Apply the inverse AES S-box to all 32 byte-lanes of a bitsliced state.
///
/// Instead of a second circuit, the inverse is built from the forward S-box
/// using `S⁻¹(x) = B(S(B(x ^ 0x63)) ^ 0x63)`, where `B` is the inverse of the
/// affine part of the S-box. This keeps the constant-time property while
/// avoiding duplicated code.
#[inline(always)]
fn bitslice_inv_sbox(q: &mut State) {
    let (mut q0, mut q1, q2, q3, q4, mut q5, mut q6, q7) = (q[0], q[1], q[2], q[3], q[4], q[5], q[6], q[7]);

    q0 = !q0;
    q1 = !q1;
    q5 = !q5;
    q6 = !q6;
    let t7 = q1 ^ q4 ^ q6;
    let t6 = q0 ^ q3 ^ q5;
    let t5 = q7 ^ q2 ^ q4;
    let t4 = q6 ^ q1 ^ q3;
    let t3 = q5 ^ q0 ^ q2;
    let t2 = q4 ^ q7 ^ q1;
    let t1 = q3 ^ q6 ^ q0;
    let t0 = q2 ^ q5 ^ q7;
    q[0] = t0;
    q[1] = t1;
    q[2] = t2;
    q[3] = t3;
    q[4] = t4;
    q[5] = t5;
    q[6] = t6;
    q[7] = t7;

    bitslice_sbox(q);

    let (mut q0, mut q1, q2, q3, q4, mut q5, mut q6, q7) = (q[0], q[1], q[2], q[3], q[4], q[5], q[6], q[7]);
    q0 = !q0;
    q1 = !q1;
    q5 = !q5;
    q6 = !q6;
    let t7 = q1 ^ q4 ^ q6;
    let t6 = q0 ^ q3 ^ q5;
    let t5 = q7 ^ q2 ^ q4;
    let t4 = q6 ^ q1 ^ q3;
    let t3 = q5 ^ q0 ^ q2;
    let t2 = q4 ^ q7 ^ q1;
    let t1 = q3 ^ q6 ^ q0;
    let t0 = q2 ^ q5 ^ q7;
    q[0] = t0;
    q[1] = t1;
    q[2] = t2;
    q[3] = t3;
    q[4] = t4;
    q[5] = t5;
    q[6] = t6;
    q[7] = t7;
}

#[inline(always)]
fn swapn(q: &mut State, cl: u32, ch: u32, s: u32, i: usize, j: usize) {
    let a = q[i];
    let b = q[j];
    q[i] = (a & cl) | ((b & cl) << s);
    q[j] = ((a & ch) >> s) | (b & ch);
}

/// Transpose the eight words between the "packed" and bitsliced layouts.
///
/// This is an involution: applying it twice restores the original words.
#[inline(always)]
fn ortho(q: &mut State) {
    swapn(q, 0x5555_5555, 0xAAAA_AAAA, 1, 0, 1);
    swapn(q, 0x5555_5555, 0xAAAA_AAAA, 1, 2, 3);
    swapn(q, 0x5555_5555, 0xAAAA_AAAA, 1, 4, 5);
    swapn(q, 0x5555_5555, 0xAAAA_AAAA, 1, 6, 7);

    swapn(q, 0x3333_3333, 0xCCCC_CCCC, 2, 0, 2);
    swapn(q, 0x3333_3333, 0xCCCC_CCCC, 2, 1, 3);
    swapn(q, 0x3333_3333, 0xCCCC_CCCC, 2, 4, 6);
    swapn(q, 0x3333_3333, 0xCCCC_CCCC, 2, 5, 7);

    swapn(q, 0x0F0F_0F0F, 0xF0F0_F0F0, 4, 0, 4);
    swapn(q, 0x0F0F_0F0F, 0xF0F0_F0F0, 4, 1, 5);
    swapn(q, 0x0F0F_0F0F, 0xF0F0_F0F0, 4, 2, 6);
    swapn(q, 0x0F0F_0F0F, 0xF0F0_F0F0, 4, 3, 7);
}

#[inline(always)]
fn add_round_key(q: &mut State, sk: &[u32]) {
    for i in 0..8 {
        q[i] ^= sk[i];
    }
}

#[inline(always)]
fn shift_rows(q: &mut State) {
    for x in q.iter_mut() {
        let v = *x;
        *x = (v & 0x0000_00FF)
            | ((v & 0x0000_FC00) >> 2)
            | ((v & 0x0000_0300) << 6)
            | ((v & 0x00F0_0000) >> 4)
            | ((v & 0x000F_0000) << 4)
            | ((v & 0xC000_0000) >> 6)
            | ((v & 0x3F00_0000) << 2);
    }
}

#[inline(always)]
fn inv_shift_rows(q: &mut State) {
    for x in q.iter_mut() {
        let v = *x;
        *x = (v & 0x0000_00FF)
            | ((v & 0x0000_3F00) << 2)
            | ((v & 0x0000_C000) >> 6)
            | ((v & 0x000F_0000) << 4)
            | ((v & 0x00F0_0000) >> 4)
            | ((v & 0x0300_0000) << 6)
            | ((v & 0xFC00_0000) >> 2);
    }
}

#[inline(always)]
fn rotr8(x: u32) -> u32 {
    x.rotate_right(8)
}

#[inline(always)]
fn rotr16(x: u32) -> u32 {
    x.rotate_right(16)
}

#[inline(always)]
fn mix_columns(q: &mut State) {
    let q0 = q[0];
    let q1 = q[1];
    let q2 = q[2];
    let q3 = q[3];
    let q4 = q[4];
    let q5 = q[5];
    let q6 = q[6];
    let q7 = q[7];
    let r0 = rotr8(q0);
    let r1 = rotr8(q1);
    let r2 = rotr8(q2);
    let r3 = rotr8(q3);
    let r4 = rotr8(q4);
    let r5 = rotr8(q5);
    let r6 = rotr8(q6);
    let r7 = rotr8(q7);

    q[0] = q7 ^ r7 ^ r0 ^ rotr16(q0 ^ r0);
    q[1] = q0 ^ r0 ^ q7 ^ r7 ^ r1 ^ rotr16(q1 ^ r1);
    q[2] = q1 ^ r1 ^ r2 ^ rotr16(q2 ^ r2);
    q[3] = q2 ^ r2 ^ q7 ^ r7 ^ r3 ^ rotr16(q3 ^ r3);
    q[4] = q3 ^ r3 ^ q7 ^ r7 ^ r4 ^ rotr16(q4 ^ r4);
    q[5] = q4 ^ r4 ^ r5 ^ rotr16(q5 ^ r5);
    q[6] = q5 ^ r5 ^ r6 ^ rotr16(q6 ^ r6);
    q[7] = q6 ^ r6 ^ r7 ^ rotr16(q7 ^ r7);
}

#[inline(always)]
fn inv_mix_columns(q: &mut State) {
    let q0 = q[0];
    let q1 = q[1];
    let q2 = q[2];
    let q3 = q[3];
    let q4 = q[4];
    let q5 = q[5];
    let q6 = q[6];
    let q7 = q[7];
    let r0 = rotr8(q0);
    let r1 = rotr8(q1);
    let r2 = rotr8(q2);
    let r3 = rotr8(q3);
    let r4 = rotr8(q4);
    let r5 = rotr8(q5);
    let r6 = rotr8(q6);
    let r7 = rotr8(q7);

    q[0] = q5 ^ q6 ^ q7 ^ r0 ^ r5 ^ r7 ^ rotr16(q0 ^ q5 ^ q6 ^ r0 ^ r5);
    q[1] = q0 ^ q5 ^ r0 ^ r1 ^ r5 ^ r6 ^ r7 ^ rotr16(q1 ^ q5 ^ q7 ^ r1 ^ r5 ^ r6);
    q[2] = q0 ^ q1 ^ q6 ^ r1 ^ r2 ^ r6 ^ r7 ^ rotr16(q0 ^ q2 ^ q6 ^ r2 ^ r6 ^ r7);
    q[3] = q0 ^ q1 ^ q2 ^ q5 ^ q6 ^ r0 ^ r2 ^ r3 ^ r5 ^ rotr16(q0 ^ q1 ^ q3 ^ q5 ^ q6 ^ q7 ^ r0 ^ r3 ^ r5 ^ r7);
    q[4] = q1 ^ q2 ^ q3 ^ q5 ^ r1 ^ r3 ^ r4 ^ r5 ^ r6 ^ r7 ^ rotr16(q1 ^ q2 ^ q4 ^ q5 ^ q7 ^ r1 ^ r4 ^ r5 ^ r6);
    q[5] = q2 ^ q3 ^ q4 ^ q6 ^ r2 ^ r4 ^ r5 ^ r6 ^ r7 ^ rotr16(q2 ^ q3 ^ q5 ^ q6 ^ r2 ^ r5 ^ r6 ^ r7);
    q[6] = q3 ^ q4 ^ q5 ^ q7 ^ r3 ^ r5 ^ r6 ^ r7 ^ rotr16(q3 ^ q4 ^ q6 ^ q7 ^ r3 ^ r6 ^ r7);
    q[7] = q4 ^ q5 ^ q6 ^ r4 ^ r6 ^ r7 ^ rotr16(q4 ^ q5 ^ q7 ^ r4 ^ r7);
}

/// Encrypt one or two blocks held in bitsliced form.
fn encrypt_bitsliced(num_rounds: usize, skey: &[u32], q: &mut State) {
    add_round_key(q, &skey[0..8]);
    for u in 1..num_rounds {
        bitslice_sbox(q);
        shift_rows(q);
        mix_columns(q);
        add_round_key(q, &skey[u * 8..u * 8 + 8]);
    }
    bitslice_sbox(q);
    shift_rows(q);
    add_round_key(q, &skey[num_rounds * 8..num_rounds * 8 + 8]);
}

/// Decrypt one or two blocks held in bitsliced form.
fn decrypt_bitsliced(num_rounds: usize, skey: &[u32], q: &mut State) {
    add_round_key(q, &skey[num_rounds * 8..num_rounds * 8 + 8]);
    for u in (1..num_rounds).rev() {
        inv_shift_rows(q);
        bitslice_inv_sbox(q);
        add_round_key(q, &skey[u * 8..u * 8 + 8]);
        inv_mix_columns(q);
    }
    inv_shift_rows(q);
    bitslice_inv_sbox(q);
    add_round_key(q, &skey[0..8]);
}

/// Apply the AES S-box to the four bytes of a 32-bit little-endian word.
///
/// Used by the key schedule (`SubWord`). Constant-time: it runs the same
/// circuit as the data path.
#[inline]
pub(crate) fn sub_word(x: u32) -> u32 {
    let mut q = [x; 8];
    ortho(&mut q);
    bitslice_sbox(&mut q);
    ortho(&mut q);
    q[0]
}

/// Bitsliced key schedule for AES-128 (`N = 11`) or AES-256 (`N = 15`).
///
/// The round keys are stored in the compressed form used by the round
/// function and expanded on demand by [`CtSchedule::expand`].
pub(crate) struct CtSchedule {
    comp: [u32; MAX_COMP_SKEY],
    num_rounds: usize,
}

impl CtSchedule {
    /// Expand the compressed schedule into eight round-key words per round.
    #[inline]
    fn expand(&self) -> [u32; MAX_SKEY] {
        let mut skey = [0u32; MAX_SKEY];
        let n = (self.num_rounds + 1) << 2;
        for u in 0..n {
            let x = self.comp[u] & 0x5555_5555;
            let y = self.comp[u] & 0xAAAA_AAAA;
            skey[2 * u] = x | (x << 1);
            skey[2 * u + 1] = y | (y >> 1);
        }
        skey
    }
}

/// Build a bitsliced schedule from byte-oriented round keys.
pub(crate) fn keysched<const N: usize>(round_keys: &RoundKeysSoftware<N>) -> CtSchedule {
    const {
        assert!(N == 11 || N == 15);
    }
    let num_rounds = if N == 11 { 10 } else { 14 };
    let mut comp = [0u32; MAX_COMP_SKEY];

    for (round, rk) in round_keys.iter().enumerate() {
        let w = [
            u32::from_le_bytes(rk[0..4].try_into().unwrap()),
            u32::from_le_bytes(rk[4..8].try_into().unwrap()),
            u32::from_le_bytes(rk[8..12].try_into().unwrap()),
            u32::from_le_bytes(rk[12..16].try_into().unwrap()),
        ];
        let mut q = [w[0], w[0], w[1], w[1], w[2], w[2], w[3], w[3]];
        ortho(&mut q);
        for i in 0..4 {
            comp[round * 4 + i] = (q[2 * i] & 0x5555_5555) | (q[2 * i + 1] & 0xAAAA_AAAA);
        }
    }

    CtSchedule {
        comp,
        num_rounds,
    }
}

#[inline(always)]
fn load_pair(a: &[u8; 16], b: &[u8; 16]) -> State {
    let mut q = [
        u32::from_le_bytes(a[0..4].try_into().unwrap()),
        u32::from_le_bytes(b[0..4].try_into().unwrap()),
        u32::from_le_bytes(a[4..8].try_into().unwrap()),
        u32::from_le_bytes(b[4..8].try_into().unwrap()),
        u32::from_le_bytes(a[8..12].try_into().unwrap()),
        u32::from_le_bytes(b[8..12].try_into().unwrap()),
        u32::from_le_bytes(a[12..16].try_into().unwrap()),
        u32::from_le_bytes(b[12..16].try_into().unwrap()),
    ];
    ortho(&mut q);
    q
}

#[inline(always)]
fn store_pair(q: &mut State, a: &mut [u8; 16], b: &mut [u8; 16]) {
    ortho(q);
    a[0..4].copy_from_slice(&q[0].to_le_bytes());
    b[0..4].copy_from_slice(&q[1].to_le_bytes());
    a[4..8].copy_from_slice(&q[2].to_le_bytes());
    b[4..8].copy_from_slice(&q[3].to_le_bytes());
    a[8..12].copy_from_slice(&q[4].to_le_bytes());
    b[8..12].copy_from_slice(&q[5].to_le_bytes());
    a[12..16].copy_from_slice(&q[6].to_le_bytes());
    b[12..16].copy_from_slice(&q[7].to_le_bytes());
}

/// Encrypt two independent 16-byte blocks in a single bitsliced pass.
///
/// Both blocks use the same round keys; the two results are independent.
#[inline]
pub(crate) fn encrypt_pair(sched: &CtSchedule, a: &mut [u8; 16], b: &mut [u8; 16]) {
    let skey = sched.expand();
    let mut q = load_pair(a, b);
    encrypt_bitsliced(sched.num_rounds, &skey, &mut q);
    store_pair(&mut q, a, b);
}

/// Encrypt a single block with an already-built schedule.
#[inline]
pub(crate) fn encrypt_block_sched(sched: &CtSchedule, block: &[u8; 16]) -> [u8; 16] {
    let mut a = *block;
    let mut b = [0u8; 16];
    encrypt_pair(sched, &mut a, &mut b);
    a
}

/// Decrypt a single block with an already-built schedule.
#[inline]
pub(crate) fn decrypt_block_sched(sched: &CtSchedule, block: &[u8; 16]) -> [u8; 16] {
    let skey = sched.expand();
    let mut a = *block;
    let mut b = [0u8; 16];
    let mut q = load_pair(&a, &b);
    decrypt_bitsliced(sched.num_rounds, &skey, &mut q);
    store_pair(&mut q, &mut a, &mut b);
    a
}

/// XOR the AES-CTR keystream into `in_out`, starting at `counter`.
///
/// `counter` is advanced by one full 128-bit big-endian block per 16 bytes
/// consumed, so a caller can resume a stream. Two blocks are produced per
/// bitsliced pass, which is what makes the software path usable on 32-bit
/// CPUs.
pub(crate) fn xor_keystream_soft<const N: usize>(
    round_keys: &RoundKeysSoftware<N>,
    counter: &mut [u8; 16],
    in_out: &mut [u8],
) {
    let sched = keysched(round_keys);
    let skey = sched.expand();
    let n = in_out.len();
    let mut i = 0;

    while i + 32 <= n {
        let mut a = *counter;
        increment_block(counter);
        let mut b = *counter;
        increment_block(counter);
        {
            let mut q = load_pair(&a, &b);
            encrypt_bitsliced(sched.num_rounds, &skey, &mut q);
            store_pair(&mut q, &mut a, &mut b);
        }
        for k in 0..16 {
            in_out[i + k] ^= a[k];
        }
        for k in 0..16 {
            in_out[i + 16 + k] ^= b[k];
        }
        i += 32;
    }

    while i + 16 <= n {
        let mut a = *counter;
        increment_block(counter);
        let mut b = [0u8; 16];
        let mut q = load_pair(&a, &b);
        encrypt_bitsliced(sched.num_rounds, &skey, &mut q);
        store_pair(&mut q, &mut a, &mut b);
        for k in 0..16 {
            in_out[i + k] ^= a[k];
        }
        i += 16;
    }

    if i < n {
        let mut a = *counter;
        let mut b = [0u8; 16];
        let mut q = load_pair(&a, &b);
        encrypt_bitsliced(sched.num_rounds, &skey, &mut q);
        store_pair(&mut q, &mut a, &mut b);
        for k in 0..n - i {
            in_out[i + k] ^= a[k];
        }
    }
}

/// Increment a 16-byte big-endian counter block by one.
///
/// The counter is public data (derived from the nonce), so the early exit on
/// the first non-carrying byte is not secret-dependent.
#[inline]
fn increment_block(counter: &mut [u8; 16]) {
    for byte in counter.iter_mut().rev() {
        let (value, carry) = byte.overflowing_add(1);
        *byte = value;
        if !carry {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aes::aes::{SBOX, SBOX_INV, expand_key};

    fn bitsliced_sbox_byte(x: u8) -> u8 {
        let mut q = [0u32; 8];
        for (b, plane) in q.iter_mut().enumerate() {
            *plane = ((x >> b) & 1) as u32;
        }
        bitslice_sbox(&mut q);
        let mut y = 0u8;
        for (b, plane) in q.iter().enumerate() {
            y |= (*plane as u8 & 1) << b;
        }
        y
    }

    #[test]
    fn bitslice_sbox_matches_table() {
        for x in 0..=255u16 {
            assert_eq!(bitsliced_sbox_byte(x as u8), SBOX[x as usize], "S-box mismatch at {x:#04x}");
        }
    }

    #[test]
    fn bitslice_inv_sbox_matches_table() {
        for x in 0..=255u16 {
            let mut q = [0u32; 8];
            for (b, plane) in q.iter_mut().enumerate() {
                *plane = ((x as u8 >> b) & 1) as u32;
            }
            bitslice_inv_sbox(&mut q);
            let mut y = 0u8;
            for (b, plane) in q.iter().enumerate() {
                y |= (*plane as u8 & 1) << b;
            }
            assert_eq!(y, SBOX_INV[x as usize], "inverse S-box mismatch at {x:#04x}");
        }
    }

    #[test]
    fn sub_word_matches_table() {
        for b0 in 0..=255u16 {
            for b1 in 0..=255u16 {
                let w = u32::from_le_bytes([b0 as u8, b1 as u8, 0x5a, 0xa5]);
                let got = sub_word(w).to_le_bytes();
                let want = [SBOX[b0 as usize], SBOX[b1 as usize], SBOX[0x5a], SBOX[0xa5]];
                assert_eq!(got, want);
            }
        }
    }

    #[test]
    fn single_and_pair_encrypt_agree() {
        let key: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
        ];
        let rk = expand_key::<11>(&key);
        let sched = keysched(&rk);
        let a: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
        ];
        let b: [u8; 16] = [0xa5; 16];
        let expected = encrypt_block_sched(&sched, &a);

        let mut a2 = a;
        let mut b2 = b;
        encrypt_pair(&sched, &mut a2, &mut b2);
        assert_eq!(a2, expected);

        // The two lanes must be independent.
        let mut a3 = a;
        let mut b3 = [0u8; 16];
        encrypt_pair(&sched, &mut a3, &mut b3);
        assert_eq!(a3, expected);
    }
}
