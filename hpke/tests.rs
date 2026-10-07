//! HPKE tests, including the official RFC 9180 test vector corpus.

use crypto::{aes::Aes256Gcm, chacha::ChaCha20Poly1305};
use serde_json::Value;

use crate::{
    HpkeError, RecipientMode, SenderMode,
    aead::ExportOnly,
    kdf::{Blake3, HkdfSha256, HkdfSha512, Kdf, Shake256},
    kem::{Kem, MLKEM768X25519, P256HkdfSha256, P521HkdfSha512, X25519HkdfSha256},
};

/// Decodes a hex field, treating a missing field as empty.
fn hex_field(value: &Value, key: &str) -> Vec<u8> {
    match value.get(key).and_then(Value::as_str) {
        Some(s) => hex::decode(s).unwrap(),
        None => Vec::new(),
    }
}

/// Checks a single RFC 9180 test vector against the recipient-side key schedule.
///
/// The sender's key schedule is validated indirectly: re-encapsulating with
/// the vector's ephemeral key must reproduce `encapped_key` and `shared_secret`, and
/// opening the vector's ciphertexts and reproducing its exported values
/// exercises the complete key schedule.
fn check_vector<K: Kem, D: crate::kdf::Kdf, A: crate::aead::Aead>(vector: &Value) {
    let mode = vector["mode"].as_u64().unwrap() as u8;
    let info = hex_field(vector, "info");

    // Recipient key pair.
    let (recipient_secret_key, recipient_public_key) = K::derive_keypair(&hex_field(vector, "ikmR")).unwrap();
    let mut recipient_public_key_bytes = vec![0u8; K::PUBLIC_KEY_SIZE];
    K::public_key_to_bytes(&mut recipient_public_key_bytes, &recipient_public_key).unwrap();
    assert_eq!(
        hex::encode(&recipient_public_key_bytes),
        vector["pkRm"].as_str().unwrap(),
        "pkRm mismatch"
    );

    // Ephemeral key pair.
    let (ephemeral_secret_key, ephemeral_public_key) = K::derive_keypair(&hex_field(vector, "ikmE")).unwrap();
    let mut ephemeral_public_key_bytes = vec![0u8; K::PUBLIC_KEY_SIZE];
    K::public_key_to_bytes(&mut ephemeral_public_key_bytes, &ephemeral_public_key).unwrap();
    assert_eq!(
        hex::encode(&ephemeral_public_key_bytes),
        vector["pkEm"].as_str().unwrap(),
        "pkEm mismatch"
    );

    // Sender static key pair, for the authenticated modes.
    let sender_static = if mode >= 2 {
        let (sender_secret_key, sender_public_key) = K::derive_keypair(&hex_field(vector, "ikmS")).unwrap();
        let mut sender_public_key_bytes = vec![0u8; K::PUBLIC_KEY_SIZE];
        K::public_key_to_bytes(&mut sender_public_key_bytes, &sender_public_key).unwrap();
        assert_eq!(
            hex::encode(&sender_public_key_bytes),
            vector["pkSm"].as_str().unwrap(),
            "pkSm mismatch"
        );
        Some((sender_secret_key, sender_public_key))
    } else {
        None
    };

    // Encapsulation.
    let (shared_secret, encapped_key) = match &sender_static {
        Some((sender_secret_key, _)) => {
            K::authenticated_encap_with_ephemeral(&ephemeral_secret_key, &recipient_public_key, sender_secret_key)
                .unwrap()
        }
        None => K::encap_with_ephemeral(&ephemeral_secret_key, &recipient_public_key).unwrap(),
    };
    assert_eq!(
        shared_secret.as_ref(),
        hex_field(vector, "shared_secret").as_slice(),
        "shared_secret mismatch"
    );
    let mut encapped_key_bytes = vec![0u8; K::ENCAPPED_KEY_SIZE];
    K::encapped_key_to_bytes(&mut encapped_key_bytes, &encapped_key).unwrap();
    assert_eq!(
        hex::encode(&encapped_key_bytes),
        vector["enc"].as_str().unwrap(),
        "enc mismatch"
    );

    // Recipient setup.
    let encapped_key_from_vector = K::encapped_key_from_bytes(&hex_field(vector, "enc")).unwrap();
    let pre_shared_key = hex_field(vector, "psk");
    let pre_shared_key_id = hex_field(vector, "psk_id");
    let recipient_mode = match mode {
        0 => RecipientMode::<K>::Base,
        1 => RecipientMode::PreSharedKey {
            pre_shared_key: &pre_shared_key,
            pre_shared_key_id: &pre_shared_key_id,
        },
        2 => RecipientMode::Authenticated {
            sender_public_key: &sender_static.as_ref().unwrap().1,
        },
        3 => RecipientMode::AuthenticatedPreSharedKey {
            pre_shared_key: &pre_shared_key,
            pre_shared_key_id: &pre_shared_key_id,
            sender_public_key: &sender_static.as_ref().unwrap().1,
        },
        _ => panic!("unexpected mode {mode}"),
    };
    let mut recipient =
        crate::new_recipient::<K, D, A>(&recipient_mode, &recipient_secret_key, &encapped_key_from_vector, &info)
            .unwrap();

    // Encryptions. The Export-Only pseudo-AEAD has no encryptions.
    if vector["aead_id"].as_u64().unwrap() != 0xffff {
        for (i, encryption) in vector["encryptions"].as_array().unwrap().iter().enumerate() {
            let associated_data = hex_field(encryption, "aad");
            let ciphertext = hex_field(encryption, "ct");
            let plaintext = hex_field(encryption, "pt");

            let split = ciphertext.len() - A::TAG_SIZE;
            let (body, tag) = ciphertext.split_at(split);
            let mut buffer = body.to_vec();
            recipient
                .open_in_place(&mut buffer, &associated_data, tag)
                .unwrap_or_else(|err| panic!("encryption {i}: {err}"));
            assert_eq!(buffer, plaintext, "encryption {i} plaintext mismatch");
        }
    }

    // Exported values.
    for export in vector["exports"].as_array().unwrap() {
        let exporter_context = hex_field(export, "exporter_context");
        let len = export["L"].as_u64().unwrap() as usize;
        let mut out = vec![0u8; len];
        recipient.export(&mut out, &exporter_context).unwrap();
        assert_eq!(
            hex::encode(&out),
            export["exported_value"].as_str().unwrap(),
            "exported value mismatch"
        );
    }
}

