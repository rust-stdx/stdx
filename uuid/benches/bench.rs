use criterion::{Criterion, criterion_group, criterion_main};
use uuid::Uuid;

fn bench_v4(c: &mut Criterion) {
    c.bench_function("Uuid::new_v4", |b| {
        b.iter(|| Uuid::new_v4());
    });
}

fn bench_v7(c: &mut Criterion) {
    c.bench_function("Uuid::new_v7", |b| {
        b.iter(|| Uuid::new_v7());
    });
}

criterion_group!(benches, bench_v4, bench_v7);
criterion_main!(benches);
