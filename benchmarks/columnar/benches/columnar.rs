use std::hint::black_box;

use columnar::{Columnar, ColumnarDecode, ColumnarIter};
use columnar_benchmarks::{
    AutoRow, DeltaOfDeltaRow, DeltaRleRow, RleRow, blocky, jittered_timestamps, random, regular_timestamps, sequential,
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

criterion_group!(benches, encodings);
criterion_main!(benches);
