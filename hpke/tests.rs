//! HPKE tests, including the official RFC 9180 test vector corpus.

use crypto::{aes::Aes256Gcm, chacha::ChaCha20Poly1305};
use serde_json::Value;

use crate::{
    HpkeError, ModeReceiver, ModeSender,
    aead::ExportOnly,
    kdf::{HkdfSha256, HkdfSha512},
    kem::{Kem, P256HkdfSha256, P521HkdfSha512, X25519HkdfSha256, XWing},
};

/// Decodes a hex field, treating a missing field as empty.
fn hex_field(value: &Value, key: &str) -> Vec<u8> {
    match value.get(key).and_then(Value::as_str) {
        Some(s) => hex::decode(s).unwrap(),
        None => Vec::new(),
    }
}

/// Checks a single RFC 9180 test vector against the receiver-side key schedule.
///
/// The sender's key schedule is validated indirectly: re-encapsulating with
/// the vector's ephemeral key must reproduce `enc` and `shared_secret`, and
/// opening the vector's ciphertexts and reproducing its exported values
/// exercises the complete key schedule.
fn check_vector<K: Kem, D: crate::kdf::Kdf, A: crate::aead::Aead>(vector: &Value) {
    let mode = vector["mode"].as_u64().unwrap() as u8;
    let info = hex_field(vector, "info");

    // Recipient key pair.
    let (sk_r, pk_r) = K::derive_keypair(&hex_field(vector, "ikmR")).unwrap();
    let mut pk_r_bytes = vec![0u8; K::NPK];
    K::public_key_to_bytes(&mut pk_r_bytes, &pk_r).unwrap();
    assert_eq!(hex::encode(&pk_r_bytes), vector["pkRm"].as_str().unwrap(), "pkRm mismatch");

    // Ephemeral key pair.
    let (sk_e, pk_e) = K::derive_keypair(&hex_field(vector, "ikmE")).unwrap();
    let mut pk_e_bytes = vec![0u8; K::NPK];
    K::public_key_to_bytes(&mut pk_e_bytes, &pk_e).unwrap();
    assert_eq!(hex::encode(&pk_e_bytes), vector["pkEm"].as_str().unwrap(), "pkEm mismatch");

    // Sender static key pair, for the authenticated modes.
    let sender_static = if mode >= 2 {
        let (sk_s, pk_s) = K::derive_keypair(&hex_field(vector, "ikmS")).unwrap();
        let mut pk_s_bytes = vec![0u8; K::NPK];
        K::public_key_to_bytes(&mut pk_s_bytes, &pk_s).unwrap();
        assert_eq!(hex::encode(&pk_s_bytes), vector["pkSm"].as_str().unwrap(), "pkSm mismatch");
        Some((sk_s, pk_s))
    } else {
        None
    };

    // Encapsulation.
    let (shared_secret, enc) = match &sender_static {
        Some((sk_s, _)) => K::auth_encap_with_ephemeral(&sk_e, &pk_r, sk_s).unwrap(),
        None => K::encap_with_ephemeral(&sk_e, &pk_r).unwrap(),
    };
    assert_eq!(
        shared_secret.as_ref(),
        hex_field(vector, "shared_secret").as_slice(),
        "shared_secret mismatch"
    );
    let mut enc_bytes = vec![0u8; K::NENC];
    K::encapped_key_to_bytes(&mut enc_bytes, &enc).unwrap();
    assert_eq!(hex::encode(&enc_bytes), vector["enc"].as_str().unwrap(), "enc mismatch");

    // Receiver setup.
    let enc_key = K::encapped_key_from_bytes(&hex_field(vector, "enc")).unwrap();
    let psk = hex_field(vector, "psk");
    let psk_id = hex_field(vector, "psk_id");
    let receiver_mode = match mode {
        0 => ModeReceiver::<K>::Base,
        1 => ModeReceiver::Psk {
            psk: &psk,
            psk_id: &psk_id,
        },
        2 => ModeReceiver::Auth {
            pk_s: &sender_static.as_ref().unwrap().1,
        },
        3 => ModeReceiver::AuthPsk {
            psk: &psk,
            psk_id: &psk_id,
            pk_s: &sender_static.as_ref().unwrap().1,
        },
        _ => panic!("unexpected mode {mode}"),
    };
    let mut receiver = crate::new_receiver::<K, D, A>(&receiver_mode, &sk_r, &enc_key, &info).unwrap();

    // Encryptions. The Export-Only pseudo-AEAD has no encryptions.
    if vector["aead_id"].as_u64().unwrap() != 0xffff {
        for (i, encryption) in vector["encryptions"].as_array().unwrap().iter().enumerate() {
            let aad = hex_field(encryption, "aad");
            let ciphertext = hex_field(encryption, "ct");
            let plaintext = hex_field(encryption, "pt");

            let split = ciphertext.len() - A::TAG_SIZE;
            let (body, tag) = ciphertext.split_at(split);
            let mut buffer = body.to_vec();
            receiver
                .open_in_place(&mut buffer, &aad, tag)
                .unwrap_or_else(|err| panic!("encryption {i}: {err}"));
            assert_eq!(buffer, plaintext, "encryption {i} plaintext mismatch");
        }
    }

    // Exported values.
    for export in vector["exports"].as_array().unwrap() {
        let exporter_context = hex_field(export, "exporter_context");
        let len = export["L"].as_u64().unwrap() as usize;
        let mut out = vec![0u8; len];
        receiver.export(&mut out, &exporter_context).unwrap();
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
            _ => panic!("unexpected KDF/AEAD combination {kdf_id:#06x}/{aead_id:#06x}"),
        }
    }};
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
        let (sk_r, pk_r) = K::generate_keypair().unwrap();
        let (enc, mut sender) = crate::new_sender::<K, D, A>(&ModeSender::Base, &pk_r, b"info").unwrap();
        let ciphertext = sender.seal(b"fronthand or backhand?", b"a gentleman's game").unwrap();

        let mut receiver = crate::new_receiver::<K, D, A>(&ModeReceiver::Base, &sk_r, &enc, b"info").unwrap();
        let plaintext = receiver.open(&ciphertext, b"a gentleman's game").unwrap();
        assert_eq!(plaintext, b"fronthand or backhand?");

        // A second message must use the next sequence number.
        let ciphertext = sender.seal(b"second message", b"aad").unwrap();
        assert_eq!(receiver.open(&ciphertext, b"aad").unwrap(), b"second message");
    }

    roundtrip::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>();
    roundtrip::<X25519HkdfSha256, HkdfSha256, ChaCha20Poly1305>();
    roundtrip::<P256HkdfSha256, HkdfSha256, Aes256Gcm>();
    roundtrip::<P521HkdfSha512, HkdfSha512, Aes256Gcm>();
    roundtrip::<XWing, HkdfSha256, Aes256Gcm>();
    roundtrip::<XWing, HkdfSha512, ChaCha20Poly1305>();
}

