# Benchmarks

Benchmarks and cross-crate comparisons for `stdx` packages.

This directory is a **separate Cargo workspace** nested in the `stdx` repository.
It depends on the `stdx` crates by path, so the main workspace does not need
`criterion` (or the other benchmark-only dependencies) in its dependency graph.

Run a suite from its own directory:

```bash
cd benchmarks/<suite>
cargo bench
```

Each benchmark package shares its name with the `stdx` crate it benchmarks, so
`cargo bench -p <name>` from the workspace root is ambiguous. Run it from the
suite directory (or use `cargo bench --manifest-path benchmarks/<suite>/Cargo.toml`).

For native runs, prepend `RUSTFLAGS="-C target-cpu=native"` to measure the
host's SIMD backends.

## Suites

| Package | What it measures |
| --- | --- |
| `base32` | `base32` encoding/decoding |
| `base64` | `base64` scalar vs SIMD encoding/decoding |
| `bel` | `bel` runtime evaluation |
| `cbor` | `cbor` against `ciborium`, `cbor2`, and `minicbor` |
| `constant_time_eq` | `constant_time_eq` and its generic implementation |
| `cose` | `cose` signing/encryption operations |
| `crc32` | `crc32` baseline vs specialized implementations |
| `crypto` | `crypto` hashes, MACs, AEADs, KEMs, signatures, KDFs |
| `hex` | `hex` scalar vs SIMD encoding/decoding |
| `hpke` | `hpke` setup and seal/open operations |
| `ipnetwork` | `ipnetwork` address/network parsing |
| `json` | `json` against `serde_json`, `simd-json`, and `sonic-rs` |
| `postgresql` | `postgresql` wire-format encoding |
| `utf8` | `utf8` validation |
| `uuid` | `uuid` parsing and formatting |
| `xxhash` | `xxhash` checksums |
| `zeroize` | `zeroize` clearing |
| `crypto_native` | `stdx/crypto` against `aws-lc-rs` and RustCrypto (see below) |
| `crypto_wasm` | `stdx/crypto` on `wasm32-wasip1` (see below) |

## crypto_native

Compares `stdx/crypto` against `aws-lc-rs` and RustCrypto (`sha2`, `sha3`,
`ascon-hash256`, `ascon-aead128`, `hmac`, `poly1305`, `aes-gcm`,
`chacha20poly1305`, `chacha20`, `aes`/`ctr`, `ed25519-dalek`, `ml-dsa`, and
upstream `blake3`) on native targets. Each algorithm is a criterion group; the
size is the benchmark parameter and the implementation is the benchmark id
(`stdx-crypto`, `aws-lc-rs`, `RustCrypto`).

`ascon-aead128` is pinned to a RustCrypto/AEADs git revision because the
published 0.1.x releases are not source-compatible with the stable `ascon`
release used by `ascon-hash256`.

Upstream `blake3` is built with its `neon` feature so that the BLAKE3 group
compares `stdx/crypto`'s SIMD backend against the official SIMD backend rather
than its portable fallback; on x86 the official crate already defaults to its
assembly implementations.

| Category | Compared algorithms |
| --- | --- |
| Hash | SHA-256, SHA-512, SHA3-256, SHA3-512, SHAKE256, BLAKE3, Ascon-Hash256 |
| MAC | HMAC-SHA256, HMAC-SHA512, Poly1305, BLAKE3-keyed |
| Stream cipher | AES-256-CTR, ChaCha8, ChaCha12, ChaCha20 |
| AEAD | AES-256-GCM, AES-128-GCM, ChaCha20-Poly1305, ChaCha20-BLAKE3, Ascon-AEAD128 (encrypt and decrypt) |
| Signatures | Ed25519, ML-DSA-44/65/87 (sign and verify) |

The following `stdx/crypto` algorithms are intentionally **not** benchmarked
here because none of the compared crates provide an equivalent to compare
against:

- ChaCha8-Poly1305
- KMAC256

SHAKE256, Ascon-Hash256 and Ascon-AEAD128 are only compared against RustCrypto
because `aws-lc-rs` does not expose them. ChaCha20-BLAKE3 is benchmarked as a
`stdx-crypto`-only group, as no compared crate provides an equivalent.

## crypto_wasm

Benchmarks `stdx/crypto` on `wasm32-wasip1` (with `+simd128`) using its own
timing harness. Categories: `hash`, `mac`, `stream`, `aead`, `sign`.

```bash
$ RUSTFLAGS="-C target-feature=+simd128" cargo run --release --target=wasm32-wasip1 -p crypto_wasm -- [category ...]
```
