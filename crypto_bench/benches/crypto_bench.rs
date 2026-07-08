#![allow(deprecated)]

use std::hint::black_box;

use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};

const DATA_SIZE: usize = 256_000;
const KEY_32: [u8; 32] = [0x42; 32];
const NONCE_12: [u8; 12] = [0; 12];

// ---- aws-lc-rs ----

fn bench_aws_lc_rs_aes256_gcm(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    data: &[u8],
) {
    use aws_lc_rs::aead::{AES_256_GCM, Aad, LessSafeKey, UnboundKey};

    let unbound = UnboundKey::new(&AES_256_GCM, &KEY_32).unwrap();
    let sealing = LessSafeKey::new(unbound);

    group.bench_function("aws-lc-rs", |b| {
        b.iter_batched(
            || data.to_vec(),
            |mut buf| {
                let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_12);
                let _tag = sealing
                    .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                    .unwrap();
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_aws_lc_rs_chacha20_poly1305(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    data: &[u8],
) {
    use aws_lc_rs::aead::{Aad, CHACHA20_POLY1305, LessSafeKey, UnboundKey};

    let unbound = UnboundKey::new(&CHACHA20_POLY1305, &KEY_32).unwrap();
    let sealing = LessSafeKey::new(unbound);

    group.bench_function("aws-lc-rs", |b| {
        b.iter_batched(
            || data.to_vec(),
            |mut buf| {
                let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_12);
                let _tag = sealing
                    .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                    .unwrap();
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_aws_lc_rs_sha256(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>, data: &[u8]) {
    use aws_lc_rs::digest::{SHA256, digest};

    group.bench_function("aws-lc-rs", |b| {
        b.iter(|| {
            let _ = black_box(digest(&SHA256, black_box(data)));
        });
    });
}

fn bench_aws_lc_rs_sha512(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>, data: &[u8]) {
    use aws_lc_rs::digest::{SHA512, digest};

    group.bench_function("aws-lc-rs", |b| {
        b.iter(|| {
            let _ = black_box(digest(&SHA512, black_box(data)));
        });
    });
}

// ---- RustCrypto ----

fn bench_rustcrypto_aes256_gcm(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    data: &[u8],
) {
    use aes_gcm::{
        Aes256Gcm,
        aead::{AeadInOut, KeyInit, Nonce},
    };

    let cipher = Aes256Gcm::new_from_slice(&KEY_32).unwrap();
    let nonce = <&Nonce<Aes256Gcm>>::try_from(&NONCE_12[..]).unwrap();

    group.bench_function("RustCrypto", |b| {
        b.iter_batched(
            || data.to_vec(),
            |mut buf| {
                let _tag = cipher
                    .encrypt_inout_detached(nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                    .unwrap();
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_rustcrypto_chacha20_poly1305(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    data: &[u8],
) {
    use chacha20poly1305::{
        ChaCha20Poly1305,
        aead::{AeadInOut, KeyInit, Nonce},
    };

    let cipher = ChaCha20Poly1305::new_from_slice(&KEY_32).unwrap();
    let nonce = <&Nonce<ChaCha20Poly1305>>::try_from(&NONCE_12[..]).unwrap();

    group.bench_function("RustCrypto", |b| {
        b.iter_batched(
            || data.to_vec(),
            |mut buf| {
                let _tag = cipher
                    .encrypt_inout_detached(nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                    .unwrap();
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_rustcrypto_sha256(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>, data: &[u8]) {
    use sha2::{Digest, Sha256};

    group.bench_function("RustCrypto", |b| {
        b.iter(|| {
            let _ = black_box(Sha256::digest(black_box(data)));
        });
    });
}

fn bench_rustcrypto_sha512(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>, data: &[u8]) {
    use sha2::{Digest, Sha512};

    group.bench_function("RustCrypto", |b| {
        b.iter(|| {
            let _ = black_box(Sha512::digest(black_box(data)));
        });
    });
}

// ---- stdx-crypto ----

fn bench_stdx_aes256_gcm(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>, data: &[u8]) {
    use crypto::{Aead, aes::Aes256Gcm};

    let cipher = Aes256Gcm::new(&KEY_32);

    group.bench_function("stdx-crypto", |b| {
        b.iter_batched(
            || data.to_vec(),
            |mut buf| {
                let _tag = cipher.encrypt_in_place(&mut buf, &NONCE_12, &[]);
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_stdx_chacha20_poly1305(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    data: &[u8],
) {
    use crypto::{Aead, chacha::ChaCha20Poly1305};

    let cipher = ChaCha20Poly1305::new(&KEY_32);

    group.bench_function("stdx-crypto", |b| {
        b.iter_batched(
            || data.to_vec(),
            |mut buf| {
                let _tag = cipher.encrypt_in_place(&mut buf, &NONCE_12, &[]);
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_stdx_sha256(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>, data: &[u8]) {
    use crypto::{Hasher, sha2::Sha256};

    group.bench_function("stdx-crypto", |b| {
        b.iter(|| {
            let _ = black_box(Sha256::hash(black_box(data)));
        });
    });
}

fn bench_stdx_sha512(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>, data: &[u8]) {
    use crypto::{Hasher, sha2::Sha512};

    group.bench_function("stdx-crypto", |b| {
        b.iter(|| {
            let _ = black_box(Sha512::hash(black_box(data)));
        });
    });
}

// ---- main bench ----

fn bench(c: &mut Criterion) {
    let data = vec![0xA5u8; DATA_SIZE];

    let mut group = c.benchmark_group("AES-256-GCM");
    group.throughput(Throughput::Bytes(DATA_SIZE as u64));
    bench_aws_lc_rs_aes256_gcm(&mut group, &data);
    bench_rustcrypto_aes256_gcm(&mut group, &data);
    bench_stdx_aes256_gcm(&mut group, &data);
    group.finish();

    let mut group = c.benchmark_group("ChaCha20Poly1305");
    group.throughput(Throughput::Bytes(DATA_SIZE as u64));
    bench_aws_lc_rs_chacha20_poly1305(&mut group, &data);
    bench_rustcrypto_chacha20_poly1305(&mut group, &data);
    bench_stdx_chacha20_poly1305(&mut group, &data);
    group.finish();

    let mut group = c.benchmark_group("SHA-256");
    group.throughput(Throughput::Bytes(DATA_SIZE as u64));
    bench_aws_lc_rs_sha256(&mut group, &data);
    bench_rustcrypto_sha256(&mut group, &data);
    bench_stdx_sha256(&mut group, &data);
    group.finish();

    let mut group = c.benchmark_group("SHA-512");
    group.throughput(Throughput::Bytes(DATA_SIZE as u64));
    bench_aws_lc_rs_sha512(&mut group, &data);
    bench_rustcrypto_sha512(&mut group, &data);
    bench_stdx_sha512(&mut group, &data);
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
