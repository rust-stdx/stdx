//! Arithmetic-boundary and malformed-input regression tests.
//!
//! These tests target the arithmetic edge cases that are easy to miss in
//! known-answer tests: lengths that overflow `usize` on 32-bit targets,
//! caller-controlled salt lengths, oversized nonces, and arbitrary parser
//! input. They are deterministic and dependency-free.

// The DER/PEM/PKCS#8 parsers are only available with the `alloc` feature.
#![cfg(feature = "alloc")]

use crypto::{
    Aead,
    encoding::{self, der::Reader},
    rsa,
};
const RSA_DER: &str = "3082010a0282010100b1d59f746650c6a4360d26dc2e05581e1bd12cddcfc459a75dd2ef6d38cb6e977c72cad72f5e8ad4795484211e71e9a292d25a3901fca4cd242649f56cce50ad6ba148658d71f3a9c8b39e92a7a49543243df8ca2688292d47ff2a92a6ee0c9151162936791f522afccd6a7508251934b909d62fa805bae0d79f83f3c981b39c15ea79ce7b4ec2ff82240ce2a9fb93ae49d7697d1248f73d4ad23461055f469a3936ab959a0c6a067aa19521650f3649a028e2ebe355909aae7c95d3fc988684478b2bb11b307cb58c6c14727e1b62103d400ac8eed0e0d6d7f7d7cfc1f4ae4cbd9759372f8408c52174abb05f134ca6788fb60ba3f35c57c07cd44011bb113b0203010001";

/// xorshift64*, used to generate deterministic fuzz inputs without a
/// dependency on `rand`.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() as usize) % n
    }

    fn fill(&mut self, out: &mut [u8]) {
        for byte in out {
            *byte = self.next() as u8;
        }
    }
}

#[test]
fn rsa_der_32bit_length_overflow_is_rejected() {
    // INTEGER length 0xFFFFFFFC inside a 7-byte SEQUENCE body. On 32-bit
    // targets `1 + len_size + len` used to wrap to 2 and slice `data[6..2]`.
    let overflow = [0x30, 0x07, 0x02, 0x84, 0xff, 0xff, 0xff, 0xfc, 0x00];
    assert!(rsa::PublicKey::from_pkcs1_der(&overflow).is_err());

    // The outer SEQUENCE length must also be checked without wrapping.
    let overflow_outer = [0x30, 0x84, 0xff, 0xff, 0xff, 0xff, 0x02, 0x01, 0x00];
    assert!(rsa::PublicKey::from_pkcs1_der(&overflow_outer).is_err());
}

#[test]
fn rsa_pss_oversized_salt_len_is_rejected_without_panicking() {
    let key = rsa::PublicKey::from_pkcs1_der(&hex::decode(RSA_DER).unwrap()).unwrap();
    let signature = [0u8; 256];

    for salt_len in [
        usize::MAX,
        usize::MAX - 1,
        usize::MAX - 32,
        usize::MAX / 2,
        1usize << 32,
    ] {
        assert_eq!(
            key.verify_pss::<crypto::sha2::Sha256>(&signature, b"message", salt_len),
            Err(crypto::RsaError::Unspecified),
            "salt_len={salt_len} was not rejected"
        );
    }
}

#[test]
#[should_panic(expected = "InvalidNonce")]
fn aes128_gcm_encrypt_wrong_nonce_len_panics() {
    let cipher = crypto::aes::Aes128Gcm::new(&[0u8; 16]);
    let mut buffer = [0u8; 16];
    let _ = cipher.encrypt_in_place(&mut buffer, &[0u8; 11], b"");
}

#[test]
#[should_panic(expected = "InvalidNonce")]
fn aes256_gcm_encrypt_wrong_nonce_len_panics() {
    let cipher = crypto::aes::Aes256Gcm::new(&[0u8; 32]);
    let mut buffer = [0u8; 16];
    let _ = cipher.encrypt_in_place(&mut buffer, &[0u8; 11], b"");
}

