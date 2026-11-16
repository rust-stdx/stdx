#![allow(deprecated)]

use std::hint::black_box;

use aws_lc_rs::{
    encoding::AsDer,
    signature::{
        self, ED25519, KeyPair, ML_DSA_44, ML_DSA_44_SIGNING, ML_DSA_65, ML_DSA_65_SIGNING, ML_DSA_87,
        ML_DSA_87_SIGNING, PqdsaKeyPair, UnparsedPublicKey,
    },
};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    curve25519::ed25519::SecretKey,
    mldsa::{MlDsa44SecretKey, MlDsa65SecretKey, MlDsa87SecretKey},
};
use ed25519_dalek::{Signer, SigningKey as EdSigningKey, Verifier};
use ml_dsa::{
    KeyInit as MlKeyInit, Keypair as MlKeypair, MlDsa44, MlDsa65, MlDsa87, Signer as MlSigner,
    SigningKey as MlSigningKey, Verifier as MlVerifier,
};

const DATA_SIZES: &[usize] = &[64, 1024, 64 * 1024, 1024 * 1024];

const SEED: [u8; 32] = [0x42u8; 32];

fn bench_ed25519(c: &mut Criterion) {
    let std_sk = SecretKey::generate().unwrap();
    let std_pk = std_sk.public_key();

    let aws_keypair = signature::Ed25519KeyPair::generate().unwrap();
    let aws_pk = aws_keypair.public_key();
    let aws_verifier = UnparsedPublicKey::new(&ED25519, aws_pk.as_ref());

    let rc_sk = EdSigningKey::from_bytes(&SEED);
    let rc_pk = rc_sk.verifying_key();

    let mut group = c.benchmark_group("Ed25519/sign");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        let data = data.as_slice();

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
            b.iter(|| {
                black_box(std_sk.sign(black_box(data)));
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
            b.iter(|| {
                black_box(aws_keypair.sign(black_box(data)));
            });
        });
        group.bench_with_input(BenchmarkId::new("Dalek", size), data, |b, data| {
            b.iter(|| {
                black_box(rc_sk.sign(black_box(data)));
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Ed25519/verify");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        let data = data.as_slice();
        let std_sig = std_sk.sign(data);
        let aws_sig = aws_keypair.sign(data);
        let rc_sig = rc_sk.sign(data);

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
            b.iter(|| {
                black_box(std_pk.verify(black_box(data), &std_sig).is_ok());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
            b.iter(|| {
                black_box(aws_verifier.verify(black_box(data), aws_sig.as_ref()).is_ok());
            });
        });
        group.bench_with_input(BenchmarkId::new("Dalek", size), data, |b, data| {
            b.iter(|| {
                black_box(rc_pk.verify(black_box(data), &rc_sig).is_ok());
            });
        });
    }
    group.finish();
}

fn bench_ml_dsa_44(c: &mut Criterion) {
    let std_sk = MlDsa44SecretKey::new(&SEED);
    let std_pk = std_sk.public_key();

    let aws_keypair = PqdsaKeyPair::generate(&ML_DSA_44_SIGNING).unwrap();
    let aws_pk = aws_keypair.public_key().as_der().unwrap();
    let aws_verifier = UnparsedPublicKey::new(&ML_DSA_44, aws_pk.as_ref());

    let rc_sk = MlSigningKey::<MlDsa44>::new_from_slice(&SEED).unwrap();
    let rc_pk = rc_sk.verifying_key();

    let mut group = c.benchmark_group("ML-DSA-44/sign");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        let data = data.as_slice();
        let mut aws_sig = vec![0u8; ML_DSA_44_SIGNING.signature_len()];

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
            b.iter(|| {
                black_box(std_sk.sign(black_box(data), &[]).unwrap());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
            b.iter(|| {
                aws_keypair.sign(black_box(data), &mut aws_sig).unwrap();
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), data, |b, data| {
            b.iter(|| {
                black_box(rc_sk.sign(black_box(data)));
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("ML-DSA-44/verify");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        let data = data.as_slice();
        let std_sig = std_sk.sign(data, &[]).unwrap();
        let mut aws_sig = vec![0u8; ML_DSA_44_SIGNING.signature_len()];
        aws_keypair.sign(data, &mut aws_sig).unwrap();
        let rc_sig = rc_sk.sign(data);

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
            b.iter(|| {
                black_box(std_pk.verify(black_box(data), &std_sig, &[]).is_ok());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
            b.iter(|| {
                black_box(aws_verifier.verify(black_box(data), &aws_sig).is_ok());
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), data, |b, data| {
            b.iter(|| {
                black_box(rc_pk.verify(black_box(data), &rc_sig).is_ok());
            });
        });
    }
    group.finish();
}

fn bench_ml_dsa_65(c: &mut Criterion) {
    let std_sk = MlDsa65SecretKey::new(&SEED);
    let std_pk = std_sk.public_key();

    let aws_keypair = PqdsaKeyPair::generate(&ML_DSA_65_SIGNING).unwrap();
    let aws_pk = aws_keypair.public_key().as_der().unwrap();
    let aws_verifier = UnparsedPublicKey::new(&ML_DSA_65, aws_pk.as_ref());

    let rc_sk = MlSigningKey::<MlDsa65>::new_from_slice(&SEED).unwrap();
    let rc_pk = rc_sk.verifying_key();

    let mut group = c.benchmark_group("ML-DSA-65/sign");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        let data = data.as_slice();
        let mut aws_sig = vec![0u8; ML_DSA_65_SIGNING.signature_len()];

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
            b.iter(|| {
                black_box(std_sk.sign(black_box(data), &[]).unwrap());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
            b.iter(|| {
                aws_keypair.sign(black_box(data), &mut aws_sig).unwrap();
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), data, |b, data| {
            b.iter(|| {
                black_box(rc_sk.sign(black_box(data)));
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("ML-DSA-65/verify");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        let data = data.as_slice();
        let std_sig = std_sk.sign(data, &[]).unwrap();
        let mut aws_sig = vec![0u8; ML_DSA_65_SIGNING.signature_len()];
        aws_keypair.sign(data, &mut aws_sig).unwrap();
        let rc_sig = rc_sk.sign(data);

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
            b.iter(|| {
                black_box(std_pk.verify(black_box(data), &std_sig, &[]).is_ok());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
            b.iter(|| {
                black_box(aws_verifier.verify(black_box(data), &aws_sig).is_ok());
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), data, |b, data| {
            b.iter(|| {
                black_box(rc_pk.verify(black_box(data), &rc_sig).is_ok());
            });
        });
    }
    group.finish();
}

fn bench_ml_dsa_87(c: &mut Criterion) {
    let std_sk = MlDsa87SecretKey::new(&SEED);
    let std_pk = std_sk.public_key();

    let aws_keypair = PqdsaKeyPair::generate(&ML_DSA_87_SIGNING).unwrap();
    let aws_pk = aws_keypair.public_key().as_der().unwrap();
    let aws_verifier = UnparsedPublicKey::new(&ML_DSA_87, aws_pk.as_ref());

    let rc_sk = MlSigningKey::<MlDsa87>::new_from_slice(&SEED).unwrap();
    let rc_pk = rc_sk.verifying_key();

    let mut group = c.benchmark_group("ML-DSA-87/sign");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        let data = data.as_slice();
        let mut aws_sig = vec![0u8; ML_DSA_87_SIGNING.signature_len()];

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
            b.iter(|| {
                black_box(std_sk.sign(black_box(data), &[]).unwrap());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
            b.iter(|| {
                aws_keypair.sign(black_box(data), &mut aws_sig).unwrap();
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), data, |b, data| {
            b.iter(|| {
                black_box(rc_sk.sign(black_box(data)));
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("ML-DSA-87/verify");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        let data = data.as_slice();
        let std_sig = std_sk.sign(data, &[]).unwrap();
        let mut aws_sig = vec![0u8; ML_DSA_87_SIGNING.signature_len()];
        aws_keypair.sign(data, &mut aws_sig).unwrap();
        let rc_sig = rc_sk.sign(data);

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
            b.iter(|| {
                black_box(std_pk.verify(black_box(data), &std_sig, &[]).is_ok());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
            b.iter(|| {
                black_box(aws_verifier.verify(black_box(data), &aws_sig).is_ok());
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), data, |b, data| {
            b.iter(|| {
                black_box(rc_pk.verify(black_box(data), &rc_sig).is_ok());
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_ed25519, bench_ml_dsa_44, bench_ml_dsa_65, bench_ml_dsa_87);
criterion_main!(benches);
