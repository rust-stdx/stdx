use std::hint::black_box;

use columnar::{Columnar, ColumnarDecode, ColumnarIter};
use columnar_benchmarks::{
    AutoRow, BorrowedRow, DeltaOfDeltaRow, DeltaRleRow, RleRow, blocky, borrowed_rows, container_rows, enum_rows,
    jittered_timestamps, random, regular_timestamps, sequential, text_rows, wide_rows,
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};

fn bench_encoding<T: Columnar + for<'de> ColumnarDecode<'de> + for<'de> ColumnarIter<'de>>(
    c: &mut Criterion,
    dataset: &str,
    encoding: &str,
    rows: &[T],
) -> usize {
    let bytes = columnar::to_vec(rows).unwrap();

    let mut group = c.benchmark_group(format!("encode/{dataset}/{encoding}"));
    group.throughput(Throughput::Elements(rows.len() as u64));
    group.bench_function("rows", |b| b.iter(|| columnar::to_vec(black_box(rows)).unwrap()));
    group.finish();

    let mut group = c.benchmark_group(format!("decode/{dataset}/{encoding}"));
    group.throughput(Throughput::Elements(rows.len() as u64));
    group.bench_function("rows", |b| b.iter(|| columnar::from_slice::<T>(black_box(&bytes)).unwrap()));
    group.finish();

    let mut group = c.benchmark_group(format!("iterate/{dataset}/{encoding}"));
    group.throughput(Throughput::Elements(rows.len() as u64));
    group.bench_function("rows", |b| {
        b.iter(|| {
            let iter = columnar::iter_from_slice::<T>(black_box(&bytes)).unwrap();
            black_box(iter.map(|row| row.unwrap()).count())
        })
    });
    group.finish();

    bytes.len()
}

fn bench_dataset(c: &mut Criterion, dataset: &str, values: &[u64]) {
    let auto: Vec<AutoRow> = values
        .iter()
        .map(|&value| AutoRow {
            value,
        })
        .collect();
    let rle: Vec<RleRow> = values
        .iter()
        .map(|&value| RleRow {
            value,
        })
        .collect();
    let delta_rle: Vec<DeltaRleRow> = values
        .iter()
        .map(|&value| DeltaRleRow {
            value,
        })
        .collect();
    let delta_of_delta: Vec<DeltaOfDeltaRow> = values
        .iter()
        .map(|&value| DeltaOfDeltaRow {
            value,
        })
        .collect();

    let auto_size = bench_encoding(c, dataset, "auto", &auto);
    let rle_size = bench_encoding(c, dataset, "rle", &rle);
    let delta_rle_size = bench_encoding(c, dataset, "delta_rle", &delta_rle);
    let delta_of_delta_size = bench_encoding(c, dataset, "delta_of_delta", &delta_of_delta);

    println!(
        "{dataset}: auto={auto_size} rle={rle_size} delta_rle={delta_rle_size} \
         delta_of_delta={delta_of_delta_size}"
    );
}

fn encodings(c: &mut Criterion) {
    for (name, values) in [
        ("sequential", sequential()),
        ("regular_timestamps", regular_timestamps()),
        ("jittered_timestamps", jittered_timestamps()),
        ("blocky", blocky()),
        ("random", random()),
    ] {
        bench_dataset(c, name, &values);
    }
}

fn containers(c: &mut Criterion) {
    let text = text_rows();
    let containers = container_rows();
    let wide = wide_rows();
    let enums = enum_rows();

    println!(
        "text={} containers={} wide={} enums={}",
        bench_encoding(c, "text", "auto", &text),
        bench_encoding(c, "containers", "auto", &containers),
        bench_encoding(c, "wide", "auto", &wide),
        bench_encoding(c, "enums", "auto", &enums),
    );
}

fn borrowed(c: &mut Criterion) {
    let rows = borrowed_rows();
    let bytes = columnar::to_vec(&rows).unwrap();
    println!("borrowed={}", bytes.len());

    let mut group = c.benchmark_group("encode/borrowed/auto");
    group.throughput(Throughput::Elements(rows.len() as u64));
    group.bench_function("rows", |b| b.iter(|| columnar::to_vec(black_box(&rows)).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("decode/borrowed/auto");
    group.throughput(Throughput::Elements(rows.len() as u64));
    group.bench_function("rows", |b| {
        b.iter(|| columnar::from_slice::<BorrowedRow<'_>>(black_box(&bytes)).unwrap())
    });
    group.finish();

    let mut group = c.benchmark_group("iterate/borrowed/auto");
    group.throughput(Throughput::Elements(rows.len() as u64));
    group.bench_function("rows", |b| {
        b.iter(|| {
            let iter = columnar::iter_from_slice::<BorrowedRow<'_>>(black_box(&bytes)).unwrap();
            black_box(iter.map(|row| row.unwrap()).count())
        })
    });
    group.finish();
}

fn small_batches(c: &mut Criterion) {
    for count in [1usize, 10, 100] {
        let rows: Vec<AutoRow> = (0..count as u64)
            .map(|value| AutoRow {
                value,
            })
            .collect();
        let bytes = columnar::to_vec(&rows).unwrap();

        for (name, op) in [("encode", 0u8), ("encode_into", 1), ("decode", 2), ("iterate", 3)] {
            let mut group = c.benchmark_group(format!("small/{count}/{name}"));
            group.throughput(Throughput::Elements(count as u64));
            let mut buffer = Vec::new();
            match op {
                0 => group.bench_function("rows", |b| b.iter(|| columnar::to_vec(black_box(&rows)).unwrap())),
                1 => group.bench_function("rows", |b| {
                    b.iter(|| {
                        columnar::to_vec_into(black_box(&rows), black_box(&mut buffer)).unwrap();
                        buffer.len()
                    })
                }),
                2 => group.bench_function("rows", |b| {
                    b.iter(|| columnar::from_slice::<AutoRow>(black_box(&bytes)).unwrap())
                }),
                _ => group.bench_function("rows", |b| {
                    b.iter(|| {
                        let iter = columnar::iter_from_slice::<AutoRow>(black_box(&bytes)).unwrap();
                        black_box(iter.map(|row| row.unwrap()).count())
                    })
                }),
            };
            group.finish();
        }
    }
}

criterion_group!(benches, encodings, containers, borrowed, small_batches);
criterion_main!(benches);