macro_rules! check_aead {
    ($kem:ty, $vector:expr) => {{
        let kdf_id = $vector["kdf_id"].as_u64().unwrap() as u16;
        let aead_id = $vector["aead_id"].as_u64().unwrap() as u16;
        match (kdf_id, aead_id) {
            (0x0001, 0x0002) => check_vector::<$kem, HkdfSha256, Aes256Gcm>($vector),
            (0x0001, 0x0003) => check_vector::<$kem, HkdfSha256, ChaCha20Poly1305>($vector),
            (0x0001, 0xffff) => check_vector::<$kem, HkdfSha256, ExportOnly>($vector),
            (0x0003, 0x0002) => check_vector::<$kem, HkdfSha512, Aes256Gcm>($vector),
            (0x0003, 0x0003) => check_vector::<$kem, HkdfSha512, ChaCha20Poly1305>($vector),
            (0x0003, 0xffff) => check_vector::<$kem, HkdfSha512, ExportOnly>($vector),
            (0x0011, 0x0002) => check_vector::<$kem, Shake256, Aes256Gcm>($vector),
            (0x0011, 0x0003) => check_vector::<$kem, Shake256, ChaCha20Poly1305>($vector),
            (0x0011, 0xffff) => check_vector::<$kem, Shake256, ExportOnly>($vector),
            _ => panic!("unexpected KDF/AEAD combination {kdf_id:#06x}/{aead_id:#06x}"),
        }
    }};
}

