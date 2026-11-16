#![allow(deprecated)]

use std::hint::black_box;

use aws_lc_rs::hmac::{HMAC_SHA256, HMAC_SHA512, Key, sign};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    blake3::Blake3,
    hmac::Hmac,
    poly1305::Poly1305,
    sha2::{Sha256 as StdSha256, Sha512 as StdSha512},
};
use hmac::{Hmac as RcHmac, Mac};
use poly1305::{
    Poly1305 as RcPoly1305,
    universal_hash::{KeyInit, UniversalHash},
};
use sha2::{Sha256, Sha512};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

const KEY: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
];

const HMAC_KEY: &[u8] = b"rust-stdx-crypto-bench-key";

fn bench_macs(c: &mut Criterion) {
    let mut group = c.benchmark_group("HMAC-SHA256");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA3u8; size];

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), &data, |b, data| {
            b.iter(|| {
                let mut mac = Hmac::<StdSha256>::new(black_box(HMAC_KEY));
                mac.update(black_box(data.as_slice()));
                black_box(mac.finalize());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), &data, |b, data| {
            let key = Key::new(HMAC_SHA256, HMAC_KEY);
            b.iter(|| {
                black_box(sign(&key, black_box(data.as_slice())));
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), &data, |b, data| {
            b.iter(|| {
                let mut mac = RcHmac::<Sha256>::new_from_slice(black_box(HMAC_KEY)).unwrap();
                mac.update(black_box(data.as_slice()));
                black_box(mac.finalize().into_bytes());
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("HMAC-SHA512");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA3u8; size];

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), &data, |b, data| {
            b.iter(|| {
                let mut mac = Hmac::<StdSha512>::new(black_box(HMAC_KEY));
                mac.update(black_box(data.as_slice()));
                black_box(mac.finalize());
            });
        });
        group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), &data, |b, data| {
            let key = Key::new(HMAC_SHA512, HMAC_KEY);
            b.iter(|| {
                black_box(sign(&key, black_box(data.as_slice())));
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), &data, |b, data| {
            b.iter(|| {
                let mut mac = RcHmac::<Sha512>::new_from_slice(black_box(HMAC_KEY)).unwrap();
                mac.update(black_box(data.as_slice()));
                black_box(mac.finalize().into_bytes());
            });
        });
    }
    group.finish();

    // stdx-crypto is the only implementation providing KMAC256 among the
    // compared crates, so there is no competitor to benchmark it against.

    let mut group = c.benchmark_group("Poly1305");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA3u8; size];

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), &data, |b, data| {
            b.iter(|| {
                black_box(Poly1305::mac(black_box(&KEY), black_box(data.as_slice())));
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), &data, |b, data| {
            b.iter(|| {
                let mut mac = RcPoly1305::new_from_slice(black_box(&KEY)).unwrap();
                mac.update_padded(black_box(data.as_slice()));
                black_box(mac.finalize());
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("BLAKE3-keyed");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA3u8; size];

        group.bench_with_input(BenchmarkId::new("stdx-crypto", size), &data, |b, data| {
            b.iter(|| {
                black_box(Blake3::keyed_hash(black_box(&KEY), black_box(data.as_slice())));
            });
        });
        group.bench_with_input(BenchmarkId::new("RustCrypto", size), &data, |b, data| {
            b.iter(|| {
                black_box(blake3::keyed_hash(black_box(&KEY), black_box(data.as_slice())));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_macs);
criterion_main!(benches);
