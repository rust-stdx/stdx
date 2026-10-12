//! Allocation regression tests for the post-quantum primitives.
//!
//! ML-KEM and ML-DSA are pure stack algorithms: key generation, encapsulation,
//! decapsulation, signing and verification must never touch the heap. Counting
//! comes from `alloc_counter`, which attributes allocations to the current
//! thread, so these assertions are not perturbed by parallel tests.

use alloc_counter::measure;
use crypto::{
    mldsa::{MlDsa44SecretKey, MlDsa65SecretKey, MlDsa87SecretKey},
    mlkem::{SecretKey768, SecretKey1024},
};

/// Runs `f` and asserts it performed no allocations.
fn assert_no_alloc<R>(name: &str, f: impl FnOnce() -> R) {
    let (_value, counts) = measure(f);
    assert_eq!(
        counts.calls, 0,
        "{name} allocated {} time(s) / {} byte(s)",
        counts.calls, counts.bytes
    );
}

#[test]
fn ml_kem_does_not_allocate() {
    let (sk768, pk768) = SecretKey768::generate_derand(&[7u8; 64]);
    let (ss768, ct768) = pk768.encapsulate_derand(&[9u8; 32]);
    assert_eq!(sk768.decapsulate(&ct768).unwrap(), ss768);

    assert_no_alloc("ml-kem-768 keygen", || SecretKey768::generate_derand(&[7u8; 64]));
    assert_no_alloc("ml-kem-768 encapsulate", || pk768.encapsulate_derand(&[9u8; 32]));
    let _ = assert_no_alloc("ml-kem-768 decapsulate", || sk768.decapsulate(&ct768));

    let (sk1024, pk1024) = SecretKey1024::generate_derand(&[7u8; 64]);
    let (ss1024, ct1024) = pk1024.encapsulate_derand(&[9u8; 32]);
    assert_eq!(sk1024.decapsulate(&ct1024).unwrap(), ss1024);

    assert_no_alloc("ml-kem-1024 keygen", || SecretKey1024::generate_derand(&[7u8; 64]));
    assert_no_alloc("ml-kem-1024 encapsulate", || pk1024.encapsulate_derand(&[9u8; 32]));
    let _ = assert_no_alloc("ml-kem-1024 decapsulate", || sk1024.decapsulate(&ct1024));
}

#[test]
fn ml_dsa_does_not_allocate() {
    let seed = [3u8; 32];
    let rnd = [1u8; 32];
    let msg = b"allocation regression";

    let k44 = MlDsa44SecretKey::new(&seed);
    let sig44 = k44.sign_derand(msg, b"", &rnd).unwrap();
    assert!(k44.public_key().verify(msg, &sig44, b"").is_ok());
    assert_no_alloc("ml-dsa-44 keygen", || MlDsa44SecretKey::new(&seed));
    assert_no_alloc("ml-dsa-44 sign", || k44.sign_derand(msg, b"", &rnd));
    let _ = assert_no_alloc("ml-dsa-44 verify", || k44.public_key().verify(msg, &sig44, b""));

    let k65 = MlDsa65SecretKey::new(&seed);
    let sig65 = k65.sign_derand(msg, b"", &rnd).unwrap();
    assert!(k65.public_key().verify(msg, &sig65, b"").is_ok());
    assert_no_alloc("ml-dsa-65 keygen", || MlDsa65SecretKey::new(&seed));
    assert_no_alloc("ml-dsa-65 sign", || k65.sign_derand(msg, b"", &rnd));
    let _ = assert_no_alloc("ml-dsa-65 verify", || k65.public_key().verify(msg, &sig65, b""));

    let k87 = MlDsa87SecretKey::new(&seed);
    let sig87 = k87.sign_derand(msg, b"", &rnd).unwrap();
    assert!(k87.public_key().verify(msg, &sig87, b"").is_ok());
    assert_no_alloc("ml-dsa-87 keygen", || MlDsa87SecretKey::new(&seed));
    assert_no_alloc("ml-dsa-87 sign", || k87.sign_derand(msg, b"", &rnd));
    let _ = assert_no_alloc("ml-dsa-87 verify", || k87.public_key().verify(msg, &sig87, b""));
}

#[test]
fn repeated_operations_do_not_leak() {
    // Running many operations must not retain heap memory (there should be no
    // heap traffic at all, but this also catches accidental caching).
    let before = alloc_counter::counters().live_bytes;
    for _ in 0..100 {
        let (sk, pk) = SecretKey768::generate_derand(&[1u8; 64]);
        let (_, ct) = pk.encapsulate_derand(&[2u8; 32]);
        let _ = sk.decapsulate(&ct);
        let k = MlDsa65SecretKey::new(&[3u8; 32]);
        let sig = k.sign_derand(b"m", b"", &[4u8; 32]).unwrap();
        let _ = k.public_key().verify(b"m", &sig, b"");
    }
    let after = alloc_counter::counters().live_bytes;
    assert_eq!(before, after, "post-quantum operations retained heap memory");
}
