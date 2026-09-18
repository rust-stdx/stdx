use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use crypto::argon2::{self, Params};

/// Memory in KiB and number of passes used for the benchmark. Large enough
/// that lane parallelism is not dominated by thread startup, yet still cheap
/// enough to run in CI.
const MEMORY_KIB: u32 = 65536;
const ITERATIONS: u32 = 1;

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

criterion_group!(benches, bench_argon2id);
criterion_main!(benches);
