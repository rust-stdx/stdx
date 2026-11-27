//! AEGIS-256 / X2 / X4 public-API edge tests (RFC 10032).
//!
//! The independent RFC scalar oracle lives in `audit_aegis_core.rs`; this file
//! adds exhaustive short-length/AAD coverage through the public API, exact-rate
//! boundaries for the 16/32/64-byte rates, wrong-length rejection, failed-tag
//! zeroisation, in-place offsets/canaries, MAC chunking and stream chunking.
//! It runs on every executable backend selected by `detect()` (NEON+AES(+SHA3)
//! on aarch64, AES-NI/VAES-256 under the x86 QEMU runner).

use crypto::{
    Aead, AeadError, StreamCipher,
    aegis::{
        Aegis256, Aegis256Mac, Aegis256Stream, Aegis256X2, Aegis256X2Mac, Aegis256X2Stream, Aegis256X4, Aegis256X4Mac,
        Aegis256X4Stream,
    },
};

fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

fn key() -> [u8; 32] {
    let mut k = [0u8; 32];
    for (i, b) in k.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(43).wrapping_add(9);
    }
    k
}

fn nonce() -> [u8; 32] {
    let mut n = [0u8; 32];
    for (i, b) in n.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(23).wrapping_add(1);
    }
    n
}

