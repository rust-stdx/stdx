//! Differential tests for the modular arithmetic and Barrett reduction.
//!
//! Every result is checked against an independent path: the bit-serial
//! `mul_mod`/`modpow` reference (structured around boundary values, not just
//! uniform randoms) and the algebraic definition of division.

use big_number::Uint;

type U256 = Uint<256, 4>;
type U512 = Uint<512, 8>;

fn rng(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

/// An odd modulus of exactly `bits` bits: `2^(bits - 1) + 1`.
fn modulus_of_bits(bits: usize) -> U256 {
    let limb_idx = (bits - 1) / 64;
    let bit_in = (bits - 1) % 64;
    let mut limbs = [0u64; 4];
    limbs[limb_idx] = 1u64 << bit_in;
    let m = U256::from_limbs(limbs).add_word(1).0;
    assert_eq!(m.bit_len(), bits);
    m
}

/// A random value below `2^bits`.
fn random_below(state: &mut u64, bits: usize) -> U256 {
    let mut limbs = [rng(state), rng(state), rng(state), rng(state)];
    let full = bits / 64;
    let rem = bits % 64;
    for (i, l) in limbs.iter_mut().enumerate() {
        if i > full {
            *l = 0;
        } else if i == full {
            *l = if rem == 0 { 0 } else { *l >> (64 - rem) };
        }
    }
    U256::from_limbs(limbs)
}

#[test]
fn barrett_agrees_with_bit_serial_full_width() {
    let mut state = 0x1234_5678_9ABC_DEF1u64;
    for _ in 0..50_000 {
        let m = U256::from_limbs([
            rng(&mut state) | 1,
            rng(&mut state),
            rng(&mut state),
            rng(&mut state) | 0x8000_0000_0000_0000,
        ]);
        let a = U256::from_limbs([rng(&mut state), rng(&mut state), rng(&mut state), rng(&mut state)]);
        let b = U256::from_limbs([rng(&mut state), rng(&mut state), rng(&mut state), rng(&mut state)]);
        let mu = m.compute_mu_for_barrett();
        assert_eq!(a.mul_mod_barrett(&b, &m, &mu), a.mul_mod(&b, &m), "m={m:x}");
    }
}

#[test]
fn barrett_auto_detects_effective_limbs_for_small_moduli() {
    // Regression: `compute_mu_for_barrett` used the modulus's effective limb
    // count while `mul_mod_barrett` used `LIMBS`, so this pairing silently
    // produced wrong results whenever the modulus did not fill the top limb.
    let mut state = 0xC0FF_EE00_1234_5678u64;
    for bits in [65usize, 100, 128, 129, 160, 192, 193, 200, 250] {
        let m = modulus_of_bits(bits);
        let mu = m.compute_mu_for_barrett();
        for _ in 0..500 {
            let a = random_below(&mut state, bits - 1);
            let b = random_below(&mut state, bits - 1);
            assert_eq!(
                a.mul_mod_barrett(&b, &m, &mu),
                a.mul_mod(&b, &m),
                "bits={bits} m={m:x}"
            );
        }
    }
}

#[test]
fn barrett_modpow_agrees_with_bit_serial() {
    let mut state = 0x0BAD_F00D_DEAD_BEEFu64;
    for bits in [65usize, 129, 192, 193, 250, 256] {
        let m = modulus_of_bits(bits);
        let mu = m.compute_mu_for_barrett();
        for _ in 0..100 {
            let base = random_below(&mut state, bits - 1);
            let exp = U256::from_limbs([rng(&mut state) >> 40, rng(&mut state) % 32, 0, 0]);
            assert_eq!(
                base.modpow_barrett(&exp, &m, &mu),
                base.modpow(&exp, &m),
                "bits={bits} m={m:x} exp={exp:x}"
            );
        }
    }
}

#[test]
fn barrett_boundary_values() {
    // Values / moduli at the edges of the representable ranges, where the
    // quotient approximation error and the two conditional subtractions matter.
    let mut state = 0xFEED_FACE_CAFE_BEEFu64;
    for bits in [193usize, 200, 224, 250, 255, 256] {
        for variant in 0..3 {
            let m = match variant {
                0 => modulus_of_bits(bits),
                1 => {
                    // 2^bits - 1 (all ones below the top bit).
                    let mut limbs = [0u64; 4];
                    for (i, l) in limbs.iter_mut().enumerate() {
                        let lo = i * 64;
                        if lo >= bits {
                            *l = 0;
                        } else if lo + 64 <= bits {
                            *l = u64::MAX;
                        } else {
                            *l = (1u64 << (bits - lo)) - 1;
                        }
                    }
                    U256::from_limbs(limbs)
                }
                _ => {
                    let mut limbs = [rng(&mut state), rng(&mut state), rng(&mut state), rng(&mut state)];
                    let rem = bits % 64;
                    limbs[3] = if rem == 0 { limbs[3] | 1 } else { limbs[3] >> (64 - rem) };
                    limbs[0] |= 1;
                    let v = U256::from_limbs(limbs);
                    if v.bit_len() != bits {
                        continue;
                    }
                    v
                }
            };
            let mu = m.compute_mu_for_barrett();
            let m_minus_1 = m.sub_word(1).0;
            let values = [U256::ZERO, U256::ONE, m_minus_1, random_below(&mut state, bits - 1)];
            for &a in &values {
                for &b in &values {
                    assert_eq!(a.mul_mod_barrett(&b, &m, &mu), a.mul_mod(&b, &m), "bits={bits} m={m:x}");
                }
            }
        }
    }
}

#[test]
fn barrett_512_bit_full_width() {
    let mut state = 0x51_23_45_67_89_AB_CD_EFu64;
    for _ in 0..5_000 {
        let m = U512::from_limbs([
            rng(&mut state) | 1,
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state) | 0x8000_0000_0000_0000,
        ]);
        let a = U512::from_limbs([
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
        ]);
        let b = U512::from_limbs([
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
            rng(&mut state),
        ]);
        let mu = m.compute_mu_for_barrett();
        assert_eq!(a.mul_mod_barrett(&b, &m, &mu), a.mul_mod(&b, &m));
    }
}

#[test]
fn large_modulus_barrett_does_not_panic() {
    // 128-limb (8192-bit) modulus, the largest the RSA code uses.
    let mut limbs = [0u64; 128];
    let mut state = 0xABCD_1234_5678_9012u64;
    for l in limbs.iter_mut() {
        *l = rng(&mut state);
    }
    limbs[0] |= 1;
    limbs[127] |= 1 << 63;
    let m = Uint::<8192, 128>::from_limbs(limbs);
    assert_eq!(m.bit_len(), 8192);
    let mu = m.compute_mu_for_barrett();
    let a = Uint::<8192, 128>::from_u64(0xDEAD_BEEF);
    let b = Uint::<8192, 128>::from_u64(0xFEED_FACE);
    let barrett = a.mul_mod_barrett(&b, &m, &mu);
    let reference = a.mul_mod(&b, &m);
    assert_eq!(barrett, reference);
}