/// Checks a single test vector from `draft-ietf-hpke-pq-05`.
///
/// Unlike the RFC 9180 vectors, hybrid KEMs do not derive an ephemeral key
/// pair from `ikmE`: `ikmE` is the KEM's deterministic encapsulation
/// randomness, consumed by [`Kem::encap_deterministic`].
fn check_pq_vector<K: Kem, D: crate::kdf::Kdf, A: crate::aead::Aead>(vector: &Value) {
    let info = hex_field(vector, "info");

    // Recipient key pair.
    let (recipient_secret_key, recipient_public_key) = K::derive_keypair(&hex_field(vector, "ikmR")).unwrap();
    let mut recipient_public_key_bytes = vec![0u8; K::PUBLIC_KEY_SIZE];
    K::public_key_to_bytes(&mut recipient_public_key_bytes, &recipient_public_key).unwrap();
    assert_eq!(
        hex::encode(&recipient_public_key_bytes),
        vector["pkRm"].as_str().unwrap(),
        "pkRm mismatch"
    );

    // Deterministic encapsulation from the vector's randomness.
    let (shared_secret, encapped_key) =
        K::encap_deterministic(&recipient_public_key, &hex_field(vector, "ikmE")).unwrap();
    assert_eq!(
        shared_secret.as_ref(),
        hex_field(vector, "shared_secret").as_slice(),
        "shared_secret mismatch"
    );
    let mut encapped_key_bytes = vec![0u8; K::ENCAPPED_KEY_SIZE];
    K::encapped_key_to_bytes(&mut encapped_key_bytes, &encapped_key).unwrap();
    assert_eq!(
        hex::encode(&encapped_key_bytes),
        vector["enc"].as_str().unwrap(),
        "enc mismatch"
    );

    // Recipient setup (the PQ vectors only exercise Base mode).
    let encapped_key_from_vector = K::encapped_key_from_bytes(&hex_field(vector, "enc")).unwrap();
    let mut recipient = crate::new_recipient::<K, D, A>(
        &RecipientMode::<K>::Base,
        &recipient_secret_key,
        &encapped_key_from_vector,
        &info,
    )
    .unwrap();

    // Encryptions. The Export-Only pseudo-AEAD has no encryptions.
    if vector["aead_id"].as_u64().unwrap() != 0xffff {
        for (i, encryption) in vector["encryptions"].as_array().unwrap().iter().enumerate() {
            let associated_data = hex_field(encryption, "aad");
            let ciphertext = hex_field(encryption, "ct");
            let plaintext = hex_field(encryption, "pt");

            let split = ciphertext.len() - A::TAG_SIZE;
            let (body, tag) = ciphertext.split_at(split);
            let mut buffer = body.to_vec();
            recipient
                .open_in_place(&mut buffer, &associated_data, tag)
                .unwrap_or_else(|err| panic!("encryption {i}: {err}"));
            assert_eq!(buffer, plaintext, "encryption {i} plaintext mismatch");
        }
    }

    // Exported values.
    for export in vector["exports"].as_array().unwrap() {
        let exporter_context = hex_field(export, "exporter_context");
        let len = export["L"].as_u64().unwrap() as usize;
        let mut out = vec![0u8; len];
        recipient.export(&mut out, &exporter_context).unwrap();
        assert_eq!(
            hex::encode(&out),
            export["exported_value"].as_str().unwrap(),
            "exported value mismatch"
        );
    }
}

#[test]
fn hpke_pq_vectors() {
    let data: Value = serde_json::from_str(include_str!("testdata/hpke-pq-test-vectors.json")).unwrap();
    let mut tested = 0usize;

    for vector in data.as_array().unwrap() {
        let kem_id = vector["kem_id"].as_u64().unwrap() as u16;
        let kdf_id = vector["kdf_id"].as_u64().unwrap() as u16;
        let aead_id = vector["aead_id"].as_u64().unwrap() as u16;

        assert_eq!(kem_id, 0x647a, "only MLKEM768-X25519 PQ vectors are bundled");
        assert_eq!(aead_id, 0x0003, "only the ChaCha20-Poly1305 PQ vectors are bundled");

        match kdf_id {
            0x0001 => check_pq_vector::<MLKEM768X25519, HkdfSha256, ChaCha20Poly1305>(vector),
            0x0011 => check_pq_vector::<MLKEM768X25519, Shake256, ChaCha20Poly1305>(vector),
            _ => panic!("unexpected kdf_id {kdf_id:#06x}"),
        }
        tested += 1;
    }

    assert_eq!(tested, 2, "expected 2 supported draft-ietf-hpke-pq-05 vectors");
}

