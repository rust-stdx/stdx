//! Unit tests for the BLAKE3 module.
//!
//! Besides the standard known-answer vectors, this module embeds a small
//! independent reference implementation (a port of the public-domain BLAKE3
//! reference code) and checks the optimized implementation against it over
//! many input shapes, including every SIMD kernel that was compiled in.

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

use super::*;
use crate::{Hasher, Xof};

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
        let expected = hex::decode(case.hash).unwrap();
        // The vectors are 131 bytes: a non-multiple of 4 so that a failure to
        // emit the final partial word is caught.
        let mut output = vec![0u8; expected.len()];
        let mut hasher = Blake3::new();
        hasher.update(&input);
        let mut xof = hasher.finalize_xof();
        xof.squeeze(&mut output);
        assert_eq!(&output[..], &expected[..], "XOF mismatch at input_len={}", case.input_len);
    }
}

#[test]
fn test_vectors_xof_incremental() {
    for case in TEST_CASES {
        let input = test_input(case.input_len);
        let expected = hex::decode(case.hash).unwrap();
        // Split right before the final partial word, and again at a small
        // prefix, so that `squeeze` has to carry state across both boundaries.
        for split in [128usize, 3, 130] {
            let mut output = vec![0u8; expected.len()];
            let mut hasher = Blake3::new();
            hasher.update(&input);
            let mut xof = hasher.finalize_xof();
            xof.squeeze(&mut output[..split]);
            xof.squeeze(&mut output[split..]);
            assert_eq!(
                &output[..],
                &expected[..],
                "XOF incremental mismatch at input_len={}, split={split}",
                case.input_len
            );
        }
    }
}

