use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use xxhash::{Checksum, Xxh3_64, Xxh3_128, Xxh32, Xxh64};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

fn bench_hashes(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(size.to_string());
        group.throughput(Throughput::Bytes(size as u64));

        let data = vec![0xA5_u8; size];

        group.bench_with_input(BenchmarkId::from_parameter("XXH32"), &data, |b, data| {
            b.iter(|| {
                let _ = Xxh32::checksum(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("XXH64"), &data, |b, data| {
            b.iter(|| {
                let _ = Xxh64::checksum(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("XXH3-64"), &data, |b, data| {
            b.iter(|| {
                let _ = Xxh3_64::checksum(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("XXH3-128"), &data, |b, data| {
            b.iter(|| {
                let _ = Xxh3_128::checksum(black_box(data.as_slice()));
            });
        });

        group.finish();
    }
}

/// Incremental hashing, to track the cost of the streaming state machine.
///
/// Feeding the data in small updates is inherently slower than one-shot
/// hashing; these benchmarks make sure it stays within a reasonable factor.
fn bench_streaming(c: &mut Criterion) {
    const STREAM_SIZES: &[usize] = &[64, 1024, 16 * 1024, 1024 * 1024];
    const CHUNKS: &[usize] = &[16, 4096];

    for &size in STREAM_SIZES {
        let data = vec![0xA5_u8; size];
        let mut group = c.benchmark_group(format!("streaming/{size}"));
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::from_parameter("one-shot"), &data, |b, data| {
            b.iter(|| {
                let _ = Xxh3_64::checksum(black_box(data.as_slice()));
            });
        });

        for &chunk in CHUNKS {
            group.bench_with_input(BenchmarkId::new("Xxh3_64", chunk), &data, |b, data| {
                b.iter(|| {
                    let mut hasher = Xxh3_64::new();
                    for slice in data.chunks(chunk) {
                        hasher.update(black_box(slice));
                    }
                    black_box(hasher.sum())
                });
            });

            group.bench_with_input(BenchmarkId::new("Xxh3_128", chunk), &data, |b, data| {
                b.iter(|| {
                    let mut hasher = Xxh3_128::new();
                    for slice in data.chunks(chunk) {
                        hasher.update(black_box(slice));
                    }
                    black_box(hasher.sum())
                });
            });
        }

        group.finish();
    }
}

criterion_group!(benches, bench_hashes, bench_streaming);
criterion_main!(benches);
