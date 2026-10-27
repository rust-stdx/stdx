//! Allocation regression tests for the post-quantum primitives.
//!
//! ML-KEM and ML-DSA are pure stack algorithms: key generation, encapsulation,
//! decapsulation, signing and verification must never touch the heap. Counting
//! comes from `alloc_counter`, which attributes allocations to the current
//! thread, so these assertions are not perturbed by parallel tests.

use alloc_counter::measure;
use crypto::{
    mldsa::{
        MlDsa44SecretKey, MlDsa65Ed25519Sha512SecretKey, MlDsa65SecretKey, MlDsa87Ed25519MessageRepresentative,
        MlDsa87Ed25519SecretKey, MlDsa87Ed25519Signature, MlDsa87SecretKey,
    },
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
    assert_no_alloc("ml-kem-768 decapsulate", || sk768.decapsulate(&ct768));

    let (sk1024, pk1024) = SecretKey1024::generate_derand(&[7u8; 64]);
    let (ss1024, ct1024) = pk1024.encapsulate_derand(&[9u8; 32]);
    assert_eq!(sk1024.decapsulate(&ct1024).unwrap(), ss1024);

    assert_no_alloc("ml-kem-1024 keygen", || SecretKey1024::generate_derand(&[7u8; 64]));
    assert_no_alloc("ml-kem-1024 encapsulate", || pk1024.encapsulate_derand(&[9u8; 32]));
    assert_no_alloc("ml-kem-1024 decapsulate", || sk1024.decapsulate(&ct1024));
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
    assert_no_alloc("ml-dsa-44 verify", || k44.public_key().verify(msg, &sig44, b""));

    let k65 = MlDsa65SecretKey::new(&seed);
    let sig65 = k65.sign_derand(msg, b"", &rnd).unwrap();
    assert!(k65.public_key().verify(msg, &sig65, b"").is_ok());
    assert_no_alloc("ml-dsa-65 keygen", || MlDsa65SecretKey::new(&seed));
    assert_no_alloc("ml-dsa-65 sign", || k65.sign_derand(msg, b"", &rnd));
    assert_no_alloc("ml-dsa-65 verify", || k65.public_key().verify(msg, &sig65, b""));

    let k87 = MlDsa87SecretKey::new(&seed);
    let sig87 = k87.sign_derand(msg, b"", &rnd).unwrap();
    assert!(k87.public_key().verify(msg, &sig87, b"").is_ok());
    assert_no_alloc("ml-dsa-87 keygen", || MlDsa87SecretKey::new(&seed));
    assert_no_alloc("ml-dsa-87 sign", || k87.sign_derand(msg, b"", &rnd));
    assert_no_alloc("ml-dsa-87 verify", || k87.public_key().verify(msg, &sig87, b""));
}

#[test]
fn mldsa65_ed25519_does_not_allocate() {
    let seed = [3u8; 64];
    let rnd = [1u8; 32];
    let msg = b"composite allocation regression";

    let key = MlDsa65Ed25519Sha512SecretKey::new(&seed);
    let public_key = key.public_key();
    let signature = key.sign_derand(msg, b"ctx", &rnd).unwrap();
    assert!(public_key.verify(msg, &signature, b"ctx").is_ok());

    assert_no_alloc("mldsa65-ed25519 keygen", || MlDsa65Ed25519Sha512SecretKey::new(&seed));
    assert_no_alloc("mldsa65-ed25519 sign", || key.sign_derand(msg, b"ctx", &rnd));
    assert_no_alloc("mldsa65-ed25519 verify", || public_key.verify(msg, &signature, b"ctx"));

    let ctx = [0u8; 256];
    assert_no_alloc("mldsa65-ed25519 sign context too long", || key.sign_derand(msg, &ctx, &rnd));
    assert_no_alloc("mldsa65-ed25519 verify context too long", || {
        public_key.verify(msg, &signature, &ctx)
    });

    let mut bad_signature = signature;
    bad_signature[0] ^= 1;
    assert_no_alloc("mldsa65-ed25519 verify reject", || {
        public_key.verify(msg, &bad_signature, b"ctx")
    });
}

