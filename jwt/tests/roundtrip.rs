use crypto::{
    curve25519::ed25519,
    mldsa::{
        MlDsa44PublicKey, MlDsa44SecretKey, MlDsa65PublicKey, MlDsa65SecretKey, MlDsa87PublicKey, MlDsa87SecretKey,
    },
    p256, p384, p521,
};
use jwt::*;

fn claims() -> serde_json::Value {
    serde_json::json!({ "sub": "user123", "exp": 9999999999_u64, "nbf": 0_u64 })
}

fn assert_sign_verify<V: Verifier>(signing_key: &dyn Signer, verifying_key: &V, alg: Algorithm) {
    let header = Header {
        alg,
        ..Default::default()
    };
    let token = sign(signing_key, &header, &claims()).unwrap();
    let parsed_header = parse_header(&token).unwrap();
    assert_eq!(parsed_header.alg, alg);

    let verified: serde_json::Value =
        parse_and_verify(verifying_key, &parsed_header, &token, &VerifyOptions::default()).unwrap();
    assert_eq!(verified["sub"], "user123");
}

#[test]
fn ed25519_roundtrip() {
    let secret_key = ed25519::SecretKey::generate();
    let public_key = secret_key.public_key();
    assert_sign_verify(&secret_key, &public_key, Algorithm::EdDSA);
}

#[test]
fn p256_roundtrip() {
    let secret_key = p256::SecretKey::generate().unwrap();
    let public_key = secret_key.public_key();
    assert_sign_verify(&secret_key, &public_key, Algorithm::ES256);
}

#[test]
fn p521_roundtrip() {
    let secret_key = p521::SecretKey::generate().unwrap();
    let public_key = secret_key.public_key();
    assert_sign_verify(&secret_key, &public_key, Algorithm::ES512);
}

fn sign_es384(secret_key: &p384::SecretKey, header: &Header, claims: &serde_json::Value) -> String {
    let header_base64 = base64::encode(
        serde_json::to_string(header).unwrap().as_bytes(),
        base64::Alphabet::UrlNoPadding,
    );
    let claims_base64 = base64::encode(
        serde_json::to_string(claims).unwrap().as_bytes(),
        base64::Alphabet::UrlNoPadding,
    );
    let signing_input = format!("{header_base64}.{claims_base64}");
    let signature = secret_key.sign(signing_input.as_bytes()).unwrap();
    format!("{signing_input}.{}", base64::encode(signature, base64::Alphabet::UrlNoPadding))
}

#[test]
fn p384_public_verify() {
    let secret_key = p384::SecretKey::generate().unwrap();
    let public_key = secret_key.public_key();
    let header = Header {
        alg: Algorithm::ES384,
        ..Default::default()
    };
    let token = sign_es384(&secret_key, &header, &claims());

    let parsed_header = parse_header(&token).unwrap();
    assert_eq!(parsed_header.alg, Algorithm::ES384);

    let verified: serde_json::Value =
        parse_and_verify(&public_key, &parsed_header, &token, &VerifyOptions::default()).unwrap();
    assert_eq!(verified["sub"], "user123");
}

#[test]
fn p384_tampered_token_is_rejected() {
    let secret_key = p384::SecretKey::generate().unwrap();
    let public_key = secret_key.public_key();
    let header = Header {
        alg: Algorithm::ES384,
        ..Default::default()
    };
    let token = sign_es384(&secret_key, &header, &claims());

    let mut tampered = token.clone();
    let last = tampered.pop().unwrap();
    tampered.push(if last == 'A' { 'B' } else { 'A' });

    let parsed_header = parse_header(&tampered).unwrap();
    let result: Result<serde_json::Value, _> =
        parse_and_verify(&public_key, &parsed_header, &tampered, &VerifyOptions::default());
    assert!(result.is_err());
}

#[test]
fn blake3_roundtrip() {
    let key = [7u8; 32];
    let signing_key = SecretKey::new(Algorithm::BLAKE3, &key);
    assert_sign_verify(&signing_key, &signing_key, Algorithm::BLAKE3);
}

#[test]
fn hmac_sha256_roundtrip() {
    let key = [7u8; 32];
    let signing_key = SecretKey::new(Algorithm::HS256, &key);
    assert_sign_verify(&signing_key, &signing_key, Algorithm::HS256);
}