#[test]
fn rfc9180_vectors() {
    let data: Value = serde_json::from_str(include_str!("testdata/test-vectors.json")).unwrap();
    let mut tested = 0usize;

    for vector in data.as_array().unwrap() {
        let kem_id = vector["kem_id"].as_u64().unwrap() as u16;
        let aead_id = vector["aead_id"].as_u64().unwrap() as u16;

        // AES-128-GCM (0x0001) is not shipped.
        if !matches!(aead_id, 0x0002 | 0x0003 | 0xffff) {
            continue;
        }

        match kem_id {
            0x0020 => check_aead!(X25519HkdfSha256, vector),
            0x0010 => check_aead!(P256HkdfSha256, vector),
            0x0012 => check_aead!(P521HkdfSha512, vector),
            // P-384 (0x0011) and X448 (0x0021) are not implemented.
            0x0011 | 0x0021 => continue,
            _ => panic!("unexpected kem_id {kem_id:#06x}"),
        }
        tested += 1;
    }

    assert_eq!(tested, 72, "expected 72 supported RFC 9180 test vectors");
}

#[test]
fn roundtrip_all_suites() {
    fn roundtrip<K: Kem, D: crate::kdf::Kdf, A: crate::aead::Aead>() {
        let (recipient_secret_key, recipient_public_key) = K::generate_keypair().unwrap();
        let (encapped_key, mut sender) =
            crate::new_sender::<K, D, A>(&SenderMode::Base, &recipient_public_key, b"info").unwrap();
        let ciphertext = sender.seal(b"fronthand or backhand?", b"a gentleman's game").unwrap();

        let mut recipient =
            crate::new_recipient::<K, D, A>(&RecipientMode::Base, &recipient_secret_key, &encapped_key, b"info")
                .unwrap();
        let plaintext = recipient.open(&ciphertext, b"a gentleman's game").unwrap();
        assert_eq!(plaintext, b"fronthand or backhand?");

        // A second message must use the next sequence number.
        let ciphertext = sender.seal(b"second message", b"aad").unwrap();
        assert_eq!(recipient.open(&ciphertext, b"aad").unwrap(), b"second message");
    }

    roundtrip::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>();
    roundtrip::<X25519HkdfSha256, HkdfSha256, ChaCha20Poly1305>();
    roundtrip::<P256HkdfSha256, HkdfSha256, Aes256Gcm>();
    roundtrip::<P521HkdfSha512, HkdfSha512, Aes256Gcm>();
    roundtrip::<MLKEM768X25519, HkdfSha256, Aes256Gcm>();
    roundtrip::<MLKEM768X25519, HkdfSha512, ChaCha20Poly1305>();
    roundtrip::<MLKEM768X25519, Shake256, Aes256Gcm>();
    roundtrip::<MLKEM768X25519, Shake256, ChaCha20Poly1305>();
    roundtrip::<MLKEM768X25519, Blake3, Aes256Gcm>();
    roundtrip::<MLKEM768X25519, Blake3, ChaCha20Poly1305>();
}

