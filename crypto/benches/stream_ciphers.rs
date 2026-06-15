use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{StreamCipher, aes::Aes256Ctr, chacha::ChaCha20Djb};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

const KEY: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
];

const NONCE_8: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

fn bench_stream_ciphers(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(size.to_string());
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::from_parameter("AES-256-CTR"), |b| {
            b.iter_batched(
                || {
                    let cipher = Aes256Ctr::new(&KEY);
                    (cipher, vec![0xA5_u8; size])
                },
                |(mut cipher, mut data)| {
                    cipher.xor_keystream(std::hint::black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::from_parameter("ChaCha20"), |b| {
            b.iter_batched(
                || {
                    let cipher = ChaCha20Djb::new(&KEY, &NONCE_8);
                    (cipher, vec![0xA5_u8; size])
                },
                |(mut cipher, mut data)| {
                    cipher.xor_keystream(std::hint::black_box(&mut data));
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

criterion_group!(benches, bench_stream_ciphers);
criterion_main!(benches);
