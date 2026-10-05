# crypto_rustls

A [`rustls`](https://github.com/rustls/rustls) `CryptoProvider` backed by the
stdx [`crypto`](../crypto) crate.

This lets rustls use pure-Rust, `no_std`-friendly cryptography from this
monorepo instead of its built-in `aws-lc-rs` or `ring` backends. rustls is
compiled with `default-features = false`, so no C or assembly-backed
cryptography is pulled in.

## Supported algorithms

| Area | Support |
| --- | --- |
| TLS 1.3 cipher suites | `TLS_AES_128_GCM_SHA256`, `TLS_AES_256_GCM_SHA384`, `TLS_CHACHA20_POLY1305_SHA256` |
| Key exchange | `X25519MLKEM768` (post-quantum hybrid, preferred), `X25519`, `secp256r1`, `secp384r1` |
| Signature verification | ECDSA P-256/P-384, Ed25519, RSA PKCS#1 v1.5 and PSS (SHA-256/384/512), ML-DSA-44/65/87 |
| Signing | ECDSA P-256/P-384, Ed25519, ML-DSA-44/65/87 |
| QUIC | RFC 9001 packet protection for all three cipher suites |

The `X25519MLKEM768` hybrid is offered first, so a post-quantum key share is
sent by default in the ClientHello, while classical groups remain available
for peers that do not support it.

## Usage

Install the provider as the process default before building any rustls
configuration:

```rust
crypto_rustls::default_provider()
    .install_default()
    .expect("failed to install CryptoProvider");
```

Or pass it explicitly:

```rust
use std::sync::Arc;

let provider = Arc::new(crypto_rustls::default_provider());

let config = rustls::ClientConfig::builder_with_provider(provider)
    .with_protocol_versions(&[&rustls::version::TLS13])
    .unwrap()
    .with_root_certificates(root_store)
    .with_no_client_auth();
```

## Limitations

* **TLS 1.2 is not supported.** Only TLS 1.3 ciphersuites are provided.
* **RSA signing is not supported.** The `crypto` crate implements RSA
  verification only, so a TLS server (or client using client authentication)
  must use an ECDSA P-256/P-384, Ed25519 or ML-DSA private key. RSA-signed
  certificate chains are still *verified* normally.
* **ML-DSA private keys must be in seed form** (RFC 9881). Keys stored only as
  an expanded key are rejected, because the seed cannot be recovered from the
  expanded key. OpenSSL's default `both` encoding (seed + expanded key) is
  accepted.
* The curve implementations bind each curve to a single hash (P-256 with
  SHA-256, P-384 with SHA-384), so the uncommon cross combinations are not
  registered.

## Testing

```sh
cargo test -p crypto_rustls
```

Unit tests cover the individual adapters (hash, HMAC, AEAD, key exchange,
signatures, and QUIC with RFC 9001 known-answer vectors). Integration tests in
`tests/handshake.rs` run full TLS 1.3 handshakes with this provider on both
ends.
