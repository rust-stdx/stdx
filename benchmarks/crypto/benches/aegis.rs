use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::aegis::{Aegis256Mac, Aegis256Stream, Aegis256X2Mac, Aegis256X2Stream, Aegis256X4Mac, Aegis256X4Stream};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

const KEY: [u8; 32] = [0x40; 32];
const NONCE: [u8; 32] = [0x01; 32];

fn bench_mac(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("aegis-mac-{size}"));
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0xa5u8; size];

        group.bench_with_input(BenchmarkId::from_parameter("AEGISMAC-256"), &data, |b, data| {
            b.iter(|| {
                let mut mac = Aegis256Mac::<16>::new_with_nonce(&KEY, &NONCE);
                mac.update(black_box(data));
                black_box(mac.finalize())
            });
        });
        group.bench_with_input(BenchmarkId::from_parameter("AEGISMAC-256X2"), &data, |b, data| {
            b.iter(|| {
                let mut mac = Aegis256X2Mac::<16>::new_with_nonce(&KEY, &NONCE);
                mac.update(black_box(data));
                black_box(mac.finalize())
            });
        });
        group.bench_with_input(BenchmarkId::from_parameter("AEGISMAC-256X4"), &data, |b, data| {
            b.iter(|| {
                let mut mac = Aegis256X4Mac::<16>::new_with_nonce(&KEY, &NONCE);
                mac.update(black_box(data));
                black_box(mac.finalize())
            });
        });
        group.bench_with_input(BenchmarkId::from_parameter("AEGISMAC-256-256-bit"), &data, |b, data| {
            b.iter(|| {
                let mut mac = Aegis256Mac::<32>::new_with_nonce(&KEY, &NONCE);
                mac.update(black_box(data));
                black_box(mac.finalize())
            });
        });
        group.finish();
    }
}

fn bench_stream(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("aegis-stream-{size}"));
        group.throughput(Throughput::Bytes(size as u64));
        let mut buffer = vec![0u8; size];

        group.bench_function(BenchmarkId::from_parameter("AEGIS-256"), |b| {
            b.iter(|| {
                let mut stream = Aegis256Stream::new(&KEY, &NONCE);
                stream.fill_keystream(black_box(&mut buffer));
                black_box(buffer[0])
            });
        });
        group.bench_function(BenchmarkId::from_parameter("AEGIS-256X2"), |b| {
            b.iter(|| {
                let mut stream = Aegis256X2Stream::new(&KEY, &NONCE);
                stream.fill_keystream(black_box(&mut buffer));
                black_box(buffer[0])
            });
        });
        group.bench_function(BenchmarkId::from_parameter("AEGIS-256X4"), |b| {
            b.iter(|| {
                let mut stream = Aegis256X4Stream::new(&KEY, &NONCE);
                stream.fill_keystream(black_box(&mut buffer));
                black_box(buffer[0])
            });
        });
        group.finish();
    }
}

fn bench_chunked(c: &mut Criterion) {
    const SIZE: usize = 1024 * 1024;
    let data = vec![0xa5u8; SIZE];
    let mut group = c.benchmark_group("aegis-chunked");
    group.throughput(Throughput::Bytes(SIZE as u64));

    for &chunk in &[16usize, 64, 1024] {
        group.bench_with_input(BenchmarkId::new("AEGISMAC-256", chunk), &chunk, |b, &chunk| {
            b.iter(|| {
                let mut mac = Aegis256Mac::<16>::new_with_nonce(&KEY, &NONCE);
                for piece in data.chunks(chunk) {
                    mac.update(piece);
                }
                black_box(mac.finalize())
            });
        });
        group.bench_with_input(BenchmarkId::new("AEGIS-256-stream", chunk), &chunk, |b, &chunk| {
            let mut buffer = vec![0u8; SIZE];
            b.iter(|| {
                let mut stream = Aegis256Stream::new(&KEY, &NONCE);
                for piece in buffer.chunks_mut(chunk) {
                    stream.fill_keystream(piece);
                }
                black_box(buffer[0])
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_mac, bench_stream, bench_chunked);
criterion_main!(benches);
