//! TurboSHAKE256, as defined in RFC 9861.
//!
//! TurboSHAKE256 is the sponge construction of FIPS 202 restricted to the last
//! 12 rounds of the Keccak-p[1600] permutation, instead of the 24 rounds used
//! by SHA-3 and SHAKE. Halving the number of rounds roughly doubles the
//! throughput while keeping the same 256-bit security claim as SHAKE256.
//!
//! It uses a 136-byte rate (1088 bits) and a 64-byte capacity (512 bits), and
//! therefore produces the same amount of output per permutation as SHAKE256.
//! On top of the plain sponge, TurboSHAKE256 takes a domain-separation byte
//! `D` in the `0x01..=0x7f` range, which makes two instances with different `D`
//! behave like independent hash functions. When the protocol does not need
//! domain separation, `D` defaults to `0x1f`
//! ([`TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR`]).

use super::keccak::KeccakSponge;
use crate::{Bytes, Hash, Hasher, Xof};

/// Rate of TurboSHAKE256, in bytes (1088 bits), as defined in RFC 9861.
pub(crate) const TURBOSHAKE256_RATE: usize = 136;

/// Default domain-separation byte `D`, as specified by RFC 9861, section 2.1.
///
/// An API that does not expose a domain-separation byte MUST use this value.
pub const TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR: u8 = 0x1f;

/// TurboSHAKE256 eXtendable-Output Function (XOF), as defined in RFC 9861.
///
/// Like [`Shake256`](super::Shake256), the output is an arbitrarily long stream
/// whose prefix property holds: a shorter output is always a prefix of a longer
/// one for the same input and domain separator.
///
/// Implements the [`Xof`] and [`Hasher`] traits.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::sha3::TurboShake256;
///
/// let mut output = [0u8; 64];
/// TurboShake256::hash(b"hello world", &mut output);
///
/// // Or with an explicit domain separator for domain separation.
/// TurboShake256::hash(b"hello world", 0x07, &mut output);
/// ```
///
/// # Incremental XOF API
///
/// ```ignore
/// use crypto::{
///     Xof,
///     sha3::{TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR, TurboShake256},
/// };
///
/// let mut xof = TurboShake256::new(TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR);
/// xof.absorb(b"hello ");
/// xof.absorb(b"world");
/// let mut out = [0u8; 64];
/// xof.squeeze(&mut out);
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct TurboShake256 {
    keccak: KeccakSponge<12>,
}

impl TurboShake256 {
    /// Creates a TurboSHAKE256 instance with the given domain-separation byte `D`.
    ///
    /// # Panics
    ///
    /// Panics if `domain_separator` is not in the `0x01..=0x7f` range mandated
    /// by RFC 9861, section 2.1. `0x00` is rejected because the domain byte
    /// carries the first bit of the `pad10*1` padding, and values with the most
    /// significant bit set are rejected because they would collide with the
    /// final `0x80` padding bit when the domain byte is the last byte of a
    /// block.
    ///
    /// Use [`TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR`] when no domain separation
    /// is required.
    #[inline]
    pub fn new(domain_separator: u8) -> Self {
        assert!(
            (0x01..=0x7f).contains(&domain_separator),
            "the domain separator byte must be in the 0x01..=0x7f range"
        );

        return TurboShake256 {
            keccak: KeccakSponge::new(TURBOSHAKE256_RATE, domain_separator),
        };
    }

    /// Changes the domain-separation byte before the output is produced.
    ///
    /// This is an internal building block for constructions such as
    /// KangarooTwelve, which absorb a prefix before they know whether it
    /// belongs to a single node or the final tree node.
    ///
    /// # Panics
    ///
    /// Panics if `domain_separator` is outside `0x01..=0x7f`, or if squeezing
    /// has already started.
    #[inline]
    pub(crate) fn set_domain_separator(&mut self, domain_separator: u8) {
        assert!(
            (0x01..=0x7f).contains(&domain_separator),
            "the domain separator byte must be in the 0x01..=0x7f range"
        );
        self.keccak.set_delimiter(domain_separator);
    }