#[test]
fn roundtrip_authenticated_modes() {
    fn roundtrip<K: Kem, D: crate::kdf::Kdf, A: crate::aead::Aead>() {
        let (recipient_secret_key, recipient_public_key) = K::generate_keypair().unwrap();
        let (sender_secret_key, sender_public_key) = K::generate_keypair().unwrap();

        // Authenticated mode.
        let (encapped_key, mut sender) = crate::new_sender::<K, D, A>(
            &SenderMode::Authenticated {
                sender_secret_key: &sender_secret_key,
            },
            &recipient_public_key,
            b"info",
        )
        .unwrap();
        let ciphertext = sender.seal(b"auth message", b"aad").unwrap();
        let mut recipient = crate::new_recipient::<K, D, A>(
            &RecipientMode::Authenticated {
                sender_public_key: &sender_public_key,
            },
            &recipient_secret_key,
            &encapped_key,
            b"info",
        )
        .unwrap();
        assert_eq!(recipient.open(&ciphertext, b"aad").unwrap(), b"auth message");

        // AuthenticatedPreSharedKey mode.
        let pre_shared_key = [0x42u8; 32];
        let pre_shared_key_id = b"psk id";
        let (encapped_key, mut sender) = crate::new_sender::<K, D, A>(
            &SenderMode::AuthenticatedPreSharedKey {
                pre_shared_key: &pre_shared_key,
                pre_shared_key_id,
                sender_secret_key: &sender_secret_key,
            },
            &recipient_public_key,
            b"info",
        )
        .unwrap();
        let ciphertext = sender.seal(b"auth-psk message", b"aad").unwrap();
        let mut recipient = crate::new_recipient::<K, D, A>(
            &RecipientMode::AuthenticatedPreSharedKey {
                pre_shared_key: &pre_shared_key,
                pre_shared_key_id,
                sender_public_key: &sender_public_key,
            },
            &recipient_secret_key,
            &encapped_key,
            b"info",
        )
        .unwrap();
        assert_eq!(recipient.open(&ciphertext, b"aad").unwrap(), b"auth-psk message");
    }

    roundtrip::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>();
    roundtrip::<P256HkdfSha256, HkdfSha256, ChaCha20Poly1305>();
}

#[test]
fn roundtrip_pre_shared_key_mode() {
    let (recipient_secret_key, recipient_public_key) = X25519HkdfSha256::generate_keypair().unwrap();
    let pre_shared_key = [0x11u8; 32];
    let pre_shared_key_id = b"psk id";

    let (encapped_key, mut sender) = crate::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
        &SenderMode::PreSharedKey {
            pre_shared_key: &pre_shared_key,
            pre_shared_key_id,
        },
        &recipient_public_key,
        b"info",
    )
    .unwrap();
    let ciphertext = sender.seal(b"psk message", b"").unwrap();

    let mut recipient = crate::new_recipient::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
        &RecipientMode::PreSharedKey {
            pre_shared_key: &pre_shared_key,
            pre_shared_key_id,
        },
        &recipient_secret_key,
        &encapped_key,
        b"info",
    )
    .unwrap();
    assert_eq!(recipient.open(&ciphertext, b"").unwrap(), b"psk message");

    // A different pre-shared key must not authenticate.
    let wrong_pre_shared_key = [0x22u8; 32];
    let mut recipient = crate::new_recipient::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
        &RecipientMode::PreSharedKey {
            pre_shared_key: &wrong_pre_shared_key,
            pre_shared_key_id,
        },
        &recipient_secret_key,
        &encapped_key,
        b"info",
    )
    .unwrap();
    assert_eq!(recipient.open(&ciphertext, b""), Err(HpkeError::OpenError));
}

#[test]
fn mlkem768x25519_is_not_an_authenticated_kem() {
    let (recipient_secret_key, recipient_public_key) = MLKEM768X25519::generate_keypair().unwrap();
    let (sender_secret_key, _sender_public_key) = MLKEM768X25519::generate_keypair().unwrap();

    let err = crate::new_sender::<MLKEM768X25519, HkdfSha256, Aes256Gcm>(
        &SenderMode::Authenticated {
            sender_secret_key: &sender_secret_key,
        },
        &recipient_public_key,
        b"",
    )
    .err()
    .unwrap();
    assert_eq!(err, HpkeError::NotSupported);

    let (encapped_key, _sender) =
        crate::new_sender::<MLKEM768X25519, HkdfSha256, Aes256Gcm>(&SenderMode::Base, &recipient_public_key, b"")
            .unwrap();
    let err = crate::new_recipient::<MLKEM768X25519, HkdfSha256, Aes256Gcm>(
        &RecipientMode::Authenticated {
            sender_public_key: &sender_secret_key.public_key(),
        },
        &recipient_secret_key,
        &encapped_key,
        b"",
    )
    .err()
    .unwrap();
    assert_eq!(err, HpkeError::NotSupported);
}

