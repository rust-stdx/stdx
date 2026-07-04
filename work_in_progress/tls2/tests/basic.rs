#[cfg(test)]
mod tests {
    use crypto::{Aead as _, Hasher, aes::Aes128Gcm, hkdf, sha2::Sha256};

    #[test]
    fn aead_encrypt_decrypt_self_consistent() {
        // Use the known derived key schedule
        let zeros = [0u8; 32];
        let early = hkdf::extract::<Sha256>(Some(&zeros), &zeros);
        let empty_hash: [u8; 32] = {
            let mut h = Sha256::new();
            h.update(&[]);
            let mut out = [0u8; 32];
            out.copy_from_slice(h.sum().as_ref());
            out
        };
        let d_label = b"tls13 derived";
        let mut d_hkdf = Vec::new();
        d_hkdf.extend_from_slice(&32u16.to_be_bytes());
        d_hkdf.push(d_label.len() as u8);
        d_hkdf.extend_from_slice(d_label);
        d_hkdf.push(empty_hash.len() as u8);
        d_hkdf.extend_from_slice(&empty_hash);
        let mut derived = [0u8; 32];
        hkdf::expand::<Sha256>(&mut derived, early.as_ref(), &d_hkdf).unwrap();

        // Simulate a shared secret and transcript
        let shared = [0xab; 32];
        let hs = hkdf::extract::<Sha256>(Some(&derived), &shared);
        let transcript = [0x42; 32];

        let c_label = b"tls13 c hs traffic";
        let mut c_hkdf = Vec::new();
        c_hkdf.extend_from_slice(&32u16.to_be_bytes());
        c_hkdf.push(c_label.len() as u8);
        c_hkdf.extend_from_slice(c_label);
        c_hkdf.push(transcript.len() as u8);
        c_hkdf.extend_from_slice(&transcript);
        let mut c_hs = [0u8; 32];
        hkdf::expand::<Sha256>(&mut c_hs, hs.as_ref(), &c_hkdf).unwrap();

        let key_label = b"tls13 key";
        let mut key_hkdf = Vec::new();
        key_hkdf.extend_from_slice(&16u16.to_be_bytes());
        key_hkdf.push(key_label.len() as u8);
        key_hkdf.extend_from_slice(key_label);
        key_hkdf.push(0u8);
        let mut key = [0u8; 16];
        hkdf::expand::<Sha256>(&mut key, &c_hs, &key_hkdf).unwrap();

        let iv_label = b"tls13 iv";
        let mut iv_hkdf = Vec::new();
        iv_hkdf.extend_from_slice(&12u16.to_be_bytes());
        iv_hkdf.push(iv_label.len() as u8);
        iv_hkdf.extend_from_slice(iv_label);
        iv_hkdf.push(0u8);
        let mut iv = [0u8; 12];
        hkdf::expand::<Sha256>(&mut iv, &c_hs, &iv_hkdf).unwrap();

        // Now test AEAD encrypt/decrypt using the Aes128Gcm from the crypto crate
        let plaintext = b"This is a test message for AEAD";
        let nonce = iv; // seq=0
        let aad = [0x17u8, 0x03, 0x03, 0x00, 20]; // ApplicationData, 0x0303, len=20

        // Encrypt
        let mut buf = [0u8; 256];
        buf[..plaintext.len()].copy_from_slice(plaintext);
        let cipher = Aes128Gcm::new(&key);
        let tag = cipher.encrypt_in_place(&mut buf[..plaintext.len()], &nonce, &aad);
        let ct_len = plaintext.len();

        // Decrypt - using the crypto crate directly
        let mut dec_buf = [0u8; 256];
        dec_buf[..ct_len].copy_from_slice(&buf[..ct_len]);
        let dec_cipher = Aes128Gcm::new(&key);
        let result = dec_cipher.decrypt_in_place(&mut dec_buf[..ct_len], &nonce, &aad, tag.as_ref());
        eprintln!("Direct AES decrypt result: {:?}", result);
        assert!(result.is_ok(), "Direct AES-GCM decrypt should work");
        assert_eq!(&dec_buf[..ct_len], plaintext);

        // Now test using our provider chain
        use tls2::{CipherSuite, CryptoProvider, crypto_default_provider::DefaultCryptoProvider, key_schedule};

        let provider = DefaultCryptoProvider::new();
        let suite = CipherSuite::TlsAes128GcmSha256;

        let (aead_key, _our_iv) =
            key_schedule::derive_traffic_keys(&provider, suite, &tls2::Hash::from_slice(&c_hs)).unwrap();

        // Encrypt with our provider
        let mut enc_buf = [0u8; 256];
        enc_buf[..plaintext.len()].copy_from_slice(plaintext);
        let enc_total = provider
            .aead_encrypt(&aead_key, &nonce, &aad, &mut enc_buf[..plaintext.len() + 16], plaintext.len())
            .unwrap();

        // Decrypt with our provider
        let pt_len = provider
            .aead_decrypt(&aead_key, &nonce, &aad, &mut enc_buf[..enc_total])
            .unwrap();
        assert_eq!(pt_len, plaintext.len());
        assert_eq!(&enc_buf[..pt_len], plaintext);
        eprintln!("Full provider chain AEAD works: OK");
    }
}
