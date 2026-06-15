use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    hmac::Hmac,
    poly1305::Poly1305,
    sha2::{Sha256, Sha512},
    sha3::Kmac256,
};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

const KEY: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
];

fn bench_macs(c: &mut Criterion) {
    let hmac_key = b"rust-stdx-crypto-bench-key";
    let customization = b"rust-stdx";

    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(size.to_string());
        group.throughput(Throughput::Bytes(size as u64));

        let data = vec![0xA3_u8; size];

        group.bench_with_input(BenchmarkId::from_parameter("HMAC-SHA256"), &data, |b, data| {
            b.iter(|| {
                let mut hmac = Hmac::<Sha256>::new(black_box(hmac_key));
                hmac.update(black_box(data.as_slice()));
                let _ = hmac.finalize();
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("HMAC-SHA512"), &data, |b, data| {
            b.iter(|| {
                let mut hmac = Hmac::<Sha512>::new(black_box(hmac_key));
                hmac.update(black_box(data.as_slice()));
                let _ = hmac.finalize();
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("KMAC256"), &data, |b, data| {
            b.iter(|| {
                let mut kmac = Kmac256::new(black_box(&KEY), black_box(customization));
                kmac.update(black_box(data.as_slice()));
                let mut out = [0u8; 32];
                kmac.finalize_into(&mut out);
                black_box(out);
            });
        });

        group.bench_with_input(BenchmarkId::from_parameter("Poly1305"), &data, |b, data| {
            b.iter(|| {
                let out = Poly1305::mac(&KEY, data);
                black_box(out);
            });
        });

        group.finish();
    }
}

criterion_group!(benches, bench_macs);
criterion_main!(benches);
