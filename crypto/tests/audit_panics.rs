//! Regression tests for caller-controlled panics in the public `crypto` API.
//!
//! Every fallible constructor is exercised with its boundary values inside
//! `catch_unwind`, so a regression that turns an error path back into a panic
//! fails here rather than only in a downstream fuzz run.

#![cfg(feature = "std")]

use std::panic::{AssertUnwindSafe, catch_unwind};

use crypto::{
    Aead, StreamCipher,
    aegis::{Aegis256, Aegis256Stream},
    aes::{Aes128Ctr, Aes128Gcm},
    ascon::{AsconAead128, AsconCxof128, AsconCxofError},
    blake2::{Blake2Error, Blake2b},
    chacha::{ChaCha, ChaCha20Poly1305},
    encoding::{der, ecdsa, pem, pkcs8},
    pbkdf2::{self, Pbkdf2Error},
    rsa,
    sha2::Sha256,
    sha3::{ParallelHash256, ParallelHashError, TurboShake256, TurboShakeError},
};
use rand::{RngExt, SeedableRng, rngs::StdRng};

fn no_panic<T>(f: impl FnOnce() -> T) -> T {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(_) => panic!("public API panicked on caller-controlled input"),
    }
}

#[test]
fn blake2_rejects_invalid_parameters_without_panicking() {
    let too_long_key = [0u8; 65];

    let err = no_panic(|| Blake2b::new_keyed(&too_long_key, 32).err());
    assert_eq!(err, Some(Blake2Error::KeyTooLong));

    let err = no_panic(|| Blake2b::new_keyed(&[], 0).err());
    assert_eq!(err, Some(Blake2Error::InvalidOutputLength));

    let err = no_panic(|| Blake2b::new_keyed(&[], 65).err());
    assert_eq!(err, Some(Blake2Error::InvalidOutputLength));

    // Boundary values remain valid.
    assert!(no_panic(|| Blake2b::new_keyed(&[0u8; 64], 1).is_ok()));
    assert!(no_panic(|| Blake2b::new_keyed(&[0u8; 64], 64).is_ok()));
}

#[test]
fn pbkdf2_rejects_zero_iterations_without_panicking() {
    let err = no_panic(|| pbkdf2::derive::<Sha256, 32>(b"password", b"salt", 0).err());
    assert_eq!(err, Some(Pbkdf2Error::ZeroIterations));

    assert!(no_panic(|| pbkdf2::derive::<Sha256, 32>(b"password", b"salt", 1).is_ok()));
}

#[test]
fn ascon_cxof_rejects_oversized_customization_without_panicking() {
    let too_long = [0u8; 257];
    let err = no_panic(|| AsconCxof128::new_with_customization(&too_long).err());
    assert_eq!(err, Some(AsconCxofError::CustomizationTooLong));

    // The NIST limit itself is accepted.
    assert!(no_panic(|| AsconCxof128::new_with_customization(&[0u8; 256]).is_ok()));
    assert!(no_panic(|| AsconCxof128::new_with_customization(&[]).is_ok()));
}

#[test]
fn parallel_hash_rejects_zero_block_size_without_panicking() {
    let err = no_panic(|| ParallelHash256::new(0, b"").err());
    assert_eq!(err, Some(ParallelHashError::ZeroBlockSize));

    let mut out = [0u8; 32];
    let err = no_panic(|| ParallelHash256::hash(b"data", 0, b"", &mut out).err());
    assert_eq!(err, Some(ParallelHashError::ZeroBlockSize));

    // One byte is the smallest legal block size.
    assert!(no_panic(|| ParallelHash256::new(1, b"").is_ok()));
    assert!(no_panic(|| ParallelHash256::hash(b"data", 1, b"", &mut out).is_ok()));
}

#[test]
fn turboshake_rejects_invalid_domain_separators_without_panicking() {
    for domain in [0x00u8, 0x80, 0xff] {
        let err = no_panic(|| TurboShake256::new(domain).err());
        assert_eq!(err, Some(TurboShakeError::InvalidDomainSeparator));
    }

    // Both ends of the RFC 9861 range are valid.
    assert!(no_panic(|| TurboShake256::new(0x01).is_ok()));
    assert!(no_panic(|| TurboShake256::new(0x7f).is_ok()));
}

#[test]
fn aead_decrypt_never_panics_on_bad_lengths() {
    let aes = Aes128Gcm::new(&[0x11; 16]);
    let chacha = ChaCha20Poly1305::new(&[0x22; 32]);
    let aegis = Aegis256::<16>::new(&[0x33; 32]);
    let ascon = AsconAead128::new(&[0x44; 16]);

    no_panic(|| {
        for nonce_len in 0..=40 {
            for tag_len in 0..=40 {
                let nonce = vec![0u8; nonce_len];
                let tag = vec![0u8; tag_len];
                let mut buf = [0u8; 17];

                let _ = aes.decrypt_in_place(&mut buf, &nonce, b"aad", &tag);
                let _ = chacha.decrypt_in_place(&mut buf, &nonce, b"aad", &tag);
                let _ = aegis.decrypt_in_place(&mut buf, &nonce, b"aad", &tag);
                let _ = ascon.decrypt_in_place(&mut buf, &nonce, b"aad", &tag);
            }
        }
    });
}

#[test]
fn stream_ciphers_never_panic_on_arbitrary_lengths() {
    let mut aes = Aes128Ctr::new(&[0x11; 16]);
    let mut chacha = ChaCha::<20, true>::new(&[0x22; 32], &[0x33; 12]);
    let mut aegis = Aegis256Stream::new(&[0x44; 32], &[0x55; 32]);

    no_panic(|| {
        for len in 0..=257 {
            let mut buf = vec![0u8; len];
            let _ = aes.xor_keystream(&mut buf);
            let _ = chacha.xor_keystream(&mut buf);
            let _ = aegis.xor_keystream(&mut buf);
        }
    });
}

#[test]
fn parsers_never_panic_on_random_input() {
    let mut rng = StdRng::seed_from_u64(0x9e37_79b9_7f4a_7c15);

    no_panic(|| {
        for len in 0..=256 {
            let mut input = vec![0u8; len];
            rng.fill(&mut input);

            let mut reader = der::Reader::new(&input);
            while reader.read_tlv().is_ok() {}
            let _ = reader.peek_tag();
            let _ = reader.finish();

            for item in pem::decode(&input) {
                let _ = item;
            }

            let _ = pkcs8::decode_p256_pkcs8_der(&input);
            let _ = pkcs8::decode_ec_sec1_der(&input);
            let _ = pkcs8::decode_ec_pkcs8_der(&input);
            let _ = pkcs8::decode_ed25519_pkcs8_der(&input);
            let _ = pkcs8::decode_mldsa_pkcs8_der(&input);
            let _ = pkcs8::is_rsa_pkcs8_der(&input);
            let _ = rsa::PublicKey::from_pkcs1_der(&input);

            for out_len in [0usize, 1, 2, 63, 64, 65, 96, 97] {
                let mut raw = vec![0u8; out_len];
                let _ = ecdsa::decode_signature_der(&input, &mut raw);
            }
        }
    });
}
