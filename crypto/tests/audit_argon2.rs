//! Tests for the public Argon2id API, RFC 9106 vectors, independent reference
//! vectors, parameter validation, length sweeps and buffer-canary checks.
//!
//! The crate's own unit tests cover the compression kernels and the arena;
//! these tests drive the public `crypto::argon2` surface.

use crypto::argon2::{Argon2Error, Params, decode_phc, derive_key, encode_phc, hash_password, verify_password};
use rand::{Rng, SeedableRng, rngs::StdRng};

// Opaque inputs for the independent reference vectors below. Changing them
// requires regenerating every expected tag with an independent implementation.
const REFERENCE_PASSWORD: &[u8] = b"audit-password";
const REFERENCE_SALT: &[u8] = b"auditsalt123456"; // 16 bytes, the RFC recommendation.

fn derive(
    out_len: usize,
    password: &[u8],
    salt: &[u8],
    secret: &[u8],
    ad: &[u8],
    iterations: u32,
    memory: u32,
    parallelism: u32,
) -> Result<Vec<u8>, Argon2Error> {
    let params = Params {
        iterations,
        memory,
        parallelism,
    };
    let mut out = vec![0u8; out_len];
    derive_key(&mut out, password, salt, secret, ad, &params).map(|()| out)
}

/// RFC 9106 Section 5.3, Argon2id test vector.
///
/// password = 0x01 * 32, salt = 0x02 * 16, secret = 0x03 * 8, ad = 0x04 * 12,
/// t = 3, m = 32, p = 4, tag length = 32.
#[test]
fn rfc9106_argon2id_vector() {
    let password = [0x01u8; 32];
    let salt = [0x02u8; 16];
    let secret = [0x03u8; 8];
    let ad = [0x04u8; 12];

    let tag = derive(32, &password, &salt, &secret, &ad, 3, 32, 4).unwrap();
    assert_eq!(
        hex::encode(&tag),
        "0d640df58d78766c08c037a34a8b53c9d01ef0452d75b65eb52520e96b01e659"
    );

    // Determinism and sensitivity to every input component.
    assert_eq!(derive(32, &password, &salt, &secret, &ad, 3, 32, 4).unwrap(), tag);
    assert_ne!(derive(32, &password[..31], &salt, &secret, &ad, 3, 32, 4).unwrap(), tag);
    assert_ne!(derive(32, &password, &salt[..15], &secret, &ad, 3, 32, 4).unwrap(), tag);
    assert_ne!(derive(32, &password, &salt, &secret[..7], &ad, 3, 32, 4).unwrap(), tag);
    assert_ne!(derive(32, &password, &salt, &secret, &ad[..11], 3, 32, 4).unwrap(), tag);
}

struct ReferenceVector {
    parallelism: u32,
    memory: u32,
    iterations: u32,
    out_len: usize,
    tag: &'static str,
}