#[test]
fn export_only_suite() {
    let (recipient_secret_key, recipient_public_key) = X25519HkdfSha256::generate_keypair().unwrap();
    let (encapped_key, sender) = crate::new_sender::<X25519HkdfSha256, HkdfSha256, ExportOnly>(
        &SenderMode::Base,
        &recipient_public_key,
        b"info",
    )
    .unwrap();

    let mut sent = [0u8; 64];
    sender.export(&mut sent, b"exporter context").unwrap();

    let recipient = crate::new_recipient::<X25519HkdfSha256, HkdfSha256, ExportOnly>(
        &RecipientMode::Base,
        &recipient_secret_key,
        &encapped_key,
        b"info",
    )
    .unwrap();
    let mut received = [0u8; 64];
    recipient.export(&mut received, b"exporter context").unwrap();
    assert_eq!(sent, received);

    // Different exporter contexts produce different outputs.
    let mut other = [0u8; 64];
    recipient.export(&mut other, b"other context").unwrap();
    assert_ne!(sent, other);

    // Sealing and opening are not supported by the Export-Only pseudo-AEAD and
    // return an error rather than panicking.
    let mut sender = sender;
    assert_eq!(sender.seal(b"message", b""), Err(HpkeError::NotSupported));
    let mut recipient = recipient;
    assert_eq!(recipient.open(&[0u8; 32], b""), Err(HpkeError::NotSupported));
}

#[test]
fn export_output_too_long() {
    let (recipient_secret_key, recipient_public_key) = X25519HkdfSha256::generate_keypair().unwrap();
    let (encapped_key, sender) =
        crate::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&SenderMode::Base, &recipient_public_key, b"")
            .unwrap();
    let recipient = crate::new_recipient::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
        &RecipientMode::Base,
        &recipient_secret_key,
        &encapped_key,
        b"",
    )
    .unwrap();

    // HKDF-SHA256 can produce at most 255 * 32 bytes.
    let mut out = vec![0u8; 255 * 32 + 1];
    assert_eq!(sender.export(&mut out, b""), Err(HpkeError::KdfOutputTooLong));
    assert_eq!(recipient.export(&mut out, b""), Err(HpkeError::KdfOutputTooLong));
}

#[test]
fn tampered_ciphertext_is_rejected() {
    let (recipient_secret_key, recipient_public_key) = X25519HkdfSha256::generate_keypair().unwrap();
    let (encapped_key, mut sender) =
        crate::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&SenderMode::Base, &recipient_public_key, b"")
            .unwrap();
    let mut ciphertext = sender.seal(b"secret", b"").unwrap();

    let last = ciphertext.len() - 1;
    ciphertext[last] ^= 0x01;

    let mut recipient = crate::new_recipient::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
        &RecipientMode::Base,
        &recipient_secret_key,
        &encapped_key,
        b"",
    )
    .unwrap();
    assert_eq!(recipient.open(&ciphertext, b""), Err(HpkeError::OpenError));
}

#[test]
fn key_serialization_roundtrips() {
    fn roundtrip<K: Kem>() {
        let (secret_key, public_key) = K::generate_keypair().unwrap();

        let mut public_key_bytes = vec![0u8; K::PUBLIC_KEY_SIZE];
        K::public_key_to_bytes(&mut public_key_bytes, &public_key).unwrap();
        let deserialized_public_key = K::public_key_from_bytes(&public_key_bytes).unwrap();

        let mut secret_key_bytes = vec![0u8; K::SECRET_KEY_SIZE];
        K::secret_key_to_bytes(&mut secret_key_bytes, &secret_key).unwrap();
        let deserialized_secret_key = K::secret_key_from_bytes(&secret_key_bytes).unwrap();

        // The public key derived from both secret keys must match, and both
        // must agree with the serialized public key.
        let mut derived_public_key_bytes = vec![0u8; K::PUBLIC_KEY_SIZE];
        K::public_key_to_bytes(&mut derived_public_key_bytes, &K::derive_public_key(&deserialized_secret_key)).unwrap();
        assert_eq!(public_key_bytes, derived_public_key_bytes);
        assert_eq!(
            {
                let mut reencoded_public_key = vec![0u8; K::PUBLIC_KEY_SIZE];
                K::public_key_to_bytes(&mut reencoded_public_key, &deserialized_public_key).unwrap();
                reencoded_public_key
            },
            public_key_bytes
        );
    }

    roundtrip::<X25519HkdfSha256>();
    roundtrip::<P256HkdfSha256>();
    roundtrip::<P521HkdfSha512>();
    roundtrip::<MLKEM768X25519>();
}

