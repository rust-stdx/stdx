use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::aes::Aes256Gcm;
use hpke::{
    self, ModeReceiver, ModeSender,
    kdf::HkdfSha256,
    kem::{Kem, P256HkdfSha256, X25519HkdfSha256, XWing},
};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024];

const INFO: &[u8] = b"HPKE benchmark";

fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("hpke-setup");

    let (_sk, pk) = X25519HkdfSha256::generate_keypair().unwrap();
    group.bench_function(BenchmarkId::from_parameter("X25519-HKDF-SHA256-AES256GCM"), |b| {
        b.iter(|| {
            let _ = hpke::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk, INFO).unwrap();
        });
    });

    let (_sk, pk) = P256HkdfSha256::generate_keypair().unwrap();
    group.bench_function(BenchmarkId::from_parameter("P-256-HKDF-SHA256-AES256GCM"), |b| {
        b.iter(|| {
            let _ = hpke::new_sender::<P256HkdfSha256, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk, INFO).unwrap();
        });
    });

    let (_sk, pk) = XWing::generate_keypair().unwrap();
    group.bench_function(BenchmarkId::from_parameter("X-Wing-HKDF-SHA256-AES256GCM"), |b| {
        b.iter(|| {
            let _ = hpke::new_sender::<XWing, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk, INFO).unwrap();
        });
    });

    group.finish();
}

fn bench_seal(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("hpke-seal-{size}"));
        group.throughput(Throughput::Bytes(size as u64));

        let (_sk, pk) = X25519HkdfSha256::generate_keypair().unwrap();
        group.bench_function(BenchmarkId::from_parameter("X25519-HKDF-SHA256-AES256GCM"), |b| {
            b.iter_batched(
                || {
                    let (enc, context) =
                        hpke::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk, INFO)
                            .unwrap();
                    (enc, context, vec![0xA5u8; size])
                },
                |(_enc, mut context, mut data)| {
                    let _tag = context.seal_in_place(&mut data, b"aad").unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });

        let (_sk, pk) = XWing::generate_keypair().unwrap();
        group.bench_function(BenchmarkId::from_parameter("X-Wing-HKDF-SHA256-AES256GCM"), |b| {
            b.iter_batched(
                || {
                    let (enc, context) =
                        hpke::new_sender::<XWing, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk, INFO).unwrap();
                    (enc, context, vec![0xA5u8; size])
                },
                |(_enc, mut context, mut data)| {
                    let _tag = context.seal_in_place(&mut data, b"aad").unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

fn bench_open(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("hpke-open-{size}"));
        group.throughput(Throughput::Bytes(size as u64));

        let (sk, pk) = X25519HkdfSha256::generate_keypair().unwrap();
        group.bench_function(BenchmarkId::from_parameter("X25519-HKDF-SHA256-AES256GCM"), |b| {
            b.iter_batched(
                || {
                    let (enc, mut sender) =
                        hpke::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk, INFO)
                            .unwrap();
                    let mut data = vec![0xA5u8; size];
                    let tag = sender.seal_in_place(&mut data, b"aad").unwrap();
                    let receiver = hpke::new_receiver::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
                        &ModeReceiver::Base,
                        &sk,
                        &enc,
                        INFO,
                    )
                    .unwrap();
                    (receiver, data, tag)
                },
                |(mut receiver, mut data, tag)| {
                    receiver.open_in_place(&mut data, b"aad", tag.as_ref()).unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

criterion_group!(benches, bench_setup, bench_seal, bench_open);
criterion_main!(benches);