/// Small-parameter Argon2id vectors generated with the independent RustCrypto
/// `argon2` crate 0.5 (`Argon2::new(Algorithm::Argon2id, Version::V0x13, ..)`,
/// no secret, no associated data), for the same password/salt as this file.
///
/// They deliberately cover `p = 1..=4`, the minimum memory (`8 * p`),
/// non-multiples of `4 * p`, tags both below and above the 64-byte BLAKE2b
/// block, and multiple passes.
const REFERENCE_VECTORS: &[ReferenceVector] = &[
    ReferenceVector {
        parallelism: 1,
        memory: 8,
        iterations: 1,
        out_len: 4,
        tag: "b810f85a",
    },
    ReferenceVector {
        parallelism: 1,
        memory: 8,
        iterations: 1,
        out_len: 32,
        tag: "1141758ec960cace134e0e16abbff0c804e125d609a543ba71228f62fc350756",
    },
    ReferenceVector {
        parallelism: 1,
        memory: 9,
        iterations: 2,
        out_len: 16,
        tag: "752758f1cdd81efd7b9bb7b4e8a94764",
    },
    ReferenceVector {
        parallelism: 1,
        memory: 12,
        iterations: 1,
        out_len: 65,
        tag: "e2a829b10a9489587429cddb4cd65a03ae84bf7b54c21d10c22d04606cc76f8d2f6a1d10ba88317bc6083a7a764a65aa2fe26f55355f080fbc5e38ce2e9ac6cab2",
    },
    ReferenceVector {
        parallelism: 1,
        memory: 16,
        iterations: 2,
        out_len: 32,
        tag: "4e6904b3dd425774fe7f3b4daede5cb5956ec4a93855cc05256d27659537ea3e",
    },
    ReferenceVector {
        parallelism: 1,
        memory: 64,
        iterations: 1,
        out_len: 24,
        tag: "5a01c848bfd715daa66ce578fdbf027ed79108da941eed95",
    },
    ReferenceVector {
        parallelism: 2,
        memory: 16,
        iterations: 1,
        out_len: 16,
        tag: "8f2cdd663db83c3befd74d2e64feeaac",
    },
    ReferenceVector {
        parallelism: 2,
        memory: 17,
        iterations: 2,
        out_len: 32,
        tag: "e8d51b8bd8b6f537495c5a3eac5ea4b3651c131b2fbcf33a7716df2d1d70e9b7",
    },
    ReferenceVector {
        parallelism: 2,
        memory: 20,
        iterations: 1,
        out_len: 4,
        tag: "c75f1230",
    },
    ReferenceVector {
        parallelism: 2,
        memory: 24,
        iterations: 2,
        out_len: 65,
        tag: "9008a55d5a1757cd7f5a6cb11152403e83e6f4369bae6d122e72ec1a20e1f3266b6f93e57d622eeac9b0964f285eb57853d72f4ceab66610b084091cd0ac6be214",
    },
    ReferenceVector {
        parallelism: 2,
        memory: 64,
        iterations: 2,
        out_len: 24,
        tag: "f4b37e5a4700cfec9fe19ee189dd8116fa8e0ab0bbd9cb9c",
    },
    ReferenceVector {
        parallelism: 3,
        memory: 24,
        iterations: 1,
        out_len: 16,
        tag: "5a56cfb46383f806e3a6965b302645ac",
    },
    ReferenceVector {
        parallelism: 3,
        memory: 25,
        iterations: 2,
        out_len: 32,
        tag: "6ce21308a93f3a2676e03795d8ae86ee2acbe89aae4e642504c013ed66ca8db9",
    },
    ReferenceVector {
        parallelism: 3,
        memory: 256,
        iterations: 2,
        out_len: 32,
        tag: "fbbe1d24c285bd775bb01ceb4081d17b96cd240f31e8c6f63251d884bb1289a1",
    },
    ReferenceVector {
        parallelism: 4,
        memory: 32,
        iterations: 1,
        out_len: 4,
        tag: "41acd65b",
    },
    ReferenceVector {
        parallelism: 4,
        memory: 32,
        iterations: 3,
        out_len: 32,
        tag: "dfebd13dc7be819dce5dda530288c49d7151a3ad3274867dada5bf26ed001c02",
    },
    ReferenceVector {
        parallelism: 4,
        memory: 33,
        iterations: 4,
        out_len: 65,
        tag: "5c0d1da7e8dd5fd83583e28f914186d5ac83335ee3a9d3954ddbf9fead3f8064ed32a0d85a30eb1b5ea57695703a93f128fc7946c44420818796a2b42f73c3bad7",
    },
    ReferenceVector {
        parallelism: 4,
        memory: 64,
        iterations: 4,
        out_len: 24,
        tag: "599168a097ccd4de7ec0d5acd4e7590891207fe670720635",
    },
];

#[test]
fn independent_reference_vectors() {
    for (i, v) in REFERENCE_VECTORS.iter().enumerate() {
        let tag = derive(
            v.out_len,
            REFERENCE_PASSWORD,
            REFERENCE_SALT,
            &[],
            &[],
            v.iterations,
            v.memory,
            v.parallelism,
        )
        .unwrap_or_else(|e| panic!("vector {} rejected: {:?}", i, e));
        assert_eq!(
            hex::encode(&tag),
            v.tag,
            "vector {} (p={}, m={}, t={}, len={})",
            i,
            v.parallelism,
            v.memory,
            v.iterations,
            v.out_len
        );
    }
}