#[test]
fn kdf_labeled_derivations() {
    let suite_id = b"HPKE\x00\x10\x00\x03\x00\x02";

    // `labeled_extract` is deterministic and returns exactly `OUTPUT_SIZE`
    // bytes.
    let pseudorandom_key = HkdfSha512::labeled_extract(b"", suite_id, b"label", b"ikm");
    let pseudorandom_key2 = HkdfSha512::labeled_extract(b"", suite_id, b"label", b"ikm");
    assert_eq!(pseudorandom_key.as_ref(), pseudorandom_key2.as_ref());
    assert_eq!(pseudorandom_key.len(), HkdfSha512::OUTPUT_SIZE);

    // `labeled_expand` writes exactly `out.len()` bytes and is deterministic.
    let mut a = [0u8; 40];
    let mut b = [0u8; 40];
    HkdfSha512::labeled_expand(&mut a, &pseudorandom_key, suite_id, b"label", b"info").unwrap();
    HkdfSha512::labeled_expand(&mut b, &pseudorandom_key, suite_id, b"label", b"info").unwrap();
    assert_eq!(a, b);
    assert_ne!(a, [0u8; 40]);

    // Output larger than 255 * OUTPUT_SIZE is rejected.
    let mut too_long = vec![0u8; 255 * 64 + 1];
    assert_eq!(
        HkdfSha512::labeled_expand(&mut too_long, &pseudorandom_key, suite_id, b"label", b"info"),
        Err(HpkeError::KdfOutputTooLong)
    );

    // A pseudorandom key that is not exactly OUTPUT_SIZE bytes is rejected.
    let mut out = [0u8; 32];
    assert_eq!(
        HkdfSha512::labeled_expand(&mut out, b"short pseudorandom key", suite_id, b"label", b"info"),
        Err(HpkeError::InvalidPseudorandomKey)
    );
    let long_pseudorandom_key = [0u8; 65];
    assert_eq!(
        HkdfSha512::labeled_expand(&mut out, &long_pseudorandom_key, suite_id, b"label", b"info"),
        Err(HpkeError::InvalidPseudorandomKey)
    );
}

#[test]
fn p256_rejects_compressed_public_key() {
    let (_, public_key) = P256HkdfSha256::generate_keypair().unwrap();
    let compressed = public_key.to_compressed_bytes();
    assert_eq!(compressed.len(), 33);
    assert_eq!(
        P256HkdfSha256::public_key_from_bytes(&compressed),
        Err(HpkeError::DeserializeError)
    );
}

#[test]
fn shake256_labeled_derive() {
    let suite_id = b"HPKE\x64\x7a\x00\x11\x00\x03";

    // Deterministic, non-degenerate output of the requested length.
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    Shake256::labeled_derive(&mut a, &[b"ikm"], suite_id, b"label", &[b"info"]).unwrap();
    Shake256::labeled_derive(&mut b, &[b"ikm"], suite_id, b"label", &[b"info"]).unwrap();
    assert_eq!(a, b);
    assert_ne!(a, [0u8; 32]);

    // Splitting an input across parts matches the concatenation.
    let mut c = [0u8; 32];
    Shake256::labeled_derive(&mut c, &[b"i", b"km"], suite_id, b"label", &[b"info"]).unwrap();
    assert_eq!(a, c);

    // A different label, suite id or context changes the output.
    let mut d = [0u8; 32];
    Shake256::labeled_derive(&mut d, &[b"ikm"], suite_id, b"other", &[b"info"]).unwrap();
    assert_ne!(a, d);

    // Output larger than 2^16 - 1 bytes is rejected.
    let mut too_long = vec![0u8; 1 << 16];
    assert_eq!(
        Shake256::labeled_derive(&mut too_long, &[b"ikm"], suite_id, b"label", &[]),
        Err(HpkeError::KdfOutputTooLong)
    );

    // Two-stage KDFs do not implement `labeled_derive`.
    assert_eq!(
        HkdfSha256::labeled_derive(&mut a, &[b"ikm"], suite_id, b"label", &[]),
        Err(HpkeError::NotSupported)
    );
}

