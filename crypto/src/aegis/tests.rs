//! Test vectors and cross-backend checks for the AEGIS module.
//!
//! The RFC 10032 Appendix A vectors are the source of truth. The vendored
//! Wycheproof AEGIS-256 vectors are also exercised; their current-version
//! vectors agree with the RFC, while the `OldVersion` entries are expected to
//! fail (and do).

use alloc::{vec, vec::Vec};

use super::*;
use crate::{
    StreamCipher,
    aegis::{aegis_core::Core, aegis_soft::Soft},
};

fn hex(s: &str) -> Vec<u8> {
    assert!(s.len().is_multiple_of(2));
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(s.len() / 2);
    let mut i = 0;
    while i < bytes.len() {
        let hi = (bytes[i] as char).to_digit(16).unwrap() as u8;
        let lo = (bytes[i + 1] as char).to_digit(16).unwrap() as u8;
        out.push((hi << 4) | lo);
        i += 2;
    }
    out
}

fn key_256() -> [u8; 32] {
    hex("1001000000000000000000000000000000000000000000000000000000000000")
        .try_into()
        .unwrap()
}

fn nonce_256() -> [u8; 32] {
    hex("1000020000000000000000000000000000000000000000000000000000000000")
        .try_into()
        .unwrap()
}

fn key_std() -> [u8; 32] {
    hex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
        .try_into()
        .unwrap()
}

fn nonce_std() -> [u8; 32] {
    hex("101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f")
        .try_into()
        .unwrap()
}

