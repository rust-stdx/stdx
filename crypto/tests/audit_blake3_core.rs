//! Tests for the BLAKE3 core engine.
//!
//! This file is self-contained: it embeds a slow, deliberately simple BLAKE3
//! implementation written directly from the specification (recursive binary
//! tree over chunk ranges, no lazy-merge stack) and compares the public API
//! against it. It also checks the official BLAKE3 test vectors.
//!
//! Covered:
//!
//! * every input length in `0..=2048`, plus 4095/4096/4097, 8191/8192/8193,
//!   1 MiB and the official-vector lengths;
//! * every two-way split of the input at every offset (300-byte input and
//!   selected chunk-boundary lengths), every three-way split of a 300-byte
//!   input, and deterministic pseudorandom split patterns for larger inputs;
//! * one-shot vs incremental vs `Clone`-then-diverge;
//! * XOF output, piecemeal squeezing, empty squeezes, and canary checks;
//! * hash / keyed hash / derive key against the official vectors;
//! * end-of-page input buffers, so any read past the final byte of the input
//!   (a plausible SIMD over-read) faults instead of silently succeeding.
//!
//! Run with `cargo test -p crypto --test audit_blake3_core`.

use crypto::{Hasher, Xof, blake3::Blake3};
use rand::{Rng, RngExt, SeedableRng, rngs::StdRng};

////////////////////////////////////////////////////////////////////////////////
// Independent reference implementation (written from the BLAKE3 specification)
////////////////////////////////////////////////////////////////////////////////
mod reference {
    const BLOCK_LEN: usize = 64;
    const CHUNK_LEN: usize = 1024;

    const CHUNK_START: u32 = 1 << 0;
    const CHUNK_END: u32 = 1 << 1;
    const PARENT: u32 = 1 << 2;
    const ROOT: u32 = 1 << 3;
    pub const KEYED_HASH: u32 = 1 << 4;
    pub const DERIVE_KEY_CONTEXT: u32 = 1 << 5;
    pub const DERIVE_KEY_MATERIAL: u32 = 1 << 6;

