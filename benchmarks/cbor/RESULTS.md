# CBOR benchmark results

Run from the benchmark's directory:

```sh
cd benchmarks/cbor
cargo bench --bench cbor -- --noplot
```

Comparison crates: `ciborium 0.2.2`, `cbor2 1.1.6`, `minicbor 2.3.1`.
The `cbor_typed` rows use `#[derive(Encode, Decode)]`; `minicbor` uses
hand-written impls matching the same wire shape where possible.

## Fixtures

- `log`: 128 telemetry records as text-key maps, 16,686 bytes for
  cbor/ciborium/cbor2 (minicbor's f64 encoding is 17,198 bytes).
- `array`: the same 128 records as positional arrays, 8,750 bytes.
- `int_keys`: 128 COSE-style integer-keyed records, 13,306 bytes.
- `blob`: one 4 KiB byte string, 4,099 bytes.
- `strings`: 1,000 short strings.
- `ints`: 1,000 `u64` values.
- `floats`: 1,000 `f64` values.
- `text`: one 64 KiB ASCII text string, or one 96 KiB non-ASCII text
  string (`€` repeated).
- `stream`: one telemetry record framed through `BufReader`.

## Results

Medians on this container (aarch64, Rust 1.99, release, thin LTO, 50
samples, 2 s measurement). The host is shared and noisy; rerun on the
target hardware before making deployment decisions. Lower is better.

| Operation | cbor (typed) | cbor2 | minicbor | ciborium |
|---|---:|---:|---:|---:|
| encode log (16.7 KB) | **3.61 µs** | 5.95 µs | 5.20 µs | 13.52 µs |
| decode log | 33.72 µs | 41.90 µs | **29.10 µs** | 77.05 µs |
| encode array (8.8 KB) | **2.75 µs** | 4.49 µs | — | 10.26 µs |
| decode array | **24.23 µs** | 27.46 µs | — | 56.94 µs |
| encode int keys (13.3 KB) | **1.62 µs** | — | 4.10 µs | — |
| decode int keys | 14.76 µs | — | **13.76 µs** | — |
| encode blob (4 KiB) | 142 ns | **125 ns** | 163 ns | 150 ns |
| decode blob | **127 ns** | 131 ns | 137 ns | 272 ns |
| decode 1,000 strings | 35.56 µs | — | **29.39 µs** | — |
| decode 1,000 ints | **3.02 µs** | — | — | 4.77 µs | — |
| decode 1,000 floats | **3.16 µs** | — | — | 3.28 µs | — |

## UTF-8 validation and text decode

Text validation and `&str` construction use the stdx `utf8` crate. The
ASCII fast path uses NEON on this host; AVX2 is selected at runtime on
`x86_64` with `std`, and WASM SIMD128 is selected at compile time. Inputs
shorter than 64 bytes, and non-ASCII inputs, use the standard library
implementation.

| Operation (64 KiB ASCII text) | cbor/utf8 | std | Speedup |
|---|---:|---:|---:|
| validate | **0.68 µs** | 2.14 µs | 3.2× |
| decode to `&str` | **0.67 µs** | 2.19 µs | 3.3× |
| decode to `String` | 2.22 µs | — | — |
| `str::to_owned` only | 0.90 µs | — | — |

For the 96 KiB non-ASCII text fixture, cbor is unchanged relative to the
standard library (about 58 µs for validation); the SIMD kernel
deliberately falls back to std when it sees a non-ASCII byte. Full
non-ASCII SIMD is a possible follow-up, not part of this change.

## Buffer reuse

`to_vec_into` reuses a caller-provided `Vec<u8>`. Exact preallocation via
`encoded_len` is slower because it adds a full counting pass.

| encode 16.7 KB log | median |
|---|---:|
| `to_vec` | 3.36 µs |
| `to_vec_into` (reused) | **3.14 µs** |
| `to_vec_with_capacity(512)` | 3.35 µs |
| `to_vec_with_capacity(encoded_len)` | 4.13 µs |

The default `to_vec` initial capacity is 256 bytes. For the single-record
fixture this removed one reallocation: about 95 ns before, 59 ns after.

## Skip and streaming

`skip` parses primitive items in the container loop instead of making a
recursive call per item.

| 1,000-integer array | median |
|---|---:|
| `skip` | 3.10 µs |
| `validate` | 4.76 µs |
| `from_slice::<Vec<u64>>` | 2.77 µs |

The same `skip` measurement before the change was about 4.87 µs, so the
loop is roughly 1.6× faster. Decode remains slightly faster because it
does not maintain the skip depth checks.

Streaming a record through `BufReader` is close to slice decoding:

| one log record | median |
|---|---:|
| `reader::read_item` | 228 ns |
| `reader::read_item_into` | 224 ns |
| `from_slice` | 209 ns |

## Fixed-buffer / no-allocation path

`cbor::to_slice` into a caller-provided buffer:

- text-map log record: ~22 ns
- array record: ~21 ns
- integer-keyed record: ~7 ns

Allocation checks in `cbor/tests/allocations.rs` verify:

- borrowed struct decode: 0 allocations
- fixed-buffer encode: 0 allocations
- `encoded_len`, `validate`, `validate_canonical`, `skip`: 0 allocations
- `RawValue::new` and borrowed `RawValue` decode: 0 allocations
- `to_vec` for a small value: 1 allocation

## Notes

- The typed path is the primary API: it avoids an intermediate data model
  and supports no-alloc decoding.
- `Vec<u8>` is a CBOR array in the typed API; use `#[cbor(bytes)]` or
  `bytes::Bytes` for byte strings. The comparison fixtures use the same
  byte-string shape.
- Criterion output is noisy at the sub-100 ns scale; rerun with a larger
  sample size for final numbers.