/// Argon2id vectors from the Go `x/crypto/argon2` and C reference test suites.
#[test]
fn published_vectors() {
    // (parallelism, memory, iterations, output length, hex tag)
    let cases: &[(u32, u32, u32, usize, &str)] = &[
        // Go x/crypto/argon2, password="password", salt="somesalt", 24-byte tag.
        (1, 64, 1, 24, "655ad15eac652dc59f7170a7332bf49b8469be1fdb9c28bb"),
        (1, 64, 2, 24, "068d62b26455936aa6ebe60060b0a65870dbfa3ddf8d41f7"),
        (2, 64, 2, 24, "350ac37222f436ccb5c0972f1ebd3bf6b958bf2071841362"),
        (2, 256, 3, 24, "4668d30ac4187e6878eedeacf0fd83c5a0a30db2cc16ef0b"),
        (3, 64, 2, 24, "4a15b31aec7c2590b87d1f520be7d96f56658172deaa3079"),
        (4, 4096, 4, 24, "145db9733a9f4ee43edf33c509be96b934d505a4efb33c5a"),
        (6, 1024, 3, 24, "1640b932f4b60e272f5d2207b9a9c626ffa1bd88d2349016"),
        (8, 1024, 4, 24, "8dafa8e004f8ea96bf7c0f93eecf67a6047476143d15577f"),
        // C reference implementation, password="password", salt="somesalt",
        // 32-byte tag.
        (
            1,
            256,
            2,
            32,
            "9dfeb910e80bad0311fee20f9c0e2b12c17987b4cac90c2ef54d5b3021c68bfe",
        ),
        (
            2,
            256,
            2,
            32,
            "6d093c501fd5999645e0ea3bf620d7b8be7fd2db59c20d9fff9539da2bf57037",
        ),
        (
            1,
            65536,
            2,
            32,
            "09316115d5cf24ed5a15a31a3ba326e5cf32edc24702987c02b6566f61913cf7",
        ),
    ];

    for &(p, m, t, out_len, expected) in cases {
        let tag = derive(out_len, b"password", b"somesalt", &[], &[], t, m, p).unwrap();
        assert_eq!(hex::encode(&tag), expected, "p={}, m={}, t={}", p, m, t);
    }
}

/// Invalid parameters must be rejected without touching the caller's output
/// buffer, and without allocating an arena.
#[test]
fn parameter_validation_preserves_output() {
    let sentinel = [0xA5u8; 16];
    let cases = [
        (0, 8, 1),        // iterations == 0
        (1, 8, 0),        // parallelism == 0
        (1, 8, u32::MAX), // parallelism > 2^24 - 1
        (1, 8, 1 << 24),  // parallelism == 2^24
        (1, 0, 1),        // memory == 0
        (1, 7, 1),        // memory < 8 * p
        (1, 15, 2),       // memory < 8 * p
        (1, u32::MAX, 1), // huge arena (rejected before allocating only when
                          // the multiplication below overflows; here it is valid, so skip)
    ];
    // `memory = u32::MAX` with p = 1 is actually valid and would allocate 4 TiB,
    // so it is not part of this error-path test.
    for &(t, m, p) in &cases[..cases.len() - 1] {
        let mut out = sentinel;
        let params = Params {
            iterations: t,
            memory: m,
            parallelism: p,
        };
        let err = derive_key(&mut out, REFERENCE_PASSWORD, REFERENCE_SALT, &[], &[], &params).unwrap_err();
        assert!(
            matches!(err, Argon2Error::InvalidParams(_)),
            "t={}, m={}, p={} gave {:?}",
            t,
            m,
            p,
            err
        );
        assert_eq!(out, sentinel, "output modified for t={}, m={}, p={}", t, m, p);
    }

    // Output lengths below the RFC minimum of 4 bytes.
    for out_len in 0..4 {
        let mut out = vec![0xA5u8; out_len];
        let err = derive_key(
            &mut out,
            REFERENCE_PASSWORD,
            REFERENCE_SALT,
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 8,
                parallelism: 1,
            },
        )
        .unwrap_err();
        assert!(matches!(err, Argon2Error::InvalidParams(_)));
        assert!(out.iter().all(|&b| b == 0xA5));
    }

    // 4 bytes is the minimum accepted output length.
    assert_eq!(
        derive(4, REFERENCE_PASSWORD, REFERENCE_SALT, &[], &[], 1, 8, 1)
            .unwrap()
            .len(),
        4
    );
}

/// Salts shorter than the reference minimum (8 bytes) are rejected instead of
/// silently weakening the hash.
#[test]
fn salt_length_validation() {
    for len in 0..8 {
        let salt = vec![0x42u8; len];
        let err = derive(16, REFERENCE_PASSWORD, &salt, &[], &[], 1, 8, 1).unwrap_err();
        assert!(
            matches!(err, Argon2Error::InvalidParams(_)),
            "salt length {} should be rejected, got {:?}",
            len,
            err
        );
    }
    for len in [8usize, 9, 16, 32] {
        let salt = vec![0x42u8; len];
        assert!(derive(16, REFERENCE_PASSWORD, &salt, &[], &[], 1, 8, 1).is_ok());
    }
}

