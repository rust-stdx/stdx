# zeroize

Securely zero memory (a.k.a. [zeroize]) while avoiding compiler optimizations.

This crate implements a portable approach to securely zeroing memory using
techniques which guarantee they won't be "optimized away" by the compiler.

The [`Zeroize` trait] is the crate's primary API.

[Documentation]

## About

[Zeroing memory securely is hard] - compilers optimize for performance, and
in doing so they love to "optimize away" unnecessary zeroing calls. There are
many documented "tricks" to attempt to avoid these optimizations and ensure
that a zeroing routine is performed reliably.

This crate isn't about tricks: it uses [core::ptr::write_volatile]
to provide easy-to-use, portable zeroing behavior which works on all of Rust's
core number types and slices thereof. On supported architectures a minimal
inline-assembly optimization barrier stops the compiler from eliding the writes,
with a portable `core::hint::black_box` fallback on every other target.

- No insecure fallbacks!
- No required dependencies!
- No FFI! A minimal optimization barrier uses inline assembly on supported targets, with a
  portable fallback that keeps **WASM** working.
- `#![no_std]` i.e. **embedded-friendly**!
- No functionality besides securely zeroing memory!
- (Optional) Custom derive support for zeroing complex structures

## Minimum Supported Rust Version

Requires Rust **1.85** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope for this crate's
semantic versioning guarantees).

## Example

```rust
use zeroize::Zeroize;

// Protip: don't embed secrets in your source code.
// This is just an example.
let mut secret = b"Air shield password: 1,2,3,4,5".to_vec();
// [ ... ] open the air shield here

// Now that we're done using the secret, zero it out.
secret.zeroize();
```


[zeroize]: https://en.wikipedia.org/wiki/Zeroisation
[`Zeroize` trait]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
[Documentation]: https://docs.rs/zeroize/
[Zeroing memory securely is hard]: http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html
[core::ptr::write_volatile]: https://doc.rust-lang.org/core/ptr/fn.write_volatile.html
[good cryptographic hygiene]: https://github.com/veorq/cryptocoding#clean-memory-of-secret-data
