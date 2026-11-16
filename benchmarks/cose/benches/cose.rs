//! Benchmarks for the COSE hot paths.

use std::hint::black_box;

use cose::{
    Algorithm, CoseKey, Header, HeaderRole, KeySetBuilder, Label, Mac0, Mac0Builder, Sign, Sign1, Sign1Builder,
    SignBuilder, SymmetricKey, VerifyOptions,
};
use criterion::{Criterion, criterion_group, criterion_main};
use crypto::{curve25519::ed25519, p256};

fn setup_sign1() -> (Vec<u8>, p256::PublicKey) {
    let secret = p256::SecretKey::from_bytes(&[7u8; 32]).unwrap();
    let public = secret.public_key();
    let protected = Header::builder()
        .algorithm(Algorithm::ESP256)
        .key_id(b"key-1")
        .build_protected()
        .unwrap();
    let token = Sign1Builder::new()
        .protected(protected)
        .payload(b"this is a small COSE payload")
        .sign(&secret)
        .unwrap();
    (token, public)
}

fn bench_sign1(c: &mut Criterion) {
    let (token, public) = setup_sign1();

    c.bench_function("sign1/parse", |b| {
        b.iter(|| black_box(Sign1::from_slice(black_box(&token)).unwrap()))
    });

    c.bench_function("sign1/parse+headers", |b| {
        b.iter(|| {
            let message = Sign1::from_slice(black_box(&token)).unwrap();
            let _ = message.headers().unwrap();
            black_box(message)
        })
    });

    c.bench_function("sign1/verify_with_scratch", |b| {
        let message = Sign1::from_slice(&token).unwrap();
        let mut scratch = [0u8; 256];
        b.iter(|| {
            message
                .verify_with_scratch(&public, black_box(b""), &VerifyOptions::default(), &mut scratch)
                .unwrap()
        })
    });

    c.bench_function("sign1/to_vec", |b| {
        let message = Sign1::from_slice(&token).unwrap();
        b.iter(|| black_box(message.to_vec()))
    });

    c.bench_function("sign1/sign_into", |b| {
        let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
        b.iter(|| {
            let mut protected_buf = [0u8; 64];
            let protected = Header::builder()
                .algorithm(Algorithm::Ed25519)
                .build_protected_into(&mut protected_buf)
                .unwrap();
            let mut message = [0u8; 256];
            Sign1Builder::new()
                .protected(protected)
                .payload(black_box(b"this is a small COSE payload"))
                .sign_into(&secret, &mut message)
                .unwrap()
        })
    });
}

fn bench_mac(c: &mut Criterion) {
    let secret = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret).unwrap();
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    let token = Mac0Builder::new()
        .protected(protected)
        .payload(b"this is a small COSE payload")
        .mac(&key)
        .unwrap();
    let message = Mac0::from_slice(&token).unwrap();
    let mut scratch = [0u8; 128];

    c.bench_function("mac0/parse", |b| {
        b.iter(|| black_box(Mac0::from_slice(black_box(&token)).unwrap()))
    });
    c.bench_function("mac0/verify_with_scratch", |b| {
        b.iter(|| {
            message
                .verify_with_scratch(&key, black_box(b""), &VerifyOptions::default(), &mut scratch)
                .unwrap()
        })
    });
}

fn bench_key(c: &mut Criterion) {
    let secret = p256::SecretKey::from_bytes(&[7u8; 32]).unwrap();
    let bytes = CoseKey::from(&secret.public_key()).to_vec();

    c.bench_function("cose_key/parse+accessors", |b| {
        b.iter(|| {
            let key = CoseKey::from_slice(black_box(&bytes)).unwrap();
            let _ = key.kty().unwrap();
            let _ = key.kid();
            let _ = key.alg();
            black_box(key)
        })
    });

    c.bench_function("cose_key/validate", |b| {
        let key = CoseKey::from_slice(&bytes).unwrap();
        b.iter(|| key.validate().unwrap())
    });
}

fn bench_keyset(c: &mut Criterion) {
    let secret = p256::SecretKey::from_bytes(&[7u8; 32]).unwrap();
    let protected = Header::builder()
        .algorithm(Algorithm::ESP256)
        .build_protected()
        .unwrap();
    let token = Sign1Builder::new()
        .protected(protected)
        .payload(b"this is a small COSE payload")
        .sign(&secret)
        .unwrap();
    let message = Sign1::from_slice(&token).unwrap();

    let mut builder = KeySetBuilder::new();
    for i in 0..16u8 {
        let key = p256::SecretKey::from_bytes(&[i + 1; 32]).unwrap();
        builder = builder.push(CoseKey::from(&key.public_key()));
    }
    let key_set = builder.build().unwrap();
    let options = VerifyOptions {
        max_key_candidates: Some(16),
        ..Default::default()
    };
    let mut scratch = [0u8; 512];

    c.bench_function("keyset/verify/16 (no match)", |b| {
        b.iter(|| {
            let _ = black_box(message.verify_with_key_set_scratch(
                black_box(&key_set),
                black_box(b""),
                &options,
                &mut scratch,
            ));
        })
    });
}

fn bench_sign_any(c: &mut Criterion) {
    let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
    let other = ed25519::SecretKey::from_bytes(&[8u8; 32]).public_key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();

    let mut builder = SignBuilder::new().payload(b"this is a small COSE payload");
    for _ in 0..8 {
        builder = builder
            .add_signature(&secret, protected.clone(), Header::empty_unprotected())
            .unwrap();
    }
    let token = builder.sign().unwrap();
    let message = Sign::from_slice(&token).unwrap();
    let mut scratch = [0u8; 512];

    c.bench_function("sign/verify_any/8 (no match)", |b| {
        b.iter(|| {
            let _ = message.verify_any_with_scratch(&other, black_box(b""), &VerifyOptions::default(), &mut scratch);
        })
    });
}

fn bench_header(c: &mut Criterion) {
    // 32-entry protected header with a 32-entry crit list.
    let mut bytes = vec![0xb8, 0x20, 0x01, 0x26, 0x02, 0x98, 0x1e];
    for label in 8..=37u8 {
        if label < 24 {
            bytes.push(label);
        } else {
            bytes.push(0x18);
            bytes.push(label);
        }
    }
    for label in 8..=37u8 {
        if label < 24 {
            bytes.push(label);
        } else {
            bytes.push(0x18);
            bytes.push(label);
        }
        bytes.push(0x00);
    }
    let understood: Vec<Label> = (8..=37).map(Label::Int).collect();

    c.bench_function("header/parse+validate/32", |b| {
        b.iter(|| {
            let header = Header::from_slice(black_box(&bytes)).unwrap();
            let unprotected = Header::from_slice(&[0xa0]).unwrap();
            let headers = cose::Headers::new(header, unprotected).unwrap();
            headers.validate_with(&understood).unwrap();
        })
    });

    c.bench_function("header/validate/32", |b| {
        let header = Header::from_slice(&bytes).unwrap();
        b.iter(|| header.validate(black_box(HeaderRole::Protected)).unwrap())
    });
}

criterion_group!(
    benches,
    bench_sign1,
    bench_mac,
    bench_key,
    bench_keyset,
    bench_sign_any,
    bench_header
);
criterion_main!(benches);