#[test]
fn mldsa87_ed25519_parallelhash256_does_not_allocate() {
    let seed = [3u8; 64];
    let rnd = [1u8; 32];
    let ctx = b"ctx";
    let msg = vec![0x5au8; 10_000];

    let key = MlDsa87Ed25519SecretKey::new(&seed);
    let public_key = key.public_key();
    let signature = key.sign_derand(&msg, ctx, &rnd).unwrap();
    assert!(public_key.verify(&msg, &signature, ctx).is_ok());

    let prehash = {
        let mut hasher = MlDsa87Ed25519MessageRepresentative::hasher();
        hasher.update(&msg);
        let mut out = [0u8; 64];
        hasher.finalize_into(&mut out);
        out
    };
    let prehashed_signature = key.sign_prehashed_derand(&prehash, ctx, &rnd).unwrap();
    assert_eq!(prehashed_signature, signature);

    assert_no_alloc("mldsa87-ed25519 keygen", || MlDsa87Ed25519SecretKey::new(&seed));
    assert_no_alloc("mldsa87-ed25519 sign", || key.sign_derand(&msg, ctx, &rnd));
    assert_no_alloc("mldsa87-ed25519 verify", || public_key.verify(&msg, &signature, ctx));
    assert_no_alloc("mldsa87-ed25519 streaming prehash", || {
        let mut hasher = MlDsa87Ed25519MessageRepresentative::hasher();
        hasher.update(&msg);
        let mut out = [0u8; 64];
        hasher.finalize_into(&mut out);
        out
    });
    assert_no_alloc("mldsa87-ed25519 sign prehashed", || {
        key.sign_prehashed_derand(&prehash, ctx, &rnd)
    });
    assert_no_alloc("mldsa87-ed25519 verify prehashed", || {
        public_key.verify_prehashed(&prehash, &signature, ctx)
    });

    let long_ctx = [0u8; 256];
    assert_no_alloc("mldsa87-ed25519 sign context too long", || {
        key.sign_derand(msg.as_slice(), &long_ctx, &rnd)
    });
    assert_no_alloc("mldsa87-ed25519 verify context too long", || {
        public_key.verify(msg.as_slice(), &signature, &long_ctx)
    });
    assert_no_alloc("mldsa87-ed25519 sign prehashed context too long", || {
        key.sign_prehashed_derand(&prehash, &long_ctx, &rnd)
    });
    assert_no_alloc("mldsa87-ed25519 verify prehashed context too long", || {
        public_key.verify_prehashed(&prehash, &signature, &long_ctx)
    });

    let mut bad_signature = signature.to_bytes();
    bad_signature[0] ^= 1;
    let bad_signature = MlDsa87Ed25519Signature::from_bytes(&bad_signature);
    assert_no_alloc("mldsa87-ed25519 verify reject", || {
        public_key.verify(msg.as_slice(), &bad_signature, ctx)
    });
}

#[test]
fn pqc_error_paths_do_not_allocate() {
    let seed = [3u8; 32];
    let rnd = [1u8; 32];
    let msg = b"allocation error path";

    let (sk768, pk768) = SecretKey768::generate_derand(&[7u8; 64]);
    let (_, mut ct768) = pk768.encapsulate_derand(&[9u8; 32]);
    ct768[0] ^= 1;
    assert_no_alloc("ml-kem-768 decapsulate reject", || sk768.decapsulate(&ct768));

    let (sk1024, pk1024) = SecretKey1024::generate_derand(&[7u8; 64]);
    let (_, mut ct1024) = pk1024.encapsulate_derand(&[9u8; 32]);
    ct1024[0] ^= 1;
    assert_no_alloc("ml-kem-1024 decapsulate reject", || sk1024.decapsulate(&ct1024));

    let key = MlDsa65SecretKey::new(&seed);
    let pk = key.public_key();
    let mut sig = key.sign_derand(msg, b"", &rnd).unwrap();
    sig[0] ^= 1;
    assert_no_alloc("ml-dsa-65 verify reject", || pk.verify(msg, &sig, b""));

    let ctx = [0u8; 256];
    assert_no_alloc("ml-dsa-65 sign context too long", || key.sign_derand(msg, &ctx, &rnd));
    assert_no_alloc("ml-dsa-65 verify context too long", || pk.verify(msg, &sig, &ctx));
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

        let ck = MlDsa65Ed25519Sha512SecretKey::new(&[5u8; 64]);
        let csig = ck.sign_derand(b"m", b"", &[6u8; 32]).unwrap();
        let _ = ck.public_key().verify(b"m", &csig, b"");

        let ck87 = MlDsa87Ed25519SecretKey::new(&[7u8; 64]);
        let csig87 = ck87.sign_derand(b"m", b"", &[8u8; 32]).unwrap();
        let _ = ck87.public_key().verify(b"m", &csig87, b"");
    }
    let after = alloc_counter::counters().live_bytes;
    assert_eq!(before, after, "post-quantum operations retained heap memory");
}