    /// Hashes `data` with the default domain-separation byte (`0x1f`), filling
    /// `output`.
    ///
    /// The output length is `output.len()` and may be arbitrary. If the same
    /// `(data, domain_separator)` pair is hashed with two different output
    /// lengths, the shorter output is a prefix of the longer one.
    ///
    /// # Panics
    ///
    /// Panics if `domain_separator` is not in the `0x01..=0x7f` range; see
    /// [`Self::new`].
    #[inline]
    pub fn hash(data: &[u8], output: &mut [u8]) {
        let mut hasher = TurboShake256::new(TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR);
        hasher.absorb(data);
        hasher.squeeze(output);
    }
}

impl Default for TurboShake256 {
    /// Creates a TurboSHAKE256 instance with the default domain-separation byte
    /// (`0x1f`).
    #[inline]
    fn default() -> Self {
        return TurboShake256::new(TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR);
    }
}

impl Xof for TurboShake256 {
    #[inline]
    fn absorb(&mut self, data: &[u8]) {
        self.keccak.absorb(data);
    }

    #[inline]
    fn squeeze(&mut self, out: &mut [u8]) {
        self.keccak.squeeze(out);
    }
}

impl Hasher for TurboShake256 {
    const BLOCK_SIZE: usize = TURBOSHAKE256_RATE;
    const OUTPUT_SIZE: usize = 64;

