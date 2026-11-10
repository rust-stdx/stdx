//! Public API tests for the AEGIS module.
//!
//! The RFC vectors themselves are unit-tested inside the module, where the
//! software reference backend is reachable. These tests exercise the exported
//! types, unaligned slices and the trait interfaces.

use crypto::{
    StreamCipher,
    aegis::{
        Aegis256, Aegis256Mac, Aegis256Stream, Aegis256X2, Aegis256X2Mac, Aegis256X2Stream, Aegis256X4, Aegis256X4Mac,
        Aegis256X4Stream,
    },
};

const KEY: [u8; 32] = [0x42; 32];
const NONCE: [u8; 32] = [0x24; 32];

fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

macro_rules! roundtrip {
    ($cipher:expr, $len:expr, $offset:expr) => {{
        let plaintext = pattern($len, 17, 3);
        let aad = pattern(37, 29, 7);
        let cipher = $cipher;
        let mut buffer = vec![0xa5u8; $len + 32];
        buffer[$offset..$offset + $len].copy_from_slice(&plaintext);
        let tag = cipher.encrypt_in_place_detached(&mut buffer[$offset..$offset + $len], &NONCE, &aad);
        assert!(buffer[..$offset].iter().all(|&b| b == 0xa5));
        assert!(buffer[$offset + $len..].iter().all(|&b| b == 0xa5));
        cipher
            .decrypt_in_place_detached(&mut buffer[$offset..$offset + $len], &NONCE, &aad, &tag)
            .unwrap();
        assert_eq!(&buffer[$offset..$offset + $len], &plaintext[..]);
    }};
}

#[test]
fn aead_roundtrip_unaligned() {
    for len in [0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 1024, 4096] {
        for offset in [0usize, 1, 7, 15] {
            roundtrip!(Aegis256::<16>::new(&KEY), len, offset);
            roundtrip!(Aegis256::<32>::new(&KEY), len, offset);
            roundtrip!(Aegis256X2::<16>::new(&KEY), len, offset);
            roundtrip!(Aegis256X2::<32>::new(&KEY), len, offset);
            roundtrip!(Aegis256X4::<16>::new(&KEY), len, offset);
            roundtrip!(Aegis256X4::<32>::new(&KEY), len, offset);
        }
    }
}

#[test]
fn mac_verify_and_stream() {
    let data = pattern(1000, 11, 2);
    let mut mac = Aegis256Mac::<16>::new_with_nonce(&KEY, &NONCE);
    mac.update(&data[..333]);
    mac.update(&data[333..]);
    let tag = mac.clone().finalize();
    assert!(mac.verify(&tag).is_ok());

    let mut bad = tag;
    bad[0] ^= 0x80;
    let mut mac = Aegis256Mac::<16>::new_with_nonce(&KEY, &NONCE);
    mac.update(&data);
    assert!(mac.verify(&bad).is_err());

    macro_rules! stream_roundtrip {
        ($stream:ty) => {{
            let mut encrypted = data.clone();
            let mut stream = <$stream>::new(&KEY, &NONCE);
            StreamCipher::xor_keystream(&mut stream, &mut encrypted);
            assert_ne!(encrypted, data);
            let mut stream = <$stream>::new(&KEY, &NONCE);
            StreamCipher::xor_keystream(&mut stream, &mut encrypted);
            assert_eq!(encrypted, data);
        }};
    }
    stream_roundtrip!(Aegis256Stream);
    stream_roundtrip!(Aegis256X2Stream);
    stream_roundtrip!(Aegis256X4Stream);
}

#[test]
fn mac_types_compile_for_both_tags() {
    let _ = Aegis256X2Mac::<32>::new(&KEY);
    let _ = Aegis256X4Mac::<16>::new(&KEY);
}