#[test]
fn test_vectors_xof_partial_word() {
    // Byte lengths that are not a multiple of 4 stress the final partial word
    // of the 64-byte output block, across a full block boundary.
    for &len in &[1usize, 2, 3, 5, 33, 63, 65, 129, 131] {
        for &size in &[0usize, 1, 64, 1024, 2048] {
            let input = test_input(size);
            let expected = hex::decode(
                TEST_CASES
                    .iter()
                    .find(|c| c.input_len == size)
                    .expect("size must be in TEST_CASES")
                    .hash,
            )
            .unwrap();

            let mut output = vec![0u8; len];
            let mut hasher = Blake3::new();
            hasher.update(&input);
            hasher.finalize_xof().squeeze(&mut output);
            assert_eq!(
                &output[..],
                &expected[..len],
                "XOF partial-word mismatch at input_len={size}, out_len={len}"
            );
        }
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Reference implementation
////////////////////////////////////////////////////////////////////////////////////////////////////

/// A small, deliberately straightforward and slow port of the public-domain
/// BLAKE3 reference implementation. Its only purpose is to be obviously
/// correct so that the optimized engine can be checked against it.
mod reference {
    use core::cmp::min;

    const OUT_LEN: usize = 32;
    const KEY_LEN: usize = 32;
    const BLOCK_LEN: usize = 64;
    const CHUNK_LEN: usize = 1024;

    const CHUNK_START: u32 = 1 << 0;
    const CHUNK_END: u32 = 1 << 1;
    const PARENT: u32 = 1 << 2;
    const ROOT: u32 = 1 << 3;
    const KEYED_HASH: u32 = 1 << 4;
    const DERIVE_KEY_CONTEXT: u32 = 1 << 5;
    const DERIVE_KEY_MATERIAL: u32 = 1 << 6;

    const IV: [u32; 8] = [
        0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
    ];

    const MSG_PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

    fn g(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
        state[a] = state[a].wrapping_add(state[b]).wrapping_add(mx);
        state[d] = (state[d] ^ state[a]).rotate_right(16);
        state[c] = state[c].wrapping_add(state[d]);
        state[b] = (state[b] ^ state[c]).rotate_right(12);
        state[a] = state[a].wrapping_add(state[b]).wrapping_add(my);
        state[d] = (state[d] ^ state[a]).rotate_right(8);
        state[c] = state[c].wrapping_add(state[d]);
        state[b] = (state[b] ^ state[c]).rotate_right(7);
    }

    fn round(state: &mut [u32; 16], m: &[u32; 16]) {
        g(state, 0, 4, 8, 12, m[0], m[1]);
        g(state, 1, 5, 9, 13, m[2], m[3]);
        g(state, 2, 6, 10, 14, m[4], m[5]);
        g(state, 3, 7, 11, 15, m[6], m[7]);
        g(state, 0, 5, 10, 15, m[8], m[9]);
        g(state, 1, 6, 11, 12, m[10], m[11]);
        g(state, 2, 7, 8, 13, m[12], m[13]);
        g(state, 3, 4, 9, 14, m[14], m[15]);
    }

    fn permute(m: &mut [u32; 16]) {
        let mut permuted = [0; 16];
        for (i, word) in permuted.iter_mut().enumerate() {
            *word = m[MSG_PERMUTATION[i]];
        }
        *m = permuted;
    }

    fn compress(
        chaining_value: &[u32; 8],
        block_words: &[u32; 16],
        counter: u64,
        block_len: u32,
        flags: u32,
    ) -> [u32; 16] {
        let counter_low = counter as u32;
        let counter_high = (counter >> 32) as u32;
        #[rustfmt::skip]
        let mut state = [
            chaining_value[0], chaining_value[1], chaining_value[2], chaining_value[3],
            chaining_value[4], chaining_value[5], chaining_value[6], chaining_value[7],
            IV[0], IV[1], IV[2], IV[3],
            counter_low, counter_high, block_len, flags,
        ];
        let mut block = *block_words;
        for r in 0..7 {
            round(&mut state, &block);
            if r < 6 {
                permute(&mut block);
            }
        }
        for i in 0..8 {
            state[i] ^= state[i + 8];
            state[i + 8] ^= chaining_value[i];
        }
        state
    }

    fn first_8_words(output: [u32; 16]) -> [u32; 8] {
        output[0..8].try_into().unwrap()
    }

    fn words_from_le_bytes(bytes: &[u8], words: &mut [u32]) {
        for (four_bytes, word) in bytes.as_chunks::<4>().0.iter().zip(words.iter_mut()) {
            *word = u32::from_le_bytes(*four_bytes);
        }
    }

    struct Output {
        input_chaining_value: [u32; 8],
        block_words: [u32; 16],
        counter: u64,
        block_len: u32,
        flags: u32,
    }

    impl Output {
        fn chaining_value(&self) -> [u32; 8] {
            first_8_words(compress(
                &self.input_chaining_value,
                &self.block_words,
                self.counter,
                self.block_len,
                self.flags,
            ))
        }

        /// Write output bytes starting at `position` in the extended stream.
        fn root_output_at(&self, position: u64, out: &mut [u8]) {
            let mut within = (position % BLOCK_LEN as u64) as usize;
            let mut block_counter = position / BLOCK_LEN as u64;
            let mut written = 0;
            while written < out.len() {
                let words = compress(
                    &self.input_chaining_value,
                    &self.block_words,
                    block_counter,
                    self.block_len,
                    self.flags | ROOT,
                );
                let mut block_bytes = [0u8; BLOCK_LEN];
                for (word, chunk) in words.iter().zip(block_bytes.as_chunks_mut::<4>().0) {
                    chunk.copy_from_slice(&word.to_le_bytes());
                }
                let take = min(out.len() - written, BLOCK_LEN - within);
                out[written..written + take].copy_from_slice(&block_bytes[within..within + take]);
                written += take;
                within = 0;
                block_counter += 1;
            }
        }
    }

    struct ChunkState {
        chaining_value: [u32; 8],
        chunk_counter: u64,
        block: [u8; BLOCK_LEN],
        block_len: u8,
        blocks_compressed: u8,
        flags: u32,
    }

    impl ChunkState {
        fn new(key_words: [u32; 8], chunk_counter: u64, flags: u32) -> Self {
            Self {
                chaining_value: key_words,
                chunk_counter,
                block: [0; BLOCK_LEN],
                block_len: 0,
                blocks_compressed: 0,
                flags,
            }
        }

        fn len(&self) -> usize {
            BLOCK_LEN * self.blocks_compressed as usize + self.block_len as usize
        }

        fn start_flag(&self) -> u32 {
            if self.blocks_compressed == 0 { CHUNK_START } else { 0 }
        }

        fn update(&mut self, mut input: &[u8]) {
            while !input.is_empty() {
                if self.block_len as usize == BLOCK_LEN {
                    let mut block_words = [0; 16];
                    words_from_le_bytes(&self.block, &mut block_words);
                    self.chaining_value = first_8_words(compress(
                        &self.chaining_value,
                        &block_words,
                        self.chunk_counter,
                        BLOCK_LEN as u32,
                        self.flags | self.start_flag(),
                    ));
                    self.blocks_compressed += 1;
                    self.block = [0; BLOCK_LEN];
                    self.block_len = 0;
                }
                let want = BLOCK_LEN - self.block_len as usize;
                let take = min(want, input.len());
                self.block[self.block_len as usize..][..take].copy_from_slice(&input[..take]);
                self.block_len += take as u8;
                input = &input[take..];
            }
        }

        fn output(&self) -> Output {
            let mut block_words = [0; 16];
            words_from_le_bytes(&self.block, &mut block_words);
            Output {
                input_chaining_value: self.chaining_value,
                block_words,
                counter: self.chunk_counter,
                block_len: self.block_len as u32,
                flags: self.flags | self.start_flag() | CHUNK_END,
            }
        }
    }

    fn parent_output(left: [u32; 8], right: [u32; 8], key_words: [u32; 8], flags: u32) -> Output {
        let mut block_words = [0; 16];
        block_words[..8].copy_from_slice(&left);
        block_words[8..].copy_from_slice(&right);
        Output {
            input_chaining_value: key_words,
            block_words,
            counter: 0,
            block_len: BLOCK_LEN as u32,
            flags: PARENT | flags,
        }
    }

    pub struct Hasher {
        chunk_state: ChunkState,
        key_words: [u32; 8],
        cv_stack: [[u32; 8]; 54],
        cv_stack_len: u8,
        flags: u32,
    }

    impl Hasher {
        fn new_internal(key_words: [u32; 8], flags: u32) -> Self {
            Self {
                chunk_state: ChunkState::new(key_words, 0, flags),
                key_words,
                cv_stack: [[0; 8]; 54],
                cv_stack_len: 0,
                flags,
            }
        }

        pub fn new() -> Self {
            Self::new_internal(IV, 0)
        }

        pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
            let mut key_words = [0; 8];
            words_from_le_bytes(key, &mut key_words);
            Self::new_internal(key_words, KEYED_HASH)
        }

        pub fn new_derive_key(context: &str) -> Self {
            let mut context_hasher = Self::new_internal(IV, DERIVE_KEY_CONTEXT);
            context_hasher.update(context.as_bytes());
            let mut context_key = [0; KEY_LEN];
            context_hasher.finalize(&mut context_key);
            let mut context_key_words = [0; 8];
            words_from_le_bytes(&context_key, &mut context_key_words);
            Self::new_internal(context_key_words, DERIVE_KEY_MATERIAL)
        }

        fn push_stack(&mut self, cv: [u32; 8]) {
            self.cv_stack[self.cv_stack_len as usize] = cv;
            self.cv_stack_len += 1;
        }

        fn pop_stack(&mut self) -> [u32; 8] {
            self.cv_stack_len -= 1;
            self.cv_stack[self.cv_stack_len as usize]
        }

        fn add_chunk_chaining_value(&mut self, mut new_cv: [u32; 8], mut total_chunks: u64) {
            while total_chunks & 1 == 0 {
                let left = self.pop_stack();
                new_cv = parent_output(left, new_cv, self.key_words, self.flags).chaining_value();
                total_chunks >>= 1;
            }
            self.push_stack(new_cv);
        }

        pub fn update(&mut self, mut input: &[u8]) {
            while !input.is_empty() {
                if self.chunk_state.len() == CHUNK_LEN {
                    let chunk_cv = self.chunk_state.output().chaining_value();
                    let total_chunks = self.chunk_state.chunk_counter + 1;
                    self.add_chunk_chaining_value(chunk_cv, total_chunks);
                    self.chunk_state = ChunkState::new(self.key_words, total_chunks, self.flags);
                }
                let want = CHUNK_LEN - self.chunk_state.len();
                let take = min(want, input.len());
                self.chunk_state.update(&input[..take]);
                input = &input[take..];
            }
        }

        pub fn finalize(&self, out_slice: &mut [u8]) {
            let mut output = self.chunk_state.output();
            let mut remaining = self.cv_stack_len as usize;
            while remaining > 0 {
                remaining -= 1;
                output = parent_output(self.cv_stack[remaining], output.chaining_value(), self.key_words, self.flags);
            }
            output.root_output_at(0, out_slice);
        }

        /// Like `finalize`, but starts the output stream at an arbitrary byte
        /// offset (used to check XOF seeking).
        pub fn finalize_at(&self, position: u64, out_slice: &mut [u8]) {
            let mut output = self.chunk_state.output();
            let mut remaining = self.cv_stack_len as usize;
            while remaining > 0 {
                remaining -= 1;
                output = parent_output(self.cv_stack[remaining], output.chaining_value(), self.key_words, self.flags);
            }
            output.root_output_at(position, out_slice);
        }
    }

    pub fn hash(input: &[u8]) -> [u8; OUT_LEN] {
        let mut hasher = Hasher::new();
        hasher.update(input);
        let mut out = [0; OUT_LEN];
        hasher.finalize(&mut out);
        out
    }

    pub fn keyed_hash(key: &[u8; KEY_LEN], input: &[u8]) -> [u8; OUT_LEN] {
        let mut hasher = Hasher::new_keyed(key);
        hasher.update(input);
        let mut out = [0; OUT_LEN];
        hasher.finalize(&mut out);
        out
    }

    pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; OUT_LEN] {
        let mut hasher = Hasher::new_derive_key(context);
        hasher.update(key_material);
        let mut out = [0; OUT_LEN];
        hasher.finalize(&mut out);
        out
    }
}