#[test]
fn rfc10032_aead_vectors() {
    // AEGIS-256, Appendix A.3.2 to A.3.6.
    let cipher16 = Aegis256::<16>::new(&key_256());
    let cipher32 = Aegis256::<32>::new(&key_256());
    let nonce = nonce_256();

    for (ad, msg, ct, tag128, tag256) in [
        (
            "",
            "00000000000000000000000000000000",
            "754fc3d8c973246dcc6d741412a4b236",
            "3fe91994768b332ed7f570a19ec5896e",
            "1181a1d18091082bf0266f66297d167d2e68b845f61a3b0527d31fc7b7b89f13",
        ),
        (
            "",
            "",
            "",
            "e3def978a0f054afd1e761d7553afba3",
            "6a348c930adbd654896e1666aad67de989ea75ebaa2b82fb588977b1ffec864a",
        ),
        (
            "0001020304050607",
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
            "f373079ed84b2709faee373584585d60accd191db310ef5d8b11833df9dec711",
            "8d86f91ee606e9ff26a01b64ccbdd91d",
            "b7d28d0c3c0ebd409fd22b44160503073a547412da0854bfb9723020dab8da1a",
        ),
        (
            "0001020304050607",
            "000102030405060708090a0b0c0d",
            "f373079ed84b2709faee37358458",
            "c60b9c2d33ceb058f96e6dd03c215652",
            "8c1cc703c81281bee3f6d9966e14948b4a175b2efbdc31e61a98b4465235c2d9",
        ),
        (
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20212223242526272829",
            "101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637",
            "57754a7d09963e7c787583a2e7b859bb24fa1e04d49fd550b2511a358e3bca252a9b1b8b30cc4a67",
            "ab8a7d53fd0e98d727accca94925e128",
            "a3aca270c006094d71c20e6910b5161c0826df233d08919a566ec2c05990f734",
        ),
    ] {
        let mut buffer = hex(msg);
        let tag = cipher16.encrypt_in_place_detached(&mut buffer, &nonce, &hex(ad));
        assert_eq!(buffer, hex(ct));
        assert_eq!(tag.to_vec(), hex(tag128));

        let mut buffer = hex(msg);
        let tag = cipher32.encrypt_in_place_detached(&mut buffer, &nonce, &hex(ad));
        assert_eq!(buffer, hex(ct));
        assert_eq!(tag.to_vec(), hex(tag256));

        let mut decrypted = hex(ct);
        let tag128: [u8; 16] = hex(tag128).try_into().unwrap();
        cipher16
            .decrypt_in_place_detached(&mut decrypted, &nonce, &hex(ad), &tag128)
            .unwrap();
        assert_eq!(decrypted, hex(msg));
    }

    // AEGIS-256X2/X4, Appendix A.6.2 and A.7.2 (Test Vector 1, empty message).
    let nonce = nonce_std();
    let c = Aegis256X2::<16>::new(&key_std());
    let mut empty = Vec::new();
    assert_eq!(
        c.encrypt_in_place_detached(&mut empty, &nonce, &[]).to_vec(),
        hex("62cdbab084c83dacdb945bb446f049c8")
    );
    let c = Aegis256X2::<32>::new(&key_std());
    let mut empty = Vec::new();
    assert_eq!(
        c.encrypt_in_place_detached(&mut empty, &nonce, &[]).to_vec(),
        hex("25d7e799b49a80354c3f881ac2f1027f471a5d293052bd9997abd3ae84014bb7")
    );
    let c = Aegis256X4::<16>::new(&key_std());
    let mut empty = Vec::new();
    assert_eq!(
        c.encrypt_in_place_detached(&mut empty, &nonce, &[]).to_vec(),
        hex("3b7fee6cee7bf17888ad11ed2397beb4")
    );
    let c = Aegis256X4::<32>::new(&key_std());
    let mut empty = Vec::new();
    assert_eq!(
        c.encrypt_in_place_detached(&mut empty, &nonce, &[]).to_vec(),
        hex("6093a1a8aab20ec635dc1ca71745b01b5bec4fc444c9ffbebd710d4a34d20eaf")
    );

    // AEGIS-256X2, Appendix A.6.3 (Test Vector 2).
    let cipher = Aegis256X2::<16>::new(&key_std());
    let nonce = nonce_std();
    let ad = hex("0102030401020304");
    let msg = hex(
        "040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607",
    );
    let expected = hex(
        "72120c2ea8236180d67859001f4729077b7064c414384fe3a7b52f1571f4f8a7d0f01e18db4f3bc0adb150702e5d147a8d36522132761b994c1bd395589e2ccf0790dfe2a3d12d61cd666b2859827739db4037dd3124c78424459376f6cac08e1a7223a2a43e398ce6385cd654a19f481cba3b8f25910b42",
    );
    let mut buffer = msg.clone();
    let tag = cipher.encrypt_in_place_detached(&mut buffer, &nonce, &ad);
    assert_eq!(buffer.len(), expected.len(), "x2 ct len");
    assert_eq!(msg.len(), 120, "x2 msg len");
    for i in 0..buffer.len().min(expected.len()) {
        assert_eq!(buffer[i], expected[i], "x2 first diff at {i}");
    }
    assert_eq!(buffer, expected);
    assert_eq!(tag.to_vec(), hex("635d391828520bf1512763f0c8f5cdbd"));

    let cipher = Aegis256X2::<32>::new(&key_std());
    let mut buffer = msg.clone();
    let tag = cipher.encrypt_in_place_detached(&mut buffer, &nonce, &ad);
    assert_eq!(buffer, expected);
    assert_eq!(
        tag.to_vec(),
        hex("b5668d3317159e9cc5d46e4803c3a76ad63bb42b3f47956d94f30db8cb366ad7")
    );

    // AEGIS-256X4, Appendix A.7.3 (Test Vector 2).
    let cipher = Aegis256X4::<16>::new(&key_std());
    let expected = hex(
        "bfc2085b7e8017da99b0b6d646ae4d01f4ba8f2e7dfca1d759ae48a135139b9aaac6b4f5db810d426be1fdaff4e1454153a34b11da78ed7e418ee2ee9853042e95536aecbb694cea1b16a478eb0d4d1bf6509b1ce652a45af58e0e46ffccfa2d0426e702391d2ff5813808b81748a490dd656465fed61f09",
    );
    let mut buffer = msg.clone();
    let tag = cipher.encrypt_in_place_detached(&mut buffer, &nonce, &ad);
    assert_eq!(buffer, expected);
    assert_eq!(tag.to_vec(), hex("b63b611b13975e2f3dc3cb6c2397bfcd"));

    let cipher = Aegis256X4::<32>::new(&key_std());
    let mut buffer = msg.clone();
    let tag = cipher.encrypt_in_place_detached(&mut buffer, &nonce, &ad);
    assert_eq!(buffer, expected);
    assert_eq!(
        tag.to_vec(),
        hex("7847eace74409ee56c8f4cf63a9c2841ce7c8bd567d7c0ca514c879a190b978c")
    );
}

fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

