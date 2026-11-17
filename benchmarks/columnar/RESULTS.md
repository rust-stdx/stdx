# Columnar benchmark results

Run from the benchmark's directory:

```sh
cd benchmarks/columnar
cargo bench --bench columnar -- --noplot
```

The benchmark suite uses 100,000 `u64` values per data set and compares the
default `Auto` encoding with `rle`, `delta_rle` and `delta_of_delta`.

## Fixtures

- `sequential`: `0..100_000`.
- `regular_timestamps`: a fixed 1,000-unit step.
- `jittered_timestamps`: a 1,000-unit step with jitter in `-10..=10`.
- `blocky`: values repeated in blocks of 1,000 rows.
- `random`: uniformly random `u64` values from a deterministic generator.

## Encoded sizes

Lower is better. Sizes are in bytes and are deterministic for a given build.

| Data set | `auto` | `rle` | `delta_rle` | `delta_of_delta` |
|---|---:|---:|---:|---:|
| sequential | 212,520 | 312,528 | **23** | 100,019 |
| regular timestamps | 337,529 | 437,537 | **29** | 100,025 |
| jittered timestamps | 337,529 | 437,537 | 285,584 | **100,025** |
| blocky | 87,520 | **310** | 518 | 100,019 |
| random | **800,029** | 900,037 | 1,055,875 | 975,389 |

These numbers show why both delta encodings are opt-in:

- `delta_rle` collapses repeated first differences to a few bytes, but it is
  larger than `auto` for random data and larger than `rle` for repeated values.
- `delta_of_delta` handles frequently changing but small second differences,
  but it cannot amortize long runs the way `rle` and `delta_rle` do.
- `auto` remains the right default for unknown data.
