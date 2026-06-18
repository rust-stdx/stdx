use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

macro_rules! bench_int {
    ($c:expr, $ty:ty, $name:expr, $vals:expr) => {{
        let mut fn_buf = format_number::Buffer::new();
        let mut itoa_buf = itoa::Buffer::new();

        let mut g = $c.benchmark_group($name);
        for &v in $vals {
            g.bench_with_input(BenchmarkId::new("format_number", v), &v, |b, &v| {
                b.iter(|| {
                    let _ = black_box(fn_buf.format(black_box(v)));
                })
            });
            g.bench_with_input(BenchmarkId::new("itoa", v), &v, |b, &v| {
                b.iter(|| {
                    let _ = black_box(itoa_buf.format(black_box(v)));
                })
            });
        }
        g.finish();
    }};
}

fn bench_u64(c: &mut Criterion) {
    bench_int!(c, u64, "u64", &[1, 42, 100, 100_000_000_000_000, u64::MAX,]);
}

fn bench_i64(c: &mut Criterion) {
    bench_int!(c, i64, "i64", &[-1, 1, 42, 100, 100_000_000_000_000, i64::MAX, i64::MIN,]);
}

fn bench_u128(c: &mut Criterion) {
    bench_int!(c, u128, "u128", &[1, 42, 100, 100_000_000_000_000, u128::MAX,]);
}

fn bench_i128(c: &mut Criterion) {
    bench_int!(c, i128, "i128", &[-1, 1, 42, 100, 100_000_000_000_000, i128::MAX, i128::MIN,]);
}

fn bench_u8(c: &mut Criterion) {
    bench_int!(c, u8, "u8", &[1, 42, 100, u8::MAX,]);
}

fn bench_u16(c: &mut Criterion) {
    bench_int!(c, u16, "u16", &[1, 42, 100, u16::MAX,]);
}

fn bench_u32(c: &mut Criterion) {
    bench_int!(c, u32, "u32", &[1, 42, 100, 100_000, u32::MAX,]);
}

fn bench_i32(c: &mut Criterion) {
    bench_int!(c, i32, "i32", &[-1, 1, 42, 100, 100_000, i32::MAX, i32::MIN,]);
}

criterion_group!(
    benches, bench_u64, bench_i64, bench_u128, bench_i128, bench_u8, bench_u16, bench_u32, bench_i32,
);

criterion_main!(benches);