macro_rules! differential_aead {
    ($cipher16:expr, $cipher32:expr, $d:expr) => {{
        for len in [
            0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 1023, 1024, 1025, 4096, 16384,
        ] {
            for aad_len in [0usize, 1, 7, 16, 17, 64, 65, 1024, 4096] {
                let key = [0x42u8; 32];
                let nonce = [0x24u8; 32];
                let ad = pattern(aad_len, 29, 3);
                let mut plaintext = pattern(len, 17, 5);

                let mut expected = plaintext.clone();
                let mut expected_tag = [0u8; 16];
                aegis_core::encrypt::<Soft<$d>>(&key, &nonce, &ad, &mut expected, &mut expected_tag);
                assert_eq!(expected_tag.len(), 16);

                let cipher16 = $cipher16;
                let mut got = plaintext.clone();
                let tag = cipher16.encrypt_in_place_detached(&mut got, &nonce, &ad);
                assert_eq!(got, expected, "len={len} aad={aad_len}");
                assert_eq!(tag, expected_tag, "len={len} aad={aad_len}");

                let mut got = expected.clone();
                cipher16.decrypt_in_place_detached(&mut got, &nonce, &ad, &tag).unwrap();
                assert_eq!(got, plaintext);

                // 256-bit tag path.
                let mut expected = plaintext.clone();
                let mut expected_tag = [0u8; 32];
                aegis_core::encrypt::<Soft<$d>>(&key, &nonce, &ad, &mut expected, &mut expected_tag);
                let cipher32 = $cipher32;
                let mut got = plaintext.clone();
                let tag = cipher32.encrypt_in_place_detached(&mut got, &nonce, &ad);
                assert_eq!(got, expected, "len={len} aad={aad_len} tag256");
                assert_eq!(tag, expected_tag, "len={len} aad={aad_len} tag256");
                plaintext.clear();
            }
        }
    }};
}

#[test]
fn hardware_matches_soft_at_boundaries() {
    differential_aead!(Aegis256::<16>::new(&[0x42; 32]), Aegis256::<32>::new(&[0x42; 32]), 1);
    differential_aead!(Aegis256X2::<16>::new(&[0x42; 32]), Aegis256X2::<32>::new(&[0x42; 32]), 2);
    differential_aead!(Aegis256X4::<16>::new(&[0x42; 32]), Aegis256X4::<32>::new(&[0x42; 32]), 4);
}

#[test]
fn aead_trait_and_zeroing() {
    let key = [0x11u8; 32];
    let nonce = [0x22u8; 32];
    let cipher = Aegis256::<16>::new(&key);
    let plaintext = b"aegis authenticated encryption";
    let mut buffer = plaintext.to_vec();
    let tag = cipher.encrypt_in_place(&mut buffer, &nonce, b"aad");
    assert_eq!(tag.len(), 16);

    let mut decrypted = buffer.clone();
    cipher
        .decrypt_in_place(&mut decrypted, &nonce, b"aad", tag.as_ref())
        .unwrap();
    assert_eq!(decrypted, plaintext);

    // Tampering must fail and must leave zeros in the buffer.
    let mut tampered = buffer.clone();
    let mut bad_tag = *tag.as_ref().first_chunk::<16>().unwrap();
    bad_tag[0] ^= 1;
    assert_eq!(
        cipher.decrypt_in_place(&mut tampered, &nonce, b"aad", &bad_tag),
        Err(AeadError::InvalidCiphertext)
    );
    assert!(tampered.iter().all(|&b| b == 0));

    for (nonce, aad, ciphertext) in [
        (
            {
                let mut n = nonce;
                n[0] ^= 1;
                n
            },
            b"aad".as_slice(),
            buffer.clone(),
        ),
        (nonce, b"other aad".as_slice(), buffer.clone()),
        (nonce, b"aad".as_slice(), {
            let mut c = buffer.clone();
            c[0] ^= 1;
            c
        }),
    ] {
        let mut tampered = ciphertext;
        assert_eq!(
            cipher.decrypt_in_place(&mut tampered, &nonce, aad, tag.as_ref()),
            Err(AeadError::InvalidCiphertext)
        );
        assert!(tampered.iter().all(|&b| b == 0));
    }

    // Wrong nonce/tag lengths.
    let mut buffer = plaintext.to_vec();
    assert_eq!(
        cipher.decrypt_in_place(&mut buffer, &nonce[..31], b"aad", tag.as_ref()),
        Err(AeadError::InvalidNonce)
    );
    assert_eq!(
        cipher.decrypt_in_place(&mut buffer, &nonce, b"aad", &tag.as_ref()[..15]),
        Err(AeadError::InvalidCiphertext)
    );

    // The allocating convenience API must append/remove the tag.
    let encrypted = cipher.encrypt(plaintext, &nonce, b"aad");
    assert_eq!(encrypted.len(), plaintext.len() + 16);
    let decrypted = cipher.decrypt(&encrypted, &nonce, b"aad").unwrap();
    assert_eq!(decrypted, plaintext);

    // The 256-bit tag path through the Aead trait.
    let cipher = Aegis256X4::<32>::new(&key);
    let mut buffer = plaintext.to_vec();
    let tag = cipher.encrypt_in_place(&mut buffer, &nonce, b"aad");
    assert_eq!(tag.len(), 32);
    cipher
        .decrypt_in_place(&mut buffer, &nonce, b"aad", tag.as_ref())
        .unwrap();
    assert_eq!(buffer, plaintext);
}

