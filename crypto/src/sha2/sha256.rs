use crate::{Bytes, Hash, Hasher};

pub(crate) const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98,
    0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8,
    0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819,
    0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
    0xc67178f2,
];

/// SHA-256 hash function (RFC 6234 / FIPS 180-4).
///
/// Implements the [`Hasher`] trait.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::{Hasher, sha2::Sha256};
///
/// let hash = Sha256::hash(b"hello world");
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{Hasher, sha2::Sha256};
///
/// let mut hasher = Sha256::new();
/// hasher.update(b"hello ");
/// hasher.update(b"world");
/// let hash = hasher.sum();
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

impl Hasher for Sha256 {
    const BLOCK_SIZE: usize = 64;
    const OUTPUT_SIZE: usize = 32;

    #[inline]
    fn new() -> Self {
        return Sha256 {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            buffer: [0u8; 64],
            buffer_len: 0,
            total_len: 0,
        };
    }

    #[inline]
    fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);

        if self.buffer_len > 0 {
            let to_fill = (64 - self.buffer_len).min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + to_fill].copy_from_slice(&data[..to_fill]);
            self.buffer_len += to_fill;
            data = &data[to_fill..];

            if self.buffer_len == 64 {
                process_block(&mut self.state, &self.buffer);
                self.buffer_len = 0;
            }
        }

        let mut chunks = data.chunks_exact(64);
        for chunk in &mut chunks {
            process_block(&mut self.state, chunk.try_into().unwrap());
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

        let mut tail = [0u8; 128];
        tail[..self.buffer_len].copy_from_slice(&self.buffer[..self.buffer_len]);
        tail[self.buffer_len] = 0x80;

        let padding_len = if self.buffer_len < 56 {
            56 - self.buffer_len
        } else {
            120 - self.buffer_len
        };

        let length_offset = self.buffer_len + padding_len;
        tail[length_offset..length_offset + 8].copy_from_slice(&bit_len.to_be_bytes());

        let total_tail_len = length_offset + 8;
        for chunk in tail[..total_tail_len].chunks_exact(64) {
            process_block(&mut self.state, chunk.try_into().unwrap());
        }

        let mut hash = Bytes::<64>::new();
        for word in self.state.iter() {
            hash.append(&word.to_be_bytes());
        }

        return Hash(hash);
    }
}

#[inline(always)]
#[allow(unreachable_code)]
fn process_block(state: &mut [u32; 8], block: &[u8; 64]) {
    #[cfg(target_arch = "x86_64")]
    {
        #[cfg(feature = "std")]
        if std::arch::is_x86_feature_detected!("sha") {
            unsafe { super::sha256_amd64::compress(state, core::slice::from_ref(block)) };
            return;
        }

        #[cfg(all(not(feature = "std"), target_feature = "sha"))]
        {
            unsafe { super::sha256_amd64::compress(state, core::slice::from_ref(block)) };
            return;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(feature = "std")]
        if std::arch::is_aarch64_feature_detected!("sha2") {
            unsafe { super::sha256_arm64::compress(state, core::slice::from_ref(block)) };
            return;
        }

        #[cfg(all(not(feature = "std"), target_feature = "sha2"))]
        {
            unsafe { super::sha256_arm64::compress(state, core::slice::from_ref(block)) };
            return;
        }
    }

    process_block_scalar(state, block);
}

