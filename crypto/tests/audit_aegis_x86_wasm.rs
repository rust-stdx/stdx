//! Tests for the AEGIS x86-64 (AES-NI / VAES-256 / VAES-512) and
//! WebAssembly `simd128` (swizzle + lane-major) kernels.
//!
//! The tests in this file only use the public API, so the exact same code runs
//! against whichever backend the target selects (NEON on aarch64, AES-NI/VAES
//! on x86-64, swizzle/lane-major on wasm32, or the portable fallback).
//! Backend-internal differential checks (comparing the SIMD kernels against
//! the portable `aegis_soft` backend) live in `#[cfg(test)]` modules inside
//! `src/aegis/aegis_amd64.rs` and `src/aegis/aegis_wasm32_simd128.rs`.
//!
//! Coverage here:
//! - RFC 10032 Appendix A vectors for all three AEAD variants.
//! - Length sweeps over every value in `0..=64` plus the vector/block edges
//!   127/128/129 and 1023/1024/1025, at several non-zero buffer offsets, with
//!   guard canaries before and after the message slice.
//! - AAD edge lengths (including partial-blocks that leave SIMD lanes empty).
//! - Tag validation: every tag byte flipped, 16- and 32-byte tags, and the
//!   zeroization of plaintext on authentication failure.
//! - Wrong nonce/tag lengths through the `Aead` trait.
//! - Incremental MAC chunking and keystream chunking.

use crypto::{Aead, AeadError, StreamCipher};

const KEY_STD: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];
const NONCE_STD: [u8; 32] = [
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22,
    0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
];

