use super::sha512;
use crate::{Bytes, Hash, Hasher};

/// SHA-384 hash function (FIPS 180-4).
///
/// SHA-384 is identical to SHA-512 but with a distinct initialization vector
/// and output truncated to 48 bytes (384 bits).
///
/// SHA-384 is provided only for TLS 1.3 support, users should prefer [`super::Sha512`].
///
/// # One-shot API
///
/// ```ignore
/// use crypto::{Hasher, sha2::Sha384};
///
/// let hash = Sha384::hash(b"hello world");
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{Hasher, sha2::Sha384};
///
/// let mut hasher = Sha384::new();
/// hasher.update(b"hello ");
/// hasher.update(b"world");
/// let hash = hasher.sum();
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Sha384 {
    state: [u64; 8],
    buffer: [u8; 128],
    buffer_len: usize,
    total_len: u128,
    has_sha_ext: bool,
}

impl Hasher for Sha384 {
    const BLOCK_SIZE: usize = 128;
    const OUTPUT_SIZE: usize = 48;

    #[inline]
    fn new() -> Self {
        return Sha384 {
            state: [
                0xcbbb9d5dc1059ed8,
                0x629a292a367cd507,
                0x9159015a3070dd17,
                0x152fecd8f70e5939,
                0x67332667ffc00b31,
                0x8eb44a8768581511,
                0xdb0c2e0d64f98fa7,
                0x47b5481dbefa4fa4,
            ],
            buffer: [0u8; 128],
            buffer_len: 0,
            total_len: 0,
            has_sha_ext: sha512::detect_sha512_ext(),
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
                sha512::compress_blocks(&mut self.state, &self.buffer, self.has_sha_ext);
                self.buffer_len = 0;
            }
        }

        let full_len = data.len() - (data.len() % 128);
        if full_len > 0 {
            sha512::compress_blocks(&mut self.state, &data[..full_len], self.has_sha_ext);
        }

        let remainder = &data[full_len..];
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
        sha512::compress_blocks(&mut self.state, &tail[..total_tail_len], self.has_sha_ext);

        let mut hash = Bytes::<64>::new();
        for word in self.state.iter().take(6) {
            hash.append(&word.to_be_bytes());
        }

        return Hash(hash);
    }
}

#[cfg(test)]
mod tests {
    use super::Sha384;
    use crate::Hasher;

    fn vectors_sha384() -> Vec<(Vec<u8>, [u8; 48])> {
        vec![
            (
                b"".to_vec(),
                hex::decode_array::<48>(b"38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b").unwrap(),
            ),
            (
                b"a".to_vec(),
                hex::decode_array::<48>(b"54a59b9f22b0b80880d8427e548b7c23abd873486e1f035dce9cd697e85175033caa88e6d57bc35efae0b5afd3145f31").unwrap(),
            ),
            (
                b"abc".to_vec(),
                hex::decode_array::<48>(b"cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7").unwrap(),
            ),
            (
                b"message digest".to_vec(),
                hex::decode_array::<48>(b"473ed35167ec1f5d8e550368a3db39be54639f828868e9454c239fc8b52e3c61dbd0d8b4de1390c256dcbb5d5fd99cd5").unwrap(),
            ),
            (
                b"abcdefghijklmnopqrstuvwxyz".to_vec(),
                hex::decode_array::<48>(b"feb67349df3db6f5924815d6c3dc133f091809213731fe5c7b5f4999e463479ff2877f5f2936fa63bb43784b12f3ebb4").unwrap(),
            ),
            (
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_vec(),
                hex::decode_array::<48>(b"1761336e3f7cbfe51deb137f026f89e01a448e3b1fafa64039c1464ee8732f11a5341a6f41e0c202294736ed64db1a84").unwrap(),
            ),
            (
                b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                    .to_vec(),
                hex::decode_array::<48>(b"b12932b0627d1c060942f5447764155655bd4da0c9afa6dd9b9ef53129af1b8fb0195996d2de9ca0df9d821ffee67026").unwrap(),
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_vec(),
                hex::decode_array::<48>(b"3391fdddfc8dc7393707a65b1b4709397cf8b1d162af05abfe8f450de5f36bc6b0455a8520bc4e6f5fe95b1fe3c8452b").unwrap(),
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
                    .to_vec(),
                hex::decode_array::<48>(b"09330c33f71147e83d192fc782cd1b4753111b173b3b05d22fa08086e3b0f712fcc7c71a557e2db966c3e9fa91746039").unwrap(),
            ),
            (
                vec![b'a'; 1_000_000],
                hex::decode_array::<48>(b"9d0e1809716474cb086e834e310a4a1ced149e9c00f248527972cec5704c2a5b07b8b3dc38ecc4ebae97ddd87f3d8985").unwrap(),
            ),
        ]
    }