/// Deterministic pseudorandom bytes, so tests do not depend on `rand` and are
/// reproducible across platforms.
fn pseudo_random(len: usize, seed: u64) -> Vec<u8> {
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        out.push((state >> 24) as u8);
    }
    out
}

#[test]
fn differential_matches_reference() {
    // Brackets every block, chunk and SIMD-subtree boundary.
    let lengths = [
        0usize,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        31,
        32,
        33,
        63,
        64,
        65,
        96,
        127,
        128,
        129,
        191,
        255,
        256,
        257,
        511,
        512,
        513,
        1000,
        1023,
        1024,
        1025,
        1535,
        2047,
        2048,
        2049,
        3071,
        3072,
        3073,
        4095,
        4096,
        4097,
        5000,
        8191,
        8192,
        8193,
        12288,
        16383,
        16384,
        16385,
        31743,
        31744,
        31745,
        32768,
        65535,
        65536,
        65537,
        100_000,
        102_400,
        1 << 20,
    ];
    let key = pseudo_random(32, 0x1234);
    let key: [u8; 32] = key.try_into().unwrap();
    let context = "differential test context";

    for &len in &lengths {
        let input = pseudo_random(len, len as u64);

        assert_eq!(
            Blake3::hash(&input).as_ref(),
            &reference::hash(&input)[..],
            "plain hash mismatch at len {len}"
        );
        assert_eq!(
            Blake3::keyed_hash(&key, &input).as_ref(),
            &reference::keyed_hash(&key, &input)[..],
            "keyed hash mismatch at len {len}"
        );
        assert_eq!(
            &Blake3::derive_key(context, &input)[..],
            &reference::derive_key(context, &input)[..],
            "derive_key mismatch at len {len}"
        );
    }
}

#[test]
fn differential_incremental_matches_reference() {
    let lengths = [0usize, 1, 63, 64, 65, 128, 1023, 1024, 1025, 2048, 3000, 8192, 20000];
    let key: [u8; 32] = pseudo_random(32, 9).try_into().unwrap();

    for &len in &lengths {
        let input = pseudo_random(len, len as u64 ^ 0xABCD);

        // Feed the input in awkward pieces: one byte at a time, and in
        // pseudorandom chunks, interleaving empty updates.
        for step in 0..3 {
            let mut expected = reference::Hasher::new();
            let mut actual = Blake3::new();
            let mut expected_keyed = reference::Hasher::new_keyed(&key);
            let mut actual_keyed = Blake3::new_keyed(&key);

            let mut offset = 0;
            while offset < input.len() {
                let chunk = match step {
                    0 => 1,
                    1 => 13,
                    _ => 1 + (offset * 7 + 3) % 100,
                };
                let end = core::cmp::min(offset + chunk, input.len());

                expected.update(&input[offset..end]);
                actual.update(&input[offset..end]);
                expected_keyed.update(&input[offset..end]);
                actual_keyed.update(&input[offset..end]);
                offset = end;

                // Empty updates must be no-ops.
                expected.update(&[]);
                actual.update(&[]);
            }

            let mut expected_out = [0u8; 32];
            expected.finalize(&mut expected_out);
            assert_eq!(actual.sum().as_ref(), &expected_out, "incremental len {len} step {step}");

            let mut expected_keyed_out = [0u8; 32];
            expected_keyed.finalize(&mut expected_keyed_out);
            assert_eq!(
                actual_keyed.sum().as_ref(),
                &expected_keyed_out,
                "incremental keyed len {len} step {step}"
            );
        }
    }
}