#[test]
fn hmac_sha384_roundtrip() {
    let key = [7u8; 32];
    let signing_key = SecretKey::new(Algorithm::HS384, &key);
    assert_sign_verify(&signing_key, &signing_key, Algorithm::HS384);
}

#[test]
fn hmac_sha512_roundtrip() {
    let key = [7u8; 32];
    let signing_key = SecretKey::new(Algorithm::HS512, &key);
    assert_sign_verify(&signing_key, &signing_key, Algorithm::HS512);
}

#[test]
fn hmac_short_key_is_rejected() {
    let key = [7u8; 8];
    let header = Header {
        alg: Algorithm::HS256,
        ..Default::default()
    };

    assert!(matches!(
        sign(&SecretKey::new(Algorithm::HS256, &key), &header, &claims()),
        Err(Error::InvalidKey)
    ));
}

#[test]
fn blake3_short_key_is_rejected() {
    let key = [7u8; 16];
    let header = Header {
        alg: Algorithm::BLAKE3,
        ..Default::default()
    };

    assert!(matches!(
        sign(&SecretKey::new(Algorithm::BLAKE3, &key), &header, &claims()),
        Err(Error::InvalidKey)
    ));
}

#[test]
fn non_mac_algorithm_is_rejected() {
    let key = [7u8; 32];
    let header = Header {
        alg: Algorithm::ES256,
        ..Default::default()
    };

    assert!(matches!(
        sign(&SecretKey::new(Algorithm::ES256, &key), &header, &claims()),
        Err(Error::InvalidKey)
    ));
}

#[test]
fn mldsa44_roundtrip() {
    let secret_key = MlDsa44SecretKey::new(&[1u8; 32]);
    let public_key = secret_key.public_key();
    assert_sign_verify(&secret_key, &public_key, Algorithm::MlDsa44);
}

#[test]
fn mldsa65_roundtrip() {
    let secret_key = MlDsa65SecretKey::new(&[2u8; 32]);
    let public_key = secret_key.public_key();
    assert_sign_verify(&secret_key, &public_key, Algorithm::MlDsa65);
}

#[test]
fn mldsa87_roundtrip() {
    let secret_key = MlDsa87SecretKey::new(&[3u8; 32]);
    let public_key = secret_key.public_key();
    assert_sign_verify(&secret_key, &public_key, Algorithm::MlDsa87);
}

#[test]
fn tampered_token_is_rejected() {
    let secret_key = ed25519::SecretKey::generate();
    let public_key = secret_key.public_key();
    let header = Header {
        alg: Algorithm::EdDSA,
        ..Default::default()
    };
    let token = sign(&secret_key, &header, &claims()).unwrap();

    let mut tampered = token.clone();
    let last = tampered.pop().unwrap();
    tampered.push(if last == 'A' { 'B' } else { 'A' });

    let parsed_header = parse_header(&tampered).unwrap();
    let result: Result<serde_json::Value, _> =
        parse_and_verify(&public_key, &parsed_header, &tampered, &VerifyOptions::default());
    assert!(result.is_err());
}

#[test]
fn jwk_roundtrip_ed25519() {
    let secret_key = ed25519::SecretKey::generate();

    let public_jwk = Jwk::from(&secret_key.public_key());
    let public_key = ed25519::PublicKey::try_from(&public_jwk).unwrap();
    assert_eq!(public_key.to_bytes(), secret_key.public_key().to_bytes());

    let secret_jwk = Jwk::from(&secret_key);
    let secret_key2 = ed25519::SecretKey::try_from(&secret_jwk).unwrap();
    assert_eq!(secret_key2.to_bytes(), secret_key.to_bytes());
}

#[test]
fn jwk_roundtrip_p256() {
    let secret_key = p256::SecretKey::generate().unwrap();

    let public_jwk = Jwk::from(&secret_key.public_key());
    let public_key = p256::PublicKey::try_from(&public_jwk).unwrap();
    assert_eq!(public_key.to_bytes(), secret_key.public_key().to_bytes());

    let secret_jwk = Jwk::from(&secret_key);
    let secret_key2 = p256::SecretKey::try_from(&secret_jwk).unwrap();
    assert_eq!(secret_key2.to_bytes(), secret_key.to_bytes());
}

