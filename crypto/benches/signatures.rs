use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    curve25519::ed25519::SecretKey,
    mldsa::{MlDsa44SecretKey, MlDsa65SecretKey, MlDsa87SecretKey},
};

const DATA_SIZES: &[usize] = &[64, 1024, 64 * 1024, 1024 * 1024];

fn bench_sign(c: &mut Criterion) {
    let ed25519_sk = black_box(SecretKey::generate());
    let mldsa44_sk = MlDsa44SecretKey::new(&[0u8; 32]);
    let mldsa65_sk = MlDsa65SecretKey::new(&[0u8; 32]);
    let mldsa87_sk = MlDsa87SecretKey::new(&[0u8; 32]);

    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("sign/{size}"));
        group.throughput(Throughput::Bytes(size as u64));

        let data = vec![0xA5_u8; size];
        let data_slice = data.as_slice();

        group.bench_with_input(BenchmarkId::from_parameter("Ed25519"), data_slice, |b, data| {
            b.iter(|| {
                let signature = ed25519_sk.sign(black_box(data));
                black_box(signature);
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("ML-DSA-44"), data_slice, |b, data| {
            b.iter(|| {
                let signature = mldsa44_sk.sign(black_box(data), &[]).unwrap();
                black_box(signature);
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("ML-DSA-65"), data_slice, |b, data| {
            b.iter(|| {
                let signature = mldsa65_sk.sign(black_box(data), &[]).unwrap();
                black_box(signature);
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("ML-DSA-87"), data_slice, |b, data| {
            b.iter(|| {
                let signature = mldsa87_sk.sign(black_box(data), &[]).unwrap();
                black_box(signature);
            });
        });

        group.finish();
    }
}

fn bench_verify(c: &mut Criterion) {
    let ed25519_sk = black_box(SecretKey::generate());
    let ed25519_pk = black_box(ed25519_sk.public_key());

    let mldsa44_sk = MlDsa44SecretKey::new(&[0u8; 32]);
    let mldsa44_pk = mldsa44_sk.public_key();
    let mldsa65_sk = MlDsa65SecretKey::new(&[0u8; 32]);
    let mldsa65_pk = mldsa65_sk.public_key();
    let mldsa87_sk = MlDsa87SecretKey::new(&[0u8; 32]);
    let mldsa87_pk = mldsa87_sk.public_key();

    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("verify/{size}"));
        group.throughput(Throughput::Bytes(size as u64));

        let data = vec![0xA5_u8; size];
        let data_slice = data.as_slice();

        group.bench_with_input(BenchmarkId::from_parameter("Ed25519"), data_slice, |b, data| {
            let signature = ed25519_sk.sign(data);
            b.iter(|| {
                black_box(ed25519_pk.verify(black_box(data), black_box(&signature)).is_ok());
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("ML-DSA-44"), data_slice, |b, data| {
            let signature = mldsa44_sk.sign(data, &[]).unwrap();
            b.iter(|| {
                black_box(mldsa44_pk.verify(black_box(data), black_box(&signature), &[]).is_ok());
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("ML-DSA-65"), data_slice, |b, data| {
            let signature = mldsa65_sk.sign(data, &[]).unwrap();
            b.iter(|| {
                black_box(mldsa65_pk.verify(black_box(data), black_box(&signature), &[]).is_ok());
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("ML-DSA-87"), data_slice, |b, data| {
            let signature = mldsa87_sk.sign(data, &[]).unwrap();
            b.iter(|| {
                black_box(mldsa87_pk.verify(black_box(data), black_box(&signature), &[]).is_ok());
            });
        });

        group.finish();
    }
}

criterion_group!(benches, bench_sign, bench_verify);
criterion_main!(benches);
