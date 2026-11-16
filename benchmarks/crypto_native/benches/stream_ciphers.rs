#![allow(deprecated)]

use std::hint::black_box;

use aes::Aes256;
use chacha20::{ChaCha8, ChaCha12, ChaCha20};
use cipher::{KeyIvInit, StreamCipher as RustCryptoStreamCipher};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    StreamCipher,
    aes::Aes256Ctr,
    chacha::{ChaCha8Djb, ChaCha12Djb, ChaCha20Djb},
};
use ctr::Ctr128BE;

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

const KEY: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
];

const NONCE_8: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
const NONCE_12: [u8; 12] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C];
const IV_16: [u8; 16] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
];

type Aes256CtrRustCrypto = Ctr128BE<Aes256>;

fn bench_stream_ciphers(c: &mut Criterion) {
    let mut group = c.benchmark_group("AES-256-CTR");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || (Aes256Ctr::new(&KEY), vec![0xA5u8; size]),
                |(mut cipher, mut data)| {
                    cipher.xor_keystream(black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || {
                    let cipher = Aes256CtrRustCrypto::new_from_slices(&KEY, &IV_16).unwrap();
                    (cipher, vec![0xA5u8; size])
                },
                |(mut cipher, mut data)| {
                    cipher.apply_keystream(black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("ChaCha8");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || (ChaCha8Djb::new(&KEY, &NONCE_8), vec![0xA5u8; size]),
                |(mut cipher, mut data)| {
                    cipher.xor_keystream(black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || {
                    let cipher = ChaCha8::new_from_slices(&KEY, &NONCE_12).unwrap();
                    (cipher, vec![0xA5u8; size])
                },
                |(mut cipher, mut data)| {
                    cipher.apply_keystream(black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("ChaCha12");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || (ChaCha12Djb::new(&KEY, &NONCE_8), vec![0xA5u8; size]),
                |(mut cipher, mut data)| {
                    cipher.xor_keystream(black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || {
                    let cipher = ChaCha12::new_from_slices(&KEY, &NONCE_12).unwrap();
                    (cipher, vec![0xA5u8; size])
                },
                |(mut cipher, mut data)| {
                    cipher.apply_keystream(black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("ChaCha20");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || (ChaCha20Djb::new(&KEY, &NONCE_8), vec![0xA5u8; size]),
                |(mut cipher, mut data)| {
                    cipher.xor_keystream(black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || {
                    let cipher = ChaCha20::new_from_slices(&KEY, &NONCE_12).unwrap();
                    (cipher, vec![0xA5u8; size])
                },
                |(mut cipher, mut data)| {
                    cipher.apply_keystream(black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_stream_ciphers);
criterion_main!(benches);