#[test]
fn jwk_roundtrip_p384_public() {
    let secret_key = p384::SecretKey::generate().unwrap();

    let public_jwk = Jwk::from(&secret_key.public_key());
    assert_eq!(public_jwk.algorithm, Algorithm::ES384);

    let json = serde_json::to_string(&public_jwk).unwrap();
    assert!(json.contains(r#""crv":"P-384""#), "{json}");

    let public_key = p384::PublicKey::try_from(&public_jwk).unwrap();
    assert_eq!(public_key.to_bytes(), secret_key.public_key().to_bytes());
}

#[test]
fn jwk_roundtrip_p521() {
    let secret_key = p521::SecretKey::generate().unwrap();

    let public_jwk = Jwk::from(&secret_key.public_key());
    assert_eq!(public_jwk.algorithm, Algorithm::ES512);

    let json = serde_json::to_string(&public_jwk).unwrap();
    assert!(json.contains(r#""crv":"P-521""#), "{json}");

    let public_key = p521::PublicKey::try_from(&public_jwk).unwrap();
    assert_eq!(public_key.to_bytes(), secret_key.public_key().to_bytes());

    let secret_jwk = Jwk::from(&secret_key);
    let secret_key2 = p521::SecretKey::try_from(&secret_jwk).unwrap();
    assert_eq!(secret_key2.to_bytes(), secret_key.to_bytes());
}

#[test]
fn jwk_roundtrip_oct() {
    let key = [7u8; 32];
    let message = b"message";

    for algorithm in [Algorithm::BLAKE3, Algorithm::HS256, Algorithm::HS384, Algorithm::HS512] {
        let jwk = Jwk::from(&SecretKey::new(algorithm, &key));
        assert_eq!(jwk.algorithm, algorithm);

        let signature = SecretKey::new(algorithm, &key).sign(message).unwrap();
        let recovered = SecretKey::try_from(&jwk).unwrap();
        assert!(recovered.verify(message, signature.as_ref()).is_ok(), "{algorithm}");
    }
}

#[test]
fn jwk_roundtrip_mldsa44() {
    let secret_key = MlDsa44SecretKey::new(&[1u8; 32]);

    let public_jwk = Jwk::from(&secret_key.public_key());
    let public_key = MlDsa44PublicKey::try_from(&public_jwk).unwrap();
    assert_eq!(public_key.to_bytes(), secret_key.public_key().to_bytes());

    let secret_jwk = Jwk::from(&secret_key);
    let secret_key2 = MlDsa44SecretKey::try_from(&secret_jwk).unwrap();
    assert_eq!(secret_key2.public_key().to_bytes(), secret_key.public_key().to_bytes());

    let json = serde_json::to_string(&secret_jwk).unwrap();
    assert!(json.contains(r#""kty":"AKP""#), "{json}");
    assert!(json.contains(r#""alg":"ML-DSA-44""#), "{json}");
    assert!(json.contains(r#""pub":"#), "{json}");
    assert!(json.contains(r#""priv":"#), "{json}");
}

#[test]
fn jwk_roundtrip_mldsa65() {
    let secret_key = MlDsa65SecretKey::new(&[2u8; 32]);

    let public_jwk = Jwk::from(&secret_key.public_key());
    let public_key = MlDsa65PublicKey::try_from(&public_jwk).unwrap();
    assert_eq!(public_key.to_bytes(), secret_key.public_key().to_bytes());

    let secret_jwk = Jwk::from(&secret_key);
    let secret_key2 = MlDsa65SecretKey::try_from(&secret_jwk).unwrap();
    assert_eq!(secret_key2.public_key().to_bytes(), secret_key.public_key().to_bytes());

    let json = serde_json::to_string(&secret_jwk).unwrap();
    assert!(json.contains(r#""kty":"AKP""#), "{json}");
    assert!(json.contains(r#""alg":"ML-DSA-65""#), "{json}");
}

#[test]
fn jwk_roundtrip_mldsa87() {
    let secret_key = MlDsa87SecretKey::new(&[3u8; 32]);

    let public_jwk = Jwk::from(&secret_key.public_key());
    let public_key = MlDsa87PublicKey::try_from(&public_jwk).unwrap();
    assert_eq!(public_key.to_bytes(), secret_key.public_key().to_bytes());

    let secret_jwk = Jwk::from(&secret_key);
    let secret_key2 = MlDsa87SecretKey::try_from(&secret_jwk).unwrap();
    assert_eq!(secret_key2.public_key().to_bytes(), secret_key.public_key().to_bytes());

    let json = serde_json::to_string(&secret_jwk).unwrap();
    assert!(json.contains(r#""kty":"AKP""#), "{json}");
    assert!(json.contains(r#""alg":"ML-DSA-87""#), "{json}");
    assert!(json.contains(r#""pub":"#), "{json}");
    assert!(json.contains(r#""priv":"#), "{json}");
}

#[test]
fn jwk_wrong_algorithm_is_rejected() {
    let secret_key = ed25519::SecretKey::generate();
    let jwk = Jwk::from(&secret_key.public_key());

    assert!(SecretKey::try_from(&jwk).is_err());
}

#[test]
fn algorithm_display_roundtrips() {
    for alg in [
        Algorithm::BLAKE3,
        Algorithm::HS256,
        Algorithm::HS384,
        Algorithm::HS512,
        Algorithm::EdDSA,
        Algorithm::ES256,
        Algorithm::ES384,
        Algorithm::ES512,
        Algorithm::MlDsa44,
        Algorithm::MlDsa65,
        Algorithm::MlDsa87,
        Algorithm::RS256,
        Algorithm::RS384,
        Algorithm::RS512,
        Algorithm::PS256,
        Algorithm::PS384,
        Algorithm::PS512,
    ] {
        assert_eq!(alg.to_string().parse::<Algorithm>().unwrap(), alg);
    }
}

fn assert_key_roundtrip(signing_key: &dyn Signer, jwk: &Jwk, alg: Algorithm) {
    let key = Key::try_from(jwk).unwrap();
    assert_eq!(Verifier::algorithm(&key), alg);
    assert_eq!(Signer::algorithm(&key), alg);

    let header = Header {
        alg,
        ..Default::default()
    };
    let token = sign(signing_key, &header, &claims()).unwrap();
    let parsed_header = parse_header(&token).unwrap();
    let verified: serde_json::Value =
        parse_and_verify(&key, &parsed_header, &token, &VerifyOptions::default()).unwrap();
    assert_eq!(verified["sub"], "user123");
}

#[test]
fn key_decodes_ed25519() {
    let secret_key = ed25519::SecretKey::generate();

    assert!(matches!(Key::try_from(&Jwk::from(&secret_key)).unwrap(), Key::Ed25519Secret(_)));
    assert!(matches!(
        Key::try_from(&Jwk::from(&secret_key.public_key())).unwrap(),
        Key::Ed25519Public(_)
    ));

    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key), Algorithm::EdDSA);
    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key.public_key()), Algorithm::EdDSA);
}

#[test]
fn key_decodes_p256() {
    let secret_key = p256::SecretKey::generate().unwrap();

    assert!(matches!(Key::try_from(&Jwk::from(&secret_key)).unwrap(), Key::P256Secret(_)));
    assert!(matches!(
        Key::try_from(&Jwk::from(&secret_key.public_key())).unwrap(),
        Key::P256Public(_)
    ));

    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key), Algorithm::ES256);
    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key.public_key()), Algorithm::ES256);
}

#[test]
fn key_decodes_p384() {
    let secret_key = p384::SecretKey::generate().unwrap();
    let jwk = Jwk::from(&secret_key.public_key());

    let key = Key::try_from(&jwk).unwrap();
    assert!(matches!(key, Key::P384Public(_)));
    assert_eq!(Verifier::algorithm(&key), Algorithm::ES384);
    assert!(!key.is_secret_key());

    let header = Header {
        alg: Algorithm::ES384,
        ..Default::default()
    };
    let token = sign_es384(&secret_key, &header, &claims());
    let parsed_header = parse_header(&token).unwrap();
    let verified: serde_json::Value =
        parse_and_verify(&key, &parsed_header, &token, &VerifyOptions::default()).unwrap();
    assert_eq!(verified["sub"], "user123");
}

#[test]
fn key_decodes_p521() {
    let secret_key = p521::SecretKey::generate().unwrap();

    assert!(matches!(Key::try_from(&Jwk::from(&secret_key)).unwrap(), Key::P521Secret(_)));
    assert!(matches!(
        Key::try_from(&Jwk::from(&secret_key.public_key())).unwrap(),
        Key::P521Public(_)
    ));

    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key), Algorithm::ES512);
    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key.public_key()), Algorithm::ES512);
}

#[test]
fn key_decodes_oct() {
    let key_bytes = [7u8; 32];

    for algorithm in [Algorithm::BLAKE3, Algorithm::HS256, Algorithm::HS384, Algorithm::HS512] {
        let secret_key = SecretKey::new(algorithm, &key_bytes);
        let jwk = Jwk::from(&secret_key);

        assert!(matches!(Key::try_from(&jwk).unwrap(), Key::Secret(_)));
        assert_key_roundtrip(&secret_key, &jwk, algorithm);
    }
}

#[test]
fn key_decodes_mldsa44() {
    let secret_key = MlDsa44SecretKey::new(&[1u8; 32]);

    assert!(matches!(Key::try_from(&Jwk::from(&secret_key)).unwrap(), Key::MlDsa44Secret(_)));
    assert!(matches!(
        Key::try_from(&Jwk::from(&secret_key.public_key())).unwrap(),
        Key::MlDsa44Public(_)
    ));

    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key), Algorithm::MlDsa44);
    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key.public_key()), Algorithm::MlDsa44);
}

#[test]
fn key_decodes_mldsa65() {
    let secret_key = MlDsa65SecretKey::new(&[2u8; 32]);

    assert!(matches!(Key::try_from(&Jwk::from(&secret_key)).unwrap(), Key::MlDsa65Secret(_)));
    assert!(matches!(
        Key::try_from(&Jwk::from(&secret_key.public_key())).unwrap(),
        Key::MlDsa65Public(_)
    ));

    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key), Algorithm::MlDsa65);
    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key.public_key()), Algorithm::MlDsa65);
}

#[test]
fn key_decodes_mldsa87() {
    let secret_key = MlDsa87SecretKey::new(&[3u8; 32]);

    assert!(matches!(Key::try_from(&Jwk::from(&secret_key)).unwrap(), Key::MlDsa87Secret(_)));
    assert!(matches!(
        Key::try_from(&Jwk::from(&secret_key.public_key())).unwrap(),
        Key::MlDsa87Public(_)
    ));

    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key), Algorithm::MlDsa87);
    assert_key_roundtrip(&secret_key, &Jwk::from(&secret_key.public_key()), Algorithm::MlDsa87);
}

#[test]
fn key_enum_size_is_bounded() {
    assert!(
        core::mem::size_of::<Key<'_>>() < 32 * 1024,
        "Key grew to {} bytes; box any large new variant",
        core::mem::size_of::<Key<'_>>()
    );
}

#[test]
fn key_secret_signs_and_verifies() {
    let secret_key = ed25519::SecretKey::generate();
    let jwk = Jwk::from(&secret_key);
    let key = Key::try_from(&jwk).unwrap();

    let header = Header {
        alg: Algorithm::EdDSA,
        ..Default::default()
    };
    let token = sign(&key, &header, &claims()).unwrap();
    let parsed_header = parse_header(&token).unwrap();
    let verified: serde_json::Value =
        parse_and_verify(&key, &parsed_header, &token, &VerifyOptions::default()).unwrap();
    assert_eq!(verified["sub"], "user123");
}

#[test]
fn key_public_only_cannot_sign() {
    let secret_key = ed25519::SecretKey::generate();
    let jwk = Jwk::from(&secret_key.public_key());
    let key = Key::try_from(&jwk).unwrap();

    let header = Header {
        alg: Algorithm::EdDSA,
        ..Default::default()
    };
    assert!(matches!(sign(&key, &header, &claims()), Err(Error::InvalidKey)));
}

#[test]
fn key_malformed_ec_key_is_rejected() {
    // A P-521 JWK whose coordinates are too short to be valid affine values
    // must be rejected instead of silently truncated.
    let jwk = Jwk {
        kid: Default::default(),
        r#use: KeyUse::Sign,
        algorithm: Algorithm::ES512,
        crypto: JwkCrypto::Ec {
            curve: EcCurve::P521,
            x: smallvec::SmallVec::from_slice_copy(&[0u8; 32]),
            y: smallvec::SmallVec::from_slice_copy(&[0u8; 32]),
            d: None,
        },
    };

    assert!(matches!(Key::try_from(&jwk), Err(Error::InvalidKey)));
}

#[test]
fn key_akp_with_wrong_algorithm_is_rejected() {
    let jwk = Jwk {
        kid: Default::default(),
        r#use: KeyUse::Sign,
        algorithm: Algorithm::ES256,
        crypto: JwkCrypto::Akp {
            pub_key: smallvec::SmallVec::from_slice_copy(&[0u8; 4]),
            private_key: None,
        },
    };

    assert!(matches!(Key::try_from(&jwk), Err(Error::InvalidKey)));
}