#[test]
fn differential_xof_seek_matches_reference() {
    for len in [0usize, 1, 64, 1024, 2000, 5000] {
        let input = pseudo_random(len, len as u64);
        let mut hasher = super::hasher::Hasher::new();
        hasher.update(&input);
        let mut ref_hasher = reference::Hasher::new();
        ref_hasher.update(&input);

        // 0, a block boundary, an unaligned offset, and a huge (but reachable)
        // offset to exercise the 64-bit output block counter.
        for &position in &[0u64, 32, 63, 64, 100, 4096, 1 << 20, 1 << 40] {
            let mut xof = hasher.finalize_xof();
            xof.set_position(position);
            let mut actual = [0u8; 100];
            xof.fill(&mut actual);
            assert_eq!(xof.position(), position + 100, "position tracking");

            let mut expected = [0u8; 100];
            ref_hasher.finalize_at(position, &mut expected);
            assert_eq!(actual, expected, "xof seek mismatch len {len} pos {position}");
        }

        // Filling in many small pieces must be identical to one big fill.
        let mut xof_a = hasher.finalize_xof();
        let mut one_shot = [0u8; 300];
        xof_a.fill(&mut one_shot);
        let mut xof_b = hasher.finalize_xof();
        let mut piecemeal = [0u8; 300];
        for piece in piecemeal.chunks_mut(7) {
            xof_b.fill(piece);
        }
        assert_eq!(one_shot, piecemeal, "piecemeal xof mismatch len {len}");
    }
}

/// Every possible two-way split must agree with the all-at-once reference
/// hash, and (for small inputs) every three-way split too.
#[test]
fn differential_all_split_points() {
    let lengths = [
        0usize, 1, 2, 3, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 511, 512, 1023, 1024, 1025, 1535, 2047, 2048,
        2049, 3000,
    ];
    let key: [u8; 32] = pseudo_random(32, 0x5151).try_into().unwrap();

    for &len in &lengths {
        let input = pseudo_random(len, 0x9000 ^ len as u64);
        let expected = reference::hash(&input);
        let expected_keyed = reference::keyed_hash(&key, &input);

        for split in 0..=len {
            let mut hasher = Blake3::new();
            hasher.update(&input[..split]);
            hasher.update(&input[split..]);
            assert_eq!(hasher.sum().as_ref(), &expected[..], "split {split} of {len}");

            let mut hasher = Blake3::new_keyed(&key);
            hasher.update(&input[..split]);
            hasher.update(&input[split..]);
            assert_eq!(hasher.sum().as_ref(), &expected_keyed[..], "keyed split {split} of {len}");
        }
    }

    // Three-way splits for lengths that cross a chunk boundary.
    for &len in &[0usize, 1, 64, 65, 127, 128, 129] {
        let input = pseudo_random(len, 0x1000 ^ len as u64);
        let expected = reference::hash(&input);
        for a in 0..=len {
            for b in a..=len {
                let mut hasher = Blake3::new();
                hasher.update(&input[..a]);
                hasher.update(&input[a..b]);
                hasher.update(&input[b..]);
                assert_eq!(hasher.sum().as_ref(), &expected[..], "splits {a}/{b} of {len}");
            }
        }
    }
}

/// Deterministic pseudorandom update patterns, checking both plain and keyed hashing.
#[test]
fn differential_fuzz_incremental() {
    let mut state = 0x1234_5678_9abc_def0u64;
    let mut rng = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };

    let key: [u8; 32] = pseudo_random(32, 42).try_into().unwrap();
    let max_len = 20_000usize;
    let data = pseudo_random(max_len, 0xF00D);

    for trial in 0..300 {
        let total = (rng() as usize) % (max_len + 1);
        let input = &data[..total];

        let mut actual = Blake3::new();
        let mut actual_keyed = Blake3::new_keyed(&key);
        let mut offset = 0;
        while offset < input.len() {
            // Empty updates must be no-ops.
            actual.update(&[]);
            actual_keyed.update(&[]);
            let remaining = input.len() - offset;
            let step = 1 + (rng() as usize) % remaining;
            let end = offset + step;
            actual.update(&input[offset..end]);
            actual_keyed.update(&input[offset..end]);
            offset = end;
        }

        assert_eq!(actual.sum().as_ref(), &reference::hash(input)[..], "fuzz hash trial {trial}");
        assert_eq!(
            actual_keyed.sum().as_ref(),
            &reference::keyed_hash(&key, input)[..],
            "fuzz keyed hash trial {trial}"
        );
    }
}

/// Pseudorandom piecemeal squeezing and seeking against the reference XOF.
#[test]
fn differential_fuzz_xof() {
    let mut state = 0x0dd0_1234_5678_9abcu64;
    let mut rng = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };

    let data = pseudo_random(4096, 0xC0FFEE);
    for trial in 0..200 {
        let total = (rng() as usize) % (data.len() + 1);
        let input = &data[..total];
        let position = rng() % (1 << 20);

        let mut hasher = super::hasher::Hasher::new();
        hasher.update(input);
        let mut ref_hasher = reference::Hasher::new();
        ref_hasher.update(input);

        let mut expected = [0u8; 517];
        ref_hasher.finalize_at(position, &mut expected);

        let mut xof = hasher.finalize_xof();
        xof.set_position(position);
        let mut actual = [0u8; 517];
        let mut offset = 0;
        while offset < actual.len() {
            let step = 1 + (rng() % 80) as usize;
            let end = (offset + step).min(actual.len());
            xof.fill(&mut actual[offset..end]);
            offset = end;
        }
        assert_eq!(actual, expected, "fuzz xof trial {trial}");
    }
}

