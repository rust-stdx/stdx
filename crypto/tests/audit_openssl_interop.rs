//! Cross-check Poly1305 and ChaCha20 against the OpenSSL 3 command-line
//! implementation (an entirely separate codebase), including the saturated
//! messages that triggered RUSTSEC-2026-0337 in `ic-cipher`.
//!
//! The test skips cleanly when `openssl` is unavailable or was built without
//! the POLY1305 MAC. It is deterministic: keys and messages come from a seeded
//! `ChaChaRng`.

use std::{
    io::Write,
    process::{Command, Stdio},
};

use crypto::{StreamCipher, chacha::ChaCha20Ietf, poly1305::Poly1305};
use random::{RngExt, SeedableRng, rngs::ChaChaRng};

fn run_with_stdin(program: &str, args: &[String], input: &[u8]) -> Option<Vec<u8>> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    child.stdin.take()?.write_all(input).ok()?;
    let out = child.wait_with_output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(out.stdout)
}

fn openssl_poly1305(key: &[u8; 32], msg: &[u8]) -> Option<[u8; 16]> {
    let args = vec![
        "mac".to_string(),
        "-macopt".to_string(),
        format!("hexkey:{}", hex::encode(key)),
        "POLY1305".to_string(),
    ];
    let out = run_with_stdin("openssl", &args, msg)?;
    let text = core::str::from_utf8(&out).ok()?.trim();
    let bytes = hex::decode(text).ok()?;
    bytes.try_into().ok()
}

fn openssl_chacha20(key: &[u8; 32], nonce: &[u8; 12], counter: u32, input: &[u8]) -> Option<Vec<u8>> {
    // OpenSSL's raw ChaCha20 EVP takes a 16-byte IV: little-endian 32-bit block
    // counter followed by the 96-bit nonce.
    let mut iv = [0u8; 16];
    iv[..4].copy_from_slice(&counter.to_le_bytes());
    iv[4..].copy_from_slice(nonce);
    let args = vec![
        "enc".to_string(),
        "-chacha20".to_string(),
        "-K".to_string(),
        hex::encode(key),
        "-iv".to_string(),
        hex::encode(iv),
        "-nopad".to_string(),
    ];
    run_with_stdin("openssl", &args, input)
}

#[test]
fn poly1305_matches_openssl() {
    let mut rng = ChaChaRng::seed_from_u64(0x0f_15_1a_2b_3c_4d_5e_6f);

    // Probe availability once; skip the whole test if this fails.
    if openssl_poly1305(&[0u8; 32], b"").is_none() {
        eprintln!("skipping: openssl with POLY1305 unavailable");
        return;
    }

    let mut cases: Vec<usize> = (0..=40).collect();
    cases.extend([
        47, 48, 49, 63, 64, 65, 79, 80, 81, 95, 96, 97, 111, 112, 113, 127, 128, 129,
    ]);
    cases.extend([191, 192, 193, 255, 256, 257, 383, 384, 385, 511, 512, 513, 600]);

    for &len in &cases {
        let mut key = [0u8; 32];
        rng.fill(&mut key);
        let mut msg = vec![0u8; len];
        rng.fill(&mut msg);
        let want = openssl_poly1305(&key, &msg).expect("openssl poly1305");
        let got = Poly1305::mac(&key, &msg);
        assert_eq!(got, want, "openssl mismatch len={len} key={}", hex::encode(key));
    }

    // Saturated messages at and beyond the four-block/64-byte threshold that
    // caused the ic-cipher overflow, plus the exact 16-byte multiple edges.
    for &len in &[
        16usize, 17, 63, 64, 65, 127, 128, 129, 255, 256, 257, 512, 513, 1024, 1025,
    ] {
        for fill in [0xffu8, 0xaau8, 0x00u8] {
            let mut key = [0xffu8; 32];
            rng.fill(&mut key);
            // Force the saturated key shape as well as random `s`.
            for b in &mut key[..16] {
                *b |= 0xc0;
            }
            let msg = vec![fill; len];
            let want = openssl_poly1305(&key, &msg).expect("openssl poly1305 saturated");
            assert_eq!(
                Poly1305::mac(&key, &msg),
                want,
                "openssl saturated mismatch fill={fill:#x} len={len}"
            );
        }
    }
}

#[test]
fn chacha20_keystream_matches_openssl() {
    let mut rng = ChaChaRng::seed_from_u64(0xcc_cc_20_00_00_00_00_01);

    // Probe availability against the RFC 8439 A.2 #1 vector. This also skips
    // targets where spawning `openssl` is unavailable, such as WebAssembly.
    let Some(probe) = openssl_chacha20(&[0u8; 32], &[0u8; 12], 0, &[0u8; 64]) else {
        eprintln!("skipping: openssl chacha20 unavailable");
        return;
    };
    assert_eq!(
        hex::encode(probe),
        "76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7\
         da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586"
    );

    for &len in &[
        0usize, 1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 191, 192, 193, 255, 256, 257, 511, 512, 513, 777, 1024, 1025,
        1400,
    ] {
        let mut key = [0u8; 32];
        let mut nonce = [0u8; 12];
        rng.fill(&mut key);
        rng.fill(&mut nonce);
        let zeros = vec![0u8; len];
        let want = openssl_chacha20(&key, &nonce, 0, &zeros).expect("openssl chacha20");
        let mut got = vec![0u8; len];
        ChaCha20Ietf::new(&key, &nonce).xor_keystream(&mut got).unwrap();
        assert_eq!(got, want, "openssl chacha20 mismatch len={len}");

        // Counter 1 is the AEAD payload counter.
        let want = openssl_chacha20(&key, &nonce, 1, &zeros).expect("openssl chacha20 counter 1");
        let mut got = vec![0u8; len];
        let mut c = ChaCha20Ietf::new(&key, &nonce);
        c.set_counter(1);
        c.xor_keystream(&mut got).unwrap();
        assert_eq!(got, want, "openssl chacha20 counter 1 mismatch len={len}");
    }
}
