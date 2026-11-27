//! Reports which hash backends the running target actually selected.
//!
//! The test itself only asserts that every reported name is one of the
//! documented values; the printed lines are the evidence used by the audit
//! report to show which kernels the other tests exercised on aarch64 and under
//! qemu-x86_64.

use crypto::sha3::{keccak_backend_name, kt256_backend_name, parallel_hash_backend_name};

#[test]
fn reports_the_selected_hash_backends() {
    println!("arch = {}", std::env::consts::ARCH);

    let single = keccak_backend_name();
    println!("keccak single-state backend = {single}");
    assert!(
        matches!(single, "scalar" | "avx2" | "avx512" | "armv8.4-sha3"),
        "unexpected single-state backend {single}"
    );

    let parallel = parallel_hash_backend_name();
    println!("parallel-hash batched backend = {parallel}");
    assert!(
        matches!(
            parallel,
            "scalar" | "neon" | "avx2" | "avx512" | "armv8.4-sha3" | "wasm-simd128"
        ),
        "unexpected batched backend {parallel}"
    );

    let kt = kt256_backend_name();
    println!("KT256 batched backend = {kt}");
    assert!(
        matches!(kt, "scalar" | "neon" | "avx2" | "avx512" | "armv8.4-sha3" | "wasm-simd128"),
        "unexpected KT256 backend {kt}"
    );

    let turbo = crypto::turbohash::backend_name();
    println!("TurboHash batched backend = {turbo}");
    assert!(
        matches!(turbo, "scalar" | "neon" | "avx2" | "avx512" | "armv8.4-sha3" | "wasm-simd128"),
        "unexpected TurboHash backend {turbo}"
    );

    #[cfg(target_arch = "aarch64")]
    {
        println!("aarch64 neon  = {}", std::arch::is_aarch64_feature_detected!("neon"));
        println!("aarch64 asimd = {}", std::arch::is_aarch64_feature_detected!("asimd"));
        println!("aarch64 sha2  = {}", std::arch::is_aarch64_feature_detected!("sha2"));
        println!("aarch64 sha3  = {}", std::arch::is_aarch64_feature_detected!("sha3"));
        println!("aarch64 aes   = {}", std::arch::is_aarch64_feature_detected!("aes"));
        println!("aarch64 pmull = {}", std::arch::is_aarch64_feature_detected!("pmull"));
    }

    #[cfg(target_arch = "x86_64")]
    {
        println!("x86 sse2      = {}", std::arch::is_x86_feature_detected!("sse2"));
        println!("x86 ssse3     = {}", std::arch::is_x86_feature_detected!("ssse3"));
        println!("x86 sse4.1    = {}", std::arch::is_x86_feature_detected!("sse4.1"));
        println!("x86 avx       = {}", std::arch::is_x86_feature_detected!("avx"));
        println!("x86 avx2      = {}", std::arch::is_x86_feature_detected!("avx2"));
        println!("x86 avx512f   = {}", std::arch::is_x86_feature_detected!("avx512f"));
        println!("x86 sha       = {}", std::arch::is_x86_feature_detected!("sha"));
        println!("x86 sha512    = {}", std::arch::is_x86_feature_detected!("sha512"));
        println!("x86 pclmulqdq = {}", std::arch::is_x86_feature_detected!("pclmulqdq"));
        println!("x86 aes       = {}", std::arch::is_x86_feature_detected!("aes"));
    }
}