    #[test]
    fn known_vectors_single_update() {
        for (input, expected) in vectors_sha384() {
            let mut hasher = Sha384::new();
            hasher.update(&input);
            let digest = hasher.sum();
            assert_eq!(digest.as_ref(), expected);
        }
    }

    #[test]
    fn known_vectors_incremental() {
        for (bytes, expected) in vectors_sha384() {
            let mut hasher = Sha384::new();
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

            let mut whole = Sha384::new();
            whole.update(&input);
            let whole_sum = whole.sum();

            let mut split = Sha384::new();
            for chunk in input.chunks(11) {
                split.update(chunk);
            }
            let split_sum = split.sum();

            assert_eq!(whole_sum.as_ref(), split_sum.as_ref());
        }
    }

    #[test]
    fn sha384_generated_vectors() {
        let test_cases: &[(&str, &str)] = &[
            (
                "e0c86be8b36dcaf990030f138e8dc6b8cdd2cc7e2bea08abb44ba72b2145b0cb01be4598456dde48b5d0",
                "7d11ba1c29e567eae7449a2793a5965c9d4808cc618fd1055f2dd66a5d2d90fa7aeda6679702a8c0d87a2cf7fdfd0eaa",
            ),
            (
                "a64f9629603789f5e8065d7b3cc0fff5606ff07ac832fe6a35f4313f2806eaa9d964d8f82ad0ae65f4c1a97fba787aa1571216f50836abd386d3258a9d9bc8",
                "5d0fe5a43e6b789c4a2b8a11ae489c3db4c0c3f81d72e1a4bdada7ca1027daca249a84603287453bb5b179cab316edb4",
            ),
            (
                "3a8eb71f82407273a35b725937d24b446cd1267c1646",
                "7917a9485cdfb2d20145e5e46a0e7367815c6725c840f5c36117c33df0adbdf098abc92f7140552af458121a571d641b",
            ),
            (
                "469a8a2dd0e0ef9292874c9077276073",
                "528b97b621336b33831a7741cd8237cfdb53ed32441aa8179995aa2e08bdbb17309f65c496f0f3f9d512c50e763a0c9f",
            ),
            (
                "3b0acd954e665feb5c870d9eea0221191f4080f2d1d1f3872f322716",
                "da51a601e9fe6f946a9c2cfe40acb51e09935932ccca0f23d7a3eddbb0ddcd4fd31a7ed467e21f7b4f478090fbd8618f",
            ),
            (
                "e6babd073205da831d670810cf2ee6d462442ad38579739383dd6f5e304583ce",
                "826fb8047db3e7c2549531bd874c167fa556364b220058058dded144dcb7eebd675c444cb3cf6e21def8b966eb97552c",
            ),
            (
                "5200ed92b4e8ab80a173cf3c573b1c6c034c4df24781772c7f46b1c26b0204e1fbd6400a472f300a",
                "c821847cb53031ecb786d8a7c856b860315f75ad0a746f71b7ec5568413929327ba72353eb46f4294cfd88427357727a",
            ),
            (
                "06dbf3f418d49dcebf22f5679cb400",
                "1fcca18e9129fa6180b97926b795a3cb72c6201472282b14a273488b740de316675a7daf0e6ecf376bdeff4fc5f9b709",
            ),
            (
                "3c243121",
                "4e8274cb0ee451daf788103941bea46dbb544c2b91fdc4ed8a03c1790111ff81f0fbbfc62b0584e19bcf3169f4734b34",
            ),
            (
                "987d5772aa8c2afc0c832e5bea8d43b9a52cd53febaeb2836212677cefba",
                "c72819682a2ff16afbbd5d15cf66728aaff524c69255f4d85b821c67f523df654d283379a8548f27e91e1c8fcfa8a769",
            ),
            (
                "4063edf941da1bde21192455b847b7ac6512467487be",
                "a09e84c3b558e2d8c32a91ca865d11dd6b755b903d6f92e238123c3ddbf52cb1bc92550999b728127fdf69a7418eeae7",
            ),
            (
                "b79e43c36fcb7b89bd37db0722221c77bd36e428cb3b7d808c2abfd4311f194a6fc770c20c387129975bb993609fbc6c698c78ae",
                "5661bc63721b66fb9a2e71cde4015650c99af8e586a596e910144542bc305b0333d9d59cfc364108e5d917e57077e648",
            ),
            (
                "a8607f13fa39c4a5d10abcf30a81c7fc154f3c651566aebeb952c1c7e089e3",
                "d9bc8e36d3f482aab1b58f3aff007de7047df5ff9173d7d2cab8802dae8741c0a2ae636d341f088c5ce644259b2086bd",
            ),
            (
                "8dc668d568ff75af55d65adc7a50017d3d550a42682902b5c2724e2d9506e29420",
                "7a318177db70ff79b4cdcbfc956e69b7d4c148c633f75134305bb106de863929be1ac3956bdbd05011fd5808a52cb578",
            ),
            (
                "7c6aef47f9354fb6d48b940a44",
                "c2e998245f8d24dff3dfc3d08bc7b151deeedd76f605851ac83dcb7647dcc82f26e758cf8e54091dee9b90a6f70f4c63",
            ),
            (
                "cb475978cdf5080ff76f54652d27980736f0edcc619713111c47e2ed6dca79819b8e7b56b567ba0b",
                "8a562ec8effde193bac7909afd9fba0b6e7479670f31ad335d083e3a4407ed38bc2aa9635b53fecf0f421f300087ddda",
            ),
            (
                "5474a866489959eab74ff8c98923266ba5570658aa0a9068223fecad9ad192944118e4273948642c4a4c22e22fa9410a7eaf227c790d",
                "7b559f9f7659b5269d712b5c5aa8a9bd23c27fc63fd0f9794b26d3af9b4613077c48c4d8dc5f2145a9fba4fdeedd9c60",
            ),
            (
                "9d0fb8c7b6aeda0623eb0d190c56fa37baab0026689aed5e7dbeafdf2cec0ac8bbdc113c34242e2cc996b48b39",
                "251d006760791ef8e018359f01663f8bfd5d41b7b0d8971160d49d0f726a3d7706049d36ceb3a00be79c837fb95978a5",
            ),
            (
                "5d98a392cb6ae460af849f3b21368c3befc1647af0ec3156a80d847ca94b76c38cfd92fbdce0f45d66e325a7caecf363be86",
                "d25976cc5b13502257f45a9708a54fe08bd97f2361bfbed4e895b16245d977b295327b356c51e212b9a4396e23a5a647",
            ),
            (
                "29f129c35b0e411f",
                "914ae1f26d9a590dbbeee0d0a52ee9644014785d58c5d5137936d4a1282c921dc1a56f35301a80a3e5b2ccd5ab51e3c0",
            ),
            (
                "cf79abfc3bf3831ef5df5629ca57f88d9ce202e3c367ca58d34086cea5165b5e1ce44004fb38462b9098686073160e5b",
                "c6c64d2dad9c551151744d7c3c95c039e4897c1dfdc54d2b71464982ce5a667fe0b6e3bc123a765dabbb46d305a7f6d4",
            ),
            (
                "55b5a6fd17628e539174fee3eaa0ea9d82517f8be86b03979389805117b9601c",
                "a343eba6f5f0ea5d5009d26280c625b2c619c5a0960251ae9f18800d873a2f0952b0092bfef0955fdb2ed955f216183c",
            ),
            (
                "b131bbd5a0ed1d7ebaa62700166518e18b5c6196f35940cf01f3a9098d5ac30c714a5b1d879bc8d69a8327",
                "00bd33548c5baaf82eee69f283203d4d9bc198047fed9893d62ae845b159e8cb67a20933f23f71cf7a4dd9ce43b52c58",
            ),
            (
                "dd6f1f73a7cd8c0f87cba02c197deb790dd5486aff91debb",
                "c98de06a2b633d493bdaf98c653acf256f2b85772d3327b04496a4e224dbc0e5880e3160c348c1bfa977fc0d3c635e2c",
            ),
            (
                "2774d0cb5a85b3313b0f",
                "3a9b4d39e686233355f4752947eca23b4b9b54651c3d664bdeb99a04aca920def4f3f908e717763870b64e69eb70bb74",
            ),
            (
                "7e9e2637e7638efe6ed9d3517e22603698255211d6b4ba",
                "aff5b3e27b1a5d6708c9422991851863ac30d001508e6ac375123b710216bee41d8087511b6afb0c9f9bfb22fb34b35f",
            ),
            (
                "574e",
                "d5be731e7057d65388d062445d2f6eb1e2cf365bbcb5c174cf1d970ffd06b4c3e6f9c6103faa05c914950205e0e73122",
            ),
            (
                "6342691cf9819f",
                "17aab1165b438ebbf48a648ff074e90a8e3b1bdcf4b592e8b2d3af22f8f1c14f914be1b8a3e1f7243c0ad21fbf5be4fb",
            ),
            (
                "a4ed28df5afc",
                "d0f431ca5a09179c0b7f851fe9d64950385e4118574a6bc29f0c2d4262271a2641523ff9665aeccf7c366ca02a16cd5f",
            ),
            (
                "6a625b8c352ca8100506db548928df961b478dfe388f",
                "1ec2a50d49109a0a51a1eda1d3069633ed9d090fb7ab7ab0e89c84ab99ca2d80e95ffee73a4ca9dc1deee3d41597fda6",
            ),
            (
                "9525e0bc7b581b40bab093e3b5162345d0d722c530fb2ffcd9c962d0a97dc106084a1ed8539a232f7701511f419cb9338cde4201071a502d6a51c78c",
                "be1892c642690942fa5fa849bea06718d4ca45dcb737563faa818ca57e253ae4444079383103f1141a3cc76f21576081",
            ),
            (
                "bbe2e9342f7b0baa0242e98bbf5b1ba4c20d51ef1f7ed90baa12b78c9ff41d2c7d878d5fa0515c2a22",
                "e2f6a42324fc4a44485a909f29fc32b79ca4a66de6e1e8dd2a005895d21bb7f6f13551f9088e9e524723c880f094c04d",
            ),
            (
                "cbf7b99de261967bf5f379ee69904de5585c64",
                "af99fc0dce899135e57fbee1b65a442d374e4f10cb81e96ac71ddc0cbb66903201b79f2bc368b3e3fe737c8220fd0669",
            ),
            (
                "71419b5c02e5d59ac94c9b6d16d2df6f",
                "1a40a15a448b0d7102905ea9591c0088c530287917b3f0bc73bc6b0ee20311c819a4cd779208ce534c5175b668e78294",
            ),
            (
                "435daa4e",
                "b06d05a9d2aeb21a50d7756773224b2482b8faa8e7986d5a12081e21cb68f6215c9302a42ff5afaba6a590b951e74045",
            ),
            (
                "34e5cb8d2813e0b79853f1cfe2cd32643209dd51481c0b87860c3cfb519000edad0180109f",
                "75298bdda0ee189a2a0e65aa1744cc921ec827093d56e83d12aeb1b746336ce7aea4c67c79d96de23e80b3791a499790",
            ),
            (
                "6039de86509cf3d1d62654e502ac68467a43a413b0d647b1feaf01833e8d95a26e79810c15c7",
                "70d3fd58264cdbb0c3c368e7546f9c6db5cc71cc48073184d5a4669e2b9f4e344835065ad8b95a9235bdba439c63d099",
            ),
            (
                "00945f34019772b167672c148804dd569fcc30d1ad388f331fcc338e361b3a2c7308b1",
                "def314975b7d7f1d056a40021705a82d28ca7e77ddbb432a1d76305e4fc58ce7ba6ec13cb8194b01e26a239cfb88279a",
            ),
            (
                "86537dbb277a050a36449eb0195d39987af918ad185da233952d9c71284c5c92eb7bb6d0ad72d393ac6a34a03b395afcc10040688c2cb110c44dd119",
                "cf76ae83dce62363bc9b19c6849407500dbdc590bfa741668ffbf6308eef94cbf0dafcdb79876cde553cc1499f841d46",
            ),
            (
                "eb3c330346ea454fcad8832def54ea5d931dcdc5a3eb76dc5ef11d",
                "6116b29455b37a6fd971644516e0d039afaacb5155534f350743b28bb3a4a21e49c916486dc6db76cf013c6645ee6ece",
            ),
            (
                "4dc3e8ef5d0aa78160",
                "92e1c0b03b965fb8e00fec9a106458d505622ec104b9b59316ea82263d76415675fd98633b03d0fd7b03bd4f646c1cd8",
            ),
            (
                "b30b23b6cd6eaca5c72339214c2269c50902ad80db8a67594f882deb",
                "9e502ca664d74d63a784d33ec776093f5eaded5c29441e5ef003fe0ee590099fe693b93df74a20647bb3c7d740064c2d",
            ),
            (
                "7b",
                "95ed401953c1d4c284ee689b77a75356c6b92a4095bace5f611aace86c4dc41925fdcf00ce4185354fdbb47d84be3d35",
            ),
            (
                "0c73660105e48707b1754fef814a6c",
                "d4366e94ec3b5c81afc2c7f02bc0297dc6b1b6cbfb1b7db76b56d9e3b945bf4b06dc262726639e4cdfe5e137bf7cc497",
            ),
            (
                "809e4462475d8dd3a299b2e55a785c263a1ee5c0c745beef1a",
                "dd37ce3ec7250702d2e977c43590ce1ff16b93ac24796e5fc9cf92c790d2c67c344ef10fedce0484150d03d7166e51b6",
            ),
            (
                "32480a0732cefb5b55398eeb2c",
                "561c142413041fcbf885d6f6bbfc940559dca0ced64aa71a137742947954a1f346eb336d238eb1e2294788dca69a13bd",
            ),
            (
                "8dd0edd059c3",
                "0089e8fa7b38a2f4f9e92770eafa27879e3c3f6cf76aba93ab2728d5370233f81ed5e66a44c32981068ccfcb13d62d01",
            ),
            (
                "6c1e61d22f6e9a447b8c5a4dc942862f60afd6d57b5c5320722a86d5881f4ed1820b6bde29547dab330546443ad97d24341128",
                "e2d9ddc9f8a4fbe6252062b391321e05476b51e71bcd6422a7d11eb53f22fc31ec3f35a8ad7d5b4e60f758e86b4f16dd",
            ),
            (
                "f53f1a4484d379c37b6e5afc058583ba8e6219775441e5d443455c7481a4d470731bd57a4542b0",
                "b19abebafb0b761b414d8982feb0335d4b1e891258d3d4b21f7421f018b5f1e8cc5f4fca3eec5b6ca4dd7358c1f0b913",
            ),
            (
                "1e78f1e8fbe4acc647b61825865556cfc0c93ca31783673a23d9314f6beda45505c1d6e2211ded0d3696a0c2c96c64da8ebc045250b2fa722a4340fb28b3",
                "ed03861e9c1ef4d5069d0178ddb90c09a7f8be77dc93bdabc963be2937a3a18f69fca41e087251386353868d83a9c55d",
            ),
            (
                "73cb9f33074abbc9dad9f3db1314f6175027dc1f55349143dde32c00952a67c2a92f8b7262f8a4a8cbce834b8fc8ba8d880f8404c69b50c9b034142ba0da9039",
                "d7c0b97cdcb713245ff708fdfad811dca4bb1c392e727106404728aa8f60d5129489030b02bf2146cc5ae44e658706e5",
            ),
            (
                "58bf3192b76c22265c903d3722a9a3f642b3a7c8",
                "be5b4c09441418e51a30d04a513da8566f3762cda9f35fadfe50e37138890d615a27dbc484d7c4ebd5b5d65051e46ce5",
            ),
            (
                "7af57a131341a654ab76e94ff89bfbc99a5f823df4477efc3d6fe751b4",
                "e9278d958b13dec8fee7ec63a31a80f5007ef7e82000157b4dc9be86bc536206702a595f3a35c7f3d4c32140183b88e7",
            ),
            (
                "962652236b4b3bebe15b2328",
                "20a9d35c31dae35c78b5a78c152199270a00ed38408e3a610480241a9d9289354a9be6b6987c8469aeb6db36663f719e",
            ),
            (
                "66e9f3ac085177b2b5b852d4a8e055297107380afbb73df3fccdd96866224cddad897973e354812ff147d3d77a",
                "fd381a1aec5afb531988698c49934de9a0f4c47f6d9b2db13a9eeeeac8faabf98d908d34fcf87885d4432b0790b0466f",
            ),
            (
                "773b743be9ddbb78114b747d18ea",
                "637843ffa5acaf2555b1cd17851abf5abfb9e3c425df755585b993b0cf1ead7c75c15c0090c8a73d3e25ff073ed2ed4d",
            ),
            (
                "5fbfdf85de807283a5b224e7c8e5946372e4d56d7099c0b5d964be9901303d30e388a6f2b37e975763c5f7afa9356acb229bc2cb21ae2588d2f7eadcd77e4eea",
                "887ff96fe1bf8c5a99ddd886b54c91c01cd419518d0defe462b3ed60d5364e899b6fa5b4a539f1f6f43860a7073b2ed1",
            ),
            (
                "7db87dfa33a1832437b03a9df22be62c575cb7571eb308ed",
                "e545cf08a0c4794765edd37cee31d01a1b4f7dc89b24656ed813fd4bd13d7398bf0c2c2a73a87d723a355f5db55ccbaa",
            ),
            (
                "9e9e3798129da97927a0bfd351dd1bdf86093046d69c14683aae240fa297334f3d122a6dd5a01b0efc635806cd969d194fd8862b1a5b",
                "5a66b233896a5ed12e566a43082a7a402f2324e321040719b9caa76483ea806946c52cf3f1ef36a1bc9dba308c9a5abd",
            ),
            (
                "ebee3aaf36148e74bc4aebd56366791e7dba",
                "841a5b5159c64e4649f7b50dfc2c4be8db7071b8db862b3ae876357d102bb7c4a3fc64d356c50d01dbcb959e357aca63",
            ),
            (
                "7cee9290a53731e1535f8066f1b1fcde6644994419ebd7a9",
                "86578f6ae8d3b6fde8dfef676fffe5775fcbbcf235af27a1ee9023c2769a37c0432048006fac2b88d95bd46e15d41217",
            ),
            (
                "28120c8bf844e9e6",
                "ace3951cb8c6bd47e71bcabb2dede9e56d779d0faeb8bfd4e6ee176f7286702237fba74b4b1763acd180706e0e247f48",
            ),
            (
                "a37220a20f2abe0b5cfdd3c920ac1548be306cfa",
                "25a84d7f8d0556ef7b18193d9487adf79b620b6e9714efd7dfaa8897d07e54c0b00ee2faee2f538c5913804f4e597290",
            ),
            (
                "697cedd1c68f3f97cfe9eb58f48763fa",
                "7a5ff69c23cbe0f2beaf10f03a5a7c8e5b8c86492a57befc71068ebfbbf4c0a81b43b2c87fe2ea27ba5f481c6d373303",
            ),
            (
                "481d2728df022dc759ed4e6b2b434a8eee75adb3aa2110",
                "a2501112baf6cb0bb9650af9b887b881d43630e00cecbd8039f3f429dfe17be50f47525bdd2043b9b2148417f7264309",
            ),
            (
                "f1df8ec7aa",
                "f9e3e6d050ac88827414170fd03d75e5544871ea376236cd3f5dce946c1c30c02d0e79f024da8c8be0d036fb32e5d549",
            ),
            (
                "47f6f4646c11e634dcd703a00a0d8320381ed9825308b64832270134a852ccad071454c42459daa3d3f63054a61b103a",
                "01a2ecc83977530134db678e0fe1e3a2b29736632a27f925988906ddec13e2960844e9b3ea21235a1fe7a119c1a7405b",
            ),
            (
                "2dc855f63f4784d34d026254f7cc79b9ca8aa9f649900cda38d40140",
                "7d74ff17e66781ed63bcc8cddd443c70fd2b8e9bce1ddb463d4f53f9a3ad0aff22a20e40babf6655bd539d23743c3f7f",
            ),
            (
                "a16f173d483cb373bbcba672857e9989fd7316378dfdc19015009c8d78761d361d76f4c5835babcfd1e36f6e8c95b96bba17587f059eff7dd5",
                "e9ce6872f0d7bceb5348676f1a6480378fac87e37fa3e79c7640604f25ffe8e4205397a229f53435520461d2d2229a95",
            ),
            (
                "e6510eccc564d3d4cd73cc208272ece49a26e67f5dfecd6001a10de0c7f54c21e6b13f6b287ecb8adcec",
                "ff9fb5f4d74be8fff73e1b42c2daa27d33e37798329a3e82e004d0dbeea5bc4ee8ad511a0372aee34d7f37b11b05663e",
            ),
            (
                "4a5e89554109aafc3892e70b08a68ce09c1e4a0292798a3e519cb9",
                "d8bfa901bd0403409676e13651726055faff98492b7110c800e08771c90f1045a68a9f336f94f0fa82acb10a851fabff",
            ),
            (
                "fe833fe13255b555718ccce8515a14aa8e626aa4aef586aa493036ae89dae7436ecb6fe4064ce010f5b7869f32",
                "4a36ec50870b0865d794da0380e8b02f9212c75be51ba3720505b6a5a65c8e8a37342c8adb14a6750bafa8d2429716be",
            ),
            (
                "0291c756ed8679a6ee7fd8ea2eadd3a3f385aa57",
                "4e491d4681edaceac78a7c5cc7da80ffeaec12e99af3a8293f92a1ffc96561dad65a6b53c1573240e67318fe9f0d0915",
            ),
            (
                "8a75cbd5d73d75c7b6c55bc62bc55f7444dbd28d22d3df20453bc2ad8d0190220fc17aac2e6c06f4",
                "0f671b9c234b9b5358ce5c689054ecfd48e375ca2d4f2e93097b516d6c4c9c6aadf302669d07b68a42719593ae1a07fd",
            ),
            (
                "326e51b0a90f4f82eb0877e14d2a45d06c2876e1a2ce6faef7c2ac1cb50d5615fe2788",
                "3ae5102f7b15397aedd9642def10fe9dfeedfc0029077efbab160121afc5b2199bd276a33c913579b85a1a8b6c3924bc",
            ),
            (
                "d400b941ed7ee9d2a1617f7fd2fcbdb947d8d84391906339a26aefb6b493dbdfcdf53e35aca20fa611d452cc86c726",
                "2a685c4066c9df23989607000eb5053ccb56b08a37cf862584440a1c3f59e648ae10f6932259de8aa4824dab69e13909",
            ),
            (
                "f16eecba457162d015703f61b8597cb89332c743dd79e9a95db881158e599b6d6d022f19d7ab67c5c62279833de3",
                "9deabf02777ef259a84c8216a8420299e7b5bf227bd4acdd98a15c071ce03511226926a0c23f7dd23144b596ad544894",
            ),
            (
                "e0a0f3320cf6976bee604773c276384293f0e3adf556a8036d542a70095322e8edfc",
                "e0e5632432477d3dd7dd1bc9083689231625adf622df282787ad1bb5e8a34e40d37107a5a4c19598b583d18e7811d2de",
            ),
            (
                "5a43dd1a51e7444ba0c10f5f865fc6192be4a82e2c642fc7107aa9",
                "2d69d371e51bf38df2f3415d7b769a867ce095808668de1ac634dd99efe820089d7208fe4e8a9b86c26bfa431b91bdb1",
            ),
            (
                "ad5a02d372fb2aa1889d96e67965b42bdfa9762fdafc3319f573fddec6db9b8ff486b3c74860bd7f4a0d1c90a0bcb7d6039a45651dc850a53b20a285e7",
                "061b7c9e90c948d247dc03b6455cac89added4ecfe0441f81f405fa778c492a7a318aa8d50a49794aaa77e4cc090dfc8",
            ),
            (
                "d7050b5f94b275afb87e315e70905ce87cda9004c585dae55354d07573ff5de54b9b2fe41b05b3a79fc13f93569257b64a",
                "c759f49502a6688eb07bdf6f1e330ba9c4b2e9a29348cea1c5e55679d0147b9222987b8f849d16af4b288c34708d3206",
            ),
            (
                "4fe5895e4461b073881d443e7a98f2bcd208cfef0147fe43316a862ea080151933a2cb681157c29c4c20",
                "1105368a53cae360ba045542f99f12bc83f2e7190c6a5cb26cf77509615ad097eee0b42dfd3208e17d800cc1c83e6f5e",
            ),
            (
                "57925245f09efb390a2e480b7caa31ea823098f2ca29369ac3cbffbebd449e0c42f04a80fadfca7b3263",
                "a5eb1dad95e527aa3c3a80b1c7503b8dc6bab48136d7493a0fab967d1a8654562b6d1bc43b0e4b5020b023793e973e8b",
            ),
            (
                "cad76754d4054910f04ee80f25469e3bf44d1cd372b8a6b448cf3b7b8264be313355bb40c586b6cdb7602e12ce34c0198a208693a04066c8ff2fe869c27b9f90",
                "1c443de9078a3f2a7232fc7209151694049f050b8db3778f64d2cbf1de5eadd20a5cb862e2c8f1b3fc62fe48bff679f2",
            ),
            (
                "c28ca4cfef32fcf86c4fcf271ad3629901a1575e9b718bf4d2b9e6c82d12375f5b6955b85b87011d445cf1b9163bd8bcdf0058605dda",
                "9a7e0607f60f3ec5de56c7cf1358585cbd9de40a9f0d09a5c352bdbbc6687f28160355a7845a0564bca568d5ab04d948",
            ),
            (
                "a86371a54784d45cedeb76149accc7b43f3e23459f8d7be8075c2440e12078b35dcf112665fc",
                "504b80c08f46474113e5cb9c1f7f7533ec34120ac140bd92b65e5d7b02559056e83797943c262b4975273d0af5164cec",
            ),
            (
                "6e702c9f408d0bc9ca239f94a41054fbd63f717fab59ec012ff8de6d3fd91c48e626b5877b8c8487b45614bd5259ea763890ffc16548ce3cf2ea641d2528bf",
                "9c8213ffa753dad5cf3f25523ca06059deab42912bde3765e0a5766d9e121ea3f0f7f8090af19837efc9e23a9a54db2b",
            ),
            (
                "dd6be36d77423a83c8b53713575b684d8c2b66",
                "6db8449d663614a9f317c58208089180a6fbfe172e8c10f54519c02513e948513dfe47484a9e61135dd5046586c23caa",
            ),
            (
                "2901bca38a397df5c444f230a61bc1f2ffda07f375a8202d8ed02ebf17ad604e8e199a7c1a7ae4df8bd6542fc821ef8abd33778bf7",
                "606faedad8aed4e58085294795b86e488b78319ab88cb1776129d35b5f7526b0564633d34bf3aa3458e74473f15e0ead",
            ),
            (
                "c26c0b70740c00ea2d664690ef01f669cd6c9e31fc2e7e5f967880edbf1883b928bf00fdd9c3b940495a62591b497e75",
                "ca2a82eb6b5d4b891ad0c613df8ab31f4b3b705689d3e00c3e0dfba019bfcfe6f0caf4207250642b81ca1af73e9f74cd",
            ),
            (
                "d56dd5ef31d54d7c94db62561f1cce670b2f579cf0e9facd30",
                "3aeccee4f1bb894661d13103b5230066268e061fe1fb163abd6f5171cb0b68c494a8230cb4f4824addcea4182b5cd0bd",
            ),
            (
                "b7ba8bf3b6c164ca9ea4c98ba51d9e56efaab28942e45c9ad36715bdce672f2b896e9db2ce634d",
                "f20e15cf8bb5d0f6899787b8549fcb03ad64a0ceac93664cdfe060b9ef1dac039ad5335c5482fad9c1d878ed1581fa4c",
            ),
            (
                "331e4806a5902df8527f73be5822d245dd01803a353dbfd016554a701315f629058074aaa426312114864ca533fe9e465d87206597",
                "526cc2e958b4f7e039b01d352cd178a7c47aebc55aceb162c648fda7aed3fee31fb7f671d1a86eb5384eb7ac74cc51d2",
            ),
            (
                "5808ae13401d01da61e41f75de6b4a924c72e5b9",
                "d51d9d14a568a6b56b41b47c351577bd464235830de9ba053cccaa884e0d97f8771ebd3f3f5ab396087c1d541cb60eae",
            ),
            (
                "74fc99cb7d6958db460a2c5645195f385271efcf34e0295c",
                "bc0ad7339fd4cb24229a13d8d2fd03089e24ab0c707ead689390089fe5627d17d9b8a86ee66f2f4237d323ba950c5543",
            ),
            (
                "e8127e4542d8bc9c5ab4fa30634a84e5cb99efebcaa1636ed1004cfd9fcbd15f691c6a2045428d5058240e7701519fea78bee57c7f3b5222b7cb74",
                "841aa1a4585ef1a49b60a03ded02f77a3d80f62ae14cac448e8cda8052ee9d11d09a981b89e524b15ed1f02ad75eb884",
            ),
            (
                "163c13fb1234d1ebb197dffa0cf5496dd71e3d47e8af53b12e8338e107c11d667aa53c02de1118a3",
                "e99af20275bd404df2994b71ec87d166bd31cbf30095d0c1a7c71c4fbbb7b3740e9008d1671a16e86e0e75f744a8633c",
            ),
            (
                "e1bd7ba8fc7cbd6caacaf3ab0928d5d21fd8a7b7a836ba134b0692a26d4c1817b096",
                "f71bc1e6ae5476a2211c55f718f271618761760cea46c63c7603e7acac68cf0df989e7f0293dca0486489cdd8b5b1f94",
            ),
            (
                "c684a2cd0f407780580a502112aba0d7b57932cce836a7b3eb8ee6fc6f",
                "7c85ccfb4d8fa89e559d51bafca96ae8df180cdf915fbeeab1b16100b5e48d148e75f106c5542edca4f642fcf2f9b77b",
            ),
            (
                "75a6a944bb7afeb34cc8dd7205215ab9e6c4576af3837f8483381d885d067d0e13",
                "82772b6f56e61cee0e5d3f423641709b186834c1780cebad861ea990b3edb68e205ced49e909539a57e380dfca9a792a",
            ),
            (
                "a527a8d4bf382f5dfc6425e6ca26",
                "a44145735a61816a9851d9b1962802d90ebaf6cfe14af1f9af095c0423b5f6a82259ce1e2a9abbc43b150d24ba3d34ce",
            ),
            (
                "0dd9b297ff1aa0",
                "cba588b8b949fe4297a0b61469108e46be8b8ec7fd66f7289de9d6a333b3bc6bfa123e9dd3054abd7314d714a90529ea",
            ),
            (
                "0691455c3850b54944baf2e7739faa9bf568f5a6462bc37cb6f2ccf7ef3979840c9e06",
                "895afac790deae53a958c9d2b12a80b4f12e398b921f70efe236faf1459196a0509382cc0838f8da76e349986d63dfd4",
            ),
            (
                "15c202646a2147c98a5803b8619f1978592bc7caa5719e8e1b55eeb7898967e5ef578334",
                "00457a59ca8f04ffa53b9361ff192fc19dd12b6b8647d0e622d6bb0d80d411dfd2a68130560d535625b4431100b6ed55",
            ),
            (
                "f69e8706e6406b7411036a795078cb197a9ec54c16dde378a87e6aa9e1d9317db06bff6c8b2032b2b3d2e3e358ea5832d793",
                "3c13b3638d17b3afccaf519c5d4be5259df578b934dcc8c01dfec9ec05e58a9a00c6ca183b10ba9a21a8dcecc9923136",
            ),
            (
                "bbbd9a9cb642dbd12ac0fb4ab063b9d0cfe60a13dfe6",
                "3264d11df16ba58df020d1a77d0dbce4526cb97cfb868edcf12d66a16b7122e5b33bac68c6eb63241740930f6460fd1a",
            ),
            (
                "d13cdd4e1c83eec3159711055212cf033a8784840116e4680fcf9fa1a8573ac196a10b1bfdb9a6198dabdae036edc28bfa0f6996b34ef3f329a768fa25",
                "99a14c3eb0a0a18db275f7b7a280c354b8b761f19bb31c5311778368b6686745fa72044b3f3fddecab5c8f08df864d9e",
            ),
        ];

        for &(input_hex, expected) in test_cases {
            let input = hex::decode(input_hex).unwrap();
            let hash = Sha384::hash(&input);
            assert_eq!(
                hash.as_ref(),
                hex::decode_array::<48>(expected.as_bytes()).unwrap(),
                "mismatch for input \"{}\"",
                input_hex
            );
        }
    }
}