    /// Creates a TurboSHAKE256 instance with the default domain-separation byte
    /// (`0x1f`).
    #[inline]
    fn new() -> Self {
        return <Self as Default>::default();
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

#[cfg(test)]
mod tests {
    use super::{TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR, TurboShake256};
    use crate::{
        Hasher, Xof,
        sha3::keccak::{ROUND_CONSTANTS, p1600},
    };

    const DEFAULT_D: u8 = TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR;

    /// RFC 9861, section 5: `ptn(n)` repeats the bytes `00 01 02 .. F9 FA` (the
    /// 251-byte cycle `00..=FA`) as many times as needed, truncated to `n` bytes.
    fn ptn(n: usize) -> Vec<u8> {
        return (0..n).map(|i| (i % 251) as u8).collect();
    }

    /// Every RFC 9861, section 5 TurboSHAKE256 vector that fits in a reasonable
    /// amount of memory, as `(input, domain_separator, expected_hex)` with a
    /// 64-byte output.
    fn vectors() -> Vec<(Vec<u8>, u8, &'static str)> {
        return vec![
            (
                vec![],
                0x1f,
                "367a329dafea871c7802ec67f905ae13c57695dc2c6663c61035f59a18f8e7db\
                 11edc0e12e91ea60eb6b32df06dd7f002fbafabb6e13ec1cc20d995547600db0",
            ),
            (
                ptn(1),
                0x1f,
                "3e1712f928f8eaf1054632b2aa0a246ed8b0c378728f60bc970410155c28820e\
                 90cc90d8a3006aa2372c5c5ea176b0682bf22bae7467ac94f74d43d39b0482e2",
            ),
            (
                ptn(17),
                0x1f,
                "b3bab0300e6a191fbe6137939835923578794ea54843f5011090fa2f3780a9e5\
                 cb22c59d78b40a0fbff9e672c0fbe0970bd2c845091c6044d687054da5d8e9c7",
            ),
            (
                ptn(17usize.pow(2)),
                0x1f,
                "66b810db8e90780424c0847372fdc95710882fde31c6df75beb9d4cd9305cfca\
                 e35e7b83e8b7e6eb4b78605880116316fe2c078a09b94ad7b8213c0a738b65c0",
            ),
            (
                ptn(17usize.pow(3)),
                0x1f,
                "c74ebc919a5b3b0dd1228185ba02d29ef442d69d3d4276a93efe0bf9a16a7dc0\
                 cd4eabadab8cd7a5edd96695f5d360abe09e2c6511a3ec397da3b76b9e1674fb",
            ),
            (
                ptn(17usize.pow(4)),
                0x1f,
                "02cc3a8897e6f4f6ccb6fd46631b1f5207b66c6de9c7b55b2d1a23134a170afd\
                 ac234eaba9a77cff88c1f020b73724618c5687b362c430b248cd38647f848a1d",
            ),
            (
                ptn(17usize.pow(5)),
                0x1f,
                "add53b06543e584b5823f626996aee50fe45ed15f20243a7165485acb4aa76b4\
                 ffda75cedf6d8cdc95c332bd56f4b986b58bb17d1778bfc1b1a97545cdf4ec9f",
            ),
            (
                vec![0xff; 3],
                0x01,
                "d21c6fbbf587fa2282f29aea620175fb0257413af78a0b1b2a87419ce031d933\
                 ae7a4d383327a8a17641a34f8a1d1003ad7da6b72dba84bb62fef28f62f12424",
            ),
            (
                vec![0xff],
                0x06,
                "738d7b4e37d18b7f22ad1b5313e357e3dd7d07056a26a303c433fa3533455280\
                 f4f5a7d4f700efb437fe6d281405e07be32a0a972e22e63adc1b090daefe004b",
            ),
            (
                vec![0xff; 3],
                0x07,
                "18b3b5b7061c2e67c1753a00e6ad7ed7ba1c906cf93efb7092eaf27fbeebb755\
                 ae6e292493c110e48d260028492b8e09b5500612b8f2578985ded5357d00ec67",
            ),
            (
                vec![0xff; 7],
                0x0b,
                "bb36764951ec97e9d85f7ee9a67a7718fc005cf42556be79ce12c0bde50e5736\
                 d6632b0d0dfb202d1bbb8ffe3dd74cb00834fa756cb03471bab13a1e2c16b3c0",
            ),
            (
                vec![0xff],
                0x30,
                "f3fe12873d34bcbb2e608779d6b70e7f86bec7e90bf113cbd4fdd0c4e2f4625e\
                 148dd7ee1a52776cf77f240514d9ccfc3b5ddab8ee255e39ee389072962c111a",
            ),
            (
                vec![0xff; 3],
                0x7f,
                "abe569c1f77ec340f02705e7d37c9ab7e155516e4a6a150021d70b6fac0bb40c\
                 069f9a9828a0d575cd99f9bae435ab1acf7ed9110ba97ce0388d074bac768776",
            ),
        ];
    }

    #[test]
    fn known_vectors() {
        for (input, domain_separator, expected) in vectors() {
            let mut output = [0u8; 64];
            let mut hasher = TurboShake256::new(domain_separator);
            hasher.update(&input);
            hasher.squeeze(&mut output);
            assert_eq!(
                hex::encode(output),
                expected.replace(['\n', ' '], ""),
                "input length {}, D {domain_separator:#04x}",
                input.len(),
            );

            // The same call through the default constructor / Hasher trait must
            // agree when the separator is the default one.
            if domain_separator == DEFAULT_D {
                let mut default_output = [0u8; 64];
                TurboShake256::hash(&input, &mut default_output);
                assert_eq!(default_output, output);
            }
        }
    }

    /// RFC 9861, section 5: `TurboSHAKE256(M=00^0, D=1F, 10032)`, last 32 bytes.
    ///
    /// The 10032-byte output also exercises the squeeze path far beyond one
    /// rate-sized block.
    #[test]
    fn long_output_matches_rfc() {
        const OUTPUT_LEN: usize = 10032;

        let mut output = vec![0u8; OUTPUT_LEN];
        TurboShake256::hash(&[], &mut output);

        assert_eq!(
            hex::encode(&output[OUTPUT_LEN - 32..]),
            "abefa11630c661269249742685ec082f207265dccf2f43534e9c61ba0c9d1d75"
        );

        // Prefix property: the first 64 bytes must match the 64-byte vector.
        assert_eq!(
            hex::encode(&output[..64]),
            "367a329dafea871c7802ec67f905ae13c57695dc2c6663c61035f59a18f8e7db\
             11edc0e12e91ea60eb6b32df06dd7f002fbafabb6e13ec1cc20d995547600db0"
                .replace(['\n', ' '], "")
        );
    }

    /// The largest RFC 9861 pattern vector, `ptn(17**6)` (~24 MiB), which
    /// exercises many absorbed rate blocks.
    #[test]
    fn long_pattern_vector() {
        let input = ptn(17usize.pow(6));
        let mut output = [0u8; 64];
        TurboShake256::hash(&input, &mut output);
        assert_eq!(
            hex::encode(output),
            "9e11bc59c24e73993c1484ec66358ef71db74aefd84e123f7800ba9c4853e02c\
             fe701d9e6bb765a304f0dc34a4ee3ba82c410f0da70e86bfbd90ea877c2d6104"
                .replace(['\n', ' '], "")
        );
    }

    /// A single `squeeze` in awkward chunks must produce the same stream as one
    /// big squeeze, and several `absorb` calls must match a one-shot hash.
    #[test]
    fn incremental_matches_one_shot() {
        const OUTPUT_LEN: usize = 10032;

        let mut one_shot = vec![0u8; OUTPUT_LEN];
        TurboShake256::hash(b"hello world", &mut one_shot);

        let mut xof = TurboShake256::default();
        xof.absorb(b"hello ");
        xof.absorb(b"world");

        let mut streamed = vec![0u8; OUTPUT_LEN];
        let mut offset = 0;
        for chunk in [1usize, 7, 63, 135, 137, 1, 1000, 5000] {
            if offset >= streamed.len() {
                break;
            }
            let n = chunk.min(streamed.len() - offset);
            xof.squeeze(&mut streamed[offset..offset + n]);
            offset += n;
        }
        xof.squeeze(&mut streamed[offset..]);

        assert_eq!(streamed, one_shot);
    }

    /// Inputs whose length is close to a multiple of the 136-byte rate must be
    /// absorbed identically no matter how they are split.
    #[test]
    fn rate_boundaries() {
        for len in [0usize, 1, 134, 135, 136, 137, 200, 271, 272, 273, 4096] {
            let data = ptn(len);

            let mut one_shot = [0u8; 64];
            TurboShake256::hash(&data, &mut one_shot);

            let mut xof = TurboShake256::default();
            for chunk in data.chunks(5) {
                xof.absorb(chunk);
            }
            let mut incremental = [0u8; 64];
            xof.squeeze(&mut incremental);

            assert_eq!(one_shot, incremental, "input length {len}");
        }
    }

    /// Differential vectors generated with PyCryptodome 4.0.1, an independent
    /// implementation: `(input_length, domain_separator, expected_32_byte_output)`
    /// where the input is `ptn(input_length)`.
    ///
    /// Together they cover message lengths that are exactly one rate block
    /// (136 bytes), one byte short of a block (so the domain byte shares the
    /// last state byte with the `0x80` padding bit) and one byte over, as well
    /// as domain bytes that the RFC 9861 vectors do not exercise.
    fn differential_vectors() -> Vec<(usize, u8, &'static str)> {
        return vec![
            (0, 0x02, "cfdbc69ec2652711dc3013cee68def374948ef09e62d82f2749e3dbc71f04dce"),
            (0, 0x41, "391e348fd25a062fcc048dcf4534d7fc00758b8c4ed7a8e1ef66dc1010f80595"),
            (0, 0x7f, "49b38a11204328440c4c40fdaee305629379936d7a31f9474c4f0fb062a2a427"),
            (1, 0x02, "ef2e014a953606720af8ee65df2c4ccbe3f5a11a364aeee3303eeb5acd28435c"),
            (1, 0x41, "97e622761524f4842beed783d35717fb6268bd7748d9442219b2c242541d43e1"),
            (1, 0x7f, "301404f5302f19210bfeae4fa5a639f4333638f983ed7b7c978460e33951868c"),
            (2, 0x02, "c787c502a002f5816415555eae3c5857aeec69aa85810769eb04db629f1f455b"),
            (2, 0x41, "f72333aa5501569af9600acbe5c3ced97fe7c44bd40eb82b60630bb3421958ed"),
            (2, 0x7f, "85b9d04d118e0f3731ea8948042875f7fcbcbf411d5faf3cb1ac6e352bb15eba"),
            (63, 0x02, "27cbf39d4dc480b5c78a509de927f13ac8e16a2253131d463282a450a4cc984b"),
            (63, 0x41, "14c145d78ca7ef1661185af7a2ef4ee82adf052d0ca74303a997c4e4bc11000b"),
            (63, 0x7f, "608d32cddf279c626302e8ad8adf4bc9360639cb56430c83eccf62d6e9d96d34"),
            (64, 0x02, "a47c03c363d19a7eb5325964aa24b4a2573063bf822918afce6d0865a28753d3"),
            (64, 0x41, "7bc25a161c03dfa30780452f05d52b68f3f0a94717c38a16832b333bcc5ca7c0"),
            (64, 0x7f, "920d313397045ffe6e3bab511bce487f53db234eebfcc0e8f273f626dc357b43"),
            (65, 0x02, "dbc29ffcf2a97958e7acff8c4eaf6b0263193e66320f8b8c247876beefbff887"),
            (65, 0x41, "a9add9502d77457aac2cd01ce3ea658d3b65e4a3e312ba0ce92ebfa2e5ec7b8d"),
            (65, 0x7f, "cc23acd3fadf36b7f7f88104a5a7fdfa6644551983586e9295e1d8a26742163d"),
            (135, 0x02, "813e91996a67d8326752d1760ed0ffdfecf19edd1ee5e3ee6416bda70f60c221"),
            (135, 0x41, "4215bbc5907674bcedb8509a006c7c2c3bf54400c229c9f7b6a61961ed249642"),
            (135, 0x7f, "87ab1db981196d8e5502963dfc9907489c8bac874f3df69e281c2e6b28ecb0cd"),
            (136, 0x02, "451f0910475a137b6fa2ee716213a9b6842e558ab99bd6ad3e5998e683013e5a"),
            (136, 0x41, "c0f96b605383f1b8e8806e429d8c9c3aca66d6cfc9942a1cb4c78451e318a2fc"),
            (136, 0x7f, "ae1f42e822abf01e5f3030eb602f805f9fbdd57c397f0293a3d78adfb73852e6"),
            (137, 0x02, "eb8c28706ec90da4bf86dbe341fbb7dfa5011665e6f129ac4b9256f601a8684f"),
            (137, 0x41, "6296e7213753536b7375272ba723b59c2e37469fa0f35c0862486833880183bf"),
            (137, 0x7f, "9224e7f00986d673da9acc29b982c95075d1d56f0803948afa1c56b4f02f24dc"),
            (271, 0x02, "6b5a02d8fbb175a067c4cbb7e55cea487d5474fc54035fc91ceb55422aa70e99"),
            (271, 0x41, "3d3ee6fbc18440cce365c8c6567cca088666030d3e06fd3a9b3097c1c135ce52"),
            (271, 0x7f, "ec95672ba9de27dba74e676a4f522450d87948c70e39ae3ea22d3752e556f72f"),
            (272, 0x02, "a43733513b3a8933bd0239210c71f2b70003055d24ebb78c632edcbb3eeccbad"),
            (272, 0x41, "d51dfec8760dea40a77f6ec89d5f4db2d8724bf6f2b54d9f9b64c76950d80a13"),
            (272, 0x7f, "512595e83228a7fc6ea4f0f9c333a9e7f4045416dd750a70c340b1f5b3d26a87"),
            (273, 0x02, "8e86a0097b709ba813b1212a5beb81f9636b2f99b00b97a15ae1d4eb85815284"),
            (273, 0x41, "060d9f933f42ce06f6d9aba4f8a2db829e57030cb62e0d49e97ca06402fd4f6f"),
            (273, 0x7f, "1c2e693077cd245d70a6af75ac60d14e7f9cf5b5af7cd077b4040bb18355fc13"),
            (4096, 0x02, "d4525f62f033bf1afd3c19fc2b579d20ca52bc8299b44022aeebd84829fb57a0"),
            (4096, 0x41, "b3640f4fe4785b4a084cfb0467580ba81e29626f6df51dd18fe0a727518e8295"),
            (4096, 0x7f, "7866e8b119b2b42e6adbaa5799fcf859f95f61c00ae5515a8b4d8cc901b41c5c"),
        ];
    }

    #[test]
    fn differential_against_pycryptodome() {
        for (len, domain_separator, expected) in differential_vectors() {
            let data = ptn(len);
            let mut output = [0u8; 32];
            let mut hasher = TurboShake256::new(domain_separator);
            hasher.update(&data);
            hasher.squeeze(&mut output);
            assert_eq!(hex::encode(output), expected, "input length {len}, D {domain_separator:#04x}");
        }
    }

    /// Different domain-separation bytes must yield unrelated outputs.
    #[test]
    fn domain_separation() {
        let mut previous: Option<[u8; 64]> = None;
        for domain_separator in [0x01u8, 0x02, 0x1f, 0x30, 0x7f] {
            let mut output = [0u8; 64];
            let mut hasher = TurboShake256::new(domain_separator);
            hasher.update(b"domain separation");
            hasher.squeeze(&mut output);

            if let Some(previous) = previous {
                assert_ne!(previous, output, "D {domain_separator:#04x} collided");
            }
            previous = Some(output);
        }
    }

    #[test]
    #[should_panic]
    fn rejects_zero_domain_separator() {
        let _ = TurboShake256::new(0x00);
    }

    #[test]
    #[should_panic]
    fn rejects_high_bit_domain_separator() {
        let _ = TurboShake256::new(0x80);
    }

    #[test]
    fn hasher_trait_impl() {
        let digest = <TurboShake256 as Hasher>::hash(b"");
        assert_eq!(
            hex::encode(digest.as_ref()),
            "367a329dafea871c7802ec67f905ae13c57695dc2c6663c61035f59a18f8e7db\
             11edc0e12e91ea60eb6b32df06dd7f002fbafabb6e13ec1cc20d995547600db0"
                .replace(['\n', ' '], "")
        );

        let mut hasher = <TurboShake256 as Hasher>::new();
        hasher.update(b"hello ");
        hasher.update(b"world");
        let streamed = hasher.sum();

        let mut expected = [0u8; 64];
        TurboShake256::hash(b"hello world", &mut expected);
        assert_eq!(streamed.as_ref(), expected);
    }

    /// The 12 rounds of Keccak-p[1600, n_r=12] are the last 12 round constants
    /// of Keccak-f[1600]. This pins the constants that the sponge selects
    /// (RFC 9861, appendix A.1) so a wrong round count cannot silently pass.
    #[test]
    fn twelve_round_constants_match_rfc() {
        // RFC 9861, appendix A.1, interpreted as little-endian 64-bit words.
        let rfc: [u64; 12] = [
            0x000000008000808b,
            0x800000000000008b,
            0x8000000000008089,
            0x8000000000008003,
            0x8000000000008002,
            0x8000000000000080,
            0x000000000000800a,
            0x800000008000000a,
            0x8000000080008081,
            0x8000000000008080,
            0x0000000080000001,
            0x8000000080008008,
        ];
        assert_eq!(&ROUND_CONSTANTS[24 - 12..], &rfc);

        // A 12-round permutation must differ from the full 24-round one.
        let mut twelve = [0u64; 25];
        let mut full = [0u64; 25];
        p1600::<12>(&mut twelve);
        p1600::<24>(&mut full);
        assert_ne!(twelve, full);
    }
}
