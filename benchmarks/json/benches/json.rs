#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use json_benchmarks::{Canada, Tweet, canada, citm, escaped, floats, ints, strings, twitter};

fn value_parse(c: &mut Criterion) {
    for (name, text) in [("twitter", twitter()), ("canada", canada()), ("citm", citm())] {
        let mut group = c.benchmark_group(format!("value_parse/{name}"));
        group.bench_function("json", |b| b.iter(|| json::from_str::<json::Value>(black_box(&text)).unwrap()));
        group.bench_function("serde_json", |b| {
            b.iter(|| serde_json::from_str::<serde_json::Value>(black_box(&text)).unwrap())
        });
        group.bench_function("json_slice", |b| {
            b.iter(|| json::from_slice::<json::Value>(black_box(text.as_bytes())).unwrap())
        });
        group.bench_function("serde_json_slice", |b| {
            b.iter(|| serde_json::from_slice::<serde_json::Value>(black_box(text.as_bytes())).unwrap())
        });
        group.bench_function("simd-json", |b| {
            b.iter(|| {
                let mut bytes = text.as_bytes().to_vec();
                let value = simd_json::to_borrowed_value(black_box(&mut bytes)).unwrap();
                black_box(&value);
            })
        });
        group.bench_function("sonic-rs", |b| {
            b.iter(|| sonic_rs::from_str::<sonic_rs::Value>(black_box(&text)).unwrap())
        });
        group.finish();
    }
}

fn typed(c: &mut Criterion) {
    let text = twitter();
    let mut group = c.benchmark_group("typed_twitter");
    group.bench_function("json", |b| b.iter(|| json::from_str::<Tweet>(black_box(&text)).unwrap()));
    group.bench_function("serde_json", |b| {
        b.iter(|| serde_json::from_str::<Tweet>(black_box(&text)).unwrap())
    });
    group.bench_function("simd-json", |b| {
        b.iter(|| {
            let mut bytes = text.as_bytes().to_vec();
            simd_json::serde::from_slice::<Tweet>(black_box(&mut bytes)).unwrap()
        })
    });
    group.bench_function("sonic-rs", |b| {
        b.iter(|| sonic_rs::from_str::<Tweet>(black_box(&text)).unwrap())
    });
    group.finish();

    let text = canada();
    let mut group = c.benchmark_group("typed_canada");
    group.bench_function("json", |b| b.iter(|| json::from_str::<Canada>(black_box(&text)).unwrap()));
    group.bench_function("serde_json", |b| {
        b.iter(|| serde_json::from_str::<Canada>(black_box(&text)).unwrap())
    });
    group.bench_function("simd-json", |b| {
        b.iter(|| {
            let mut bytes = text.as_bytes().to_vec();
            simd_json::serde::from_slice::<Canada>(black_box(&mut bytes)).unwrap()
        })
    });
    group.bench_function("sonic-rs", |b| {
        b.iter(|| sonic_rs::from_str::<Canada>(black_box(&text)).unwrap())
    });
    group.finish();
}

fn serialize(c: &mut Criterion) {
    for (name, text) in [("twitter", twitter()), ("canada", canada()), ("citm", citm())] {
        let value: json::Value = json::from_str(&text).unwrap();
        let serde_value: serde_json::Value = serde_json::from_str(&text).unwrap();
        let sonic_value = sonic_rs::from_str::<sonic_rs::Value>(&text).unwrap();
        let mut group = c.benchmark_group(format!("serialize/{name}"));
        group.bench_function("json", |b| b.iter(|| json::to_string(black_box(&value))));
        group.bench_function("serde_json", |b| {
            b.iter(|| serde_json::to_string(black_box(&serde_value)).unwrap())
        });
        group.bench_function("sonic-rs", |b| b.iter(|| sonic_rs::to_string(black_box(&sonic_value)).unwrap()));
        group.finish();
    }
}

fn validate(c: &mut Criterion) {
    for (name, text) in [("twitter", twitter()), ("canada", canada()), ("citm", citm())] {
        let mut group = c.benchmark_group(format!("validate/{name}"));
        group.bench_function("json", |b| b.iter(|| json::validate(black_box(text.as_bytes())).unwrap()));
        group.finish();
    }
}

fn components(c: &mut Criterion) {
    for (name, text) in [
        ("strings", strings()),
        ("escaped", escaped()),
        ("ints", ints()),
        ("floats", floats()),
    ] {
        let mut group = c.benchmark_group(format!("components/{name}"));
        group.bench_function("json_value", |b| {
            b.iter(|| json::from_str::<json::Value>(black_box(&text)).unwrap())
        });
        group.bench_function("serde_json_value", |b| {
            b.iter(|| serde_json::from_str::<serde_json::Value>(black_box(&text)).unwrap())
        });
        group.bench_function("json_validate", |b| {
            b.iter(|| json::validate(black_box(text.as_bytes())).unwrap())
        });
        group.finish();
    }
}

criterion_group!(benches, value_parse, typed, serialize, validate, components);
criterion_main!(benches);