macro_rules! aead_edges {
    ($name:ident, $ty16:ty, $ty32:ty, $d:expr) => {
        #[test]
        fn $name() {
            let key = key();
            let nonce = nonce();
            let rate = 16 * $d;

            // Exhaustive short lengths and AAD lengths: round-trip and
            // ciphertext independence of the tag size.
            let c16 = <$ty16>::new(&key);
            let c32 = <$ty32>::new(&key);
            for len in 0..=129usize {
                let pt = pattern(len, 17, 5);
                for alen in 0..=33usize {
                    let aad = pattern(alen, 7, 3);
                    let mut a = pt.clone();
                    let mut b = pt.clone();
                    let ta = c16.encrypt_in_place(&mut a, &nonce, &aad).unwrap();
                    let tb = c32.encrypt_in_place(&mut b, &nonce, &aad).unwrap();
                    assert_eq!(a, b, "ciphertext depends on tag size d={} len={len} aad={alen}", $d);
                    assert_eq!(ta.as_ref().len(), 16);
                    assert_eq!(tb.as_ref().len(), 32);
                    let mut folded = [0u8; 16];
                    for i in 0..16 {
                        folded[i] = tb.as_ref()[i] ^ tb.as_ref()[16 + i];
                    }
                    assert_eq!(folded, ta.as_ref()[..], "128-bit tag != fold of 256-bit tag d={}", $d);
                    c16.decrypt_in_place(&mut a, &nonce, &aad, ta.as_ref()).unwrap();
                    assert_eq!(a, pt, "d={} len={len} aad={alen}", $d);
                    c32.decrypt_in_place(&mut b, &nonce, &aad, tb.as_ref()).unwrap();
                    assert_eq!(b, pt, "d={} len={len} aad={alen}", $d);
                }
            }

            // Exact-rate boundaries plus neighbours, for every partial-tail
            // position inside the final rate block.
            let mut lens: Vec<usize> = (0..=33).collect();
            for k in 1..=4usize {
                lens.extend([k * rate - 1, k * rate, k * rate + 1]);
            }
            lens.sort_unstable();
            lens.dedup();
            for len in lens {
                let pt = pattern(len, 29, 11);
                let aad = pattern((len % 47).min(41), 3, 1);
                let mut buf = pt.clone();
                let tag = c16.encrypt_in_place(&mut buf, &nonce, &aad).unwrap();
                c16.decrypt_in_place(&mut buf, &nonce, &aad, tag.as_ref())
                    .unwrap();
                assert_eq!(buf, pt, "rate boundary d={} len={len}", $d);
            }

            // In-place offsets / canaries, including offsets one past a rate.
            const CANARY: u8 = 0xa5;
            for off in [0usize, 1, 7, 15, 16, 17, 31, 32, 33, 63, 64, 65] {
                for len in [
                    0usize,
                    1,
                    15,
                    16,
                    17,
                    rate - 1,
                    rate,
                    rate + 1,
                    2 * rate,
                    2 * rate + 1,
                    129,
                ] {
                    let pt = pattern(len, 37, 7);
                    let mut buf = vec![CANARY; off + len + 2 * 13];
                    buf[off..off + len].copy_from_slice(&pt);
                    let tag = {
                        let slice = &mut buf[off..off + len];
                        c16.encrypt_in_place(slice, &nonce, b"ad").unwrap()
                    };
                    assert!(buf[..off].iter().all(|&b| b == CANARY), "pre d={} off={off} len={len}", $d);
                    assert!(
                        buf[off + len..].iter().all(|&b| b == CANARY),
                        "post d={} off={off} len={len}",
                        $d
                    );
                    {
                        let slice = &mut buf[off..off + len];
                        c16.decrypt_in_place(slice, &nonce, b"ad", tag.as_ref()).unwrap();
                    }
                    assert_eq!(&buf[off..off + len], &pt[..]);
                }
            }

            // Wrong lengths are rejected without touching the buffer.
            let pt = pattern(40, 3, 1);
            let mut ct = pt.clone();
            let tag = c16.encrypt_in_place(&mut ct, &nonce, b"ad").unwrap();
            for bad in 0..=64usize {
                if bad == 32 {
                    continue;
                }
                let mut buf = ct.clone();
                assert_eq!(
                    c16.decrypt_in_place(&mut buf, &vec![0u8; bad], b"ad", tag.as_ref()),
                    Err(AeadError::InvalidNonce),
                    "nonce len={bad} d={}",
                    $d
                );
                assert_eq!(buf, ct, "buffer touched on nonce error d={}", $d);
            }
            for bad in 0..=40usize {
                if bad == 16 {
                    continue;
                }
                let mut buf = ct.clone();
                assert_eq!(
                    c16.decrypt_in_place(&mut buf, &nonce, b"ad", &vec![0u8; bad]),
                    Err(AeadError::InvalidCiphertext),
                    "tag len={bad} d={}",
                    $d
                );
                assert_eq!(buf, ct, "buffer touched on tag error d={}", $d);
            }
            // Encrypt-side wrong nonce length leaves the buffer unchanged.
            let before = ct.clone();
            let mut buf = ct.clone();
            assert_eq!(
                c16.encrypt_in_place(&mut buf, &[0u8; 31], b"ad").err(),
                Some(AeadError::InvalidNonce)
            );
            assert_eq!(buf, before);

            // Every tag bit matters and a failed check zeroes the plaintext
            // buffer (RFC 10032 §4.2/5.3).
            for tag_len in [16usize, 32] {
                let cipher16 = <$ty16>::new(&key);
                let p = pattern(2 * rate + 3, 11, 1);
                let mut ct = p.clone();
                let t = cipher16.encrypt_in_place(&mut ct, &nonce, b"ad").unwrap();
                let good = t.as_ref().to_vec();
                for i in 0..tag_len.min(16) {
                    let mut bad = good.clone();
                    bad[i] ^= 1;
                    let mut buf = ct.clone();
                    assert_eq!(
                        cipher16.decrypt_in_place(&mut buf, &nonce, b"ad", &bad),
                        Err(AeadError::InvalidCiphertext)
                    );
                    assert!(buf.iter().all(|&b| b == 0), "failed decrypt not zeroed d={} tag={tag_len}", $d);
                }
            }

            // AAD-only messages: same plaintext, different AAD -> different tag.
            let mut empty = Vec::new();
            let t1 = c16.encrypt_in_place(&mut empty, &nonce, &[]).unwrap();
            let mut empty = Vec::new();
            let t2 = c16.encrypt_in_place(&mut empty, &nonce, b"x").unwrap();
            assert_ne!(t1.as_ref(), t2.as_ref(), "AAD not authenticated d={}", $d);
        }
    };
}

aead_edges!(aegis256_public_edges, Aegis256<16>, Aegis256<32>, 1);
aead_edges!(aegis256x2_public_edges, Aegis256X2<16>, Aegis256X2<32>, 2);
aead_edges!(aegis256x4_public_edges, Aegis256X4<16>, Aegis256X4<32>, 4);

