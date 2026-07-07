use super::keccak::KeccakSponge;
use crate::{Hash, Hasher, Xof, bytes::Bytes};

pub(crate) const SHAKE256_RATE: usize = 136;
const CSHAKE256_DOMAIN_SEPARATOR: u8 = 0x04;
const SHAKE256_DOMAIN_SEPARATOR: u8 = 0x1f;

// fixed-size buffer to avoid allocations when encoding inputs
type EncodedBytes = Bytes<9>;

/// SHAKE256 extensible-output function (XOF) as defined in FIPS 202.
///
/// Implements both the [`Xof`] and [`Hasher`] traits.
///
/// # One-shot API
///
/// ```ignore
/// use crypto{Hasher, sha3::Shake256};
///
/// let mut output = [0u8; 64];
/// Shake256::hash(b"hello world", &mut output);
/// ```
///
/// # Incremental XOF API
///
/// ```ignore
/// use crypto::{sha3::Shake256, Xof};
///
/// let mut shake = Shake256::new();
/// shake.absorb(b"hello ");
/// shake.absorb(b"world");
/// let mut out = [0u8; 64];
/// shake.squeeze(&mut out);
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Shake256 {
    keccak: KeccakSponge<24>,
}

impl Shake256 {
    #[inline]
    pub fn new() -> Self {
        return Shake256 {
            keccak: KeccakSponge::new(SHAKE256_RATE, SHAKE256_DOMAIN_SEPARATOR),
        };
    }

    #[inline]
    pub fn hash(data: &[u8], output: &mut [u8]) {
        let mut hasher = Shake256::new();
        hasher.absorb(data);
        hasher.squeeze(output);
    }
}

impl Xof for Shake256 {
    #[inline]
    fn absorb(&mut self, data: &[u8]) {
        self.keccak.absorb(data);
    }

    #[inline]
    fn squeeze(&mut self, out: &mut [u8]) {
        self.keccak.squeeze(out);
    }
}

impl Hasher for Shake256 {
    const BLOCK_SIZE: usize = SHAKE256_RATE;
    const OUTPUT_SIZE: usize = 64;

    #[inline]
    fn new() -> Self {
        return Shake256::new();
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.absorb(data);
    }

    #[inline]
    fn sum(mut self) -> Hash {
        let mut hash = Bytes::<64>::with_length(Self::OUTPUT_SIZE);
        self.squeeze(hash.as_mut());
        return Hash(hash);
    }
}

/// cSHAKE256 (customizable SHAKE256) as defined in SP 800-185.
///
/// Allows domain separation via a function name and customization string.
/// When both parameters are empty, `CShake256` behaves identically to [`Shake256`].
///
/// Implements the [`Xof`] trait.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::sha3::CShake256;
///
/// let mut output = [0u8; 64];
/// CShake256::hash(b"message", b"FUNCTION", b"customization", &mut output);
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{sha3::CShake256, Xof};
///
/// let mut cshake = CShake256::new(b"FUNCTION", b"customization");
/// cshake.absorb(b"hello ");
/// cshake.absorb(b"world");
/// let mut out = [0u8; 64];
/// cshake.squeeze(&mut out);
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct CShake256 {
    keccak: KeccakSponge<24>,
}

impl CShake256 {
    #[inline]
    pub fn hash(data: &[u8], function_name: &[u8], customization: &[u8], output: &mut [u8]) {
        let mut xof = CShake256::new(function_name, customization);
        xof.absorb(data);
        xof.squeeze(output);
    }

    #[inline]
    pub fn new(function_name: &[u8], customization: &[u8]) -> Self {
        if function_name.is_empty() && customization.is_empty() {
            return CShake256 {
                keccak: KeccakSponge::new(SHAKE256_RATE, SHAKE256_DOMAIN_SEPARATOR),
            };
        }

        let mut keccak = KeccakSponge::new(SHAKE256_RATE, CSHAKE256_DOMAIN_SEPARATOR);

        // absorb bytepad(encode_string(N) || encode_string(S), w)

        // bytepad: left_encode(w)
        let enc_w = left_encode(SHAKE256_RATE);
        keccak.absorb(enc_w.as_ref());

        // encode_string(N): left_encode(bitlen(N)) || N
        let enc_n = left_encode(function_name.len() * 8);
        keccak.absorb(enc_n.as_ref());
        keccak.absorb(function_name);

        // encode_string(S): left_encode(bitlen(S)) || S
        let enc_s = left_encode(customization.len() * 8);
        keccak.absorb(enc_s.as_ref());
        keccak.absorb(customization);

        // bytepad: zero-pad to block boundary
        let total = enc_w.len() + enc_n.len() + function_name.len() + enc_s.len() + customization.len();
        let pad = (SHAKE256_RATE - (total % SHAKE256_RATE)) % SHAKE256_RATE;
        if pad > 0 {
            let zeros = [0u8; SHAKE256_RATE];
            keccak.absorb(&zeros[..pad]);
        }

        return CShake256 {
            keccak,
        };
    }
}

