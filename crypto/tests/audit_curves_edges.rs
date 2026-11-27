//! Adversarial edge-case tests for Curve25519, Ed25519, X25519 and the four
//! NIST prime curves.
//!
//! The crate already exercises RFC/Wycheproof/CAVP vectors. This file focuses
//! on the failure boundaries that those vectors do not enumerate:
//!
//! * X25519 low-order and non-canonical public keys, all-zero shared secrets;
//! * Ed25519 identity/small-order keys, non-canonical `R`/`A`, `S >= L`, and
//!   high-`s` ECDSA analogues;
//! * P-224/256/384/521 public-key length and point validation, and ECDSA
//!   `r`/`s` boundary values (0, n, n-1, all-0xFF);
//! * RSA key parsing and signature length boundaries (verification only).

use crypto::{
    EllipticCurveError, RsaError,
    curve25519::{ed25519, x25519},
    p224, p256, p384, p521,
    rsa::{DIGEST_INFO_SHA256_PREFIX, PublicKey as RsaPublicKey},
};

/// Canonical u-coordinates of the five low-order Curve25519 points (u = 0, 1,
/// p-1 and the two order-8 points). X25519 with a clamped scalar maps each to
/// the point at infinity, encoded as u = 0.
const LOW_ORDER_U: [[u8; 32]; 5] = [
    [0u8; 32],
    {
        let mut u = [0u8; 32];
        u[0] = 1;
        u
    },
    [
        0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
    ],
    [
        0xe0, 0xeb, 0x7a, 0x7c, 0x3b, 0x41, 0xb8, 0xae, 0x16, 0x56, 0xe3, 0xfa, 0xf1, 0x9f, 0xc4, 0x6a, 0xda, 0x09,
        0x8d, 0xeb, 0x9c, 0x32, 0xb1, 0xfd, 0x86, 0x62, 0x05, 0x16, 0x5f, 0x49, 0xb8, 0x00,
    ],
    [
        0x5f, 0x9c, 0x95, 0xbc, 0xa3, 0x50, 0x8c, 0x24, 0xb1, 0xd0, 0xb1, 0x55, 0x9c, 0x83, 0xef, 0x5b, 0x04, 0x44,
        0x5c, 0xc4, 0x58, 0x1c, 0x8e, 0x86, 0xd8, 0x22, 0x4e, 0xdd, 0xd0, 0x9f, 0x11, 0x57,
    ],
];

/// All five canonical low-order points must be identified and must make ECDH
/// fail with `InvalidSharedSecret` rather than returning an all-zero key.
#[test]
fn x25519_low_order_points_are_rejected() {
    let alice = x25519::SecretKey::from_bytes(&[0x42u8; 32]);
    for u in LOW_ORDER_U {
        let peer = x25519::PublicKey::from_bytes(&u);
        assert!(peer.is_low_order(), "u={u:02x?} not detected as low order");
        assert_eq!(alice.ecdh(&peer), Err(EllipticCurveError::InvalidSharedSecret));
    }

    // The basepoint and a random honest key are not low order.
    let mut base = [0u8; 32];
    base[0] = 9;
    assert!(!x25519::PublicKey::from_bytes(&base).is_low_order());
    assert!(!alice.public_key().is_low_order());
}

/// Non-canonical X25519 encodings (`u + p`, when it still fits 255 bits) decode
/// to the same Montgomery u-coordinate and therefore have the same low-order
/// status. Values `>= 2^255` are masked by the RFC 7748 decoder.
#[test]
fn x25519_non_canonical_encodings() {
    let p: [u8; 32] = [
        0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
    ];

    // u = 1 + p = 2^255 + 0x...? Actually p+1 is 32 bytes with the top bit set,
    // so RFC 7748 masks it back to 1; both encodings must be low order and
    // canonicalise identically.
    let mut u_plus_p = [0u8; 32];
    u_plus_p.copy_from_slice(&p);
    // Add 1 to the little-endian integer p.
    let mut carry = 1u16;
    for byte in &mut u_plus_p {
        let sum = *byte as u16 + carry;
        *byte = sum as u8;
        carry = sum >> 8;
    }
    // p + 1 has the top bit set, so the decoder masks it to 1.
    assert_eq!(x25519::PublicKey::from_bytes(&u_plus_p).to_bytes(), {
        let mut one = [0u8; 32];
        one[0] = 1;
        one
    });
    assert!(x25519::PublicKey::from_bytes(&u_plus_p).is_low_order());

    // u = p decodes to 0.
    assert_eq!(x25519::PublicKey::from_bytes(&p).to_bytes(), [0u8; 32]);
    assert!(x25519::PublicKey::from_bytes(&p).is_low_order());
}