const MAC_DATA: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122";

#[test]
fn rfc10032_mac_vectors() {
    let key = key_256();
    let nonce = nonce_256();
    let data = hex(MAC_DATA);

    let mut mac = Aegis256Mac::<16>::new_with_nonce(&key, &nonce);
    mac.update(&data);
    assert_eq!(mac.finalize().to_vec(), hex("c08e20cfc56f27195a46c9cef5c162d4"));

    let mut mac = Aegis256Mac::<32>::new_with_nonce(&key, &nonce);
    mac.update(&data);
    assert_eq!(
        mac.finalize().to_vec(),
        hex("a5c906ede3d69545c11e20afa360b221f936e946ed2dba3d7c75ad6dc2784126")
    );

    let mut mac = Aegis256X2Mac::<16>::new_with_nonce(&key, &nonce);
    mac.update(&data);
    assert_eq!(mac.finalize().to_vec(), hex("fb319cb6dd728a764606fb14d37f2a5e"));

    let mut mac = Aegis256X2Mac::<32>::new_with_nonce(&key, &nonce);
    mac.update(&data);
    assert_eq!(
        mac.finalize().to_vec(),
        hex("0844b20ed5147ceae89c7a160263afd4b1382d6b154ecf560ce8a342cb6a8fd1")
    );

    let mut mac = Aegis256X4Mac::<16>::new_with_nonce(&key, &nonce);
    mac.update(&data);
    assert_eq!(mac.finalize().to_vec(), hex("a51f9bc5beae60cce77f0dbc60761edd"));

    let mut mac = Aegis256X4Mac::<32>::new_with_nonce(&key, &nonce);
    mac.update(&data);
    assert_eq!(
        mac.finalize().to_vec(),
        hex("b36a16ef07c36d75a91f437502f24f545b8dfa88648ed116943c29fead3bf10c")
    );
}

