# JSON benchmark results

Run from the benchmark's directory:

```sh
cd benchmarks/json
cargo bench --bench json -- --noplot
```

The benchmark uses `mimalloc` as the global allocator, matching production.
`serde_json` is built with its default features. Fixtures are generated in
`src/lib.rs`, so the benchmark is reproducible without downloading large files.
The results below are medians from the development container (aarch64, Rust
1.99, release, thin LTO). Shared-host benchmarks are noisy; run the command
locally before drawing conclusions about small differences.

| Workload | json | serde_json | simd-json | sonic-rs |
|---|---:|---:|---:|---:|
| Value parse twitter | 0.76 ms | 0.99 ms | 0.58 ms | 0.28 ms |
| Value parse canada | 1.75 ms | 1.82 ms | 1.11 ms | 0.59 ms |
| Value parse citm | 0.28 ms | 0.58 ms | 0.26 ms | 0.09 ms |
| Typed twitter | 0.52 ms | 0.52 ms | — | — |
| Typed canada | 0.93 ms | 0.85 ms | 0.61 ms | 0.56 ms |
| Serialize twitter Value | 0.44 ms | 0.36 ms | — | 0.34 ms |
| Serialize canada Value | 0.79 ms | 0.70 ms | — | 0.71 ms |
| Serialize citm Value | 0.17 ms | 0.12 ms | — | — |
| Validate twitter | 0.42 ms | — | — | — |
| Validate canada | 0.73 ms | — | — | — |
| Validate citm | 0.19 ms | — | — | — |

`json` now beats default `serde_json` on DOM parsing and is competitive on
serialization and typed decoding. The remaining DOM gap to `sonic-rs` is
primarily its arena-based DOM; this crate deliberately does not use an arena.

Current bottlenecks and follow-ups:

- DOM parsing still allocates one `Vec`/`Map` per container and one `String` per
  escaped string. This is inherent to the current `Value` representation.
- Typed string decoding and float parsing remain slightly behind `serde_json`
  on some fixtures.
- Full SIMD UTF-8 validation is now used for `from_slice`, `validate`,
  `Document`, and `RawValue`; non-ASCII inputs are substantially faster than
  the previous per-string fallback.