/// `squeeze` must write exactly the requested number of bytes, never touching
/// the surrounding memory (an output-block-width over-write is a known bug
/// class in XOF implementations).
#[test]
fn xof_squeeze_does_not_overwrite() {
    let mut hasher = Blake3::new();
    hasher.update(&pseudo_random(1234, 7));
    let mut xof = hasher.finalize_xof();

    for &len in &[0usize, 1, 2, 3, 31, 63, 64, 65, 127] {
        let mut buf = [0xAAu8; 200];
        xof.squeeze(&mut buf[8..8 + len]);
        assert!(
            buf[..8].iter().all(|&b| b == 0xAA),
            "squeeze wrote before the slice (len {len})"
        );
        assert!(
            buf[8 + len..].iter().all(|&b| b == 0xAA),
            "squeeze wrote past the slice (len {len})"
        );
    }
}

/// Every chunking of the same output stream must produce the same bytes. The
/// reader caches the unread part of the current output block; this sweep covers
/// every total length across the 64-byte boundaries (and across a full SIMD
/// pass) with chunk sizes that split, straddle and coincide with them.
#[test]
fn xof_chunked_matches_one_shot() {
    let input = pseudo_random(2000, 0x1234_5678);
    let totals = (0..=2 * super::platform::BLOCK_LEN + 8)
        .chain([255, 256, 257, 511, 512, 513, 1023, 1024, 1025, 4096, 4097, 8192])
        .collect::<Vec<_>>();

    for &total in &totals {
        let mut expected = vec![0u8; total];
        let mut h = Blake3::new();
        h.update(&input);
        h.finalize_xof().squeeze(&mut expected);

        for &chunk in &[1usize, 2, 3, 5, 62, 63, 64, 65, 66, 127, 128, 129, 200, 1000] {
            let mut h = Blake3::new();
            h.update(&input);
            let mut xof = h.finalize_xof();
            let mut out = vec![0u8; total];
            let mut offset = 0;
            while offset < total {
                let end = (offset + chunk).min(total);
                xof.squeeze(&mut out[offset..end]);
                offset = end;
            }
            assert_eq!(out, expected, "total={total} chunk={chunk}");
        }
    }
}

/// Reading one byte at a time must match a single read, for both plain and
/// keyed hashing. (This is the pattern the previous, unbuffered reader handled
/// by recomputing the output block on every call.)
#[test]
fn xof_one_byte_at_a_time_matches_one_shot() {
    for &keyed in &[false, true] {
        let input = pseudo_random(200, 0x5EED ^ u64::from(keyed));
        let build = || {
            let mut h = if keyed {
                Blake3::new_keyed(TEST_KEY)
            } else {
                Blake3::new()
            };
            h.update(&input);
            h
        };

        let mut expected = [0u8; 1024];
        build().finalize_xof().squeeze(&mut expected);

        let mut xof = build().finalize_xof();
        let mut byte = [0u8; 1];
        for (i, &want) in expected.iter().enumerate() {
            xof.squeeze(&mut byte);
            assert_eq!(byte[0], want, "keyed={keyed}, byte {i}");
        }
    }
}

/// Empty squeezes are no-ops wherever they occur, including between two partial
/// reads that share a cached block.
#[test]
fn xof_empty_squeeze_is_noop() {
    let input = pseudo_random(64, 0xABC);
    let mut expected = [0u8; 256];
    let mut h = Blake3::new();
    h.update(&input);
    h.finalize_xof().squeeze(&mut expected);

    let mut h = Blake3::new();
    h.update(&input);
    let mut xof = h.finalize_xof();
    let mut out = [0u8; 256];
    let steps = [0usize, 1, 0, 31, 0, 0, 32, 0, 65, 0, 127, 0];
    let mut offset = 0;
    for &step in &steps {
        let end = (offset + step).min(out.len());
        xof.squeeze(&mut out[offset..end]);
        offset = end;
    }
    assert_eq!(offset, out.len(), "steps must cover the whole buffer");
    assert_eq!(out, expected);
}

/// A single call that starts inside the cached block, crosses whole blocks and
/// ends in a fresh partial block must stitch the three regions correctly.
#[test]
fn xof_read_spans_cache_and_whole_blocks() {
    let input = pseudo_random(77, 0xBEEF);
    let mut expected = [0u8; 512];
    let mut h = Blake3::new();
    h.update(&input);
    h.finalize_xof().squeeze(&mut expected);

    for prefix in 0..super::platform::BLOCK_LEN {
        for &len in &[
            1usize,
            62,
            63,
            64,
            65,
            127,
            128,
            129,
            190,
            200,
            255,
            256,
            512 - super::platform::BLOCK_LEN,
        ] {
            assert!(prefix + len <= expected.len());

            let mut h = Blake3::new();
            h.update(&input);
            let mut xof = h.finalize_xof();
            let mut out = [0u8; 512];
            xof.squeeze(&mut out[..prefix]);
            xof.squeeze(&mut out[prefix..prefix + len]);

            assert_eq!(out[..prefix + len], expected[..prefix + len], "prefix={prefix} len={len}");
        }
    }
}