#[test]
fn blake3_does_not_allocate() {
    use crypto::{Hasher, Xof, blake3::Blake3};

    // Built before the measured closures so the buffer itself is not counted.
    let data = vec![0x5au8; 3000];
    let key = [0x11u8; 32];

    assert_no_alloc("blake3 one-shot", || Blake3::hash(&data));
    assert_no_alloc("blake3 keyed_hash", || Blake3::keyed_hash(&key, &data));
    assert_no_alloc("blake3 derive_key", || Blake3::derive_key("allocation test context", &data));

    assert_no_alloc("blake3 incremental", || {
        let mut hasher = Blake3::new();
        for chunk in data.chunks(7) {
            hasher.update(chunk);
        }
        hasher.sum()
    });

    assert_no_alloc("blake3 keyed incremental", || {
        let mut hasher = Blake3::new_keyed(&key);
        hasher.update(&data);
        hasher.sum()
    });

    assert_no_alloc("blake3 new_derive_key", || {
        let mut hasher = Blake3::new_derive_key("allocation test context");
        hasher.update(&data);
        hasher.sum()
    });

    assert_no_alloc("blake3 xof squeeze", || {
        let mut out = [0u8; 256];
        let mut hasher = Blake3::new();
        hasher.update(&data);
        let mut xof = hasher.finalize_xof();
        xof.squeeze(&mut out);
        out[0]
    });
}

#[test]
fn blake3_does_not_leak() {
    use crypto::{Hasher, Xof, blake3::Blake3};

    let data = vec![0x33u8; 4096];

    let before_bytes = alloc_counter::counters().live_bytes;
    let before_allocs = alloc_counter::counters().live_allocations;

    for i in 0..1000usize {
        let mut hasher = Blake3::new_keyed(&[i as u8; 32]);
        hasher.update(&data);
        let _ = hasher.sum();

        let mut hasher = Blake3::new();
        hasher.update(&data[..100]);
        let mut xof = hasher.finalize_xof();
        let mut out = [0u8; 128];
        xof.squeeze(&mut out);
    }

    assert_eq!(
        before_bytes,
        alloc_counter::counters().live_bytes,
        "blake3 retained heap memory"
    );
    assert_eq!(
        before_allocs,
        alloc_counter::counters().live_allocations,
        "blake3 leaked allocations"
    );
}

#[test]
fn parallel_hash_does_not_allocate() {
    use crypto::sha3::ParallelHash256;

    // Built outside the measured closures so the input buffer itself is not
    // counted.
    let data = vec![0x5au8; 64 * 1024];

    assert_no_alloc("parallel-hash one-shot", || {
        let mut out = [0u8; 64];
        ParallelHash256::hash(&data, 1024, b"alloc", &mut out);
        out[0]
    });

    assert_no_alloc("parallel-hash streaming", || {
        let mut hasher = ParallelHash256::new(1024, b"alloc");
        for chunk in data.chunks(7) {
            hasher.update(chunk);
        }
        let mut out = [0u8; 64];
        hasher.finalize_into(&mut out);
        out[0]
    });

    assert_no_alloc("parallel-hash xof", || {
        let mut hasher = ParallelHash256::new(1024, b"alloc");
        hasher.update(&data);
        let mut reader = hasher.finalize_xof();
        let mut out = [0u8; 512];
        reader.squeeze(&mut out);
        out[0]
    });
}