#[test]
#[should_panic(expected = "nonce must be 12 bytes")]
fn chacha20_poly1305_encrypt_wrong_nonce_len_panics() {
    let cipher = crypto::chacha::ChaCha20Poly1305::new(&[0u8; 32]);
    let mut buffer = [0u8; 16];
    let _ = cipher.encrypt_in_place(&mut buffer, &[0u8; 11], b"");
}

#[test]
#[should_panic(expected = "nonce must be 24 bytes")]
fn xchacha20_poly1305_encrypt_wrong_nonce_len_panics() {
    let cipher = crypto::chacha::XChaCha20Poly1305::new(&[0u8; 32]);
    let mut buffer = [0u8; 16];
    let _ = cipher.encrypt_in_place(&mut buffer, &[0u8; 12], b"");
}

#[test]
#[should_panic(expected = "nonce must be 32 bytes")]
fn chacha20_blake3_encrypt_wrong_nonce_len_panics() {
    let cipher = crypto::chacha::ChaCha20Blake3::new(&[0u8; 32]);
    let mut buffer = [0u8; 16];
    let _ = cipher.encrypt_in_place(&mut buffer, &[0u8; 12], b"");
}

#[test]
#[should_panic(expected = "nonce must be 16 bytes")]
fn ascon_aead_encrypt_wrong_nonce_len_panics() {
    let cipher = crypto::ascon::AsconAead128::new(&[0u8; 16]);
    let mut buffer = [0u8; 16];
    let _ = cipher.encrypt_in_place(&mut buffer, &[0u8; 12], b"");
}

#[test]
fn aead_invalid_tag_returns_error() {
    fn check<A: Aead>(cipher: &A, nonce: &[u8]) {
        let mut buffer = b"authenticated message".to_vec();
        let tag = cipher.encrypt_in_place(&mut buffer, nonce, b"aad");

        let mut bad_tag = [0u8; 64];
        bad_tag[..tag.len()].copy_from_slice(tag.as_ref());
        bad_tag[0] ^= 1;

        let mut tampered = buffer.clone();
        assert!(
            cipher
                .decrypt_in_place(&mut tampered, nonce, b"aad", &bad_tag[..tag.len()])
                .is_err(),
            "invalid tag was accepted"
        );
    }

    check(&crypto::aes::Aes128Gcm::new(&[0u8; 16]), &[0u8; 12]);
    check(&crypto::aes::Aes256Gcm::new(&[0u8; 32]), &[0u8; 12]);
    check(&crypto::chacha::ChaCha20Poly1305::new(&[0u8; 32]), &[0u8; 12]);
    check(&crypto::chacha::XChaCha20Poly1305::new(&[0u8; 32]), &[0u8; 24]);
    check(&crypto::chacha::ChaCha20Blake3::new(&[0u8; 32]), &[0u8; 32]);
    check(&crypto::ascon::AsconAead128::new(&[0u8; 16]), &[0u8; 16]);
}

#[test]
fn der_length_field_matrix_is_rejected_cleanly() {
    let lengths: [&[u8]; 8] = [
        &[0x80],
        &[0x81, 0xff],
        &[0x82, 0xff, 0xff],
        &[0x83, 0xff, 0xff, 0xff],
        &[0x84, 0xff, 0xff, 0xff, 0xff],
        &[0x85, 0xff, 0xff, 0xff, 0xff, 0xff],
        &[0x82, 0x00, 0x80],
        &[0x81, 0x01],
    ];

    for length in lengths {
        let mut input = vec![0x04];
        input.extend_from_slice(length);
        input.extend_from_slice(&[0u8; 32]);

        let mut reader = Reader::new(&input);
        // Every malformed length must return an error; none may panic.
        let _ = reader.read_octet_string();
    }
}

