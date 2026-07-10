use super::shake256::{CShake256, left_encode, right_encode};
use crate::Xof;

const KMAC256_RATE: usize = 136;

/// KMAC256 (cSHAKE256-based MAC) as defined in SP 800-185.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::sha3::Kmac256;
///
/// let key = b"super secret key 32 bytes!";
/// let mut tag = [0u8; 64];
/// Kmac256::mac(key, b"message", b"customization", &mut tag);
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::sha3::Kmac256;
///
/// let key = b"super secret key 32 bytes!";
/// let mut kmac = Kmac256::new(key, b"customization");
/// kmac.update(b"hello ");
/// kmac.update(b"world");
/// let mut tag = [0u8; 64];
/// kmac.finalize_into(&mut tag);
/// ```
#[derive(Clone)]
pub struct Kmac256 {
    cshake: CShake256,
}

impl Kmac256 {
    #[inline]
    pub fn new(key: &[u8], customization: &[u8]) -> Self {
        let mut cshake = CShake256::new(b"KMAC", customization);

        // absorb bytepad(encode_string(key), KMAC256_RATE)

        // bytepad(encode_string(key), w)
        let enc_w = left_encode(KMAC256_RATE);
        cshake.absorb(enc_w.as_ref());

        let enc_key = left_encode(key.len() * 8);
        cshake.absorb(enc_key.as_ref());
        cshake.absorb(key);

        let total = enc_w.len() + enc_key.len() + key.len();
        let pad = (KMAC256_RATE - (total % KMAC256_RATE)) % KMAC256_RATE;
        if pad > 0 {
            let zeros = [0u8; KMAC256_RATE];
            cshake.absorb(&zeros[..pad]);
        }

        return Kmac256 {
            cshake,
        };
    }

    #[inline]
    pub fn update(&mut self, data: &[u8]) {
        self.cshake.absorb(data);
    }

    #[inline]
    pub fn finalize_into(mut self, output: &mut [u8]) {
        let output_bits = output.len().checked_mul(8).expect("output size too large for KMAC");
        let encoded_output_len = right_encode(output_bits);
        self.cshake.absorb(encoded_output_len.as_ref());
        self.cshake.squeeze(output);
    }

    #[inline]
    pub fn mac(key: &[u8], data: &[u8], customization: &[u8], output: &mut [u8]) {
        let mut kmac = Kmac256::new(key, customization);
        kmac.update(data);
        kmac.finalize_into(output);
    }
}

#[cfg(test)]
mod tests {
    use hex;

    use super::Kmac256;

    const KEY: [u8; 32] = [
        0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51,
        0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
    ];
    const TAGGED_APP: &[u8] = b"My Tagged Application";

    // NIST SP 800-185 KMAC_samples.pdf sample #4
    const KMAC256_SAMPLE_4: &str = "20c570c31346f703c9ac36c61c03cb64c3970d0cfc787e9b79599d273a68d2f7f69d4cc3de9d104a351689f27cf6f5951f0103f33f4f24871024d9c27773a8dd";
    // NIST SP 800-185 KMAC_samples.pdf sample #5
    const KMAC256_SAMPLE_5: &str = "75358cf39e41494e949707927cee0af20a3ff553904c86b08f21cc414bcfd691589d27cf5e15369cbbff8b9a4c2eb17800855d0235ff635da82533ec6b759b69";
    // NIST SP 800-185 KMAC_samples.pdf sample #6 (streaming-friendly)
    const KMAC256_SAMPLE_6: &str = "b58618f71f92e1d56c1b8c55ddd7cd188b97b4ca4d99831eb2699a837da2e4d970fbacfde50033aea585f1a2708510c32d07880801bd182898fe476876fc8965";

    #[test]
    fn kmac256_nist_sample_4() {
        let mut out = [0u8; 64];
        Kmac256::mac(&KEY, &[0x00, 0x01, 0x02, 0x03], TAGGED_APP, &mut out);
        assert_eq!(hex::encode(out), KMAC256_SAMPLE_4);
    }

    #[test]
    fn kmac256_nist_sample_5() {
        let input: Vec<u8> = (0u8..200).collect();
        let mut out = [0u8; 64];
        Kmac256::mac(&KEY, &input, b"", &mut out);
        assert_eq!(hex::encode(out), KMAC256_SAMPLE_5);
    }

    #[test]
    fn kmac256_nist_sample_6_incremental() {
        let input: Vec<u8> = (0u8..200).collect();
        let mut kmac = Kmac256::new(&KEY, TAGGED_APP);
        for chunk in input.chunks(1) {
            kmac.update(chunk);
        }
        let mut out = [0u8; 64];
        kmac.finalize_into(&mut out);
        assert_eq!(hex::encode(out), KMAC256_SAMPLE_6);
    }

    #[test]
    fn kmac256_incremental_matches_one_shot() {
        let input: Vec<u8> = (0u8..200).collect();

        let mut one_shot = [0u8; 64];
        Kmac256::mac(&KEY, &input, TAGGED_APP, &mut one_shot);

        let mut kmac = Kmac256::new(&KEY, TAGGED_APP);
        for chunk in input.chunks(7) {
            kmac.update(chunk);
        }
        let mut incremental = [0u8; 64];
        kmac.finalize_into(&mut incremental);
        assert_eq!(incremental, one_shot);
    }

    #[test]
    fn wycheproof_kmac256() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/kmac256_no_customization_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let key_hex = test["key"].as_str().unwrap();
                let msg_hex = test["msg"].as_str().unwrap();
                let tag_hex = test["tag"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let key = hex::decode(key_hex).unwrap();
                let msg = hex::decode(msg_hex).unwrap();
                let expected_tag = hex::decode(tag_hex).unwrap();

                let mut out = vec![0u8; expected_tag.len()];
                Kmac256::mac(&key, &msg, b"", &mut out);

                if result == "valid" {
                    assert_eq!(
                        out.as_slice(),
                        expected_tag.as_slice(),
                        "wycheproof KMAC256 tcId={}",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else if result == "acceptable" {
                    if out.as_slice() != expected_tag.as_slice() {
                        continue;
                    }
                    valid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid KMAC256 wycheproof tests were run");
    }
}
