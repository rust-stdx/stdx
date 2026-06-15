use crate::{Bytes, Hash, Hasher, Xof};

/// BLAKE3 hash function.
///
/// Implements the [`Hasher`] trait.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::{Hasher, blake3::Blake3};
///
/// let hash = Blake3::hash(b"hello world");
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{Hasher, blake3::Blake3};
///
/// let mut hasher = Blake3::new();
/// hasher.update(b"hello ");
/// hasher.update(b"world");
/// let hash = hasher.sum();
/// ```
///
/// # XOF (Extendable-Output Function) API
///
/// ```ignore
/// use crypto::{Hasher, blake3::Blake3};
///
/// let mut out = [0u8; 64];
/// let mut hasher = Blake3::new();
/// hasher.update(b"hello world");
/// hasher.finalize_xof().squeeze(&mut out);
/// ```
/// # Keyed hashing
///
/// ```ignore
/// use crypto::{Hasher, blake3::Blake3};
///
/// let key = b"whats the Elvish word for friend";
/// let mut hasher = Blake3::new_keyed(key);
/// hasher.update(b"hello world");
/// let hash = hasher.sum();
/// ```
///
/// The [`keyed_hash`](Blake3::keyed_hash) one-shot is also available:
///
/// ```ignore
/// use crypto::blake3::Blake3;
///
/// let key = b"whats the Elvish word for friend";
/// let hash = Blake3::keyed_hash(key, b"hello world");
/// ```
///
/// # Derive-key mode
///
/// ```ignore
/// use crypto::blake3::Blake3;
///
/// let key = Blake3::derive_key("my app context", b"key material");
/// let mut hasher = Blake3::new_keyed(&key);
/// hasher.update(b"hello world");
/// let hash = hasher.sum();
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Blake3 {
    hasher: blake3::Hasher,
}

impl Blake3 {
    #[inline]
    pub fn new_keyed(key: &[u8; 32]) -> Self {
        Self {
            hasher: blake3::Hasher::new_keyed(key),
        }
    }

    #[inline]
    pub fn new_derive_key(context: &str) -> Self {
        Self {
            hasher: blake3::Hasher::new_derive_key(context),
        }
    }

    #[inline]
    pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; 32] {
        blake3::derive_key(context, key_material)
    }

    #[inline]
    pub fn keyed_hash(key: &[u8; 32], input: &[u8]) -> Hash {
        let blake3_hash = blake3::keyed_hash(key, input);
        Hash(Bytes::<64>::from(blake3_hash.as_bytes()))
    }

    #[inline]
    pub fn finalize_xof(self) -> Blake3XofReader {
        Blake3XofReader {
            reader: self.hasher.finalize_xof(),
        }
    }
}

impl Hasher for Blake3 {
    const BLOCK_SIZE: usize = 64;
    const OUTPUT_SIZE: usize = 32;

    fn new() -> Self {
        Self {
            hasher: blake3::Hasher::new(),
        }
    }

    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    fn sum(self) -> Hash {
        let blake3_hash = self.hasher.finalize();
        Hash(Bytes::<64>::from(blake3_hash.as_bytes()))
    }
}

/// BLAKE3 extensible-output function (XOF) reader.
///
/// Produced by [`Blake3::finalize_xof`]. Implements the [`Xof`] trait.
///
/// ```ignore
/// use crypto::{blake3::Blake3, Xof};
///
/// let mut hasher = Blake3::new();
/// hasher.update(b"hello world");
/// let mut xof = hasher.finalize_xof();
/// let mut out = [0u8; 64];
/// xof.squeeze(&mut out);
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Blake3XofReader {
    reader: blake3::OutputReader,
}

impl Xof for Blake3XofReader {
    /// Do nothing.
    fn absorb(&mut self, _data: &[u8]) {}