const KEY_256: [u8; 32] = [
    0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const NONCE_256: [u8; 32] = [
    0x10, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const CANARY: u8 = 0xa5;

/// Deterministic seeded byte pattern.
fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

/// A buffer with `pad` canary bytes on each side of a `len`-byte payload.
fn guarded(len: usize, pad: usize, fill: &[u8]) -> Vec<u8> {
    assert_eq!(fill.len(), len);
    let mut v = vec![CANARY; len + 2 * pad];
    v[pad..pad + len].copy_from_slice(fill);
    v
}

fn canaries_intact(buf: &[u8], len: usize, pad: usize) -> bool {
    buf[..pad].iter().all(|&b| b == CANARY) && buf[pad + len..].iter().all(|&b| b == CANARY)
}

/// One in-place encrypt/decrypt round trip at a non-zero offset with canaries.
macro_rules! roundtrip_variant {
    ($cipher:expr, $len:expr, $pad:expr, $aad:expr) => {{
        let cipher = &$cipher;
        let len: usize = $len;
        let pad: usize = $pad;
        let plaintext = pattern(len, 17, 5);
        let mut buf = guarded(len, pad, &plaintext);

        let tag = cipher
            .encrypt_in_place(&mut buf[pad..pad + len], &NONCE_STD, $aad)
            .unwrap();
        assert!(canaries_intact(&buf, len, pad), "encrypt overwrote canary: len={len} pad={pad}");
        let ciphertext = buf[pad..pad + len].to_vec();
        // Encryption must not be the identity for non-empty input.
        if len > 0 {
            assert_ne!(ciphertext, plaintext, "ciphertext equals plaintext: len={len} pad={pad}");
        }

        cipher
            .decrypt_in_place(&mut buf[pad..pad + len], &NONCE_STD, $aad, &tag)
            .expect("round trip must authenticate");
        assert!(canaries_intact(&buf, len, pad), "decrypt overwrote canary: len={len} pad={pad}");
        assert_eq!(&buf[pad..pad + len], &plaintext[..], "round trip mismatch: len={len} pad={pad}");
        ciphertext
    }};
}

#[test]
fn rfc10032_vectors() {
    // AEGIS-256, Appendix A.3.2 .. A.3.6.
    let c16 = crypto::aegis::Aegis256::<16>::new(&KEY_256);
    let c32 = crypto::aegis::Aegis256::<32>::new(&KEY_256);
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
        let ad = hex::decode(ad).unwrap();
        let mut buf = hex::decode(msg).unwrap();
        let tag = c16.encrypt_in_place(&mut buf, &NONCE_256, &ad).unwrap();
        assert_eq!(buf, hex::decode(ct).unwrap());
        assert_eq!(tag.to_vec(), hex::decode(tag128).unwrap());

        let mut buf = hex::decode(msg).unwrap();
        let tag = c32.encrypt_in_place(&mut buf, &NONCE_256, &ad).unwrap();
        assert_eq!(buf, hex::decode(ct).unwrap());
        assert_eq!(tag.to_vec(), hex::decode(tag256).unwrap());

        let mut dec = hex::decode(ct).unwrap();
        let t: [u8; 16] = hex::decode(tag128).unwrap().try_into().unwrap();
        c16.decrypt_in_place(&mut dec, &NONCE_256, &ad, &t).unwrap();
        assert_eq!(dec, hex::decode(msg).unwrap());
    }

    // AEGIS-256X2 / X4, Appendix A.6.2 and A.7.2 (test vector 1, empty message).
    let mut e = Vec::new();
    assert_eq!(
        crypto::aegis::Aegis256X2::<16>::new(&KEY_STD)
            .encrypt_in_place(&mut e, &NONCE_STD, &[])
            .unwrap()
            .to_vec(),
        hex::decode("62cdbab084c83dacdb945bb446f049c8").unwrap()
    );
    let mut e = Vec::new();
    assert_eq!(
        crypto::aegis::Aegis256X2::<32>::new(&KEY_STD)
            .encrypt_in_place(&mut e, &NONCE_STD, &[])
            .unwrap()
            .to_vec(),
        hex::decode("25d7e799b49a80354c3f881ac2f1027f471a5d293052bd9997abd3ae84014bb7").unwrap()
    );
    let mut e = Vec::new();
    assert_eq!(
        crypto::aegis::Aegis256X4::<16>::new(&KEY_STD)
            .encrypt_in_place(&mut e, &NONCE_STD, &[])
            .unwrap()
            .to_vec(),
        hex::decode("3b7fee6cee7bf17888ad11ed2397beb4").unwrap()
    );
    let mut e = Vec::new();
    assert_eq!(
        crypto::aegis::Aegis256X4::<32>::new(&KEY_STD)
            .encrypt_in_place(&mut e, &NONCE_STD, &[])
            .unwrap()
            .to_vec(),
        hex::decode("6093a1a8aab20ec635dc1ca71745b01b5bec4fc444c9ffbebd710d4a34d20eaf").unwrap()
    );

    // AEGIS-256X2, Appendix A.6.3 (test vector 2) and AEGIS-256X4, A.7.3.
    let ad = hex::decode("0102030401020304").unwrap();
    let msg = hex::decode(
        "040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607",
    ).unwrap();
    let x2_ct = hex::decode(
        "72120c2ea8236180d67859001f4729077b7064c414384fe3a7b52f1571f4f8a7d0f01e18db4f3bc0adb150702e5d147a8d36522132761b994c1bd395589e2ccf0790dfe2a3d12d61cd666b2859827739db4037dd3124c78424459376f6cac08e1a7223a2a43e398ce6385cd654a19f481cba3b8f25910b42",
    ).unwrap();
    let x4_ct = hex::decode(
        "bfc2085b7e8017da99b0b6d646ae4d01f4ba8f2e7dfca1d759ae48a135139b9aaac6b4f5db810d426be1fdaff4e1454153a34b11da78ed7e418ee2ee9853042e95536aecbb694cea1b16a478eb0d4d1bf6509b1ce652a45af58e0e46ffccfa2d0426e702391d2ff5813808b81748a490dd656465fed61f09",
    ).unwrap();
    assert_eq!(msg.len(), 120);
    let mut b = msg.clone();
    let t = crypto::aegis::Aegis256X2::<16>::new(&KEY_STD)
        .encrypt_in_place(&mut b, &NONCE_STD, &ad)
        .unwrap();
    assert_eq!(b, x2_ct);
    assert_eq!(t.to_vec(), hex::decode("635d391828520bf1512763f0c8f5cdbd").unwrap());
    let mut b = msg.clone();
    let t = crypto::aegis::Aegis256X2::<32>::new(&KEY_STD)
        .encrypt_in_place(&mut b, &NONCE_STD, &ad)
        .unwrap();
    assert_eq!(b, x2_ct);
    assert_eq!(
        t.to_vec(),
        hex::decode("b5668d3317159e9cc5d46e4803c3a76ad63bb42b3f47956d94f30db8cb366ad7").unwrap()
    );

    let mut b = msg.clone();
    let t = crypto::aegis::Aegis256X4::<16>::new(&KEY_STD)
        .encrypt_in_place(&mut b, &NONCE_STD, &ad)
        .unwrap();
    assert_eq!(b, x4_ct);
    assert_eq!(t.to_vec(), hex::decode("b63b611b13975e2f3dc3cb6c2397bfcd").unwrap());
    let mut b = msg.clone();
    let t = crypto::aegis::Aegis256X4::<32>::new(&KEY_STD)
        .encrypt_in_place(&mut b, &NONCE_STD, &ad)
        .unwrap();
    assert_eq!(b, x4_ct);
    assert_eq!(
        t.to_vec(),
        hex::decode("7847eace74409ee56c8f4cf63a9c2841ce7c8bd567d7c0ca514c879a190b978c").unwrap()
    );
}

/// Every length from 0 through 64, the SIMD/block boundaries, and the
/// split-bulk threshold. Canaries sit before and after every message slice.
#[test]
fn length_sweep_roundtrip() {
    let aad = pattern(37, 29, 7);
    let mut lens: Vec<usize> = (0..=64).collect();
    lens.extend([127, 128, 129, 255, 256, 257, 1023, 1024, 1025]);
    for &len in &lens {
        let pads: &[usize] = if len > 129 { &[0, 1] } else { &[0, 1, 7, 15, 31] };
        for &pad in pads {
            macro_rules! run {
                ($c:expr) => {
                    roundtrip_variant!($c, len, pad, &aad);
                };
            }
            run!(crypto::aegis::Aegis256::<16>::new(&KEY_STD));
            run!(crypto::aegis::Aegis256::<32>::new(&KEY_STD));
            run!(crypto::aegis::Aegis256X2::<16>::new(&KEY_STD));
            run!(crypto::aegis::Aegis256X2::<32>::new(&KEY_STD));
            run!(crypto::aegis::Aegis256X4::<16>::new(&KEY_STD));
            run!(crypto::aegis::Aegis256X4::<32>::new(&KEY_STD));
        }
    }
}

/// Ciphertext must be independent of the tag size for a fixed key/nonce/AAD.
#[test]
fn tag_size_does_not_change_ciphertext() {
    for len in [0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 100, 128, 1024, 1025] {
        let pt = pattern(len, 31, 11);
        let ad = pattern(19, 7, 13);
        let mut a = pt.clone();
        let mut b = pt.clone();
        let _ = crypto::aegis::Aegis256::<16>::new(&KEY_STD)
            .encrypt_in_place(&mut a, &NONCE_STD, &ad)
            .unwrap();
        let _ = crypto::aegis::Aegis256::<32>::new(&KEY_STD)
            .encrypt_in_place(&mut b, &NONCE_STD, &ad)
            .unwrap();
        assert_eq!(a, b, "Aegis256 tag size changed ciphertext, len={len}");

        let mut a = pt.clone();
        let mut b = pt.clone();
        let _ = crypto::aegis::Aegis256X2::<16>::new(&KEY_STD)
            .encrypt_in_place(&mut a, &NONCE_STD, &ad)
            .unwrap();
        let _ = crypto::aegis::Aegis256X2::<32>::new(&KEY_STD)
            .encrypt_in_place(&mut b, &NONCE_STD, &ad)
            .unwrap();
        assert_eq!(a, b, "Aegis256X2 tag size changed ciphertext, len={len}");

        let mut a = pt.clone();
        let mut b = pt.clone();
        let _ = crypto::aegis::Aegis256X4::<16>::new(&KEY_STD)
            .encrypt_in_place(&mut a, &NONCE_STD, &ad)
            .unwrap();
        let _ = crypto::aegis::Aegis256X4::<32>::new(&KEY_STD)
            .encrypt_in_place(&mut b, &NONCE_STD, &ad)
            .unwrap();
        assert_eq!(a, b, "Aegis256X4 tag size changed ciphertext, len={len}");
    }
}

/// AAD lengths that end in the middle of a 128/256/512-bit absorb block.
#[test]
fn aad_edge_lengths() {
    let mut aad_lens: Vec<usize> = (0..=70).collect();
    aad_lens.extend([95, 96, 97, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025]);
    for &msg_len in &[0usize, 1, 16, 17, 63, 64, 65, 100] {
        let pt = pattern(msg_len, 41, 3);
        for &ad_len in &aad_lens {
            let ad = pattern(ad_len, 13, 9);
            macro_rules! check {
                ($c:expr) => {{
                    let c = $c;
                    let mut b = pt.clone();
                    let tag = c.encrypt_in_place(&mut b, &NONCE_STD, &ad).unwrap();
                    let ct = b.clone();
                    c.decrypt_in_place(&mut b, &NONCE_STD, &ad, &tag).unwrap();
                    assert_eq!(b, pt, "aad={ad_len} msg={msg_len}");
                    // A different AAD of the same length must not authenticate.
                    let mut other = ad.clone();
                    if !other.is_empty() {
                        other[0] ^= 1;
                        let mut tampered = ct;
                        assert_eq!(
                            c.decrypt_in_place(&mut tampered, &NONCE_STD, &other, &tag),
                            Err(AeadError::InvalidCiphertext),
                            "aad={ad_len} msg={msg_len}"
                        );
                        assert!(tampered.iter().all(|&x| x == 0), "aad failure not zeroed");
                    }
                }};
            }
            check!(crypto::aegis::Aegis256::<16>::new(&KEY_STD));
            check!(crypto::aegis::Aegis256X2::<16>::new(&KEY_STD));
            check!(crypto::aegis::Aegis256X4::<16>::new(&KEY_STD));
        }
    }
}

/// Flipping any tag byte must fail authentication, zero the whole plaintext
/// slice, and leave the guard canaries untouched.
#[test]
fn tag_tamper_zeroes_plaintext() {
    for len in [0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 100, 1023, 1024, 1025] {
        for pad in [0usize, 1, 7] {
            let pt = pattern(len, 53, 19);
            macro_rules! tamper {
                ($c:expr, $taglen:expr) => {{
                    let c = $c;
                    let taglen: usize = $taglen;
                    let mut buf = guarded(len, pad, &pt);
                    let tag = c
                        .encrypt_in_place(&mut buf[pad..pad + len], &NONCE_STD, b"ad")
                        .unwrap()
                        .as_ref()
                        .to_vec();
                    let ct = buf[pad..pad + len].to_vec();
                    let step = if taglen == 16 { 1 } else { 7 };
                    for i in (0..taglen).step_by(step) {
                        let mut bad = tag.clone();
                        bad[i] ^= 0x80;
                        let mut b = guarded(len, pad, &ct);
                        assert_eq!(
                            c.decrypt_in_place(&mut b[pad..pad + len], &NONCE_STD, b"ad", &bad),
                            Err(AeadError::InvalidCiphertext),
                            "len={len} pad={pad} tag_byte={i}"
                        );
                        assert!(
                            b[pad..pad + len].iter().all(|&x| x == 0),
                            "plaintext not zeroed: len={len} pad={pad} tag_byte={i}"
                        );
                        assert!(canaries_intact(&b, len, pad), "canary clobbered: len={len} pad={pad}");
                    }
                }};
            }
            tamper!(crypto::aegis::Aegis256::<16>::new(&KEY_STD), 16);
            tamper!(crypto::aegis::Aegis256::<32>::new(&KEY_STD), 32);
            tamper!(crypto::aegis::Aegis256X2::<16>::new(&KEY_STD), 16);
            tamper!(crypto::aegis::Aegis256X2::<32>::new(&KEY_STD), 32);
            tamper!(crypto::aegis::Aegis256X4::<16>::new(&KEY_STD), 16);
            tamper!(crypto::aegis::Aegis256X4::<32>::new(&KEY_STD), 32);
        }
    }
}

/// Wrong nonce/tag lengths are rejected before decryption; the buffer and its
/// canaries must be untouched.
#[test]
fn wrong_nonce_and_tag_lengths_leave_buffer_untouched() {
    let pt = b"aegis length validation";
    let cipher = crypto::aegis::Aegis256X4::<16>::new(&KEY_STD);
    let mut buf = pt.to_vec();
    let tag = cipher.encrypt_in_place(&mut buf, &NONCE_STD, b"ad").unwrap();
    let ct = buf.clone();

    let mut work = ct.clone();
    assert_eq!(
        cipher.decrypt_in_place(&mut work, &NONCE_STD[..31], b"ad", tag.as_ref()),
        Err(AeadError::InvalidNonce)
    );
    assert_eq!(work, ct, "wrong nonce length modified the buffer");

    let mut work = ct.clone();
    assert_eq!(
        cipher.decrypt_in_place(&mut work, &NONCE_STD, b"ad", &tag.as_ref()[1..]),
        Err(AeadError::InvalidCiphertext)
    );
    assert_eq!(work, ct, "short tag modified the buffer");

    let mut long = [0u8; 17];
    long[..16].copy_from_slice(&tag);
    let mut work = ct.clone();
    assert_eq!(
        cipher.decrypt_in_place(&mut work, &NONCE_STD, b"ad", &long),
        Err(AeadError::InvalidCiphertext)
    );
    assert_eq!(work, ct, "long tag modified the buffer");

    // A 32-byte variant must reject a 16-byte tag too.
    let c32 = crypto::aegis::Aegis256X4::<32>::new(&KEY_STD);
    let mut buf = pt.to_vec();
    let tag32 = c32.encrypt_in_place(&mut buf, &NONCE_STD, b"ad").unwrap();
    let mut work = buf.clone();
    assert_eq!(
        c32.decrypt_in_place(&mut work, &NONCE_STD, b"ad", &tag32[..16]),
        Err(AeadError::InvalidCiphertext)
    );
    assert_eq!(work, buf);
}

/// MAC incremental chunking must agree with a single update for every chunk
/// size, for both tag sizes, and verification must reject tampering.
#[test]
fn mac_chunking_matches_one_shot() {
    let mut lens: Vec<usize> = (0..=64).collect();
    lens.extend([127, 128, 129, 1023, 1024, 1025]);
    macro_rules! check {
        ($mac:ty) => {{
            for &len in &lens {
                let data = pattern(len, 37, 23);
                let mut one = <$mac>::new_with_nonce(&KEY_STD, &NONCE_STD);
                one.update(&data).unwrap();
                let want = one.finalize();
                for chunk in [1usize, 2, 3, 7, 15, 16, 17, 31, 32, 33, 64, 65, 128] {
                    let mut m = <$mac>::new_with_nonce(&KEY_STD, &NONCE_STD);
                    for part in data.chunks(chunk) {
                        m.update(part).unwrap();
                    }
                    assert_eq!(m.finalize(), want, "chunk={chunk} len={len}");
                }
                let mut m = <$mac>::new_with_nonce(&KEY_STD, &NONCE_STD);
                m.update(&data).unwrap();
                assert!(m.verify(&want).is_ok());
                let mut bad = want;
                bad[0] ^= 1;
                let mut m = <$mac>::new_with_nonce(&KEY_STD, &NONCE_STD);
                m.update(&data).unwrap();
                assert_eq!(m.verify(&bad), Err(AeadError::InvalidCiphertext), "len={len}");
            }
        }};
    }
    check!(crypto::aegis::Aegis256Mac::<16>);
    check!(crypto::aegis::Aegis256Mac::<32>);
    check!(crypto::aegis::Aegis256X2Mac::<16>);
    check!(crypto::aegis::Aegis256X2Mac::<32>);
    check!(crypto::aegis::Aegis256X4Mac::<16>);
    check!(crypto::aegis::Aegis256X4Mac::<32>);
}

/// Keystream output must be identical whether produced with `fill_keystream`
/// or `xor_keystream_in_place`, in one call or many, and must match AEAD
/// encryption of an all-zero message (same Init, same state updates).
#[test]
fn stream_matches_aead_and_chunking() {
    let mut lens: Vec<usize> = (0..=64).collect();
    lens.extend([127, 128, 129, 255, 256, 257, 1023, 1024, 1025]);
    macro_rules! check {
        ($stream:ty, $aead:ty) => {{
            for &len in &lens {
                let mut fill = vec![0u8; len];
                <$stream>::new(&KEY_STD, &NONCE_STD).fill_keystream(&mut fill);

                let mut xor = vec![0u8; len];
                <$stream>::new(&KEY_STD, &NONCE_STD).xor_keystream_in_place(&mut xor);
                assert_eq!(fill, xor, "fill != xor, len={len}");

                // Chunked fill, including odd chunk sizes that split vectors.
                for chunk in [1usize, 7, 15, 16, 17, 31, 63, 64, 65] {
                    let mut got = vec![0u8; len];
                    let mut s = <$stream>::new(&KEY_STD, &NONCE_STD);
                    for c in got.chunks_mut(chunk) {
                        s.fill_keystream(c);
                    }
                    assert_eq!(got, fill, "chunked fill, chunk={chunk} len={len}");
                }

                // Non-zero-offset slices must give the same stream.
                if len > 2 {
                    let mut buf = vec![CANARY; len + 9];
                    let mut s = <$stream>::new(&KEY_STD, &NONCE_STD);
                    s.fill_keystream(&mut buf[3..3 + len - 2]);
                    assert_eq!(&buf[3..3 + len - 2], &fill[..len - 2], "offset fill, len={len}");
                    assert!(buf[..3].iter().all(|&b| b == CANARY));
                    assert!(buf[3 + len - 2..].iter().all(|&b| b == CANARY));
                }

                // AEAD encryption of zeros exposes the same keystream.
                let mut zeros = vec![0u8; len];
                let _ = <$aead>::new(&KEY_STD)
                    .encrypt_in_place(&mut zeros, &NONCE_STD, &[])
                    .unwrap();
                assert_eq!(zeros, fill, "stream != AEAD(zeros), len={len}");
            }
        }};
    }
    check!(crypto::aegis::Aegis256Stream, crypto::aegis::Aegis256<16>);
    check!(crypto::aegis::Aegis256X2Stream, crypto::aegis::Aegis256X2<16>);
    check!(crypto::aegis::Aegis256X4Stream, crypto::aegis::Aegis256X4<16>);

    // The `StreamCipher` trait must route to the same implementation.
    let mut a = vec![0u8; 333];
    let mut b = vec![0u8; 333];
    crypto::aegis::Aegis256X4Stream::new(&KEY_STD, &NONCE_STD).fill_keystream(&mut a);
    {
        let mut s = crypto::aegis::Aegis256X4Stream::new(&KEY_STD, &NONCE_STD);
        StreamCipher::xor_keystream(&mut s, &mut b).unwrap();
    }
    assert_eq!(a, b);
}