/// Seeking must refresh the cache: bytes read after a seek continue from the new
/// position, never from a block cached before the seek.
#[test]
fn xof_seek_refreshes_cache() {
    let input = pseudo_random(300, 0x9001);
    let mut hasher = super::hasher::Hasher::new();
    hasher.update(&input);
    let mut reference_hasher = reference::Hasher::new();
    reference_hasher.update(&input);

    for &position in &[0u64, 1, 63, 64, 65, 127, 128, 129, 512, 1 << 20] {
        let mut expected = [0u8; 300];
        reference_hasher.finalize_at(position, &mut expected);

        let mut xof = hasher.finalize_xof();
        xof.set_position(position);
        let mut actual = [0u8; 300];
        // Odd-sized reads straddle the cache and the wide path.
        let mut offset = 0;
        for &step in &[1usize, 62, 1, 63, 1, 72] {
            let end = (offset + step).min(actual.len());
            xof.fill(&mut actual[offset..end]);
            offset = end;
        }
        xof.fill(&mut actual[offset..]);
        assert_eq!(actual, expected, "seek to {position}");
    }
}

/// Zeroization must clear the cached output block, not just the root state: the
/// cached bytes are key-derived for keyed hashing.
#[cfg(feature = "zeroize")]
#[test]
fn xof_zeroize_clears_cached_block() {
    let build = || {
        let mut h = super::hasher::Hasher::new_keyed(TEST_KEY);
        h.update(b"key material");
        h
    };

    // (a) partially consumed block
    let mut reader = build().finalize_xof();
    let mut out = [0u8; 33];
    reader.fill(&mut out);
    reader.zeroize();
    assert!(reader.debug_is_zeroized(), "partially consumed cached block was not zeroized");

    // (b) exactly on a block boundary (the cache is stale but still holds output)
    let mut reader = build().finalize_xof();
    let mut out = [0u8; 64];
    reader.fill(&mut out);
    reader.zeroize();
    assert!(reader.debug_is_zeroized(), "block-boundary state was not zeroized");

    // (c) after a seek, which refreshes the cache eagerly
    let mut reader = build().finalize_xof();
    reader.set_position(100);
    let mut out = [0u8; 4];
    reader.fill(&mut out);
    reader.zeroize();
    assert!(reader.debug_is_zeroized(), "post-seek state was not zeroized");
}

/// Input slices at every alignment, so the SIMD kernels' unaligned loads are
/// exercised (they must never assume 16/32-byte alignment).
#[test]
fn differential_unaligned_inputs() {
    for &len in &[0usize, 1, 63, 64, 65, 1024, 1025, 3000] {
        let backing = pseudo_random(len + 64, 0xABCD ^ len as u64);
        for offset in 0..64 {
            let input = &backing[offset..offset + len];
            assert_eq!(
                Blake3::hash(input).as_ref(),
                &reference::hash(input)[..],
                "unaligned mismatch at len {len}, offset {offset}"
            );
        }
    }
}

/// The message schedule constant must be exactly the schedule generated by
/// repeatedly applying the BLAKE3 permutation.
#[test]
fn msg_schedule_matches_permutation() {
    use super::platform::MSG_SCHEDULE;

    const PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

    let mut generated = [[0usize; 16]; 7];
    let mut row = core::array::from_fn::<usize, 16, _>(|i| i);
    for round in &mut generated {
        *round = row;
        row = core::array::from_fn(|i| row[PERMUTATION[i]]);
    }
    assert_eq!(generated, MSG_SCHEDULE);
}

/// Table test for the "largest power of two <= n" helper, including the
/// `usize::MAX` boundary that would overflow a naive shift.
#[test]
fn largest_power_of_two_leq_table() {
    use super::hasher::largest_power_of_two_leq;

    let cases = [
        (0usize, 1usize),
        (1, 1),
        (2, 2),
        (3, 2),
        (4, 4),
        (5, 4),
        (6, 4),
        (7, 4),
        (8, 8),
        (9, 8),
        (15, 8),
        (16, 16),
        (17, 16),
        (1023, 512),
        (1024, 1024),
        (1025, 1024),
    ];
    for (n, want) in cases {
        assert_eq!(largest_power_of_two_leq(n), want, "n = {n}");
    }
    let max = usize::MAX;
    assert_eq!(largest_power_of_two_leq(max), (max >> 1) + 1);
}

/// Table test for the subtree split point, which must leave at least one byte
/// on the right and only ever cut on a chunk boundary.
#[test]
fn left_subtree_len_table() {
    use super::hasher::hazmat_left_subtree_len;

    let cases = [
        (1025u64, 1024u64),
        (2048, 1024),
        (2049, 2048),
        (3072, 2048),
        (4096, 2048),
        (4097, 4096),
        (8192, 4096),
        (1 << 20, 1 << 19),
    ];
    for (content_len, want) in cases {
        let got = hazmat_left_subtree_len(content_len);
        assert_eq!(got, want, "content_len = {content_len}");
        assert!(got.is_multiple_of(1024), "split must be on a chunk boundary");
        assert!(got < content_len, "right subtree must be non-empty");
    }
}

#[test]
fn zero_length_update_is_noop() {
    let mut hasher = Blake3::new();
    hasher.update(&[]);
    hasher.update(&[]);
    assert_eq!(hasher.sum().as_ref(), Blake3::hash(&[]).as_ref());
}

/// The block buffers must be cache-line aligned on native targets: the
/// single-chunk path fills one with a `memcpy` and immediately reads it back,
/// which only forwards between store and load when the addresses line up the
/// same way (see `Aligned64`). Without this, `Blake3::hash` on 64-byte inputs
/// varies with the stack layout by up to ~40% on affected x86 parts.
///
/// WebAssembly is excluded: it has no store-forwarding penalty and the
/// alignment is a measurable slowdown there.
#[test]
fn block_buffer_alignment() {
    use core::mem::align_of;
    if cfg!(target_arch = "wasm32") {
        return;
    }
    assert_eq!(align_of::<super::hasher::Aligned64>(), 64);
    assert_eq!(size_of::<super::hasher::Aligned64>(), 64);
}