#[inline]
pub(crate) fn process_block_scalar(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    let mut i = 0usize;
    while i < 16 {
        let offset = i * 4;
        w[i] = u32::from_be_bytes([block[offset], block[offset + 1], block[offset + 2], block[offset + 3]]);
        i += 1;
    }

    while i < 64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
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
    while i < 64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(SHA256_K[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
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
    use super::Sha256;
    use crate::Hasher;

    fn vectors_sha256() -> Vec<(Vec<u8>, [u8; 32])> {
        vec![
            // RFC 6234 / common SHA-256 vectors
            (
                b"".to_vec(),
                hex::decode_array::<32>(b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").unwrap(),
            ),
            (
                b"a".to_vec(),
                hex::decode_array::<32>(b"ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb").unwrap(),
            ),
            (
                b"abc".to_vec(),
                hex::decode_array::<32>(b"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap(),
            ),
            (
                b"message digest".to_vec(),
                hex::decode_array::<32>(b"f7846f55cf23e14eebeab5b4e1550cad5b509e3348fbc4efa3a1413d393cb650").unwrap(),
            ),
            (
                b"abcdefghijklmnopqrstuvwxyz".to_vec(),
                hex::decode_array::<32>(b"71c480df93d6ae2f1efad1447c66c9525e316218cf51fc8d9ed832f2daf18b73").unwrap(),
            ),
            (
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_vec(),
                hex::decode_array::<32>(b"db4bfcbd4da0cd85a60c3c37d3fbd8805c77f15fc6b1fdfe614ee0a7c8fdb4c0").unwrap(),
            ),
            (
                b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                    .to_vec(),
                hex::decode_array::<32>(b"f371bc4a311f2b009eef952dd83ca80e2b60026c8e935592d0f9c308453c813e").unwrap(),
            ),
            // NIST FIPS 180-4
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_vec(),
                hex::decode_array::<32>(b"248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1").unwrap(),
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
                    .to_vec(),
                hex::decode_array::<32>(b"cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1").unwrap(),
            ),
            (vec![b'a'; 1_000_000], hex::decode_array::<32>(b"cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0").unwrap()),
        ]
    }

    #[test]
    fn known_vectors_single_update() {
        for (input, expected) in vectors_sha256() {
            let mut hasher = Sha256::new();
            hasher.update(&input);
            let digest = hasher.sum();
            assert_eq!(digest.as_ref(), expected);
        }
    }

    #[test]
    fn known_vectors_incremental() {
        for (bytes, expected) in vectors_sha256() {
            let mut hasher = Sha256::new();
            for chunk in bytes.chunks(3) {
                hasher.update(chunk);
            }
            let digest = hasher.sum();
            assert_eq!(digest.as_ref(), expected);
        }
    }

    #[test]
    fn block_boundary_lengths() {
        for len in [55usize, 56, 57, 63, 64, 65, 127, 128, 129] {
            let input = vec![b'a'; len];

            let mut whole = Sha256::new();
            whole.update(&input);
            let whole_sum = whole.sum();

            let mut split = Sha256::new();
            for chunk in input.chunks(7) {
                split.update(chunk);
            }
            let split_sum = split.sum();

            assert_eq!(whole_sum.as_ref(), split_sum.as_ref());
        }
    }

    #[test]
    fn sha256_generated_vectors() {
        let test_cases: &[(&str, &str)] = &[
            (
                "dbb2f8949e06d7e52a47c0bfd9",
                "d970ede60c8e58f10f71ddfb9f13227289fa333ab0ae17e19d11dadb6de98fcc",
            ),
            (
                "eb409c57dc6e689dd976e50e1fd7844a4aee947a",
                "ac87a9650f85ce14c09078f50a859e08cdd9c41e16936a730f80bb9c49f0de15",
            ),
            (
                "a99a65b0b5099589c015514a1055039da1cac708f5738d2a31d3f1",
                "ff92dc556cf1a29ff87e6ad46d36172a2458f1a4555bc9212002e347f766185c",
            ),
            (
                "b38e7b70d3389e85efc420776bd99a57a7e85c9e4eb0103bf0482e3fb5693a3ed9a8",
                "8e217e288cbf80827cc898b4cd3f7475aa3248b5c65f8039478b859c2d69a6ef",
            ),
            (
                "c103e71990663615827ee4ba1ca5d0c73af7da8cd6c355f5467473d7fc776834b1137ebd383c374865",
                "035ef24499be5b7332bb896dcf1dce2aa46a8d425225c2e0aea7e6a9546e5345",
            ),
            (
                "291d76fcf8e4189cf96605131d4ad0d57ddb2e9c2c949f3524e1a978ba9f52cdf97af8bf5d80501123a8918984fa3595",
                "8a00a6cf97930b04bc8918a502ba54cb82b318e857bc7c1b173690f98aa0d52c",
            ),
            (
                "1a0e7e77bff8cee22a7299ac539ee995b2581bc0ec5f42ef438193a451be500c66bfabdfd7f95d8eb6e0d848f5d9eee32e540c86df9b88",
                "75e8eb85e47df6f753cbe6e9bd723aed93bf5288ba1efbae3468bafbe632576d",
            ),
            (
                "323cf4c5b0625e60df4584afebe30281b416cc9ca2e7ef36634f944592bb5b301d5cc75462ed8b47823bb03bb050bbe65361a22fc7a7e38f341001b3fda4",
                "a7049c2e2a1660a610127d70db0fed64a996cbe36af99d5b284d6f194c8e7b66",
            ),
            ("32fa31e0", "457c5b9a13c0d7210fe893697db139706d5569d5083e3fea6d043cd31cc21d4e"),
            (
                "88c9a601a1f00b2f177b25",
                "aba9ca72bddadc903826a2a640b0c157b2d7025e0e8b678f84c9526e1be65be0",
            ),
            (
                "1e9ca9b4a9af371a9f6f87afd350c5f49a51",
                "bde48e7df1d80281541e64237bf7ade697b1711e0a070efb5614551c11319595",
            ),
            (
                "571d0711f702310f73eb2af372bb5a1328b353d50e1adafb2d",
                "7df6171b59d7b300c482d803255ce736e857663ff0d6e89dd447cf906428de46",
            ),
            (
                "673325f182b86d13f3daf22e7991d90a890aed1d5bca2bf822abf43ca0e6864e",
                "4d3a318e9f268d62764fb946d7519a08b6d466f2ed2dbcf0e93ab9429531afa9",
            ),
            (
                "e7e9b3b0fffb03f1ce4addff7ad7c6f090aa19e04d6552faf3619b47e85a8b0fcd6d1bec29c73b",
                "5b9948841394b3ea75d2754a213976f44b9c7b026d95389236b8dcf84ab44d50",
            ),
            (
                "4641fdf46c06739b7fef2a400a3bc281c661f6c9b344a184857654741692fec0d35237c357599b2d85939933eb4b",
                "836b90af0b82843a7c84617b00a4281a25b3dde914a0aa04edddcead6d959ff6",
            ),
            (
                "a2b8f795f70eddf5de8dc18d80c04a4775437046831ea4f2361951e4358e999d56313bb5049a2e81f0f6ce10b7039388ed0c99022d",
                "e4d189da6415c2959cc2208fccc5087a4c178314ead690bfa10028b48e495c87",
            ),
            (
                "849d2379438aba5c1fb9648835884f22e0e7b528c20da34d3496ac299cb93103187e19279f44f764afc89230bcdebe9ae604e2b255bb1691e26f2dd5",
                "24867539b04a314bdcd277fcd12683b86438db87f0d80ce76f9e8c8bf3ee1d18",
            ),
            ("706c", "3485639faf1591f3c16f295198e9389db5b33c949587ec48663597d4e00299d5"),
            (
                "711f30005ffe3fb060",
                "dd14fbd204ed8cc0fc19f9060da88e7b07b72c31fd9cd45f37e09ea78afc6453",
            ),
            (
                "554b57fa7398cc9090550cad70744f26",
                "397eced20be563d1500879df42b209f4399a49457d33bb2da37e76bd092aef4a",
            ),
            (
                "7589be3754ec491acdd58325350a0b29d572075acec8ff",
                "8425e59cccb3476ceed8c288d44675edc0ef404a768bd99e7da7ccef25fc9540",
            ),
            (
                "f23fbdf8a9e1a849e89e0254c14a847a2605ce785f09db04d31aac7a8008",
                "203d39fb4221cfc1ca2d1924e0edbbd9b863aa583a5e0d781001d03ecf9a61be",
            ),
            (
                "7f5208aa41ab6c5971e460aa327280e1e91a980df90dfb3fae18d0eb28ffb905ccc02531e5",
                "88b998d889acbd533abc7a3590b676786bb679fd32f57156f44ee0863f708bb4",
            ),
            (
                "063a04b3e815aa11bb6c4b5b4df9f788b198e0801b4402e909ed4f338c4dc1bade29711f0d4cf25b84268df3",
                "a9d0ce278f6e0165fca76749fbff6f870f549b91ed28b5765313d136785f29c4",
            ),
            (
                "5bdf921b475edd977c60e8388315f2d084e4f649e030b322ea90ae8ce949de9bce6c6e6cde6dbaad4e0b9aa996588d326503d8",
                "b3eb0c3cb67313f5f89b4cbb7a7b63cd274ae52646de2e9e36eb3afe58f8ec2d",
            ),
            (
                "3d478d5ca25a1ac5bab4878ae5dc56b2fe62a70adc70580dcb8294e1f1dec95815f53901fe3a5f642aedafd760cfb538e26112fd21bb1a0c243f",
                "c6c827a166cdc878d612d8d7bc72fe782699a1ae021ec773751f1749db36a3d2",
            ),
            ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            (
                "580998222001a7",
                "847d0cda6be10daf0de8d42e765337f9c0c1e1e38e16666eee95f04bc825adaa",
            ),
            (
                "ae5912da9f3383cc805de0200bc5",
                "4513e9e89dab72e6ff1a8aed455b83c1fb7d4a242f660e89090ad58a67fc44f0",
            ),
            (
                "879934026b61c35bc09f3c0381639f46e950de6dbc",
                "ce44271e09c0bb5f86477ac33ce8e42fdc0c9393647fb56e8278cf464a3b8ddd",
            ),
            (
                "d9e0eb60305ca7b4d27d39057990c80d0d5c0529e38c190cf04b8772",
                "c93c1f6ff33a3af1ffeddcc3a8a72ee51c01f27fc05b952fc3f87348886da8ad",
            ),
            (
                "d5462923ca756d93901ccce8036a5aa06908bc8bbe0ac0a42159a7e4ae9208ec8834e5",
                "de5cb2c16cabf660fdf74189badd9ad34449f10f3632f6814191fa59ccd040c9",
            ),
            (
                "fd1affdf2eedb633afc9fe518c695a620b96c9da85e155dff50ba3d97d6a5e7eb2382777f9adaa8a6fb0",
                "0e8b47551049d322e12cbe2ff43d2ec08eda712fd11d550ab177d79f06e28163",
            ),
            (
                "fef6cebfac94f9f46aa1074c90c1a3b7c0fd6c84b9ea326663838476572ba77af5602d8d8466ea47145f369360fc605937",
                "e5dd1be8210962fd96df639a3ed871c655f8872459d71a7ee3500d4f3e027265",
            ),
            (
                "85940f2c4c91cfdce0b67102e83bbefcfd9bff4f729a4e63c79c8879d9482cfb3d25c119f50729e33efc872a9fcf27fbb5b699ea58501ff0",
                "56647edc2b78561ace7b915c1c6a887080122e66c758576026dd644bd9d224c1",
            ),
            (
                "8ca87973006154380a6b46e8ddc6e1dc705a91c7387a39d02f56247c5773bab3b17a04c0cd8a022606aafecc2a77a166cbb3285f41153b9f7cfb9bd37ba00f",
                "681c586ef676da893ade0f0ec1a47cda4e17a5a703a359a52bf1617bb7e49b1a",
            ),
            ("9da0ddcbe9", "9dfbef13f8e563e24105586ced11bcbf348828d1060063076eb60a61f5b8821e"),
            (
                "a053f3efd8a555cd31bdd1c9",
                "d78ac3300609c6058529c95b48ace89921402ac4c26c9ad8154953db42c3ad4a",
            ),
            (
                "7cec17557b48936681287eae21218e87e54ebc",
                "ad709028414ad926baf415730d8929db0bfa5eb68793431c3e57f21a7f009824",
            ),
            (
                "cc814421759c8d17ad0b10e2bc0b308b7166261c1a826b4cb7b0",
                "f4869d34acf8b9082bea07247c10f070f3446b56833b3eaff9a920b8e9927c58",
            ),
            (
                "06a31ebfb0cea09f006cd3e135738811207ac160c7e297336bd7cf6fb946ac0fae",
                "24fc6b72ec49a3fc905857f0e7be0ff42959b8b2e5c03f4d8ffef777ead783d2",
            ),
            (
                "64dc70622b2f68ec4a7d72dc1cb0cf79d6e0c882d8ca03e9dd458366e77d544c4c50fd8529899f7f",
                "37bc45cae40d588350c46d2d207952790e6470b9c71b266ba4ad41f7e39018ce",
            ),
            (
                "d71a18cbbe2fdd5802b43ae87b7177bda7ae028d5fecd008b56c377b96dbb549ed8b23753fd8348e9fcd0272899030",
                "f26133db9ff33db54187f53ec5544e3ea76ae024f00c3503db8400f478416383",
            ),
            (
                "3d5aa40a3e73873dca440596bbfaa3a8b3e9e8724753912160a50193aece85cfc11e8b721ac804e99670836fb29e3d55f4d6dadcae68",
                "226beeb435de91d897d193b316d0f3da75269bd036f82f8186a63f5c2109ab5e",
            ),
            (
                "f020eee1c66eac18213ad3228684458010382d2657d1d97e7d365cf824fc061c6ee0bcf1dcd82021f8e4263b38ed34e26a94e0a84b3496e97f83344337",
                "db807626b7de2fd6bfb3818aec5d73d4edd3c906678ce22e761855a5dbf8736d",
            ),
            ("751e35", "ad3504020e937e5db906623dacfee45280fa567bf5289e3a098001ee318243fb"),
            (
                "be255e52c43f924dbc1e",
                "151873926085e89410b8b2b3d91860cfeea6649ddbba24f4fca838d308a296bc",
            ),
            (
                "a7218b8728033d7b84fb3f66c993644470",
                "32c02065031fc87eb5bd4d85273d0b8f449abbfe878e83cc30aaf1d630e19369",
            ),
            (
                "e7399eacdf0ec496f31db0015d9ed83bdf323492b461338f",
                "4628e79c490462f9a4fd1b06a17c4962099eedf9d3b33721d298dbfd8fc76056",
            ),
            (
                "698a62c23ebad896c63fe086f1254da31ee5e9620aeccaac2844867a5d12d2",
                "963c033d0c157fffe862d2e75568a8656b5ec78c6c0ced4f81161710d8d8d89d",
            ),
            (
                "750e8a21a40dbb56ac987157c0d5a05032b88ff297b8b88942a36fb864b30bc85b1ee7f5c98e",
                "2f37d2062b8bd0d502aa2a9612bd4f249a42eb8bb7a15f77ef04d28019fe1f5b",
            ),
            (
                "2f8b9a4713b2b2e49d24c24a4416d8f0f41674321fccee34a5be27a353c4294594faae22a7cacd0fae409b42ed",
                "504f85a87594f936d718bdd7411c6d996789a0acf7c5a7853042dd8dc0a48f2e",
            ),
            (
                "40506600ef19c2e690e7b88b924313aa0347209aac0622a8c81977f7b1d84e11d4eb4275bd4c5f2dd7a97a005031ad44646a5fe6",
                "6840d915fc2d5b95d5961c9d5bff98393e80044673a1df15ca4cce7c151c62f4",
            ),
            (
                "6f353f73229b906de7b2e5ad34bab368ae5b6c17a8c5fdca23d351a4486133217cb0b3845c756a88b7a6de95229dc6a96448cc7e47d51268dfb988",
                "bfb5e2c05a8ebc431853bf475d24726c406eeb4af4a8087d55973f486ee16d13",
            ),
            ("c2", "c557e71380112b980eaf1145fa80621130dfbdfa1e375d87ae0018b7c60ac16b"),
            (
                "fc738c262e5eecc4",
                "0ada1ed3e5fe2ad1858048a16294c8f0b55bfc7f15dafcb463dd672ebf04d40e",
            ),
            (
                "1783c7c602ff98bdf4fbfcd8ea6b6c",
                "863923efb0af98c325249db572d52a0673c5ffe9a9bf31e495b2d4fff93111b2",
            ),
            (
                "d090c2618714675b1c8e2237fe53c9eabc4297bda47b",
                "0aff4a9141824012c1f799763f92399030e741a31197b2e2363ba4f8818ca380",
            ),
            (
                "f666261a4c3803e604686b42ce91d889ca66f052cff5255d53bfc717a7",
                "0737fbf9fd63ccc2d3fa0a376d366aac16b7a07d5103121176a3690e6863eec5",
            ),
            (
                "f516dfca4054522943c117295d803d6046c6dde4b9f0c7acc30ec012f15a1e909eebaf61",
                "9c182d901054eae51342b9d17d41e82401bf4616c56fd5b3e3a5cd00122c6b98",
            ),
            (
                "cb4f807652fd163a06b8f6e8fbe57b67c3169ac692984f5a6ebe5571e660d12f31eb4423b15a2753f90190",
                "547ee5b2e4409893827e832bd1044df4cbe61bba2f5b5276a299ad5d382fc3e6",
            ),
            (
                "9b1b00b57f3a6920b44dce6e6ef6d30d6dee0f7ee9b1395e6e38fbd25d7705a773254a0e85ccba89f342b62a2b0776fed802",
                "95418971e7280dcce142b95e17beb8e0c788af90d4c5a2341724047f6996b6c2",
            ),
            (
                "0c659f7173d1cea6762fa32525d231fb8d1e17ed7336d50481545bfac13d4d1fa66bf206cb20ce3d492672f9c6fc4ac3920f3761c3619c89a7",
                "ee527f03467f2993243487bb463964013a1936133e5f43620c93c682cc7a0b5c",
            ),
            (
                "0a6b79ac99ee746ffe5c04d224e1a4e58cf2b2060c542dca5ac6363906a9189900dba9098dcb8b7f2b22b4228aa37ad0d47f76e66c38eeb235e420ada105de22",
                "0065a0d94a0d5947642f34fb5b257dcebaf6363a47e5fc65d08b95aa4cdc8084",
            ),
            (
                "727a503090ed",
                "0bb1c0507112fe197f0f3ce42a80dceab08d8fb993157cc525b511dc4da2d564",
            ),
            (
                "cfc85371f2000f8892667f7159",
                "f855e0ab39f9cfb1563aff454136e6ffa0aa90b0ee6129093547629e81b23f11",
            ),
            (
                "686c063aff17b43ebba006ab04849773da2c4a0a",
                "99df1c3f89ead9e0785f90f87e25b73d25f88dffaec34612a5ad7ea3cefd6a16",
            ),
            (
                "281313ec576188f7562aeaa1442cb37fa696ec71b6729df4b1d758",
                "e5f5eb5c5d945223e3a8b08181a3be61d71d5751b99898b434d1c7b4308553d2",
            ),
            (
                "2a74e9bcf47a4028a434c7f2ebaf6a5fb52b6d5c2fa3029baaa2bce1344ffaa30a55",
                "49882ca8651ca057448c7843911dd447559ed28e32db4bae7d4f26208df5d4cd",
            ),
            (
                "387c347658c3cfb67f01db80393f7e4c0f1e50faec549ee800001f555deaf14ae400f839215820cd6e",
                "8aa166327c0ec5cd298cd13d1bbd919b9437f200bbdc20fb60a20561b255a456",
            ),
            (
                "c990c31bdd85a6653eb64d185bcd0b6ea91141b5ab72c3559cf060f9e74e96ecc8591d8de903a3150c163527a0c7901c",
                "9d1ffed227d8ec8b044210082233ad7e1b912573263154ab90d06a8caad96967",
            ),
            (
                "7d0b4c697cef32516b86d96752d6b3d00f3c7cb969f92939f3121867fded47ff176bbd15e3a99468fd15660ae9644bec3bbd5216b5c172",
                "4206a229243815b3dcab4be3fd39d24f7deae3091c64ef9d313d3d1ee3eb3b86",
            ),
            (
                "c57baa055c272b21b48f3a98469ca08dfa435272569072c828145755ffba8a8cf6d8d4071f40b5a8904cc0a4936f7f5e2195a329d2b933be2c1a0855aec2",
                "3eb38115de1f4183359f673e697f7beb900504eada43f9043da7f753f00d913c",
            ),
            ("4203e2a0", "3ecd23bea7f5c52050b50e8aa0f163be47d90e840ce425be079962af9e9342dd"),
            (
                "6c33ae15f93da90a1745f2",
                "fa18a66229996604db1c4158a3d12f9d1468ced5296ff72f014a073671c94299",
            ),
            (
                "859c96d81af041bfa025efc4194eb105ee9a",
                "13223ea266d386b45490be76dc17a85febe99b1518099d7a6c0e9a719f6458ff",
            ),
            (
                "61f34a55b9bb38e01a29ed4b7c8ce1afbd65a9d855a0cab6c2",
                "4dc44ba37b842e52f15bc8f7bac96c63dd2e621d53ffb8caa8cefa50fd10922d",
            ),
            (
                "6a9ebf3c484afed269371969c7b613b3bae3e2c585f5cfc93a44ae5373c3ba83",
                "0a851c6538bf83cebc61e0d2d7aab109ac4eee11dec47c84bc1ebcae9dacbbf7",
            ),
            (
                "63686df9c41d7ebeb9210d3a80944acf5d6673c9f6cc593740216b84c044c2b69793de293e47da",
                "f69be2fefd5b7f95d4cb4e503d93ccc4419bd3c84ac2b927344816703b4484e4",
            ),
            (
                "b9a5fba3f9341de1768dd6547a325d71a880c624be17a382fc517a63d6bcd0ee7b7330187b27854acb3f78ff3a89",
                "bf2e66ff5526f2ae18bcf337943731705cffcb09eef20be12c9bd6f0380190d3",
            ),
            (
                "c4605fb3ae2f0e33bfebe0a4951688cb9893ee9f426a7ec4c611aafa450d37cf69c9febe3d8cbbe482c6721bf3dc4432a4a6bdb393",
                "4c1ea15b2589e83a6dda16f9c1763282c1d91553a7a2eca4d35c571c15702a86",
            ),
            (
                "f087d2bc3d0162117a7182d585e6ed63d2c961173cb6e6c3fdcbf12718919bae013af573fe0e384c9d72fd0d7f3ef119b2131e02badba24cdd1e8937",
                "e8ef5e5c59de4f77c9dadfceb5a0b3d1b169b2acb2270136a2e2334b4b975861",
            ),
            ("4354", "15cbe5f066da796145940479d571424695e3077a1de270cb04972942d5b16260"),
            (
                "74cf5301bc2bde1401",
                "49d43fd62391f63feaa6825132ee470929a01aae959f21362b889b95c007ccaa",
            ),
            (
                "f8f985774a865fe4111c8d88f8185367",
                "be9d1a07065d3520618977d94f4c676c98049e51e01a4edc9138d4d5eeb523ac",
            ),
            (
                "8e9f9f37ddef9c44ce856fc04d4959f096783f9ff7c04b",
                "393a745350c377d3344918ce1c162d1da134da88687d08271b1489d01b8c4072",
            ),
            (
                "bb1378adc4c0bab282c0b69a8487e7281e80c8c904eec0245673d2e833ae",
                "69ac9f99ccb55b225cc81e119ec91acf57d28466d559f16ca1216b1e419d7dea",
            ),
            (
                "dd97b0f72c62161e917b28d8c18e8de8f5a28e4fc7b37d1fac9db3360365c81b35e0776f0f",
                "6130cde3fabd59cfe79e9986d19e3827729b671427c544d4a049e656061cacad",
            ),
            (
                "e2ab97041fdc9c4730aa0d37665b665c3248f3fee752a24e5fe76e378b567eb0fa671cea428f4986f0219e94",
                "223b63957309d9f026c75af8da23d1e5559f7ae056ab3557560841a4e1539dca",
            ),
            (
                "0110829a9a3ccdbf29d3d20b7730674de4f2cde57abf7926a637d1f9c8428d5f3c942e82632133c0d55a9168a927705218036d",
                "e46d7427fe521fb12e214e633b02eb025d1530508a9cd6d6950e5d464109f7b8",
            ),
            (
                "39d58c3424396a88020a796c09272c0f054402678c29eae43f1267b0a4ca47bbfc116ac53e7e954d35a8beb63e29ad102c79ce70388df23cc2a7",
                "182c83a49991baf211977cdc44fc503495854332816bc5cda662f0eb5a64fdfd",
            ),
            ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            (
                "4d273643248639",
                "57d19cff0c4837d002cadd02e77a9c94249b83a866b4c055147c77ecc4372caf",
            ),
            (
                "8fe379b03218b8e775073b1dea88",
                "998f139df347c8031da8f8723bb96e4bc2e32e2e2f0c7bf151ace1e2b0361c26",
            ),
            (
                "2f550fa526ecb711207699b9b9710ebe361ab37bde",
                "c8248a698192abd951b5f30bd9189ba7b45ca9bc4e75e8f54d9187627a71f85f",
            ),
            (
                "b8578400bae02b7d0d87d54d25644e8bd6ed15c460ce9b28f97e6656",
                "f91b8a8a0737c7aac7f2b9c08ed105ae8d19b9b461f6b80fab9aa603b7231cae",
            ),
            (
                "3b83dad175f2a62fd3bc39c87cf8e075d801c7f2c7933a8540c338ef83c5272d408b07",
                "6b3518f3444006b2b920892e5961d76751c322978ce9b3036a8c60c08dab8854",
            ),
            (
                "49bb3f03a327cc2db0a1c50d58b28b842c7710b52bf747754110f910392315c72c1fd970e379fe6ecf91",
                "07c1188569a12e8af52c94f5fa60f9561c0e68aae8c98be5d8fb8004c9cf05e3",
            ),
            (
                "6a7c98181a19455dc38d1e0b8f20ecd843cb400b0992611208c468d0a11084e0405bf2692ffd0017a523532d95ef9b08cd",
                "e7c58fda3c5abf794e7766cf50be3261a613e26599caa63c7165b9f1d59eb6ff",
            ),
            (
                "c65d5a3f1b7702dcf940e1e57e898da814c7c03b422989e3d4bb0490f8fef30c66d67083f4f71431733350a0a00fffd2d2bc090daa281ebf",
                "5c3eb1aaf475c87ce87b34da6e9525aacc4424c3bbf99b4096ab6dcc98c377bf",
            ),
            (
                "4fa8cefd941963baae67a0d46375c75004764d29e97a9df8f8fdf6c767ecce24155b74e99ae4d000d77b185ed792e86270a81ea9d294d7b97d3d69485d49a8",
                "5d69eb343ba79f527960323f338336d6866aaf9e32b08d6f2d54e1dd65c1fcdf",
            ),
            ("0b83f0bd70", "e8977f6f8459985d2d74b51ffad819d160159c71583d992f892ba39b996dff3f"),
            (
                "a9eebcf51d3cec3b3d4447f6",
                "6315ad77e6addcc40c4ae196b1a882d0f401a3b4c7355972adfe8c4cedf062a8",
            ),
            (
                "a2e7bf7cad909a786bc8836ea707995d0d9d53",
                "dc9a2b2550ef3186f5a5429f3962b9593ab2dac7402d3e540f9b0f27a995ad70",
            ),
            (
                "e0dd2c06b7386245163f761bf7f38b353ede2a088c031defc2f4",
                "c9448ea4eda98513ebd7d963d527389c3996bdbdf1da97057d4048028982c0c9",
            ),
            (
                "b25db250f30b615054a58371757ef0ca529a02299a51863bb0eead0e853546dddf",
                "d8b401174f9a53881fb9fbda98a242b039628600517c78f1001ca70bd8a1a4d4",
            ),
            (
                "5832f1f6d58d288abea576e8806b37b11327e66829cb0bf79c1b7f76efe807f699c10e4865b79e54",
                "dc10155bc8e00e4325de0225b8182be3cc16fe248b62a8a98492a5e24d544adc",
            ),
            (
                "ea6197a0629b8e5e7f57cd1caad0a659fc2f36ef5708dc646e25f7f5a292e2e8313dbed385505be23455c27e6be3bd",
                "4bc2d68a8379ced24db8dc5343bd298c1b192f24c7a8055ed0b323ee5ccea393",
            ),
            (
                "bfda22201cdf552d520e8b8537e9962886a29a5f1371d0cb422550f74ef51f7c087eea9837297e5657d39d3fbfc37c76a77e973ea7b4",
                "a690eb129e679856b2184e1e4d755178216db78d243f7ebe3dcf345185e4a8b8",
            ),
            (
                "80bc01b14d26ba7d589117f7bedaa275b3754d7bf69e9d27b70531ab78018a8f2f95f4d315e3c755884e2179476d88ddc1ccfc4f1ae4384b44fa23bb2e",
                "3d3a17895dc22a8159a9d399cc2add762507de19351394ff5051cc6eda65b74d",
            ),
            ("c47c47", "af5173ff5eac5d0fff9a6c3902f09ffd61d0aef25b13c185d69b80e10c30107a"),
            (
                "f080ac710806bcf64dee",
                "9382a4f9037b4db318542b7be35a7a6dafb4722aa26c759436be2bbf4e292e60",
            ),
            (
                "b242cea5f041f2994874b1a62b14905191",
                "f9f03aa0c2d190f7f6500b61e65d993cc8e4af4d373629d7b7cfb6a55c59a5b0",
            ),
            (
                "78f732d6c784f4ccacc3c00eda5cbe7a110f8e90ad8cc384",
                "7ff1e2d49d0c4a2e5b90e51588587bbd70f79a90111863730228e097402f7e76",
            ),
            (
                "37b0733b747263fb37ccb470f84487ccfe148ea38dd8367a456bd17b5a9dc9",
                "98c9fa505f30b488137d98975717d54c21f8e2cc761eb6adfdb5fce12d24f105",
            ),
            (
                "846ef672cf854d676c87de9bc446b0c76bfb05c9c0e99694b379f11cbf4b5cbe1c10b521bda6",
                "678d4d60b36c470f7d049cdf95640adaaec2b2067a77c3aca3c988ae754914fc",
            ),
            (
                "521deab44bee2a023555438d5ccbff75ec02f4042e19c67c9ef8e6f57327f0f9068eb858f2199bef6064977e2f",
                "73a896a9a1c965ecc315f03a9aac58597f2956506e60b0a68fbac33315a2f61f",
            ),
            (
                "fc7c3f61cab4c8144e52dabdf5a3e9ef72009da8c037c2314ed6e1ea9a43afad1a46ec1d8c9d1e4be653d0f4c13e9b17f8cd5c54",
                "d8098c0ea0304d35ed726c26d4d725aba337095040c5397dd62cc9a70e3e1306",
            ),
            (
                "71b24fa83c18c00278adc2e343066046a77728c98b7ba81449a7d0e8dea90444a8a75b490c27ef353f075bf0c0a11e41da4ab4cb13086bb5f87c8e",
                "309302e6b96754e3ff59f0ffb90caf4ae90b6cb3e27940dec9454201189a3c97",
            ),
            ("97", "2a0ab732b4e9d85ef7dc25303b64ab527c25a4d77815ebb579f396ec6caccad3"),
            (
                "0d94a717f7cf5df9",
                "4542447791a885e66fba323792c4118dafd8361d7863e9bd0ddd9bbaab9ffaba",
            ),
            (
                "5cbdd132d2209012d2290f55e3fe1f",
                "8ecb69236a2a9f6be56f38c84603be9b5b92fb1c25005e43259115d8454b9b67",
            ),
            (
                "768eee8abea17656940551056c9713716d320a2a0cb9",
                "94d6dfef5cc4dead654dc8a650e2e132b06aa114cbffdf23c68039947349259c",
            ),
            (
                "47712e12afa327ad192c452a0da90bcda1ef906698bc1285316846cc22",
                "b90bb75bd297b8f546c139b5a26fb46fd112316fa5a3fddfdcdd4556cfd2dff6",
            ),
            (
                "126891727439acdc1161400642a80999de1f28da0c518b4af1278b8841b9c6d8b244180f",
                "40757cfdff2c73e98735951518dc091dc34b68fff1d68ff251a3321863c37345",
            ),
            (
                "54ecf2789b2e3eef1724b166367551e1e646afd9cce5767f8aabecc2f7fe6da69214ec57e211a7befc044b",
                "3b887f5f9c0c5bc6ac4f52736eec4ded964e9dcd117e8f9baa85032cea9c4e8e",
            ),
            (
                "7289d69a04babaa27fee66171cd1666b1628592204179673fd85327fcc397a6a8861ca7ad4c597cc09d2160c0877dd2b0cbd",
                "19e74cf19a326d0740027c319556ff831e65aa015da82fcb48985356a5d061ef",
            ),
            (
                "57a1a075a7ce49e36a5416941fc429ce21da61500f1c09cd834b488f5dd67a89a92e7a5e2a6e47324dfe2bc68d57a527cdd4a56a57425c4265",
                "0dc99c7dd4fe4ab85a5d8a0b7515e9534b19f8cacf493e8884493cc21d07b3bc",
            ),
            (
                "36e6464bef8d81cc1b98d3fbae1858991c549eb4d0fcb2c710255c8902ad4e7f01b14474fc76e0b5a6320c7dd6e794ac4b67326d55df6a86705312e3c9055510",
                "b4e57163fecca83e5b9dd7894d900b4d8ea01b7725e643a55846ce50853de970",
            ),
            (
                "d02319d15e87",
                "f455d9f7064df368166187fe17b14a15a93e67ccbc99550faac44a457edc2263",
            ),
            (
                "5be0a84201b7335640e6b07d3a",
                "9b33df4d8f7bf36d4a136aba552c3771d77dbd0270d350e3530f7a67f3fdde0a",
            ),
            (
                "b5e6806b44ba1dc2f80aa8d5476b5f872fd2dc91",
                "8a68514081af10ef7918660e44fbcc1c8a21f9d718c050106069a4bb31ea480a",
            ),
            (
                "d1d78b6a108cdf127d8c12fbd9a76452840db1273bce2a275889e5",
                "165b4bd73e2b938f6dda923b9f85157cfbccb3c5e7f61a670a81f23ef361845c",
            ),
            (
                "efc04ec547bd62dd5128244095048075a4e6467b8a6d18484d98578eb7d9cbda46ca",
                "9737e6d521bea8658ede7bd14e0ea89ce9aae7857b83834da50ba058d2e9c49f",
            ),
            (
                "ca1bf388d3c0c1f9e46da7e60431f295050178954b5755660e5ead3e6a199ae8bb7a5af372a8bab885",
                "f9b756edf36fa0a30b908f33be029540bb6b3e3319e4b32156e2e99f083fcb3a",
            ),
            (
                "ef341223e4345e5bd00b3e04515baec9d6b4572914c7b6870913894e8fe34cc0809d8805edc0d0ba95ec152f4bba9095",
                "2684cae4aa25adfcd28a91bf112bdfedaf8fdbc523df703ea8e67ac4e78d729c",
            ),
            (
                "d5303bd0c6f162e463f72a8b87d052f0ab87fafe2f68f1bcbdc40f469d4c2e540499c2f6b93659f368762d514e8a15d5c88b048e886550",
                "59a040487ed1c3bcc3b99c65ef811cb891a7f0ac7ac37aa1ef042cd1520fae39",
            ),
            (
                "46ddf95c5c68e60c4c20cf1dc2dbd750ec4260d377c378b23b986ee63ef8cfd7b86183c5b5e8089179f35b3222e2d7dd2ac9e9616c752b6d6b650916c20d",
                "1991783569a87cba2f48aee5160f6c8426a41b08ede5871c02bdbf694d467a95",
            ),
            ("90790696", "686bb7bee35d9686a38cda6def0398019c81b62d6010a8466a09f76206d037a3"),
            (
                "97f4f9d2bec84b1ca2db58",
                "c8b72bd84f580e94fa7ef8b4e88926d00a98b6f323fab835efbeb9808703548d",
            ),
            (
                "0dc0faf6c699812409a61fa80032ab87c64e",
                "f7c741567f52668613d4e161f2370e09eb31351f68f64ec682961b9f909585ee",
            ),
            (
                "69897e5d95ad505cc1e6838480a42a1a06a21595909ce559ea",
                "994211f718af7a3c3f3d7cde23f987bcf563317186ad9fe532c3ef54e159d276",
            ),
            (
                "9e9b1922fb266f55edbe32353b43bbd949da8a5b23aaab14173d3d41763939f0",
                "28e6c47557e78b2e1435e3e79677326ca5b45f28ef248f88bc955266288eaeec",
            ),
            (
                "0ba96e765c820b51408ddbc627b550926a41ec3786017ccacb936fdbd5030cde92e0177b3f5e59",
                "c60a584bbf1c94839dfd70d63f51dd6580a5dd30882556ae860265ad3083c744",
            ),
            (
                "7efc4a939f794fde6fff976a4e36750219c006200a4b8b482d03cc14a5bc9aa6c2eae1a962bf808a95facb390f9f",
                "abeb0f8170e3b34e89b300992c202adf65dff87ef55c2b1be845e71e73ce158d",
            ),
            (
                "69754d540d5be93b0c6c418b024052cc756800e7d385d426a782b1984bd4ee66d3719fbb27a0a0eb117ca154ab7ad6db15f9a6e9a7",
                "6e1d575d3feadc85cd654e209c846c8faa8eb34cc22b29bae2b767031b0e82fe",
            ),
            (
                "a17e78e502ac381990a3f52cfe18f177c3fa180c9e1e6fbb203121c41f51651a4c07a9f4abacbd650d1fcfbe45d5499a141720a9a34601a534d46244",
                "2d5f498459653442000ecf2a1502eacdb5ef84eca24e34d7c6f18e43f108b90b",
            ),
            ("0cc3", "7a0c18383cbee9a9f915458d171443cfd5e1cfddda45c2ae47116677a9ddb47e"),
            (
                "173b4e07d58da2b84a",
                "0900b18a88fe00998f30576a53a2110f624c12472f1cfd71ec477bf298ebef74",
            ),
            (
                "9989bc9276844ae2c38575e7a6f6459f",
                "ab9d4711ae8e020e0e1d4c34df449aad016fa41db94ef3dd9008af8e5be33683",
            ),
            (
                "e3adf34897d1d3505a6e20f80d92f298a2b8243b93d5cc",
                "585270fa18f29a8da9bc55f443126e69dd7d19c559a24105a2b202905b7daa27",
            ),
            (
                "1d884350dbda55b2c5305527ff323863193b49ed73ecc4eb93ebdda22d1b",
                "bfe1ac59f75fae2dfe1209929ec0a2713e5ff03e3bf5953016f3fb838b80a675",
            ),
            (
                "4e2e42e997b2419cb3a730dd86da3822f538b768f786c8a1baca7c37ffd263f2ec02ae3c49",
                "9e81b9634d27eb93863898ae74a9da2fdae5f75419a1874957efddcabb41715b",
            ),
            (
                "9645b681c6931964a75c2bd3a59e466341a8b07ef6dd6710eb6e4ded1da04a20be06bc4e9d7641981a62d1a4",
                "e24b1e7e7ff96921f9e0a172a5bcc251e448b40efc74fbecb40e2bc0ed0bfb13",
            ),
            (
                "1b7904a1c7f76e932f43fb076373ca0c0e190cc3c9916d1d334289bcf419c75f84e02aa0004b573b3aeb960513b7c11f8b32b3",
                "76eb4962e3328fe51df5cdcbbcb95713623b18fa9a096e6cca83c3f5fb6fa556",
            ),
            (
                "b7cb4e2223061892ee4a208da31df3692648cf332fa9a40d9446b1728c92281aaad42cb4154b9e413474bca529a5022159f88bd54a959a557f72",
                "e46aed5f9ae3665e5c0cc69f54faa4cf4df902d282645cf5a1d9393c75ce3291",
            ),
            ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            (
                "c1e8f05d93f1b6",
                "d3ecc667f1dba94a8d0fc596ebbb04034aefee3d61c4b0215aa039b83f33588a",
            ),
            (
                "b6ec8a3cd88a39495426773f7b34",
                "773743abcc2a19d291141de4f04e623b4304a25ecef1455d7da012bcddee5549",
            ),
            (
                "29800cbf188a9429e9d7fc88ceb7c68f6cedf9142d",
                "d997201cf5852ab4cbfe591c586146f110a40119756f84290759a295ec8f6dc8",
            ),
            (
                "1f2e55c623584e94f7608d792dff5de4f51c6e855066c7d7d1d1f657",
                "45762d28e49eae7c37e5eea3dfb659864f125a4738802d9b3eea33c9aad26e04",
            ),
            (
                "fac1e027153bef48fb9ed604a6144530f1b487b55dfda9eb14ae9aa5a680b0711ffd63",
                "8d84aa41a59959a42e689c7b016304dc864135b225b193c7fe3d458c5927bc50",
            ),
            (
                "a64fb45e63075365d49e208870d5c06a0517f98e479bd038c95747c7c0f9c1e91797f30de1a8d886ba9a",
                "6ec45e8f2da626bac4b8199900d7d5fa0a6212810eb03166630ec6a3316e245f",
            ),
            (
                "4581cc09e67075fbe9976deefd849d6b3d54c60b1fbec60de47d72b52bf5fb3c25d9aea8a6249d0b818852071def0b4d4c",
                "67b3a86c2598aaaaa332b1d440bd39541145e959e22a84998f7b6e1903f5b39b",
            ),
            (
                "59be4229c61eef4449f8d4e6c53556207cd27969c26bf577821455c5458061cdfceab41777e760956d3081bc00459baa0c04d3c5018bd25d",
                "8d4dd229acd5ae95d42f34be13a309f62ec6dcaa931bac296e65fb00aafbfaa4",
            ),
            (
                "6afe929710a922129e6e23aad4a7ab65ab044fc8f87aeceb3fc5de08f81f75d93df9e72ac1698cea13252486275d2899e54758af97237a2c5bc7bf17d7fdd8",
                "8b9a4cc1113bb0d91c3fbbf8f091ad1eb41454bf90c50cc9b474d32a3a90e727",
            ),
            ("d37df0536c", "f962631e447da006755f1aba864eb7e021cc605bb2a25154905a50f3fef363a3"),
            (
                "ef52c33f39a4c719727d7272",
                "7da8623c68fe25495065e20e3c3edf0c46e4707173a2a8ded939b78e88cf6925",
            ),
            (
                "3191eed0f9a0c4f40e82a701fc3b81f546e90f",
                "4e6453eb13458cfb321d7ee73f56e4e7b5823939909db258ae1570411acb5dae",
            ),
            (
                "4dd6ec69dad0424273718c2e1aa667135a8061a537c32e6ab387",
                "3c5772110cfb2bdbc0643fde4121b24993b1201e54067a5547eaa0af0b1cd29b",
            ),
            (
                "e278ec9b0e852a2ae50b0cb9c8429b9874eddd62a61bfd511b301e8e85bef0cd8f",
                "ed98fefc09bcecbb51d6d72d0445fe103eaa342759521fb70559c437e9dfe1e2",
            ),
            (
                "3d0ac03306981beea76dbaab8706fad915d305da68e22376c134e297f7c94a2e13477bd3375b2ae4",
                "ffc5fceffcddb95f1239df82c7a5de0f98b84771e152161ee7f56d2772bae22f",
            ),
            (
                "e9dd4d457b326c8e2be92b40bf09599414e079ef8d5067654ba9248bdf1ea2043f5bb00efdb5b791647d1a295146fe",
                "1f1965e6dcc9dc0d741aa5b70b8ec9f8510027d8b740d941d5cbf80edb754b33",
            ),
            (
                "a8d28a8f62d904e8aa59bc0e47984620fda6c8e59121dcddbf34606bbb703d1d67d60d82de8cab73c96441c0dd330c753208e06b6928",
                "b36aa00e382d2e23b00dde72d3f981389e4928e326f73964eb74351f3fdd8fe0",
            ),
            (
                "11f6d88f650b05c4c841783c327c2f02e067babf9d4aacdbec6a456b29b602eb24d65fec772cde7f149eaedb7c4c73ce28bf5a47d88d56009c4cee9e0b",
                "995606c012e72e7462fb353e133932be5e3f72ac70e80e40bd9d02b8bfb97789",
            ),
            ("54b986", "f233d372a0fd0af4e1ad718e354e46f79c059083d5d0424777cf19cf7993ec90"),
            (
                "0010dadb4c7f7a08ca9d",
                "af9b75e25668fb27953475e7e767e7dde962d4b1b0f57d6083fe02049f9d9b39",
            ),
            (
                "059501003f8fb444fd4ddf9efd18bda772",
                "4aa701ba48775c23e937b1f66f3a88b2d830dc1d2cace6b680822f167dc55591",
            ),
            (
                "9cc48d737d834fb4c795bfc77e8b34df5974be0b30403813",
                "77e324e53199462527d3596c3f2383a75f771926c73922362771dd648c55b7a6",
            ),
            (
                "31f8e05f995ed7014b544b4d526e2a95a6a7ef7966fe39d911fc95ef05a9b9",
                "09ac9726a19c8042dccffd1042a4e8e0ba29a1c7a34b11c1793a07ccc62cc1e5",
            ),
            (
                "1772fb44f784cbe9e0c3d111c7ba5195bd3a7bee57307e9df8f82cfd87fb77435b1db92de834",
                "be35194b054594f4619d9adfc76065be039921cfa609446da90b69601a345721",
            ),
            (
                "dd51ab1e908829bbcdb7419653a324aed941e60dfe3968b83589949443f28cfb261cfb747eebae184bf3445dc7",
                "8e2682590be42c9e6f88ec466746df1f16f64728b2ee675e0cb4e04d89f1bb5b",
            ),
            (
                "eb118d656016e9317a3f117559f0c0f535955f988a3735e20d4c4bd7cea50f64e383407be95112c205fab9e3c76b357dd6987c01",
                "15d42dff9a3108a7c962aca5b8062b247d096c2e68715095169bc37cadf450d1",
            ),
            (
                "13248056eb202d8d36ed3a018858868c4d838e683355fb6c0993da165ae1c51d2418456ecd9fb63e4c26b06b0e343a55a8e6b35bf0e8ebe779b972",
                "3a3d1fdfc18a1aa49b6a4648501620bf1dc3049bce5ccb78a905f6884658099c",
            ),
            ("37", "7902699be42c8a8e46fbbb4501726517e86b22c56a189f7625a6da49081b2451"),
            (
                "600e1530fb67088e",
                "92bae31cf8936990038dabb24801147fd21fbfa0f7a49e3d5eaccd1fe890c541",
            ),
            (
                "c47b98cea0071b39ccd44b3532af2d",
                "c7dfb6edf6074c6b62fa0849b68caffaa242f6d6939816049f2b0174b9a86d5c",
            ),
            (
                "88d8f4d679e2aa0f173e8b1c71a6730683fe0616c714",
                "1bc456e4b2333ffed9e4f96be28183491e0dcf8e3019238b93cab0f2ab3cbc02",
            ),
            (
                "1278a675038b13fb2f985ee503e7483eaef9492fa3f357fb1888343140",
                "e5cf3950c316f88a2a9cf558bbaf21d0284b9dd05ed08c4ba81644e64ef34993",
            ),
            (
                "418972e3e21c249e1c2a787f8623ce286cd8ab52a702dccc0b299c2adb679574f038ca38",
                "d7ed7d62efa7bbb0a42341209ed36bd71ff5ced67de71804b4b9982288e34564",
            ),
            (
                "10009d665791399c229af01d001dc2277ee8dd43aeea64b20959dbe5c4e8e0b41279a528a81f5273afd1c8",
                "457ee361787414b2c5b2a87410fd7281474e99e0da993f5358b6384cc9641d67",
            ),
            (
                "1014b899a4932e80222c56788f2c3f265a9946a648d7277c7a4f11998a641d6198e3f3b1c83942e78b1711ca122094a3e299",
                "43824b484bb322b595e7a8a5699d7d89beae57b335bb61ba782421c323ca3b1a",
            ),
            (
                "facf4a41a8f6db07a8f973dd914665d753e7eda48c285cccce431f54f7650daaa1a8a868845fd032726e7cd7cbde7b0c88081fbf5169583a6f",
                "3bcdbf3b403e2e1387cc057297703cc7bc3bed78623ee0f1d07ea2aab94b0230",
            ),
            (
                "cdb68c8db229fc673d1679c2484195f2fd4bdaaae7ca5cf9ce50e0d71da14a09756af240f541768c1f67924ff795a39dae48fd79e7a345a1db9447f21c45edfd",
                "2f4703b92e9c476cd0b16c3de811751d3399f33cc21d680b145d793fa08e0ec9",
            ),
            (
                "eda9f5ed5056",
                "d7251555fca8f07bd568636a0a53feae825f8532d263c5c3b2a94eae9844eaf6",
            ),
            (
                "947013199e766a02adcf9e2a3f",
                "231d2c0d064cf449a2ff5d4b1333dd08be8cdc42fa90b24ea09f6fa809475a87",
            ),
            (
                "bc05f4f8921b919ae0ef0b73d8a09ad9a02d4df4",
                "7789267e3851332393a8907b31f4dafe26e9497e3f3c66acccffef8c46a79f02",
            ),
            (
                "6549e81717f96ed4a90e441e0333300da89f52386baa7fb33e0dee",
                "cdef0f8a70c89a8e78ea7ae9daf296823f521ea0db49ead190d3c8435edfcce2",
            ),
            (
                "14c590d78feecb87bc466d930e702867804423b4f19bc7c5fab694161ec12574e4e2",
                "3aaa05d05f5cb94b971ae35a652fed2b15469e8224a6897b61253da59f613861",
            ),
            (
                "fcabbcb6cd6ea5b33e445ed305f4386441134370338c946a5cc328b2cce2df404d07c271db5ef9bae3",
                "38463f6563d5df4627f573b51ca8894b19a26d508d871f0a4284f90e41200b14",
            ),
        ];

        for &(input_hex, expected) in test_cases {
            let input = hex::decode(input_hex).unwrap();
            let mut h = Sha256::new();
            h.update(&input);
            let digest = h.sum();
            assert_eq!(
                digest.as_ref(),
                hex::decode_array::<32>(expected.as_bytes()).unwrap(),
                "mismatch for input \"{}\"",
                input_hex
            );
        }
    }
}
