//! Cross-backend tests for the AES hardware paths.
//!
//! These tests exercise the public dispatch (AES-NI, VAES-256, VAES-512, ARMv8,
//! or software) and compare it against a portable reference built from the
//! software block cipher. They intentionally use lengths around every SIMD loop
//! boundary so that the 128-bit, 256-bit, and 512-bit kernels are all reached on
//! hardware that supports them.

use crypto::{
    Aead, StreamCipher,
    aes::{Aes128Block, Aes128Ctr, Aes128Gcm, Aes256Block, Aes256Ctr, Aes256Gcm},
};

const KEY_128: [u8; 16] = [
    0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
];
const KEY_256: [u8; 32] = [
    0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81, 0x1f, 0x35, 0x2c,
    0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4,
];

fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

fn increment(counter: &mut [u8; 16]) {
    for byte in counter.iter_mut().rev() {
        let (value, carry) = byte.overflowing_add(1);
        *byte = value;
        if !carry {
            break;
        }
    }
}

fn ctr_reference_128(counter: &[u8; 16], data: &mut [u8]) {
    let cipher = Aes128Block::new(&KEY_128);
    let mut counter = *counter;
    for chunk in data.chunks_mut(16) {
        let keystream = cipher.encrypt_block(&counter);
        for (byte, key_byte) in chunk.iter_mut().zip(keystream.iter()) {
            *byte ^= *key_byte;
        }
        increment(&mut counter);
    }
}

fn ctr_reference_256(counter: &[u8; 16], data: &mut [u8]) {
    let cipher = Aes256Block::new(&KEY_256);
    let mut counter = *counter;
    for chunk in data.chunks_mut(16) {
        let keystream = cipher.encrypt_block(&counter);
        for (byte, key_byte) in chunk.iter_mut().zip(keystream.iter()) {
            *byte ^= *key_byte;
        }
        increment(&mut counter);
    }
}

fn boundary_lengths() -> Vec<usize> {
    let mut lengths = Vec::new();
    for base in [0usize, 16, 32, 64, 128, 256, 512, 1024, 4096, 16 * 1024, 1024 * 1024] {
        for delta in [0usize, 1, 2, 15, 16, 17, 31, 32, 33] {
            lengths.push(base.saturating_add(delta));
        }
    }
    lengths.sort_unstable();
    lengths.dedup();
    lengths
}

#[test]
fn ctr_matches_software_at_all_boundaries() {
    let counters: [[u8; 16]; 4] = [
        [0u8; 16],
        [0xffu8; 16],
        [0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        [0x5a; 16],
    ];

    for len in boundary_lengths() {
        for counter in counters {
            let input = pattern(len, 37, 11);

            let mut got = input.clone();
            let mut cipher = Aes128Ctr::new(&KEY_128);
            cipher.set_counter(&counter);
            cipher.xor_keystream(&mut got);
            let mut want = input.clone();
            ctr_reference_128(&counter, &mut want);
            assert_eq!(got, want, "AES-128-CTR mismatch at len={len}");

            let mut got = input.clone();
            let mut cipher = Aes256Ctr::new(&KEY_256);
            cipher.set_counter(&counter);
            cipher.xor_keystream(&mut got);
            let mut want = input.clone();
            ctr_reference_256(&counter, &mut want);
            assert_eq!(got, want, "AES-256-CTR mismatch at len={len}");
        }
    }
}

#[test]
fn gcm_roundtrip_at_all_boundaries() {
    let nonce = [0x99u8; 12];
    let aad_lengths = [0usize, 1, 15, 16, 17, 31, 32, 33, 127, 128, 129];

    for len in boundary_lengths() {
        for aad_len in aad_lengths {
            let plaintext = pattern(len, 53, 7);
            let aad = pattern(aad_len, 29, 3);

            let cipher128 = Aes128Gcm::new(&KEY_128);
            let mut buffer = plaintext.clone();
            let tag = cipher128.encrypt_in_place(&mut buffer, &nonce, &aad);
            let mut decrypted = buffer.clone();
            cipher128
                .decrypt_in_place(&mut decrypted, &nonce, &aad, tag.as_ref())
                .expect("AES-128-GCM decrypt");
            assert_eq!(decrypted, plaintext, "AES-128-GCM mismatch at len={len} aad={aad_len}");

            let cipher256 = Aes256Gcm::new(&KEY_256);
            let mut buffer = plaintext.clone();
            let tag = cipher256.encrypt_in_place(&mut buffer, &nonce, &aad);
            let mut decrypted = buffer.clone();
            cipher256
                .decrypt_in_place(&mut decrypted, &nonce, &aad, tag.as_ref())
                .expect("AES-256-GCM decrypt");
            assert_eq!(decrypted, plaintext, "AES-256-GCM mismatch at len={len} aad={aad_len}");
        }
    }
}

#[test]
fn ctr_unaligned_slices_preserve_canaries() {
    for len in [0usize, 1, 15, 16, 17, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025] {
        let mut buffer = vec![0xa5u8; len + 32];
        let mut expected = vec![0xa5u8; len];
        let counter = [0x42u8; 16];

        let mut cipher = Aes128Ctr::new(&KEY_128);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buffer[15..15 + len]);

        ctr_reference_128(&counter, &mut expected);
        assert_eq!(&buffer[15..15 + len], &expected[..], "len={len}");
        assert!(buffer[..15].iter().all(|&b| b == 0xa5));
        assert!(buffer[15 + len..].iter().all(|&b| b == 0xa5));
    }
}