#[test]
fn mac_empty_and_incremental() {
    let key = key_256();
    let nonce = nonce_256();

    // Generated with libaegis. Empty data does not absorb a zero block.
    let mac = Aegis256Mac::<16>::new_with_nonce(&key, &nonce);
    assert_eq!(mac.finalize().to_vec(), hex("ab7ffd51500ede3f408618398fa233bd"));
    let mac = Aegis256Mac::<32>::new_with_nonce(&key, &nonce);
    assert_eq!(
        mac.finalize().to_vec(),
        hex("2485726fd6d2561016be8f953d5aa632a29602b0a4c74a8f98aea48b66110a76")
    );

    // Generated with libaegis; the X variants must skip state 0 in the tag
    // combination and fold the final tag from lane 0 only.
    let mac = Aegis256X2Mac::<16>::new_with_nonce(&key, &nonce);
    assert_eq!(mac.finalize().to_vec(), hex("5010d371a2dca677314fc65acbaf3f41"));
    let mac = Aegis256X2Mac::<32>::new_with_nonce(&key, &nonce);
    assert_eq!(
        mac.finalize().to_vec(),
        hex("f746f70b3ff9016dc634026878ac93345fe93e6a746c86f6e419e4765514ffec")
    );
    let mac = Aegis256X4Mac::<16>::new_with_nonce(&key, &nonce);
    assert_eq!(mac.finalize().to_vec(), hex("c0361f7119ce52e8f70509b953f07fc3"));
    let mac = Aegis256X4Mac::<32>::new_with_nonce(&key, &nonce);
    assert_eq!(
        mac.finalize().to_vec(),
        hex("6e4d57d8f94048b4f2c51c786bdf850c4aa7ebf5cc2a241f2a48682c379a54f1")
    );

    let data = hex(MAC_DATA);
    for chunk in [1usize, 2, 7, 16, 17, 31, 32, 33, 64, 65] {
        let mut mac = Aegis256Mac::<16>::new_with_nonce(&key, &nonce);
        for piece in data.chunks(chunk) {
            mac.update(piece);
        }
        assert_eq!(
            mac.finalize().to_vec(),
            hex("c08e20cfc56f27195a46c9cef5c162d4"),
            "chunk={chunk}"
        );
    }

    // Clone lets one initialized state authenticate several messages.
    let mut base = Aegis256Mac::<16>::new_with_nonce(&key, &nonce);
    base.update(&data);
    let expected = base.clone().finalize();
    let mut other = base.clone();
    other.update(b"x");
    assert_ne!(other.finalize(), expected);
    assert_eq!(base.finalize(), expected);

    // Hardware and soft MACs agree, including the X2/X4 finalization path.
    for len in [
        0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 1024, 4096, 16384,
    ] {
        let data = pattern(len, 13, 7);
        let mut soft = Core::<Soft<1>>::new(&key, &nonce);
        soft.absorb(&data);
        let mut expected = [0u8; 32];
        soft.finalize_mac(len as u64, &mut expected);
        let mut mac = Aegis256Mac::<32>::new_with_nonce(&key, &nonce);
        mac.update(&data);
        assert_eq!(mac.finalize(), expected, "256 len={len}");

        let mut soft = Core::<Soft<2>>::new(&key, &nonce);
        soft.absorb(&data);
        let mut expected = [0u8; 32];
        soft.finalize_mac(len as u64, &mut expected);
        let mut mac = Aegis256X2Mac::<32>::new_with_nonce(&key, &nonce);
        mac.update(&data);
        assert_eq!(mac.finalize(), expected, "x2 len={len}");

        let mut soft = Core::<Soft<4>>::new(&key, &nonce);
        soft.absorb(&data);
        let mut expected = [0u8; 32];
        soft.finalize_mac(len as u64, &mut expected);
        let mut mac = Aegis256X4Mac::<32>::new_with_nonce(&key, &nonce);
        mac.update(&data);
        assert_eq!(mac.finalize(), expected, "x4 len={len}");
    }
}

macro_rules! stream_tests {
    ($stream:ty, $aead:ty) => {{
        let key = [0x42u8; 32];
        let nonce = [0x24u8; 32];
        for len in [
            0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 1024, 4096, 16384,
        ] {
            let cipher = <$aead>::new(&key);
            let mut expected = vec![0u8; len];
            let _tag = cipher.encrypt_in_place(&mut expected, &nonce, b"");

            let mut stream = <$stream>::new(&key, &nonce);
            let mut got = vec![0u8; len];
            stream.fill_keystream(&mut got);
            assert_eq!(got, expected, "fill len={len}");

            let mut stream = <$stream>::new(&key, &nonce);
            let mut got = expected.clone();
            stream.xor_keystream_in_place(&mut got);
            assert!(got.iter().all(|&b| b == 0), "xor len={len}");

            // Chunked calls must produce the same continuous keystream.
            let mut stream = <$stream>::new(&key, &nonce);
            let mut got = vec![0u8; len];
            let mut offset = 0;
            for chunk in [1usize, 3, 7, 16, 31] {
                if offset == got.len() {
                    break;
                }
                let end = (offset + chunk).min(got.len());
                stream.fill_keystream(&mut got[offset..end]);
                offset = end;
            }
            if offset < got.len() {
                stream.fill_keystream(&mut got[offset..]);
            }
            assert_eq!(got, expected, "chunked len={len}");

            let mut stream = <$stream>::new(&key, &nonce);
            let mut got = expected.clone();
            let mut offset = 0;
            for chunk in [1usize, 3, 7, 16, 31] {
                if offset == got.len() {
                    break;
                }
                let end = (offset + chunk).min(got.len());
                stream.xor_keystream_in_place(&mut got[offset..end]);
                offset = end;
            }
            if offset < got.len() {
                stream.xor_keystream_in_place(&mut got[offset..]);
            }
            assert!(got.iter().all(|&b| b == 0), "chunked xor len={len}");
        }
    }};
}

