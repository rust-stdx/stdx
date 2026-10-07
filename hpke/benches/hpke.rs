use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::aes::Aes256Gcm;
use hpke::{
    self, RecipientMode, SenderMode,
    kdf::HkdfSha256,
    kem::{Kem, MLKEM768X25519, P256HkdfSha256, X25519HkdfSha256},
};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024];

const INFO: &[u8] = b"HPKE benchmark";

fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("hpke-setup");

    let (_recipient_secret_key, recipient_public_key) = X25519HkdfSha256::generate_keypair().unwrap();
    group.bench_function(BenchmarkId::from_parameter("X25519-HKDF-SHA256-AES256GCM"), |b| {
        b.iter(|| {
            let _ = hpke::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
                &SenderMode::Base,
                &recipient_public_key,
                INFO,
            )
            .unwrap();
        });
    });

    let (_recipient_secret_key, recipient_public_key) = P256HkdfSha256::generate_keypair().unwrap();
    group.bench_function(BenchmarkId::from_parameter("P-256-HKDF-SHA256-AES256GCM"), |b| {
        b.iter(|| {
            let _ = hpke::new_sender::<P256HkdfSha256, HkdfSha256, Aes256Gcm>(
                &SenderMode::Base,
                &recipient_public_key,
                INFO,
            )
            .unwrap();
        });
    });

    let (_recipient_secret_key, recipient_public_key) = MLKEM768X25519::generate_keypair().unwrap();
    group.bench_function(BenchmarkId::from_parameter("MLKEM768-X25519-HKDF-SHA256-AES256GCM"), |b| {
        b.iter(|| {
            let _ = hpke::new_sender::<MLKEM768X25519, HkdfSha256, Aes256Gcm>(
                &SenderMode::Base,
                &recipient_public_key,
                INFO,
            )
            .unwrap();
        });
    });

    group.finish();
}

fn bench_seal(c: &mut Criterion) {
    for &size in DATA_SIZES {
        let mut group = c.benchmark_group(format!("hpke-seal-{size}"));
        group.throughput(Throughput::Bytes(size as u64));

        let (_recipient_secret_key, recipient_public_key) = X25519HkdfSha256::generate_keypair().unwrap();
        group.bench_function(BenchmarkId::from_parameter("X25519-HKDF-SHA256-AES256GCM"), |b| {
            b.iter_batched(
                || {
                    let (encapped_key, context) = hpke::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
                        &SenderMode::Base,
                        &recipient_public_key,
                        INFO,
                    )
                    .unwrap();
                    (encapped_key, context, vec![0xA5u8; size])
                },
                |(_encapped_key, mut context, mut data)| {
                    let _tag = context.seal_in_place(&mut data, b"associated data").unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });

        let (_recipient_secret_key, recipient_public_key) = MLKEM768X25519::generate_keypair().unwrap();
        group.bench_function(BenchmarkId::from_parameter("MLKEM768-X25519-HKDF-SHA256-AES256GCM"), |b| {
            b.iter_batched(
                || {
                    let (encapped_key, context) = hpke::new_sender::<MLKEM768X25519, HkdfSha256, Aes256Gcm>(
                        &SenderMode::Base,
                        &recipient_public_key,
                        INFO,
                    )
                    .unwrap();
                    (encapped_key, context, vec![0xA5u8; size])
                },
                |(_encapped_key, mut context, mut data)| {
                    let _tag = context.seal_in_place(&mut data, b"associated data").unwrap();
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

        let (recipient_secret_key, recipient_public_key) = X25519HkdfSha256::generate_keypair().unwrap();
        group.bench_function(BenchmarkId::from_parameter("X25519-HKDF-SHA256-AES256GCM"), |b| {
            b.iter_batched(
                || {
                    let (encapped_key, mut sender) = hpke::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
                        &SenderMode::Base,
                        &recipient_public_key,
                        INFO,
                    )
                    .unwrap();
                    let mut data = vec![0xA5u8; size];
                    let tag = sender.seal_in_place(&mut data, b"associated data").unwrap();
                    let recipient = hpke::new_recipient::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
                        &RecipientMode::Base,
                        &recipient_secret_key,
                        &encapped_key,
                        INFO,
                    )
                    .unwrap();
                    (recipient, data, tag)
                },
                |(mut recipient, mut data, tag)| {
                    recipient
                        .open_in_place(&mut data, b"associated data", tag.as_ref())
                        .unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

criterion_group!(benches, bench_setup, bench_seal, bench_open);
criterion_main!(benches);
