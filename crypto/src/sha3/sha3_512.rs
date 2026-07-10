use super::keccak::KeccakSponge;
use crate::{Bytes, Hash, Hasher};

const SHA3_512_RATE: usize = 72;
const SHA3_512_DOMAIN_SEPARATOR: u8 = 0x06;

/// SHA3-512 hash function (FIPS 202).
///
/// Implements the [`Hasher`] trait.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::{Hasher, sha3::Sha3_512};
///
/// let hash = Sha3_512::hash(b"hello world");
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{Hasher, sha3::Sha3_512};
///
/// let mut hasher = Sha3_512::new();
/// hasher.update(b"hello ");
/// hasher.update(b"world");
/// let hash = hasher.sum();
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Sha3_512 {
    keccak: KeccakSponge<24>,
}

impl Hasher for Sha3_512 {
    const BLOCK_SIZE: usize = SHA3_512_RATE;
    const OUTPUT_SIZE: usize = 64;

    #[inline]
    fn new() -> Self {
        Sha3_512 {
            keccak: KeccakSponge::new(SHA3_512_RATE, SHA3_512_DOMAIN_SEPARATOR),
        }
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.keccak.absorb(data);
    }

    #[inline]
    fn sum(mut self) -> Hash {
        let mut hash = Bytes::<64>::with_length(Self::OUTPUT_SIZE);
        self.keccak.squeeze(hash.as_mut());
        return Hash(hash);
    }
}

#[cfg(test)]
mod tests {
    use super::Sha3_512;
    use crate::Hasher;

    fn vectors_sha3_512() -> Vec<(Vec<u8>, &'static str)> {
        vec![
            (
                b"".to_vec(),
                "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26",
            ),
            (
                b"abc".to_vec(),
                "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0",
            ),
            (
                b"hello world".to_vec(),
                "840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a",
            ),
            (
                b"The quick brown fox jumps over the lazy dog".to_vec(),
                "01dedd5de4ef14642445ba5f5b97c15e47b9ad931326e4b0727cd94cefc44fff23f07bf543139939b49128caf436dc1bdee54fcb24023a08d9403f9b4bf0d450",
            ),
            (
                b"The quick brown fox jumps over the lazy dog.".to_vec(),
                "18f4f4bd419603f95538837003d9d254c26c23765565162247483f65c50303597bc9ce4d289f21d1c2f1f458828e33dc442100331b35e7eb031b5d38ba6460f8",
            ),
            (
                vec![b'a'; 1_000_000],
                "3c3a876da14034ab60627c077bb98f7e120a2a5370212dffb3385a18d4f38859ed311d0a9d5141ce9cc5c66ee689b266a8aa18ace8282a0e0db596c90b0a7b87",
            ),
        ]
    }

    #[test]
    fn known_vectors_single_update() {
        for (input, expected) in vectors_sha3_512() {
            assert_eq!(hex::encode(<Sha3_512 as Hasher>::hash(&input)), expected);
        }
    }

    #[test]
    fn known_vectors_incremental() {
        for (input, expected) in vectors_sha3_512() {
            let mut sha3_512 = <Sha3_512 as Hasher>::new();
            for chunk in input.chunks(11) {
                sha3_512.update(chunk);
            }
            assert_eq!(hex::encode(&sha3_512.sum()), expected);
        }
    }

    #[test]
    fn hasher_trait_impl() {
        for (input, expected) in vectors_sha3_512() {
            let digest = <Sha3_512 as Hasher>::hash(&input);
            assert_eq!(hex::encode(&digest), expected);
        }
    }
}
