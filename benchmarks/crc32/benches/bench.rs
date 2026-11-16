use std::hint::black_box;

use crc32::Hasher;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

const DATA_SIZES: &[usize] = &[1024, 1024 * 1024];

fn fill(bytes: &mut [u8]) {
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = (i.wrapping_mul(31) ^ 0x5a) as u8;
    }
}

fn bench_hasher(c: &mut Criterion, name: &str, make_hasher: fn() -> Hasher) {
    let mut group = c.benchmark_group("crc32");

    for &size in DATA_SIZES {
        let mut bytes = vec![0u8; size];
        fill(&mut bytes);

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new(name, size), &bytes, |b, bytes| {
            b.iter(|| {
                let mut hasher = make_hasher();
                hasher.update(black_box(bytes));
                black_box(hasher.finalize())
            });
        });
    }

    group.finish();
}

fn bench_baseline(c: &mut Criterion) {
    bench_hasher(c, "baseline", || Hasher::internal_new_baseline(0, 0));
}

fn bench_specialized(c: &mut Criterion) {
    bench_hasher(c, "specialized", || Hasher::internal_new_specialized(0, 0).unwrap());
}

criterion_group!(benches, bench_baseline, bench_specialized);
criterion_main!(benches);
