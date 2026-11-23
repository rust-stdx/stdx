#![allow(deprecated)]
#![allow(clippy::unit_arg)]

use std::hint::black_box;

use aes_gcm::{
    Aes128Gcm as RcAes128Gcm, Aes256Gcm as RcAes256Gcm,
    aead::{AeadInOut, KeyInit, Nonce},
};
use ascon_aead128::AsconAead128 as RcAsconAead128;
use aws_lc_rs::aead::{AES_128_GCM, AES_256_GCM, Aad, CHACHA20_POLY1305, LessSafeKey, UnboundKey};
use chacha20poly1305::ChaCha20Poly1305 as RcChaCha20Poly1305;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    Aead,
    aes::{Aes128Gcm, Aes256Gcm},
    ascon::AsconAead128,
    chacha::ChaCha20Poly1305,
};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

const KEY_16: [u8; 16] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,
];

const KEY_32: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
];

const NONCE_96: [u8; 12] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C];

const NONCE_128: [u8; 16] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
];

fn bench_aes256_gcm(c: &mut Criterion) {
    let aes256 = Aes256Gcm::new(&KEY_32);

    let unbound = UnboundKey::new(&AES_256_GCM, &KEY_32).unwrap();
    let sealing = LessSafeKey::new(unbound);

    let rc_aes256 = RcAes256Gcm::new_from_slice(&KEY_32).unwrap();
    let rc_nonce = <&Nonce<RcAes256Gcm>>::try_from(&NONCE_96[..]).unwrap();

    let mut group = c.benchmark_group("AES-256-GCM/encrypt");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    let _tag = aes256.encrypt_in_place(&mut buf, &NONCE_96, &[]).unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("aws-lc-rs", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    let _tag = sealing
                        .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                        .unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    black_box(
                        rc_aes256
                            .encrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("AES-256-GCM/decrypt");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let tag = aes256.encrypt_in_place(&mut buf, &NONCE_96, &[]).unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    let _result = aes256.decrypt_in_place(&mut buf, &NONCE_96, &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("aws-lc-rs", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    let tag = sealing
                        .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                        .unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    black_box(
                        sealing
                            .open_in_place_separate_tag(nonce, Aad::empty(), tag.as_ref(), &mut buf)
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let tag = rc_aes256
                        .encrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                        .unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    black_box(
                        rc_aes256
                            .decrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()), &tag)
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_aes128_gcm(c: &mut Criterion) {
    let aes128 = Aes128Gcm::new(&KEY_16);

    let unbound = UnboundKey::new(&AES_128_GCM, &KEY_16).unwrap();
    let sealing = LessSafeKey::new(unbound);

    let rc_aes128 = RcAes128Gcm::new_from_slice(&KEY_16).unwrap();
    let rc_nonce = <&Nonce<RcAes128Gcm>>::try_from(&NONCE_96[..]).unwrap();

    let mut group = c.benchmark_group("AES-128-GCM/encrypt");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    let _tag = aes128.encrypt_in_place(&mut buf, &NONCE_96, &[]).unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("aws-lc-rs", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    let _tag = sealing
                        .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                        .unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    black_box(
                        rc_aes128
                            .encrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("AES-128-GCM/decrypt");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let tag = aes128.encrypt_in_place(&mut buf, &NONCE_96, &[]).unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    let _result = aes128.decrypt_in_place(&mut buf, &NONCE_96, &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("aws-lc-rs", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    let tag = sealing
                        .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                        .unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    black_box(
                        sealing
                            .open_in_place_separate_tag(nonce, Aad::empty(), tag.as_ref(), &mut buf)
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let tag = rc_aes128
                        .encrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                        .unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    black_box(
                        rc_aes128
                            .decrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()), &tag)
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_chacha20_poly1305(c: &mut Criterion) {
    let chacha = ChaCha20Poly1305::new(&KEY_32);

    let unbound = UnboundKey::new(&CHACHA20_POLY1305, &KEY_32).unwrap();
    let sealing = LessSafeKey::new(unbound);

    let rc_chacha = RcChaCha20Poly1305::new_from_slice(&KEY_32).unwrap();
    let rc_nonce = <&Nonce<RcChaCha20Poly1305>>::try_from(&NONCE_96[..]).unwrap();

    let mut group = c.benchmark_group("ChaCha20-Poly1305/encrypt");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    let _tag = chacha.encrypt_in_place(&mut buf, &NONCE_96, &[]).unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("aws-lc-rs", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    let _tag = sealing
                        .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                        .unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    black_box(
                        rc_chacha
                            .encrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("ChaCha20-Poly1305/decrypt");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let tag = chacha.encrypt_in_place(&mut buf, &NONCE_96, &[]).unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    let _result = chacha.decrypt_in_place(&mut buf, &NONCE_96, &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("aws-lc-rs", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    let tag = sealing
                        .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                        .unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    let nonce = aws_lc_rs::aead::Nonce::assume_unique_for_key(NONCE_96);
                    black_box(
                        sealing
                            .open_in_place_separate_tag(nonce, Aad::empty(), tag.as_ref(), &mut buf)
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let tag = rc_chacha
                        .encrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                        .unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    black_box(
                        rc_chacha
                            .decrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()), &tag)
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ascon_aead128(c: &mut Criterion) {
    let ascon = AsconAead128::new(&KEY_16);

    let rc_ascon = RcAsconAead128::new_from_slice(&KEY_16).unwrap();
    let rc_nonce = <&Nonce<RcAsconAead128>>::try_from(&NONCE_128[..]).unwrap();

    let mut group = c.benchmark_group("Ascon-AEAD128/encrypt");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    let _tag = ascon.encrypt_in_place(&mut buf, &NONCE_128, &[]).unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || vec![0xA5u8; size],
                |mut buf| {
                    black_box(
                        rc_ascon
                            .encrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Ascon-AEAD128/decrypt");
    for &size in DATA_SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("stdx-crypto", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let tag = ascon.encrypt_in_place(&mut buf, &NONCE_128, &[]).unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    let _result = ascon.decrypt_in_place(&mut buf, &NONCE_128, &[], tag.as_ref());
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("RustCrypto", size), |b| {
            b.iter_batched(
                || {
                    let mut buf = vec![0xA5u8; size];
                    let tag = rc_ascon
                        .encrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()))
                        .unwrap();
                    (buf, tag)
                },
                |(mut buf, tag)| {
                    black_box(
                        rc_ascon
                            .decrypt_inout_detached(rc_nonce, b"", inout::InOutBuf::from(buf.as_mut_slice()), &tag)
                            .unwrap(),
                    );
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_aes256_gcm,
    bench_aes128_gcm,
    bench_chacha20_poly1305,
    bench_ascon_aead128
);
criterion_main!(benches);
