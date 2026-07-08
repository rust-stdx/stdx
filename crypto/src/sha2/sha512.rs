use crate::{Bytes, Hash, Hasher};

pub(crate) const SHA512_K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

/// SHA-512 hash function (RFC 6234 / FIPS 180-4).
///
/// Implements the [`Hasher`] trait.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::{Hasher, sha2::Sha512};
///
/// let hash = Sha512::hash(b"hello world");
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{Hasher, sha2::Sha512};
///
/// let mut hasher = Sha512::new();
/// hasher.update(b"hello ");
/// hasher.update(b"world");
/// let hash = hasher.sum();
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Sha512 {
    state: [u64; 8],
    buffer: [u8; 128],
    buffer_len: usize,
    total_len: u128,
    has_sha_ext: bool,
}

impl Hasher for Sha512 {
    const BLOCK_SIZE: usize = 128;
    const OUTPUT_SIZE: usize = 64;

    #[inline]
    fn new() -> Self {
        return Sha512 {
            state: [
                0x6a09e667f3bcc908,
                0xbb67ae8584caa73b,
                0x3c6ef372fe94f82b,
                0xa54ff53a5f1d36f1,
                0x510e527fade682d1,
                0x9b05688c2b3e6c1f,
                0x1f83d9abfb41bd6b,
                0x5be0cd19137e2179,
            ],
            buffer: [0u8; 128],
            buffer_len: 0,
            total_len: 0,
            has_sha_ext: detect_sha512_ext(),
        };
    }

    #[inline]
    fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u128);

        if self.buffer_len > 0 {
            let to_fill = (128 - self.buffer_len).min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + to_fill].copy_from_slice(&data[..to_fill]);
            self.buffer_len += to_fill;
            data = &data[to_fill..];

            if self.buffer_len == 128 {
                process_block(&mut self.state, &self.buffer, self.has_sha_ext);
                self.buffer_len = 0;
            }
        }

        let mut chunks = data.chunks_exact(128);
        for chunk in &mut chunks {
            process_block(&mut self.state, chunk.try_into().unwrap(), self.has_sha_ext);
        }

        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            self.buffer[..remainder.len()].copy_from_slice(remainder);
            self.buffer_len = remainder.len();
        }
    }

    #[inline]
    fn sum(mut self) -> Hash {
        let bit_len = self.total_len.wrapping_mul(8);

        let mut tail = [0u8; 256];
        tail[..self.buffer_len].copy_from_slice(&self.buffer[..self.buffer_len]);
        tail[self.buffer_len] = 0x80;

        let padding_len = if self.buffer_len < 112 {
            112 - self.buffer_len
        } else {
            240 - self.buffer_len
        };

        let length_offset = self.buffer_len + padding_len;
        tail[length_offset..length_offset + 16].copy_from_slice(&bit_len.to_be_bytes());

        let total_tail_len = length_offset + 16;
        for chunk in tail[..total_tail_len].chunks_exact(128) {
            process_block(&mut self.state, chunk.try_into().unwrap(), self.has_sha_ext);
        }

        let mut hash = Bytes::<64>::new();
        for word in self.state.iter() {
            hash.append(&word.to_be_bytes());
        }

        return Hash(hash);
    }
}

#[inline(always)]
fn process_block(state: &mut [u64; 8], block: &[u8; 128], has_sha_ext: bool) {
    if has_sha_ext {
        #[cfg(target_arch = "x86_64")]
        {
            unsafe { super::sha512_amd64::compress(state, block) };
            return;
        }

        #[cfg(target_arch = "aarch64")]
        {
            unsafe { super::sha512_arm64::compress(state, block) };
            return;
        }
    }

    process_block_scalar(state, block);
}

/// Determine whether the CPU supports hardware SHA-512 acceleration.
pub(crate) fn detect_sha512_ext() -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(feature = "std")]
        return std::arch::is_aarch64_feature_detected!("sha3");

        #[cfg(all(not(feature = "std"), target_feature = "sha3"))]
        return true;

        #[cfg(all(not(feature = "std"), not(target_feature = "sha3")))]
        return false;
    }
    #[cfg(target_arch = "x86_64")]
    {
        #[cfg(feature = "std")]
        return std::arch::is_x86_feature_detected!("sha512") && std::arch::is_x86_feature_detected!("avx");

        #[cfg(all(not(feature = "std"), target_feature = "sha512", target_feature = "avx"))]
        return true;

        #[cfg(all(not(feature = "std"), not(all(target_feature = "sha512", target_feature = "avx"))))]
        return false;
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    return false;
}