#[test]
fn roundtrip_authenticated_modes() {
    fn roundtrip<K: Kem, D: crate::kdf::Kdf, A: crate::aead::Aead>() {
        let (sk_r, pk_r) = K::generate_keypair().unwrap();
        let (sk_s, pk_s) = K::generate_keypair().unwrap();

        // Auth mode.
        let (enc, mut sender) = crate::new_sender::<K, D, A>(
            &ModeSender::Auth {
                sk_s: &sk_s,
            },
            &pk_r,
            b"info",
        )
        .unwrap();
        let ciphertext = sender.seal(b"auth message", b"aad").unwrap();
        let mut receiver = crate::new_receiver::<K, D, A>(
            &ModeReceiver::Auth {
                pk_s: &pk_s,
            },
            &sk_r,
            &enc,
            b"info",
        )
        .unwrap();
        assert_eq!(receiver.open(&ciphertext, b"aad").unwrap(), b"auth message");

        // AuthPSK mode.
        let psk = [0x42u8; 32];
        let psk_id = b"psk id";
        let (enc, mut sender) = crate::new_sender::<K, D, A>(
            &ModeSender::AuthPsk {
                psk: &psk,
                psk_id,
                sk_s: &sk_s,
            },
            &pk_r,
            b"info",
        )
        .unwrap();
        let ciphertext = sender.seal(b"auth-psk message", b"aad").unwrap();
        let mut receiver = crate::new_receiver::<K, D, A>(
            &ModeReceiver::AuthPsk {
                psk: &psk,
                psk_id,
                pk_s: &pk_s,
            },
            &sk_r,
            &enc,
            b"info",
        )
        .unwrap();
        assert_eq!(receiver.open(&ciphertext, b"aad").unwrap(), b"auth-psk message");
    }

    roundtrip::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>();
    roundtrip::<P256HkdfSha256, HkdfSha256, ChaCha20Poly1305>();
}

