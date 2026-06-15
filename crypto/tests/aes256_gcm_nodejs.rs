use std::process::Command;

use crypto::{Aead, aes::Aes256Gcm};

fn node_encrypt(key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], plaintext: &[u8]) -> (Vec<u8>, [u8; 16]) {
    let output = Command::new("node")
        .arg("tests/aes256_gcm_node.js")
        .arg("encrypt")
        .arg(hex::encode(key))
        .arg(hex::encode(nonce))
        .arg(hex::encode(aad))
        .arg(hex::encode(plaintext))
        .output()
        .expect("Failed to execute node");

    assert!(
        output.status.success(),
        "Node encrypt failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    let ciphertext = if lines[0].is_empty() {
        vec![]
    } else {
        hex::decode(lines[0]).unwrap()
    };
    let tag: [u8; 16] = hex::decode(lines[1]).unwrap().try_into().unwrap();
    (ciphertext, tag)
}

fn node_decrypt(key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], ciphertext: &[u8], tag: &[u8]) -> Vec<u8> {
    let mut data = ciphertext.to_vec();
    data.extend_from_slice(tag);

    let output = Command::new("node")
        .arg("tests/aes256_gcm_node.js")
        .arg("decrypt")
        .arg(hex::encode(key))
        .arg(hex::encode(nonce))
        .arg(hex::encode(aad))
        .arg(hex::encode(&data))
        .output()
        .expect("Failed to execute node");

    assert!(
        output.status.success(),
        "Node decrypt failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        vec![]
    } else {
        hex::decode(trimmed).unwrap()
    }
}

#[test]
fn aes256_gcm_rust_encrypt_node_decrypt_roundtrip() {
    let test_cases: Vec<([u8; 32], [u8; 12], Vec<u8>, Vec<u8>)> = vec![
        ([0x00; 32], [0x00; 12], vec![], vec![]),
        ([0x01; 32], [0x02; 12], b"additional data".to_vec(), b"hello world".to_vec()),
        (
            [0xab; 32],
            [0xcd; 12],
            b"authenticated".to_vec(),
            (0..256).map(|_| rand::random::<u8>()).collect(),
        ),
        (
            [0xff; 32],
            [0xaa; 12],
            vec![],
            (0..1024).map(|_| rand::random::<u8>()).collect(),
        ),
    ];

    for (key, nonce, aad, plaintext) in test_cases {
        let cipher = Aes256Gcm::new(&key);
        let mut buf = plaintext.clone();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce[..], &aad);

        let decrypted = node_decrypt(&key, &nonce, &aad, &buf, tag.as_ref());
        assert_eq!(decrypted, plaintext, "Roundtrip failed for key={}", hex::encode(key));
    }
}

#[test]
fn aes256_gcm_node_encrypt_rust_decrypt_roundtrip() {
    let test_cases: Vec<([u8; 32], [u8; 12], Vec<u8>, Vec<u8>)> = vec![
        ([0x00; 32], [0x00; 12], vec![], vec![]),
        ([0x01; 32], [0x02; 12], b"additional data".to_vec(), b"hello world".to_vec()),
        (
            [0xab; 32],
            [0xcd; 12],
            b"authenticated".to_vec(),
            (0..256).map(|_| rand::random::<u8>()).collect(),
        ),
        (
            [0xff; 32],
            [0xaa; 12],
            vec![],
            (0..1024).map(|_| rand::random::<u8>()).collect(),
        ),
    ];

    for (key, nonce, aad, plaintext) in test_cases {
        let (ciphertext, tag) = node_encrypt(&key, &nonce, &aad, &plaintext);

        let cipher = Aes256Gcm::new(&key);
        let mut buf = ciphertext.clone();
        cipher
            .decrypt_in_place(&mut buf, &nonce[..], &aad, &tag[..])
            .expect("Rust decrypt failed");

        assert_eq!(buf, plaintext, "Roundtrip failed for key={}", hex::encode(key));
    }
}

#[test]
fn aes256_gcm_bidirectional_large_payload() {
    let key = [0xde; 32];
    let nonce = [0xad; 12];
    let aad = b"large payload test";
    let plaintext: Vec<u8> = (0..10000).map(|_| rand::random::<u8>()).collect();

    let cipher = Aes256Gcm::new(&key);
    let mut buf = plaintext.clone();
    let tag = cipher.encrypt_in_place(&mut buf, &nonce[..], aad);

    let decrypted = node_decrypt(&key, &nonce, aad, &buf, tag.as_ref());
    assert_eq!(decrypted, plaintext);

    let (ciphertext, node_tag) = node_encrypt(&key, &nonce, aad, &plaintext);
    let mut buf2 = ciphertext.clone();
    cipher
        .decrypt_in_place(&mut buf2, &nonce[..], aad, &node_tag[..])
        .expect("Rust decrypt failed");
    assert_eq!(buf2, plaintext);
}

#[test]
fn aes256_gcm_bidirectional_empty_plaintext_with_aad() {
    let key = [0x55; 32];
    let nonce = [0x66; 12];
    let aad = b"only authenticated data";
    let plaintext: Vec<u8> = vec![];

    let cipher = Aes256Gcm::new(&key);
    let mut buf = plaintext.clone();
    let tag = cipher.encrypt_in_place(&mut buf, &nonce[..], aad);

    let decrypted = node_decrypt(&key, &nonce, aad, &buf, tag.as_ref());
    assert_eq!(decrypted, plaintext);

    let (ciphertext, node_tag) = node_encrypt(&key, &nonce, aad, &plaintext);
    let mut buf2 = ciphertext.clone();
    cipher
        .decrypt_in_place(&mut buf2, &nonce[..], aad, &node_tag[..])
        .expect("Rust decrypt failed");
    assert_eq!(buf2, plaintext);
}

#[test]
fn aes256_gcm_bidirectional_various_sizes() {
    let key = [0x77; 32];
    let nonce = [0x88; 12];
    let aad = b"test";

    for size in 1..512 {
        let plaintext: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();

        let cipher = Aes256Gcm::new(&key);
        let mut buf = plaintext.clone();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce[..], aad);

        let decrypted = node_decrypt(&key, &nonce, aad, &buf, tag.as_ref());
        assert_eq!(decrypted, plaintext, "Failed for size {}", size);

        let (ciphertext, node_tag) = node_encrypt(&key, &nonce, aad, &plaintext);
        let mut buf2 = ciphertext.clone();
        cipher
            .decrypt_in_place(&mut buf2, &nonce[..], aad, &node_tag[..])
            .expect(&format!("Rust decrypt failed for size {}", size));
        assert_eq!(buf2, plaintext, "Failed for size {}", size);
    }
}
