use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::{
    argon2::{self, Params},
    pbkdf2,
    sha2::{Sha256, Sha512},
};

/// Memory in KiB and number of passes used for the benchmark. Large enough
/// that lane parallelism is not dominated by thread startup, yet still cheap
/// enough to run in CI.
const MEMORY_KIB: u32 = 65536;
const ITERATIONS: u32 = 1;

/// Iteration count used for the PBKDF2 benchmark, matching the OWASP
/// recommendation for PBKDF2-HMAC.
const PBKDF2_ITERATIONS: u32 = 600_000;

fn bench_argon2id(c: &mut Criterion) {
    let mut group = c.benchmark_group("Argon2id");
    group.throughput(Throughput::Bytes(MEMORY_KIB as u64 * 1024 * ITERATIONS as u64));

    let password = b"correct horse battery staple";
    let salt = b"randomsalt123456";

    for &parallelism in &[1u32, 2, 4] {
        let params = Params {
            iterations: ITERATIONS,
            memory: MEMORY_KIB,
            parallelism,
        };
        group.bench_with_input(BenchmarkId::from_parameter(format!("p={parallelism}")), &params, |b, params| {
            b.iter(|| {
                let mut out = [0u8; 32];
                argon2::derive_key(&mut out, black_box(password), black_box(salt), &[], &[], black_box(params))
                    .unwrap();
                black_box(out);
            });
        });
    }

    group.finish();
}

fn bench_pbkdf2(c: &mut Criterion) {
    let mut group = c.benchmark_group("PBKDF2");
    group.throughput(Throughput::Elements(PBKDF2_ITERATIONS as u64));

    let password = b"correct horse battery staple";
    let salt = b"randomsalt123456";

    group.bench_function("HMAC-SHA256", |b| {
        b.iter(|| {
            black_box(pbkdf2::derive::<Sha256, 32>(
                black_box(password),
                black_box(salt),
                black_box(PBKDF2_ITERATIONS),
            ));
        });
    });

    group.bench_function("HMAC-SHA512", |b| {
        b.iter(|| {
            black_box(pbkdf2::derive::<Sha512, 32>(
                black_box(password),
                black_box(salt),
                black_box(PBKDF2_ITERATIONS),
            ));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_argon2id, bench_pbkdf2);
criterion_main!(benches);