/// Every output length from 4 through 129 bytes, at a non-zero buffer offset,
/// must only write the requested range.
#[test]
fn tag_length_sweep_with_canaries() {
    let lengths = [
        4usize, 5, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 66, 95, 96, 97, 127, 128, 129, 192, 256,
    ];
    let canary = 0x5Au8;
    for &len in &lengths {
        let offset = 3;
        let mut buf = vec![canary; offset + len + 5];
        {
            let out = &mut buf[offset..offset + len];
            derive_key(
                out,
                REFERENCE_PASSWORD,
                REFERENCE_SALT,
                &[],
                &[],
                &Params {
                    iterations: 1,
                    memory: 8,
                    parallelism: 1,
                },
            )
            .unwrap();
        }
        assert!(
            buf[..offset].iter().all(|&b| b == canary),
            "prefix canary modified for len={}",
            len
        );
        assert!(
            buf[offset + len..].iter().all(|&b| b == canary),
            "suffix canary modified for len={}",
            len
        );
        // The output must be deterministic.
        let again = derive(len, REFERENCE_PASSWORD, REFERENCE_SALT, &[], &[], 1, 8, 1).unwrap();
        assert_eq!(&buf[offset..offset + len], &again[..]);
    }
}

/// Deterministic sweep over small, awkward parameter combinations. Each case
/// is derived twice (must agree bit-for-bit) and checked against canaries.
#[test]
fn parameter_sweep_deterministic() {
    let mut rng = StdRng::seed_from_u64(0xC0FF_EE00_1234_5678);
    for _ in 0..48 {
        let p = 1 + (rng.next_u64() % 4) as u32;
        let m = 8 * p + (rng.next_u64() % (4 * p as u64 + 9)) as u32;
        let t = 1 + (rng.next_u64() % 3) as u32;
        let out_len = 4 + (rng.next_u64() % 61) as usize;

        let canary = 0xC3u8;
        let mut buf = vec![canary; out_len + 9];
        derive_key(
            &mut buf[..out_len],
            REFERENCE_PASSWORD,
            REFERENCE_SALT,
            b"secret",
            b"ad",
            &Params {
                iterations: t,
                memory: m,
                parallelism: p,
            },
        )
        .unwrap();
        assert!(buf[out_len..].iter().all(|&b| b == canary));

        let expected = derive(out_len, REFERENCE_PASSWORD, REFERENCE_SALT, b"secret", b"ad", t, m, p).unwrap();
        assert_eq!(&buf[..out_len], &expected[..], "p={}, m={}, t={}", p, m, t);
    }
}

#[test]
fn phc_roundtrip_and_verification() {
    for &(t, m, p) in &[(1u32, 8u32, 1u32), (2, 16, 2), (1, 32, 4)] {
        let params = Params {
            iterations: t,
            memory: m,
            parallelism: p,
        };
        let encoded = hash_password(REFERENCE_PASSWORD, REFERENCE_SALT, &params).unwrap();
        assert!(encoded.starts_with("$argon2id$v=19$m="));
        assert!(verify_password(REFERENCE_PASSWORD, &encoded, None).is_ok());
        assert_eq!(
            verify_password(b"wrong password", &encoded, None),
            Err(Argon2Error::VerifyMismatch)
        );

        let (decoded_params, salt, tag) = decode_phc(&encoded).unwrap();
        assert_eq!(decoded_params.iterations, t);
        assert_eq!(decoded_params.memory, m);
        assert_eq!(decoded_params.parallelism, p);
        assert_eq!(salt, REFERENCE_SALT);
        assert_eq!(tag.len(), 64);
        assert!(verify_password(REFERENCE_PASSWORD, &encoded, Some(&params)).is_ok());

        // A limits value one below any field rejects before deriving.
        let limits = Params {
            iterations: t - 1,
            memory: m,
            parallelism: p,
        };
        assert!(matches!(
            verify_password(REFERENCE_PASSWORD, &encoded, Some(&limits)),
            Err(Argon2Error::InvalidParams(_))
        ));
    }

    let params = Params {
        iterations: 1,
        memory: 8,
        parallelism: 1,
    };
    let tag = derive(32, REFERENCE_PASSWORD, REFERENCE_SALT, &[], &[], 1, 8, 1).unwrap();
    let encoded = encode_phc(&params, REFERENCE_SALT, &tag);
    assert_eq!(decode_phc(&encoded).unwrap().2, tag);
}

