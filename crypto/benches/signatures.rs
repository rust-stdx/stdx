use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    curve25519::ed25519::SecretKey,
    mldsa::{ml_dsa_65_generate_keypair, ml_dsa_65_sign, ml_dsa_65_verify},
};

const DATA_SIZES: &[usize] = &[64, 1024, 64 * 1024, 1024 * 1024];

fn bench_sign(c: &mut Criterion) {
    let ed25519_sk = black_box(SecretKey::generate());
    let (mldsa65_seed, _) = black_box(ml_dsa_65_generate_keypair());

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

        group.bench_with_input(BenchmarkId::from_parameter("ML-DSA-65"), data_slice, |b, data| {
            b.iter(|| {
                let signature = ml_dsa_65_sign(black_box(&mldsa65_seed), black_box(data), &[]).unwrap();
                black_box(signature);
            });
        });

        group.finish();
    }
}

fn bench_verify(c: &mut Criterion) {
    let ed25519_sk = black_box(SecretKey::generate());
    let ed25519_pk = black_box(ed25519_sk.public_key());
    let (mldsa65_seed, mldsa65_pk) = black_box(ml_dsa_65_generate_keypair());

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

        group.bench_with_input(BenchmarkId::from_parameter("ML-DSA-65"), data_slice, |b, data| {
            let signature = ml_dsa_65_sign(&mldsa65_seed, &data, &[]).unwrap();
            b.iter(|| {
                black_box(
                    ml_dsa_65_verify(black_box(&mldsa65_pk), black_box(data), black_box(&signature), &[]).is_ok(),
                );
            });
        });

        group.finish();
    }
}

criterion_group!(benches, bench_sign, bench_verify);
criterion_main!(benches);
