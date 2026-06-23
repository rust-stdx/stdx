use super::*;
use crate::{Aead, AeadError, Bytes, Hash};

/// Ascon-AEAD128 authenticated encryption with associated data (NIST SP 800-232).
///
/// # Parameters
///
/// - Key: 128 bits (16 bytes)
/// - Nonce: 128 bits (16 bytes)
/// - Tag: 128 bits (16 bytes)
/// - Rate: 128 bits, capacity: 192 bits
/// - Initialization/finalization rounds: 12
/// - Data processing rounds: 8
///
/// # Usage limits (NIST SP 800-232 §4.3)
///
/// - Max data per key: 2^54 bytes
/// - Nonces must be distinct per key (up to 2^8 repetitions tolerated)
/// - Max decryption failures: 2^(tag_len - 32) before key rotation
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct AsconAead128 {
    iv: u64,
    key: [u8; 16],
}

impl AsconAead128 {
    /// Creates a new Ascon-AEAD128 instance from a 16-byte key.
    pub fn new(key: &[u8; 16]) -> Self {
        AsconAead128 {
            iv: 0x0000_1000_808c_0001,
            key: *key,
        }
    }

    fn k0(&self) -> u64 {
        u64::from_le_bytes(self.key[0..8].try_into().unwrap())
    }

    fn k1(&self) -> u64 {
        u64::from_le_bytes(self.key[8..16].try_into().unwrap())
    }

    /// Initialize the state with key + nonce + IV.
    fn init_state(&self, nonce: &[u8; 16]) -> State {
        let mut state = State::init_aead(&self.key, nonce, self.iv);
        p12(&mut state);
        state.xor_word(3, self.k0());
        state.xor_word(4, self.k1());
        state
    }

    /// Process associated data.
    fn process_ad(state: &mut State, aad: &[u8]) {
        if aad.is_empty() {
            return;
        }
        let mut chunks = aad.chunks_exact(16);
        for chunk in &mut chunks {
            state.xor_rate128_bytes(chunk.try_into().unwrap());
            p8(state);
        }
        let remainder = chunks.remainder();
        let mut padded = [0u8; 16];
        padded[..remainder.len()].copy_from_slice(remainder);
        padded[remainder.len()] = 0x01;
        state.xor_rate128_bytes(&padded);
        p8(state);
    }

    /// Compute the authentication tag from the final state.
    fn compute_tag(&self, state: &State) -> [u8; 16] {
        let mut tag = state.tag_bytes();
        for i in 0..16 {
            tag[i] ^= self.key[i];
        }
        tag
    }
}

impl Aead for AsconAead128 {
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 16;

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        let nonce: &[u8; 16] = nonce.try_into().expect("nonce must be 16 bytes");
        let mut state = self.init_state(nonce);
        Self::process_ad(&mut state, aad);
        state.apply_domain_sep();

        let full_blocks = in_out.len() / 16;
        let rem_len = in_out.len() % 16;

        for i in 0..full_blocks {
            let ofs = i * 16;
            let mut pt = [0u8; 16];
            pt.copy_from_slice(&in_out[ofs..ofs + 16]);
            state.xor_rate128_bytes(&pt);
            state.read_rate_bytes(&mut in_out[ofs..ofs + 16]);
            p8(&mut state);
        }

        if rem_len > 0 {
            let ofs = full_blocks * 16;
            let mut pt = [0u8; 16];
            pt[..rem_len].copy_from_slice(&in_out[ofs..ofs + rem_len]);
            state.xor_partial_rate(&pt[..rem_len]);
            state.read_rate_bytes(&mut in_out[ofs..ofs + rem_len]);
        }
        state.apply_aead_pad(rem_len);

        state.xor_word(2, self.k0());
        state.xor_word(3, self.k1());
        p12(&mut state);

