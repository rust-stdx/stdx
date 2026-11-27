//! Additional Argon2 parameter, length and overflow-boundary tests.
//!
//! These complement `audit_argon2.rs`: the small-parameter grid below is
//! exhaustive rather than sampled, and the invalid-parameter cases target the
//! arithmetic guards (`8 * parallelism`, `m_prime = 4p * floor(m / 4p)`, the
//! `q = m_prime / p` minimum) and the secret/AD length boundaries.

use crypto::argon2::{Argon2Error, Params, derive_key};

const PASSWORD: &[u8] = b"audit-edge-password";
const SALT: &[u8] = b"audit-edge-salt-16";

fn derive(out_len: usize, secret: &[u8], ad: &[u8], p: u32, m: u32, t: u32) -> Result<Vec<u8>, Argon2Error> {
    let mut out = vec![0u8; out_len];
    derive_key(
        &mut out,
        PASSWORD,
        SALT,
        secret,
        ad,
        &Params {
            iterations: t,
            memory: m,
            parallelism: p,
        },
    )?;
    Ok(out)
}

/// Exhaustive `m = 8p..=64 KiB`, `t = 1..=3`, `p = 1..=4` grid. Each derivation
/// must succeed deterministically, write exactly the requested bytes, and
/// leave surrounding canaries untouched.
///
/// This covers every `m mod 4p` residue, including the minimum `m = 8p` where
/// the first slice of pass 0 contains no fill work, and the `q = m_prime / p`
/// values around `SYNC_POINTS` and the threaded-fill threshold.
#[test]
fn small_parameter_grid_is_exhaustive_and_deterministic() {
    const CANARY: u8 = 0xE7;
    for p in 1..=4u32 {
        for t in 1..=3u32 {
            for m in (8 * p)..=64u32 {
                let out_len = 16usize;
                let mut buf = vec![CANARY; out_len + 11];
                derive_key(
                    &mut buf[5..5 + out_len],
                    PASSWORD,
                    SALT,
                    &[],
                    &[],
                    &Params {
                        iterations: t,
                        memory: m,
                        parallelism: p,
                    },
                )
                .unwrap_or_else(|e| panic!("p={p} t={t} m={m} rejected: {e:?}"));
                assert!(buf[..5].iter().all(|&b| b == CANARY), "p={p} t={t} m={m}");
                assert!(buf[5 + out_len..].iter().all(|&b| b == CANARY), "p={p} t={t} m={m}");

                let again = derive(out_len, &[], &[], p, m, t).unwrap();
                assert_eq!(&buf[5..5 + out_len], &again[..], "p={p} t={t} m={m} nondeterministic");
            }
        }
    }
}

/// `m = 8p - 1` and smaller must be rejected before allocating a large arena,
/// for every supported `p`. Also exercise the `parallelism` upper bound and
/// the checked `8 * parallelism` multiplication at its extremes.
#[test]
fn invalid_parameter_boundaries_are_rejected_without_touching_output() {
    let mut p = 1u32;
    while p <= 4 {
        for m in 0..(8 * p) {
            let sentinel = [0xA5u8; 8];
            let mut out = sentinel;
            let err = derive_key(
                &mut out,
                PASSWORD,
                SALT,
                &[],
                &[],
                &Params {
                    iterations: 1,
                    memory: m,
                    parallelism: p,
                },
            )
            .unwrap_err();
            assert!(matches!(err, Argon2Error::InvalidParams(_)), "p={p} m={m}");
            assert_eq!(out, sentinel, "output modified for p={p} m={m}");
        }
        p += 1;
    }

    // `8 * parallelism` must not overflow: both values are just below/at the
    // parallelism cap and are rejected by the `memory < 8p` check, not by a
    // debug-build multiply overflow.
    for (p, m) in [
        (u32::MAX, 0),
        (1 << 24, 1),
        ((1 << 24) - 1, 8 * ((1 << 24) - 1) - 1),
        (0, 8),
    ] {
        let err = derive(8, &[], &[], p, m, 1).unwrap_err();
        assert!(matches!(err, Argon2Error::InvalidParams(_)), "p={p} m={m}");
    }

    // t == 0 and the output-length minimum.
    assert!(matches!(
        derive(32, &[], &[], 1, 8, 0).unwrap_err(),
        Argon2Error::InvalidParams(_)
    ));
    for out_len in 0..4 {
        let err = derive(out_len, &[], &[], 1, 8, 1).unwrap_err();
        assert!(matches!(err, Argon2Error::InvalidParams(_)), "out_len={out_len}");
    }

    // Salt minimum: 0..8 bytes are rejected, 8 is accepted. Every case must
    // leave the caller's output untouched on error.
    for salt_len in 0..8usize {
        let salt = vec![0x42u8; salt_len];
        let sentinel = [0x11u8; 8];
        let mut out = sentinel;
        let err = derive_key(
            &mut out,
            PASSWORD,
            &salt,
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 8,
                parallelism: 1,
            },
        )
        .unwrap_err();
        assert!(matches!(err, Argon2Error::InvalidParams(_)), "salt_len={salt_len}");
        assert_eq!(out, sentinel);
    }
    assert!(derive(8, &[], &[], 1, 8, 1).is_ok());
}

/// Secret and associated-data length boundaries: each length must be accepted,
/// must affect the tag, and must not change the output length.
#[test]
fn secret_and_ad_length_boundaries_change_the_tag() {
    let base = derive(32, &[], &[], 1, 8, 1).unwrap();

    for len in [0usize, 1, 7, 8, 31, 32, 33, 63, 64, 65, 255, 256, 1024] {
        let secret = vec![0x5Au8; len];
        let ad = vec![0xC3u8; len];

        let with_secret = derive(32, &secret, &[], 1, 8, 1).unwrap();
        let with_ad = derive(32, &[], &ad, 1, 8, 1).unwrap();
        let with_both = derive(32, &secret, &ad, 1, 8, 1).unwrap();

        assert_eq!(with_secret.len(), 32);
        assert_eq!(with_ad.len(), 32);
        assert_eq!(with_both.len(), 32);

        if len == 0 {
            assert_eq!(with_secret, base, "empty secret changed the tag");
            assert_eq!(with_ad, base, "empty AD changed the tag");
            assert_eq!(with_both, base, "empty secret+AD changed the tag");
        } else {
            assert_ne!(with_secret, base, "secret len={len} did not change the tag");
            assert_ne!(with_ad, base, "AD len={len} did not change the tag");
            assert_ne!(with_both, with_secret, "AD len={len} did not change the tag");
            assert_ne!(with_both, with_ad, "secret len={len} did not change the tag");
        }
    }
}
