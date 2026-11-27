//! Audit tests for `constant_time_eq`: length handling, every misalignment, every
//! differing bit position, SIMD block boundaries and a differential check against a
//! naive reference.
//!
//! Under Miri the crate selects the `generic` backend (`asm!` is disabled), so these
//! tests also exercise the pointer/`read_unaligned` logic there.

// The `check_n!` macro is also instantiated with `N == 0`, whose bit loop is
// intentionally empty.
#![allow(clippy::reversed_empty_ranges)]

use constant_time_eq::{
    constant_time_eq, constant_time_eq_16, constant_time_eq_32, constant_time_eq_64, constant_time_eq_n, generic,
};

const MASKS: [u8; 8] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];

/// 64-byte aligned scratch so that `&buf[off..]` has a controlled address residue.
#[repr(align(64))]
struct Aligned<const N: usize>([u8; N]);

fn reference(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x == y)
}

fn max_len() -> usize {
    if cfg!(miri) { 40 } else { 80 }
}

fn offsets() -> core::ops::Range<usize> {
    if cfg!(miri) { 0..4 } else { 0..8 }
}

/// Equal-length comparisons: identical inputs must compare equal, and flipping any
/// bit of either input must compare unequal, for every length and address residue.
#[test]
fn equal_lengths_every_bit_every_offset() {
    let mut abuf = Aligned([0x55u8; 160]);
    let mut bbuf = Aligned([0x55u8; 160]);

    for off in offsets() {
        for n in 0..=max_len() {
            abuf.0.fill(0x55);
            bbuf.0.fill(0x55);

            let a = &mut abuf.0[off..off + n];
            let b = &mut bbuf.0[off..off + n];

            assert!(constant_time_eq(a, b), "off={off} n={n} equal");
            assert!(generic::constant_time_eq(a, b), "off={off} n={n} generic equal");
            assert!(reference(a, b));

            for i in 0..n {
                for mask in MASKS {
                    a[i] ^= mask;
                    assert!(!constant_time_eq(a, b), "off={off} n={n} a[{i}] mask {mask:#04x}");
                    assert_eq!(
                        constant_time_eq(a, b),
                        generic::constant_time_eq(a, b),
                        "backend mismatch off={off} n={n} a[{i}] mask {mask:#04x}"
                    );
                    a[i] ^= mask;

                    b[i] ^= mask;
                    assert!(!constant_time_eq(a, b), "off={off} n={n} b[{i}] mask {mask:#04x}");
                    assert_eq!(
                        constant_time_eq(a, b),
                        generic::constant_time_eq(a, b),
                        "backend mismatch off={off} n={n} b[{i}] mask {mask:#04x}"
                    );
                    b[i] ^= mask;
                }
            }

            assert!(constant_time_eq(a, b), "off={off} n={n} restored");
        }
    }
}

/// Independent (different) address residues for the two operands.
#[test]
fn independent_misalignment() {
    let mut abuf = Aligned([0u8; 160]);
    let mut bbuf = Aligned([0u8; 160]);

    for n in 0..=48 {
        for ao in 0..8 {
            for bo in 0..8 {
                let a = &mut abuf.0[ao..ao + n];
                let b = &mut bbuf.0[bo..bo + n];
                a.fill(0xA5);
                b.fill(0xA5);
                assert!(constant_time_eq(a, b), "n={n} ao={ao} bo={bo}");
                if n > 0 {
                    // differ in the last byte only
                    b[n - 1] ^= 0x01;
                    assert!(!constant_time_eq(a, b), "n={n} ao={ao} bo={bo} last");
                    // differ in the first byte only
                    b[n - 1] ^= 0x01;
                    b[0] ^= 0x80;
                    assert!(!constant_time_eq(a, b), "n={n} ao={ao} bo={bo} first");
                }
            }
        }
    }
}

/// All unequal length pairs must return `false` and must not read out of bounds.
#[test]
fn unequal_lengths() {
    let abuf = Aligned([0x11u8; 160]);
    let bbuf = Aligned([0x11u8; 160]);

    for la in 0..=max_len() {
        for lb in 0..=max_len() {
            let a = &abuf.0[..la];
            let b = &bbuf.0[..lb];
            if la == lb {
                assert!(constant_time_eq(a, b), "la={la} lb={lb}");
            } else {
                assert!(!constant_time_eq(a, b), "la={la} lb={lb}");
            }
            assert_eq!(constant_time_eq(a, b), reference(a, b));
            assert_eq!(generic::constant_time_eq(a, b), reference(a, b));
        }
    }
}

