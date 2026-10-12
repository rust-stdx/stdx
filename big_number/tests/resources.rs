//! Resource budgets: heap allocations on hot paths and stack usage.
//!
//! Verification, signing and the modular arithmetic must not allocate, and the
//! deepest path (RSA-8192 verification) must fit in a modest thread stack.
//! These guard against regressions such as reintroducing intermediate `Vec`s
//! or oversized stack temporaries.
//!
//! Counting is per-thread (see `alloc_counter`), so the tests in this file are
//! unaffected by each other or by the test harness running in parallel.

use alloc_counter::measure;
use big_number::Uint;

fn odd_modulus_bytes(bytes: usize) -> Vec<u8> {
    let mut n = vec![0x5Au8; bytes];
    n[0] |= 0x80; // full width
    n[bytes - 1] |= 1; // odd
    n
}

#[test]
fn hot_paths_do_not_allocate() {
    type U256 = Uint<256, 4>;
    let a = U256::from_limbs([1, 2, 3, 4]);
    let m = U256::from_limbs([u64::MAX, u64::MAX, u64::MAX, 1 << 63]);
    let mu = m.compute_mu_for_barrett();

    let ((), counters) = measure(|| {
        for _ in 0..1_000 {
            std::hint::black_box(a.mul_mod(&a, &m));
            std::hint::black_box(a.mul_mod_barrett(&a, &m, &mu));
        }
    });
    assert_eq!(counters.calls, 0, "modular multiplication must not allocate");

    let ((), counters) = measure(|| {
        std::hint::black_box(U256::from_str_radix("123456789012345678901234567890", 10).unwrap());
    });
    assert_eq!(counters.calls, 0, "from_str_radix must not allocate");

    let ((), counters) = measure(|| {
        std::hint::black_box(a.to_be_bytes());
    });
    assert!(counters.calls <= 1, "to_be_bytes allocates once");

    let ((), counters) = measure(|| {
        std::hint::black_box(a.to_string_radix(10));
    });
    assert!(counters.calls <= 1, "to_string_radix should allocate a single String");

    // RSA and ECDSA hot paths.
    let n = odd_modulus_bytes(256);
    let e = [0x01u8, 0x00, 0x01];
    let key = crypto::rsa::PublicKey::from_n_e(&n, &e).unwrap();
    let sig = vec![0xA5u8; 256];
    let ((), counters) = measure(|| {
        let _ = std::hint::black_box(key.verify_pkcs1_sha256(&sig, b"message"));
        let _ = std::hint::black_box(key.verify_pss_sha256(&sig, b"message"));
    });
    assert_eq!(counters.calls, 0, "RSA verification must not allocate");

    let sk = crypto::p256::SecretKey::generate().unwrap();
    let pk = sk.public_key();
    let signature = sk.sign(b"message").unwrap();
    let ((), counters) = measure(|| {
        let _ = std::hint::black_box(sk.sign(b"message"));
        let _ = std::hint::black_box(pk.verify(b"message", &signature));
    });
    assert_eq!(counters.calls, 0, "ECDSA sign/verify must not allocate");
}

/// wasm32-wasip1 has no threads, so the small thread stack cannot be set up.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn rsa_8192_verification_fits_a_small_stack() {
    let n = odd_modulus_bytes(1024); // 8192-bit modulus
    let e = [0x01u8, 0x00, 0x01];
    let key = crypto::rsa::PublicKey::from_n_e(&n, &e).unwrap();
    let sig = vec![0xA5u8; 1024];

    // 256 KiB is comfortably above the observed footprint but far below the
    // default 8 MiB main-thread stack, so a regression in temporary sizing is
    // caught here.
    let handle = std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(move || {
            assert!(key.verify_pkcs1_sha256(&sig, b"message").is_err());
        })
        .unwrap();
    handle.join().unwrap();
}