/// X25519 ECDH is symmetric, deterministic and changes when either key changes.
#[test]
fn x25519_ecdh_boundaries() {
    let a = x25519::SecretKey::from_bytes(&[0u8; 32]);
    let b = x25519::SecretKey::from_bytes(&[0xFFu8; 32]);
    let a_pub = a.public_key();
    let b_pub = b.public_key();

    let ab = a.ecdh(&b_pub).unwrap();
    let ba = b.ecdh(&a_pub).unwrap();
    assert_eq!(ab, ba);
    assert_eq!(ab, a.ecdh(&b_pub).unwrap());
    assert_ne!(ab, [0u8; 32]);

    // A self-ECDH with an all-zero scalar is still a well-defined non-zero u.
    assert_ne!(a.public_key().to_bytes(), [0u8; 32]);
}

/// Ed25519 identity and low-order public keys are not usable keys.
#[test]
fn ed25519_identity_and_low_order_keys_rejected() {
    // Identity, order-2 and the remaining canonical small-order encodings.
    let bad_keys: [[u8; 32]; 8] = [
        [0u8; 32],
        {
            let mut b = [0u8; 32];
            b[0] = 1;
            b
        },
        {
            let mut b = [0xFFu8; 32];
            b[0] = 0xec;
            b[31] = 0x7f;
            b
        },
        [
            0x26, 0xe8, 0x95, 0x8f, 0xc2, 0xb2, 0x27, 0xb0, 0x45, 0xc3, 0xf4, 0x89, 0xf2, 0xef, 0x98, 0xf0, 0xd5, 0xdf,
            0xac, 0x05, 0xd3, 0xc6, 0x33, 0x39, 0xb1, 0x38, 0x02, 0x88, 0x6d, 0x53, 0xfc, 0x05,
        ],
        [
            0x26, 0xe8, 0x95, 0x8f, 0xc2, 0xb2, 0x27, 0xb0, 0x45, 0xc3, 0xf4, 0x89, 0xf2, 0xef, 0x98, 0xf0, 0xd5, 0xdf,
            0xac, 0x05, 0xd3, 0xc6, 0x33, 0x39, 0xb1, 0x38, 0x02, 0x88, 0x6d, 0x53, 0xfc, 0x85,
        ],
        [
            0xc7, 0x17, 0x6a, 0x70, 0x3d, 0x4d, 0xd8, 0x4f, 0xba, 0x3c, 0x0b, 0x76, 0x0d, 0x10, 0x67, 0x0f, 0x2a, 0x20,
            0x53, 0xfa, 0x2c, 0x39, 0xcc, 0xc6, 0x4e, 0xc7, 0xfd, 0x77, 0x92, 0xac, 0x03, 0x7a,
        ],
        [
            0xc7, 0x17, 0x6a, 0x70, 0x3d, 0x4d, 0xd8, 0x4f, 0xba, 0x3c, 0x0b, 0x76, 0x0d, 0x10, 0x67, 0x0f, 0x2a, 0x20,
            0x53, 0xfa, 0x2c, 0x39, 0xcc, 0xc6, 0x4e, 0xc7, 0xfd, 0x77, 0x92, 0xac, 0x03, 0xfa,
        ],
        [0xFFu8; 32],
    ];

    for key in bad_keys {
        assert_eq!(
            ed25519::PublicKey::from_bytes(&key),
            Err(EllipticCurveError::InvalidKey),
            "Ed25519 accepted invalid public key {key:02x?}"
        );
    }
}