impl Xof for CShake256 {
    #[inline]
    fn absorb(&mut self, data: &[u8]) {
        self.keccak.absorb(data);
    }

    #[inline]
    fn squeeze(&mut self, out: &mut [u8]) {
        self.keccak.squeeze(out);
    }
}

// SP 800-185 encoding helpers

#[inline]
pub(crate) fn left_encode(x: usize) -> EncodedBytes {
    let bytes = x.to_be_bytes();
    let first_non_zero = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len() - 1);
    let n = bytes.len() - first_non_zero;

    let mut out = Bytes::new();
    out.push(n as u8);
    out.append(&bytes[first_non_zero..]);
    return out;
}

#[inline]
pub(crate) fn right_encode(x: usize) -> EncodedBytes {
    let mut bytes = left_encode(x);
    let out = bytes.as_mut();
    let n = out[0];
    out[0] = out[1];
    for i in 1..(n as usize) {
        out[i] = out[i + 1];
    }
    out[n as usize] = n;
    return bytes;
}

// #[inline]
// pub(crate) fn encode_string(s: &[u8]) -> Vec<u8> {
//     let encoded = left_encode(s.len() * 8);
//     let mut out = Vec::with_capacity(s.len() + encoded.len());
//     out.extend_from_slice(encoded.as_ref());
//     out.extend_from_slice(s);
//     return out;
// }

// #[inline]
// pub(crate) fn bytepad(x: &[u8], w: usize) -> Vec<u8> {
//     let encoded = left_encode(w);
//     // the length of left_encode(w) || X
//     let wx_length = encoded.len() + x.len();
//     let pad_length = (w - (wx_length % w)) % w;

//     let mut out = Vec::with_capacity(wx_length + pad_length);
//     out.extend_from_slice(encoded.as_ref());
//     out.extend_from_slice(x);
//     out.resize(out.len() + pad_length, 0);
//     return out;
// }

#[cfg(test)]
mod tests {
    use super::{CShake256, Shake256};
    use crate::{Hasher, Xof};

    // ── Shake256 vectors ──────────────────────────────────────────────────────

    fn vectors_shake256() -> Vec<(Vec<u8>, usize, &'static str)> {
        vec![
            (
                b"".to_vec(),
                64,
                "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be",
            ),
            (
                b"".to_vec(),
                128,
                "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be141e96616fb13957692cc7edd0b45ae3dc07223c8e92937bef84bc0eab862853349ec75546f58fb7c2775c38462c5010d846c185c15111e595522a6bcd16cf86",
            ),
            (
                b"abc".to_vec(),
                64,
                "483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739d5a15bef186a5386c75744c0527e1faa9f8726e462a12a4feb06bd8801e751e4",
            ),
            (
                b"hello world".to_vec(),
                64,
                "369771bb2cb9d2b04c1d54cca487e372d9f187f73f7ba3f65b95c8ee7798c527f4f3c2d55c2d46a29f2e945d469c3df27853a8735271f5cc2d9e889544357116",
            ),
            (
                b"The quick brown fox jumps over the lazy dog".to_vec(),
                64,
                "2f671343d9b2e1604dc9dcf0753e5fe15c7c64a0d283cbbf722d411a0e36f6ca1d01d1369a23539cd80f7c054b6e5daf9c962cad5b8ed5bd11998b40d5734442",
            ),
            (
                b"The quick brown fox jumps over the lazy dog.".to_vec(),
                64,
                "bd225bfc8b255f3036f0c8866010ed0053b5163a3cae111e723c0c8e704eca4e5d0f1e2a2fa18c8a219de6b88d5917ff5dd75b5fb345e7409a3b333b508a65fb",
            ),
            (
                vec![b'a'; 1_000_000],
                64,
                "3578a7a4ca9137569cdf76ed617d31bb994fca9c1bbf8b184013de8234dfd13a3fd124d4df76c0a539ee7dd2f6e1ec346124c815d9410e145eb561bcd97b18ab",
            ),
        ]
    }

