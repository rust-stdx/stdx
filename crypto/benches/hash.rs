use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    Hasher, Xof,
    ascon::AsconHash256,
    blake2::Blake2b,
    blake3::Blake3,
    sha2::{Sha256, Sha512},
    sha3::{
        Kt256, ParallelHash256, Sha3_256, Sha3_512, Shake256, TurboShake256, keccak_backend_name, kt256_backend_name,
        parallel_hash_backend_name,
    },
};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024, 10 * 1024 * 1024];

fn bench_hashes(c: &mut Criterion) {
    // Printed once so CI logs record which kernels the numbers measure.
    // `ParallelHash` at B=8192 should land near `width` times the SHAKE256
    // throughput; if it does not, the batched backend is not doing its job.
    eprintln!("Keccak single-lane backend: {}", keccak_backend_name());
    eprintln!("ParallelHash backend: {}", parallel_hash_backend_name());
    eprintln!("KT256 backend: {}", kt256_backend_name());

    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(size.to_string());
        group.throughput(Throughput::Bytes(size as u64));

        let data = vec![0xA5_u8; size];

        group.bench_with_input(BenchmarkId::from_parameter("SHA-256"), &data, |b, data| {
            b.iter(|| {
                let _ = Sha256::hash(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("SHA-512"), &data, |b, data| {
            b.iter(|| {
                let _ = Sha512::hash(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("SHA3-256"), &data, |b, data| {
            b.iter(|| {
                let _ = Sha3_256::hash(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("SHA3-512"), &data, |b, data| {
            b.iter(|| {
                let _ = Sha3_512::hash(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("SHAKE256-64"), &data, |b, data| {
            b.iter(|| {
                let _ = <Shake256 as Hasher>::hash(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("TurboSHAKE256-64"), &data, |b, data| {
            b.iter(|| {
                let _ = <TurboShake256 as Hasher>::hash(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("KT256"), &data, |b, data| {
            b.iter(|| {
                let mut out = [0u8; 32];
                Kt256::hash(black_box(data.as_slice()), b"", &mut out);
                black_box(out);
            });
        });

        // ParallelHash256 with the most common block sizes.
        for (label, block_size) in [("B1024", 1024usize), ("B4096", 4096), ("B8192", 8192)] {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("ParallelHash256-{label}")),
                &data,
                |b, data| {
                    b.iter(|| {
                        let mut out = [0u8; 32];
                        ParallelHash256::hash(black_box(data.as_slice()), black_box(block_size), b"", &mut out);
                        black_box(out);
                    });
                },
            );
        }

        group.bench_with_input(BenchmarkId::from_parameter("BLAKE3"), &data, |b, data| {
            b.iter(|| {
                let _ = Blake3::hash(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("BLAKE2b-512"), &data, |b, data| {
            b.iter(|| {
                let _ = Blake2b::hash(black_box(data.as_slice()));
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("Ascon-Hash256"), &data, |b, data| {
            b.iter(|| {
                let _ = AsconHash256::hash(black_box(data.as_slice()));
            });
        });

        group.finish();
    }
}

/// Extended-output throughput: `Blake3::hash` only produces 32 bytes, but the
/// XOF reader can squeeze long streams, where the SIMD backends compress
/// several 64-byte blocks at once.
fn bench_blake3_xof(c: &mut Criterion) {
    const OUT_SIZES: &[usize] = &[64, 1024, 16 * 1024, 1024 * 1024];

    let input = vec![0xA5_u8; 1024];
    let mut group = c.benchmark_group("blake3-xof");

    for &size in OUT_SIZES {
        let mut out = vec![0u8; size];
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                let mut hasher = Blake3::new();
                hasher.update(black_box(input.as_slice()));
                let mut xof = hasher.finalize_xof();
                xof.squeeze(black_box(&mut out));
            });
        });
    }

    group.finish();
}

/// Squeezing a long stream in chunks: the reader buffers the unread part of the
/// current 64-byte output block, so small reads must not repeat compression
/// work, while whole blocks still go through the wide SIMD path.
fn bench_blake3_xof_chunked(c: &mut Criterion) {
    const TOTAL: usize = 64 * 1024;
    const CHUNKS: &[usize] = &[1, 32, 33, 63, 64, 65, 128, 1024, TOTAL];

    let input = vec![0xA5_u8; 1024];
    let mut group = c.benchmark_group("blake3-xof-chunked");
    group.throughput(Throughput::Bytes(TOTAL as u64));

    for &chunk in CHUNKS {
        let mut out = vec![0u8; chunk];
        group.bench_with_input(BenchmarkId::from_parameter(chunk), &chunk, |b, _| {
            b.iter(|| {
                let mut hasher = Blake3::new();
                hasher.update(black_box(input.as_slice()));
                let mut xof = hasher.finalize_xof();
                let mut left = TOTAL;
                while left > 0 {
                    let n = chunk.min(left);
                    xof.squeeze(black_box(&mut out[..n]));
                    left -= n;
                }
            });
        });
    }

    group.finish();
}

/// One-shot squeezes, including the 72-byte stream ChaCha-BLAKE3 derives per
/// message: buffering the reader must add no measurable overhead here.
fn bench_blake3_xof_oneshot(c: &mut Criterion) {
    const OUT_SIZES: &[usize] = &[32, 64, 72, 96, 128];

    let input = vec![0xA5_u8; 64];
    let mut group = c.benchmark_group("blake3-xof-oneshot");

    for &size in OUT_SIZES {
        let mut out = vec![0u8; size];
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                let mut hasher = Blake3::new();
                hasher.update(black_box(input.as_slice()));
                hasher.finalize_xof().squeeze(black_box(&mut out));
            });
        });
    }

    group.finish();
}

/// Extended-output throughput for TurboSHAKE256: a fixed short input, but a
/// long squeezed stream, to show the cost of the squeeze phase on its own.
fn bench_turboshake_xof(c: &mut Criterion) {
    const OUT_SIZES: &[usize] = &[64, 1024, 16 * 1024, 1024 * 1024];

    let input = vec![0xA5_u8; 1024];
    let mut group = c.benchmark_group("turboshake256-xof");

    for &size in OUT_SIZES {
        let mut out = vec![0u8; size];
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                let mut xof = TurboShake256::default();
                xof.absorb(black_box(input.as_slice()));
                xof.squeeze(black_box(&mut out));
            });
        });
    }

    group.finish();
}

/// Extended-output throughput for KT256: a fixed long input and a long squeezed
/// stream, so the numbers include at least one leaf and the final node.
fn bench_kt256_xof(c: &mut Criterion) {
    const OUT_SIZES: &[usize] = &[64, 1024, 16 * 1024, 1024 * 1024];

    let input = vec![0xA5_u8; 64 * 1024];
    let mut group = c.benchmark_group("kt256-xof");

    for &size in OUT_SIZES {
        let mut out = vec![0u8; size];
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                let mut hasher = Kt256::new(b"");
                hasher.update(black_box(input.as_slice()));
                let mut reader = hasher.finalize_xof();
                reader.squeeze(black_box(&mut out));
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_hashes,
    bench_blake3_xof,
    bench_blake3_xof_chunked,
    bench_blake3_xof_oneshot,
    bench_turboshake_xof,
    bench_kt256_xof
);
criterion_main!(benches);