    pub const IV: [u32; 8] = [
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

    fn block_words(block: &[u8; BLOCK_LEN]) -> [u32; 16] {
        let mut words = [0u32; 16];
        for (word, bytes) in words.iter_mut().zip(block.as_chunks::<4>().0) {
            *word = u32::from_le_bytes(*bytes);
        }
        words
    }

    fn compress(cv: &[u32; 8], block: &[u8; BLOCK_LEN], counter: u64, block_len: u32, flags: u32) -> [u32; 16] {
        let mut m = block_words(block);
        #[rustfmt::skip]
        let mut v = [
            cv[0], cv[1], cv[2], cv[3], cv[4], cv[5], cv[6], cv[7],
            IV[0], IV[1], IV[2], IV[3],
            counter as u32, (counter >> 32) as u32, block_len, flags,
        ];
        for r in 0..7 {
            round(&mut v, &m);
            if r < 6 {
                let mut permuted = [0u32; 16];
                for (i, word) in permuted.iter_mut().enumerate() {
                    *word = m[MSG_PERMUTATION[i]];
                }
                m = permuted;
            }
        }
        for i in 0..8 {
            v[i] ^= v[i + 8];
            v[i + 8] ^= cv[i];
        }
        v
    }

    fn first8(words: [u32; 16]) -> [u32; 8] {
        words[..8].try_into().unwrap()
    }

    fn words_to_bytes(words: &[u32; 16]) -> [u8; BLOCK_LEN] {
        let mut out = [0u8; BLOCK_LEN];
        for (word, bytes) in words.iter().zip(out.as_chunks_mut::<4>().0) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        out
    }

    fn cv_to_bytes(cv: &[u32; 8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        for (word, bytes) in cv.iter().zip(out.as_chunks_mut::<4>().0) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        out
    }

    pub fn key_words(key: &[u8; 32]) -> [u32; 8] {
        let mut words = [0u32; 8];
        for (word, bytes) in words.iter_mut().zip(key.as_chunks::<4>().0) {
            *word = u32::from_le_bytes(*bytes);
        }
        words
    }

    /// The node produced by hashing one chunk. The final block is kept
    /// uncompressed so that `CHUNK_END`/`ROOT` can still be added.
    pub struct Node {
        cv: [u32; 8],
        block: [u8; BLOCK_LEN],
        block_len: u32,
        counter: u64,
        flags: u32,
    }

    impl Node {
        fn chaining_value(&self) -> [u32; 8] {
            first8(compress(&self.cv, &self.block, self.counter, self.block_len, self.flags))
        }

        fn root_bytes(&self, position: u64, out: &mut [u8]) {
            let mut within = (position % BLOCK_LEN as u64) as usize;
            let mut block_counter = position / BLOCK_LEN as u64;
            let mut written = 0;
            while written < out.len() {
                let bytes = words_to_bytes(&compress(
                    &self.cv,
                    &self.block,
                    block_counter,
                    self.block_len,
                    self.flags | ROOT,
                ));
                let take = core::cmp::min(out.len() - written, BLOCK_LEN - within);
                out[written..written + take].copy_from_slice(&bytes[within..within + take]);
                written += take;
                within = 0;
                block_counter = block_counter.wrapping_add(1);
            }
        }
    }

    fn chunk_node(chunk: &[u8], counter: u64, key: &[u32; 8], flags: u32) -> Node {
        assert!(chunk.len() <= CHUNK_LEN);
        let mut cv = *key;
        let mut offset = 0;
        // Compress all but the last block.
        while chunk.len() - offset > BLOCK_LEN {
            let block: &[u8; BLOCK_LEN] = chunk[offset..offset + BLOCK_LEN].try_into().unwrap();
            let block_flags = flags | if offset == 0 { CHUNK_START } else { 0 };
            cv = first8(compress(&cv, block, counter, BLOCK_LEN as u32, block_flags));
            offset += BLOCK_LEN;
        }
        let mut block = [0u8; BLOCK_LEN];
        block[..chunk.len() - offset].copy_from_slice(&chunk[offset..]);
        Node {
            cv,
            block,
            block_len: (chunk.len() - offset) as u32,
            counter,
            flags: flags | CHUNK_END | if offset == 0 { CHUNK_START } else { 0 },
        }
    }

    fn parent_node(left: [u32; 8], right: [u32; 8], key: &[u32; 8], flags: u32) -> Node {
        let mut block = [0u8; BLOCK_LEN];
        block[..32].copy_from_slice(&cv_to_bytes(&left));
        block[32..].copy_from_slice(&cv_to_bytes(&right));
        Node {
            cv: *key,
            block,
            block_len: BLOCK_LEN as u32,
            counter: 0,
            flags: flags | PARENT,
        }
    }

    /// The largest power of two strictly less than `n` (`n >= 2`).
    fn largest_pow2_below(n: u64) -> u64 {
        1u64 << (63 - (n - 1).leading_zeros())
    }

    /// Hash a non-empty subtree using the canonical tree shape: the left
    /// subtree is the largest power-of-two number of chunks strictly smaller
    /// than the whole subtree.
    fn subtree_node(input: &[u8], counter: u64, key: &[u32; 8], flags: u32) -> Node {
        if input.len() <= CHUNK_LEN {
            return chunk_node(input, counter, key, flags);
        }
        let chunks = ((input.len() - 1) / CHUNK_LEN + 1) as u64;
        let left_chunks = largest_pow2_below(chunks);
        let split = (left_chunks as usize) * CHUNK_LEN;
        let left = subtree_node(&input[..split], counter, key, flags).chaining_value();
        let right = subtree_node(&input[split..], counter.wrapping_add(left_chunks), key, flags).chaining_value();
        parent_node(left, right, key, flags)
    }

    /// BLAKE3 in the selected mode, starting at chunk counter 0.
    pub fn node(input: &[u8], key: &[u32; 8], flags: u32) -> Node {
        subtree_node(input, 0, key, flags)
    }

    pub fn hash(input: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        node(input, &IV, 0).root_bytes(0, &mut out);
        out
    }

    pub fn keyed_hash(key: &[u8; 32], input: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        node(input, &key_words(key), KEYED_HASH).root_bytes(0, &mut out);
        out
    }

    pub fn derive_key(context: &str, material: &[u8]) -> [u8; 32] {
        let context_key = hash_with_flags(context.as_bytes(), &IV, DERIVE_KEY_CONTEXT);
        let mut out = [0u8; 32];
        node(material, &key_words(&context_key), DERIVE_KEY_MATERIAL).root_bytes(0, &mut out);
        out
    }

    fn hash_with_flags(input: &[u8], key: &[u32; 8], flags: u32) -> [u8; 32] {
        let mut out = [0u8; 32];
        node(input, key, flags).root_bytes(0, &mut out);
        out
    }

    /// Extended output of the plain (unkeyed) hash.
    pub fn xof(input: &[u8], out: &mut [u8]) {
        node(input, &IV, 0).root_bytes(0, out);
    }
}

/// Official BLAKE3 test vectors (`hash`, `keyed_hash`, `derive_key`), from
/// <https://github.com/BLAKE3-team/BLAKE3/blob/master/test_vectors/test_vectors.json>.
/// The inputs are the repeating byte sequence 0, 1, ..., 250, 0, 1, ...
/// Hash outputs are the first 131 bytes of the XOF stream (131 is not a
/// multiple of 4, so a missing partial word is caught).
const OFFICIAL_KEY: &[u8; 32] = b"whats the Elvish word for friend";
const OFFICIAL_CONTEXT: &str = "BLAKE3 2019-12-27 16:29:52 test vectors context";

#[allow(clippy::type_complexity)]
const OFFICIAL_VECTORS: &[(usize, &str, &str, &str)] = &[
    (
        0,
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262e00f03e7b69af26b7faaf09fcd333050338ddfe085b8cc869ca98b206c08243a26f5487789e8f660afe6c99ef9e0c52b92e7393024a80459cf91f476f9ffdbda7001c22e159b402631f277ca96f2defdf1078282314e763699a31c5363165421cce14d",
        "92b2b75604ed3c761f9d6f62392c8a9227ad0ea3f09573e783f1498a4ed60d26",
        "2cc39783c223154fea8dfb7c1b1660f2ac2dcbd1c1de8277b0b0dd39b7e50d7d",
    ),
    (
        1,
        "2d3adedff11b61f14c886e35afa036736dcd87a74d27b5c1510225d0f592e213c3a6cb8bf623e20cdb535f8d1a5ffb86342d9c0b64aca3bce1d31f60adfa137b358ad4d79f97b47c3d5e79f179df87a3b9776ef8325f8329886ba42f07fb138bb502f4081cbcec3195c5871e6c23e2cc97d3c69a613eba131e5f1351f3f1da786545e5",
        "6d7878dfff2f485635d39013278ae14f1454b8c0a3a2d34bc1ab38228a80c95b",
        "b3e2e340a117a499c6cf2398a19ee0d29cca2bb7404c73063382693bf66cb06c",
    ),
    (
        2,
        "7b7015bb92cf0b318037702a6cdd81dee41224f734684c2c122cd6359cb1ee63d8386b22e2ddc05836b7c1bb693d92af006deb5ffbc4c70fb44d0195d0c6f252faac61659ef86523aa16517f87cb5f1340e723756ab65efb2f91964e14391de2a432263a6faf1d146937b35a33621c12d00be8223a7f1919cec0acd12097ff3ab00ab1",
        "5392ddae0e0a69d5f40160462cbd9bd889375082ff224ac9c758802b7a6fd20a",
        "1f166565a7df0098ee65922d7fea425fb18b9943f19d6161e2d17939356168e6",
    ),
    (
        3,
        "e1be4d7a8ab5560aa4199eea339849ba8e293d55ca0a81006726d184519e647f5b49b82f805a538c68915c1ae8035c900fd1d4b13902920fd05e1450822f36de9454b7e9996de4900c8e723512883f93f4345f8a58bfe64ee38d3ad71ab027765d25cdd0e448328a8e7a683b9a6af8b0af94fa09010d9186890b096a08471e4230a134",
        "39e67b76b5a007d4921969779fe666da67b5213b096084ab674742f0d5ec62b9",
        "440aba35cb006b61fc17c0529255de438efc06a8c9ebf3f2ddac3b5a86705797",
    ),
    (
        4,
        "f30f5ab28fe047904037f77b6da4fea1e27241c5d132638d8bedce9d40494f328f603ba4564453e06cdcee6cbe728a4519bbe6f0d41e8a14b5b225174a566dbfa61b56afb1e452dc08c804f8c3143c9e2cc4a31bb738bf8c1917b55830c6e65797211701dc0b98daa1faeaa6ee9e56ab606ce03a1a881e8f14e87a4acf4646272cfd12",
        "7671dde590c95d5ac9616651ff5aa0a27bee5913a348e053b8aa9108917fe070",
        "f46085c8190d69022369ce1a18880e9b369c135eb93f3c63550d3e7630e91060",
    ),
    (
        5,
        "b40b44dfd97e7a84a996a91af8b85188c66c126940ba7aad2e7ae6b385402aa2ebcfdac6c5d32c31209e1f81a454751280db64942ce395104e1e4eaca62607de1c2ca748251754ea5bbe8c20150e7f47efd57012c63b3c6a6632dc1c7cd15f3e1c999904037d60fac2eb9397f2adbe458d7f264e64f1e73aa927b30988e2aed2f03620",
        "73ac69eecf286894d8102018a6fc729f4b1f4247d3703f69bdc6a5fe3e0c8461",
        "1f24eda69dbcb752847ec3ebb5dd42836d86e58500c7c98d906ecd82ed9ae47f",
    ),
    (
        6,
        "06c4e8ffb6872fad96f9aaca5eee1553eb62aed0ad7198cef42e87f6a616c844611a30c4e4f37fe2fe23c0883cde5cf7059d88b657c7ed2087e3d210925ede716435d6d5d82597a1e52b9553919e804f5656278bd739880692c94bff2824d8e0b48cac1d24682699e4883389dc4f2faa2eb3b4db6e39debd5061ff3609916f3e07529a",
        "82d3199d0013035682cc7f2a399d4c212544376a839aa863a0f4c91220ca7a6d",
        "be96b30b37919fe4379dfbe752ae77b4f7e2ab92f7ff27435f76f2f065f6a5f4",
    ),
    (
        7,
        "3f8770f387faad08faa9d8414e9f449ac68e6ff0417f673f602a646a891419fe66036ef6e6d1a8f54baa9fed1fc11c77cfb9cff65bae915045027046ebe0c01bf5a941f3bb0f73791d3fc0b84370f9f30af0cd5b0fc334dd61f70feb60dad785f070fef1f343ed933b49a5ca0d16a503f599a365a4296739248b28d1a20b0e2cc8975c",
        "af0a7ec382aedc0cfd626e49e7628bc7a353a4cb108855541a5651bf64fbb28a",
        "dc3b6485f9d94935329442916b0d059685ba815a1fa2a14107217453a7fc9f0e",
    ),
    (
        8,
        "2351207d04fc16ade43ccab08600939c7c1fa70a5c0aaca76063d04c3228eaeb725d6d46ceed8f785ab9f2f9b06acfe398c6699c6129da084cb531177445a682894f9685eaf836999221d17c9a64a3a057000524cd2823986db378b074290a1a9b93a22e135ed2c14c7e20c6d045cd00b903400374126676ea78874d79f2dd7883cf5c",
        "be2f5495c61cba1bb348a34948c004045e3bd4dae8f0fe82bf44d0da245a0600",
        "2b166978cef14d9d438046c720519d8b1cad707e199746f1562d0c87fbd32940",
    ),
    (
        63,
        "e9bc37a594daad83be9470df7f7b3798297c3d834ce80ba85d6e207627b7db7b1197012b1e7d9af4d7cb7bdd1f3bb49a90a9b5dec3ea2bbc6eaebce77f4e470cbf4687093b5352f04e4a4570fba233164e6acc36900e35d185886a827f7ea9bdc1e5c3ce88b095a200e62c10c043b3e9bc6cb9b6ac4dfa51794b02ace9f98779040755",
        "bb1eb5d4afa793c1ebdd9fb08def6c36d10096986ae0cfe148cd101170ce37ae",
        "b6451e30b953c206e34644c6803724e9d2725e0893039cfc49584f991f451af3",
    ),
    (
        64,
        "4eed7141ea4a5cd4b788606bd23f46e212af9cacebacdc7d1f4c6dc7f2511b98fc9cc56cb831ffe33ea8e7e1d1df09b26efd2767670066aa82d023b1dfe8ab1b2b7fbb5b97592d46ffe3e05a6a9b592e2949c74160e4674301bc3f97e04903f8c6cf95b863174c33228924cdef7ae47559b10b294acd660666c4538833582b43f82d74",
        "ba8ced36f327700d213f120b1a207a3b8c04330528586f414d09f2f7d9ccb7e6",
        "a5c4a7053fa86b64746d4bb688d06ad1f02a18fce9afd3e818fefaa7126bf73e",
    ),
    (
        65,
        "de1e5fa0be70df6d2be8fffd0e99ceaa8eb6e8c93a63f2d8d1c30ecb6b263dee0e16e0a4749d6811dd1d6d1265c29729b1b75a9ac346cf93f0e1d7296dfcfd4313b3a227faaaaf7757cc95b4e87a49be3b8a270a12020233509b1c3632b3485eef309d0abc4a4a696c9decc6e90454b53b000f456a3f10079072baaf7a981653221f2c",
        "c0a4edefa2d2accb9277c371ac12fcdbb52988a86edc54f0716e1591b4326e72",
        "51fd05c3c1cfbc8ed67d139ad76f5cf8236cd2acd26627a30c104dfd9d3ff8a8",
    ),
    (
        127,
        "d81293fda863f008c09e92fc382a81f5a0b4a1251cba1634016a0f86a6bd640de3137d477156d1fde56b0cf36f8ef18b44b2d79897bece12227539ac9ae0a5119da47644d934d26e74dc316145dcb8bb69ac3f2e05c242dd6ee06484fcb0e956dc44355b452c5e2bbb5e2b66e99f5dd443d0cbcaaafd4beebaed24ae2f8bb672bcef78",
        "c64200ae7dfaf35577ac5a9521c47863fb71514a3bcad18819218b818de85818",
        "c91c090ceee3a3ac81902da31838012625bbcd73fcb92e7d7e56f78deba4f0c3",
    ),
    (
        128,
        "f17e570564b26578c33bb7f44643f539624b05df1a76c81f30acd548c44b45efa69faba091427f9c5c4caa873aa07828651f19c55bad85c47d1368b11c6fd99e47ecba5820a0325984d74fe3e4058494ca12e3f1d3293d0010a9722f7dee64f71246f75e9361f44cc8e214a100650db1313ff76a9f93ec6e84edb7add1cb4a95019b0c",
        "b04fe15577457267ff3b6f3c947d93be581e7e3a4b018679125eaf86f6a628ec",
        "81720f34452f58a0120a58b6b4608384b5c51d11f39ce97161a0c0e442ca0225",
    ),
    (
        129,
        "683aaae9f3c5ba37eaaf072aed0f9e30bac0865137bae68b1fde4ca2aebdcb12f96ffa7b36dd78ba321be7e842d364a62a42e3746681c8bace18a4a8a79649285c7127bf8febf125be9de39586d251f0d41da20980b70d35e3dac0eee59e468a894fa7e6a07129aaad09855f6ad4801512a116ba2b7841e6cfc99ad77594a8f2d181a7",
        "d4a64dae6cdccbac1e5287f54f17c5f985105457c1a2ec1878ebd4b57e20d38f",
        "938d2d4435be30eafdbb2b7031f7857c98b04881227391dc40db3c7b21f41fc1",
    ),
    (
        1023,
        "10108970eeda3eb932baac1428c7a2163b0e924c9a9e25b35bba72b28f70bd11a182d27a591b05592b15607500e1e8dd56bc6c7fc063715b7a1d737df5bad3339c56778957d870eb9717b57ea3d9fb68d1b55127bba6a906a4a24bbd5acb2d123a37b28f9e9a81bbaae360d58f85e5fc9d75f7c370a0cc09b6522d9c8d822f2f28f485",
        "c951ecdf03288d0fcc96ee3413563d8a6d3589547f2c2fb36d9786470f1b9d6e",
        "74a16c1c3d44368a86e1ca6df64be6a2f64cce8f09220787450722d85725dea5",
    ),
    (
        1024,
        "42214739f095a406f3fc83deb889744ac00df831c10daa55189b5d121c855af71cf8107265ecdaf8505b95d8fcec83a98a6a96ea5109d2c179c47a387ffbb404756f6eeae7883b446b70ebb144527c2075ab8ab204c0086bb22b7c93d465efc57f8d917f0b385c6df265e77003b85102967486ed57db5c5ca170ba441427ed9afa684e",
        "75c46f6f3d9eb4f55ecaaee480db732e6c2105546f1e675003687c31719c7ba4",
        "7356cd7720d5b66b6d0697eb3177d9f8d73a4a5c5e968896eb6a689684302706",
    ),
    (
        1025,
        "d00278ae47eb27b34faecf67b4fe263f82d5412916c1ffd97c8cb7fb814b8444f4c4a22b4b399155358a994e52bf255de60035742ec71bd08ac275a1b51cc6bfe332b0ef84b409108cda080e6269ed4b3e2c3f7d722aa4cdc98d16deb554e5627be8f955c98e1d5f9565a9194cad0c4285f93700062d9595adb992ae68ff12800ab67a",
        "357dc55de0c7e382c900fd6e320acc04146be01db6a8ce7210b7189bd664ea69",
        "effaa245f065fbf82ac186839a249707c3bddf6d3fdda22d1b95a3c970379bcb",
    ),
    (
        2048,
        "e776b6028c7cd22a4d0ba182a8bf62205d2ef576467e838ed6f2529b85fba24a9a60bf80001410ec9eea6698cd537939fad4749edd484cb541aced55cd9bf54764d063f23f6f1e32e12958ba5cfeb1bf618ad094266d4fc3c968c2088f677454c288c67ba0dba337b9d91c7e1ba586dc9a5bc2d5e90c14f53a8863ac75655461cea8f9",
        "879cf1fa2ea0e79126cb1063617a05b6ad9d0b696d0d757cf053439f60a99dd1",
        "7b2945cb4fef70885cc5d78a87bf6f6207dd901ff239201351ffac04e1088a23",
    ),
    (
        2049,
        "5f4d72f40d7a5f82b15ca2b2e44b1de3c2ef86c426c95c1af0b687952256303096de31d71d74103403822a2e0bc1eb193e7aecc9643a76b7bbc0c9f9c52e8783aae98764ca468962b5c2ec92f0c74eb5448d519713e09413719431c802f948dd5d90425a4ecdadece9eb178d80f26efccae630734dff63340285adec2aed3b51073ad3",
        "9f29700902f7c86e514ddc4df1e3049f258b2472b6dd5267f61bf13983b78dd5",
        "2ea477c5515cc3dd606512ee72bb3e0e758cfae7232826f35fb98ca1bcbdf273",
    ),
    (
        3072,
        "b98cb0ff3623be03326b373de6b9095218513e64f1ee2edd2525c7ad1e5cffd29a3f6b0b978d6608335c09dc94ccf682f9951cdfc501bfe47b9c9189a6fc7b404d120258506341a6d802857322fbd20d3e5dae05b95c88793fa83db1cb08e7d8008d1599b6209d78336e24839724c191b2a52a80448306e0daa84a3fdb566661a37e11",
        "044a0e7b172a312dc02a4c9a818c036ffa2776368d7f528268d2e6b5df191770",
        "050df97f8c2ead654d9bb3ab8c9178edcd902a32f8495949feadcc1e0480c46b",
    ),
    (
        3073,
        "7124b49501012f81cc7f11ca069ec9226cecb8a2c850cfe644e327d22d3e1cd39a27ae3b79d68d89da9bf25bc27139ae65a324918a5f9b7828181e52cf373c84f35b639b7fccbb985b6f2fa56aea0c18f531203497b8bbd3a07ceb5926f1cab74d14bd66486d9a91eba99059a98bd1cd25876b2af5a76c3e9eed554ed72ea952b603bf",
        "68dede9bef00ba89e43f31a6825f4cf433389fedae75c04ee9f0cf16a427c95a",
        "72613c9ec9ff7e40f8f5c173784c532ad852e827dba2bf85b2ab4b76f7079081",
    ),
    (
        4096,
        "015094013f57a5277b59d8475c0501042c0b642e531b0a1c8f58d2163229e9690289e9409ddb1b99768eafe1623da896faf7e1114bebeadc1be30829b6f8af707d85c298f4f0ff4d9438aef948335612ae921e76d411c3a9111df62d27eaf871959ae0062b5492a0feb98ef3ed4af277f5395172dbe5c311918ea0074ce0036454f620",
        "befc660aea2f1718884cd8deb9902811d332f4fc4a38cf7c7300d597a081bfc0",
        "1e0d7f3db8c414c97c6307cbda6cd27ac3b030949da8e23be1a1a924ad2f25b9",
    ),
    (
        4097,
        "9b4052b38f1c5fc8b1f9ff7ac7b27cd242487b3d890d15c96a1c25b8aa0fb99505f91b0b5600a11251652eacfa9497b31cd3c409ce2e45cfe6c0a016967316c426bd26f619eab5d70af9a418b845c608840390f361630bd497b1ab44019316357c61dbe091ce72fc16dc340ac3d6e009e050b3adac4b5b2c92e722cffdc46501531956",
        "00df940cd36bb9fa7cbbc3556744e0dbc8191401afe70520ba292ee3ca80abbc",
        "aca51029626b55fda7117b42a7c211f8c6e9ba4fe5b7a8ca922f34299500ead8",
    ),
    (
        8192,
        "aae792484c8efe4f19e2ca7d371d8c467ffb10748d8a5a1ae579948f718a2a635fe51a27db045a567c1ad51be5aa34c01c6651c4d9b5b5ac5d0fd58cf18dd61a47778566b797a8c67df7b1d60b97b19288d2d877bb2df417ace009dcb0241ca1257d62712b6a4043b4ff33f690d849da91ea3bf711ed583cb7b7a7da2839ba71309bbf",
        "dc9637c8845a770b4cbf76b8daec0eebf7dc2eac11498517f08d44c8fc00d58a",
        "ad01d7ae4ad059b0d33baa3c01319dcf8088094d0359e5fd45d6aeaa8b2d0c3d",
    ),
    (
        8193,
        "bab6c09cb8ce8cf459261398d2e7aef35700bf488116ceb94a36d0f5f1b7bc3bb2282aa69be089359ea1154b9a9286c4a56af4de975a9aa4a5c497654914d279bea60bb6d2cf7225a2fa0ff5ef56bbe4b149f3ed15860f78b4e2ad04e158e375c1e0c0b551cd7dfc82f1b155c11b6b3ed51ec9edb30d133653bb5709d1dbd55f4e1ff6",
        "954a2a75420c8d6547e3ba5b98d963e6fa6491addc8c023189cc519821b4a1f5",
        "af1e0346e389b17c23200270a64aa4e1ead98c61695d917de7d5b00491c9b0f1",
    ),
    (
        16384,
        "f875d6646de28985646f34ee13be9a576fd515f76b5b0a26bb324735041ddde49d764c270176e53e97bdffa58d549073f2c660be0e81293767ed4e4929f9ad34bbb39a529334c57c4a381ffd2a6d4bfdbf1482651b172aa883cc13408fa67758a3e47503f93f87720a3177325f7823251b85275f64636a8f1d599c2e49722f42e93893",
        "9e9fc4eb7cf081ea7c47d1807790ed211bfec56aa25bb7037784c13c4b707b0d",
        "160e18b5878cd0df1c3af85eb25a0db5344d43a6fbd7a8ef4ed98d0714c3f7e1",
    ),
    (
        31744,
        "62b6960e1a44bcc1eb1a611a8d6235b6b4b78f32e7abc4fb4c6cdcce94895c47860cc51f2b0c28a7b77304bd55fe73af663c02d3f52ea053ba43431ca5bab7bfea2f5e9d7121770d88f70ae9649ea713087d1914f7f312147e247f87eb2d4ffef0ac978bf7b6579d57d533355aa20b8b77b13fd09748728a5cc327a8ec470f4013226f",
        "efa53b389ab67c593dba624d898d0f7353ab99e4ac9d42302ee64cbf9939a419",
        "39772aef80e0ebe60596361e45b061e8f417429d529171b6764468c22928e28e",
    ),
    (
        102400,
        "bc3e3d41a1146b069abffad3c0d44860cf664390afce4d9661f7902e7943e085e01c59dab908c04c3342b816941a26d69c2605ebee5ec5291cc55e15b76146e6745f0601156c3596cb75065a9c57f35585a52e1ac70f69131c23d611ce11ee4ab1ec2c009012d236648e77be9295dd0426f29b764d65de58eb7d01dd42248204f45f8e",
        "1c35d1a5811083fd7119f5d5d1ba027b4d01c0c6c49fb6ff2cf75393ea5db4a7",
        "4652cff7a3f385a6103b5c260fc1593e13c778dbe608efb092fe7ee69df6e9c6",
    ),
];

fn test_input(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i % 251) as u8).collect()
}

