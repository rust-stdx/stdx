#![allow(deprecated)]

use std::hint::black_box;

use ascon_hash256::AsconHash256;
use aws_lc_rs::digest::{self, SHA3_256, SHA3_512, SHA256, SHA512};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    Hasher,
    ascon::AsconHash256 as StdAsconHash256,
    blake3::Blake3,
    sha2::{Sha256 as StdSha256, Sha512 as StdSha512},
    sha3::{Sha3_256 as StdSha3_256, Sha3_512 as StdSha3_512, Shake256 as StdShake256},
};
use sha2::{Digest, Sha256, Sha512};
use sha3::{
    Sha3_256, Sha3_512, Shake256,
    digest::{ExtendableOutput, Update},
};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

fn bench_stdx<F>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    size: usize,
    data: &[u8],
    f: F,
) where
    F: Fn(&[u8]),
{
    group.bench_with_input(BenchmarkId::new("stdx-crypto", size), data, |b, data| {
        b.iter(|| f(black_box(data)));
    });
}

fn bench_aws_lc_rs<F>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    size: usize,
    data: &[u8],
    f: F,
) where
    F: Fn(&[u8]),
{
    group.bench_with_input(BenchmarkId::new("aws-lc-rs", size), data, |b, data| {
        b.iter(|| f(black_box(data)));
    });
}

fn bench_rustcrypto<F>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    size: usize,
    data: &[u8],
    f: F,
) where
    F: Fn(&[u8]),
{
    group.bench_with_input(BenchmarkId::new("RustCrypto", size), data, |b, data| {
        b.iter(|| f(black_box(data)));
    });
}

fn bench_hashes(c: &mut Criterion) {
    let mut group = c.benchmark_group("SHA-256");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        bench_stdx(&mut group, size, &data, |data| {
            black_box(StdSha256::hash(data));
        });
        bench_aws_lc_rs(&mut group, size, &data, |data| {
            black_box(digest::digest(&SHA256, data));
        });
        bench_rustcrypto(&mut group, size, &data, |data| {
            black_box(Sha256::digest(data));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("SHA-512");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        bench_stdx(&mut group, size, &data, |data| {
            black_box(StdSha512::hash(data));
        });
        bench_aws_lc_rs(&mut group, size, &data, |data| {
            black_box(digest::digest(&SHA512, data));
        });
        bench_rustcrypto(&mut group, size, &data, |data| {
            black_box(Sha512::digest(data));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("SHA3-256");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        bench_stdx(&mut group, size, &data, |data| {
            black_box(StdSha3_256::hash(data));
        });
        bench_aws_lc_rs(&mut group, size, &data, |data| {
            black_box(digest::digest(&SHA3_256, data));
        });
        bench_rustcrypto(&mut group, size, &data, |data| {
            black_box(Sha3_256::digest(data));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("SHA3-512");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        bench_stdx(&mut group, size, &data, |data| {
            black_box(StdSha3_512::hash(data));
        });
        bench_aws_lc_rs(&mut group, size, &data, |data| {
            black_box(digest::digest(&SHA3_512, data));
        });
        bench_rustcrypto(&mut group, size, &data, |data| {
            black_box(Sha3_512::digest(data));
        });
    }
    group.finish();

    // aws-lc-rs does not expose SHAKE256, so only stdx-crypto and RustCrypto are compared.
    let mut group = c.benchmark_group("SHAKE256");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        bench_stdx(&mut group, size, &data, |data| {
            black_box(<StdShake256 as Hasher>::hash(data));
        });
        bench_rustcrypto(&mut group, size, &data, |data| {
            let mut hasher = Shake256::default();
            hasher.update(data);
            let mut output = [0u8; 32];
            hasher.finalize_xof_into(&mut output);
            black_box(output);
        });
    }
    group.finish();

    let mut group = c.benchmark_group("BLAKE3");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        bench_stdx(&mut group, size, &data, |data| {
            black_box(Blake3::hash(data));
        });
        group.bench_with_input(BenchmarkId::new("Official", size), &data, |b, data| {
            b.iter(|| blake3::hash(black_box(data)));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Ascon-Hash256");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xA5u8; size];
        bench_stdx(&mut group, size, &data, |data| {
            black_box(StdAsconHash256::hash(data));
        });
        bench_rustcrypto(&mut group, size, &data, |data| {
            black_box(AsconHash256::digest(data));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_hashes);
criterion_main!(benches);