#[test]
fn stream_matches_aead_zeros() {
    stream_tests!(Aegis256Stream, Aegis256<16>);
    stream_tests!(Aegis256X2Stream, Aegis256X2<16>);
    stream_tests!(Aegis256X4Stream, Aegis256X4<16>);
}

#[test]
fn stream_trait_roundtrip() {
    let key = [0x5au8; 32];
    let nonce = [0xa5u8; 32];
    let mut stream = Aegis256Stream::new(&key, &nonce);
    let mut buffer = b"the quick brown fox jumps over the lazy dog".to_vec();
    let original = buffer.clone();
    StreamCipher::xor_keystream(&mut stream, &mut buffer);
    assert_ne!(buffer, original);
    let mut stream = Aegis256Stream::new(&key, &nonce);
    StreamCipher::xor_keystream(&mut stream, &mut buffer);
    assert_eq!(buffer, original);
}

#[cfg(all(target_arch = "aarch64", feature = "std"))]
#[test]
fn sha3_backend_selection() {
    if std::arch::is_aarch64_feature_detected!("sha3") {
        assert_eq!(detect::<1>(), Backend::ArmSha3);
        assert_eq!(detect::<2>(), Backend::ArmSha3);
        // The four-lane mode keeps the plain NEON path.
        assert_eq!(detect::<4>(), Backend::Arm);
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[test]
fn wasm_lane_backend_selection() {
    assert_eq!(detect::<1>(), Backend::Wasm);
    assert_eq!(detect::<2>(), Backend::WasmLane);
    assert_eq!(detect::<4>(), Backend::WasmLane);
}

#[test]
fn fill_keystream_overwrites_buffer() {
    macro_rules! check {
        ($stream:ty) => {{
            let key = [0x42u8; 32];
            let nonce = [0x24u8; 32];
            for len in [0usize, 1, 15, 16, 17, 31, 32, 33, 64, 65, 100, 256, 1024, 4096] {
                let mut zeros = vec![0u8; len];
                let mut ones = vec![0xffu8; len];
                <$stream>::new(&key, &nonce).fill_keystream(&mut zeros);
                <$stream>::new(&key, &nonce).fill_keystream(&mut ones);
                assert_eq!(zeros, ones, "fill must overwrite, len={len}");

                let mut xored = vec![0u8; len];
                <$stream>::new(&key, &nonce).xor_keystream_in_place(&mut xored);
                assert_eq!(zeros, xored, "fill must equal xor-on-zero, len={len}");

                let mut chunked = vec![0u8; len];
                let mut stream = <$stream>::new(&key, &nonce);
                for chunk in chunked.chunks_mut(7) {
                    stream.fill_keystream(chunk);
                }
                assert_eq!(chunked, zeros, "chunked fill, len={len}");

                // Mix fill and xor across an arbitrary split.
                let split = (len / 2).min(len);
                let mut mixed = vec![0u8; len];
                let mut stream = <$stream>::new(&key, &nonce);
                let (a, b) = mixed.split_at_mut(split);
                stream.fill_keystream(a);
                stream.xor_keystream_in_place(b);
                assert_eq!(mixed, zeros, "mixed fill/xor, len={len}");
            }
        }};
    }
    check!(Aegis256Stream);
    check!(Aegis256X2Stream);
    check!(Aegis256X4Stream);
}

#[test]
fn tag_256_tamper_zeroes_buffer() {
    macro_rules! check {
        ($cipher:ty) => {{
            let key = [0x11u8; 32];
            let nonce = [0x22u8; 32];
            let cipher = <$cipher>::new(&key);
            let mut buffer = b"aegis 256-bit tag tamper".to_vec();
            let mut tag = cipher.encrypt_in_place_detached(&mut buffer, &nonce, b"aad");
            tag[0] ^= 1;
            let mut tampered = buffer.clone();
            assert_eq!(
                cipher.decrypt_in_place_detached(&mut tampered, &nonce, b"aad", &tag),
                Err(AeadError::InvalidCiphertext)
            );
            assert!(tampered.iter().all(|&b| b == 0));
        }};
    }
    check!(Aegis256<32>);
    check!(Aegis256X2<32>);
    check!(Aegis256X4<32>);
}

#[test]
#[should_panic(expected = "AEGIS nonce must be 32 bytes")]
fn encrypt_in_place_rejects_wrong_nonce_length() {
    let cipher = Aegis256::<16>::new(&[0x11u8; 32]);
    let mut buffer = [0u8; 16];
    let _ = cipher.encrypt_in_place(&mut buffer, &[0u8; 31], &[]);
}

#[test]
fn rfc10032_invalid_vectors() {
    // Appendix A.3.7 to A.3.10: wrong key, modified ciphertext, modified AD,
    // modified tag. Each must fail and leave the plaintext zeroed.
    let key = key_256();
    let nonce = nonce_256();
    let swapped_key = nonce;
    let swapped_nonce = key;
    let ad = hex("0001020304050607");

    let cases = [
        // (key, nonce, ad, ct, tag128)
        (
            swapped_key,
            swapped_nonce,
            ad.clone(),
            hex("f373079ed84b2709faee37358458"),
            hex("c60b9c2d33ceb058f96e6dd03c215652"),
        ),
        (
            key,
            nonce,
            ad.clone(),
            hex("f373079ed84b2709faee37358459"),
            hex("c60b9c2d33ceb058f96e6dd03c215652"),
        ),
        (
            key,
            nonce,
            hex("0001020304050608"),
            hex("f373079ed84b2709faee37358458"),
            hex("c60b9c2d33ceb058f96e6dd03c215652"),
        ),
        (
            key,
            nonce,
            ad.clone(),
            hex("f373079ed84b2709faee37358458"),
            hex("c60b9c2d33ceb058f96e6dd03c215653"),
        ),
    ];

    for (case_key, case_nonce, case_ad, ct, tag) in &cases {
        let cipher = Aegis256::<16>::new(case_key);
        let mut buffer = ct.clone();
        let tag: [u8; 16] = tag.as_slice().try_into().unwrap();
        assert_eq!(
            cipher.decrypt_in_place_detached(&mut buffer, case_nonce, case_ad, &tag),
            Err(AeadError::InvalidCiphertext)
        );
        assert!(buffer.iter().all(|&b| b == 0));
    }

    // The same cases with a 256-bit tag.
    let cases32 = [
        (
            swapped_key,
            swapped_nonce,
            ad.clone(),
            hex("f373079ed84b2709faee37358458"),
            hex("8c1cc703c81281bee3f6d9966e14948b4a175b2efbdc31e61a98b4465235c2d9"),
        ),
        (
            key,
            nonce,
            ad.clone(),
            hex("f373079ed84b2709faee37358459"),
            hex("8c1cc703c81281bee3f6d9966e14948b4a175b2efbdc31e61a98b4465235c2d9"),
        ),
        (
            key,
            nonce,
            hex("0001020304050608"),
            hex("f373079ed84b2709faee37358458"),
            hex("8c1cc703c81281bee3f6d9966e14948b4a175b2efbdc31e61a98b4465235c2d9"),
        ),
        (
            key,
            nonce,
            ad.clone(),
            hex("f373079ed84b2709faee37358458"),
            hex("8c1cc703c81281bee3f6d9966e14948b4a175b2efbdc31e61a98b4465235c2da"),
        ),
    ];
    for (case_key, case_nonce, case_ad, ct, tag) in &cases32 {
        let cipher = Aegis256::<32>::new(case_key);
        let mut buffer = ct.clone();
        let tag: [u8; 32] = tag.as_slice().try_into().unwrap();
        assert_eq!(
            cipher.decrypt_in_place_detached(&mut buffer, case_nonce, case_ad, &tag),
            Err(AeadError::InvalidCiphertext)
        );
        assert!(buffer.iter().all(|&b| b == 0));
    }
}

#[test]
fn wycheproof_aegis256_vectors() {
    let text = include_str!("../../testdata/wycheproof/testvectors_v1/aegis256_test.json");
    let document: serde_json::Value = serde_json::from_str(text).unwrap();
    let mut valid = 0usize;
    let mut invalid = 0usize;

    for group in document["testGroups"].as_array().unwrap() {
        for test in group["tests"].as_array().unwrap() {
            let key: [u8; 32] = hex(test["key"].as_str().unwrap()).try_into().unwrap();
            let nonce: [u8; 32] = hex(test["iv"].as_str().unwrap()).try_into().unwrap();
            let ad = hex(test["aad"].as_str().unwrap());
            let msg = hex(test["msg"].as_str().unwrap());
            let ct = hex(test["ct"].as_str().unwrap());
            let tag: [u8; 16] = hex(test["tag"].as_str().unwrap()).try_into().unwrap();
            let cipher = Aegis256::<16>::new(&key);
            let tc_id = test["tcId"].as_u64().unwrap();

            match test["result"].as_str().unwrap() {
                "valid" => {
                    let mut buffer = msg.clone();
                    let got = cipher.encrypt_in_place_detached(&mut buffer, &nonce, &ad);
                    assert_eq!(buffer, ct, "wycheproof tcId={tc_id} ciphertext");
                    assert_eq!(got, tag, "wycheproof tcId={tc_id} tag");

                    let mut decrypted = ct.clone();
                    cipher
                        .decrypt_in_place_detached(&mut decrypted, &nonce, &ad, &tag)
                        .unwrap_or_else(|e| panic!("wycheproof tcId={tc_id} decrypt failed: {e:?}"));
                    assert_eq!(decrypted, msg, "wycheproof tcId={tc_id} plaintext");
                    valid += 1;
                }
                "invalid" => {
                    let mut decrypted = ct.clone();
                    assert!(
                        cipher
                            .decrypt_in_place_detached(&mut decrypted, &nonce, &ad, &tag)
                            .is_err(),
                        "wycheproof tcId={tc_id} expected invalid"
                    );
                    assert!(decrypted.iter().all(|&b| b == 0));
                    invalid += 1;
                }
                other => panic!("unknown wycheproof result {other}"),
            }
        }
    }

    assert_eq!((valid, invalid), (360, 112));
}

#[cfg(all(target_arch = "x86_64", feature = "std"))]
#[test]
fn x86_backend_selection() {
    let aes = std::arch::is_x86_feature_detected!("aes");
    let vaes = aes && std::arch::is_x86_feature_detected!("vaes") && std::arch::is_x86_feature_detected!("avx2");
    let vaes512 =
        vaes && std::arch::is_x86_feature_detected!("avx512f") && std::arch::is_x86_feature_detected!("avx512bw");
    assert_eq!(detect::<1>(), x86_select::<1>(aes, vaes, vaes512));
    assert_eq!(detect::<2>(), x86_select::<2>(aes, vaes, vaes512));
    assert_eq!(detect::<4>(), x86_select::<4>(aes, vaes, vaes512));
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[test]
fn wasm_lane_d1_matches_block_major() {
    let key = [0x10u8; 32];
    let nonce = [0x20u8; 32];

    let mut lane = [0u8; 96];
    let mut block = [0u8; 96];
    unsafe { aegis_wasm32_simd128::init_state_lane::<1>(&key, &nonce, &mut lane) };
    unsafe { aegis_wasm32_simd128::init_state::<1>(&key, &nonce, &mut block) };
    assert_eq!(lane, block);

    for tag_len in [16usize, 32] {
        let mut lane_tag = [0u8; 32];
        let mut block_tag = [0u8; 32];
        unsafe { aegis_wasm32_simd128::finalize_mac_state_lane::<1>(&lane, 35, &mut lane_tag[..tag_len]) };
        let mut core = Core::<aegis_wasm32_simd128::Wasm<1>>::from_bytes(&block);
        core.finalize_mac(35, &mut block_tag[..tag_len]);
        assert_eq!(lane_tag[..tag_len], block_tag[..tag_len], "tag_len={tag_len}");
    }
}

#[test]
fn stream_large_matches_soft() {
    let key = [0x42u8; 32];
    let nonce = [0x24u8; 32];
    macro_rules! check {
        ($d:expr, $stream:ty) => {{
            for len in [1024usize, 4096, 16384] {
                let mut expected = vec![0u8; len];
                let mut tag = [0u8; 16];
                aegis_core::encrypt::<Soft<$d>>(&key, &nonce, &[], &mut expected, &mut tag);

                let mut fill = vec![0u8; len];
                <$stream>::new(&key, &nonce).fill_keystream(&mut fill);
                assert_eq!(fill, expected, "fill d={} len={len}", $d);

                let mut xor = expected.clone();
                <$stream>::new(&key, &nonce).xor_keystream_in_place(&mut xor);
                assert!(xor.iter().all(|&b| b == 0), "xor d={} len={len}", $d);
            }
        }};
    }
    check!(1, Aegis256Stream);
    check!(2, Aegis256X2Stream);
    check!(4, Aegis256X4Stream);
}
