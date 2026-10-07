use std::hint::black_box;

use base64::{
    Alphabet, decode_into, decode_into_constant_time, encode, encode_into, encode_into_constant_time, encoded_length,
};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

const DATA_SIZES: &[usize] = &[32, 256, 4096, 64 * 1024];

fn bench_encode(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("encode/{size}"));
        group.throughput(Throughput::Bytes(size as u64));

        let data = vec![0xABu8; size];
        let mut encoded = vec![0u8; encoded_length(data.len(), true).expect("encoded length overflow")];

        group.bench_with_input(BenchmarkId::from_parameter("scalar (constant time)"), &data, |b, data| {
            b.iter(|| encode_into_constant_time(black_box(&mut encoded), black_box(data), Alphabet::Standard));
        });
        group.bench_with_input(BenchmarkId::from_parameter("SIMD"), &data, |b, data| {
            b.iter(|| encode_into(black_box(&mut encoded), black_box(data), Alphabet::Standard));
        });

        group.finish();
    }
}

fn bench_decode(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("decode/{size}"));
        group.throughput(Throughput::Bytes(size as u64));

        let mut data = vec![0xABu8; size];
        let encoded = encode(&data, Alphabet::Standard);

        group.bench_with_input(
            BenchmarkId::from_parameter("scalar (constant time)"),
            encoded.as_bytes(),
            |b, encoded| {
                b.iter(|| {
                    decode_into_constant_time(black_box(&mut data), black_box(encoded), Alphabet::Standard).unwrap()
                });
            },
        );
        group.bench_with_input(BenchmarkId::from_parameter("SIMD"), encoded.as_bytes(), |b, encoded| {
            b.iter(|| decode_into(black_box(&mut data), black_box(encoded), Alphabet::Standard));
        });

        group.finish();
    }
}

criterion_group!(benches, bench_encode, bench_decode);

criterion_main!(benches);