/// Differences exactly at SIMD block boundaries (16/32/64) and the byte just after the
/// compared prefix (which must not influence the result).
#[test]
fn block_boundaries_and_tail() {
    let mut abuf = Aligned([0x33u8; 160]);
    let mut bbuf = Aligned([0x33u8; 160]);

    for &n in &[0usize, 1, 15, 16, 17, 31, 32, 33, 47, 48, 63, 64, 65, 79, 80] {
        for off in offsets() {
            abuf.0.fill(0x33);
            bbuf.0.fill(0x33);
            // The byte just past the compared region differs.
            if n < abuf.0.len() - off {
                bbuf.0[off + n] = 0xCC;
            }

            let a = &abuf.0[off..off + n];
            let b = &bbuf.0[off..off + n];
            assert!(constant_time_eq(a, b), "n={n} off={off} tail must be ignored");

            if n > 0 {
                bbuf.0[off + n - 1] ^= 0xFF;
                let b = &bbuf.0[off..off + n];
                assert!(!constant_time_eq(a, b), "n={n} off={off} last byte");
            }
        }
    }
}

/// Equal chunks of 16/32/64 bytes with a single differing byte at every position.
#[test]
fn exact_block_sizes() {
    for n in [16usize, 32, 64] {
        let mut abuf = Aligned([0u8; 160]);
        let mut bbuf = Aligned([0u8; 160]);
        for off in 0..8 {
            abuf.0.fill(0);
            bbuf.0.fill(0);
            let a = &abuf.0[off..off + n];
            {
                let b = &mut bbuf.0[off..off + n];
                assert!(constant_time_eq(a, b));
                for i in 0..n {
                    b[i] = 1;
                    assert!(!constant_time_eq(a, b), "n={n} off={off} i={i}");
                    b[i] = 0;
                }
            }
        }
    }
}

/// Const-generic array variants must agree with the slice variant at block edges.
#[test]
fn fixed_size_variants() {
    macro_rules! check_n {
        ($($n:literal),+) => {
            $({
                let mut a = [0x5Au8; $n];
                let mut b = [0x5Au8; $n];
                assert!(constant_time_eq_n(&a, &b), stringify!($n));
                for i in 0..$n {
                    b[i] ^= 0xFF;
                    assert!(!constant_time_eq_n(&a, &b), concat!("N=", stringify!($n), " i={}"), i);
                    assert_eq!(
                        constant_time_eq_n(&a, &b),
                        constant_time_eq(&a[..], &b[..]),
                    );
                    b[i] ^= 0xFF;
                    // also perturb `a`
                    a[i] ^= 0x01;
                    assert!(!constant_time_eq_n(&a, &b), concat!("N=", stringify!($n), " a i={}"), i);
                    a[i] ^= 0x01;
                }
            })+
        };
    }

    check_n!(0, 1, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65);

    assert!(constant_time_eq_16(&[3; 16], &[3; 16]));
    assert!(!constant_time_eq_16(&[3; 16], &[4; 16]));
    assert!(constant_time_eq_32(&[3; 32], &[3; 32]));
    assert!(!constant_time_eq_32(&[3; 32], &[4; 32]));
    assert!(constant_time_eq_64(&[3; 64], &[3; 64]));
    assert!(!constant_time_eq_64(&[3; 64], &[4; 64]));
}

/// Deterministic pseudo-random differential test against the reference, several lengths.
#[test]
fn random_differential() {
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };

    let mut abuf = Aligned([0u8; 192]);
    let mut bbuf = Aligned([0u8; 192]);

    for _ in 0..if cfg!(miri) { 100 } else { 2000 } {
        let n = (next() as usize) % 129;
        let off = (next() as usize) % 8;
        for i in 0..n {
            abuf.0[off + i] = next() as u8;
            bbuf.0[off + i] = if next() % 8 == 0 {
                abuf.0[off + i] ^ (next() as u8 | 1)
            } else {
                abuf.0[off + i]
            };
        }
        let a = &abuf.0[off..off + n];
        let b = &bbuf.0[off..off + n];
        let expected = reference(a, b);
        assert_eq!(constant_time_eq(a, b), expected, "n={n} off={off}");
        assert_eq!(generic::constant_time_eq(a, b), expected, "generic n={n} off={off}");
    }
}
