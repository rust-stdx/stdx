use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    Aead,
    aes::Aes256Gcm,
    ascon::AsconAead128,
    chacha::{ChaCha8Poly1305, ChaCha20Blake3, ChaCha20Poly1305},
};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

const KEY: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
];

const KEY_16: [u8; 16] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,
];

const NONCE_96: [u8; 12] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C];

const NONCE_128: [u8; 16] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
];

const NONCE_256: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
    0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
];

fn bench_encrypt(c: &mut Criterion) {
    let aes = Aes256Gcm::new(&KEY);
    let chacha20poly1305 = ChaCha20Poly1305::new(&KEY);
    let chacha8poly1305 = ChaCha8Poly1305::new(&KEY);
    let chacha_blake3 = ChaCha20Blake3::new(&KEY);
    let ascon = AsconAead128::new(&KEY_16);

    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(size.to_string());
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::from_parameter("AES-256-GCM-encrypt"), |b| {
            b.iter_batched(
                || vec![0xA5_u8; size],
                |mut data| {
                    let _tag = aes.encrypt_in_place(&mut data, &NONCE_96[..], &[]);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("ChaCha8-Poly1305-encrypt"), |b| {
            b.iter_batched(
                || vec![0xA5_u8; size],
                |mut data| {
                    let _tag = chacha8poly1305.encrypt_in_place(&mut data, &NONCE_96[..], &[]);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("ChaCha20-Poly1305-encrypt"), |b| {
            b.iter_batched(
                || vec![0xA5_u8; size],
                |mut data| {
                    let _tag = chacha20poly1305.encrypt_in_place(&mut data, &NONCE_96[..], &[]);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("ChaCha20-BLAKE3-encrypt"), |b| {
            b.iter_batched(
                || vec![0xA5_u8; size],
                |mut data| {
                    let _tag = chacha_blake3.encrypt_in_place(&mut data, &NONCE_256[..], &[]);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("Ascon-AEAD128-encrypt"), |b| {
            b.iter_batched(
                || vec![0xA5_u8; size],
                |mut data| {
                    let _tag = ascon.encrypt_in_place(&mut data, &NONCE_128[..], &[]);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

fn bench_decrypt(c: &mut Criterion) {
    let aes = Aes256Gcm::new(&KEY);
    let chacha8poly1305 = ChaCha8Poly1305::new(&KEY);
    let chacha20poly1305 = ChaCha20Poly1305::new(&KEY);
    let chacha_blake3 = ChaCha20Blake3::new(&KEY);
    let ascon = AsconAead128::new(&KEY_16);

    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(size.to_string());
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::from_parameter("AES-256-GCM-decrypt"), |b| {
            b.iter_batched(
                || {
                    let mut data = vec![0xA5_u8; size];
                    let tag = aes.encrypt_in_place(&mut data, &NONCE_96[..], &[]);
                    (data, tag)
                },
                |(mut data, tag)| {
                    let _result = aes.decrypt_in_place(&mut data, &NONCE_96[..], &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("ChaCha8-Poly1305-decrypt"), |b| {
            b.iter_batched(
                || {
                    let mut data = vec![0xA5_u8; size];
                    let tag = chacha8poly1305.encrypt_in_place(&mut data, &NONCE_96[..], &[]);
                    (data, tag)
                },
                |(mut data, tag)| {
                    let _result = chacha8poly1305.decrypt_in_place(&mut data, &NONCE_96[..], &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("ChaCha20-Poly1305-decrypt"), |b| {
            b.iter_batched(
                || {
                    let mut data = vec![0xA5_u8; size];
                    let tag = chacha20poly1305.encrypt_in_place(&mut data, &NONCE_96[..], &[]);
                    (data, tag)
                },
                |(mut data, tag)| {
                    let _result = chacha20poly1305.decrypt_in_place(&mut data, &NONCE_96[..], &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("ChaCha20-BLAKE3-decrypt"), |b| {
            b.iter_batched(
                || {
                    let mut data = vec![0xA5_u8; size];
                    let tag = chacha_blake3.encrypt_in_place(&mut data, &NONCE_256[..], &[]);
                    (data, tag)
                },
                |(mut data, tag)| {
                    let _result = chacha_blake3.decrypt_in_place(&mut data, &NONCE_256[..], &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("Ascon-AEAD128-decrypt"), |b| {
            b.iter_batched(
                || {
                    let mut data = vec![0xA5_u8; size];
                    let tag = ascon.encrypt_in_place(&mut data, &NONCE_128[..], &[]);
                    (data, tag)
                },
                |(mut data, tag)| {
                    let _result = ascon.decrypt_in_place(&mut data, &NONCE_128[..], &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

criterion_group!(benches, bench_encrypt, bench_decrypt);
criterion_main!(benches);