#[test]
fn roundtrip_psk_mode() {
    let (sk_r, pk_r) = X25519HkdfSha256::generate_keypair().unwrap();
    let psk = [0x11u8; 32];
    let psk_id = b"psk id";

    let (enc, mut sender) = crate::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
        &ModeSender::Psk {
            psk: &psk,
            psk_id,
        },
        &pk_r,
        b"info",
    )
    .unwrap();
    let ciphertext = sender.seal(b"psk message", b"").unwrap();

    let mut receiver = crate::new_receiver::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
        &ModeReceiver::Psk {
            psk: &psk,
            psk_id,
        },
        &sk_r,
        &enc,
        b"info",
    )
    .unwrap();
    assert_eq!(receiver.open(&ciphertext, b"").unwrap(), b"psk message");

    // A different PSK must not authenticate.
    let wrong_psk = [0x22u8; 32];
    let mut receiver = crate::new_receiver::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
        &ModeReceiver::Psk {
            psk: &wrong_psk,
            psk_id,
        },
        &sk_r,
        &enc,
        b"info",
    )
    .unwrap();
    assert_eq!(receiver.open(&ciphertext, b""), Err(HpkeError::OpenError));
}

#[test]
fn xwing_is_not_an_authenticated_kem() {
    let (sk_r, pk_r) = XWing::generate_keypair().unwrap();
    let (sk_s, _pk_s) = XWing::generate_keypair().unwrap();

    let err = crate::new_sender::<XWing, HkdfSha256, Aes256Gcm>(
        &ModeSender::Auth {
            sk_s: &sk_s,
        },
        &pk_r,
        b"",
    )
    .err()
    .unwrap();
    assert_eq!(err, HpkeError::NotSupported);

    let (enc, _sender) = crate::new_sender::<XWing, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk_r, b"").unwrap();
    let err = crate::new_receiver::<XWing, HkdfSha256, Aes256Gcm>(
        &ModeReceiver::Auth {
            pk_s: &sk_s.public_key(),
        },
        &sk_r,
        &enc,
        b"",
    )
    .err()
    .unwrap();
    assert_eq!(err, HpkeError::NotSupported);
}

#[test]
fn export_only_suite() {
    let (sk_r, pk_r) = X25519HkdfSha256::generate_keypair().unwrap();
    let (enc, sender) =
        crate::new_sender::<X25519HkdfSha256, HkdfSha256, ExportOnly>(&ModeSender::Base, &pk_r, b"info").unwrap();

    let mut sent = [0u8; 64];
    sender.export(&mut sent, b"exporter context").unwrap();

    let receiver =
        crate::new_receiver::<X25519HkdfSha256, HkdfSha256, ExportOnly>(&ModeReceiver::Base, &sk_r, &enc, b"info")
            .unwrap();
    let mut received = [0u8; 64];
    receiver.export(&mut received, b"exporter context").unwrap();
    assert_eq!(sent, received);

    // Different exporter contexts produce different outputs.
    let mut other = [0u8; 64];
    receiver.export(&mut other, b"other context").unwrap();
    assert_ne!(sent, other);

    // Sealing and opening are not supported by the Export-Only pseudo-AEAD and
    // return an error rather than panicking.
    let mut sender = sender;
    assert_eq!(sender.seal(b"message", b""), Err(HpkeError::NotSupported));
    let mut receiver = receiver;
    assert_eq!(receiver.open(&[0u8; 32], b""), Err(HpkeError::NotSupported));
}

#[test]
fn export_output_too_long() {
    let (sk_r, pk_r) = X25519HkdfSha256::generate_keypair().unwrap();
    let (enc, sender) =
        crate::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk_r, b"").unwrap();
    let receiver =
        crate::new_receiver::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&ModeReceiver::Base, &sk_r, &enc, b"").unwrap();

    // HKDF-SHA256 can produce at most 255 * 32 bytes.
    let mut out = vec![0u8; 255 * 32 + 1];
    assert_eq!(sender.export(&mut out, b""), Err(HpkeError::KdfOutputTooLong));
    assert_eq!(receiver.export(&mut out, b""), Err(HpkeError::KdfOutputTooLong));
}

