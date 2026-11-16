use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn inputs() -> Vec<(&'static str, Vec<u8>)> {
    let ascii = |length: usize| {
        let mut bytes = Vec::with_capacity(length);
        for index in 0..length {
            bytes.push(b'a' + (index % 26) as u8);
        }
        bytes
    };
    let non_ascii = |length: usize| {
        let mut bytes = Vec::with_capacity(length);
        while bytes.len() + 3 <= length {
            bytes.extend_from_slice("€".as_bytes());
        }
        while bytes.len() < length {
            bytes.push(b'a');
        }
        bytes
    };

    let mut inputs = Vec::new();
    for length in [16usize, 64, 1024, 64 * 1024, 1024 * 1024] {
        inputs.push(("ascii", ascii(length)));
        inputs.push(("non_ascii", non_ascii(length)));
    }
    inputs
}

fn validate_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("validate");
    for (name, bytes) in inputs() {
        group.bench_with_input(
            BenchmarkId::new("std", format!("{name}/{}", bytes.len())),
            &bytes,
            |b, bytes| b.iter(|| core::str::from_utf8(black_box(bytes)).is_ok()),
        );
        group.bench_with_input(
            BenchmarkId::new("utf8", format!("{name}/{}", bytes.len())),
            &bytes,
            |b, bytes| b.iter(|| utf8::validate(black_box(bytes))),
        );
    }
    group.finish();
}

fn from_utf8_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_utf8");
    for (name, bytes) in inputs() {
        group.bench_with_input(
            BenchmarkId::new("std", format!("{name}/{}", bytes.len())),
            &bytes,
            |b, bytes| b.iter(|| core::str::from_utf8(black_box(bytes)).unwrap()),
        );
        group.bench_with_input(
            BenchmarkId::new("utf8", format!("{name}/{}", bytes.len())),
            &bytes,
            |b, bytes| b.iter(|| utf8::from_utf8(black_box(bytes)).unwrap()),
        );
    }
    group.finish();
}

criterion_group!(benches, validate_bench, from_utf8_bench);
criterion_main!(benches);
