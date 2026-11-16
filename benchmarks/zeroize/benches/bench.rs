//! Benchmarks for the `Zeroize` implementations.
//!
//! These track the cost of securely clearing memory, in particular that bulk (slice/Vec/String)
//! zeroization uses a single volatile set rather than one volatile write per element.
//!
//! The buffers are re-poisoned with `fill`/`push_str` before each zeroization so the zeroize
//! operation always has real work to do and no allocation happens inside the measured loop.

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use zeroize::{Zeroize, Zeroizing};

const SIZES: &[usize] = &[32, 256, 4096, 64 * 1024];

fn bench_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("array");
    group.throughput(Throughput::Bytes(32));
    group.bench_function("32", |b| {
        let mut key = [0xABu8; 32];
        b.iter(|| {
            key.fill(0xAB);
            key.zeroize();
        });
    });
    group.finish();
}

fn bench_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("slice");
    for &size in SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let mut buf = vec![0xABu8; size];
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                buf.fill(0xAB);
                buf.as_mut_slice().zeroize();
            });
        });
    }
    group.finish();
}

fn bench_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec");
    for &size in SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let mut buf = vec![0xABu8; size];
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                buf.fill(0xAB);
                buf.zeroize();
            });
        });
    }
    group.finish();
}

fn bench_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("string");
    for &size in SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let poison = "A".repeat(size);
        let mut string = String::with_capacity(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                string.clear();
                string.push_str(&poison);
                string.zeroize();
            });
        });
    }
    group.finish();
}

fn bench_zeroizing_drop(c: &mut Criterion) {
    let mut group = c.benchmark_group("zeroizing_drop");
    for &size in SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_batched(|| Zeroizing::new(vec![0xABu8; size]), drop, BatchSize::SmallInput);
        });
    }
    group.finish();
}

criterion_group!(benches, bench_array, bench_slice, bench_vec, bench_string, bench_zeroizing_drop);
criterion_main!(benches);