/// Sign/verify with a deterministic seed, and malformed signature boundaries:
/// all-zero, all-0xFF, `R = identity`, `S = L` and `S = 2^255-1`.
#[test]
fn ed25519_signature_boundaries() {
    let seed = [0x5Au8; 32];
    let key = ed25519::SecretKey::from_bytes(&seed);
    let pk = key.public_key();

    for len in [0usize, 1, 31, 32, 63, 64, 65, 127, 128, 129, 1024] {
        let msg: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(13)).collect();
        let sig = key.sign(&msg);
        pk.verify(&msg, &sig).unwrap();
        if len > 0 {
            let mut other = msg.clone();
            other[0] ^= 1;
            assert!(pk.verify(&other, &sig).is_err());
        }
    }

    let msg = b"boundary";
    let valid = key.sign(msg);
    assert!(pk.verify(msg, &valid).is_ok());

    for sig in [[0u8; 64], [0xFFu8; 64]] {
        assert!(pk.verify(msg, &sig).is_err());
    }

    // R = identity is a non-canonical/small-order R encoding; must be rejected.
    let mut identity_r = valid;
    identity_r[..32].fill(0);
    identity_r[0] = 1;
    assert!(pk.verify(msg, &identity_r).is_err());

    // S = L (the group order) is non-canonical and must be rejected.
    let l: [u8; 32] = [
        0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
    ];
    let mut s_eq_l = valid;
    s_eq_l[32..].copy_from_slice(&l);
    assert!(pk.verify(msg, &s_eq_l).is_err());

    let mut s_max = valid;
    s_max[32..].fill(0xFF);
    assert!(pk.verify(msg, &s_max).is_err());
}

fn p256_order() -> [u8; 32] {
    [
        0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xbc, 0xe6,
        0xfa, 0xad, 0xa7, 0x17, 0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2, 0xfc, 0x63, 0x25, 0x51,
    ]
}

fn p384_order() -> [u8; 48] {
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc7, 0x63, 0x4d, 0x81, 0xf4, 0x37, 0x2d, 0xdf, 0x58, 0x1a, 0x0d, 0xb2,
        0x48, 0xb0, 0xa7, 0x7a, 0xec, 0xec, 0x19, 0x6a, 0xcc, 0xc5, 0x29, 0x73,
    ]
}

fn p521_order() -> [u8; 66] {
    let mut n = [0u8; 66];
    n[0] = 0x01;
    n[1..].copy_from_slice(&[
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfa, 0x51, 0x86, 0x87,
        0x83, 0xbf, 0x2f, 0x96, 0x6b, 0x7f, 0xcc, 0x01, 0x48, 0xf7, 0x09, 0xa5, 0xd0, 0x3b, 0xb5, 0xc9, 0xb8, 0x89,
        0x9c, 0x47, 0xae, 0xbb, 0x6f, 0xb7, 0x1e, 0x91, 0x38, 0x64, 0x09,
    ]);
    n
}

fn p224_order() -> [u8; 28] {
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x16, 0xa2, 0xe0, 0xb8,
        0xf0, 0x3e, 0x13, 0xdd, 0x29, 0x45, 0x5c, 0x5c, 0x2a, 0x3d,
    ]
}