#[test]
fn turboshake_does_not_allocate() {
    use crypto::{Hasher, Xof, sha3::TurboShake256};

    // Built outside the measured closures so the input buffer itself is not
    // counted.
    let data = vec![0x5au8; 64 * 1024];

    assert_no_alloc("turboshake one-shot", || {
        let mut out = [0u8; 64];
        TurboShake256::hash(&data, &mut out);
        out[0]
    });

    assert_no_alloc("turboshake incremental", || {
        let mut xof = TurboShake256::new(0x07);
        for chunk in data.chunks(7) {
            xof.absorb(chunk);
        }
        let mut out = [0u8; 512];
        xof.squeeze(&mut out);
        out[0]
    });

    assert_no_alloc("turboshake hasher", || {
        let mut hasher = <TurboShake256 as Hasher>::new();
        hasher.update(&data);
        hasher.sum()
    });
}

#[test]
fn turboshake_does_not_leak() {
    use crypto::{Xof, sha3::TurboShake256};

    let data = vec![0x33u8; 4096];

    let before_bytes = alloc_counter::counters().live_bytes;
    let before_allocs = alloc_counter::counters().live_allocations;

    for i in 0..1000usize {
        // Vary the domain separator too: it must not be retained anywhere.
        let mut xof = TurboShake256::new(0x01 + (i % 0x7f) as u8);
        xof.absorb(&data);
        let mut out = [0u8; 128];
        xof.squeeze(&mut out);
    }

    assert_eq!(
        before_bytes,
        alloc_counter::counters().live_bytes,
        "turboshake retained heap memory"
    );
    assert_eq!(
        before_allocs,
        alloc_counter::counters().live_allocations,
        "turboshake leaked allocations"
    );
}

#[test]
fn parallel_hash_does_not_leak() {
    use crypto::sha3::ParallelHash256;

    let data = vec![0x33u8; 4096];

    let before_bytes = alloc_counter::counters().live_bytes;
    let before_allocs = alloc_counter::counters().live_allocations;

    for i in 0..1000usize {
        // Vary the block size too: it must not be retained anywhere.
        let mut out = [0u8; 64];
        ParallelHash256::hash(&data, 136 + (i % 7), b"leak", &mut out);
    }

    assert_eq!(
        before_bytes,
        alloc_counter::counters().live_bytes,
        "parallel-hash retained heap memory"
    );
    assert_eq!(
        before_allocs,
        alloc_counter::counters().live_allocations,
        "parallel-hash leaked allocations"
    );
}

#[test]
fn hopmac256_does_not_allocate() {
    use crypto::sha3::HopMac256;

    // Built outside the measured closures so the input buffers themselves are
    // not counted.
    let data = vec![0x5au8; 64 * 1024];
    let key = b"hopmac allocation key";
    let customization = b"alloc";

    assert_no_alloc("hopmac256 one-shot", || {
        let mut out = [0u8; 64];
        HopMac256::mac(key, &data, customization, &mut out);
        out[0]
    });

    assert_no_alloc("hopmac256 streaming", || {
        let mut mac = HopMac256::new(key, customization);
        for chunk in data.chunks(7) {
            mac.update(chunk);
        }
        let mut out = [0u8; 64];
        mac.finalize_into(&mut out);
        out[0]
    });

    assert_no_alloc("hopmac256 xof", || {
        let mut mac = HopMac256::new(key, customization);
        mac.update(&data);
        let mut reader = mac.finalize_xof();
        let mut out = [0u8; 512];
        reader.squeeze(&mut out);
        out[0]
    });
}