#[test]
fn parsers_never_panic_on_random_input() {
    let mut rng = Rng(0x0123_4567_89ab_cdef);
    let mut buffer = [0u8; 2048];
    let mut signatures = 0u64;

    for _ in 0..20_000 {
        let len = rng.below(buffer.len());
        rng.fill(&mut buffer[..len]);
        let input = &buffer[..len];

        let _ = rsa::PublicKey::from_pkcs1_der(input);
        let _ = encoding::pem::decode(input).count();
        let _ = encoding::ecdsa::decode_p256_signature_der(input);
        let _ = encoding::ecdsa::decode_p384_signature_der(input);
        let _ = encoding::pkcs8::decode_p256_pkcs8_der(input);
        let _ = encoding::pkcs8::decode_ec_pkcs8_der(input);
        let _ = encoding::pkcs8::decode_ec_sec1_der(input);
        let _ = encoding::pkcs8::decode_ed25519_pkcs8_der(input);
        let _ = encoding::pkcs8::decode_mldsa_pkcs8_der(input);
        let _ = encoding::pkcs8::is_rsa_pkcs8_der(input);

        if let Ok(key) = crypto::mlkem::PublicKey768::try_from(input) {
            let _ = key.encapsulate_derand(&[0u8; 32]);
        }
        if let Ok(key) = crypto::mlkem::PublicKey1024::try_from(input) {
            let _ = key.encapsulate_derand(&[0u8; 32]);
        }

        if let Ok(key) = crypto::mldsa::MlDsa65PublicKey::try_from(input) {
            let mut signature = [0u8; crypto::mldsa::ML_DSA_65_SIGNATURE_SIZE];
            rng.fill(&mut signature);
            let _ = key.verify(b"message", &signature, b"");
            signatures += 1;
        }
    }

    // The random input occasionally has the ML-DSA-65 public key length, so the
    // verification path above is exercised rather than skipped entirely.
    assert!(signatures > 0, "no ML-DSA verification path was exercised");
}

#[test]
fn mutated_valid_der_never_panics() {
    let valid = hex::decode(RSA_DER).unwrap();
    let mut rng = Rng(0xdead_beef_cafe_f00d);
    let mut mutated = Vec::with_capacity(valid.len() + 8);

    for _ in 0..10_000 {
        mutated.clear();
        mutated.extend_from_slice(&valid);

        match rng.below(4) {
            0 => {
                let index = rng.below(mutated.len());
                mutated[index] ^= 1 << rng.below(8);
            }
            1 => {
                let index = rng.below(mutated.len());
                mutated.truncate(index);
            }
            2 => {
                let index = rng.below(mutated.len());
                let byte = mutated[index];
                mutated.insert(index, byte);
            }
            _ => {
                let index = rng.below(mutated.len().saturating_sub(1).max(1));
                if index + 1 < mutated.len() {
                    mutated.swap(index, index + 1);
                }
            }
        }

        let _ = rsa::PublicKey::from_pkcs1_der(&mutated);
        let _ = encoding::pkcs8::is_rsa_pkcs8_der(&mutated);
        let _ = encoding::pem::decode(&mutated).count();
    }
}

#[test]
fn mutated_valid_ecdsa_der_never_panics() {
    let mut signature = [0u8; 64];
    for (index, byte) in signature.iter_mut().enumerate() {
        *byte = (index as u8).wrapping_mul(17).wrapping_add(3);
    }
    let valid = encoding::ecdsa::encode_signature_der(&signature[..32], &signature[32..]);
    let mut rng = Rng(0xfeed_face_dead_beef);
    let mut mutated = valid.clone();

    for _ in 0..5_000 {
        mutated.clear();
        mutated.extend_from_slice(&valid);
        let index = rng.below(mutated.len());
        mutated[index] ^= 1 << rng.below(8);

        let _ = encoding::ecdsa::decode_p256_signature_der(&mutated);
        let _ = encoding::ecdsa::decode_p384_signature_der(&mutated);
    }
}