#[test]
fn send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Blake3>();
    assert_send_sync::<Blake3XofReader>();
    assert_send_sync::<Hash>();
}

#[cfg(feature = "zeroize")]
#[test]
fn zeroize_clears_state() {
    let key = [0x42u8; 32];
    let mut hasher = super::hasher::Hasher::new_keyed(&key);
    hasher.update(&pseudo_random(5000, 7));
    hasher.zeroize();
    assert!(hasher.debug_is_zeroized(), "hasher state was not fully zeroized");

    let mut hasher = super::hasher::Hasher::new();
    hasher.update(b"some data");
    let mut reader = hasher.finalize_xof();
    let mut out = [0u8; 64];
    reader.fill(&mut out);
    reader.zeroize();
    assert!(reader.debug_is_zeroized(), "xof reader state was not fully zeroized");
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// SIMD kernel parity
////////////////////////////////////////////////////////////////////////////////////////////////////

/// The SIMD backends compiled into this build. The portable backend is always
/// first (it is the reference), the rest are appended when available.
#[cfg(any(
    all(
        target_arch = "x86_64",
        any(feature = "std", target_feature = "avx2", target_feature = "avx512f")
    ),
    all(target_arch = "aarch64", target_feature = "neon"),
    all(target_arch = "wasm32", target_feature = "simd128"),
))]
#[allow(clippy::vec_init_then_push)]
fn available_simd_platforms() -> Vec<super::platform::Platform> {
    #[allow(unused_mut)]
    let mut platforms = Vec::new();

    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    if is_x86_feature_detected!("avx512f") {
        platforms.push(super::platform::Platform::Avx512);
    }
    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    if is_x86_feature_detected!("avx2") {
        platforms.push(super::platform::Platform::Avx2);
    }
    #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx512f"))]
    platforms.push(super::platform::Platform::Avx512);
    #[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx2"))]
    platforms.push(super::platform::Platform::Avx2);

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    platforms.push(super::platform::Platform::Neon);

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    platforms.push(super::platform::Platform::Wasm32Simd);

    platforms
}

#[cfg(any(
    all(
        target_arch = "x86_64",
        any(feature = "std", target_feature = "avx2", target_feature = "avx512f")
    ),
    all(target_arch = "aarch64", target_feature = "neon"),
    all(target_arch = "wasm32", target_feature = "simd128"),
))]
fn bytes_to_words<const N: usize>(bytes: &[u8]) -> [u32; N] {
    let mut words = [0u32; N];
    for (word, chunk) in words.iter_mut().zip(bytes.as_chunks::<4>().0) {
        *word = u32::from_le_bytes(*chunk);
    }
    words
}