fn ecdsa_r_s_boundaries<const KEY: usize, const SIG: usize>(
    name: &str,
    secret: [u8; KEY],
    sign: impl Fn(&[u8; KEY], &[u8]) -> [u8; SIG],
    verify: impl Fn(&[u8], &[u8; SIG]) -> Result<(), EllipticCurveError>,
    verify_strict: impl Fn(&[u8], &[u8; SIG]) -> Result<(), EllipticCurveError>,
    order: [u8; KEY],
) {
    let msg = b"ecdsa boundaries";
    let sig = sign(&secret, msg);
    verify(msg, &sig).unwrap_or_else(|e| panic!("{name}: valid signature rejected: {e}"));
    verify_strict(msg, &sig).unwrap_or_else(|e| panic!("{name}: valid low-s signature rejected: {e}"));

    // High-s form of the same signature: accepted by `verify`, rejected by
    // `verify_strict`.
    let mut high_s = sig;
    let half = SIG / 2;
    let mut s = [0u8; KEY];
    s.copy_from_slice(&sig[half..]);
    let mut borrow = 0i16;
    let mut neg = [0u8; KEY];
    for i in (0..KEY).rev() {
        let d = order[i] as i16 - s[i] as i16 - borrow;
        borrow = if d < 0 { 1 } else { 0 };
        neg[i] = (d + if borrow == 1 { 256 } else { 0 }) as u8;
    }
    high_s[half..].copy_from_slice(&neg);
    verify(msg, &high_s).unwrap_or_else(|e| panic!("{name}: verify rejected high-s form: {e}"));
    assert!(
        verify_strict(msg, &high_s).is_err(),
        "{name}: verify_strict accepted high-s form"
    );

    // r or s in {0, n, all-0xFF} must be rejected.
    let mut cases: Vec<([u8; KEY], [u8; KEY])> = Vec::new();
    let all_ff = [0xFFu8; KEY];
    let zero = [0u8; KEY];
    let mut n = order;
    cases.push((n, sig[half..].try_into().unwrap()));
    cases.push((zero, sig[half..].try_into().unwrap()));
    cases.push((all_ff, sig[half..].try_into().unwrap()));
    let r: [u8; KEY] = sig[..half].try_into().unwrap();
    cases.push((r, zero));
    cases.push((r, n));
    cases.push((r, all_ff));
    // n-1 is a valid scalar but not necessarily the signature's r/s.
    n[KEY - 1] -= 1;
    cases.push((n, n));

    for (r, s) in cases {
        let mut bad = [0u8; SIG];
        bad[..half].copy_from_slice(&r);
        bad[half..].copy_from_slice(&s);
        assert!(verify(msg, &bad).is_err(), "{name}: accepted invalid r/s {r:02x?}/{s:02x?}");
    }
}

/// P-256 ECDSA r/s boundaries and SEC1 public-key length sweep.
#[test]
fn p256_ecdsa_and_public_key_edges() {
    let secret = [0x11u8; 32];
    let key = p256::SecretKey::from_bytes(&secret).unwrap();
    let pk = key.public_key();
    let pk_bytes = pk.to_bytes();
    let compressed = pk.to_compressed_bytes();

    ecdsa_r_s_boundaries::<32, 64>(
        "p256",
        secret,
        |k, m| p256::SecretKey::from_bytes(k).unwrap().sign(m).unwrap(),
        |m, s| pk.verify(m, s),
        |m, s| pk.verify_strict(m, s),
        p256_order(),
    );

    // Only the exact compressed and uncompressed lengths may parse.
    for len in 0..=pk_bytes.len() + 1 {
        let bytes = vec![0x04u8; len];
        let parsed = p256::PublicKey::from_bytes(&bytes);
        if len == pk_bytes.len() || len == compressed.len() {
            // The all-0x04 buffer is not a valid point, so parsing still fails;
            // just require no panic and a clean Err.
            assert!(parsed.is_err());
        } else {
            assert_eq!(parsed, Err(EllipticCurveError::InvalidKey), "length {len}");
        }
    }
    assert_eq!(pk.to_bytes(), pk_bytes);
    p256::PublicKey::from_bytes(&pk_bytes).unwrap();
    p256::PublicKey::from_bytes(&compressed).unwrap();

    // Degenerate points.
    assert!(p256::PublicKey::from_bytes(&[0u8; 65]).is_err());
    assert!(p256::PublicKey::from_bytes(&[0xFFu8; 65]).is_err());
    assert!(p256::PublicKey::from_x_y(&[0u8; 32], &[0u8; 32]).is_err());
    assert!(p256::PublicKey::from_x_y(&[0xFFu8; 32], &[0xFFu8; 32]).is_err());
}

/// P-384 ECDSA r/s boundaries and public-key validation.
#[test]
fn p384_ecdsa_and_public_key_edges() {
    let secret = [0x22u8; 48];
    let key = p384::SecretKey::from_bytes(&secret).unwrap();
    let pk = key.public_key();

    ecdsa_r_s_boundaries::<48, 96>(
        "p384",
        secret,
        |k, m| p384::SecretKey::from_bytes(k).unwrap().sign(m).unwrap(),
        |m, s| pk.verify(m, s),
        |m, s| pk.verify_strict(m, s),
        p384_order(),
    );

    assert!(p384::PublicKey::from_bytes(&[0u8; 97]).is_err());
    assert!(p384::PublicKey::from_bytes(&[0xFFu8; 97]).is_err());
    assert!(p384::PublicKey::from_x_y(&[0u8; 48], &[0u8; 48]).is_err());
}