/// Deterministic pseudorandom bytes (seeded `StdRng`), so failures reproduce.
fn pseudo_random(len: usize, seed: u64) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = vec![0u8; len];
    rng.fill(&mut out);
    out
}

fn assert_hash_eq(input: &[u8]) {
    let expected = reference::hash(input);
    let got = Blake3::hash(input);
    assert_eq!(&got[..], &expected[..], "one-shot hash mismatch at len {}", input.len());
}

////////////////////////////////////////////////////////////////////////////////
// Length sweeps
////////////////////////////////////////////////////////////////////////////////
/// Every length in `0..=2048`, plus the named boundaries. This brackets every
/// block boundary, chunk boundary and small-SIMD-subtree boundary.
#[test]
fn length_sweep_one_shot_matches_reference() {
    for len in 0..=2048 {
        assert_hash_eq(&test_input(len));
    }
    for len in [4095usize, 4096, 4097, 8191, 8192, 8193, 1 << 20] {
        assert_hash_eq(&pseudo_random(len, len as u64 ^ 0xB1A3));
    }
}

/// Keyed and derive-key modes over a smaller set of lengths (the tree logic is
/// shared, so this mostly checks the flag/key plumbing).
#[test]
fn modes_match_reference_across_lengths() {
    let key: [u8; 32] = pseudo_random(32, 0x5EED).try_into().unwrap();
    let context = "blake3 key derivation test context";

    let lengths = [
        0usize, 1, 63, 64, 65, 127, 128, 129, 1023, 1024, 1025, 2048, 2049, 3072, 3073, 4095, 4096, 4097, 8191, 8192,
        8193,
    ];
    for &len in &lengths {
        let input = pseudo_random(len, len as u64 ^ 0xC0FFEE);
        let got = Blake3::keyed_hash(&key, &input);
        assert_eq!(
            &got[..],
            &reference::keyed_hash(&key, &input)[..],
            "keyed hash mismatch at len {len}"
        );

        let got = Blake3::derive_key(context, &input);
        assert_eq!(
            &got[..],
            &reference::derive_key(context, &input)[..],
            "derive_key mismatch at len {len}"
        );
    }
}