#[cfg(any(
    all(
        target_arch = "x86_64",
        any(feature = "std", target_feature = "avx2", target_feature = "avx512f")
    ),
    all(target_arch = "aarch64", target_feature = "neon"),
    all(target_arch = "wasm32", target_feature = "simd128"),
))]
#[test]
fn simd_kernels_match_portable() {
    use super::platform::{
        BLOCK_LEN, CHUNK_END, CHUNK_START, DERIVE_KEY_MATERIAL, IncrementCounter, KEYED_HASH, PARENT, Platform, ROOT,
    };

    // Counters chosen so that the per-lane increment carries out of the low
    // 32-bit word (`u32::MAX` and neighbours), stays just below it
    // (`i32::MAX`), and starts inside the high word.
    let counters = [
        0u64,
        1,
        7,
        i32::MAX as u64,
        u32::MAX as u64,
        (u32::MAX as u64) - 3,
        // Carries out of the low 32-bit word *inside* a 16-lane AVX-512 group
        // (the arrangement that exposed BLAKE3 issue #271).
        (u32::MAX as u64) - 15,
        (0xDEAD_BEEF_u64 << 32) | 5,
    ];
    // Flags combinations that exercise the "end"/"root"/keyed bit handling.
    let flags = [0u8, KEYED_HASH, DERIVE_KEY_MATERIAL, CHUNK_END | ROOT | KEYED_HASH];

    for platform in available_simd_platforms() {
        // --- single-block compression, including a partial final block ---
        for &block_len in &[1u8, 31, 32, 61, 63, 64] {
            for &counter in &counters {
                for &flags in &flags {
                    let cv = bytes_to_words::<8>(&pseudo_random(32, 1000 + block_len as u64));
                    let block_bytes = pseudo_random(64, 2000 + block_len as u64);
                    let block: [u8; 64] = block_bytes.as_slice().try_into().unwrap();

                    let mut cv_ref = cv;
                    let mut cv_simd = cv;
                    Platform::Portable.compress_in_place(&mut cv_ref, &block, block_len, counter, flags);
                    platform.compress_in_place(&mut cv_simd, &block, block_len, counter, flags);
                    assert_eq!(
                        cv_ref, cv_simd,
                        "compress_in_place mismatch ({platform:?}, block_len {block_len}, counter {counter}, flags {flags})"
                    );

                    let xof_ref = Platform::Portable.compress_xof(&cv, &block, block_len, counter, flags);
                    let xof_simd = platform.compress_xof(&cv, &block, block_len, counter, flags);
                    assert_eq!(
                        xof_ref, xof_simd,
                        "compress_xof mismatch ({platform:?}, block_len {block_len}, counter {counter}, flags {flags})"
                    );
                }
            }
        }

        let key = bytes_to_words::<8>(&pseudo_random(32, 77));

        // --- parallel chunk hashing (16 blocks per input). The counts cover
        //     full SIMD groups plus scalar tails (33 = 16x2 + 1). ---
        for &n in &[
            1usize, 2, 3, 4, 5, 6, 7, 8, 9, 12, 15, 16, 17, 23, 24, 31, 32, 33, 47, 48,
        ] {
            let inputs_owned: Vec<Vec<u8>> = (0..n).map(|i| pseudo_random(1024, 5000 + i as u64)).collect();
            let inputs: Vec<&[u8]> = inputs_owned.iter().map(Vec::as_slice).collect();
            for &counter in &counters {
                let mut out_ref = vec![0u8; n * 32];
                let mut out_simd = vec![0u8; n * 32];
                Platform::Portable.hash_many(
                    &inputs,
                    &key,
                    counter,
                    IncrementCounter::Yes,
                    0,
                    CHUNK_START,
                    CHUNK_END,
                    &mut out_ref,
                );
                platform.hash_many(
                    &inputs,
                    &key,
                    counter,
                    IncrementCounter::Yes,
                    0,
                    CHUNK_START,
                    CHUNK_END,
                    &mut out_simd,
                );
                assert_eq!(
                    out_ref, out_simd,
                    "chunk hash_many mismatch ({platform:?}, n {n}, counter {counter})"
                );
            }
        }

        // --- parallel parent hashing (one block per input), counter not incremented ---
        for &n in &[2usize, 3, 4, 5, 7, 8, 9, 16, 17, 31, 32, 33] {
            let inputs_owned: Vec<Vec<u8>> = (0..n).map(|i| pseudo_random(64, 9000 + i as u64)).collect();
            let inputs: Vec<&[u8]> = inputs_owned.iter().map(Vec::as_slice).collect();
            let mut out_ref = vec![0u8; n * 32];
            let mut out_simd = vec![0u8; n * 32];
            Platform::Portable.hash_many(&inputs, &key, 0, IncrementCounter::No, PARENT, 0, 0, &mut out_ref);
            platform.hash_many(&inputs, &key, 0, IncrementCounter::No, PARENT, 0, 0, &mut out_simd);
            assert_eq!(out_ref, out_simd, "parent hash_many mismatch ({platform:?}, n {n})");
        }

        // --- wide XOF output: several 64-byte blocks produced at once ---
        for &blocks in &[1usize, 2, 3, 4, 5, 7, 8, 15, 16, 17, 31, 32, 33] {
            for &block_len in &[1u8, 64] {
                for &counter in &counters {
                    for &flags in &[ROOT | CHUNK_END, ROOT | CHUNK_END | KEYED_HASH] {
                        let cv = bytes_to_words::<8>(&pseudo_random(32, 4242 + block_len as u64));
                        let block_bytes = pseudo_random(64, 4242 + block_len as u64);
                        let block: [u8; 64] = block_bytes.as_slice().try_into().unwrap();

                        let mut out_ref = vec![0u8; blocks * BLOCK_LEN];
                        for (i, out_block) in out_ref.as_chunks_mut::<BLOCK_LEN>().0.iter_mut().enumerate() {
                            out_block.copy_from_slice(&Platform::Portable.compress_xof(
                                &cv,
                                &block,
                                block_len,
                                counter.wrapping_add(i as u64),
                                flags,
                            ));
                        }

                        let mut out_simd = vec![0u8; blocks * BLOCK_LEN];
                        platform.xof_many(&cv, &block, block_len, counter, flags, &mut out_simd);
                        assert_eq!(
                            out_ref, out_simd,
                            "xof_many mismatch ({platform:?}, blocks {blocks}, block_len {block_len}, counter {counter}, flags {flags})"
                        );
                    }
                }
            }
        }

        // An empty output must be a no-op: several backends can only write a
        // whole SIMD group at a time, so a shorter slice must be rejected by
        // the contract rather than written out of bounds.
        platform.xof_many(&key, &[0u8; BLOCK_LEN], BLOCK_LEN as u8, 0, ROOT, &mut []);
    }
}

#[test]
fn derive_key_context_edge_cases() {
    let material = pseudo_random(3000, 0xC0FFEE);

    // Empty, multi-byte UTF-8, and very long contexts must not panic and must
    // agree with the reference implementation.
    let long_context = "x".repeat(10_000);
    let contexts: [&str; 3] = ["", "clé 🔑 derived", long_context.as_str()];

    for context in contexts {
        assert_eq!(
            &Blake3::derive_key(context, &material)[..],
            &reference::derive_key(context, &material)[..],
            "derive_key mismatch for context of len {}",
            context.len()
        );

        let mut hasher = Blake3::new_derive_key(context);
        hasher.update(&material);
        let mut ref_hasher = reference::Hasher::new_derive_key(context);
        ref_hasher.update(&material);
        let mut expected = [0u8; 32];
        ref_hasher.finalize(&mut expected);
        assert_eq!(hasher.sum().as_ref(), &expected[..]);
    }
}

#[test]
fn xof_seek_near_u64_max_does_not_panic() {
    let input = pseudo_random(2048, 0xBEEF);
    let mut hasher = super::hasher::Hasher::new();
    hasher.update(&input);
    let mut ref_hasher = reference::Hasher::new();
    ref_hasher.update(&input);

    // A position just below the 64-bit limit exercises the high output-block
    // counter; nothing must overflow or panic.
    let position = u64::MAX - 128;
    let mut xof = hasher.finalize_xof();
    xof.set_position(position);
    let mut actual = [0u8; 64];
    xof.fill(&mut actual);

    let mut expected = [0u8; 64];
    ref_hasher.finalize_at(position, &mut expected);
    assert_eq!(actual, expected, "xof output near u64::MAX mismatch");

    // Filling beyond the 2^64-byte limit must not panic either (the behaviour
    // is unspecified, but it must be memory-safe).
    let mut beyond = [0u8; 64];
    xof.fill(&mut beyond);
}