/// P-521 ECDSA r/s boundaries, including the non-byte-aligned 521-bit order.
#[test]
fn p521_ecdsa_and_public_key_edges() {
    let mut secret = [0x33u8; 66];
    secret[0] = 0x00;
    let key = p521::SecretKey::from_bytes(&secret).unwrap();
    let pk = key.public_key();

    ecdsa_r_s_boundaries::<66, 132>(
        "p521",
        secret,
        |k, m| p521::SecretKey::from_bytes(k).unwrap().sign(m).unwrap(),
        |m, s| pk.verify(m, s),
        |m, s| pk.verify_strict(m, s),
        p521_order(),
    );

    // The top seven bits of a 66-byte field element must be zero.
    let mut overlong = [0xFFu8; 66];
    overlong[0] = 0x02;
    assert!(p521::PublicKey::from_bytes(&overlong).is_err());
    assert!(p521::PublicKey::from_bytes(&[0xFFu8; 133]).is_err());
}

/// P-224 ECDSA r/s boundaries and public-key validation.
#[test]
fn p224_ecdsa_and_public_key_edges() {
    let secret = [0x44u8; 28];
    let key = p224::SecretKey::from_bytes(&secret).unwrap();
    let pk = key.public_key();

    ecdsa_r_s_boundaries::<28, 56>(
        "p224",
        secret,
        |k, m| p224::SecretKey::from_bytes(k).unwrap().sign(m).unwrap(),
        |m, s| pk.verify(m, s),
        |m, s| pk.verify_strict(m, s),
        p224_order(),
    );

    assert!(p224::PublicKey::from_bytes(&[0u8; 57]).is_err());
    assert!(p224::PublicKey::from_bytes(&[0xFFu8; 57]).is_err());
}

/// Secret-key scalars 0 and >= n are rejected for every curve, and the `[1; N]`
/// pattern is a valid scalar.
#[test]
fn p_curve_secret_key_ranges() {
    assert!(p256::SecretKey::from_bytes(&[0u8; 32]).is_err());
    assert!(p256::SecretKey::from_bytes(&p256_order()).is_err());
    assert!(p256::SecretKey::from_bytes(&[0xFFu8; 32]).is_err());
    assert!(p256::SecretKey::from_bytes(&[1u8; 32]).is_ok());

    assert!(p384::SecretKey::from_bytes(&[0u8; 48]).is_err());
    assert!(p384::SecretKey::from_bytes(&p384_order()).is_err());
    assert!(p384::SecretKey::from_bytes(&[0xFFu8; 48]).is_err());
    assert!(p384::SecretKey::from_bytes(&[1u8; 48]).is_ok());

    assert!(p521::SecretKey::from_bytes(&[0u8; 66]).is_err());
    assert!(p521::SecretKey::from_bytes(&p521_order()).is_err());
    assert!(p521::SecretKey::from_bytes(&[0xFFu8; 66]).is_err());
    assert!(p521::SecretKey::from_bytes(&[1u8; 66]).is_ok());

    assert!(p224::SecretKey::from_bytes(&[0u8; 28]).is_err());
    assert!(p224::SecretKey::from_bytes(&p224_order()).is_err());
    assert!(p224::SecretKey::from_bytes(&[0xFFu8; 28]).is_err());
    assert!(p224::SecretKey::from_bytes(&[1u8; 28]).is_ok());
}