    #[test]
    fn known_vectors() {
        for (input, output_len, expected) in vectors_shake256() {
            let mut output = vec![0u8; output_len];
            Shake256::hash(&input, &mut output);
            assert_eq!(hex::encode(output), expected);
        }
    }

    #[test]
    fn incremental_and_streaming_read() {
        let mut one_shot = vec![0u8; 128];
        Shake256::hash(b"", &mut one_shot);

        let mut shake = Shake256::new();
        shake.absorb(b"");
        let mut first = [0u8; 64];
        let mut second = [0u8; 64];
        shake.squeeze(&mut first);
        shake.squeeze(&mut second);

        let mut combined = vec![0u8; 128];
        combined[..64].copy_from_slice(&first);
        combined[64..].copy_from_slice(&second);

        assert_eq!(combined, one_shot);
    }

    #[test]
    fn hasher_trait_impl() {
        let expected = "369771bb2cb9d2b04c1d54cca487e372d9f187f73f7ba3f65b95c8ee7798c527f4f3c2d55c2d46a29f2e945d469c3df27853a8735271f5cc2d9e889544357116";
        let digest = <Shake256 as Hasher>::hash(b"hello world");
        assert_eq!(hex::encode(digest.as_ref()), expected);
    }

    #[test]
    fn xof_trait_impl() {
        let mut xof = Shake256::new();
        xof.absorb(b"abc");
        let mut out = [0u8; 64];
        xof.squeeze(&mut out);
        assert_eq!(
            hex::encode(out),
            "483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739d5a15bef186a5386c75744c0527e1faa9f8726e462a12a4feb06bd8801e751e4"
        );
    }

    // ── CShake256 vectors (NIST SP 800-185) ──────────────────────────────────

    const EMAIL_SIGNATURE: &[u8] = b"Email Signature";
    const SAMPLE_3_EXPECTED: &str = "d008828e2b80ac9d2218ffee1d070c48b8e4c87bff32c9699d5b6896eee0edd164020e2be0560858d9c00c037e34a96937c561a74c412bb4c746469527281c8c";
    const SAMPLE_4_EXPECTED: &str = "07dc27b11e51fbac75bc7b3c1d983e8b4b85fb1defaf218912ac86430273091727f42b17ed1df63e8ec118f04b23633c1dfb1574c8fb55cb45da8e25afb092bb";

    #[test]
    fn cshake256_nist_sample_3() {
        let mut out = [0u8; 64];
        CShake256::hash(&[0x00, 0x01, 0x02, 0x03], b"", EMAIL_SIGNATURE, &mut out);
        assert_eq!(hex::encode(out), SAMPLE_3_EXPECTED);
    }

    #[test]
    fn cshake256_nist_sample_4() {
        let input: Vec<u8> = (0u8..200).collect();
        let mut out = [0u8; 64];
        CShake256::hash(&input, b"", EMAIL_SIGNATURE, &mut out);
        assert_eq!(hex::encode(out), SAMPLE_4_EXPECTED);
    }

    #[test]
    fn cshake256_incremental_matches_one_shot() {
        let input: Vec<u8> = (0u8..200).collect();
        let mut one_shot = [0u8; 64];
        CShake256::hash(&input, b"", EMAIL_SIGNATURE, &mut one_shot);

        let mut cshake = CShake256::new(b"", EMAIL_SIGNATURE);
        for chunk in input.chunks(9) {
            cshake.absorb(chunk);
        }
        let mut streamed = [0u8; 64];
        cshake.squeeze(&mut streamed);
        assert_eq!(streamed, one_shot);
    }

    #[test]
    fn cshake256_empty_params_passes_shake256_vectors() {
        for (input, output_len, expected) in vectors_shake256() {
            let mut cshake_out = vec![0u8; output_len];
            CShake256::hash(&input, b"", b"", &mut cshake_out);
            assert_eq!(hex::encode(cshake_out), expected);
        }
    }

    #[test]
    fn cshake256_xof_trait_impl() {
        let mut xof = CShake256::new(b"", b"");
        xof.absorb(b"abc");
        let mut out = [0u8; 64];
        xof.squeeze(&mut out);
        assert_eq!(
            hex::encode(out),
            "483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739d5a15bef186a5386c75744c0527e1faa9f8726e462a12a4feb06bd8801e751e4"
        );
    }
}
