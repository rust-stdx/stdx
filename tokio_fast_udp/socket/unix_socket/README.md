# tokio_fast_udp audit tests

`audit_socket_tests.rs` is a `#[cfg(test)]` child module of
`socket::unix_socket`. It calls the private `decode_cmsgs`, `CmsgBuf`,
`RecvCmsgBuf`, `fill_sockaddr` and `storage_to_addr` helpers without changing
their visibility or behaviour.

Rust resolves a submodule of the inline `unix_socket` module relative to
`tokio_fast_udp/socket/unix_socket/`, so the module is wired by this block at
the end of `mod unix_socket` in `tokio_fast_udp/socket.rs`:

```rust
    #[cfg(test)]
    #[path = "audit_socket_tests.rs"]
    mod audit_socket_tests;
```

Run it with:

```sh
CARGO_TARGET_DIR=/tmp/stdx-target-tokio CARGO_BUILD_JOBS=3 \
    cargo test -p tokio_fast_udp --locked --lib

CARGO_TARGET_DIR=/tmp/stdx-target-tokio CARGO_BUILD_JOBS=3 \
    cargo test -p tokio_fast_udp --locked --release --lib

CARGO_TARGET_DIR=/tmp/stdx-target-tokio CARGO_BUILD_JOBS=3 \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUNNER=qemu-x86_64 \
    cargo test -p tokio_fast_udp --locked --lib --target x86_64-unknown-linux-musl

CARGO_TARGET_DIR=/tmp/stdx-target-tokio CARGO_BUILD_JOBS=3 \
    MIRIFLAGS="-Zmiri-strict-provenance" \
    cargo +nightly miri test -p tokio_fast_udp --lib audit_socket_tests::decode
```

The `decode_*` tests pin down the cmsg bounds and progress fixes, including the
adversarial `cmsg_len` cases (F-1, F-2), and are exercised under Miri. The
`sockaddr_*` tests cover the IPv6 flowinfo byte-order fix (F-4), the
`*_max_batch_*` tests cover the zero-batch clamp (F-6), the seccomp subprocess
test covers the `sendmmsg` ENOSYS fallback (F-7), and the GRO tests cover the
opt-in requirement (default off, 63/64 slots, `MSG_TRUNC`). There are no
ignored tests: every finding now asserts the safe behaviour.