/// The official test vectors, via the public one-shot and incremental APIs and
/// the XOF reader (131 bytes: a partial final word).
#[test]
fn official_vectors() {
    for &(len, hash_hex, keyed_hex, derive_hex) in OFFICIAL_VECTORS {
        let input = test_input(len);
        let expected = hex::decode(hash_hex).unwrap();
        assert_eq!(expected.len(), 131);

        let one_shot = Blake3::hash(&input);
        assert_eq!(&one_shot[..], &expected[..32], "official hash vector mismatch at len {len}");

        let mut hasher = Blake3::new();
        hasher.update(&input);
        assert_eq!(
            &hasher.clone().sum()[..],
            &expected[..32],
            "incremental official hash at len {len}"
        );

        let mut xof = hasher.finalize_xof();
        let mut stream = vec![0u8; expected.len()];
        xof.squeeze(&mut stream);
        assert_eq!(stream, expected, "official XOF vector mismatch at len {len}");

        let keyed = Blake3::keyed_hash(OFFICIAL_KEY, &input);
        assert_eq!(
            &keyed[..],
            &hex::decode(keyed_hex).unwrap()[..],
            "official keyed_hash vector mismatch at len {len}"
        );

        let derived = Blake3::derive_key(OFFICIAL_CONTEXT, &input);
        assert_eq!(
            &derived[..],
            &hex::decode(derive_hex).unwrap()[..],
            "official derive_key vector mismatch at len {len}"
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
// Incremental update patterns
////////////////////////////////////////////////////////////////////////////////
/// Every two-way split of the input at every byte offset must match the
/// one-shot reference. The 300-byte input exercises the tree layout; the
/// larger lengths bracket chunk and SIMD boundaries.
#[test]
fn every_two_way_split_of_300_bytes_and_chunk_boundaries() {
    let lengths = [
        0usize, 1, 2, 3, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 300, 511, 512, 513, 1023, 1024, 1025,
        1536, 2048, 2049, 3072, 4096,
    ];

    for &len in &lengths {
        let input = pseudo_random(len, len as u64 ^ 0x5151);
        let expected = reference::hash(&input);
        for split in 0..=len {
            let mut hasher = Blake3::new();
            hasher.update(&input[..split]);
            hasher.update(&input[split..]);
            assert_eq!(&hasher.sum()[..], &expected[..], "two-way split {split}/{len} mismatched");
        }
    }
}

/// Every three-way split of a 300-byte input (and of a few block-boundary
/// lengths).
#[test]
fn every_three_way_split_of_300_bytes() {
    for len in [300usize, 65, 129] {
        let input = pseudo_random(len, len as u64 ^ 0x7777);
        let expected = reference::hash(&input);
        for a in 0..=len {
            for b in a..=len {
                let mut hasher = Blake3::new();
                hasher.update(&input[..a]);
                hasher.update(&input[a..b]);
                hasher.update(&input[b..]);
                assert_eq!(&hasher.sum()[..], &expected[..], "three-way split {a}/{b}/{len} mismatched");
            }
        }
    }
}

/// Deterministic pseudorandom split patterns for larger inputs, with empty
/// updates interleaved, in all three modes.
#[test]
fn randomized_splits_larger_inputs() {
    let mut rng = StdRng::seed_from_u64(0x9E37_79B9_7F4A_7C15);

    let key: [u8; 32] = pseudo_random(32, 0x1234_5678).try_into().unwrap();
    let context = "blake3 randomized splits test context";

    for &len in &[2047usize, 2048, 2049, 5000, 8192, 20000, 33_000] {
        let input = pseudo_random(len, len as u64 ^ 0xDEAD);
        // Compute each reference once; feed many random splittings.
        let expected = reference::hash(&input);
        let expected_keyed = reference::keyed_hash(&key, &input);
        let expected_derived = reference::derive_key(context, &input);

        for trial in 0..24 {
            let mut hasher = Blake3::new();
            let mut keyed = Blake3::new_keyed(&key);
            let mut derived = Blake3::new_derive_key(context);
            let mut offset = 0;
            while offset < len {
                let remaining = len - offset;
                let step = 1 + (rng.next_u64() as usize) % remaining;
                let end = offset + step;
                hasher.update(&[]);
                hasher.update(&input[offset..end]);
                keyed.update(&input[offset..end]);
                derived.update(&input[offset..end]);
                offset = end;
            }
            assert_eq!(&hasher.sum()[..], &expected[..], "random split trial {trial} len {len}");
            assert_eq!(
                &keyed.sum()[..],
                &expected_keyed[..],
                "random keyed split trial {trial} len {len}"
            );
            assert_eq!(
                &derived.sum()[..],
                &expected_derived[..],
                "random derive_key split trial {trial} len {len}"
            );
        }
    }
}

/// A `Clone` taken mid-stream must be independent: updating either copy (in
/// either direction) must still equal the reference for the corresponding
/// total input.
#[test]
fn clone_then_diverge_matches_reference() {
    for &(split, tail_a, tail_b) in &[
        (0usize, 1usize, 2usize),
        (1, 63, 64),
        (64, 1, 1023),
        (1023, 1, 2),
        (1024, 1, 2048),
        (1500, 500, 100),
        (2048, 1024, 1),
        (4096, 4097, 4096),
    ] {
        let prefix = pseudo_random(split, split as u64 ^ 0x1111);
        let a = pseudo_random(tail_a, 0xAAAA);
        let b = pseudo_random(tail_b, 0xBBBB);

        let mut base = Blake3::new();
        base.update(&prefix);
        let mut clone = base.clone();

        base.update(&a[1..]);
        clone.update(&b[1..]);

        let mut expected = prefix.clone();
        expected.extend_from_slice(&a[1..]);
        assert_eq!(
            &base.sum()[..],
            &reference::hash(&expected)[..],
            "clone branch A, prefix {split}"
        );

        let mut expected = prefix;
        expected.extend_from_slice(&b[1..]);
        assert_eq!(
            &clone.sum()[..],
            &reference::hash(&expected)[..],
            "clone branch B, prefix {split}"
        );
    }
}

/// `finalize` is documented as idempotent; a clone can finalize, keep
/// absorbing, and finalize again.
#[test]
fn finalize_is_idempotent_and_non_destructive() {
    let input = pseudo_random(3000, 0x2222);
    let mut hasher = Blake3::new();
    hasher.update(&input[..1000]);
    let once = hasher.clone().sum();
    let twice = hasher.clone().sum();
    assert_eq!(&once[..], &twice[..], "finalize is not idempotent");

    let mut resumed = hasher.clone();
    resumed.update(&input[1000..]);
    assert_eq!(&resumed.sum()[..], &reference::hash(&input)[..], "finalize consumed state");
}

////////////////////////////////////////////////////////////////////////////////
// XOF
////////////////////////////////////////////////////////////////////////////////
/// Existing vector lengths plus the seam between the cached block and wide
/// output paths.
#[test]
fn xof_stream_matches_reference_in_every_chunking() {
    for len in [0usize, 1, 63, 64, 65, 1023, 1024, 1500, 2048, 4096, 8192] {
        let input = pseudo_random(len, len as u64 ^ 0xA0F0);
        let mut expected = [0u8; 512];
        reference::xof(&input, &mut expected);

        // One big squeeze.
        let mut hasher = Blake3::new();
        hasher.update(&input);
        let mut one_shot = [0u8; 512];
        hasher.clone().finalize_xof().squeeze(&mut one_shot);
        assert_eq!(one_shot, expected, "one-shot XOF mismatch at len {len}");

        // Piecewise squeezes, with empty squeezes interleaved. Sizes include
        // 1, 63/64/65 around the output-block boundary, and 127/128/129.
        for &step in &[1usize, 3, 32, 63, 64, 65, 127, 128, 200] {
            let mut xof = hasher.clone().finalize_xof();
            let mut out = [0u8; 512];
            let mut offset = 0;
            while offset < out.len() {
                xof.squeeze(&mut []);
                let end = core::cmp::min(offset + step, out.len());
                xof.squeeze(&mut out[offset..end]);
                offset = end;
            }
            assert_eq!(out, expected, "piecewise XOF mismatch at len {len}, step {step}");
        }
    }
}

/// A cloned XOF reader continues from the same position and produces the same
/// remaining bytes as the original.
#[test]
fn xof_reader_clone_continues_identically() {
    let input = pseudo_random(2049, 0x5150);
    let mut hasher = Blake3::new();
    hasher.update(&input);

    let mut expected = [0u8; 256];
    reference::xof(&input, &mut expected);

    let mut reader = hasher.finalize_xof();
    let mut first = [0u8; 100];
    reader.squeeze(&mut first);
    let mut clone = reader.clone();

    let mut a = [0u8; 156];
    let mut b = [0u8; 156];
    reader.squeeze(&mut a);
    clone.squeeze(&mut b);
    assert_eq!(a, b, "cloned XOF reader produced a different remainder");
    assert_eq!(&first[..], &expected[..100]);
    assert_eq!(&a[..], &expected[100..256]);
}

/// `squeeze` must never write outside the slice it is given, including at the
/// start, middle and end of an output stream.
#[test]
fn xof_squeeze_respects_slice_bounds() {
    let input = pseudo_random(1234, 0x3131);
    let mut hasher = Blake3::new();
    hasher.update(&input);

    for &(prefix_len, out_len, suffix_len) in &[
        (0usize, 0usize, 0usize),
        (0, 1, 0),
        (7, 1, 9),
        (7, 63, 9),
        (7, 64, 9),
        (7, 65, 9),
        (1, 511, 1),
        (33, 512, 33),
    ] {
        let mut buf = vec![0xAAu8; prefix_len + out_len + suffix_len];
        let mut xof = hasher.clone().finalize_xof();
        xof.squeeze(&mut buf[prefix_len..prefix_len + out_len]);
        assert!(buf[..prefix_len].iter().all(|&b| b == 0xAA), "squeeze wrote before its slice");
        assert!(
            buf[prefix_len + out_len..].iter().all(|&b| b == 0xAA),
            "squeeze wrote after its slice ({out_len} bytes)"
        );
    }
}

/// A XOF reader is an output-only stream: `absorb` must ignore its argument
/// and must not change the output.
#[test]
fn xof_absorb_is_a_noop() {
    let input = pseudo_random(777, 0x4242);
    let mut hasher = Blake3::new();
    hasher.update(&input);

    let mut plain = [0u8; 200];
    hasher.clone().finalize_xof().squeeze(&mut plain);

    let mut xof = hasher.finalize_xof();
    xof.absorb(b"ignored");
    let mut after = [0u8; 200];
    xof.absorb(b"also ignored");
    xof.squeeze(&mut after[..100]);
    xof.absorb(&[0xFF; 64]);
    xof.squeeze(&mut after[100..]);
    assert_eq!(plain, after, "absorb changed the XOF output");
}

////////////////////////////////////////////////////////////////////////////////
// End-of-page input buffers (out-of-bounds read check)
////////////////////////////////////////////////////////////////////////////////
/// Hash inputs that end exactly at an inaccessible guard page, so any read of
/// even one byte past the end of the input (the classic final-partial-chunk
/// over-read) segfaults the test instead of failing silently.
///
/// Only run on Linux natively; Miri does not implement `mmap`/`sysconf`.
#[cfg(all(target_os = "linux", not(miri)))]
#[test]
fn input_ending_at_a_guard_page_is_never_over_read() {
    use core::ffi::c_void;

    unsafe extern "C" {
        fn mmap(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut c_void;
        fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
        fn munmap(addr: *mut c_void, len: usize) -> i32;
        fn sysconf(name: i32) -> i64;
    }

    const PROT_NONE: i32 = 0;
    const PROT_READ: i32 = 1;
    const PROT_WRITE: i32 = 2;
    const MAP_PRIVATE: i32 = 2;
    const MAP_ANONYMOUS: i32 = 0x20;
    const SC_PAGESIZE: i32 = 30;

    struct Mapping {
        base: *mut u8,
        len: usize,
        page: usize,
    }

    impl Mapping {
        fn new() -> Self {
            let page = unsafe { sysconf(SC_PAGESIZE) } as usize;
            assert!(page >= 4096 && page.is_power_of_two(), "unexpected page size {page}");
            let len = 2 * page;
            let base = unsafe {
                mmap(
                    core::ptr::null_mut(),
                    len,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    0,
                )
            };
            assert_ne!(base as isize, -1, "mmap failed");
            let base = base.cast::<u8>();
            // Make the second page inaccessible: the data page is `base..base+page`.
            assert_eq!(
                unsafe { mprotect(base.add(page).cast(), page, PROT_NONE) },
                0,
                "mprotect failed"
            );
            Self {
                base,
                len,
                page,
            }
        }

        /// Writable/readable data page.
        fn data(&mut self) -> &mut [u8] {
            unsafe { core::slice::from_raw_parts_mut(self.base, self.page) }
        }

        /// Read-only view of the data page.
        fn data_ref(&self) -> &[u8] {
            unsafe { core::slice::from_raw_parts(self.base, self.page) }
        }
    }

    impl Drop for Mapping {
        fn drop(&mut self) {
            unsafe {
                munmap(self.base.cast(), self.len);
            }
        }
    }

    let mut mapping = Mapping::new();
    let page = mapping.page;

    // Lengths that end at the guard page and bracket every internal boundary.
    for &len in &[
        1usize, 2, 3, 31, 32, 33, 63, 64, 65, 66, 127, 128, 129, 255, 256, 1023, 1024, 1025, 2047, 2048, 2049, 4095,
        4096, 4097, 8191, 8192, 8193, 16_384, 32_768,
    ] {
        let len = core::cmp::min(len, page);
        let start = page - len;
        let expected_input = {
            let data = mapping.data();
            for (i, byte) in data[start..].iter_mut().enumerate() {
                *byte = ((start + i) % 251) as u8;
            }
            data[start..].to_vec()
        };
        let expected = reference::hash(&expected_input);
        // Reborrow immutably, ending exactly at the guard page.
        let input: &[u8] = &mapping.data_ref()[start..];

        assert_eq!(
            &Blake3::hash(input)[..],
            &expected[..],
            "guard-page one-shot mismatch at len {len}"
        );

        let key = [0x42u8; 32];
        assert_eq!(
            &Blake3::keyed_hash(&key, input)[..],
            &reference::keyed_hash(&key, &expected_input)[..],
            "guard-page keyed mismatch at len {len}"
        );

        // Incremental, with pieces that also end at the guard page.
        for &step in &[1usize, 61, 64, 65, 1023, 1024] {
            let mut hasher = Blake3::new();
            let mut offset = 0;
            while offset < input.len() {
                let end = core::cmp::min(offset + step, input.len());
                hasher.update(&input[offset..end]);
                offset = end;
            }
            assert_eq!(
                &hasher.sum()[..],
                &expected[..],
                "guard-page incremental mismatch at len {len}, step {step}"
            );
        }

        let mut xof = {
            let mut hasher = Blake3::new();
            hasher.update(input);
            hasher.finalize_xof()
        };
        let mut out = [0u8; 128];
        xof.squeeze(&mut out);
        let mut expected_xof = [0u8; 128];
        reference::xof(&expected_input, &mut expected_xof);
        assert_eq!(out, expected_xof, "guard-page XOF mismatch at len {len}");
    }

    // Also place the input at a non-zero offset in the middle of the page,
    // ending at the guard page, and read it via an unaligned slice.
    for offset in [1usize, 3, 7, 61, 63] {
        let expected_input = {
            let data = mapping.data();
            for (i, byte) in data[offset..].iter_mut().enumerate() {
                *byte = ((i * 7) % 251) as u8;
            }
            data[offset..].to_vec()
        };
        let expected = reference::hash(&expected_input);
        let input: &[u8] = &mapping.data_ref()[offset..];
        assert_eq!(
            &Blake3::hash(input)[..],
            &expected[..],
            "guard-page large unaligned input at offset {offset}"
        );
    }
}