#[inline]
pub(crate) fn process_block_scalar(state: &mut [u64; 8], block: &[u8; 128]) {
    let mut w = [0u64; 80];
    let mut i = 0usize;
    while i < 16 {
        let offset = i * 8;
        w[i] = u64::from_be_bytes([
            block[offset],
            block[offset + 1],
            block[offset + 2],
            block[offset + 3],
            block[offset + 4],
            block[offset + 5],
            block[offset + 6],
            block[offset + 7],
        ]);
        i += 1;
    }

    while i < 80 {
        let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
        let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        i += 1;
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    i = 0;
    while i < 80 {
        let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(SHA512_K[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(maj);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);

        i += 1;
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

#[cfg(test)]
mod tests {
    use super::Sha512;
    use crate::Hasher;

    fn vectors_sha512() -> Vec<(Vec<u8>, [u8; 64])> {
        vec![
            // RFC 6234 / common SHA-512 vectors
            (
                b"".to_vec(),
                hex::decode_array::<64>(b"cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e").unwrap(),
            ),
            (
                b"a".to_vec(),
                hex::decode_array::<64>(b"1f40fc92da241694750979ee6cf582f2d5d7d28e18335de05abc54d0560e0f5302860c652bf08d560252aa5e74210546f369fbbbce8c12cfc7957b2652fe9a75").unwrap(),
            ),
            (
                b"abc".to_vec(),
                hex::decode_array::<64>(b"ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f").unwrap(),
            ),
            (
                b"message digest".to_vec(),
                hex::decode_array::<64>(b"107dbf389d9e9f71a3a95f6c055b9251bc5268c2be16d6c13492ea45b0199f3309e16455ab1e96118e8a905d5597b72038ddb372a89826046de66687bb420e7c").unwrap(),
            ),
            (
                b"abcdefghijklmnopqrstuvwxyz".to_vec(),
                hex::decode_array::<64>(b"4dbff86cc2ca1bae1e16468a05cb9881c97f1753bce3619034898faa1aabe429955a1bf8ec483d7421fe3c1646613a59ed5441fb0f321389f77f48a879c7b1f1").unwrap(),
            ),
            (
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_vec(),
                hex::decode_array::<64>(b"1e07be23c26a86ea37ea810c8ec7809352515a970e9253c26f536cfc7a9996c45c8370583e0a78fa4a90041d71a4ceab7423f19c71b9d5a3e01249f0bebd5894").unwrap(),
            ),
            (
                b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                    .to_vec(),
                hex::decode_array::<64>(b"72ec1ef1124a45b047e8b7c75a932195135bb61de24ec0d1914042246e0aec3a2354e093d76f3048b456764346900cb130d2a4fd5dd16abb5e30bcb850dee843").unwrap(),
            ),
            // NIST FIPS 180-4
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_vec(),
                hex::decode_array::<64>(b"204a8fc6dda82f0a0ced7beb8e08a41657c16ef468b228a8279be331a703c33596fd15c13b1b07f9aa1d3bea57789ca031ad85c7a71dd70354ec631238ca3445").unwrap(),
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
                    .to_vec(),
                hex::decode_array::<64>(b"8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909").unwrap(),
            ),
            (
                vec![b'a'; 1_000_000],
                hex::decode_array::<64>(b"e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b").unwrap(),
            ),
        ]
    }

    #[test]
    fn known_vectors_single_update() {
        for (input, expected) in vectors_sha512() {
            let mut hasher = Sha512::new();
            hasher.update(&input);
            let digest = hasher.sum();
            assert_eq!(digest.as_ref(), expected);
        }
    }

    #[test]
    fn known_vectors_incremental() {
        for (bytes, expected) in vectors_sha512() {
            let mut hasher = Sha512::new();
            for chunk in bytes.chunks(5) {
                hasher.update(chunk);
            }
            let digest = hasher.sum();
            assert_eq!(digest.as_ref(), expected);
        }
    }

    #[test]
    fn block_boundary_lengths() {
        for len in [111usize, 112, 113, 127, 128, 129, 255, 256, 257] {
            let input = vec![b'a'; len];

            let mut whole = Sha512::new();
            whole.update(&input);
            let whole_sum = whole.sum();

            let mut split = Sha512::new();
            for chunk in input.chunks(11) {
                split.update(chunk);
            }
            let split_sum = split.sum();

            assert_eq!(whole_sum.as_ref(), split_sum.as_ref());
        }
    }

    #[test]
    fn sha512_generated_vectors() {
        let test_cases: &[(&str, &str)] = &[
            (
                "e0c86be8b36dcaf990030f138e8dc6b8cdd2cc7e2bea08abb44ba72b2145b0cb01be4598456dde48b5d0",
                "22082dc74a3c799e4cf7dae011424656212e7a264e0cb39fd27053680d5b30ede94777e850bcf2e5d6d55f54f5665d991bc8c3f256ced67758c44f187d745d35",
            ),
            (
                "a64f9629603789f5e8065d7b3cc0fff5606ff07ac832fe6a35f4313f2806eaa9d964d8f82ad0ae65f4c1a97fba787aa1571216f50836abd386d3258a9d9bc8",
                "6b0962dd98e1571fd2fe9e35f0fe136f6e5cf875bfefde37ab431c7f0f5560b9ff5b9b7183d93bd54024b358f8eec108a92a079934a1b51895c9b85ce9360180",
            ),
            (
                "3a8eb71f82407273a35b725937d24b446cd1267c1646",
                "15a4dfa53fb31f514609ac83a99451aae0ebe396eb729b8129e8630b504fa0475dc363119e013639912da4bd3b852e7ecb678300715ab743181959bcbcb92178",
            ),
            (
                "469a8a2dd0e0ef9292874c9077276073",
                "f7797c3bf183c888386fb608339ba8fc1f778ef6c8bf2513bba68cf8306225c1b6b3a136962ba77260cff609f80d9622bb99ef4c820602fe7b31a9a5e599de59",
            ),
            (
                "3b0acd954e665feb5c870d9eea0221191f4080f2d1d1f3872f322716",
                "2be60f06d1ae6f9376b93392a9b17cb94de4c4c656d1eb7ae1ee98c9ae7151f534dd229fe5db7d435cb2667e1abcdee77ef7cdbfbfd1b3f9d28017e1d9d90975",
            ),
            (
                "e6babd073205da831d670810cf2ee6d462442ad38579739383dd6f5e304583ce",
                "1c7d0d85d8dd3813bd71302be1ce0a0c414ac373303ca1bb3ec4e8fd6c00b6062bd2f50fe3b9d9c788bfafd7938f38071f6133b749f72352ac1b06f0a442176e",
            ),
            (
                "5200ed92b4e8ab80a173cf3c573b1c6c034c4df24781772c7f46b1c26b0204e1fbd6400a472f300a",
                "96a26e113bd76705ba142000ebdf1d396d0bccb6e7d171f9126d3e2671ce5a5d9d281173f4c326a0bc6aec367dd70f4652d588eea2800e9939cad50872c62c8e",
            ),
            (
                "06dbf3f418d49dcebf22f5679cb400",
                "d1d295484c93cbe9e7ac070879a5e9358e3bb7f84cb507c207d7ebb9b1eaaafee3e4181f8e72ba6bb5817e012606c6ef81d5e011c451e637f251b86fd618a697",
            ),
            (
                "3c243121",
                "fc855cbc95040d463feb330603cc0f7be53845f3504500dfe8480f4ab8120ca56e2e3192d03643c5056535e0df50ed554d3eb4bf5c100fe934b5efd348fbe54e",
            ),
            (
                "987d5772aa8c2afc0c832e5bea8d43b9a52cd53febaeb2836212677cefba",
                "74b7946874fc1c0adf13f5b2f623420941181d3de075d8474823e36c73a931754d8408c33e7a9bf7d7f1cbfaf73daa9529fc0b42e7043de900b09a1d2283d49b",
            ),
            (
                "4063edf941da1bde21192455b847b7ac6512467487be",
                "1786eb523091fac3885730f6545a8e95ad643011c63ab5765d078b5a1e446a87c67c8006cce42fd53b448b0d2ac12928cff9cb7592deaed51f06ca8955600e89",
            ),
            (
                "b79e43c36fcb7b89bd37db0722221c77bd36e428cb3b7d808c2abfd4311f194a6fc770c20c387129975bb993609fbc6c698c78ae",
                "547d3963d8dd6288c8a9b2a491532440b8f47238b57dc4283c6aa5e2ee776f68f02561e5aef3dfd5a9a976a9707af1681684111070174c62c1f27f7b013d282e",
            ),
            (
                "a8607f13fa39c4a5d10abcf30a81c7fc154f3c651566aebeb952c1c7e089e3",
                "c990ebbb822903996ca455dbd0624e00f54e34964fa073f0a74ae71039752489d1d9c51ba43af5a9d9e89c454a146d24b29edb8b6b894aae61e1ae6c3612051a",
            ),
            (
                "8dc668d568ff75af55d65adc7a50017d3d550a42682902b5c2724e2d9506e29420",
                "b549818ee940a0186ab0d33e207255529ee4426640004104bde4848665576c0518ea44d0c42820de1a820be79a81bcfc22a8dd65fd68fde243351c2663994ea1",
            ),
            (
                "7c6aef47f9354fb6d48b940a44",
                "bd2ad3d098981540dc494475a9c60f5b37c41b3d88b3d87b031344c4bccc30d1558906d532699cf9b3d0f9ffada55720a2d47bcbc513182c9de8f72fcdc243f2",
            ),
            (
                "cb475978cdf5080ff76f54652d27980736f0edcc619713111c47e2ed6dca79819b8e7b56b567ba0b",
                "95c6da1f16324a1a0d48a953c174c6bbb514d0ee48f476bd8e433aef0bfa104c50d8eb7eecb8d9f93a5fe784e5bfcd09a9a629ca21433701571765e2935f5b60",
            ),
            (
                "5474a866489959eab74ff8c98923266ba5570658aa0a9068223fecad9ad192944118e4273948642c4a4c22e22fa9410a7eaf227c790d",
                "a7808e1191d530a4791a47d75f5344c9e07783805ec75ad53504641e2736d4ee81e585fe77a69592e87508e703e0cc0b8be0fc4eb13bbf6747a0af153a4a5ff7",
            ),
            (
                "9d0fb8c7b6aeda0623eb0d190c56fa37baab0026689aed5e7dbeafdf2cec0ac8bbdc113c34242e2cc996b48b39",
                "60f268641f56f1421443d2719a2e4111ee8f6013dec0def23398e564ef35f3197f0bb27e910f45c6a28aed57a83ca6341ff62b1a468a889b371ee3947137a43d",
            ),
            (
                "5d98a392cb6ae460af849f3b21368c3befc1647af0ec3156a80d847ca94b76c38cfd92fbdce0f45d66e325a7caecf363be86",
                "2cdf930001c060fe61cf24663ba91401630a70d2bebd8fa8cb56d5e3379f4c7c82c5ef087ea8df12a3aaab2f01ec43acc5a32363a41889ce445bd2fc8e5c8b5d",
            ),
            (
                "29f129c35b0e411f",
                "a41aa8d798b0e4a7c1c7597ee9c7cf326d14a067d196c568328a98d76105b4705bb1947682d412da0ba4ee494858d7dd687c124ea78afde7853959e136f8402b",
            ),
            (
                "cf79abfc3bf3831ef5df5629ca57f88d9ce202e3c367ca58d34086cea5165b5e1ce44004fb38462b9098686073160e5b",
                "1da421629746351fdbf10da7826c20e114034f19e42005d7cdb26e098bf9b0508c666540af1cea4abef41b6e5161dada7832573ab76cf0069d618736bfe6f9dc",
            ),
            (
                "55b5a6fd17628e539174fee3eaa0ea9d82517f8be86b03979389805117b9601c",
                "6c7944b5cc7686541efdca72a29d52b950cbb7a57ea5ada4c84d1eed751b737276fd1953aece41a310006aa7f6240dd13df2b6d469dff9e83e376ef8a484b5a3",
            ),
            (
                "b131bbd5a0ed1d7ebaa62700166518e18b5c6196f35940cf01f3a9098d5ac30c714a5b1d879bc8d69a8327",
                "a12a542bf5853810fc4ce391d98d06e9f5375882b98f47657eb74ab776aacd7779507c55b3e46c1d4510b8e15df88e3a3be3ebfcf13bf82215179df0c031a6eb",
            ),
            (
                "dd6f1f73a7cd8c0f87cba02c197deb790dd5486aff91debb",
                "4aa1a3724cbd1bcefd0503c562dc4e3311d3910086ab792a51eb692e647dc64e949be39a8b80914bedc373cf6c6182e79fc64dc01c9101c9282074ea22140b69",
            ),
            (
                "2774d0cb5a85b3313b0f",
                "2bfbfa5a8a3e9c937fb00e0ecd4c85fbb6077b0800798c864eb5ab07831a884df4f4e5c529b643fb9334c96647b232c166a1e77732de5904bead1cfa3b08dc56",
            ),
            (
                "7e9e2637e7638efe6ed9d3517e22603698255211d6b4ba",
                "a21061f1728053587db3a7afacc592ea829c15cda6414cec0999999545b918a093d4f58bac3cff1b9e6f56c059f81230321c8435f05d46a1581c29166eabeb32",
            ),
            (
                "574e",
                "ee2c190cdea87c3075bb9b7a90c8c0d5054d83b440fcce4369a5fca804eb9b4cd869fee4e902410f7bb111597cc3d5cc510c6300f88ff8a00161ad47c9b86ae3",
            ),
            (
                "6342691cf9819f",
                "0c6651ebb489a5fcca4de9827417f3131c131901aa9e9be75ca6aa48fa36eddcc54591d4b40ebeaf817f3a09283167974863991391dae5fb2e91150f8b9fb9ce",
            ),
            (
                "a4ed28df5afc",
                "5b49a5d84537a7e9a9e4441af32b338153b606475b41dca4a95dec6e65ceaddf75b8e35999da745c22035fbafa320dca301881ebcf9ece9c5365b7c442d2e38a",
            ),
            (
                "6a625b8c352ca8100506db548928df961b478dfe388f",
                "e971c554b2b9628e3cd445668b00cb30bd5b7885cf355d81c68245644109c769ce0186073039743b3235c9bf7998f9be56aecf6c8f229265935f294c2c8b7749",
            ),
            (
                "9525e0bc7b581b40bab093e3b5162345d0d722c530fb2ffcd9c962d0a97dc106084a1ed8539a232f7701511f419cb9338cde4201071a502d6a51c78c",
                "bc6413b1736ca15b0a35c1245c512a191f34099a15d72fe7208c08e2bf8f9a4a8f1ff64db10dd9291ccbb6051b08400d2ab0bbd21973716fe2d7ea474f911d9f",
            ),
            (
                "bbe2e9342f7b0baa0242e98bbf5b1ba4c20d51ef1f7ed90baa12b78c9ff41d2c7d878d5fa0515c2a22",
                "8e525a071919a3587fad81eb142c862448ae7d78fc4c38ef9f1ff6189dc5cd093f97cba1e5c76cd8cece634e075a89bd8c1eec9b5029bdc12e9ebf2357ed4633",
            ),
            (
                "cbf7b99de261967bf5f379ee69904de5585c64",
                "2795db95837ca35b1a5787299b90a312b3dab0b4af161f9fb8c6c22e403b9f22e1ee28c2bc5e632e82191d0e0e7d8bd210a609bfb161b9bbc172f43e33735683",
            ),
            (
                "71419b5c02e5d59ac94c9b6d16d2df6f",
                "dc8be46340658a6371a484866d34d8be3ea1308dc743bbbf4bf3ce19d8a66b259812b30554576f9ba0964b528821b7c1679667cbe16c83ce6ad3ae6f8ea77e58",
            ),
            (
                "435daa4e",
                "5961c4b0d5548689ed161c4b8dbbefe08078006356b5aef83d03ecc25ad871c20047093aedb60e024b0d3cbc2290ae465a35daaafd4b4595b38e7c20303c62cc",
            ),
            (
                "34e5cb8d2813e0b79853f1cfe2cd32643209dd51481c0b87860c3cfb519000edad0180109f",
                "50c47a6f6b30c74019944ee8ad6057e251d18f73499dbf87f39882baaabc4d63ded4dad21e71a0f3f1ea494ba7cbdef784ad5df3f6f9350a4c3195f865f8adf1",
            ),
            (
                "6039de86509cf3d1d62654e502ac68467a43a413b0d647b1feaf01833e8d95a26e79810c15c7",
                "80693367ed8ca5c85cc2e2b068fa81139017f685e9d6f5eb7fddb030f25b6f290c1cb71543509d7b91f07b5f0c1745d00a589864e8c9e9f7ae8261f08812608a",
            ),
            (
                "00945f34019772b167672c148804dd569fcc30d1ad388f331fcc338e361b3a2c7308b1",
                "c7ae8ffa19ae46f83f8249a92b0dbc7d1f71461e7ee68d03dd5824179a2c2afa6c7a27094c74b4b5d6145151b4634646188672fb2e3da034ce70af3ea0088c59",
            ),
            (
                "86537dbb277a050a36449eb0195d39987af918ad185da233952d9c71284c5c92eb7bb6d0ad72d393ac6a34a03b395afcc10040688c2cb110c44dd119",
                "ff825119df56f3d203858c0b38602ae3111e8e957ea0c665a93786b23b4c1d462d351e9dea1b52e2e681ea5ce1bff0301006df0074775ff5904e63686f331fc0",
            ),
            (
                "eb3c330346ea454fcad8832def54ea5d931dcdc5a3eb76dc5ef11d",
                "79e320f94d5b9a03c99813e37909eaf45f324c3379c09a1b36dd34891dd5cf003743420105820ca3de8c972b782278e7822a582c99d2184d3cc11af91f718c7e",
            ),
            (
                "4dc3e8ef5d0aa78160",
                "1abff8e3dee13d7a957dc320fad6de96420f7b5ee4c6b5fa49efb52310d69aeac5757ed623f6a1340ea9df75c84cb59aaf2682c7d2fb558af526574a798f5fe0",
            ),
            (
                "7b",
                "c2d03c6efb16c3f8064b0d059e45f951f1748421a622571a52009ddcc2a670851e1ad0269fbd81d45856fa20ffacd081dd20fece7611420befb49eb984bc23ca",
            ),
            (
                "0c73660105e48707b1754fef814a6c",
                "7535ea278364ed7f62280268af2976b90b3f15ac7c33042c4785a38ea0c7602f249c822a5788369af5fe56e30dfd7c5b4d436d00a88556688643c9220fbcf750",
            ),
            (
                "809e4462475d8dd3a299b2e55a785c263a1ee5c0c745beef1a",
                "1d1688c8e1cd783228d79f68aece1203236378aad94d2e60c415f8853dac7184d215ac8d74e60b2d1835ea1cb2cfd49ccca44e55af2059e08b3925e5894c5709",
            ),
            (
                "32480a0732cefb5b55398eeb2c",
                "32b385bb4f89f81e3dbb309ecd1cfaf5d0a7fcd93151c92f8668253a3a20fb771dbd1582ac638e7d43e6804cf4bac067aed987a9c44c3dfe6d98f1edbe8ba934",
            ),
            (
                "8dd0edd059c3",
                "c5140bbf2f936dbe1b53916639861e1b528002890e166fc03bc02b6af3aca0cbc12c04b8ea080c6207b7b7f33e2ec6f934ae65505864ccf35697f5edf73afc3d",
            ),
            (
                "6c1e61d22f6e9a447b8c5a4dc942862f60afd6d57b5c5320722a86d5881f4ed1820b6bde29547dab330546443ad97d24341128",
                "b9ebdc21ee2bc895196221b02c08553caf205544715a53591fe4e2223e366dc0e11d0bc93fd159f9ab0d5affe491e33ea170062d4d329819261eec3cd4c261f5",
            ),
            (
                "f53f1a4484d379c37b6e5afc058583ba8e6219775441e5d443455c7481a4d470731bd57a4542b0",
                "8754c9e65b4235a0287f884ca287c5e8b186eac7e936cc9afa9e521d1ecede40afe44db12c28a2bed0d54093c5c5c4d8265426441c7226cdbd6482261b27e665",
            ),
            (
                "1e78f1e8fbe4acc647b61825865556cfc0c93ca31783673a23d9314f6beda45505c1d6e2211ded0d3696a0c2c96c64da8ebc045250b2fa722a4340fb28b3",
                "44073c28ff016e9c52ae688ddbd5f9f1df7c2f749bd07ddcba0c537ea65c088d2c9e023e0ba4bd890a79fc1c779eeecef1c6dd65377b0b0a0f63d2e948584921",
            ),
            (
                "73cb9f33074abbc9dad9f3db1314f6175027dc1f55349143dde32c00952a67c2a92f8b7262f8a4a8cbce834b8fc8ba8d880f8404c69b50c9b034142ba0da9039",
                "c39ad65e32a05666286c5c389817c39c70d690f64dca335f60e70e9df2b59830392f255148540fd8aa587d4deb8a410bfb8e7e2786247b28ecbd897dd8cf4485",
            ),
            (
                "58bf3192b76c22265c903d3722a9a3f642b3a7c8",
                "59d66aa726f7e323ef95fd98a7ac44a5b8e408913ac31dfdc68b1ef49286c70f487ad7a8908e2a9d77bcf8a1ee4c5db99a2aa58d702cbcbb3bbf2b9c1aa2cb83",
            ),
            (
                "7af57a131341a654ab76e94ff89bfbc99a5f823df4477efc3d6fe751b4",
                "d3643c6223b0a1e64a9316a9cb3f3f82021be72c272223e38e8ca6e8c0dfa3d66b35beb975d17d21c26461444d26e84317f64ae94965360d26d63323ea0a3f18",
            ),
            (
                "962652236b4b3bebe15b2328",
                "bbcfc7b84edb9a5bf9d721195862256b76cb73c509399ad585466b84e0cf19bb3ff27148a7ca4c8a4116985c74cf2387235ea054d306ad13457fca9366336799",
            ),
            (
                "66e9f3ac085177b2b5b852d4a8e055297107380afbb73df3fccdd96866224cddad897973e354812ff147d3d77a",
                "d34aab8f489a7d38510da5eb51657744b9af47a6fa4a022ae5ea13be9d5b512fed4785d61a5cc833815d1830b52359b140292159504d530fbc5680ea800b53e7",
            ),
            (
                "773b743be9ddbb78114b747d18ea",
                "c7aa0b821cc8107e24d2f18499dc7088f5bfdede104a6b50007518f23da054dc287a218d36f10a748d45aa131f6ad3528b94f808c62b44ced88563a0c4908495",
            ),
            (
                "5fbfdf85de807283a5b224e7c8e5946372e4d56d7099c0b5d964be9901303d30e388a6f2b37e975763c5f7afa9356acb229bc2cb21ae2588d2f7eadcd77e4eea",
                "616b50d4eb02734953260d14618258715a38756f8cb0f65118309b3c9295f6a92fee6b8e38198c81b06842aeb54787feae47e12e4e2a7dd550da33f61e8bf96c",
            ),
            (
                "7db87dfa33a1832437b03a9df22be62c575cb7571eb308ed",
                "37ca3f847de86372255411657a8d875641cbf405118fb25e0b49390aa6cd35d300258b4ee39d02d166c82bc8825b6c20e608eb5503c6a4db289156c8c446728e",
            ),
            (
                "9e9e3798129da97927a0bfd351dd1bdf86093046d69c14683aae240fa297334f3d122a6dd5a01b0efc635806cd969d194fd8862b1a5b",
                "5ff7faeffbe1589bd7426e2327babb5661b94adfecaf3511152776345463cc94ec79745bdde157ff1488e829619d5d31363db6590d561aef81449fed8344c245",
            ),
            (
                "ebee3aaf36148e74bc4aebd56366791e7dba",
                "a2bb48497120d4e95c3cca234797e5b129bc076af0d79dbfe668a95ea8290650c884adcb024c59deb224bc98381fa08c3307fce576c7befca6f3d187b5d73631",
            ),
            (
                "7cee9290a53731e1535f8066f1b1fcde6644994419ebd7a9",
                "327dddb5742d805ca50384c0db7ff6a8a7946923eea234c60e50ca77f50eab468572f292abcd0d8a5688153c52406ab87a82b69bf974c03b07c79dcd4d6dba7a",
            ),
            (
                "28120c8bf844e9e6",
                "9ad15de224077d664a09e006672513ab278e10a856be2d75d048caf717198c3d27f8e84fb15d2b02342910415a92b9adae0adb7c04f62a393a4ec94dd518efac",
            ),
            (
                "a37220a20f2abe0b5cfdd3c920ac1548be306cfa",
                "97a3b63fd4f9b2a601c438f206825b444de4c31256555e7b631f4cdb003f8378947f65795101c9dd049a496098502707a4af024dbd82fb35586195a80d8059fd",
            ),
            (
                "697cedd1c68f3f97cfe9eb58f48763fa",
                "d8aedc4faedbbf54612c6362720f7a4d7f3ffb1c10b67617d1919468f4474b8fc18f2f590b443051b5b23f491b51f8d5bf97bef76e46d3daaa88cefc7746e630",
            ),
            (
                "481d2728df022dc759ed4e6b2b434a8eee75adb3aa2110",
                "225f6dfa21e4f0a162569c343eda92eb5d1bcf31cdec6af9d6fae9dbaf0dddc20cae4fc8f0fcb61133f0b2e8ee474c72d2a5ab7c0b8a88861a6931dbe4b5dc7e",
            ),
            (
                "f1df8ec7aa",
                "9d8d71922d295d4c1504fc03ece8b25e733b5b90d47f120ee2fc1372a31f98ab562b0db41ed592e5f5440ac0a70d520aab4dec213fb9bcc645c291f9d4de47a9",
            ),
            (
                "47f6f4646c11e634dcd703a00a0d8320381ed9825308b64832270134a852ccad071454c42459daa3d3f63054a61b103a",
                "9a9fb453c1d4366b35f7ab29986ebda593ded76a61d908ec0d0d469d512d841e5cb275e3b6d080542475762945556b40a89c55517f0da7687dafb43d6d00ec79",
            ),
            (
                "2dc855f63f4784d34d026254f7cc79b9ca8aa9f649900cda38d40140",
                "55b65a5aa25ca2f00c5cc598171c6dd7b9d2a5135b93140e515b72460d286df412796af955d488dd081aad5c94a9bf43089c220e239461026255f4d23c7c2883",
            ),
            (
                "a16f173d483cb373bbcba672857e9989fd7316378dfdc19015009c8d78761d361d76f4c5835babcfd1e36f6e8c95b96bba17587f059eff7dd5",
                "18dd83e359cad3d1b3df52c75a1bb7ca4230438148dcade6d4293ed670abb41c64fb7e5ccbd4879b8472c769c24fed652e6f6074dd2fa98793ca53ecc73e6386",
            ),
            (
                "e6510eccc564d3d4cd73cc208272ece49a26e67f5dfecd6001a10de0c7f54c21e6b13f6b287ecb8adcec",
                "31239ac3fd5b996ff19600183d7f38337845445cb3fd6c7df1e5011f5bda105bccc3cbcbb9c3791e6b1842161eb49343d225bf5df101409049aba29e4ec34564",
            ),
            (
                "4a5e89554109aafc3892e70b08a68ce09c1e4a0292798a3e519cb9",
                "53cd54ebfa3e3099dbb7cd6fc0acd0b18a61d25c225ed65c05f34ca0c3624456af0d8e18420a3666751e1f144733c11f94d1cb2cec2862203745a1c028fea990",
            ),
            (
                "fe833fe13255b555718ccce8515a14aa8e626aa4aef586aa493036ae89dae7436ecb6fe4064ce010f5b7869f32",
                "c9380ac0cfee2d636ccd381904caa977b8b0c4528d23bce05fd855e1d10c0f23731613c19ea967967eb6dfc41aedb4c8da3efb156ef40af2ccbcc69cc5ba23a5",
            ),
            (
                "0291c756ed8679a6ee7fd8ea2eadd3a3f385aa57",
                "c7bf8f0dbe385546a43dbeec11d8a0a4f4d53f2bcc5c67eaa99669499e5ef93db200638035efb46de783547bbd2e622017b0141c667f44dcd624357b292b7df7",
            ),
            (
                "8a75cbd5d73d75c7b6c55bc62bc55f7444dbd28d22d3df20453bc2ad8d0190220fc17aac2e6c06f4",
                "71d7d4ddee133e2c43245f487511ab205d2a000335b4137763f09cb96bae285f5e323e21bd65dce532a2f28cff1eb5b48a99e8c2f666abe5d588366d7abd0347",
            ),
            (
                "326e51b0a90f4f82eb0877e14d2a45d06c2876e1a2ce6faef7c2ac1cb50d5615fe2788",
                "e1a61b0f853a1934fda80ded63a4a76fea6589c05d3532bb4e3ed18b99620a82fcbb8a114c5bec1126c92f2bd96d32a5b18a19bce7002370fc2d9843a0343d69",
            ),
            (
                "d400b941ed7ee9d2a1617f7fd2fcbdb947d8d84391906339a26aefb6b493dbdfcdf53e35aca20fa611d452cc86c726",
                "8f2de46a317ac8e5e9c55e0cb4cf7b9af2dc37058f9b32425253626525b6be768bd154d1bcab55577405b66d97036b1a3132b1b86a01341a4774e8ef40383eea",
            ),
            (
                "f16eecba457162d015703f61b8597cb89332c743dd79e9a95db881158e599b6d6d022f19d7ab67c5c62279833de3",
                "779dc71919b42629c1b61beec6c7ce415921f87cf42d59ce992e4e959030a9bf5254dd35e6694c153866f4f027ed5d1055f6bc34746d3ac0e1b8dd1565888107",
            ),
            (
                "e0a0f3320cf6976bee604773c276384293f0e3adf556a8036d542a70095322e8edfc",
                "f35e88e8e4933cd0d664b2fab57590eea227dae7b1d43dfbe556e8c56b3d983657ca510ab513d42d7b8bd1f5e3d5298b29ba0863933b7d71fb5fdcfcc1f0d734",
            ),
            (
                "5a43dd1a51e7444ba0c10f5f865fc6192be4a82e2c642fc7107aa9",
                "e86356af6bf5c5e0a1014c4578ed4fa9415dd21eac78de6dc35f5d1502e332214148719df50fb527c54755de1bb03f03748a998592e775f7881fab6430da410e",
            ),
            (
                "ad5a02d372fb2aa1889d96e67965b42bdfa9762fdafc3319f573fddec6db9b8ff486b3c74860bd7f4a0d1c90a0bcb7d6039a45651dc850a53b20a285e7",
                "ab17a25660b589c274bdfb78122e0b208f50c623727bd7d2d86fb1d824970d9c60d0978ee1e692d501d43ff8216e71c270f034cdbfc00e39a20543a2808f7504",
            ),
            (
                "d7050b5f94b275afb87e315e70905ce87cda9004c585dae55354d07573ff5de54b9b2fe41b05b3a79fc13f93569257b64a",
                "f30b1658b82110b6a3a4cd8549a8c8a87520fdfbda4a746bcd84dfde76ae756453472d55c363d7639997e559f4e3c06e7bc9bc791f218ac23173d0640d9af247",
            ),
            (
                "4fe5895e4461b073881d443e7a98f2bcd208cfef0147fe43316a862ea080151933a2cb681157c29c4c20",
                "a087ca8cfa0bb20d7d6fd95d73a54d6ae4ebaf19c3c34b4980562d9e954b0b7f3a6cc381d94d8768e9ab7cc15609ce4e8c705e7a6af2fd5f79babd42153a11fd",
            ),
            (
                "57925245f09efb390a2e480b7caa31ea823098f2ca29369ac3cbffbebd449e0c42f04a80fadfca7b3263",
                "473f47a84454691f5d8fcd1d050ccb498c411277bf3330c274a46a9c2ff862250d3c20e950ada13b01c160f736e738d103fb6c98c500367cb357afa9f4e14a34",
            ),
            (
                "cad76754d4054910f04ee80f25469e3bf44d1cd372b8a6b448cf3b7b8264be313355bb40c586b6cdb7602e12ce34c0198a208693a04066c8ff2fe869c27b9f90",
                "ca5c1fc00f9519d4463fe4ce1c5b5b9180bddbb63c77c0eefc1962f2645df0eef67fed67c465112ba2d2f730a275e66e1abe09a59746369ca060d053296ef63d",
            ),
            (
                "c28ca4cfef32fcf86c4fcf271ad3629901a1575e9b718bf4d2b9e6c82d12375f5b6955b85b87011d445cf1b9163bd8bcdf0058605dda",
                "17fe4d1e5af84b0cca790365500e43cf13809b90d7a40184e37780ce1b6c62fab2db794ddbcc86ec59679c8985cbaf0fcbd31060666e3de22b2f934cc1dda7e7",
            ),
            (
                "a86371a54784d45cedeb76149accc7b43f3e23459f8d7be8075c2440e12078b35dcf112665fc",
                "ef8cfd58785bf0973d25833a71688356576ab0935abd5dc6e399a6cb84d76c12b499e8ae7afb6c2ecbb94861f7ec9c24e4c5eca89ecab4c2785ec217216d7a4f",
            ),
            (
                "6e702c9f408d0bc9ca239f94a41054fbd63f717fab59ec012ff8de6d3fd91c48e626b5877b8c8487b45614bd5259ea763890ffc16548ce3cf2ea641d2528bf",
                "58357588cb1a3604b861b07ced9e2f17f1119d86d67fe7cdc69be0489694f1a869f3fcb93c9a36f0de13e2382db813c70c9f775c047bbe8c7db1240e727a5391",
            ),
            (
                "dd6be36d77423a83c8b53713575b684d8c2b66",
                "a292be9dbd48ee75999c5bdef7e6375e5a6ef2b26bcb4a7ebcafe830f122bfb6823256a542c0264131eb2eb4318a377a3f492ccfce2dec7f3143f263cf0d930c",
            ),
            (
                "2901bca38a397df5c444f230a61bc1f2ffda07f375a8202d8ed02ebf17ad604e8e199a7c1a7ae4df8bd6542fc821ef8abd33778bf7",
                "8a9a83d6d551cc44937a978ccc7ae6ca34ddd8c2894cbee85db17f7d3a892020c2251a1dc2529d4283fcdbab159313b290c398558220f79fb31452656c1745ee",
            ),
            (
                "c26c0b70740c00ea2d664690ef01f669cd6c9e31fc2e7e5f967880edbf1883b928bf00fdd9c3b940495a62591b497e75",
                "2b06bd73140aaf4afbf0bf5ecd1b64ea4e310e6bcac7b5900d3e42ea0e213be34ad7f7543df84ed52c1217df6ad762e33ac6a203d02a914d28163d6ed9f8bc03",
            ),
            (
                "d56dd5ef31d54d7c94db62561f1cce670b2f579cf0e9facd30",
                "25f5af5272dcfd0b6c446804f8b74997617d12450dba3376b43fe0c9c23c2bf4acf4e3019e232b0a5df111421eab1e6da83d0d59f0011609cc3636b9235c3c1e",
            ),
            (
                "b7ba8bf3b6c164ca9ea4c98ba51d9e56efaab28942e45c9ad36715bdce672f2b896e9db2ce634d",
                "546de76f836e544745432391c148d9427d5c61086a16b9c0a0e51eb48d796797b0f3424bfb8ff4f709e216d5d65fdbebbd107ad788a9a9533b14912aea4fe91a",
            ),
            (
                "331e4806a5902df8527f73be5822d245dd01803a353dbfd016554a701315f629058074aaa426312114864ca533fe9e465d87206597",
                "d0f8fccd2123d58854f66c2d70c7924215433cdf0aee3d07bb7e2ef229df7b17d58e4dbab2259e8d71fd4c9616ba87d9cc12bffeaf47c16d0bed5ea0e4421ebb",
            ),
            (
                "5808ae13401d01da61e41f75de6b4a924c72e5b9",
                "fa32388438aa6465e85fbffc54fa1332c907c7a206a742d4efaf96f1dba95c607ba8b7913f01a1997b94a2476ccc2cb1d7ff55523234c6bff69e8592c0830025",
            ),
            (
                "74fc99cb7d6958db460a2c5645195f385271efcf34e0295c",
                "5a3b8c38819e6ad4a3b7e4c1b1b916697d2aa91c5cfc3525106aae688540df297542579b5493b5b6dc7a16411edcf32308418244497f70736821b289db8109f8",
            ),
            (
                "e8127e4542d8bc9c5ab4fa30634a84e5cb99efebcaa1636ed1004cfd9fcbd15f691c6a2045428d5058240e7701519fea78bee57c7f3b5222b7cb74",
                "1301bc5e9b25921a922a6bbe315af70974be2479c5c30df22f426058c6d5b504c8047769948e1361b2d0b3feb1fab85f9e5f1d705ed625871461825a196027a7",
            ),
            (
                "163c13fb1234d1ebb197dffa0cf5496dd71e3d47e8af53b12e8338e107c11d667aa53c02de1118a3",
                "184eec1fc7467c40902c84d04849ff886cdbc1460db23d5522b9e0f10b5c111d601bf5f0c997c646697f8daf8b5a451aaaee9462f0a871ea01afdbb504ea10b1",
            ),
            (
                "e1bd7ba8fc7cbd6caacaf3ab0928d5d21fd8a7b7a836ba134b0692a26d4c1817b096",
                "38c1237f7625f10faa163a16cacd11b14d8078fe5702379320a08e42d9f196349e7881d521feee002c6619ec4dfb4d73fd19bc6a88bfd2be3f67221b8bcf88f6",
            ),
            (
                "c684a2cd0f407780580a502112aba0d7b57932cce836a7b3eb8ee6fc6f",
                "9dad3532555a143276ef5ee7001b77170a13e491c6010fc7afe4aea81cb463d0e4255ac4a345dd867d0cb931bf1ef83e42b65b997fe8cff0853dadcfa8aa35bc",
            ),
            (
                "75a6a944bb7afeb34cc8dd7205215ab9e6c4576af3837f8483381d885d067d0e13",
                "cce0c3f37097021dcfd6e8a43c2cb25f3cfec3b24df881d1265a97ed8610bbe25cf77894a1dec6215ddd4a1fbf6e3388fe5e9eeb699e8521c68eef891e09f80d",
            ),
            (
                "a527a8d4bf382f5dfc6425e6ca26",
                "d8c2415a0c501523261f442a5e35ded49b296e3c90ef0c11ade23dc2bf841048b2d3301671ae70f42b06030809a4d6b66e1101f2391091ef256edba180960fef",
            ),
            (
                "0dd9b297ff1aa0",
                "504c074eb7e6ba5918cdcab9c6e523e16a319ddea9d6bd294072a51d43d184ce10859a395f8289757d1f5ea7008abc11fbffa1f3f7d20ec7c3c58ee4cc80c411",
            ),
            (
                "0691455c3850b54944baf2e7739faa9bf568f5a6462bc37cb6f2ccf7ef3979840c9e06",
                "902125cfd4a0bb54ec62a52145ceb06f30371337a69540abf2cfd8b52d12eed826da97b919e44b4867bb9c50b3dade06e000fc6a144c013473a00901d225d8f8",
            ),
            (
                "15c202646a2147c98a5803b8619f1978592bc7caa5719e8e1b55eeb7898967e5ef578334",
                "a18b98e7de844ebbc371cba13b8da5fbe93902a7bdc398dffb4fbb843f1d8852364a299d51d295285255dab8c5148b3a2cc075fd65705bab21c70d2974fc956f",
            ),
            (
                "f69e8706e6406b7411036a795078cb197a9ec54c16dde378a87e6aa9e1d9317db06bff6c8b2032b2b3d2e3e358ea5832d793",
                "cc1c6c59bedab496b1a88c5db22cb6a9180c302cb77f8e6c0d00aecddb6ba287ac18cfb673f8eaa69c5885c1d7a43e4aa2ecc96aa22e5c3fb670637dc1ba13df",
            ),
            (
                "bbbd9a9cb642dbd12ac0fb4ab063b9d0cfe60a13dfe6",
                "4fe791296e041b1b40c919c162fba812b85c8bbce037bd0dee3699a928f55713053dc9f1b752c1b3bc823dd72183f183f80eb03fe7e35f83a442da7737b0aef0",
            ),
            (
                "d13cdd4e1c83eec3159711055212cf033a8784840116e4680fcf9fa1a8573ac196a10b1bfdb9a6198dabdae036edc28bfa0f6996b34ef3f329a768fa25",
                "b84b92c69a2302b0e94d21c477c54e49302377eb0ec1b6a79ffb2704755f94a3d056bd57521d50d0aa1e9539089d7b6ae4bd9a452bc43010155bd5a3dc40d411",
            ),
            (
                "28f0cb1f9cfcee0a900287b2dc9ce3eabd74639e1a7ad766fc1a5a4c9396729ca4fb",
                "874d99cba320d20eb474a1b833d46e1caeac9208ec0c513254d5b39203bdaa699e06bc02e0edad1b65238ea546b9a71d09b5ea57256425800297768431e5c1fc",
            ),
            (
                "c9cd88355b74f379286ce4802aecbb388b33f9e4e526a0238e58553a83d0a1a1edf69489975a1e0af9ca875d320db49e0f",
                "f183f4e73bc432ce2461120b7b9568adb184d0e3a35921ad6ad72c260eee58564648c740490c5b362608248a4cfd67fab98290e695367148e729d2ae0e618275",
            ),
            (
                "d8fcd1f1b0ef796927f3ed4ede6af8fcd645d5a052d7656eae71380ce7d2bc20",
                "8405eff3a72ceef42559627bd7328d8fa4b49ec56f9b570b921bfd14031df346c203e8eeefb2a7ae2b8e2659f4224480e6362410bf84e89d946149aa6312c5f5",
            ),
            (
                "d06e7d8de1351500d3a2fa",
                "03988a9c35a1164f88bffc51cb119e546187bb064156fe97369b781f942b371efa11dc3bce82db01d71fbafa8c96e0415ca9f2b84f292dbf31811362be807db9",
            ),
            (
                "a444a5c318773bacf48aec6617a53e2052c58d46963465cdf3",
                "b25845c392df24432f92d94ad6b9e02e4a09ed5d0a1841f5eb8115c465a0817be205b9c7a8c4116b5bc9f6a47856a4bfca7c7c156e48d2dd4e11d3734bdd2b48",
            ),
            (
                "0f",
                "c5a7eb2fbd6963d661d80543807aa9461bdec858dfc9f75d8ae2ef050e10a6956469f2cb56e16a62219ae943e8bc64d28b28ee4d65d15e6e4ddd95b626209963",
            ),
            (
                "f43e4364d1c564aef9f7642baaa3ecb8ca4aa00aa25e7c9533",
                "c762173523a08725b1d847629b677678e3266907654b963967f75a4a0c254fd6a3db3713068bbf3cadf02d385b852dbc5fb41cbbb8200d30b08334d0f547d598",
            ),
            (
                "8d7f8353ed8a50181017f48463268f0d1413b41edf1810f62a5b739fba31adb9b48ecded94a6d64960a4d88d97e4424bbd",
                "166de1c99ff5446d8b26b69e7016b6ea09372d459e00f31b7c5955ed8de8d4639d2d083bc2efec55d550d3b37000ef863b0736635243b26b25b4ac9843a2033a",
            ),
            (
                "c704282c8d0aea733413c8e7021e31d3fa870d5021d11dada739750259c248d2dc55be21f203eb082bab",
                "16e28b53d07b9bff91d4628b71a6add44fcdacb61cc3c1e4c8ad8d00180a829481d86538efcd17f95c6b7f63582291a441636d229fa3f7da26cf401b9a5ccf77",
            ),
            (
                "d0aa8f4c74a3a162f0c7851888d369561160f16bfe817b0d6ea5711457490f91e626fd77",
                "148eddeaa166b169231c16521bcff4178ad6520cfa09f6eb23c619e45e907804521ae5f3b5e8cf4bb597252994c9dc67dccb4e0b9d0e6c02768aad50417f4f43",
            ),
            (
                "fd1e7652581f47f6ca349058533be019231f26ca",
                "84d6c72e9855edcc8315f0d174532da4eb61a2ba73ac0f3d665c04d6e4bc7860d0ff6b44f443cd9a89958d468a310e5aa6e91ab5c0ca562c6d4da19ad8169e85",
            ),
            (
                "12f2418fa792134e2814b955469493c90a99050533ea43",
                "36e92dbfd42517590cf283faebab87d6d9acf5c8e22e6825fe7c2c64da379cd50bef42be4afa8f867a677e8c04e33120c5427ef76318768d198646ad3bfde453",
            ),
            (
                "e7b901df5b4318123491d9a3752169657ff407205b055fefb98cb174493dbe1f8f80d881e33e58aba215",
                "ec57835ef6929d85b0a5f05f7e63d0880900df0f5d39214d2c321e1c327c8508e50e9d3151f79556ec9ec1e707ad68ef0596ff64192d396a544b0bedf23efb16",
            ),
            (
                "03350dfe95cf1e160a59d289620baa55c255568bf3cfefa669e29835b5f04e9acd76b0bc",
                "aa3c1e3f9f6b45054b806438958c9e3c6eca7ff311398bb4a1f1aa5876288d5819231639cb135121cabc8351a6ff6e2775c6f03f59ff86a4da63097d18612acd",
            ),
            (
                "d3e88f1cffe5309790387a579d860bd4620e687c98ea44f1f91a523aa80be956540858a2125a0200a2c9082768",
                "f894ead45c320353c3b88e8653eb23406e39847467d3031c7f3548627b2ae5ef60cd00b613b608380ee3ba557df4e4c5db68ba198ff8f18a03fc76a588cff4c7",
            ),
            (
                "724946b592",
                "96baa576a5b358ff2e0365759f4a0ab90e22316a7bdcc122ee7605042addeacb692b13b09c44b0a9c2ed9dbf131a8797ffa92f6d70e3670aa9d5eec221eeb4d2",
            ),
            (
                "cc34439c",
                "cf4c114253ab08457e86615d2cddbcbc74a407d96ac1a6ff7686e02a98e48a520c5814ff8aec1f5af94d15a87f233a3e6e9bb5cb210cf5b83849c4148cf94193",
            ),
            (
                "709065813ac3e12c47036d85f3535d8a9675a7f7c28d98a9938bba3fcc4e60ab27544f92785c76321e2d17276f716613effb1c",
                "3d9502aa56c906492c5fc5176c118d6a9d7db33c3e967c42709f09c0ebc8acf2e7ed8204b62343de40d628a67d7eaed174e0d3785451c3f64004f5c3e1e67c77",
            ),
            (
                "0b9a0dc093ddc0f77112fa47178bbf4c",
                "e30aa1e5f1784335f5a548916c40d3ccc561fcfd7f874a804bf71e5f570e046381b28337bb396e58d178b0bde82b4414be2b6e2912a3f883fc7baa45dd9734f5",
            ),
            (
                "aa04",
                "058f1a27da041a5423928eac75ed054ae59044b8ec646a2907f12045bbca586b6f26be025f4380f0ae5bbb70681f1ff8b009db6d521256db13a4d0b9d8b658d6",
            ),
            (
                "dd72bafa36c92f832cd86e07bf4398b1b5fd80ea4a6a17efd754187636690087afd577071c1d96829a0d3c",
                "d381734cb96809d928dbdc110f7a4de15201ea302278f82ddb650813ca984a5581af6f8d9fe8092fc88042f3047fc2750d3b55d7aae936a813aeec2fa1544029",
            ),
            (
                "62e423974f2eaab6",
                "8fed8af0b1af174f3672718ec59f7a1dc57309847b2e1497d9d72f3d9cc7e1463150068ac9108399d00d28dae4af7cec6fc3740222b80baf4e725e940255309b",
            ),
            (
                "f2568c9424a506990f98162a9514e00945",
                "6007b583d966d485540a85943ca62b0e91a02e8b32ae97c00a961d46a2600a591cbb9150588665c8f9f4a0fce78bee4ad75e551e6ce24ecec3d1e9c01f6f9f19",
            ),
            (
                "f9f5",
                "853c05b920c65e29cf5169636f995ceef2d30ed2ff8b8a7d1f4f7bd5bff85d7735af2930f5569800d8e30351759b39922789e2f13aecc82471f8e4f735dd6354",
            ),
            (
                "8b66003b46b8951c05e05f100ee80e50974d5d864a66b966e01c406416a954f880960e95ea3460a031355d7e983ed1b22d45ccd7739ae1342893e4",
                "68bc794916b9c87f7925806f19295fad5f9f4477995cc95ca303f903e640070b5107be6a09aa7771b602f4cbcc6a4649158bcd57b44737a83de05ab78ea50f12",
            ),
            (
                "6818673e3abbe4ddd5aca8e77a36f775b22900453b73d2e7",
                "5b9f2af5bb7e95af52e113471f4f6ca78f97a981c0e24159154ca04b8374f8e69e0b6de6b3eb9cce93e06188a7f84b3dd06e7288e483ee04e9a03ce29e177159",
            ),
            (
                "b4d0c0d797c6c34a1f0752d001f837e72dbcc56d33059456c6855b9768dac09391834e03d7980b7231a65d14bdb649c165808782e048112a",
                "688a177a2e6ef96df0470ea653fdb4e26befb50d4999c571a441a2d8a4b89aa27b9fed1a0036124ec4ab3e619dc281af6dc06ad8b73c5f1871f77bdd16917640",
            ),
            (
                "2626092f5d3b0f63260d6ef68aa2b16c285a10e0ba1c80311e",
                "0448b851e83e36f4a869be5247e0f1a37ad1da21f93a56532621a7214fd0348efd7107c142b114a3525ef43ce38de4bf5a296efb1bcb5169d43dbeef3fe2c5b1",
            ),
            (
                "63366de6a7bea4c6e893eeade27fe6fb6ef0aa3b43927e070f0dd387169b874db3",
                "dcf0e2a5a5e9265d949b1ae4a3c084ca6099a9e46e7532c56f6a967d457b881532d708996df8c6b652f7f3589a54c7ca65078a1b2e74bf1568cbd0b73713ef10",
            ),
            (
                "df188b61e9100b1cb818c5d4e278c2da140fe5f7857aff56909b4cff2e62eb517c57bc79a7801285a772e7561099b730165154f4395d",
                "0ba77b34ea5e452a8190ba1320df01840354c320549d1e162ce24217fa83095dc42b42a92291130fb423128219138bcb839d9aff60d56d244dac17a333bcc48d",
            ),
            (
                "9157b16354f0aa5aeff0458555223a267ff848f42de5be975cd5857e8b3eede8264cd5f9546144aeaa0b016c",
                "bb1a20282fafff086871c4c9a8e06a890854602b925d6995ccfded09b2222453fba10e208b9e9a83f23db05fb205e95fcb8562ac73a59dd3a9214823896f611f",
            ),
            (
                "667c17d7422b5735d5e7d6748acfe89cb2fdf7a3",
                "2ee62a68595713b52a7e337d158397ed33ed064b35b48caace167285a1a852976c37d305614bbb24b67510687062a635c7305684be2da6998a572a42e8bb91ef",
            ),
            (
                "744f454d6c4e110ee3937d0b7e2fb0ef32a3e965c012",
                "6c6d815e610a308dbcd7bdbc7b18b803aeb4d4f812bac4aa5a6fd9508c5064e7c81cf60b8498dac0803ff119acb602962b6895980885edd8cc82d120463189c3",
            ),
            (
                "8d21e23d63394a4a19bc4b3d50ec0ed2ae23568eeb4ece856acbd751259f69ec45bd873a4fa85061b349924a37ac172e1bfdbd93e890bbea",
                "6793644071d3b20bfdc96f17e886c72e4a2c50ebeaf9e5bc86f8f4be6e1479e4a9f56885b94ba7965bd3255809e64f6280eb019ef69b7206a0add772a3cc7223",
            ),
            (
                "b439a184d96ea546c30995d9bc6df2fe826333658ad7f41c16d6abf22e4c8b0c3425",
                "802c7547752d158ebc2a7fedb58872b28245278715d7574c5b90af2f4a099ca4bec8316123115f9d518337884e2aeddf9e9a3dbb0f3f17f69825b876ca22316a",
            ),
            (
                "dbd97f56e901dc61e35c07b7ba98a2d3815f11f09303da43b9834e0fccc9822cd2f9a31ffa4fdd2245d2f419",
                "87d4311437b500dee8f9b38f9dcd9d56679113957e1b6291fbeb5463884ac9652d6713752d3ba4d585949121b34c63e6b582f174cc90ac040037c5b330d8c8e3",
            ),
            (
                "c8e43b95d0f93ced0a31ea1b128e09a17a1353fd3d37a1",
                "38bc73e1938990e35ee6e328ecef439e16dd4d96855d0fc6360796f0275677ec587cab87fcb707a4511fa9f2bb5835455d1994df629914d8e0e129282494be04",
            ),
            (
                "c3873c48c58ccd9ca2f94f54b8c749d1779bdd47458ca960ccddcb1905b26f4e3ac471da280ad4919de5f405",
                "0ccdcc959121829285bb523767d32b53a349fab0526e9a574abae0f1731b1ba10dc2d9e606b55fa32f01b1dfcb44f2a7c20f742388217650f3181255195e732e",
            ),
            (
                "9caa4229e8b2e0820a4d14be52653b810c5b510da1265ae1c9724f0001ff",
                "58ff45acc9ba5c04d61bb7667d7b651fa76319c9b529ab359a3b940b9bd4d454f953c6f94b27c24b0392ff80f8ffd5216b15e288f072c5ca85ac3cfeb97eb629",
            ),
            (
                "ed752ef516d34fc06d363598e116dd5a5ae553010d9e07c4b133",
                "1637cec93a66bced03718532783efff60209e52ef327c234bc441f40ca2288e07e01560f7692613f2fd9bb99d7f811f89bce26443d0b5adf1b162a883d5186a7",
            ),
            (
                "2fa42b611e04afcef559ff98a2c8",
                "cf3947ccddf9281e14333c147e3a33878fa6c3b714b39c9aeb14ff04e382653d35dd25cbda14a2287f17b93607219667b22f4d5c5cd7947c182e921b3d5474b6",
            ),
            (
                "255e9dc0aa6fa5222b0ddc1be1a9210d90ef",
                "2e26650b1dfd4cd0b3f15e7446e4bdb3150060e6efc93f9e15d42b0e7c342b574582d8b77d9ba92c57804f7bfabb5527c3a07a07e34bd604a52e6a7cc2d26466",
            ),
            (
                "89e9ad3ef382dd218c777e654760620f5dfb52e986778b",
                "b1216fb40e182aefcb6da162fef9e636971b52c374a3514423d8d780de074bab41a0ff1d1831780c3a2c25c10a8dfd3a8d017b887e48b395bd84d6b87c18e69f",
            ),
            (
                "ee4886dbf617d2dee7f67963d497e815248e52f7dd5a15d9e1f90f9d70269d76edafaa872b6f44aeacde81a1",
                "bf416a5b68b8b67ffcbc8300cb48797be6c9235b3a128edef4342c6c1996e5212c14e03ac613f349ef668fde76de6a282e2edeef08a743a45a97649f6c968655",
            ),
            (
                "2056ce49a5deed5c0e97b52a343a42dd955a4a6c9c9f69ed57ed8cfe505bc2161dcb146b2bce8cb7494bf8754f18e9fd",
                "d5b990c9543a36f6d5c72cc4bae0d0bc6c07a0b4da55f65e08245c9f21bfd0f57dc2243fe1b131a675c192765966623259c7c5f2fc2bd7467d8c83305802c39a",
            ),
            (
                "d114a9db2bb8d2d3",
                "41d3dcd02466ab1c709aaddde86f80022303b5f3db5bab31a85665abc93362f7a2c5354ccaef9670619901314734db3d89219c781b7f6ef0863dba7c6ebc948b",
            ),
            (
                "a950a2c5385d3be1e2d8c52a0707d2a0e4547fb00a2caccc8cd02f20159a69be8f8123",
                "57e78942698d6323646d3cd7d580ebe93b820a1eefa2a8bf24b1a6415f99ea3dfb6d2bd230f4a3b71ce0dec151d14c68028f641f8b5501252cb845b953ba1516",
            ),
            (
                "02408050598306301e90ae93eb939e73577aaa8ecd1a1f72ffeecb3098a8116991aa18139f052d663bdc11a67371e98a3d7d779a4b175a61bae4",
                "564a5020f550f0b680db4360e3f139a769615c13b032a14269a29cf535abae980341f477ffc8bd95627d14285c55fa2fdc3aa77dccbf5afc7d8ea04f68942bea",
            ),
            (
                "96b7deaf39317e46acac57462d16daa8a879bd3ca83220bc82e5",
                "e6cbd98f47bf1ccb66c86b1570e44f147636d01d66fac762eda91b2c390e853f97d88b33eaadd66dcd53775d1a783f90aa3d52613be3ac0c7e81465815f637d0",
            ),
            (
                "117268cc13f4e2bfb2",
                "e168c5e046b4f8ecce34050e60f01f6bad9300f738715113d51d03b8850ce8ee0d556604f169e8da14874c2935afd26ecedac89a5b3c21e64d90d80936e215d2",
            ),
            (
                "d052a6085d360052",
                "706c6270ca2ed96898618760cb0dd99ef62eb0e31a3d29970cea393d26f3604e777c88452c0980e721d90896d63a9f2358cf10b4685464aa73b54f30ab1dc83e",
            ),
            (
                "197f37d3cfaa7c5530e7e7c35b725f52c6df4d51cffba9015fc63833d7ccfc8dc5f5dfece7",
                "a49662cfec9169c64cb352b13614f4b1e2f7a9606bb971fdece05634532c6eb41ada256af5e235081ae7b953ef413de366af819d5720b14fc20e19eb13c2d15a",
            ),
            (
                "b30b23b6cd6eaca5c72339214c2269c50902ad80db8a67594f882deb",
                "8ccf827ca0b5cd6bb196067372eb838c5d721d78c7803fbc6a7b334814e79d98953d6aa6bcfb29d33a6c4f10c46c4b8f706b4a7fd657b81a99524e5b80d4b64b",
            ),
            (
                "3c8f2f9729e4604965085d8467acc2c53a06f54fc05dda07faf7302e22baf992a81ea1395f6ec843d5d671a4fe398a919c3806bd29fc31b7d31f389d91fc",
                "4c6f498ab3de890d21fbee4587d9ecab54ee2180394e2dc018062812bf151be7fc646e4d0ac596d8838a21cad7c29ffdd6323a05d428c2043b72c9bd40c4112b",
            ),
            (
                "64f327f4716119db501f36a8784f4e8f094b7e1730cce054e4613d804148546501b5686f3c",
                "cff7482587ceea3414c303eef8579c00b0872e959343f628c4e3e16d37c9e84f7740b9225fa791578353ec87d5ec52e21b434ef567af65ac5f3013f93a047ac2",
            ),
            (
                "4911e683453064061b61354202fd1d9337c2d762043713ad28c6",
                "bca4252e1cd6d2c13793ff163742256644063e1c8e8ed551d9e67d48802752f398aed8130ed99b2c678a5479db4186b9fe9f98748032c989102c4496a430b696",
            ),
            (
                "1293938761181776e2eb9330c8ca6ec462d6ee886a2ac2",
                "301cde6e52062ff20d87f1c83566039972d901e357dc5fec8ef23943e40f5de96d140a2a4a3118a7e7228857d24614e499cd9214e9a29c324ac60e61f152f939",
            ),
            (
                "0c56f33d4e030c6516a08b41818ff63b6c7f97c3ee14a3d6e4bd5b2f2cd645d1a2c3123dd1",
                "dee9ef444162adc2666c5107e02e77fe52825ba4a4d712ba8a9b07737ab0eaa298bcd29dafe004f1f3222142bcdb26fca30a9792e0bbf35963281e60612033bb",
            ),
            (
                "104bd4e2831b94f47c8ecebf1c4f9ab93745b5aebbed13660208a3b1eff110ae0cbdb689e5b542039d8746fe41ef6cfb2c24",
                "8982e88eea9dbe672d036b7259ff9c7a0668bb1a9136e29b929059e3606daad43686b24f776cb0985e0d21ef8cc7c23a7930c987d754d7dd2c4239cbecf2b662",
            ),
            (
                "c4ffcf218c8721d6902782f5be6664",
                "c4659636ef3c4810c5871d0dc2cc66832ea65c555a9d794bcd316c0300acc262ebb2408a9365ff82f819c034c709337e41c791ba799cf7c7141f6d56328e8242",
            ),
            (
                "f59862a57093979009b025315306c330aba1d913afd6b402e0d48fff5a94a45c",
                "9b551303f7610c4394cc432047b2c57b559a89dad382b22ecaa8d67afc16df2e415598eabe6699ca419b77301e22368e6e15966a3cd6d1323c1b570fd6f64b66",
            ),
            (
                "69406bf96218981a27c24dbf7764a25b0fc3b6e808dcfdfff9f76493c3ac33683c1dd6fb8a15fba4fa4501a5dc31933f38a169f6dcd3",
                "9bbbc1c4b19574a63bc6186d59aa9e87f63b86dda92eef3fde4a5c3651eaa3d49414b878ef5e64c64b3635083a12ecf40b59f3c21fdc0aaedc16502cc7ff2255",
            ),
            (
                "7d55b84ce74de6968ceb50241bacb3120f701bcb82033d39ba71881b99ea076410552508cf4629",
                "6e9a0176e42ee653d2136bbdeaa7064091a82e6a7fc4d4494185c708045dd325879d088334139051b837fce34053820135369e1e6e208c70401467d8f5b3b2cb",
            ),
            (
                "06a5d66297e8333058d9b45f42b717c3cddf3d19d0735cec368b1b931dbd59de4052e1be8c79",
                "0763e9c72102ae83c90ec97fa89eed09b8029cb2074d697a7fdb1751714a2b6bde0561e91dabbc3975f64a4f02c9239b3ea4d71dd87abd5ed6b8aea722ffa307",
            ),
            (
                "780d28d970dec71684e5e874d177a639a2b2b69ea6798a784db6967216eec810bd70d7d92dcd94cea7a3c2759929a726",
                "9fda62682173d302bf3a704c6a8b5390f4471b04fba1e6ef4f856347c3c5a381b64ede854e20e2c69ce26c0111443f15d158f84701be2c078e2e7680e7f2199f",
            ),
            (
                "35f19cacbdec0fc546facff8972754268655edc6e0b377a7e8aba86700b02527503a2cb652749519c0bea9",
                "c79695e61faf85133ed140459d23dee389d8687a4c98d60947c4817a3619f1fc8515915a6852fe966baedae194d4b1faf6ae1581624c16342e492b05e4133810",
            ),
            (
                "f0ccba723f00d1c506044c4e9999fe648473fb38d651e09c48b926c213c057b6eff888f7dc9b19d13748971009b3812b2bd232bec371f03729480068f9",
                "484d4c7ab0c086a73c8184802fe73f85d69240c9e402494e09a2f487013b8c87a3039092b58e19500035db3abf74ca081ab09e05103e3ee9a1cf82dfae007ae0",
            ),
            (
                "4b6e01fbbeb74169beb60189739575506f6ecb9423104271551f01aa89312977f57b17e18c05cc6912c1ea85590456d3d8b1a9",
                "4d54e91491b43567beca5e9c5ebb28f9907fbdfddf8711aaa7f0c26a57f1b7c2c5e795d760f4780c72875619f3b32594a4f0885a9d7992931725a4d4765c3712",
            ),
            (
                "932f25b9cdd3d86c520f7c97aacdf20a45e5df09cd705fe9cb18efd2ee17f84d4f26c2953daaaba9a957f24ccc580cc95aa7b48dfdf44804f707568c",
                "eb77ffb734d839742a6b4f0e9a49b14ce621a7a83365aeb47346f1d6cf6e9691f308d1aca2f3eb473e7205c9f3d875d4fe257a29ec8e3965fcc0464038324d0e",
            ),
            (
                "9d04400ac6d73fba0fbd30206e6f42788cc30dd75a25c67b99bef4cf8d5f",
                "78c20a6c19de27f129f01985cd31e1104ff1fcc622c1dbfb2d996eccdda60ec35660aa94c2b7979ae0ad4f461467f6c2e5f7b0acbccc9d9c7733bba83cf0cdc8",
            ),
            (
                "632abf194d7a",
                "f0962043957a3ef4e4f925e0ae07d65bc601cd8c7af92e2d5d0dc33e8208ff6f16e1dc97afbd6a0b82607e2d7cde3d740d51f55bda06ea7256da273497541201",
            ),
            (
                "71b650bd85ee8a9a9f81babe8928bef7d0daa21dd7",
                "53dbcd5a840f72bda7e2edffdbdc6b590868449aedd27d0c1c3457a6013a4968413aed416ce8535449c8635f712efd705d7ee3045bd4f1490195cf58c8e4f196",
            ),
            (
                "8366ca",
                "292343ca9467580532367dae2bb5bb26826d8d16ddf27f9a5700c6abaa7faf10c82911a30dcfd24578d7579f77f6fcadf50e6a2edd3ba735015a646ba24eb474",
            ),
            (
                "0a5b67f8dbd9c80cd04449670a8269906207ef5251244e9b2f8c42ddd13768d5c14c187622e60e7b5bdf96067b420812ea76ba06f3543b4907aa2d6425599f",
                "99915641fead1d7a8043987c2e4163eb31e26103efa9c3d93a09ea51163a59e8e266158b8b1004ccd4172517208e010ae17da9cd7bdb33da410a3d59acd598e0",
            ),
            (
                "d6dfdc8fd6bd09c93b5bd41e0488",
                "dc1966b92708e4cfa14cb11b4e275f988edc95128d70723258babf582cf4643572821cd41ae2439b494fcc60e9c91ced5b19c5575813fa8360194dda92c77ad7",
            ),
            (
                "b776960cc155f7ddb8d876ab802a5b9755b6af08a7cf5091dbb7e28186e8ebbf98dd6426025edf5ccf03633e1d85b56faa26a23ba2e6bcb329089ea62a59",
                "801747668ada955483decdb7badeb906d33415b230e3f6ad729f97efff99f26b690646397ef63b7459137852b4755a1624075488fced5ad02a41d05367a2328c",
            ),
            (
                "c4e7f946cb03ad99e99c",
                "d49e51901de071b5544ab4c9f606a1b0e8561a5ff80a8f513653bc7ca6c5e00f20abe2c939df972da9c7d800e18ee5de4c24c46359204a53fe355e1db8093b47",
            ),
            (
                "a0a89a0158583f738efde7ec7c2495a0e4ec003f63bade0fe86bff866adae473cedad51adc8f8e070ab5f0fb250fe1ca2aed",
                "3682b83f72d46ec05fb42f48a8c9da20a1f8765cd68541fab01fb4c94415d5fb0012d1381bc49690be50b4d45daa3f5e90c27a9f99724a059ea165029be1eaa1",
            ),
            (
                "1f6fa6139ac66c58a49137be5aa5b11a8062c194e30acb683b2d0a00",
                "f39a49b47cdec8b2c0507fca5fa2d737a9dc9f451b3c86665dcc11a953329a13c3a6a0f6473f621471af15dae9e9fbe2451bb9d9c2df3040bd04f3c5e67b3af7",
            ),
            (
                "9fd3d4af7e9e5b4cda4bd8408d086b16c582492a343e17e32a73d749198cd03adae98b9b3d6a3e8fac0ad0",
                "12dcd07548668a5afbf9edc98b9e8cd6a689d71d3a3dd984846a18b36e4ba85d51c18c11fffecc1e0d625bc96bdd862b6c45812ebae5f755e8796beeb92acf84",
            ),
            (
                "68ed9e59",
                "e275f94727f1d426ee2e2e8d72015fb05e9d18fa377d7f375477c25260ccac1cc43ebb2efc21d367bac8cb64fbf632dc07e427893c5f5519f387cda80b4fb69d",
            ),
            (
                "003146",
                "12ed2d4ac028cf840bc5ca530f3be558aa8f244ae6fac8ca673aeeaaa5b11923bf3e583566634ed3433c805ff2010ee32d3c3813d62749f24d0c6cf54d832333",
            ),
            (
                "895b4e739c397d6c4eeacdd5f2",
                "8b7f18125853f116b9356bfc26020a32c092113372793ed1dd037c506c0cc0b59fb94f7b588406161589227e25702e720d8809c8d1d1bcd58fb6448a3ba71a69",
            ),
            (
                "6e6aa6a96fb0c052e600d31a083cc5fe405ad8453fcc938e44706d42e09e13fdc473e382fc8a2d390d4425449a8aff03966c27080584b38d03",
                "7081bc6b9e84ddeea6aca984509f9b0f269196c0d27735b8aab5f21111615939dc269069a9f9d0e52d842c8104941deb13cb53733ecacc088f5bf97e3b055e5c",
            ),
            (
                "301b1c2814df527eb8aa17dbfae2f238b36da07a9d7e1df9709d6f420fa1f842df824abbf3ceb3b230691ef7403cb742e20750826b6b3ae5ed",
                "50f58c9b0447aa235d20ca23015947a3dac835292de089b32c4ff1e8fea7d40ff27b021f1da6686b406ef311850771064b85fca00e6220f27d87bafe29d213f5",
            ),
            (
                "7f53f2a01751143fcb3558a15facc109650f390d169052d51c80bc0bf7e5025d93e1158ee01516e065d7a7",
                "c2384787b42766861e634ac3942a63a3c30246c0a6d71f4004a74558a19d0b4ea902ea40d95b55c958ce88bcd6a4adffddb39a942a2907650e18456eca988752",
            ),
            (
                "66c736",
                "82264658f881f2b42b729ed5789452328f16813458fd328507103e7ee313dbbed41fefa7ab5289f8f582d0d5a1efddfa2a0934d871625ea5259f6187508e8cf8",
            ),
            (
                "553102b3b591a8cc17c32b932ae1f99535b979b6cd490da176f28ff549bf6b15d6417e4a722224279b21ed203cddca7d64",
                "7dc8cbb510a2e9bbe6a65310cb4490d039245e6db43cfea8267157c4c99a14eb5fdbb7243f79ed75e796a1556498a71aa4876c271ebb9b045b0a1c7042001bb9",
            ),
            (
                "fe8b3d51f6930b67549f77fbba28e6763736b308ecf462",
                "60b153fb8821e2d15e911c719c1714013e853f1cf4c734c99531bf6d31b895f3bec8152b5be3fa7b25d535567bea1b4c5f3469127961ec56be8bb873dcfd5c2c",
            ),
            (
                "d78935d2a4f0f86fee97b90dbd0ee2a4fcb7ab2e11693ac1dc39f41303d1fa378f683b4b20c1630d",
                "d8608f07b67d9d723d0e0d499f2188e6658ab80760c390a203ed1b6f13293dac2092a6b1cd8cdcf15e39aec6658f07206cf888faa280ed864551b262a88e5165",
            ),
            (
                "dedbafd4f7c0b38898dd730cb9c58dcc9eed39326b0e2aee10b816dcd1f11a9e7d25b8939014c3d7e0283336",
                "6b3a348d1c2f157c6bd10ba3c1c00d7672fd7d2f5b0f69449c616e4198dc51a301529bd8538f9e89b59a9ee21097b9246fe1533f206c50a6574656a63d80d28b",
            ),
            (
                "ec40a9045d6d08c0c72cf95ecff0acadcee7861e7642eb0f8df415b2a2b237a4d0452c5aaafa2a9230c6767e3beb2887de438bada1",
                "9e19e9d5d683687797fb4fb45fc54255ea4fbefee6fa2ba25c2b56a737cc2f96fd98ceb245783820f778af470b4869b6f1a238ebccd82eeec8aeee685d8657d9",
            ),
            (
                "e9556f12b1360c9d6aff83a345",
                "cda3dbaa89c60567685288900d476b6191840923b7effb515fd32f38767e9b1bf95f68f4897ac676af45321144c18b8afbd4209417f682b6c5a1961959075f74",
            ),
            (
                "c3be1d72a256bc3c24f38d8c81a36864f7e7f30f5090087c44c1079c05c9bfbb8575",
                "b38df1ec5e4bd931ef0d5f134ad6f58f6d7684e998a2964b9e0abe0fbc816f984d4e7aa3b46ee9dc0511d8f30561100cb0ed84faf3ba7f241f48e094dd22f278",
            ),
        ];

        for &(input_hex, expected) in test_cases {
            let input = hex::decode(input_hex).unwrap();
            let hash = Sha512::hash(&input);
            assert_eq!(
                hash.as_ref(),
                hex::decode_array::<64>(expected.as_bytes()).unwrap(),
                "mismatch for input \"{}\"",
                input_hex
            );
        }
    }
}
