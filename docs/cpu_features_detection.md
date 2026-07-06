# CPU features detection

Rust enables developers to detect specific CPU features (instruction sets) both at runtime and at compile-time. When relevant, `stdx` packages must provide both runtime and compile-time CPU features detection. Runtime detection must be guarded by a `"std"` cargo feature.

Runtime detection requires the standard library:
```rust
if std::arch::is_XXX_feature_detected!("aes") {
    // ...
}
```

Compile-time detection:
```rust
#[cfg(target_feature = "aes")]
{
    // ...
}
```

It's important to note that compile-time detection uses a default baseline which DOES NOT depend on the features available on the compiling machine. The baseline for a target can be queried with `rustc --print cfg --target [TARGET]`.

Therefore, it's up to the developers to provide the desired `RUSTFLAGS="-C target-feature=...` when compiling code for `no_std` platforms otherwise the compiler will assume a very underperforming baseline.


As of today:

```
$ rustc --print cfg --target x86_64-unknown-linux-gnu
debug_assertions
panic="unwind"
target_abi=""
target_arch="x86_64"
target_endian="little"
target_env="gnu"
target_family="unix"
target_feature="fxsr"
target_feature="sse"
target_feature="sse2"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="ptr"
target_os="linux"
target_pointer_width="64"
target_vendor="unknown"
unix

$ rustc --print cfg --target aarch64-unknown-linux-gnu
debug_assertions
panic="unwind"
target_abi=""
target_arch="aarch64"
target_endian="little"
target_env="gnu"
target_family="unix"
target_feature="neon"
target_has_atomic="128"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="ptr"
target_os="linux"
target_pointer_width="64"
target_vendor="unknown"
unix
```