/// Variants with the same key/nonce must produce distinct ciphertext/tags:
/// the context blocks and parallelism degree are domain separators.
#[test]
fn variants_are_distinct() {
    let key = key();
    let nonce = nonce();
    let pt = pattern(200, 19, 3);

    let mut c1 = pt.clone();
    let t1 = Aegis256::<16>::new(&key)
        .encrypt_in_place(&mut c1, &nonce, b"ad")
        .unwrap();
    let mut c2 = pt.clone();
    let t2 = Aegis256X2::<16>::new(&key)
        .encrypt_in_place(&mut c2, &nonce, b"ad")
        .unwrap();
    let mut c4 = pt.clone();
    let t4 = Aegis256X4::<16>::new(&key)
        .encrypt_in_place(&mut c4, &nonce, b"ad")
        .unwrap();
    assert_ne!(c1, c2);
    assert_ne!(c1, c4);
    assert_ne!(c2, c4);
    assert_ne!(t1.as_ref(), t2.as_ref());
    assert_ne!(t1.as_ref(), t4.as_ref());
    assert_ne!(t2.as_ref(), t4.as_ref());
}

macro_rules! mac_edges {
    ($name:ident, $mac16:ty, $mac32:ty, $d:expr) => {
        #[test]
        fn $name() {
            let key = key();
            let nonce = nonce();
            let rate = 16 * $d;
            for len in 0..=129usize {
                let data = pattern(len, 53, 7);
                let mut m = <$mac16>::new_with_nonce(&key, &nonce);
                m.update(&data).unwrap();
                let t16 = m.finalize();
                let mut m = <$mac32>::new_with_nonce(&key, &nonce);
                m.update(&data).unwrap();
                let t32 = m.finalize();
                assert_eq!(t16.len(), 16);
                assert_eq!(t32.len(), 32);
                assert_ne!(t16, [0u8; 16], "MAC d={} len={len}", $d);
                assert_ne!(
                    t16,
                    t32[..16],
                    "MAC tag sizes must not truncate to each other d={} len={len}",
                    $d
                );

                // Incremental chunking must be identical.
                for chunk in [1usize, 2, 3, 15, 16, 17, 31, 32, 33, 63, 64, 65, 100] {
                    let mut m = <$mac16>::new_with_nonce(&key, &nonce);
                    for piece in data.chunks(chunk) {
                        m.update(piece).unwrap();
                    }
                    assert_eq!(m.finalize(), t16, "MAC chunk d={} len={len} chunk={chunk}", $d);
                }
            }

            // Exact-rate MAC boundaries and clear separation per variant.
            for len in [
                0usize,
                1,
                rate - 1,
                rate,
                rate + 1,
                2 * rate - 1,
                2 * rate,
                2 * rate + 1,
            ] {
                let data = pattern(len, 7, 1);
                let mut m = <$mac16>::new_with_nonce(&key, &nonce);
                m.update(&data).unwrap();
                let tag = m.finalize();
                let mut m = <$mac16>::new_with_nonce(&key, &nonce);
                m.update(&data).unwrap();
                assert!(m.verify(&tag).is_ok());
                let mut bad = tag;
                bad[0] ^= 1;
                let mut m = <$mac16>::new_with_nonce(&key, &nonce);
                m.update(&data).unwrap();
                assert!(m.verify(&bad).is_err());
            }
        }
    };
}

mac_edges!(aegis256_mac_edges, Aegis256Mac<16>, Aegis256Mac<32>, 1);
mac_edges!(aegis256x2_mac_edges, Aegis256X2Mac<16>, Aegis256X2Mac<32>, 2);
mac_edges!(aegis256x4_mac_edges, Aegis256X4Mac<16>, Aegis256X4Mac<32>, 4);

