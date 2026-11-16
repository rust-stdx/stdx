//! ML-KEM benchmarks.
//!
//! Deterministic entry points are used so the measurements exclude the
//! operating-system RNG. Decapsulation is measured both for a valid ciphertext
//! and for one that triggers implicit rejection (the two paths differ by one
//! constant-time comparison and the derivation of the rejection secret).

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use crypto::mlkem::{SecretKey768, SecretKey1024};

fn bench_ml_kem(c: &mut Criterion) {
    let coins768 = [7u8; 64];
    let (sk768, pk768) = SecretKey768::generate_derand(&coins768);
    let (_ss, ct768) = pk768.encapsulate_derand(&[9u8; 32]);
    let mut bad768 = ct768;
    bad768[0] ^= 1;

    let coins1024 = [7u8; 64];
    let (sk1024, pk1024) = SecretKey1024::generate_derand(&coins1024);
    let (_ss, ct1024) = pk1024.encapsulate_derand(&[9u8; 32]);
    let mut bad1024 = ct1024;
    bad1024[0] ^= 1;

    let mut group = c.benchmark_group("ml-kem");

    group.bench_function("768/keygen", |b| {
        b.iter(|| black_box(SecretKey768::generate_derand(&coins768)));
    });
    group.bench_function("768/encapsulate", |b| {
        b.iter(|| black_box(pk768.encapsulate_derand(&[9u8; 32])));
    });
    group.bench_function("768/decapsulate", |b| {
        b.iter(|| black_box(sk768.decapsulate(black_box(&ct768)).unwrap()));
    });
    group.bench_function("768/decapsulate-reject", |b| {
        b.iter(|| black_box(sk768.decapsulate(black_box(&bad768)).unwrap()));
    });

    group.bench_function("1024/keygen", |b| {
        b.iter(|| black_box(SecretKey1024::generate_derand(&coins1024)));
    });
    group.bench_function("1024/encapsulate", |b| {
        b.iter(|| black_box(pk1024.encapsulate_derand(&[9u8; 32])));
    });
    group.bench_function("1024/decapsulate", |b| {
        b.iter(|| black_box(sk1024.decapsulate(black_box(&ct1024)).unwrap()));
    });
    group.bench_function("1024/decapsulate-reject", |b| {
        b.iter(|| black_box(sk1024.decapsulate(black_box(&bad1024)).unwrap()));
    });

    group.finish();
}

criterion_group!(benches, bench_ml_kem);
criterion_main!(benches);
