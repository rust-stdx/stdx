use super::keccak::KeccakSponge;
use crate::{Bytes, Hash, Hasher, Xof};

pub(crate) const SHAKE128_RATE: usize = 168;
const SHAKE128_DOMAIN_SEPARATOR: u8 = 0x1f;

/// SHAKE128 extensible-output function (XOF) standardized in FIPS 202.
///
/// Implements both the [`Xof`] and [`Hasher`] traits.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::sha3::Shake128;
///
/// let mut output = [0u8; 32];
/// Shake128::hash(b"hello world", &mut output);
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{sha3::Shake128, Xof};
///
/// let mut shake = Shake128::new();
/// shake.absorb(b"hello ");
/// shake.absorb(b"world");
/// let mut out = [0u8; 32];
/// shake.squeeze(&mut out);
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Shake128 {
    keccak: KeccakSponge<24>,
}

impl Shake128 {
    #[inline]
    pub fn new() -> Self {
        return Shake128 {
            keccak: KeccakSponge::new(SHAKE128_RATE, SHAKE128_DOMAIN_SEPARATOR),
        };
    }

    #[inline]
    pub fn hash(data: &[u8], output: &mut [u8]) {
        let mut hasher = Shake128::new();
        hasher.absorb(data);
        hasher.squeeze(output);
    }
}

impl Hasher for Shake128 {
    const BLOCK_SIZE: usize = SHAKE128_RATE;
    const OUTPUT_SIZE: usize = 32;

    #[inline]
    fn new() -> Self {
        return Shake128::new();
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

impl Xof for Shake128 {
    #[inline]
    fn absorb(&mut self, data: &[u8]) {
        self.keccak.absorb(data);
    }

    #[inline]
    fn squeeze(&mut self, out: &mut [u8]) {
        self.keccak.squeeze(out);
    }
}

#[cfg(test)]
mod tests {
    use super::Shake128;
    use crate::Xof;

    // NIST SHAKE128 test vectors
    fn vectors_shake128() -> Vec<(Vec<u8>, usize, &'static str)> {
        vec![
            (
                b"".to_vec(),
                32,
                "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26",
            ),
            (
                b"".to_vec(),
                64,
                "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef263cb1eea988004b93103cfb0aeefd2a686e01fa4a58e8a3639ca8a1e3f9ae57e2",
            ),
            (
                b"abc".to_vec(),
                32,
                "5881092dd818bf5cf8a3ddb793fbcba74097d5c526a6d35f97b83351940f2cc8",
            ),
            (
                b"The quick brown fox jumps over the lazy dog".to_vec(),
                32,
                "f4202e3c5852f9182a0430fd8144f0a74b95e7417ecae17db0f8cfeed0e3e66e",
            ),
        ]
    }

    #[test]
    fn known_vectors() {
        for (input, output_len, expected) in vectors_shake128() {
            let mut output = vec![0u8; output_len];
            Shake128::hash(&input, &mut output);
            assert_eq!(hex::encode(&output), expected);
        }
    }

    #[test]
    fn incremental_and_streaming_read() {
        let mut one_shot = vec![0u8; 64];
        Shake128::hash(b"", &mut one_shot);

        let mut shake = Shake128::new();
        shake.absorb(b"");
        let mut first = [0u8; 32];
        let mut second = [0u8; 32];
        shake.squeeze(&mut first);
        shake.squeeze(&mut second);

        let mut combined = vec![0u8; 64];
        combined[..32].copy_from_slice(&first);
        combined[32..].copy_from_slice(&second);

        assert_eq!(combined, one_shot);
    }
}