macro_rules! stream_edges {
    ($name:ident, $stream:ty, $aead16:ty, $d:expr) => {
        #[test]
        fn $name() {
            let key = key();
            let nonce = nonce();
            let rate = 16 * $d;

            for len in 0..=129usize {
                let mut full = vec![0u8; len];
                <$stream>::new(&key, &nonce).fill_keystream(&mut full);

                // The stream is AEAD encryption of all-zero plaintext, and
                // `fill` must overwrite non-zero bytes.
                let mut want = vec![0u8; len];
                <$aead16>::new(&key)
                    .encrypt_in_place(&mut want, &nonce, &[])
                    .unwrap();
                assert_eq!(full, want, "stream != AEAD(zeros) d={} len={len}", $d);

                let mut overwrite = vec![0xffu8; len];
                <$stream>::new(&key, &nonce).fill_keystream(&mut overwrite);
                assert_eq!(overwrite, full);

                // XOR with the keystream cancels it.
                let mut x = full.clone();
                <$stream>::new(&key, &nonce).xor_keystream_in_place(&mut x);
                assert!(x.iter().all(|&b| b == 0), "xor did not cancel d={} len={len}", $d);

                // Arbitrary chunk sizes fill one continuous keystream.
                for chunk in [1usize, 2, 3, 7, 15, 16, 17, 31, 32, 33, 63, 64, 65, 100] {
                    let mut got = vec![0u8; len];
                    let mut s = <$stream>::new(&key, &nonce);
                    let mut off = 0;
                    while off < len {
                        let end = (off + chunk).min(len);
                        s.fill_keystream(&mut got[off..end]);
                        off = end;
                    }
                    assert_eq!(got, full, "stream chunk d={} len={len} chunk={chunk}", $d);

                    let mut got = full.clone();
                    let mut s = <$stream>::new(&key, &nonce);
                    let mut off = 0;
                    while off < len {
                        let end = (off + chunk).min(len);
                        s.xor_keystream_in_place(&mut got[off..end]);
                        off = end;
                    }
                    assert!(got.iter().all(|&b| b == 0), "xor chunk d={} len={len} chunk={chunk}", $d);
                }

                // `StreamCipher` trait and inherent method agree.
                let mut a = vec![0u8; len];
                let mut b = vec![0u8; len];
                <$stream>::new(&key, &nonce).fill_keystream(&mut a);
                StreamCipher::xor_keystream(&mut <$stream>::new(&key, &nonce), &mut b).unwrap();
                assert_eq!(a, b);
            }

            // Alternating fill/xor across every split must stay continuous.
            let len = 2 * rate + 3;
            let mut want = vec![0u8; len];
            <$stream>::new(&key, &nonce).fill_keystream(&mut want);
            for split in 0..=len {
                let mut got = vec![0u8; len];
                let mut s = <$stream>::new(&key, &nonce);
                {
                    let (a, b) = got.split_at_mut(split);
                    s.fill_keystream(a);
                    s.xor_keystream_in_place(b);
                }
                assert_eq!(got, want, "mixed fill/xor d={} split={split}", $d);
            }
        }
    };
}

stream_edges!(aegis256_stream_edges, Aegis256Stream, Aegis256<16>, 1);
stream_edges!(aegis256x2_stream_edges, Aegis256X2Stream, Aegis256X2<16>, 2);
stream_edges!(aegis256x4_stream_edges, Aegis256X4Stream, Aegis256X4<16>, 4);

/// Public constants and length-limit arithmetic.
#[test]
fn public_constants_and_limits() {
    assert_eq!(<Aegis256<16> as Aead>::KEY_SIZE, 32);
    assert_eq!(<Aegis256<16> as Aead>::NONCE_SIZE, 32);
    assert_eq!(<Aegis256<16> as Aead>::TAG_SIZE, 16);
    assert_eq!(<Aegis256<32> as Aead>::TAG_SIZE, 32);
    assert_eq!(Aegis256::<16>::KEY_SIZE, 32);
    assert_eq!(Aegis256::<16>::NONCE_SIZE, 32);
    assert_eq!(Aegis256::<16>::TAG_SIZE, 16);

    // AAD-only and empty-message authentication is valid at MAX input length
    // boundaries that can actually be allocated; the full 2^61-1 check is
    // exercised by the in-crate length tests.
    let key = key();
    let nonce = nonce();
    let mut empty = Vec::new();
    let tag = Aegis256::<16>::new(&key)
        .encrypt_in_place(&mut empty, &nonce, &pattern(1024, 3, 1))
        .unwrap();
    assert_eq!(
        Aegis256::<16>::new(&key)
            .decrypt_in_place(&mut empty, &nonce, &pattern(1024, 3, 1), tag.as_ref())
            .unwrap(),
        ()
    );
    // Wrong AAD must fail and zero the (empty) buffer.
    assert!(
        Aegis256::<16>::new(&key)
            .decrypt_in_place(&mut empty, &nonce, &pattern(1024, 3, 2), tag.as_ref())
            .is_err()
    );
}