#[test]
fn shake256_export_output_too_long() {
    let (recipient_secret_key, recipient_public_key) = MLKEM768X25519::generate_keypair().unwrap();
    let (encapped_key, sender) =
        crate::new_sender::<MLKEM768X25519, Shake256, Aes256Gcm>(&SenderMode::Base, &recipient_public_key, b"")
            .unwrap();
    let recipient = crate::new_recipient::<MLKEM768X25519, Shake256, Aes256Gcm>(
        &RecipientMode::Base,
        &recipient_secret_key,
        &encapped_key,
        b"",
    )
    .unwrap();

    // A single-stage KDF can produce at most 2^16 - 1 bytes.
    let mut out = vec![0u8; 1 << 16];
    assert_eq!(sender.export(&mut out, b""), Err(HpkeError::KdfOutputTooLong));
    assert_eq!(recipient.export(&mut out, b""), Err(HpkeError::KdfOutputTooLong));
}

#[test]
fn blake3_labeled_derive() {
    // The unofficial stdx identifier is encoded in the suite id.
    assert_eq!(Blake3::HPKE_KDF_ID, 0xFF01);

    let suite_id = b"HPKE\x64\x7a\xff\x01\x00\x03";

    // Deterministic, non-degenerate output of the requested length.
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    Blake3::labeled_derive(&mut a, &[b"ikm"], suite_id, b"label", &[b"info"]).unwrap();
    Blake3::labeled_derive(&mut b, &[b"ikm"], suite_id, b"label", &[b"info"]).unwrap();
    assert_eq!(a, b);
    assert_ne!(a, [0u8; 32]);

    // Splitting an input across parts matches the concatenation.
    let mut c = [0u8; 32];
    Blake3::labeled_derive(&mut c, &[b"i", b"km"], suite_id, b"label", &[b"info"]).unwrap();
    assert_eq!(a, c);

    // A different label changes the output.
    let mut d = [0u8; 32];
    Blake3::labeled_derive(&mut d, &[b"ikm"], suite_id, b"other", &[b"info"]).unwrap();
    assert_ne!(a, d);

    // The XOF can produce more than the 32-byte output size. The requested length is
    // part of the labeled input, so this is unrelated to the 32-byte `a`.
    let mut long = [0u8; 64];
    Blake3::labeled_derive(&mut long, &[b"ikm"], suite_id, b"label", &[b"info"]).unwrap();
    assert_ne!(long, [0u8; 64]);

    // Output larger than 2^16 - 1 bytes is rejected.
    let mut too_long = vec![0u8; 1 << 16];
    assert_eq!(
        Blake3::labeled_derive(&mut too_long, &[b"ikm"], suite_id, b"label", &[]),
        Err(HpkeError::KdfOutputTooLong)
    );
}

#[test]
fn blake3_export_output_too_long() {
    let (recipient_secret_key, recipient_public_key) = MLKEM768X25519::generate_keypair().unwrap();
    let (encapped_key, sender) =
        crate::new_sender::<MLKEM768X25519, Blake3, Aes256Gcm>(&SenderMode::Base, &recipient_public_key, b"").unwrap();
    let recipient = crate::new_recipient::<MLKEM768X25519, Blake3, Aes256Gcm>(
        &RecipientMode::Base,
        &recipient_secret_key,
        &encapped_key,
        b"",
    )
    .unwrap();

    // A single-stage KDF can produce at most 2^16 - 1 bytes.
    let mut out = vec![0u8; 1 << 16];
    assert_eq!(sender.export(&mut out, b""), Err(HpkeError::KdfOutputTooLong));
    assert_eq!(recipient.export(&mut out, b""), Err(HpkeError::KdfOutputTooLong));
}