#[test]
fn tampered_ciphertext_is_rejected() {
    let (sk_r, pk_r) = X25519HkdfSha256::generate_keypair().unwrap();
    let (enc, mut sender) =
        crate::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&ModeSender::Base, &pk_r, b"").unwrap();
    let mut ciphertext = sender.seal(b"secret", b"").unwrap();

    let last = ciphertext.len() - 1;
    ciphertext[last] ^= 0x01;

    let mut receiver =
        crate::new_receiver::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(&ModeReceiver::Base, &sk_r, &enc, b"").unwrap();
    assert_eq!(receiver.open(&ciphertext, b""), Err(HpkeError::OpenError));
}

#[test]
fn key_serialization_roundtrips() {
    fn roundtrip<K: Kem>() {
        let (sk, pk) = K::generate_keypair().unwrap();

        let mut pk_bytes = vec![0u8; K::NPK];
        K::public_key_to_bytes(&mut pk_bytes, &pk).unwrap();
        let pk2 = K::public_key_from_bytes(&pk_bytes).unwrap();

        let mut sk_bytes = vec![0u8; K::NSK];
        K::secret_key_to_bytes(&mut sk_bytes, &sk).unwrap();
        let sk2 = K::secret_key_from_bytes(&sk_bytes).unwrap();

        // The public key derived from both secret keys must match, and both
        // must agree with the serialized public key.
        let mut pk2_bytes = vec![0u8; K::NPK];
        K::public_key_to_bytes(&mut pk2_bytes, &K::sk_to_pk(&sk2)).unwrap();
        assert_eq!(pk_bytes, pk2_bytes);
        assert_eq!(
            {
                let mut b = vec![0u8; K::NPK];
                K::public_key_to_bytes(&mut b, &pk2).unwrap();
                b
            },
            pk_bytes
        );
    }

    roundtrip::<X25519HkdfSha256>();
    roundtrip::<P256HkdfSha256>();
    roundtrip::<P521HkdfSha512>();
    roundtrip::<XWing>();
}

#[test]
fn kdf_labeled_derivations() {
    use crate::kdf::{HkdfSha512, Kdf};

    let suite_id = b"HPKE\x00\x10\x00\x03\x00\x02";

    // `labeled_extract` is deterministic and returns exactly `NH` bytes.
    let prk = HkdfSha512::labeled_extract(b"", suite_id, b"label", b"ikm");
    let prk2 = HkdfSha512::labeled_extract(b"", suite_id, b"label", b"ikm");
    assert_eq!(prk.as_ref(), prk2.as_ref());
    assert_eq!(prk.len(), HkdfSha512::NH);

    // `labeled_expand` writes exactly `out.len()` bytes and is deterministic.
    let mut a = [0u8; 40];
    let mut b = [0u8; 40];
    HkdfSha512::labeled_expand(&mut a, &prk, suite_id, b"label", b"info").unwrap();
    HkdfSha512::labeled_expand(&mut b, &prk, suite_id, b"label", b"info").unwrap();
    assert_eq!(a, b);
    assert_ne!(a, [0u8; 40]);

    // Output larger than 255 * Nh is rejected.
    let mut too_long = vec![0u8; 255 * 64 + 1];
    assert_eq!(
        HkdfSha512::labeled_expand(&mut too_long, &prk, suite_id, b"label", b"info"),
        Err(HpkeError::KdfOutputTooLong)
    );

    // A PRK that is not exactly Nh bytes is rejected.
    let mut out = [0u8; 32];
    assert_eq!(
        HkdfSha512::labeled_expand(&mut out, b"short prk", suite_id, b"label", b"info"),
        Err(HpkeError::InvalidPrk)
    );
    let long_prk = [0u8; 65];
    assert_eq!(
        HkdfSha512::labeled_expand(&mut out, &long_prk, suite_id, b"label", b"info"),
        Err(HpkeError::InvalidPrk)
    );
}

#[test]
fn p256_rejects_compressed_public_key() {
    let (_, pk) = P256HkdfSha256::generate_keypair().unwrap();
    let compressed = pk.to_compressed_bytes();
    assert_eq!(compressed.len(), 33);
    assert_eq!(
        P256HkdfSha256::public_key_from_bytes(&compressed),
        Err(HpkeError::DeserializeError)
    );
}
