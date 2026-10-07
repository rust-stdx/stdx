use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use crypto::{
    Hasher, Xof,
    blake2::Blake2b,
    blake3::Blake3,
    hkdf,
    sha2::{Sha256, Sha512},
    sha3::Shake256,
};

/// 32-byte input key material used by every KDF benchmark.
const KEY: &[u8; 32] = b"rust-stdx-kdf-benchmark-key-0000";

/// Context / info string mixed into the derivation.
const INFO: &str = "rust-stdx crypto kdf benchmark";

/// Derives a 32-byte subkey from `key` and `info` using SHAKE256 with
/// length-prefixed domain separation.
fn derive_key<const N: usize>(key: &[u8], info: &str) -> [u8; N] {
    let mut kdf = Shake256::new();
    kdf.absorb(info.as_bytes());
    kdf.absorb(&(info.len() as u64).to_le_bytes());
    kdf.absorb(key);
    kdf.absorb(&(key.len() as u64).to_le_bytes());

    let mut subkey = [0u8; N];
    kdf.squeeze(&mut subkey);
    subkey
}

fn bench_kdfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("KDF");

    group.bench_function("SHAKE256", |b| {
        b.iter(|| black_box(derive_key::<32>(black_box(KEY), black_box(INFO))));
    });

    group.bench_function("HKDF-SHA256", |b| {
        b.iter(|| {
            black_box(hkdf::derive_key::<Sha256, 32>(black_box(KEY), INFO.as_bytes(), None).unwrap());
        });
    });

    group.bench_function("HKDF-SHA512", |b| {
        b.iter(|| {
            black_box(hkdf::derive_key::<Sha512, 32>(black_box(KEY), INFO.as_bytes(), None).unwrap());
        });
    });

    group.bench_function("BLAKE3", |b| {
        b.iter(|| black_box(Blake3::derive_key(black_box(INFO), black_box(KEY))));
    });

    group.bench_function("BLAKE2b", |b| {
        b.iter(|| {
            let mut hasher = Blake2b::new_keyed(black_box(KEY), 32);
            hasher.update(black_box(INFO.as_bytes()));
            black_box(hasher.sum());
        });
    });

    group.finish();
}

criterion_group!(benches, bench_kdfs);
criterion_main!(benches);
