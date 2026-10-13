use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use uuid::Uuid;

fn bench_v4(c: &mut Criterion) {
    c.bench_function("Uuid::new_v4", |b| b.iter(Uuid::new_v4));
}

fn bench_v7(c: &mut Criterion) {
    c.bench_function("Uuid::new_v7", |b| b.iter(Uuid::new_v7));
}

fn bench_parse(c: &mut Criterion) {
    let canonical = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
    c.bench_function("Uuid::parse", |b| b.iter(|| Uuid::parse(black_box(canonical))));
}

fn bench_format(c: &mut Criterion) {
    let uuid = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();

    c.bench_function("Uuid::to_string", |b| b.iter(|| black_box(uuid).to_string()));
    c.bench_function("Uuid::to_heapless_string", |b| b.iter(|| black_box(uuid).to_heapless_string()));
}

criterion_group!(benches, bench_v4, bench_v7, bench_parse, bench_format);
criterion_main!(benches);