    /// Fill `out` with bytes.
    fn squeeze(&mut self, out: &mut [u8]) {
        self.reader.fill(out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Hasher;

    const TEST_KEY: &[u8; 32] = b"whats the Elvish word for friend";

    fn test_input(len: usize) -> Vec<u8> {
        (0..len).map(|i| (i % 251) as u8).collect()
    }

    struct TestCase {
        input_len: usize,
        hash: &'static str,
        keyed_hash: &'static str,
        derive_key: &'static str,
    }

    const TEST_CASES: &[TestCase] = &[
        TestCase {
            input_len: 0,
            hash: "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262e00f03e7b69af26b7faaf09fcd333050338ddfe085b8cc869ca98b206c08243a26f5487789e8f660afe6c99ef9e0c52b92e7393024a80459cf91f476f9ffdbda7001c22e159b402631f277ca96f2defdf1078282314e763699a31c5363165421cce14d",
            keyed_hash: "92b2b75604ed3c761f9d6f62392c8a9227ad0ea3f09573e783f1498a4ed60d26b18171a2f22a4b94822c701f107153dba24918c4bae4d2945c20ece13387627d3b73cbf97b797d5e59948c7ef788f54372df45e45e4293c7dc18c1d41144a9758be58960856be1eabbe22c2653190de560ca3b2ac4aa692a9210694254c371e851bc8f",
            derive_key: "2cc39783c223154fea8dfb7c1b1660f2ac2dcbd1c1de8277b0b0dd39b7e50d7d905630c8be290dfcf3e6842f13bddd573c098c3f17361f1f206b8cad9d088aa4a3f746752c6b0ce6a83b0da81d59649257cdf8eb3e9f7d4998e41021fac119deefb896224ac99f860011f73609e6e0e4540f93b273e56547dfd3aa1a035ba6689d89a0",
        },
        TestCase {
            input_len: 1,
            hash: "2d3adedff11b61f14c886e35afa036736dcd87a74d27b5c1510225d0f592e213c3a6cb8bf623e20cdb535f8d1a5ffb86342d9c0b64aca3bce1d31f60adfa137b358ad4d79f97b47c3d5e79f179df87a3b9776ef8325f8329886ba42f07fb138bb502f4081cbcec3195c5871e6c23e2cc97d3c69a613eba131e5f1351f3f1da786545e5",
            keyed_hash: "6d7878dfff2f485635d39013278ae14f1454b8c0a3a2d34bc1ab38228a80c95b6568c0490609413006fbd428eb3fd14e7756d90f73a4725fad147f7bf70fd61c4e0cf7074885e92b0e3f125978b4154986d4fb202a3f331a3fb6cf349a3a70e49990f98fe4289761c8602c4e6ab1138d31d3b62218078b2f3ba9a88e1d08d0dd4cea11",
            derive_key: "b3e2e340a117a499c6cf2398a19ee0d29cca2bb7404c73063382693bf66cb06c5827b91bf889b6b97c5477f535361caefca0b5d8c4746441c57617111933158950670f9aa8a05d791daae10ac683cbef8faf897c84e6114a59d2173c3f417023a35d6983f2c7dfa57e7fc559ad751dbfb9ffab39c2ef8c4aafebc9ae973a64f0c76551",
        },
        TestCase {
            input_len: 63,
            hash: "e9bc37a594daad83be9470df7f7b3798297c3d834ce80ba85d6e207627b7db7b1197012b1e7d9af4d7cb7bdd1f3bb49a90a9b5dec3ea2bbc6eaebce77f4e470cbf4687093b5352f04e4a4570fba233164e6acc36900e35d185886a827f7ea9bdc1e5c3ce88b095a200e62c10c043b3e9bc6cb9b6ac4dfa51794b02ace9f98779040755",
            keyed_hash: "bb1eb5d4afa793c1ebdd9fb08def6c36d10096986ae0cfe148cd101170ce37aea05a63d74a840aecd514f654f080e51ac50fd617d22610d91780fe6b07a26b0847abb38291058c97474ef6ddd190d30fc318185c09ca1589d2024f0a6f16d45f11678377483fa5c005b2a107cb9943e5da634e7046855eaa888663de55d6471371d55d",
            derive_key: "b6451e30b953c206e34644c6803724e9d2725e0893039cfc49584f991f451af3b89e8ff572d3da4f4022199b9563b9d70ebb616efff0763e9abec71b550f1371e233319c4c4e74da936ba8e5bbb29a598e007a0bbfa929c99738ca2cc098d59134d11ff300c39f82e2fce9f7f0fa266459503f64ab9913befc65fddc474f6dc1c67669",
        },
        TestCase {
            input_len: 64,
            hash: "4eed7141ea4a5cd4b788606bd23f46e212af9cacebacdc7d1f4c6dc7f2511b98fc9cc56cb831ffe33ea8e7e1d1df09b26efd2767670066aa82d023b1dfe8ab1b2b7fbb5b97592d46ffe3e05a6a9b592e2949c74160e4674301bc3f97e04903f8c6cf95b863174c33228924cdef7ae47559b10b294acd660666c4538833582b43f82d74",
            keyed_hash: "ba8ced36f327700d213f120b1a207a3b8c04330528586f414d09f2f7d9ccb7e68244c26010afc3f762615bbac552a1ca909e67c83e2fd5478cf46b9e811efccc93f77a21b17a152ebaca1695733fdb086e23cd0eb48c41c034d52523fc21236e5d8c9255306e48d52ba40b4dac24256460d56573d1312319afcf3ed39d72d0bfc69acb",
            derive_key: "a5c4a7053fa86b64746d4bb688d06ad1f02a18fce9afd3e818fefaa7126bf73e9b9493a9befebe0bf0c9509fb3105cfa0e262cde141aa8e3f2c2f77890bb64a4cca96922a21ead111f6338ad5244f2c15c44cb595443ac2ac294231e31be4a4307d0a91e874d36fc9852aeb1265c09b6e0cda7c37ef686fbbcab97e8ff66718be048bb",
        },
        TestCase {
            input_len: 65,
            hash: "de1e5fa0be70df6d2be8fffd0e99ceaa8eb6e8c93a63f2d8d1c30ecb6b263dee0e16e0a4749d6811dd1d6d1265c29729b1b75a9ac346cf93f0e1d7296dfcfd4313b3a227faaaaf7757cc95b4e87a49be3b8a270a12020233509b1c3632b3485eef309d0abc4a4a696c9decc6e90454b53b000f456a3f10079072baaf7a981653221f2c",
            keyed_hash: "c0a4edefa2d2accb9277c371ac12fcdbb52988a86edc54f0716e1591b4326e72d5e795f46a596b02d3d4bfb43abad1e5d19211152722ec1f20fef2cd413e3c22f2fc5da3d73041275be6ede3517b3b9f0fc67ade5956a672b8b75d96cb43294b9041497de92637ed3f2439225e683910cb3ae923374449ca788fb0f9bea92731bc26ad",
            derive_key: "51fd05c3c1cfbc8ed67d139ad76f5cf8236cd2acd26627a30c104dfd9d3ff8a82b02e8bd36d8498a75ad8c8e9b15eb386970283d6dd42c8ae7911cc592887fdbe26a0a5f0bf821cd92986c60b2502c9be3f98a9c133a7e8045ea867e0828c7252e739321f7c2d65daee4468eb4429efae469a42763f1f94977435d10dccae3e3dce88d",
        },
        TestCase {
            input_len: 127,
            hash: "d81293fda863f008c09e92fc382a81f5a0b4a1251cba1634016a0f86a6bd640de3137d477156d1fde56b0cf36f8ef18b44b2d79897bece12227539ac9ae0a5119da47644d934d26e74dc316145dcb8bb69ac3f2e05c242dd6ee06484fcb0e956dc44355b452c5e2bbb5e2b66e99f5dd443d0cbcaaafd4beebaed24ae2f8bb672bcef78",
            keyed_hash: "c64200ae7dfaf35577ac5a9521c47863fb71514a3bcad18819218b818de85818ee7a317aaccc1458f78d6f65f3427ec97d9c0adb0d6dacd4471374b621b7b5f35cd54663c64dbe0b9e2d95632f84c611313ea5bd90b71ce97b3cf645776f3adc11e27d135cbadb9875c2bf8d3ae6b02f8a0206aba0c35bfe42574011931c9a255ce6dc",
            derive_key: "c91c090ceee3a3ac81902da31838012625bbcd73fcb92e7d7e56f78deba4f0c3feeb3974306966ccb3e3c69c337ef8a45660ad02526306fd685c88542ad00f759af6dd1adc2e50c2b8aac9f0c5221ff481565cf6455b772515a69463223202e5c371743e35210bbbbabd89651684107fd9fe493c937be16e39cfa7084a36207c99bea3",
        },
        TestCase {
            input_len: 1023,
            hash: "10108970eeda3eb932baac1428c7a2163b0e924c9a9e25b35bba72b28f70bd11a182d27a591b05592b15607500e1e8dd56bc6c7fc063715b7a1d737df5bad3339c56778957d870eb9717b57ea3d9fb68d1b55127bba6a906a4a24bbd5acb2d123a37b28f9e9a81bbaae360d58f85e5fc9d75f7c370a0cc09b6522d9c8d822f2f28f485",
            keyed_hash: "c951ecdf03288d0fcc96ee3413563d8a6d3589547f2c2fb36d9786470f1b9d6e890316d2e6d8b8c25b0a5b2180f94fb1a158ef508c3cde45e2966bd796a696d3e13efd86259d756387d9becf5c8bf1ce2192b87025152907b6d8cc33d17826d8b7b9bc97e38c3c85108ef09f013e01c229c20a83d9e8efac5b37470da28575fd755a10",
            derive_key: "74a16c1c3d44368a86e1ca6df64be6a2f64cce8f09220787450722d85725dea59c413264404661e9e4d955409dfe4ad3aa487871bcd454ed12abfe2c2b1eb7757588cf6cb18d2eccad49e018c0d0fec323bec82bf1644c6325717d13ea712e6840d3e6e730d35553f59eff5377a9c350bcc1556694b924b858f329c44ee64b884ef00d",
        },
        TestCase {
            input_len: 1024,
            hash: "42214739f095a406f3fc83deb889744ac00df831c10daa55189b5d121c855af71cf8107265ecdaf8505b95d8fcec83a98a6a96ea5109d2c179c47a387ffbb404756f6eeae7883b446b70ebb144527c2075ab8ab204c0086bb22b7c93d465efc57f8d917f0b385c6df265e77003b85102967486ed57db5c5ca170ba441427ed9afa684e",
            keyed_hash: "75c46f6f3d9eb4f55ecaaee480db732e6c2105546f1e675003687c31719c7ba4a78bc838c72852d4f49c864acb7adafe2478e824afe51c8919d06168414c265f298a8094b1ad813a9b8614acabac321f24ce61c5a5346eb519520d38ecc43e89b5000236df0597243e4d2493fd626730e2ba17ac4d8824d09d1a4a8f57b8227778e2de",
            derive_key: "7356cd7720d5b66b6d0697eb3177d9f8d73a4a5c5e968896eb6a6896843027066c23b601d3ddfb391e90d5c8eccdef4ae2a264bce9e612ba15e2bc9d654af1481b2e75dbabe615974f1070bba84d56853265a34330b4766f8e75edd1f4a1650476c10802f22b64bd3919d246ba20a17558bc51c199efdec67e80a227251808d8ce5bad",
        },
        TestCase {
            input_len: 2048,
            hash: "e776b6028c7cd22a4d0ba182a8bf62205d2ef576467e838ed6f2529b85fba24a9a60bf80001410ec9eea6698cd537939fad4749edd484cb541aced55cd9bf54764d063f23f6f1e32e12958ba5cfeb1bf618ad094266d4fc3c968c2088f677454c288c67ba0dba337b9d91c7e1ba586dc9a5bc2d5e90c14f53a8863ac75655461cea8f9",
            keyed_hash: "879cf1fa2ea0e79126cb1063617a05b6ad9d0b696d0d757cf053439f60a99dd10173b961cd574288194b23ece278c330fbb8585485e74967f31352a8183aa782b2b22f26cdcadb61eed1a5bc144b8198fbb0c13abbf8e3192c145d0a5c21633b0ef86054f42809df823389ee40811a5910dcbd1018af31c3b43aa55201ed4edaac74fe",
            derive_key: "7b2945cb4fef70885cc5d78a87bf6f6207dd901ff239201351ffac04e1088a23e2c11a1ebffcea4d80447867b61badb1383d842d4e79645d48dd82ccba290769caa7af8eaa1bd78a2a5e6e94fbdab78d9c7b74e894879f6a515257ccf6f95056f4e25390f24f6b35ffbb74b766202569b1d797f2d4bd9d17524c720107f985f4ddc583",
        },
        TestCase {
            input_len: 4096,
            hash: "015094013f57a5277b59d8475c0501042c0b642e531b0a1c8f58d2163229e9690289e9409ddb1b99768eafe1623da896faf7e1114bebeadc1be30829b6f8af707d85c298f4f0ff4d9438aef948335612ae921e76d411c3a9111df62d27eaf871959ae0062b5492a0feb98ef3ed4af277f5395172dbe5c311918ea0074ce0036454f620",
            keyed_hash: "befc660aea2f1718884cd8deb9902811d332f4fc4a38cf7c7300d597a081bfc0bbb64a36edb564e01e4b4aaf3b060092a6b838bea44afebd2deb8298fa562b7b597c757b9df4c911c3ca462e2ac89e9a787357aaf74c3b56d5c07bc93ce899568a3eb17d9250c20f6c5f6c1e792ec9a2dcb715398d5a6ec6d5c54f586a00403a1af1de",
            derive_key: "1e0d7f3db8c414c97c6307cbda6cd27ac3b030949da8e23be1a1a924ad2f25b9d78038f7b198596c6cc4a9ccf93223c08722d684f240ff6569075ed81591fd93f9fff1110b3a75bc67e426012e5588959cc5a4c192173a03c00731cf84544f65a2fb9378989f72e9694a6a394a8a30997c2e67f95a504e631cd2c5f55246024761b245",
        },
        TestCase {
            input_len: 8192,
            hash: "aae792484c8efe4f19e2ca7d371d8c467ffb10748d8a5a1ae579948f718a2a635fe51a27db045a567c1ad51be5aa34c01c6651c4d9b5b5ac5d0fd58cf18dd61a47778566b797a8c67df7b1d60b97b19288d2d877bb2df417ace009dcb0241ca1257d62712b6a4043b4ff33f690d849da91ea3bf711ed583cb7b7a7da2839ba71309bbf",
            keyed_hash: "dc9637c8845a770b4cbf76b8daec0eebf7dc2eac11498517f08d44c8fc00d58a4834464159dcbc12a0ba0c6d6eb41bac0ed6585cabfe0aca36a375e6c5480c22afdc40785c170f5a6b8a1107dbee282318d00d915ac9ed1143ad40765ec120042ee121cd2baa36250c618adaf9e27260fda2f94dea8fb6f08c04f8f10c78292aa46102",
            derive_key: "ad01d7ae4ad059b0d33baa3c01319dcf8088094d0359e5fd45d6aeaa8b2d0c3d4c9e58958553513b67f84f8eac653aeeb02ae1d5672dcecf91cd9985a0e67f4501910ecba25555395427ccc7241d70dc21c190e2aadee875e5aae6bf1912837e53411dabf7a56cbf8e4fb780432b0d7fe6cec45024a0788cf5874616407757e9e6bef7",
        },
        TestCase {
            input_len: 102400,
            hash: "bc3e3d41a1146b069abffad3c0d44860cf664390afce4d9661f7902e7943e085e01c59dab908c04c3342b816941a26d69c2605ebee5ec5291cc55e15b76146e6745f0601156c3596cb75065a9c57f35585a52e1ac70f69131c23d611ce11ee4ab1ec2c009012d236648e77be9295dd0426f29b764d65de58eb7d01dd42248204f45f8e",
            keyed_hash: "1c35d1a5811083fd7119f5d5d1ba027b4d01c0c6c49fb6ff2cf75393ea5db4a7f9dbdd3e1d81dcbca3ba241bb18760f207710b751846faaeb9dff8262710999a59b2aa1aca298a032d94eacfadf1aa192418eb54808db23b56e34213266aa08499a16b354f018fc4967d05f8b9d2ad87a7278337be9693fc638a3bfdbe314574ee6fc4",
            derive_key: "4652cff7a3f385a6103b5c260fc1593e13c778dbe608efb092fe7ee69df6e9c6d83a3e041bc3a48df2879f4a0a3ed40e7c961c73eff740f3117a0504c2dff4786d44fb17f1549eb0ba585e40ec29bf7732f0b7e286ff8acddc4cb1e23b87ff5d824a986458dcc6a04ac83969b80637562953df51ed1a7e90a7926924d2763778be8560",
        },
    ];

    #[test]
    fn test_vectors_hash() {
        for case in TEST_CASES {
            let input = test_input(case.input_len);
            let hash = Blake3::hash(&input);
            let expected = hex::decode(case.hash).unwrap();
            assert_eq!(hash.as_ref(), &expected[..32], "hash mismatch at input_len={}", case.input_len);
        }
    }

    #[test]
    fn test_vectors_hash_extended() {
        for case in TEST_CASES {
            let input = test_input(case.input_len);
            let mut hasher = Blake3::new();
            hasher.update(&input);
            let hash = hasher.sum();
            let expected = hex::decode(case.hash).unwrap();
            assert_eq!(
                hash.as_ref(),
                &expected[..hash.len()],
                "extended hash mismatch at input_len={}",
                case.input_len
            );
        }
    }

    #[test]
    fn test_vectors_keyed_hash() {
        for case in TEST_CASES {
            let input = test_input(case.input_len);
            let hash = Blake3::keyed_hash(TEST_KEY, &input);
            let expected = hex::decode(case.keyed_hash).unwrap();
            assert_eq!(
                hash.as_ref(),
                &expected[..32],
                "keyed_hash mismatch at input_len={}",
                case.input_len
            );
        }
    }

    #[test]
    fn test_vectors_derive_key() {
        let context = "BLAKE3 2019-12-27 16:29:52 test vectors context";
        for case in TEST_CASES {
            let input = test_input(case.input_len);
            let key = Blake3::derive_key(context, &input);
            let expected = hex::decode(case.derive_key).unwrap();
            assert_eq!(&key[..], &expected[..32], "derive_key mismatch at input_len={}", case.input_len);
        }
    }

    #[test]
    fn test_vectors_xof() {
        for case in TEST_CASES {
            let input = test_input(case.input_len);
            let mut output = vec![0u8; 128];
            let mut hasher = Blake3::new();
            hasher.update(&input);
            let mut xof = hasher.finalize_xof();
            xof.squeeze(&mut output);
            let expected = hex::decode(case.hash).unwrap();
            assert_eq!(&output[..], &expected[..128], "XOF mismatch at input_len={}", case.input_len);
        }
    }

    #[test]
    fn test_vectors_xof_incremental() {
        for case in TEST_CASES {
            let input = test_input(case.input_len);
            let mut output = vec![0u8; 128];
            let mut hasher = Blake3::new();
            hasher.update(&input);
            let mut xof = hasher.finalize_xof();
            xof.squeeze(&mut output[..64]);
            xof.squeeze(&mut output[64..]);
            let expected = hex::decode(case.hash).unwrap();
            assert_eq!(
                &output[..],
                &expected[..128],
                "XOF incremental mismatch at input_len={}",
                case.input_len
            );
        }
    }

    #[test]
    fn test_incremental_vs_oneshot() {
        for len in [0usize, 1, 13, 64, 65, 127, 128, 129, 1023, 1024, 1025, 2048] {
            let input = test_input(len);
            let mut full = Blake3::new();
            full.update(&input);
            let full_hash = full.sum();

            let mut incremental = Blake3::new();
            for chunk in input.chunks(13) {
                incremental.update(chunk);
            }
            let inc_hash = incremental.sum();

            assert_eq!(full_hash.as_ref(), inc_hash.as_ref(), "incremental mismatch at len={}", len);
        }
    }

    #[test]
    fn test_xof_incremental_absorb() {
        let input = test_input(1024);
        let mut output_one = vec![0u8; 32];
        let mut h1 = Blake3::new();
        h1.update(&input);
        let mut xof = h1.finalize_xof();
        xof.squeeze(&mut output_one);

        let mut output_two = vec![0u8; 32];
        let mut h2 = Blake3::new();
        for chunk in input.chunks(13) {
            h2.update(chunk);
        }
        let mut xof = h2.finalize_xof();
        xof.squeeze(&mut output_two);

        assert_eq!(output_one, output_two);
    }

    #[test]
    fn test_empty_input() {
        let hash = Blake3::hash(b"");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_new_keyed() {
        let mut h = Blake3::new_keyed(TEST_KEY);
        h.update(b"");
        let hash = h.sum();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_new_derive_key() {
        let context = "BLAKE3 2019-12-27 16:29:52 test vectors context";
        let mut h = Blake3::new_derive_key(context);
        h.update(b"test key material");
        let hash = h.sum();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_clone() {
        let input = b"hello world";
        let mut h1 = Blake3::new();
        h1.update(input);
        let mut h2 = h1.clone();
        h1.update(b" more");
        h2.update(b" more");
        assert_eq!(h1.sum().as_ref(), h2.sum().as_ref());
    }
}