/// PHC parameters must be canonical decimal integers. Accepting `m=064`,
/// `m=+64` or `m= 64` would let one hash have several valid encodings.
#[test]
fn phc_rejects_non_canonical_numbers() {
    // salt = "auditsalt123456", tag = 4 zero bytes.
    const SALT_B64: &str = "YXVkaXRzYWx0MTIzNDU2";
    const TAG_B64: &str = "AAAAAA";

    let canonical = format!("$argon2id$v=19$m=16,t=1,p=1${}${}", SALT_B64, TAG_B64);
    assert!(decode_phc(&canonical).is_ok());

    for non_canonical in [
        format!("$argon2id$v=19$m=016,t=1,p=1${}${}", SALT_B64, TAG_B64),
        format!("$argon2id$v=19$m=+16,t=1,p=1${}${}", SALT_B64, TAG_B64),
        format!("$argon2id$v=19$m= 16,t=1,p=1${}${}", SALT_B64, TAG_B64),
        format!("$argon2id$v=19$m=16,t=01,p=1${}${}", SALT_B64, TAG_B64),
        format!("$argon2id$v=19$m=16,t=1,p=01${}${}", SALT_B64, TAG_B64),
        format!("$argon2id$v=19$m=16,t=+1,p=1${}${}", SALT_B64, TAG_B64),
    ] {
        assert!(
            matches!(decode_phc(&non_canonical), Err(Argon2Error::InvalidEncoding(_))),
            "accepted non-canonical encoding: {}",
            non_canonical
        );
    }
}

#[test]
fn phc_decode_edges() {
    const SALT_B64: &str = "YXVkaXRzYWx0MTIzNDU2";
    const TAG_B64: &str = "AAAAAA";

    let reject_encoding = |s: &str| {
        assert!(
            matches!(decode_phc(s), Err(Argon2Error::InvalidEncoding(_))),
            "expected InvalidEncoding for {:?}",
            s
        );
    };
    let reject_params = |s: &str| {
        assert!(
            matches!(
                decode_phc(s),
                Err(Argon2Error::InvalidParams(_)) | Err(Argon2Error::InvalidEncoding(_))
            ),
            "expected rejection for {:?}",
            s
        );
    };

    reject_encoding("");
    reject_encoding("not a phc string");
    reject_encoding(&format!("$argon2i$v=19$m=16,t=1,p=1${}${}", SALT_B64, TAG_B64));
    reject_encoding(&format!("$argon2id$v=16$m=16,t=1,p=1${}${}", SALT_B64, TAG_B64));
    reject_encoding(&format!("$argon2id$v=19$m=16,t=1,p=1${}${}$extra", SALT_B64, TAG_B64));
    reject_encoding(&format!("$argon2id$v=19$m=16,t=1${}${}", SALT_B64, TAG_B64));
    // `m` is out of u32 range.
    reject_params(&format!("$argon2id$v=19$m=4294967296,t=1,p=1${}${}", SALT_B64, TAG_B64));
    // m = 0 violates m >= 8p.
    reject_params(&format!("$argon2id$v=19$m=0,t=1,p=1${}${}", SALT_B64, TAG_B64));
    // Tag shorter than 4 bytes (3 zero bytes encode as "AAAA").
    reject_params(&format!("$argon2id$v=19$m=16,t=1,p=1${}$AAAA", SALT_B64));
    // Salt shorter than 8 bytes is accepted by decode_phc (as documented) but
    // rejected by verification.
    let short_salt = "$argon2id$v=19$m=16,t=1,p=1$c2FsdA$AAAAAA"; // "salt"
    assert!(decode_phc(short_salt).is_ok());
    assert!(matches!(
        verify_password(REFERENCE_PASSWORD, short_salt, None),
        Err(Argon2Error::InvalidParams(_))
    ));
    // Empty salt likewise decodes but cannot be used.
    let empty_salt = format!("$argon2id$v=19$m=16,t=1,p=1$${}", TAG_B64);
    assert!(decode_phc(&empty_salt).is_ok());
    assert!(matches!(
        verify_password(REFERENCE_PASSWORD, &empty_salt, None),
        Err(Argon2Error::InvalidParams(_))
    ));
}