#[test]
fn kt256_does_not_allocate() {
    use crypto::sha3::Kt256;

    // Built outside the measured closures so the input buffers themselves are
    // not counted.
    let data = vec![0x5au8; 64 * 1024];
    let customization = b"alloc";

    assert_no_alloc("kt256 one-shot", || {
        let mut out = [0u8; 64];
        Kt256::hash(&data, customization, &mut out);
        out[0]
    });

    assert_no_alloc("kt256 streaming", || {
        let mut hasher = Kt256::new(customization);
        for chunk in data.chunks(7) {
            hasher.update(chunk);
        }
        let mut out = [0u8; 64];
        hasher.finalize_into(&mut out);
        out[0]
    });

    assert_no_alloc("kt256 xof", || {
        let mut hasher = Kt256::new(customization);
        hasher.update(&data);
        let mut reader = hasher.finalize_xof();
        let mut out = [0u8; 512];
        reader.squeeze(&mut out);
        out[0]
    });
}

#[test]
fn kt256_does_not_leak() {
    use crypto::sha3::Kt256;

    let data = vec![0x33u8; 2 * 8192 + 17];

    let before_bytes = alloc_counter::counters().live_bytes;
    let before_allocs = alloc_counter::counters().live_allocations;

    for i in 0..1000usize {
        // Vary the customization length too: it must not be retained anywhere.
        let customization = vec![0xa5u8; i % 41];
        let mut out = [0u8; 64];
        Kt256::hash(&data, &customization, &mut out);
    }

    assert_eq!(before_bytes, alloc_counter::counters().live_bytes, "kt256 retained heap memory");
    assert_eq!(
        before_allocs,
        alloc_counter::counters().live_allocations,
        "kt256 leaked allocations"
    );
}

#[test]
fn turbohash_does_not_allocate() {
    use crypto::turbohash::TurboHash;

    // Built outside the measured closures so the input buffers themselves are
    // not counted.
    let data = vec![0x5au8; 64 * 1024];
    let context = b"alloc";
    let key = [0x11u8; 32];

    assert_no_alloc("turbohash one-shot", || {
        let mut out = [0u8; 64];
        TurboHash::hash(context, &data, &mut out);
        out[0]
    });

    assert_no_alloc("turbohash streaming", || {
        let mut hasher = TurboHash::new(context);
        for chunk in data.chunks(7) {
            hasher.update(chunk);
        }
        let mut out = [0u8; 64];
        hasher.finalize_into(&mut out);
        out[0]
    });

    assert_no_alloc("turbohash xof", || {
        let mut hasher = TurboHash::new(context);
        hasher.update(&data);
        let mut reader = hasher.finalize_xof();
        let mut out = [0u8; 512];
        reader.squeeze(&mut out);
        out[0]
    });

    assert_no_alloc("turbohash keyed one-shot", || {
        let mut out = [0u8; 64];
        TurboHash::mac(&key, context, &data, &mut out);
        out[0]
    });

    assert_no_alloc("turbohash keyed streaming", || {
        let mut hasher = TurboHash::new_keyed(&key, context);
        for chunk in data.chunks(7) {
            hasher.update(chunk);
        }
        let mut out = [0u8; 64];
        hasher.finalize_into(&mut out);
        out[0]
    });

    assert_no_alloc("turbohash keyed xof", || {
        let mut hasher = TurboHash::new_keyed(&key, context);
        hasher.update(&data);
        let mut reader = hasher.finalize_xof();
        let mut out = [0u8; 512];
        reader.squeeze(&mut out);
        out[0]
    });
}

#[test]
fn turbohash_does_not_leak() {
    use crypto::turbohash::TurboHash;

    let data = vec![0x33u8; 2 * 4096 + 17];
    let key = [0x22u8; 32];

    let before_bytes = alloc_counter::counters().live_bytes;
    let before_allocs = alloc_counter::counters().live_allocations;

    for i in 0..1000usize {
        // Vary the context length too: it must not be retained anywhere.
        let context = vec![0xa5u8; i % 41];
        let mut out = [0u8; 64];
        TurboHash::hash(&context, &data, &mut out);
        TurboHash::mac(&key, &context, &data, &mut out);
    }

    assert_eq!(
        before_bytes,
        alloc_counter::counters().live_bytes,
        "turbohash retained heap memory"
    );
    assert_eq!(
        before_allocs,
        alloc_counter::counters().live_allocations,
        "turbohash leaked allocations"
    );
}