/// RSA verification-only edges: malformed DER, modulus/exponent boundaries,
/// signature length, all-zero/all-0xFF signatures and PSS salt-length overflow.
#[test]
fn rsa_verification_edges() {
    // A valid 2048-bit modulus/exponent pair (from the crate's own test).
    let n = hex::decode(
        "b1d59f746650c6a4360d26dc2e05581e1bd12cddcfc459a75dd2ef6d38cb6e977c72cad72f5e8ad4795484211e71e9a292d25a3901fca4cd242649f56cce50ad6ba148658d71f3a9c8b39e92a7a49543243df8ca2688292d47ff2a92a6ee0c9151162936791f522afccd6a7508251934b909d62fa805bae0d79f83f3c981b39c15ea79ce7b4ec2ff82240ce2a9fb93ae49d7697d1248f73d4ad23461055f469a3936ab959a0c6a067aa19521650f3649a028e2ebe355909aae7c95d3fc988684478b2bb11b307cb58c6c14727e1b62103d400ac8eed0e0d6d7f7d7cfc1f4ae4cbd9759372f8408c52174abb05f134ca6788fb60ba3f35c57c07cd44011bb113b",
    )
    .unwrap();
    let e = [0x01, 0x00, 0x01];
    let key = RsaPublicKey::from_n_e(&n, &e).unwrap();
    assert_eq!(key.modulus_len_bytes(), 256);

    // Signature lengths 0..=258 must never panic; only 256 reaches modpow and
    // the padding check.
    for len in 0..=258 {
        let sig = vec![0x00u8; len];
        assert_eq!(key.verify_pkcs1_sha256(&sig, b"message"), Err(RsaError::Unspecified));
        let sig = vec![0xFFu8; len];
        assert!(key.verify_pkcs1_sha256(&sig, b"message").is_err());
    }

    // All-zero and all-0xFF signatures of the exact modulus length.
    assert!(key.verify_pkcs1_sha256(&[0u8; 256], b"message").is_err());
    assert!(key.verify_pss_sha256(&[0u8; 256], b"message").is_err());
    assert!(key.verify_pss_sha256(&[0xFFu8; 256], b"message").is_err());

    // Caller-controlled salt lengths, including values that would overflow a
    // naive `em_len - hash_len - salt_len - 2` computation.
    for salt_len in [0usize, 1, 32, 33, 223, 224, 225, usize::MAX, usize::MAX - 32] {
        assert!(
            key.verify_pss::<crypto::sha2::Sha256>(&[0u8; 256], b"message", salt_len)
                .is_err()
        );
    }

    // Modulus/exponent boundaries.
    assert!(matches!(RsaPublicKey::from_n_e(&[], &e), Err(RsaError::InvalidKey)));
    assert!(matches!(RsaPublicKey::from_n_e(&n, &[]), Err(RsaError::InvalidKey)));
    assert!(matches!(RsaPublicKey::from_n_e(&n, &[0]), Err(RsaError::InvalidKey)));
    assert!(matches!(RsaPublicKey::from_n_e(&n, &[1]), Err(RsaError::InvalidKey)));
    assert!(matches!(RsaPublicKey::from_n_e(&n, &[2]), Err(RsaError::InvalidKey)));
    assert!(RsaPublicKey::from_n_e(&n, &[3]).is_ok());
    assert!(matches!(RsaPublicKey::from_n_e(&n, &[4]), Err(RsaError::InvalidKey)));
    assert!(matches!(RsaPublicKey::from_n_e(&n, &n), Err(RsaError::InvalidKey)));
    assert!(matches!(
        RsaPublicKey::from_n_e(&[0xFFu8; 1025], &e),
        Err(RsaError::NotSupported)
    ));

    // DER parsing of arbitrary/truncated input must return Err, never panic.
    for len in 0..=80 {
        let data = vec![0x30u8; len];
        let _ = RsaPublicKey::from_pkcs1_der(&data);
        let data = vec![0xFFu8; len];
        let _ = RsaPublicKey::from_pkcs1_der(&data);
    }
    assert!(RsaPublicKey::from_pkcs1_der(&[]).is_err());
    assert!(RsaPublicKey::from_pkcs1_der(&[0x30, 0x00]).is_err());
    assert!(RsaPublicKey::from_pkcs1_der(&[0x30, 0x80, 0x00]).is_err());

    // DigestInfo prefix is bound: a different prefix/digest must fail.
    let sig = [0u8; 256];
    assert!(
        key.verify_pkcs1_v1_5(&sig, &[0u8; 32], DIGEST_INFO_SHA256_PREFIX)
            .is_err()
    );
}
