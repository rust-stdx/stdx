# Columnar benchmark results

Run from the benchmark's directory:

```sh
cd benchmarks/columnar
cargo bench --bench columnar -- --noplot
```

The suite uses 100,000 values per data set (50,000 for containers and enums).
The timing numbers below were collected on an Apple aarch64 VM (5 cores) with
Rust 1.100, Criterion (`--sample-size 15 --measurement-time 1`). Absolute
numbers vary by machine; regenerate them locally before comparing changes.

## Fixtures

- `sequential`: `0..100_000`.
- `regular_timestamps`: a fixed 1,000-unit step.
- `jittered_timestamps`: a 1,000-unit step with jitter in `-10..=10`.
- `blocky`: values repeated in blocks of 1,000 rows.
- `random`: uniformly random `u64` values from a deterministic generator.

`Auto` now runs a cheap statistics pass and can select `rle`, `delta_rle` and
`delta_of_delta` when the estimated payload is materially smaller. The explicit
columns still use the requested encoding; their automatic child columns may in
turn be compressed (for example, `rle` run values use `Auto`).

## Encoded sizes

Lower is better. Sizes are in bytes and are deterministic for a given build.

| Data set | `auto` | `rle` | `delta_rle` | `delta_of_delta` |
|---|---:|---:|---:|---:|
| sequential | **23** | 100,031 | **23** | 100,019 |
| regular timestamps | **29** | 100,037 | **29** | 100,025 |
| jittered timestamps | 100,025 | 200,033 | 285,584 | **100,025** |
| blocky | **226** | **226** | 518 | 100,019 |
| random | **800,017** | 900,025 | 1,055,875 | 975,389 |

Auto picks the smallest encoding on every fixture. Compare with the previous
default, which used packed/FOR only: sequential was 212,520 bytes, jittered
337,529, blocky 87,520 and random 800,017.

Mixed workloads (Auto):

| Data set | Size |
|---|---:|
| `TextRow` (id + unique name + 4-value tag) | 3,214,874 |
| `ContainerRow` (id + two lists + option + map) | 1,206,513 |
| `WideRow` (8 numeric fields) | 1,212,595 |
| `EnumRow` (id + 3-variant enum) | 135,477 |
| `BorrowedRow` (`Cow<str>`) | 2,620,023 |

## Selected timings

100,000 rows; time for the whole batch, with ns/value in parentheses.

| Benchmark | Before | After |
|---|---:|---:|
| decode sequential `auto` | 778 µs (7.8) | **351 µs (3.5)** |
| iterate sequential `auto` | 584 µs (5.8) | **259 µs (2.6)** |
| decode random `auto` | 317 µs (3.2) | **198 µs (2.0)** |
| decode jittered `auto` | 847 µs (8.5) | **374 µs (3.7)** |
| decode blocky `auto` | 760 µs (7.6) | **273 µs (2.7)** |
| encode random `auto` | 124 µs (1.2) | 270 µs (2.7) |
| encode sequential `auto` | 129 µs (1.3) | 469 µs (4.7) |
| encode blocky `auto` | 121 µs (1.2) | 382 µs (3.8) |

Auto makes encoding slower when it has to compute statistics and write a
run/delta encoding, in exchange for 3×–9,000× smaller output and faster decode.
Random data (the packed fast path) pays about 2× for the statistics pass.

Other Auto workloads:

| Benchmark | Encode | Decode | Iterate |
|---|---:|---:|---:|
| text | 2.11 ms | 9.43 ms | 8.15 ms |
| containers | 11.26 ms | 10.15 ms | 7.49 ms |
| wide | 3.41 ms | 2.32 ms | 2.02 ms |
| enums | 0.81 ms | 1.41 ms | 1.46 ms |
| borrowed text | 1.26 ms | 1.83 ms | 1.60 ms |

Small batches (AutoRow):

| Rows | Encode | Encode (reused buffer) | Decode | Iterate |
|---:|---:|---:|---:|---:|
| 1 | 24 ns | 16 ns | 83 ns | 44 ns |
| 10 | 74 ns | 46 ns | 123 ns | 83 ns |
| 100 | 641 ns | 574 ns | 330 ns | 316 ns |

## Allocation measurements

Measured with `alloc_counter` on 100,000 values:

| Operation | Before | After |
|---|---:|---:|
| encode 1×`u64` random | 2 calls, 4.0 MB | **1 call, 800 KB** |
| encode 1×`u64` sequential (Auto → delta RLE) | 2 calls, 3.4 MB | 2 calls, 817 KB |
| encode `String` lengths | 3 calls, 3.75 MB | **1 call, 800 KB** |
| encode `usize` | 3 calls, 4.2 MB | **2 calls, 817 KB** |
| encode `char` | 3 calls, 3.69 MB | **1 call, 400 KB** |
| decode string offsets | 100,003 calls, 6.05 MB | **100,002 calls, 5.25 MB** |
| encode into a reused buffer (second call) | — | **0 calls** |

## Notes

- `decode` materializes one `Vec` per column plus the row vector. Use
  `iterate` to stream rows without column vectors, or `Cow<'a, str>` fields to
  avoid per-value string allocations.
- RLE and dictionary columns materialize their run values or dictionary once
  per column, even on the lazy path.
- The encoder validates a column's metadata before reserving its output, and
  `DecodeOptions::max_decoded_bytes` (1 GiB by default) bounds one decoded
  container. A tiny declaration without a matching payload therefore fails
  before a large allocation.