        let tag_bytes = self.compute_tag(&state);
        let mut tag = Hash(Bytes::<64>::with_length(16));
        tag.as_mut().copy_from_slice(&tag_bytes);
        tag
    }

    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        if tag.len() != Self::TAG_SIZE {
            return Err(AeadError::InvalidCiphertext);
        }
        let nonce: &[u8; 16] = nonce.try_into().map_err(|_| AeadError::InvalidNonce)?;

        let mut state = self.init_state(nonce);
        Self::process_ad(&mut state, aad);
        state.apply_domain_sep();

        let full_blocks = in_out.len() / 16;
        let rem_len = in_out.len() % 16;

        for i in 0..full_blocks {
            let ofs = i * 16;
            let mut ct = [0u8; 16];
            ct.copy_from_slice(&in_out[ofs..ofs + 16]);

            let mut rate = [0u8; 16];
            state.read_rate_bytes(&mut rate);
            for j in 0..16 {
                in_out[ofs + j] = rate[j] ^ ct[j];
            }
            state.write_rate_bytes(&ct);
            p8(&mut state);
        }

        if rem_len > 0 {
            let ofs = full_blocks * 16;
            let mut ct = [0u8; 16];
            ct[..rem_len].copy_from_slice(&in_out[ofs..ofs + rem_len]);

            let mut rate = [0u8; 16];
            state.read_rate_bytes(&mut rate);
            for j in 0..rem_len {
                in_out[ofs + j] = rate[j] ^ ct[j];
            }
            state.apply_aead_pad(rem_len);
            state.write_rate_bytes(&ct[..rem_len]);
        } else {
            state.apply_aead_pad(0);
        }

        state.xor_word(2, self.k0());
        state.xor_word(3, self.k1());
        p12(&mut state);

        let computed = self.compute_tag(&state);

        if !constant_time_eq::constant_time_eq(&computed, tag) {
            in_out.fill(0);
            return Err(AeadError::InvalidCiphertext);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Aead;

    static KEY: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    ];
    static NONCE: [u8; 16] = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    ];

    #[test]
    fn empty_pt_empty_ad() {
        let ae = AsconAead128::new(&KEY);
        let mut pt = vec![];
        let tag = ae.encrypt_in_place(&mut pt, &NONCE, &[]);
        let expected_ct = hex::decode("").unwrap();
        let expected_tag = hex::decode("4F9C278211BEC9316BF68F46EE8B2EC6").unwrap();
        assert_eq!(pt, expected_ct);
        assert_eq!(tag.as_ref(), expected_tag.as_slice());
    }

    #[test]
    fn one_byte_pt_empty_ad() {
        let ae = AsconAead128::new(&KEY);
        let mut pt = hex::decode("20").unwrap();
        let tag = ae.encrypt_in_place(&mut pt, &NONCE, &[]);
        // CT line: E8DD576ABA1CD3E6FC704DE02AEDB79588 (34 hex chars = 17 bytes)
        // ciphertext = first byte = E8, tag = remaining 16 = DD576ABA1CD3E6FC704DE02AEDB79588
        let expected_ct = hex::decode("E8").unwrap();
        let expected_tag = hex::decode("DD576ABA1CD3E6FC704DE02AEDB79588").unwrap();
        assert_eq!(pt, expected_ct);
        assert_eq!(tag.as_ref(), expected_tag.as_slice());
    }

    #[test]
    fn one_byte_pt_one_byte_ad() {
        let ae = AsconAead128::new(&KEY);
        let mut pt = hex::decode("20").unwrap();
        let aad = hex::decode("30").unwrap();
        let tag = ae.encrypt_in_place(&mut pt, &NONCE, &aad);
        // ciphertext=96, tag=2B8016836C75A7D86866588CA245D886
        let expected_ct = hex::decode("96").unwrap();
        let expected_tag = hex::decode("2B8016836C75A7D86866588CA245D886").unwrap();
        assert_eq!(pt, expected_ct);
        assert_eq!(tag.as_ref(), expected_tag.as_slice());
    }

    #[test]
    fn two_byte_pt_six_byte_ad() {
        let ae = AsconAead128::new(&KEY);
        let mut pt = hex::decode("2021").unwrap();
        let aad = hex::decode("303132333435").unwrap();
        let tag = ae.encrypt_in_place(&mut pt, &NONCE, &aad);
        // ciphertext=9310, tag=6848C186CA92DCC20741A92F7AAFE673
        let expected_ct = hex::decode("9310").unwrap();
        let expected_tag = hex::decode("6848C186CA92DCC20741A92F7AAFE673").unwrap();
        assert_eq!(pt, expected_ct);
        assert_eq!(tag.as_ref(), expected_tag.as_slice());
    }

    #[test]
    fn thirtytwo_byte_pt_twentysix_byte_ad() {
        let ae = AsconAead128::new(&KEY);
        let mut pt = hex::decode("202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F").unwrap();
        let aad = hex::decode("303132333435363738393A3B3C3D3E3F40414243444546474849").unwrap();
        let tag = ae.encrypt_in_place(&mut pt, &NONCE, &aad);
        // ciphertext=32 bytes, tag=16 bytes
        let expected_ct = hex::decode("A92EF70DF2EF0FAA74A21F9739FB89237DF62F9A2B4080B850046DDD386DED48").unwrap();
        let expected_tag = hex::decode("E7833CF56F755945AEB70D2BAAAA361C").unwrap();
        assert_eq!(pt, expected_ct);
        assert_eq!(tag.as_ref(), expected_tag.as_slice());
    }

    #[test]
    fn roundtrip() {
        let key = [0x55u8; 16];
        let nonce = [0xAAu8; 16];
        let aad = b"associated data";
        let plaintext = b"hello, world!";

        let ae = AsconAead128::new(&key);
        let mut ct = plaintext.to_vec();
        let tag = ae.encrypt_in_place(&mut ct, &nonce, aad);

        let mut decrypted = ct.clone();
        ae.decrypt_in_place(&mut decrypted, &nonce, aad, tag.as_ref()).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn roundtrip_empty() {
        let key = [0x55u8; 16];
        let nonce = [0xAAu8; 16];

        let ae = AsconAead128::new(&key);
        let mut ct = vec![];
        let tag = ae.encrypt_in_place(&mut ct, &nonce, b"ad");

        let mut decrypted = ct.clone();
        ae.decrypt_in_place(&mut decrypted, &nonce, b"ad", tag.as_ref())
            .unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn tampered_tag_fails() {
        let key = [0x55u8; 16];
        let nonce = [0xAAu8; 16];

        let ae = AsconAead128::new(&key);
        let mut ct = b"secret".to_vec();
        let mut tag = ae.encrypt_in_place(&mut ct, &nonce, b"");
        tag.as_mut()[0] ^= 1;

        let mut decrypted = ct.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &nonce, b"", tag.as_ref());
        assert!(result.is_err());
        assert!(decrypted.iter().all(|b| *b == 0));
    }

    #[test]
    fn tampered_ciphertext_fails() {
        let key = [0x55u8; 16];
        let nonce = [0xAAu8; 16];

        let ae = AsconAead128::new(&key);
        let mut ct = b"secret".to_vec();
        let tag = ae.encrypt_in_place(&mut ct, &nonce, b"");
        ct[3] ^= 1;

        let mut decrypted = ct.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &nonce, b"", tag.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn wrong_nonce_fails() {
        let key = [0x55u8; 16];

        let ae = AsconAead128::new(&key);
        let mut ct = b"secret".to_vec();
        let tag = ae.encrypt_in_place(&mut ct, &[0u8; 16], b"");
        let mut decrypted = ct.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &[1u8; 16], b"", tag.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn wrong_ad_fails() {
        let key = [0x55u8; 16];
        let nonce = [0xAAu8; 16];

        let ae = AsconAead128::new(&key);
        let mut ct = b"secret".to_vec();
        let tag = ae.encrypt_in_place(&mut ct, &nonce, b"correct ad");
        let mut decrypted = ct.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &nonce, b"wrong ad", tag.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn wrong_key_fails() {
        let nonce = [0xAAu8; 16];

        let ae = AsconAead128::new(&[0x55u8; 16]);
        let mut ct = b"secret".to_vec();
        let tag = ae.encrypt_in_place(&mut ct, &nonce, b"");
        let mut decrypted = ct.clone();
        let ae2 = AsconAead128::new(&[0xAAu8; 16]);
        let result = ae2.decrypt_in_place(&mut decrypted, &nonce, b"", tag.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn empty_ad_with_data() {
        let key = [0x55u8; 16];
        let nonce = [0xAAu8; 16];

        let ae = AsconAead128::new(&key);
        let mut ct = b"data with empty AD".to_vec();
        let tag = ae.encrypt_in_place(&mut ct, &nonce, b"");

        let mut decrypted = ct.clone();
        ae.decrypt_in_place(&mut decrypted, &nonce, b"", tag.as_ref()).unwrap();
        assert_eq!(decrypted, b"data with empty AD");
    }

    #[test]
    fn kat_vectors() {
        let data = include_str!("../../testdata/ascon/LWC_AEAD_KAT_128_128.txt");
        let mut count = 0u64;
        let mut key = None;
        let mut nonce = None;
        let mut pt_hex = String::new();
        let mut ad_hex = String::new();
        let mut ct_hex = String::new();

        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() {
                if let (Some(k), Some(n)) = (&key, &nonce) {
                    let key_bytes: [u8; 16] = hex::decode(k).unwrap().try_into().unwrap();
                    let nonce_bytes: [u8; 16] = hex::decode(n).unwrap().try_into().unwrap();
                    let pt = hex::decode(&pt_hex).unwrap();
                    let ct_bytes = hex::decode(&ct_hex).unwrap();
                    let aad = hex::decode(&ad_hex).unwrap();

                    let expected_ct = &ct_bytes[..pt.len()];
                    let expected_tag = &ct_bytes[pt.len()..];

                    let ae = AsconAead128::new(&key_bytes);
                    let mut enc = pt.clone();
                    let tag = ae.encrypt_in_place(&mut enc, &nonce_bytes, &aad);
                    assert_eq!(enc, expected_ct, "KAT AEAD Count={count} ct mismatch");
                    assert_eq!(tag.as_ref(), expected_tag, "KAT AEAD Count={count} tag mismatch");

                    let mut dec = expected_ct.to_vec();
                    ae.decrypt_in_place(&mut dec, &nonce_bytes, &aad, expected_tag)
                        .unwrap_or_else(|e| panic!("KAT AEAD Count={count} decrypt failed: {e:?}"));
                    assert_eq!(dec, pt, "KAT AEAD Count={count} decrypt pt mismatch");
                }
                // Reset
                key = None;
                nonce = None;
                pt_hex.clear();
                ad_hex.clear();
                ct_hex.clear();
                continue;
            }

            if line.starts_with("Count = ") {
                count = line["Count = ".len()..].parse().unwrap();
                continue;
            }
            if line.starts_with("Key = ") {
                key = Some(line[6..].to_string());
                continue;
            }
            if line.starts_with("Nonce = ") {
                nonce = Some(line[8..].to_string());
                continue;
            }
            if line.starts_with("PT = ") {
                pt_hex = line[5..].to_string();
                continue;
            }
            if line.starts_with("AD = ") {
                ad_hex = line[5..].to_string();
                continue;
            }
            if line.starts_with("CT = ") {
                ct_hex = line[5..].to_string();
                continue;
            }
        }

        // Process last entry
        if let (Some(k), Some(n)) = (&key, &nonce) {
            let key_bytes: [u8; 16] = hex::decode(k).unwrap().try_into().unwrap();
            let nonce_bytes: [u8; 16] = hex::decode(n).unwrap().try_into().unwrap();
            let pt = hex::decode(&pt_hex).unwrap();
            let ct_bytes = hex::decode(&ct_hex).unwrap();
            let aad = hex::decode(&ad_hex).unwrap();

            let expected_ct = &ct_bytes[..pt.len()];
            let expected_tag = &ct_bytes[pt.len()..];

            let ae = AsconAead128::new(&key_bytes);
            let mut enc = pt.clone();
            let tag = ae.encrypt_in_place(&mut enc, &nonce_bytes, &aad);
            assert_eq!(enc, expected_ct, "KAT AEAD Count={count} ct mismatch");
            assert_eq!(tag.as_ref(), expected_tag, "KAT AEAD Count={count} tag mismatch");

            let mut dec = expected_ct.to_vec();
            ae.decrypt_in_place(&mut dec, &nonce_bytes, &aad, expected_tag)
                .unwrap_or_else(|e| panic!("KAT AEAD Count={count} decrypt failed: {e:?}"));
            assert_eq!(dec, pt, "KAT AEAD Count={count} decrypt pt mismatch");
        }
    }
}
