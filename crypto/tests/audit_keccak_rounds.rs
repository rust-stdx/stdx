//! Edge tests for the public Keccak-p round-count generic.
//!
//! `crypto::sha3::keccak::p1600::<ROUNDS>` and `KeccakSponge::<ROUNDS>` accept
//! any `ROUNDS <= 24` at compile time. FIPS 202 defines Keccak-p[b, 0] as the
//! identity, so `ROUNDS = 0` must return the state unchanged on every backend.

use crypto::sha3::keccak::p1600;

/// Keccak-p[1600, 0] is the identity permutation (FIPS 202, Algorithm 5 with
/// `nr = 0`). This round count was previously untested: the portable, AVX2,
/// AVX-512 and Armv8.4-A SHA-3 kernels all slice the round constants at
/// `24 - ROUNDS`, so every detected backend must leave the state untouched.
/// Runs the identity check against whichever backend this target selects.
#[test]
fn p1600_zero_rounds_is_identity() {
    let mut state = [0u64; 25];
    for (i, lane) in state.iter_mut().enumerate() {
        *lane = (i as u64)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .rotate_left((i % 64) as u32);
    }
    let original = state;

    p1600::<0>(&mut state);

    assert_eq!(state, original, "Keccak-p[1600, 0] must be the identity");
}
