//! BLAKE2 Cryptographic Hash and Message Authentication Code (MAC) (RFC 7693).

use crate::{Bytes, Hash, Hasher};

const IV: [u64; 8] = [
    0x6A09E667F3BCC908,
    0xBB67AE8584CAA73B,
    0x3C6EF372FE94F82B,
    0xA54FF53A5F1D36F1,
    0x510E527FADE682D1,
    0x9B05688C2B3E6C1F,
    0x1F83D9ABFB41BD6B,
    0x5BE0CD19137E2179,
];

const SIGMA: [[u8; 16]; 12] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
];

#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Blake2b {
    h: [u64; 8],
    buffer: [u8; 128],
    buffer_len: usize,
    total_len: u128,
    outlen: usize,
    key_pending: bool,
}

impl Blake2b {
    pub fn new_keyed(key: &[u8], outlen: usize) -> Self {
        assert!(outlen >= 1 && outlen <= 64);
        assert!(key.len() <= 64);

        let mut state = Blake2b {
            h: IV,
            buffer: [0u8; 128],
            buffer_len: if key.len() > 0 { 128 } else { 0 },
            total_len: 0,
            outlen,
            key_pending: key.len() > 0,
        };

        state.h[0] ^= 0x01010000 ^ ((key.len() as u64) << 8) ^ (outlen as u64);

        if key.len() > 0 {
            state.buffer[..key.len()].copy_from_slice(key);
        }

        state
    }

    #[inline]
    fn g(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
        v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
        v[d] = (v[d] ^ v[a]).rotate_right(32);
        v[c] = v[c].wrapping_add(v[d]);
        v[b] = (v[b] ^ v[c]).rotate_right(24);
        v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
        v[d] = (v[d] ^ v[a]).rotate_right(16);
        v[c] = v[c].wrapping_add(v[d]);
        v[b] = (v[b] ^ v[c]).rotate_right(63);
    }

    fn compress(&mut self, last: bool) {
        let mut v: [u64; 16] = [0u64; 16];
        v[..8].copy_from_slice(&self.h);
        v[8..16].copy_from_slice(&IV);

        v[12] ^= self.total_len as u64;
        v[13] ^= (self.total_len >> 64) as u64;

        if last {
            v[14] = !v[14];
        }

        let mut m = [0u64; 16];
        for i in 0..16 {
            let offset = i * 8;
            m[i] = u64::from_le_bytes([
                self.buffer[offset],
                self.buffer[offset + 1],
                self.buffer[offset + 2],
                self.buffer[offset + 3],
                self.buffer[offset + 4],
                self.buffer[offset + 5],
                self.buffer[offset + 6],
                self.buffer[offset + 7],
            ]);
        }

        for i in 0..12 {
            let s = &SIGMA[i];
            Self::g(&mut v, 0, 4, 8, 12, m[s[0] as usize], m[s[1] as usize]);
            Self::g(&mut v, 1, 5, 9, 13, m[s[2] as usize], m[s[3] as usize]);
            Self::g(&mut v, 2, 6, 10, 14, m[s[4] as usize], m[s[5] as usize]);
            Self::g(&mut v, 3, 7, 11, 15, m[s[6] as usize], m[s[7] as usize]);
            Self::g(&mut v, 0, 5, 10, 15, m[s[8] as usize], m[s[9] as usize]);
            Self::g(&mut v, 1, 6, 11, 12, m[s[10] as usize], m[s[11] as usize]);
            Self::g(&mut v, 2, 7, 8, 13, m[s[12] as usize], m[s[13] as usize]);
            Self::g(&mut v, 3, 4, 9, 14, m[s[14] as usize], m[s[15] as usize]);
        }

        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }
    }
}

impl Hasher for Blake2b {
    const BLOCK_SIZE: usize = 128;
    const OUTPUT_SIZE: usize = 64;

    #[inline]
    fn new() -> Self {
        let mut state = Blake2b {
            h: IV,
            buffer: [0u8; 128],
            buffer_len: 0,
            total_len: 0,
            outlen: 64,
            key_pending: false,
        };
        state.h[0] ^= 0x01010000 ^ 64;
        state
    }

    #[inline]
    fn update(&mut self, mut data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if self.key_pending {
            self.total_len = self.total_len.wrapping_add(128);
            self.compress(false);
            self.key_pending = false;
            self.buffer_len = 0;
        }

        if self.buffer_len > 0 {
            let to_fill = (128 - self.buffer_len).min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + to_fill].copy_from_slice(&data[..to_fill]);
            self.buffer_len += to_fill;
            data = &data[to_fill..];

            if self.buffer_len == 128 && !data.is_empty() {
                self.total_len = self.total_len.wrapping_add(128);
                self.compress(false);
                self.buffer_len = 0;
            }
        }

        while data.len() > 128 {
            self.buffer[..128].copy_from_slice(&data[..128]);
            self.total_len = self.total_len.wrapping_add(128);
            self.compress(false);
            data = &data[128..];
        }

        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
            self.buffer_len = data.len();
        }
    }

    #[inline]
    fn sum(mut self) -> Hash {
        if self.key_pending {
            self.total_len = self.total_len.wrapping_add(128);
            for i in self.buffer_len..128 {
                self.buffer[i] = 0;
            }
        } else {
            self.total_len = self.total_len.wrapping_add(self.buffer_len as u128);
            for i in self.buffer_len..128 {
                self.buffer[i] = 0;
            }
        }
        self.compress(true);

        let mut out = [0u8; 64];
        for i in 0..self.outlen {
            out[i] = (self.h[i >> 3] >> ((i & 7) * 8)) as u8;
        }
        let mut hash = Bytes::<64>::new();
        hash.append(&out[..self.outlen]);
        Hash(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::Blake2b;
    use crate::Hasher;

    fn decode_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn rfc_blake2b_512_abc() {
        let mut h = Blake2b::new();
        h.update(b"abc");
        let digest = h.sum();
        let expected = decode_hex(
            "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923",
        );
        assert_eq!(digest.as_ref(), expected.as_slice());
    }

    #[test]
    fn rfc_blake2b_512_empty() {
        let mut h = Blake2b::new();
        h.update(b"");
        let digest = h.sum();
        let expected = decode_hex(
            "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce",
        );
        assert_eq!(digest.as_ref(), expected.as_slice());
    }

    #[test]
    fn blake2b_empty_variants() {
        let mut h = Blake2b::new_keyed(&[], 20);
        h.update(b"");
        assert_eq!(h.sum().len(), 20);

        let mut h = Blake2b::new_keyed(&[], 32);
        h.update(b"");
        assert_eq!(h.sum().len(), 32);

        let mut h = Blake2b::new_keyed(&[], 48);
        h.update(b"");
        assert_eq!(h.sum().len(), 48);
    }

    #[test]
    fn blake2b_keyed_empty_key() {
        let key = decode_hex(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f",
        );
        let mut h = Blake2b::new_keyed(&key, 64);
        h.update(b"");
        let digest = h.sum();
        let expected = decode_hex(
            "10ebb67700b1868efb4417987acf4690ae9d972fb7a590c2f02871799aaa4786b5e996e8f0f4eb981fc214b005f42d2ff4233499391653df7aefcbc13fc51568",
        );
        assert_eq!(hex::encode(digest.as_ref()), hex::encode(expected));
    }

    #[test]
    fn blake2b_keyed_nonempty() {
        let key = decode_hex(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f",
        );
        let input = decode_hex("00");
        let mut h = Blake2b::new_keyed(&key, 64);
        h.update(&input);
        let digest = h.sum();
        let expected = "961f6dd1e4dd30f63901690c512e78e4b45e4742ed197c3c5e45c549fd25f2e4187b0bc9fe30492b16b0d0bc4ef9b0f34c7003fac09a5ef1532e69430234cebd";
        assert_eq!(hex::encode(digest.as_ref()), expected);
    }

    fn selftest_seq(len: usize, seed: u32) -> Vec<u8> {
        let mut out = vec![0u8; len];
        let mut a = 0xDEAD4BADu32.wrapping_mul(seed);
        let mut b = 1u32;
        for i in 0..len {
            let t = a.wrapping_add(b);
            a = b;
            b = t;
            out[i] = (t >> 24) as u8;
        }
        out
    }

    #[test]
    fn blake2b_selftest_individual_hashes() {
        let expected_hashes: Vec<(&str, &str, &str)> = vec![
            (
                "20,0",
                "3345524abf6bbe1809449224b5972c41790b6cf2",
                "ad75ead79f7121d1f08afe599927a5a38be1b179",
            ),
            (
                "20,3",
                "70699c8dd92406789f2c3a5b85d644a1c56f5e43",
                "82799d7be8f4d169fb85e6636a7b6c50a01f70a2",
            ),
            (
                "20,128",
                "00830d363834916ecb336b93782e6ba320acc951",
                "7d145fe4756ee73b9e7873f1c5c08c3ccb134d1f",
            ),
            (
                "20,129",
                "450bdb71a0978ff09fe71c3cb4de5591dfa559b2",
                "b43f1b98a2faad56db45152455cb2546d36cdd7a",
            ),
            (
                "20,255",
                "74952cc68fa35387677c8e63d02676a8195cb9a4",
                "3589fa767728b7eecdeb6fb0463cb30dbf482ee8",
            ),
            (
                "20,1024",
                "5133fd036f6727b284e2135f0ef78503eef6f290",
                "cb8e9bf1b43d827cf72fa671d12268a97858dc3d",
            ),
            (
                "32,0",
                "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8",
                "893629475279dfd82a841a8f21a372ed30ccb8ae3462e1907f50663f3c036683",
            ),
            (
                "32,3",
                "c6fcb3c3542e65f2db9a75aca089bfe3ff63e5b3889f65bf1d2da61964710782",
                "016a18bb10e0c3a5e59fcefd1a407ab7f1c0361b3f9834774254d3f04cda3898",
            ),
            (
                "32,128",
                "9f51dad780c3c7f1915262e80ccbd443a91e5c5c97e8127f9c9f8536aec85960",
                "f30f6e381e31a1e84b710cc103bad03cb41f60a69da0b1a3ec633deada962c35",
            ),
            (
                "32,129",
                "5a9585ea35499d7d883d3a22eed1d3ebc46c1b1937d479b8be92b358c3eed78e",
                "1636b4448b28e3805416c9b960ed0c2e271eb87661efe8260c03d8318705b052",
            ),
            (
                "32,255",
                "517f6f8c5afea55f1c27cb8cdb4ff015c9602a550ba3080e5ac32796931711d3",
                "1d4eba1f03467518e224f1679846d93ad23fad4fb1adb808c4a21f0e7e7732cf",
            ),
            (
                "32,1024",
                "a001b41dd6bfcb13c73bf83a1fe5c881d49476163e472906c200375d8482b004",
                "64fab4bda27882cbdfd02b1cf3c2cf18e0248766e3972dbd2cf95e16d5f2999a",
            ),
            (
                "48,0",
                "b32811423377f52d7862286ee1a72ee540524380fda1724a6f25d7978c6fd3244a6caf0498812673c5e05ef583825100",
                "d72c9b4a734eb207e9ddbff00b10c370c89d67d796c3a7b96815a953921bb29759d29d2563f3da4d7f3ea4a6e34c326b",
            ),
            (
                "48,3",
                "241719372fc6ee737f7c51f5bcd1b93ab6f3aa4458d2eb6fbbebb364d1104dbd8d428ebb39b498317e961906da83e2c8",
                "ef46fa54a2c220da06a84c776e87dd0a21eeb5e9401a0a7811197418fe92701577d0a8532448e8b8536aa6c742cd2c62",
            ),
            (
                "48,128",
                "b3136da33705c76ea180813f9bd3a9751894eadc5e24795cb40d598d50d0c2b6625b08be1b793d3ea04a391d39a3e92e",
                "61801fc31461adcfa7072e3312581ffa46a285e7dbaccb651a91f3cf887662a82d43a18057f0b3077226ccfc8dbbcf15",
            ),
            (
                "48,129",
                "01b68ceef88c91df02551e491f8158856b78e974aba2c6921e8ce3a64610fba07ed68db633063b5e69bda944db8bd02b",
                "699ed0644914a9dc0dca937e7b509416f0ff5b3c9bf2e431cd699d1bf3772169371e6d7638e8e47bdf6f467b7a09ba80",
            ),
            (
                "48,255",
                "3bd8da79e5206479f24baa2bcb047cb86b29393725980b4aa222f8d5bf34a9b66f58d5594433d02d83f3751ee7f179bb",
                "8a6e17a63ac981b127e42539383d21861f9b150194b2ccce6a67e5fa46b31cb32b58a56476316dbec73ee8009128473f",
            ),
            (
                "48,1024",
                "0bdea1e5f570fb4a7c7765c374c2ee5118ed4264a95cde380fb1c005ed2667e0b5c881344866668efbfd2572f45bb398",
                "67c1c43b435cb7ff77ad8e61afa6881d60bc8f7262f4467b8253f60dce562fc4d28e532d87964911f9c5d537ec5fea80",
            ),
            (
                "64,0",
                "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce",
                "d74bf31e5ce5d8a25d09215253cad2f8d2fda91009301605a68cc3865bb7935bcaff6f2af643a77699e80261a1fd2c80e837b56232f7b146434aa74d7118bb16",
            ),
            (
                "64,3",
                "695fdeec42bd4554047c27e9f31f8a72ba566a1139708dee66f4d4e3caa71eabfd074dae263f6e8720fe5aa7a9ec90459a9a99eab6f98180df266872b608460e",
                "70fc57e1495fe4390d38a1d39705eef6aabbd264c7ce66118d0a87d42594b387dc50188bba61f091d6b34ff54e091e70240183cdb9211f1439775cc6e6e93573",
            ),
            (
                "64,128",
                "df525b0ab7842f39e8c40ca46a53d4f709db3891447ad0239d62f24c49cdf2f83c115ac82b6497cc4ee975cd5a83c29296f568efa4f26f64e1e2550c2ddda291",
                "dddf87e807c146cb2429dce212cfe2aa39c5f4784516b36b8a9f288d304d2dfa477eb637a450c85468c8559e3a65d06e7df9889ffcf3c9adce08e9bdbfdd28e7",
            ),
            (
                "64,129",
                "528a9ac84918bce406564e7e1d976e399d0e1744fe31eb3a27e64d7be707a826dd4be249fa3505bb172a0fcdc134eb6086310325f06131b11e52225fe25795b8",
                "4a2fb76dd89a695c8299d323ff96dbf35e2da8b9d51b6a4ac58f222ac60cb79b699fede415adc44ded4bd6ff3f74c3347ffc99d05e9020451baafbd83c6fb739",
            ),
            (
                "64,255",
                "f7c5fc8e9f50c53a29b1231e24c867848de1e136180ada04ee9352c420a966b015fe68c93f9207d985bb06aa8d98f3052443d320e728a2aceda699b5385c95a0",
                "c648c1f4aecc038b66404c8277ba16bced67d77e3bfa8bc2a94fe355bb6c4ce30d1111c29cf158db6922db8de932139ae30f2845ccc6b8e374499ce69699ee77",
            ),
            (
                "64,1024",
                "3b824548e1ba9c9e8510079458158a03a2150bda57cf363b907025cae965a96ae1f1c2ec41a60b7ebe1e2ac2eff4c9b5ade91954015966c5f01618b7c3b77a3e",
                "96cb77a5f16c16c37e518a0c2a6a77cc681030c1d9cfcff64ed9125d17673fc48d8bb540a0b322c72ccd92b5c260343f71a152e9e597258a83ea58edaf612ca1",
            ),
        ];

        let mut ctx = Blake2b::new_keyed(&[], 32);
        for (label, unkeyed_exp, keyed_exp) in expected_hashes.iter() {
            let (outlen_str, inlen_str) = label.split_once(',').unwrap();
            let outlen: usize = outlen_str.parse().unwrap();
            let inlen: usize = inlen_str.parse().unwrap();
            let input = selftest_seq(inlen, inlen as u32);

            let mut h = Blake2b::new_keyed(&[], outlen);
            h.update(&input);
            let md = h.sum();
            assert_eq!(hex::encode(md.as_ref()), *unkeyed_exp, "unkeyed mismatch at {}", label);
            ctx.update(md.as_ref());

            let key = selftest_seq(outlen, outlen as u32);
            let mut h2 = Blake2b::new_keyed(&key, outlen);
            h2.update(&input);
            let md2 = h2.sum();
            assert_eq!(hex::encode(md2.as_ref()), *keyed_exp, "keyed mismatch at {}", label);
            ctx.update(md2.as_ref());
        }
        let blake2b_res: [u8; 32] = [
            0xC2, 0x3A, 0x78, 0x00, 0xD9, 0x81, 0x23, 0xBD, 0x10, 0xF5, 0x06, 0xC6, 0x1E, 0x29, 0xDA, 0x56, 0x03, 0xD7,
            0x63, 0xB8, 0xBB, 0xAD, 0x2E, 0x73, 0x7F, 0x5E, 0x76, 0x5A, 0x7B, 0xCC, 0xD4, 0x75,
        ];
        let result = ctx.sum();
        assert_eq!(
            result.as_ref(),
            blake2b_res.as_slice(),
            "\nfinal mismatch! got: {}",
            hex::encode(result.as_ref())
        );
    }

    #[test]
    fn rfc_selftest_blake2b() {
        let blake2b_res: [u8; 32] = [
            0xC2, 0x3A, 0x78, 0x00, 0xD9, 0x81, 0x23, 0xBD, 0x10, 0xF5, 0x06, 0xC6, 0x1E, 0x29, 0xDA, 0x56, 0x03, 0xD7,
            0x63, 0xB8, 0xBB, 0xAD, 0x2E, 0x73, 0x7F, 0x5E, 0x76, 0x5A, 0x7B, 0xCC, 0xD4, 0x75,
        ];
        let b2b_md_len: [usize; 4] = [20, 32, 48, 64];
        let b2b_in_len: [usize; 6] = [0, 3, 128, 129, 255, 1024];

        let mut ctx = Blake2b::new_keyed(&[], 32);
        for &outlen in &b2b_md_len {
            for &inlen in &b2b_in_len {
                let input = selftest_seq(inlen, inlen as u32);
                let mut h = Blake2b::new_keyed(&[], outlen);
                h.update(&input);
                let md = h.sum();
                ctx.update(md.as_ref());

                let key = selftest_seq(outlen, outlen as u32);
                let mut h2 = Blake2b::new_keyed(&key, outlen);
                h2.update(&input);
                let md2 = h2.sum();
                ctx.update(md2.as_ref());
            }
        }
        let result = ctx.sum();
        assert_eq!(result.as_ref(), blake2b_res.as_slice());
    }

    #[test]
    fn blake2b_incremental() {
        let input = b"The quick brown fox jumps over the lazy dog";
        let mut full = Blake2b::new();
        full.update(input);
        let full_digest = full.sum();

        let mut incremental = Blake2b::new();
        for chunk in input.chunks(4) {
            incremental.update(chunk);
        }
        let inc_digest = incremental.sum();
        assert_eq!(full_digest.as_ref(), inc_digest.as_ref());
    }

    #[test]
    fn blake2b_block_boundaries() {
        for len in [0usize, 1, 111, 112, 113, 127, 128, 129, 254, 255, 256, 257, 512, 1024] {
            let input = vec![b'a'; len];
            let mut whole = Blake2b::new();
            whole.update(&input);
            let whole_sum = whole.sum();

            let mut split = Blake2b::new();
            for chunk in input.chunks(13) {
                split.update(chunk);
            }
            let split_sum = split.sum();
            assert_eq!(whole_sum.as_ref(), split_sum.as_ref(), "mismatch at len {}", len);
        }
    }

    #[test]
    fn blake2b_truncated_outputs() {
        let mut h = Blake2b::new_keyed(&[], 32);
        h.update(b"abc");
        assert_eq!(h.sum().len(), 32);

        let mut h = Blake2b::new_keyed(&[], 16);
        h.update(b"abc");
        assert_eq!(h.sum().len(), 16);
    }

    #[test]
    fn blake2b_keyed_empty() {
        let mut h = Blake2b::new_keyed(&[], 64);
        h.update(b"");
        assert_eq!(h.sum().len(), 64);
    }

    #[test]
    fn blake2b_generated_vectors() {
        use super::Blake2b;
        use crate::Hasher;

        fn decode_hex(s: &str) -> Vec<u8> {
            (0..s.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
                .collect()
        }

        let test_cases: &[(&str, &str)] = &[
            (
                "75d1b86d954d6e547e2f68d4f0931383b4e164a1fe92efa8e359d7184c7da014b7f5a50963fba234",
                "c8cd7910a752ad2c80e9f90c6943a1f6bdd97592f35779c4f2058250ab2228512a983a485616b017aaf555cf9e871ddf70cdebe12861eec48739a668cd8c10a0",
            ),
            (
                "8d32841d71c6e01ab66c644de8a530cb1fc72575912ae0fec3697e8f122bff216f652489211ad797e08a779490",
                "181bce7c81bc1025f7e4dcebb52bc985f896678f4c6ad9225cb993b2d26a8f5cc96f40bb5e4b7ede0ff7a1bd435c20eeaca11c88bd6395ea040df667dbe7ee1a",
            ),
            (
                "1eaf12db395d2aa5b2a8677c52b7877289",
                "a181cf05d8f3b5cf2750059d50233075c19e03e897cbb172a1d48df2ad90fefb45def3c0a395957e1810a68563857b8390f61641fd2035238f4bfb28740fbd2b",
            ),
            (
                "5c42992ab639f96bafdf2414fbaa5dbc18af889395822716fede82740b597734868401259fe68f",
                "61d7f06aebfd941dc08209e9b04fc730fa133804c1875e07b76d9aa6d9f413e786bfef5143c3dd6e4fa61e3eda7c0d7a0289a323f39399ad6fceeea4ee5abd1c",
            ),
            (
                "9e6f97587492593c49665073ea4aab585f4cb7891939619d57192937a9bb83ecdc9f1ecee7bf03baeae2d4fa2bd2d9489d0951ca8d0c110a",
                "482c83590278d22efdb701ca979fbcf49448a3a9336953a5642d97822036e2809357370f24937d0845c66c0734dd30f0c7325c83bc1960f851f658a1c962e683",
            ),
            (
                "74f7d172b60c9bec31092cbe9ae7b8af9f67b72fdeb89f79eaf893f0464db2fa6121a9371dbf",
                "1094e474a1ab3f26bcd7e82157048741f76e9f06df0fbcad38bd402952864cfdee9b707f57440677cd8864c3f60e1d96797c93e50ae36049a5a599a81d83ed63",
            ),
            (
                "bf65cad1832ad13160c4537d4152b967",
                "9f089258c27a2cd1ebba64c55311d953a97b15161bc11c42c29f79f28dcd8f686be78c54bb55f34a249d09638d5e620d0e358cc69b0d4497e1f79f80f2132e36",
            ),
            (
                "e9a3b037cd9780fed921d1c4f709984b5f3f6f2fc56fbfc0a5f36e72e6a99f2153f0d0",
                "dfa5b50afc602b7182bedfa555983667d2fcd4a8ae29a4163dfabb4b96da2b0bd1ff1ff420ba9f4bb5b8fc577691c9514ca23bbbf19c18c30495150f3a6c2d63",
            ),
            (
                "a509fc91c1d0735757f7f2c2993244f59c64f7177aabd2944232bd6f46",
                "0a3c0c6b4032098769584a199f1a64b2e8d54336a06759cc055a8a6cc9f97f1cbd11c156ab65d8753f07132561a55bef6584bfe6258e9b42ec9915040ec88306",
            ),
            (
                "3cf1d5eb3e527365975742ace715767751e5be16",
                "fd93695c896fc22aa2c5972dbce313db9af6eac05971b40bc52d98216dbb9dbedf6b23b35a6f5e2a9bbd91c14025aa4db9408b501d1e448403dfb481b4593fd7",
            ),
            (
                "1c8f98b06501fb393114a71c885bcc7bb6da7983b077dd67219372f82699948e782383a6b5daa6a725a7",
                "c6ac5977e9a058fb5658dd920056b7d6eb964d3b5c4ab7b25064632ca7bdd43dfdbdbda03a21c62044fe438f8086d10f59b7da492ff1050df8fd005183c67868",
            ),
            (
                "4d9c20cd5fd7660276",
                "a857b0889584c69c5956dab0a4311fdd5615d1c7a7f0fe572de85b5a608a31655040a70f0a7b53373c964609efd85b90f9554ad407d2d2655fff0c5773a3ff1e",
            ),
            (
                "49213d42338de9aed5ad353056250f9b898fba37a307aa8c671f7e1c3da982ea464381fa07a433472d13c0677ada22e0ee6cee802fc72795c79bf38f",
                "a09f4a8f685cfa07e5b84f3aa0a20d5ba69fa95eb1e1a6e0a3e72bfb9587ba4666af94161890813618c02f113a72a697b5f85d4d1aa686d7f2f22851da881644",
            ),
            (
                "8df16b491def64b3",
                "997080b8b5934828881576fe752ff34985ebf84fd69cef901a1723a18dd906475de70097d526f185dccb9366c6bf61ceabd54587859e02eac69050526edcb76d",
            ),
            (
                "e1382390b68734a4764748b5cc15546b782c0e3207d9e7d949f83c35b4ea4d043c876251cd33ec4f66bd",
                "5fccca2d44ecf8ceb3cec4478c561033200e636f8d77d8f8964e5b1df85ee807786d51cf8b14087c4e579c1808c4cb518eafe7b93ea97dd89ff671d0493ecea5",
            ),
            (
                "126056a2e85457a48287dae09a95a12aea50df629f7a4769082828b96d754d03e1588450",
                "1fb4c44e92349bedcef183972afb26a1d1f762c168f91252c3638f04597fb70c3568863653bfd0783993716bf8b67fb993ea9df2832107987b2545f9a95d5dec",
            ),
            (
                "6da257f867133cb527729db5f892ca62913f09ec66a06c635aad1dbcee4f9afcf4f297d6d490",
                "e01f58af794561d03a0a40888a52edc4a4bd19b173534dd10938d889d2b1640c27ebf5dd49eedcbdcfcc5db90d1f5ecc1851257a9664cf58a3dce6a44ca8a00f",
            ),
            (
                "4a1ba94df9a30757837b",
                "9cd37e3b8a5b4a70b60d38f2f4509c39756a8dc62d0d5d607277a4554167e25a81542aefdd928ec51daae6680688f40d9ae26f2e6a40b8182a4754c4e673ac61",
            ),
            (
                "b1049fec844affa3a11fab8777549a9f5cebe032569b15aab2a3cb0ea8890ca938dc6166878347f0ae3ef1ba43a119dec454a2ccfefe2c56143d1b07bb1a48",
                "4a319f0f5afcf99de48cd5c609890652eeb554e00f14cbe70d5105370749ca9f8bc742bc4b0541736d40ef8bb1bd7ba69e80bd7a45cb17c223d71dcb44676cc8",
            ),
            (
                "3be11af9dc526b17f53aa3415cd12daaaa8f61a14c9dcde0c876c6a9c2188b52e43edabb2e527a810e466afa518465f404007580ab876a1258",
                "88622efe158afabbd08b2cd1cd80213a8424c43b58621c29e186aa8774f417d4a069947b0db471926269239bc20cd8df151864ac6954cab691dcc27c61055ce9",
            ),
            (
                "9ffcd27e73bbd4481bd544b94160c1246f09332919e144f080ede113",
                "7713264790a6139b93bbf34e1d75d8da3bf237b6a6abf3efdb267e93442fac666c746e841a84973a7cb8f115e63825bae2ce8b3bd87b03cff21eb7d0bf8547a5",
            ),
            (
                "05ce499b7a20ebb409a999959ba9be8c069cdf782b420ba9f093629034f69a6b3fc6f5d04f9d8e32495f97",
                "1d4c3d0af0ba8acf6a6667fa5e29f96bb97aecf793622eb00166ffbeb51f2c8c82f8e9f25fe5f40985af70aa5065e4f085e71abda840e69b2c937446a0b64e05",
            ),
            (
                "b9ebd01f412cf3a04fb2150c41f9476d265089a55f5b389980181aa47a15af5c16fac0af677648",
                "2d7d1f22328279a8910662082fe72ae64c8d5b0369f0222a7b0378d546702561712d85c1bb75424dea67bbb3427fadec28cea6906a0ccb051d01f729258ccd85",
            ),
            (
                "5d012fc195aa08cfc543e7460f2dae58512d",
                "4d34526816f777b0cd390942f82c44c64e25b0c872e0ac21a3f2a2ca77f33e0913c84f0e1d1a04f8ff2718f2bbc95b6721f9b19e0709ff4930e6e25a18ef18a2",
            ),
            (
                "e4ba8ee19153e08c2b5ac07e1ece",
                "3833021e1b51becbc74c1977585c4ec532eaa17b9f41f32b169b273f73ed0c4965847697bfa1d730072a4e43c507daa2c58ecb33f3f99affb15cc914525cafbc",
            ),
            (
                "c0d9f06b6b6becc5608877252a16a54838a88e448d4acde6538acdcb1732cba2e219996431e923b9819756a78ced4c",
                "1d45548e8271263576a5edecd51ffe1cdd323222b8f984e3e70ae0b460d11a8418c4a35f47e639e882d0d18b36623dbeb64c009ef1d2dc36c478e1d8f58f741b",
            ),
            (
                "db63473d836398f84e286c2688659c9c1a6e2083ee5d820a7d4188d1f90d8befed7e9e48d06aff39d8089577e57abddeeee9daa4",
                "5a7d076f7022e72fcf622ab5b54c3b1ecd29fe9f57d15b7803e957dd8cd268c1c42fc772f4b98f867253af04fe56760ead5b195c35ba5a4c148418bbefeab7a7",
            ),
            (
                "5f0bc2",
                "9dc83166327090b23993c2021d538be15ebd15c4c75cecb3c19e684fb482296b158744ecf9246e86ff68022f558fb96e79be87b84ba0cefc0524910f7c53e567",
            ),
            (
                "fca579bdecd6c70eb2a6d7000196112bb04184d951abb8c5699852ba86b5a976d81f31d5f0dcb2791fa2c8efee720f326299275bfee21e533430",
                "3e5a9a62fedee7d69a38a01f6055738db84fd1c093665e44541618237a9b3c65c6a7b878bba599e40f56026a42ce2c61cb79ca6dbe5f215d8f4f68ddae385c7c",
            ),
            (
                "aad682cf2ac2d77fbcfc7335e4e429f1fb61a8e76852afb077",
                "50eb0b33856f0ecd6c4b8a3359837cc8784b7f3fd08a658e5a4d6eb4752bd33dc708007263f1a651ecd251186a92a0a869ab8e47e0ab844562fab0119194b630",
            ),
            (
                "3aba2b9335068d1f126885fd9724545c56991243912c6d5088754d7808777797ed3d06dd35416081ff2f0e",
                "4a54cad243ffbd9b2b929ace7cc05bdaeb65fc934ab1e15a5728d45e1c56bcbfbd8cb1e405da81b9ed889aff88e1b8fb03ac65998459260bb32144b35cdb7352",
            ),
            (
                "dd789a6bcbef0d9eaed0b4182d79b6c575e35070ca5a0ba78473ad0a04d1c9c07e746314c5384217",
                "de094008f5f68f440eb9c40f2559f2d5415ecd6c06e9d02448addbb998f48cbecebdbbf4070af19bf1c36fcf589cfba0f226bb168e9bb1c8fb4329d85c11fe08",
            ),
            (
                "86d65df15714b7a5725df7eb499b5ce76cc820a2173334687ac9de59e4e51a703292af0eb7d55e2ad3b033fe4c519e5456f49344dc64ea549d6813",
                "536a6f9e777763fbaece71a84168c418907fc4f16b6d385f3e4f04a463ee20bee589215f8de5de66af04c8771c72e67724db9648bce2f4611f1898ed73a18dd8",
            ),
            (
                "1aedf8ac3922dd10264cf68264ab9f0844",
                "56b9fc245fc5dd1eb0860d7227c5fbe9ea5c2d9c76d5b40e030bd257c88b364e93fe9b060aebcce8ec044bad02b73fe003ac9b657a69b7d8f5a553bd3cd336af",
            ),
            (
                "872c6e341c148cf3254f98a6a7578d904443a969",
                "ee327ca8b3033bb1f162d0b0763aa48fa6cc41a5b826c78a27e8cc3a9748d6cc5c6e0b394fc502fb7895b40a6b45af9de742efee6163e59432696e5d803020f8",
            ),
            (
                "5504",
                "3c9b63144b9a3a336a23f2318dc835e899a2bb19929d80294461fe3af79d54ef496dcb96c20a437708f7d3295cbf24daccd4c1b496ac95f5594f8fc6f322b5ff",
            ),
            (
                "b4e6b94f5d8c6b21e5b4",
                "5499dd884c78179ea2bbb870f1aebf96dd540f900e715b1cfba54855e684f40ce6a161014282cd90c4822b3cfe2d53cd5f949544ba90232496a97516f66d24ca",
            ),
            (
                "7120925d3c81dbddee5245600056",
                "eab1ee71a47b5da70ab90ec22057d0fe27b71f43d149f20a0f32c3fc00855fed8cc4176b9012f6b89b0d7e6248ed21957f5364061e51a64debe82222785fa607",
            ),
            (
                "5518d3bac7c4",
                "ab853629b40b52f133840eb8d15cd151d6861cfa3397136d6809c19278b5b190be4c6080cd29dbacaaf9cad3577b587e94e6d16adbd1a76fd810bd7b22c47de1",
            ),
            (
                "c30fd57bf53e76232cd751d8528354363a8fcb96fd8ef301849aa67f759dca9f2e6b74d7d828f803531a496163130e",
                "2850c18b4d845fbc8aea90bd206132314ab323937c61a0610bd326584e2ae96ca3476e162e758b84c19ba35815d0fb5067860323bde169c8a3ecdc7ea75e5c5d",
            ),
            (
                "e85ea1f54202462061e4758651d6",
                "f34414e90afb4a7fbe25beb6e811eddc007b3b060708effaab5b6ed4d226d167af7d6b0ca458fda1d9cd05d373d1bdceb36205e6ff75fa017e911ae23f7a2a1e",
            ),
            (
                "87c89c893a7566bdde2d8b",
                "2b985f342c75a3b47fbfff676b8c30c1de5cbf7ef851821f978ceca4d769945d20034b7a1553bf9a2d911e437e2ff478381536329b22826a70e7ad1ede73d8d7",
            ),
            (
                "a64416172a92e1a00ac3be50320e38bb1a4b68220f97542f2747e17846b2474311dd9dc79e2d16c4",
                "9776de8c9a3ccceef3cc7432c810e5b04411badebe4e3f6b2e0bd45f8b82cde46872d100df09edcadccc2af88026be2334fff43fe36cd3f4308d9232dfae3df5",
            ),
            (
                "91698d94124b40ac0e96778b80d01a62c586e09c5eecdfd3f6c82d9ae00918ae15b4",
                "950e3a453a2e3b782a88de918c64310e9cc457038650d7c1975d2289848106fe233672c1275da6ed200a7a96d24efba88d7fe6c89277de8e02d91040b279b74a",
            ),
            (
                "1c335e0be7bd220117c39c688c5992",
                "bb8ca6c69e571d364a2c34261df69457ea20f67c6905fca5bb98e4a64ac0927d539556ccc3b635542f2b1e866796697ac445930d42fa900b07d54e5bb9686db6",
            ),
            (
                "4010c441abfab690368da1b6cea229c7abee96c8a8778aa2be2bb8880105b3e64df17f45a2cadfdc2ac3f470edba9a9d7a2d05282964",
                "7ffa0634d78e4f02abbbbbb1b0094e21de4c1f5fe5d0a627d99d9364d2890e0dbc521da2161414be0f12222c4a43c68a318bd8ec3f8d85bd2ba7c0939c7f8674",
            ),
            (
                "2b9ab227b3e5bb8c",
                "a422694b9c51fc4b5728ca2f8c2495d28381887c206b91c38e843b4f94d0a2c53851a6bb2ed9324181d3f75322b4cf5bc92e216405f94a0bb1c0930f5436a1f3",
            ),
            (
                "711dc80ab32ca18d619c7ef8d97df0dd4b6fa04d19e34dce0adaa241a2e4d4de353ef572d4d7ab049d",
                "a055c7278bcce058eecc3e5f66283a5ba8defe12ec3f9f76ab8745c295a5436e0beb07f245fcffdf67638f103db592672e7799fcffeba1c50a1995bdb6fe98ae",
            ),
            (
                "2787f39a51e3e3e3192be17e04e6e55e4f7f367bfab3a66a3db0ea6210126b31",
                "43ad893b403e7f1f89bdc61f68e9fd8de434a31c2283c76c841a97773cfc79b9f4f091649bcf81e26d35b68d8b11ef50505a344d6c30bf756725f2a31576afb7",
            ),
            (
                "2ec5e8c625902c5a7a0ad163d15160620fa3f3b027ec65a55c",
                "4aa26aa207c913d20f676b04fe440d895099090df298440b1e47051ca896d4c7df584195cff9b519ab9d25daa1d0e433df8be6543eed9911e69dd5b5c6becfcd",
            ),
            (
                "825b07ceb8a8fa980dc047b792c48bec13a0f99d1cc345a810a847e92cc6a903ac1267f5ffab61aebdd35bd910beddf4df",
                "b8396530b84edd5155e3c88c1bb726f0d925cea22048c083c0f193b96dcc24034c38b02bf92297e9356a89ffe462c7ee8f95d13212134413548cefb9be2bf654",
            ),
            (
                "616326d8390443e3fd02a1adcf6402",
                "1cc3e6909a9430b4d74b1a94fcfe2e87c4a837d531668aac30ef9c1342cf2b2e75387f05897072e928b17ef2f306c4f111f23ffe6585a3eb1741d5498682036f",
            ),
            (
                "a79bb0f77f0dd7227795663794631ebf6cca5bc4ac49be10e936cb96c877d46908c6c4bed9f03d",
                "4e86a872c306cbc61be800e638b5a927d215059b02e6885428ee2aa683663e0b31deb623fe509cd95d465f0437097e0fc4d03523bad2b77ee4286c4639f9b52a",
            ),
            (
                "498984f11328350a11cd682595c205d467df4c22627a7b35ba3efa81be40bad3a0fb642f79cd991ed5ea2ffc",
                "9c38e4fa53f7f508fba1ce7b1ec449d6d4e4d6cf38e786fc0eafd8909d087534ebeaf4869b999b64b7f3af17fbc5f90c768a7fedac004ec70ee7a5effdea224b",
            ),
            (
                "b898e430837bb8de4ef2",
                "d90af307fb1f1290b7d9d6492919617743bbfe3a173b521cf7758d265de4d81e71bd6c9a411b0719c0ea31e60ceca469d71760cac63bd545c331ffea7b2f1799",
            ),
            (
                "2a7fd6dcb59b17d88817016268",
                "89646930250c0b261dc3efae7bc825098e7bdb57b08cf1f672101c103620df49a8e4438588edc978d2c4c92ae02fc0861c5df436218e83a728435095431a425e",
            ),
            (
                "8817c66ef6590d0bcfd7e3",
                "fcef24aaaeeda05169fe111ce2bfbd49e0625e9fd9c1f2daf579bab0dc48fdab1650825c6f42c08d275faa94b9e54b5f91572327cad37f43fdfec65fa573872b",
            ),
            (
                "5f422542d61b436e2fc1d623f1803c497aab74f8ffbe70e416968bcd1892f310470b1eb03960e74705026ed1308f59e9bbd587f6a9c2ae53b9",
                "2d4622d25645ac7c58d9bd25734812bcc583be9b717905d970fa597a19cca93f6712de1cd35d08388ccd8824c3e8b4b396b6c0087a1d4f903728203b947c456d",
            ),
            (
                "bfab33170cc73f49aa99e5d515b91bedd6db23901911027d58d7835eb9df5ec7e51d779fbade0edccf7a60ac",
                "d885a9c8c44c5d94278ebc7e0c4a4024fd8a151df8facf6766b6213e42f39449c787f0fecae125946e550bce0d237f607c0122431cf6920821bfc529331020a4",
            ),
            (
                "c2bce8d8a3b767e22c82ea00b219e0c94f06dec113f13a0305614adceacb77238add569b18e0dfad7d96d5",
                "5f49eb903e77f9e5df64f807ed9b92875d77415baea974af31b2c795b54a9dfdc87cad463781bfb70376b4bf4998d1db52ff58050049adae2dbac8ee52e5013b",
            ),
            (
                "b0f72af3e29cbe0269cca1e7f2a9e499b3bca5bfc47e",
                "f380b286644e48e7b7f11d345015f65e2e91d0758bd860a4565e1d5fa8838ee19d7ae9be5ce395075377db6621bf37e9d6697475c19cb84c01bd96cd469001a9",
            ),
            (
                "e8211e313eed1c3fa12035f34bf5dfb7c4aac9660fc9858515f5d056930bdc1af468379403b03cc5ddbbe14c",
                "89d54361ae5a320c0509ae09aca4d30d94f34552ebe57940af3b7d9b4874b3d7b75ad562edcd3f3cd6d4fecfe19b23175b2badbbd440e175f6ebab18a869fa67",
            ),
            (
                "c83a9f2029c131ae9dba46e26241f790322efb40a9706f0f",
                "50cc981b31eb66ec6655c82847980b734ad52020661914d51cefb5367863089fda1c96c8a8970570be77e11c25d907185bc1fc1b0257f00357ff40f77fa83254",
            ),
            (
                "cd29f126df27c7c87719f9",
                "217174185e1e9c57a17cbe332ce487b2bbc5eef6005044129694cceea697a2be1b7c84e0e7e214978791b2589e7b43cb0fc36006598a94f7cdfdb7fbe2128e64",
            ),
            (
                "b85ca7baff8534b2352843f218cd25f11c94a56b8ebd93111f68ac056a061a5a93a43e",
                "553f166ae2ab5fd42e5bf4c1eeca9bc239a7b89c6223cf161ae50e95c39d753da9b465689c65412439fcdaa2f3f44e1f3f5cbbe9eb71eb38e9b697cb20d19d21",
            ),
            (
                "374e626df5a321b55cae1739b3",
                "311e357b3e938be703e6e82d96c34db16e5b9fb09bbcc831d5a1ae714e7fc38cbd960213091a5208a2c8d948f0c52828da2a270c1165eea5993f8a7404f505a9",
            ),
            (
                "27592526426a433d428188c375ca7db154e0853d316b",
                "614c17a80462079c6388f8c424baf6c453cd3c767b4134428ab5a53460dcb1ac4ea0452cd1ebeabac7020a442571e1ff245b92de8554a40c341d364611afb6fd",
            ),
            (
                "884176f69e81efeb1cb8e5227b5a5de8bddce352e494b7b13e0dcbd1cb89316795f6549047212be058172dee67b13f37",
                "1b7976bed6fd7595dccafcebc24c97093ba19fbfb2e66caf3a225cb1b5e88af8f39a47c5ecc686c5d4f4f8149de75291d02c1ec1a91116eff667c728741348a5",
            ),
            (
                "00dd50a0b033e957a60e1848d36ff77d6446fdf87b82d5e0",
                "31eb072d179e767ec4177a7befa38944793f4fcae3c224dcb18a63906a6d3bd59c62937c7be82910ec45f368d1873bcc6291271b2b59b0cdc25d4fd10de76332",
            ),
            (
                "12b73a7d",
                "b31268bef14fef5f1d0709c0f20c8ba69929bcb0c6a1cca55b88e3b0085858f08b025f34ef6f5d6d5d1de0123b1fbe1683c263c9aa4d683dc42ec0df3853d4d5",
            ),
            (
                "7724c26ff0ff7397858f57bd28f2f41da597ae88",
                "d0712f5c4335ad18100c72a6647b98ff99114076c00580c597a6a0e6ae5ffffe7759f9db53f0fd24685b2b2fabfa0b466da40525dc8d94c90b1efb2413a35c62",
            ),
            (
                "f9c5cf1856958ddf36a074f86438b3160788bcb5a1b89f1f24be225de8249f485007e26d",
                "e0f7b321dba479c4a4b6f1a92018eefdb5941f7613791c09f7c5b5e36fb7303f984c11bd7cdb65581b2e75cb5d77defaea3727bac2b3e8506f79707f18034223",
            ),
            (
                "f1eeba5262f9a9a10d7356e5a1dd987f3bdd521f8100b1e398802d4771",
                "a05826bee76aa87b36e5a4b56996ba1ad3ed2bc2f400284c79f46dbd5957e486c5fca209547f7c743211f7d766d11f0ff4a73e96de769c6af820804fb0a5b430",
            ),
            (
                "58e6e4e642b510ba9cbabd82bafbb649",
                "532ba491bba0de91c1e3349096bfb23929bc014c1bdd367b1b8bc93c6880fdbe329e5387b75710f94fd21a3cfdf2f4f3c69575622424dc7b41465c59bdbfc3ab",
            ),
            (
                "6f35a812cb38d19f784eb18929b2ddaaec812e6483f6df824ee69777048ce08beda768340830025b1363b8988233e51838885e29f221f23a70be",
                "80247f9aa68ec676bc733075b2f4cdf02182ee11cfa62cc2240512fa379666dec3632cfb3d2d16c7e1bd69c2be78124bcc629171c9de4d1b6a6d9c3c697cf74f",
            ),
            (
                "42853f601ee66af8f1c47d926f509f9298407397ff5eedbfd46d",
                "f829d182f02c43ace55c5a1ce804d5f49f2d242fa7df88f7a2a9a46e605e9157da882d44c3c8f74f6433fffc26b9434934a78194317cb280742cf1b11ce314b2",
            ),
            (
                "34e10ef49da55f40d82aeb28d1aa8135674491a496aca2ffc13043f8f992e597d615cccec3fa91235578d8e63969e90869af38241325957f8504473b64",
                "71ef0c4e1e9691a6b3ba2190d2c290b19c39dcde532d5a9f912731d70f714d7826d3478194bbe5fd5b5e038d30878002c6c8cd0499a6a8543d0a472fc6f24a04",
            ),
            (
                "bd30b72d876fc7b91093e6d839105b77118f8b8916a79530eba0dc52d6ef",
                "305f2855c89b62132d80858fe84cddb5952d294f3a6e0dc32665858e90ed1c8b9fc8e252a7f841750717e478756c2b7284ad56f2629cc0834393625e19727292",
            ),
            (
                "d5923e985cc9f0d479f776b7e528b68be6c048a6e160dab70479a037af38eb608e",
                "43997d3d573955cbe1942ed04696dc09a701e79cea0a03b9df8b75bfd6b7fe855c2643a16fe8b786035fb6f7fc2a041bf4e8f2008f80a2d15c4d0d8b957051c4",
            ),
            (
                "4f60eef64370330590f89e6d44586717a968decdad73095f6324ce628b6e72ae719cf68d680e69",
                "5cc0933027a09682e8249f349df237341533541ed7c5e7998ac87cc41ca627fa1f37b3067e968e4681c135d734f7493694a1697fd41dfcad9f35d271d3b38969",
            ),
            (
                "564f0301b97a4b9ecfea089832ac207c2b66e55bceb60167dd352060818c3a67b5f94adcef843944caf08aac7eb43db9dcb712d2feba142c384f",
                "e858809529b53191fc300621f90b8e0d5f72a2ae42cc5c1c415dd71ba1c6c802617d3f607cf3a40b90b557f9b3dfa86ea771256d66699826322b0da4324c1f47",
            ),
            (
                "9493beff8e85651f626ecadda46b4557a758d233dd8dfa006d12ab9a9dc139d60c1335676d4f63da0e",
                "08efb6b6d3dc5a9d20a5539f8870896db6a6ba5d985c86baf15f7df84490fafd16c87183115bbc86ba3d485d8f36aed7cb25f62767c2f004a6e26866d0022eb4",
            ),
            (
                "1795966627aaa94f6ef970ea1929f0537fe2a2a6d384094d1f459cb34cf50caf38fa9260b8deec1e8f0374ed0737",
                "6443e7c96d6c36f4359ef2bdfa42577eac85cfa1ffbe398111dc4e6be192d4601bba5129cac9baa0c98806f8b14c08aa0147de370f421c19e9202fca323dac8f",
            ),
            (
                "2657fb4f10620ac41dab384abae38ef937c918be53d5d9ddb62e4f876a9e0d9b7410c4960b8b8b003dc2",
                "f3dba475a244e3cd59f26351ab0fcfe586114b254aaf3f909c1970017cf322bf4b86ad3d60b7b75027640a01d3ce1ad0a80b2207eb71fc594657defca1cd292f",
            ),
            (
                "d5ee780a2a1ecccab8",
                "01af0cbb60db40a71146d3f169cc794c89c651be00210ca89efcdb31fa662db68ae77473675b79e063d12d2fd2e2343ab383ce689aa4666ac5473bbe70c32601",
            ),
            (
                "154dfe97f71fc05af5f615b8996044b5fa7bc3373e18ce215dae135fadf4665db40e13658e3ad33206",
                "ef21ff6c4a92e0abb58fb8900e3b2c705fa5741b1da6a228d9eadb4a0c86341050c7f86003222871a6b2fc226b775f34ec3b81c14d6386216f4981defabad6ef",
            ),
            (
                "3a834ce0a918d42215da664ac12ff74cc4e802e62147b8b272b78a41a1138aa41ef38c8da6adbc5cfd628b5d02fea97e01c1a94c361923434e86",
                "bf2a9af18d0b444d29b74013f9b46f831d0d824d433306cb1d5e52f79339d2f6878502dd86d552518905bc2777bc880d2e61996925e03132d3e358dddfcf3f54",
            ),
            (
                "5d39ce3ed936f3ab8aac3cd87173821f82d1085e44de859d20a305101e6ca02f5819be376d9540cff80a629fad84a012e77d6a0727148661e78213",
                "7049a1747eab68fd31c208e00851a8b95d9ce69c667b7c251602477bf6b6dee7117a0269ab38311f46ecfcb3a9affb7a5eff854d3cebd5548501bec3e83ce82e",
            ),
            (
                "c6903781da7eee7aa9594eac0fe034d47ba332baf259ee8269af3d23a8b0389d18ed7773a9338be61bef2b62bc40d81748aa53d5982700aa3da2ab80aa238864",
                "acedfe4639c080cdb6ff1eb89921cabdc09200e25d34505661f6ab53e1c46ac07d8217337f8d5f1c4ba8248bdce11789b92d1cba4e513dd8f3bc0d01039c65de",
            ),
            (
                "29ee90442e280e",
                "3fe00f06410f3665869b36d02df37c5a152bb56a8225b3d146af5807d1a788e68d15c2834b861c48edd35ba20f8d51f706383c3f750ba1e3dd6c2f74eb466db0",
            ),
            (
                "8eeba92f2408301165830e23fba4da1c646bbabc6c44f4c26a21b236",
                "ce30423089904ac719b02581785e1adf41e98dc047c5a9154c71d594b90411b12b98f8e02cf1e4010784efdfa0f5a9475e2af1131566978c6aabe304c3314235",
            ),
            (
                "e381641adc6d6ecc47677a5aa348db6ae6b99dfef80e7e3a6f503fb441f95daf39e5e07b673b4042b3ac8a",
                "ee4632af10ecb21af8e96f5f2816c644d3e39e246b03e567cf4340e3bc6f130a2d0085b9f930418f019f8d2b4da09293e4f67af49e5f64f0387b0a81bf4e2319",
            ),
            (
                "79b35d641b55fa86b7d6c9",
                "27c1ba4e7dc5e596641e36d5d88f204e49cfa0f2b6270db921080dcb89419004fb2db683b6b6c604c16e846846ad1fcada78b848bd241845b6bc2008166179ed",
            ),
            (
                "faf7808e6941adac20b1e3226b8bb7d998a8a4ab9d6eb419af77799115e3846f47aaff100fe163195a270627f74ea81a",
                "2244ed2923165693a53c07444283cfe27b3c6fce69a26266cf0d21767575a482f38bdf0955285f9c0128801494c0d09051bbfe986929e43579c7afbcdefc8a19",
            ),
            (
                "283363a12f3003c254ac9db83936b5efe7b78b3e2f7bf8b558",
                "d7845d1a58c9282f24fe2d41243e94f9858739defc1b61f84a1f7725f09ff7301e73a768fa8f0f2f954edf124a3a5d25952024c55f44bfd79ef883a158569643",
            ),
            (
                "418a0ceadae884a2e6cbda3aa58022d33e1b798242f476a0b1d5",
                "9c97fd641262e79a164800b642b48e9bd6cdfe7d1ac486726bfc2eff235f5b3d0cf432762db2d4d67fb09c0d4c1717ac2d25788abada28332032513141292fab",
            ),
            (
                "ce5a077fb2",
                "93f53380d5c0aef98badbad8bd69764723315f381151cf9956b5e49231547983decd5a5e4addcee8f155d04d39a8d99d4941c03c6b2b6a84b7d1a01412691f0b",
            ),
            (
                "e1ef2287ce85ec518747dd979c62dec89ee7c0dc93ad2c79eb586f7005",
                "c13d881404403e50aef0ba4238131f4d98fb1a4759c948c0cdbbb91ed1948584b5390d134a5ab90791b979f85959f132fa06e31f27b4d4cb522b335aa45966ea",
            ),
            (
                "85203f82f5b36f904a7def6eb2ae3189d7fd9f7c28b41cc152a18ab711710cde1b7f75179a9cabafed7ae3",
                "205efad1ef7ddf1e5d9074dae96c83cf213a07648308db6ee1abc1b35a127d936691bd71626a7f11d630bd071805946e081232f0c1ffe015b24daf4d3448857c",
            ),
            (
                "7a3fe61f08ba93f39cb8118f3cbf56a0686ec66235028f6208cc162d5bf8a71bc98a1612c5be39fbac36cacb71a44a37dd8d5bffbf1d57",
                "68a957da27aedd7288ce78c767aaa00fbf0324ae64c7548781541d278c02b1f2048b8eaf25a8ad49d182596d9cc0b9d583dd71f04c158466d83732bb685d872c",
            ),
            (
                "d065b7466a0c400b952ff6b6895ab8856c7cdcfa5f219247f04fa900d39b795a65d945bb1715d8b9",
                "64747d7e0aad813d7e00d19c2fd4fb4f5fd763661cdfda06ec687d0ce23cc52190e24798bb464ff7e5912884694e8c29e9a60b85d516bb304db463d6bb5f2983",
            ),
            (
                "45269909e5b85c3a40fae33544f0b5c7e3ce4b869dc2a546744adbc130d4c8e0b2ce",
                "600adde3b3edc4802ec637d5159ce12fbe5e00f0101c3441bd9850d8df5e6a12ed9e4b093ac0db0ad2de8976d885cfae7d94e2b6fffcdde033d2163f1fad85c4",
            ),
            (
                "8cdcc5f4b9fc826bab381a76",
                "b218529216eea5a10c6c211cc4314bcbfd146f1db7a54c4f9b05a89ce3d6e488484fc755ea585604180a90304d2a7afa44c10f110a8a97cc8ec8f8d0e6e540cb",
            ),
            (
                "a0057472ef489452d993eccbe458dada846974060cc0426c024400bacfb586336bce15dc05d35ea7",
                "4cb35a662360b7cd9518b076c109efbf9a9e6343288d8dec021e400a60547c7f1d3b17cbd01bb1eba77fc2b126126f03cbf58f5d8016d01fe6c5c30034551507",
            ),
            (
                "ec2b8247f7ee799c0672f1be80",
                "06b48d7f0bcfee0b677b243e85742ba67a72d9b22f702fda0e35f82f41f106e6388f58b23dacdf1a83cf269342fdd058a72ad0296fc9682441f4452ae0537147",
            ),
            (
                "91cc2415fea3fcc4cfcf0253f07d87e557ca7e1b4b8dfbb761bea9fce3aecead306bfed04bd1a1ae6e43d20b89ad7d999673a15162866679",
                "47f5fcc4cf5ef843ef96f8bb3a35f8d48406682927d7c9373d83f671978fdf9547bed5eda9c79e4f0104b470348bfabb5b890e3f1c200daf1396e4d0c0492344",
            ),
            (
                "e2b312dfdc75c884b73129193cef9a7189c636f0bfa4ac4bb15d7d930cd6151612ea4e87333e969ca5cd786833e249c26dbe773d",
                "6a40a0b73c22b36c9f9d48648697b3f5dadff87a6c55e318f3adbb346df86be12d3208ab04ef708b37997e2104a9cc5747e9c79247c439cd27c8a362c56281c2",
            ),
            (
                "2695c8ef350aadcd74bea6ac49cfe5e596b99a53824a69d7850755809c",
                "474f1eff696dabf6c89321978de5a9f49d6155b79aed65260b30acfb058640a1059c11b8d56dcfcc99c54e00d3e779cc4df2e9d7afd0a89ca64b7e923fe08ce1",
            ),
            (
                "dc6bd47c5b28e1d03db86527f1acb5fae03357c7216f1365cdef2ef313486ba209b832e6a8650369",
                "d7797c48b0398072a25fa5094994d162e6f67856e883803015a8501cdb95b28a2dbc0344f83f428f0265be8efbe224bea4e4c7ec7856b431d7e781185bffcd92",
            ),
            (
                "4d",
                "ae5c7a2ff5cfa36bd46cc861d5d3b8466ef568026b7d4112e87849c05d8f06b16fa0cbdb80fed567fc2753af143c11debe3c29ea6df1876a51eaee481a620f7e",
            ),
            (
                "c1edde4e5ef6a835035f131c890c8b93dab002f551c128e16c7e38657cc118b64a53eee5d4903390424b88259c2fd85ad8add1bb8569e39215151936949964",
                "425b5e213435a0d208bc7204b57f01107d0da3cc3d7d3267dc9c94c99571adea2997fc189f14aa0ae29038fa38d2d47345fd01a66d0527c5873b78e0d7934f8a",
            ),
            (
                "5903dcddc945c2dc9efd54c72af3433b5ecf1d1c2a35aaa5f301ac8b72f1715d58",
                "d5c13ee5ca97b0275bb22ab0792b670aede31a7bc994185921cac2c4141f23fa937e9eae1ed6bd39d7c6b43da3b62c48bc450a11c8ead6727a66287eee793c95",
            ),
            (
                "4b97a288c5996ced9287e6ec738db7f7af92a60112bc422fdb713e7a45a53ac64d679fec2d2500bb8b755361343262a2ec09788954",
                "e7c9d3557065c5c29473fa0afd34f8579b3da860e77a5081899df7d3403ff94b16b57539d90f06a10d5c0f18f18ef94d678ec32a75ed6f5dca9e35b5d1ba79d4",
            ),
            (
                "8f0376c69d89a1a44f0aa61e1d7f2eee10fea17399f02164413c29af98baa054903e72d234c4b3d6619e2495788837ac2b13623b71",
                "656c7a4eda751771b3252cca4e39a927be45bd187f4e2da25ac8d7eb12494041bc23088be7c91849ff29161a4abf14fe0dec279b27e7ecefca95c9ff74faac0d",
            ),
            (
                "0c0b61d0c3fb443b42256276dd9c570be08fce72e6aa38b7bea0d10bf09f369b288b481c67a06a92d4288a09ee198706d4",
                "fdbf89d558058ba31ab6c70894d7979844a391bc09467c5b61ce76324eb26e20ef082e75e01e867f170b7c14fa9806f67ae697cec239469653a16c8ab28144e6",
            ),
            (
                "7504d7f51cefdbd4deea82ba4176d084b9ca310912ae78cd166113e6c71ec473ec23dda1f403fe773f3db27978bca90857d640e4",
                "5c3d3082fc4fce04f6b8d8ab0edccf30113539fdab708674456926c493efb861e15b953c160c130cb76c2765ac4b74729363bf2ea9d35fafbfd698330ee71cda",
            ),
            (
                "72",
                "fafa573a4caaa6d0367f122a3f40d74a61513cfa7bad05259ffe674eb7c10ae38b5e4271287c74b72d88f79019004acf95316d4142294db0b32cb69d1139f376",
            ),
            (
                "3791daaf15df92711b4eba88db31e34d39df0ce7d829209617df4a17598cf516",
                "f03f3ae2b65f71052a1ac6c5883625b4463322e0fb414453fb942c18f3fbd7ac8788e49839c044528e5f7fec21ed06b81aa4d0693ab52324a23a3b839d93dca7",
            ),
            (
                "c5a897fc99ece1469a7bbaac17e8914f",
                "09802d1a30430174fda68e4591bb73dae3f027da7847537eda6f9e1d630d4ca6bd16eb2388313cdf9932fdd55f6493ebfd1cd56577cea6e6722af1869bd861c7",
            ),
            (
                "b7045fe4ed07da15bca1b20789b8782471cacb58",
                "3f7e9c6c5724c47909b3f81131d5098ecb6b72ce4096aa23637c9b72e731c190be0a8a7b4450a4f368daf224c211557146fe10f3f011cd6004c8f6e94fd7e014",
            ),
            (
                "6c9d1060a452a41aadd8656c09d1b0fae368b8940ce54ac9e0cd760bf135058517e841c6cf9631f9fea900f719219994",
                "d4887698944e02da9fe4267c36b12310f17f1b5a4666047519d1023545aaeffe20727ddbae5bdf0694ba06d8e7aa24939ddccdc416074d8b7c1fa29935b01cdc",
            ),
            (
                "2d26",
                "7ff1db01ec19dd1e2159e81e953126bd1015fbee5560ce094319ceefd2443f96813eaa3be197800ce9471bc3d4212bc0a0c979d49cc9e4daa48c6304f751efcb",
            ),
            (
                "93d520b1d9054cc0083382ea9de793378f0372abeab2f0bf0e7fe02e3a4dee97bf4e10ee4f307b19bfa5a42f12",
                "bac20a0757464f189ca3250814676f54129dc3bed05cd300ee66a061524b1ec6003f7d1c3312f1d64e4b6ab3b5dc1c94b9627ff0dd001984110586d3b00279fb",
            ),
            (
                "fa6bc88e25044a1ab91432295dbae342f3aaba302f89340c744026ba4637814a25f9130383595bddb31714bfac3d95bca015d3",
                "3aea7bbeb1e93da4a9685f8b7b497246fa3581e1fd14baa878f9688b589a6e70b888252263cf98b13ab04ff0381706dd57ff9caf98b5f1ac7fbebdb44fa860d3",
            ),
            (
                "b81b0ba55b4c692453b22040d0bff6e5be071bf953e2ec6a521effe8",
                "0f27feaace4b8493f4ef1cea0dac2a5decea1e1d84d8b18fc7098bdb2adffabc32d31706a1b0e93bb2f510773619854b871dbab358ffa90cf8a065ef6c685294",
            ),
            (
                "3940d74e97f93329629830682351d3597f5bc430354bde726cc19b337c53bf8ac05c77d8",
                "8678cfdc7f554874da933de53d662be3b10278342ad69e0caf18c058371b1a6c188b44f540c6c9e3c7880279000177c8f6dadef58b65904a59fd32f5df8992dd",
            ),
            (
                "183212d42dde6cba7729d0101e7aafdb",
                "84dad517c6f860e79151f45935d256dd61382ab1a9dfa2fa066bf103cf4009a50deb115f04c9872d54af4ccef935edd401abdedc9fc024c923c47c72f12154af",
            ),
            (
                "7c99c17e",
                "fe3a5fcd2ddf763d7be485e408af31c17aac3512435960ec86a117abccbf4ea0248f4d8e513a700443ebc2ff23e09b3f6c61cb8591cc907263a093ee4b0a1b1e",
            ),
            (
                "8ae902bb4a35b46d2c",
                "eb864f93a0f3a60d2f08bb0785c063f1f42da37ce8045406faafbe536b97a0d183b7415935d84a5a52b650f93793507f43367f4e3d77e123e67d4d493395a73c",
            ),
            (
                "366f563e5d4e0f5fd2605eb1579d883bea3adfe8f20cc4d5",
                "f4618338bde4d2a6544a86bb0af3f6bc4cad103e989d17d8a5b4040d31b2bcff072505f1311f10dbd1d4487da3b99bb13f68a023eb00b40170dbd2cb91167742",
            ),
            (
                "53de42b12a3ca904695070d190ef70199deab755bfd175167cc778fd1729d3d4cba2d4fd8f2d886e1a",
                "3e1c8d264d2352a50c972e721090b8adda41a5608b42bb4225f5f11b9aca5e7a688b2ddd95b21904a950841994f86a4e36c74a9ceefcf4631dd0716789578603",
            ),
            (
                "a82188cc544ec3b53cdb8883e912a0c085b43687ad7f4464bf0981c9e7157cc91a0b2560935049fa46ac554810bf59936e8d0e9d4e174930e204",
                "d529bac9fe91751b209d3a31f95005baabbe2b9d7368f6e4f9454e46fd55e914266ee8a814db4f72ae843a850a4c05e75f2189c37cfa225457596a56cb344f09",
            ),
            (
                "ef767d90eca7f4962a490bca595817f76270fa22db781f7815ea433eed266e45f6e22bcde021eafb7a49cf47d19a7877547bedf63a118eda7b",
                "de7d457d38263752c608d89627b00d6de5acd357011a8bbc500bf96dd5a12d9f374c6462027c70eddaa3e114ca414999c14b572cafd9610535aa6b462dc660c8",
            ),
            (
                "9ffc10f5b5901f65a9a100a6515d7da1c4a176fe5faf25",
                "a6cdf39f8532085ea7261c7e24a3721f9c9c252716b62cb612e2da8eecd34d0329e18c1b4ef1c609752f346f72754b160def19b47fedad663bd359f51fd6898b",
            ),
            (
                "710b8daae084fd66ae863eb0a79292d46be86316b6341bf159a32b34dda73ac1dcb76595ad0a07af206ad50d69d742dfdbb67e60",
                "d237eda3b4f50e04479d96cfd847300d84437b6552b088d43a1a7dea97c9270a2b12be6f5d61137ccbf962babae8bd9b175ee15bbf9de781114f080747d3040a",
            ),
            (
                "e3fecffa30d378",
                "cd3d16f88643b56ee658aeb29a7224be9022c3abb18ea2edc7c6c3b786f454078ea17e9f4c4c0978dfd11f8fadb0cd70dfb042c5d1658a72880044e009145865",
            ),
            (
                "d2b8bf966b1a7d3706",
                "fe748740f1cbff9de7ecfa4276ccb741dc2418dc179c987c4cd5a0e05243ce5aedb1a6c426a7462944d8a49f5fc7b1c2f29b9b78704b88c0207cd7c3a20eb412",
            ),
            (
                "f6842e123c816a4191350e15bf4e99ea65b02f19990c783ef82e02d7605de0500e6ab2ecf4",
                "2f2189a31f00980ba8ffbadf2a17e0a4251eb588853a59330b0a6a388fb97938fcaee95148fbe159fda43eb78c47260a2fefb37fb60a8f225fcf27628d5c2b4c",
            ),
            (
                "4c6675c2ee1a12f3b4d6a3e496d5c244e3451b4f887e90",
                "2fb6046f017ecf6da343047320029854a81793df7a87bf7526f8d36a9b262e20c7e6fb07a8800313f8e65e0411cc7d92f2387442b476842860ff5a238b259109",
            ),
            (
                "ac101bbfb04166a2b1e5808b4b",
                "54280fb5b02f18bc9c840d918c2ee37fff4e24e03a79abfc8308c8453fe6e54180a805b11cad035d456e9a49c2d17395ceba3d664689f5ca3114c1ba7414a1dc",
            ),
            (
                "bcf508c48dc93438ba782574ec385130b92ad9833f406d2bb8de8302",
                "c0287c9199734845fb259b42f9943f9549190506ffc24237b24fc87cb31bfcbba10002d07e6c80f4c9a0f7d55ade23e4acb41ecaf55645c8510e461b817fe5bc",
            ),
            (
                "",
                "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce",
            ),
            (
                "d1a9a2a131b73ddb9a5f3984",
                "42b60ec8a60f673ca23fd28075ce11b9854a476f193083ae166964c4e3429092b0078f7bda066f472191dd39c0f2242d837449cbbbe234ef80be1fd002adaacf",
            ),
            (
                "86824dbb767ca35b87488896ebd0875e3db271c9ca9997705ffa1a435291fbfdd40162dd6c2e07fd117b0d07e170bd5a574a1b4a742d0f",
                "08d13923034938ea1173f7d40b39b7a05ba956a334cb530ae60a537bbaa43ce3abe8257de010391531b93d2e3885a8175771a256ae48a6bcb6bc4dc7f38dac05",
            ),
            (
                "bcbe4d67883d1f85cf7cad81c8bb",
                "961217b71ff7d94a0713f4f06ec9cf5f064a41bc54f9992268f8544b0c11e4b6d4ce6bf97fc62e97492e8e05f3b7f8e28e51737a060e2bf6d41794f18fc75802",
            ),
            (
                "dbc89751cfb03c4417c0feffe0f53f6fdd4680cd5dae7188645ddf12a39933ac0416b07627a7f2f1",
                "4fe17258f2f99e9b87b2d47e84cda915c4c566a2db8874d3dd16450202b4c77dc7e439983e5cd532399f1512fcf358cfe65de36ad45be39c9ce48652e6226177",
            ),
            (
                "cb7fd30ea3a6e7d164e9f68219c5ab94a619839214670e88c57557365c36eb480cc6cf9cb4c6628e35060cd1f207c5759033",
                "c435a14c97a13360705581990bbe47fa90834009a1a92fa184a7d7526983893fa9670b8ac4d9cb6b57b094bca57660a6c37b9702fdae0fe5adaf2f0c826e2f68",
            ),
            (
                "",
                "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce",
            ),
            (
                "e14bcf83f7ec85f6638a2ea74387d755f6a73d8ab936f3973579e1fd9a2fde9ade1ee55a98fe1d2c254b792583136a76c07a3a4e6b43c1eea1cf1f92d5c0",
                "5785e8b442f4cb98057bb5535b5335ac5b62e7a097996e2a66974f2eaedc3a8897da7424c395d7137ae740a29b8b0252471994ba6fafd6ab32c2569676837440",
            ),
            (
                "6c66",
                "33cdf6468faf43414722dc39d40965fbf871ae62749ae3ba8360606bf81efb62dfd185cb115fef2ba55e4b774bd1b57dee7459dde57035e76d940d3857b3b090",
            ),
            (
                "72625b1b69384962add9d6afe3fddc6824ad8f706a64bf02c6e602a38a58be1d9021c75000a871e4f10383eb5eaf9d2751fc543b3f5dcf5103ad67f6",
                "0c6806aadaad0593a628ef4632838ecf8c9543a27003710b634cbe8f392b27c6acd52b8ec086ce7c7e0d42338bf7ec841048e77461efd6c5ee30917bafd338a9",
            ),
            (
                "3466c6b8014c54e3cc3a33ddde635eb5b667f03b53597e80d5e028ce598609cf48c771",
                "69cafd16bfaae5b46020970268129b37f3a28acda3c30272a17ea1d5a8bf77861e1a13f5574db155700fc1144a6bd60e0e352c82c0e41577f3c5f9ac051c4f9a",
            ),
            (
                "2e67540d47370dec3d8851b3154f494837188c70de87eeb34dc04ce63e36b674b1c234a717f50d227722d154753dcdac2fcf2bcad44ef18e099a831e",
                "ac578aa9c9d088a767c3319cedaf7fa33490f8b33653b5fd7a8d6637ebe12c524268ba26d09b4719fc17c2bb4007912148693f8dee0de0423316cde3d94338d2",
            ),
            (
                "2ac01b2c0487597c23c530b39ab210170b18c401fbdcaef14bf6d3d384054ee6e08aedc3cdc3aa6f41df45b2bafda4de6ce56e29c9599a948b64a59702",
                "fc45b2571df27392907e1a1fdabbbc1511b03cf8aac4f6976e0837cea36f25816277e0bcc17f33eabe8d110ff48ebc943ff185c33908390a1c204c48a007bfba",
            ),
            (
                "8291e84c4d405a10f51d",
                "ffc1bea447c75f994dc9a3dc1d1c7fe4bf7cd078c48c39714712a61391ef6758f8b9d39169a75ad21c4b248363efbc936d9c7f307ed9b6d54d646d4ab089bd22",
            ),
            (
                "532523b95160827b7107b9cbcd4403f2c01e3657b7d9f3bcddb1d931e6db6458c85e1ac6dd082df00f27245e1bdb54ccb7664812c5",
                "d6dc8a9a94fd182f38722e4598358efe3da5fa995462742e045cc4227c192d17fa836ec19537c4a19a2a9c93e82a5d3c5e16ab7e1829acf9b8b630e543b8e1de",
            ),
            (
                "1adeda1ea58938399016d56067219bbeabf104b9d2",
                "b2cc3d3d87b35dffa72553a44f39a22e4374fbdc8baed39690f718f69bd93ec94df22f03f62fab1d033a890f7b268319aa2af3ab85b6d21287b54e6c27a6dea8",
            ),
            (
                "80eb171256ab9e54bafe968a829c358c632db9e6597f2102c617f4481cf632b236bd43909509919ea64cfb8cbdb3d7c0",
                "5ea39eff045dda34732921a443ac91f15845b614b562c839293395ec4db0c3c8975fe8f4a01579652e905113d7786c169be5560315fc23d7000fbc5b67a26650",
            ),
            (
                "276ee98bd73c48df887df85dfc1e5e20a7d1ac43add51f5105dbcf82cd873caa1fdd52ed79841aa62715cd8a9d4a8e04a1567c643067",
                "57b2125c21973543aa7c5915a8da3e90d117e71e0c24da10b6c1f2d195d8f4e4f41a5ffec07a2a671325158d58f112ea1d07248fa03c3c506c20d2ed9047d01d",
            ),
            (
                "bab5f8962c0aa3753ef0014695b00d72698d109dc2427367d210fc2293b2d393d91550f429f6179f5e02287dd967c0e6c0e047cae1",
                "4824fe1f035cb34f437380289effe4f0811f0f2d9ad89f70e5b49a3a7334242faff1df7e876751a0177655edd6bad26cde40333c796ca5fb0f18c5045ebd24e9",
            ),
            (
                "dabbb397e3fafc83903115ec3f0d5f",
                "c743cbfd570222a609a81a4815341119728df8272198741a4d88a3b9aaffc38d7f4018627e61b16441fe16974eb84604f5941ed4f81ec4c18adb5c8fa9fbde58",
            ),
            (
                "1842a64fbf1e3188fcb20c2b5bdbe7f41becdaea1d78b92fb5b7af3e2b",
                "7c5d21e951937ab2f92d995ef448a74dad49396a39fdb94def86744bfa7ae6ba2351bdd39c3796c664a1b8fc561be4004470feb4acea3ce8f2d671fc5a3983c6",
            ),
            (
                "fd1c69f5389bc31765a158c90e6d0f9ea19048256c14d50aae74f6696b05b72a0652d75aa2cfbcfa67659cfcdd18f9d3ffc997",
                "ef5c5fa591f0b158c9f62668957c0c1d15511ac676271df1be0d8e2f02f1d07f352c62e117f71c01f24f6d6815e627226eef08d75c49168fb7f8c22e3de73b75",
            ),
            (
                "a7d3add30b75c9992298df66da5e4cc0b836ee4374eb95790fc82829f7",
                "9b7c460faedf303c5a628730f9e85ba6e3a7346e3a4d5d0912bea27c7eae3f883d15a5f9a44ee73dbf67e92d1a8e33a9363d19d790637d6a74b12c0eaa1c04c2",
            ),
            (
                "c1a191ed422297293a49c9",
                "4472619cb7886c792e6db18fbd993d7b43cb7ac3e4dba7807a9025c40cf0f88678a6f82c97691a76e59cc69a1422064dbfd9999a431e57c4c5bdeda3f26c9d5a",
            ),
            (
                "5a03a80edd3fdd3c8d4243477ca4b30b4e9e764b0b3c01a1",
                "c549bc5c9a3ddf30399d5a543437bede92c3648a40b0a06fd6ef33ccb107cf5349e6009ac5bbda3b15f32110c1f14d3df06849ea74d736fed45703b97b2bf9a1",
            ),
            (
                "70ef80ab4fa30056d4031b7273264b7ea40359c606160d774717e8755d88",
                "8ee0cf58a84d460b19b5b946cf14f8256cc9b4dafd3721a3db020aab8593a394533bea1a6c38324b4303d4f0e906a55aafecba02cffd3c17937c006da18e60cc",
            ),
            (
                "30aaf53dbb17716f06eab025173fcb345e2ad36ed094a192e77af668f493e49b732519661a4acf0f38775878ebb53d07aa1662f05cfd91073a379f1e335171",
                "52f4522d307ddea78a1ae9ae27b2c26506149397aabd14d97b6ce47797d44e5c1076516bbe45e24f70c1397b957fe0725d96e2108e4b22ddd3aeeebbdd3adbbc",
            ),
            (
                "efcae94e48cfca76a02996f0f0df3076bfb050fbb209dd8b5ccbee",
                "75b71dbfa92043e7a5345280dc987d1d1a445375342db1e272929f5bc1607343c22b06ecf61a639df6a53de200051a1f541444783ee466e93736a187e0f6d5a5",
            ),
            (
                "f82f126e77e9b94514df1d74c9034cdb944f34623566eb6b0e3b5e36190c45cc2b",
                "a424dd7737b854b78b4e0efca1ac4b689f60e3a43a3f900d491f747133a788c3256c53cd479162e93965f5443a4a9378f4a0dee3d5c88fc8062d7995e1e1308b",
            ),
            (
                "0430efff4a506010f775730a2dd48a8e2690a6a1ecbcd20edc5ecdd48ce7d9c82a1897",
                "a8642c1b5de32da45f73585cfdc236c60b19d435b6debc4eebe3a149d7e3d87915a607872a7c959a6384a3d441edb45af58284bb97496572ddd54e3ddda2a961",
            ),
            (
                "1009af15862d47f202edcc4bc6a43eb07297fc74f3003133bd3869ad9f7a30cb12e2c48e041dd2954e73a28698cf015d6bce",
                "ccd0fd9920e90bdca57d81350fd03de1014f8d430f5d49f44ca161e38c445aadabc42961ba440a057e01985d325f887b092a7f84a2fed18cafd9446a50869afa",
            ),
            (
                "9015346faeae9ce56ceeb0b62f10523eb1",
                "54518705ca3c08eb8af54db15e76ffe21030c87876b6424bb4411cf8ee95900bab98ed1c0d18983827c39f7ad05254e81364a0d2aeb2406419259c87b5216ceb",
            ),
            (
                "3ea5d44088e86944b114193f90039548297c368e",
                "2ad439c41e6eaba88b903cadd8a8030898c67bad08fc48b261ca82f636d15a548f93af1c3d0faff85ffc56788f3cc13dbd2bb1d47a23e8520c7c49e4227d8c78",
            ),
            (
                "6be14e32d8932501a36ae8829ade841a2e6dc326",
                "54fee21b421f122640260fa730d011d6782091bf381610a433d889b22507a87d2a64e8476bc828afc6223902a944d7a309bce8de8991ae710c51f312dbb24b8b",
            ),
            (
                "cb7a59e2a978c5d7fb356a0089b28ad26afcb95cabd2a03cdf55f5d12bf7d69fa1f1e137e3783667310ea44df9fe",
                "72489e1e38bbd319162ddeb6a9b9dc8d90151e9379b9f59c21cec080d9a29f852c02a611bfdbdddbc363d314c85df136f205b8ae46a1ae207e4585c704bb92ec",
            ),
            (
                "",
                "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce",
            ),
            (
                "59393bddbeb958ded5f1beb967453c",
                "7efacaab682eb9473f5702c3f562a92c96ed44225886c14375fd4a3e4ffd46903d4c769ad937b4fa9d95644e60e557f2307db63e3a80cde3676d5bb12e9ef8a2",
            ),
            (
                "03574b",
                "93b1c48db2559a24d55e2ebda864c4f74e7f814d27bcf144668e46071393a9853148921c52c35c58e91d73a6434e214349e40c4bbcf7f1f50da49961eff770ca",
            ),
            (
                "b017b23305c5ec04c29f9f3d39fbce93b4d15f",
                "aa3f840950c924c9aa070cc0cb6da7f6a30b93ee5c9a08d66fdbeb67808264be47d8a6df88bb5dee80e1cca37a2ec15ee1581d525314bedffad1510b547c5ed0",
            ),
            (
                "e02f8a",
                "ad7204c5d5d58fb6b2781ce590bcb4a23551d35a403bc2a94183b8f3ef08d377e3238e11b6aa4b54b4b9ea64084f8829e2df5cf50860dfd929845c72480ad0ba",
            ),
            (
                "fe8c717ecacaa53773f373e39c3e89453a05c070e678dd514056d2212e3d7610",
                "046603247879516bff94b4b546e23565fa22521288cd166b82e3670544d21653b72cd569ad5492ff363b6dc0242d330423456218b1eef05bb7681f489ef2d467",
            ),
            (
                "fb711e94e15881594ff12ffb9accd25461d21628be2b097570827b8fca81d2",
                "bd54040ffe2323eef566081685aad7a0ec2b22de2fb68c7d615411df2d6136c33a32902c7478ea8887cebda82559ab04c0a8d7d44d804cee764228a7d4523459",
            ),
            (
                "158d5d6461c7fce9766fa2ebcf1674876648a147b4bfbc3807ebb8f28bef",
                "f892359ce5983171b40ee3264a24520d042c71adc3f352b686bd1ecba99a93a89d59e16fab50be0adcf176b90bead4129a2c1e23f844e78ad46106c6b8ee8329",
            ),
            (
                "6b8ac5c436c71544d954202e6184497880751ed0d52518d6dfcf6964cf4316266faa92664b5a4ef1240b",
                "c9910fd6dbcb11a2767c96afdb6ca0238e9df6e51de8248785a5da88b6b5a2bf2571668f81ad2495797fabdd277246a267c03cea93aa4e66004462284a3d5af6",
            ),
            (
                "91662f7ce4b4dd",
                "91c10c45beec8badd71dd6b446d03ed8a8b782f3c4c17b3e6376129c98c46bf1cffe358d4366e6e7b7aa4411ae26a33b0ee47d221a0275869ac4e53f23f62e04",
            ),
            (
                "4df36b626cc25a782d50076b966c85869142",
                "236571083a752bf4a8b74572b499334f0c1705acda228d50eab9fd929d4d3af523a93365e3ee7430f59a84dbd6bf3813c23f13655cf615775eb4d6df241171bb",
            ),
            (
                "d28785aa24d3a76269d611286b8b09843b7a70a2635a7a4c6e15a5163b665fbe7298fe33200932",
                "fc196cf63874e50ecf0ce5a64a230f878c773cccc6bab463482c0b5857764464af868b329b7fa10694fa774b3ece48c5e3365002ec94eeef0cde4066c1e0003f",
            ),
            (
                "a937b69b73e5fc4d6f5e45c8a4dd",
                "0af3954ae01087a1a5965859883cc30bbe39bb536eac42abf1245a7b24a5523cc0643b160776927884eb765969f25041b9606eac0f459bb17e56a7c2a0340654",
            ),
            (
                "2a6bc1b7ca341c958b515e02290e",
                "640c24dd11bc895af56e5ecbacf00daf5b5beb8002216e36d961340add0fd7939bcf9fd976d4f00f98792660a8da7ba123b7150a90c14b78e21b64c9d62b0130",
            ),
            (
                "a89eb3ba517254324636ecbf120bdabc324443ff20d3",
                "55c4428693c693947f8ff574ea3e7023f9400b2ad63dbca21765715b36ab50e92795d5188cf1661385c0ce2fc9280af212cb7204985792de81b58a78698519cc",
            ),
            (
                "9fa371a70f7210d146be61679f05d4f150b656009d09bede4fa7c088ef1d0b73974505058e7123e3a00e5f0407797afcba2ca5",
                "abe57fe29acd304e284f1cd1e3620b439a0d8835b23aeaac58f372c7211081e2f4d45041e023ec24178d92e55afda1a1c4b0251fcb0ead2ab346eb1f3e8bb2be",
            ),
            (
                "909566bd3de1c72338b2d246602946fd3d385df654e64ceb62b6ec2b66b34065a970fb0adc553227224a6bc7af9da4",
                "8eca04432053d532e2feafbba19d2fed32f2a137db331464e9fcd8bbbcf87ed24104dddcbd5654c9da0fabfb3d054dad037f86ae9f323553b9a9e5e94632a74e",
            ),
            (
                "9ebb027e4531ea547b31436b89154dbc1cdda9cbb5a1d3f37973197b719855256fa50b711c45f81fb56723c2ac657766d0d94fa7be2f90da845dd811f2a223",
                "5a012a516578f42020ef92f6ba0bae471892c1c179b00844faadcb35d7481b9f01b1115bb4e74cc0957afd88556160ec8a633572a32d733af10e6168c54590e0",
            ),
            (
                "582a3e95f2b21af25e41821a48709e8239f9294b5593d1e0c72d46e7",
                "6a62f9bb1f6b1117185ce8bb2534cb9bf7a272d75021d2434900da97b7a7e5e2ba51f3b6511425e29b9f821a885ddf90ef2e506f16ace282637a9bff8391ece8",
            ),
            (
                "fb831b939c823f30897ebf4d7d93dcb23d2615fb36",
                "37f4823d3fae6ee900d0bd8d99e85a43a5e10b7af7f04f073049ccd7b24affd8f109806f328f778094fd79de6a8320726bc379a28b174c4bc1d567efd7023f8e",
            ),
            (
                "c3baf0c1b8a2cb402920a2eba0b43a5738a56345a5506e16d314b26164af82ac31ba36afd844d993bcb830faffa61243",
                "6700b19da6c318d62380ec62690c0152601076e9735497d8411a59f15abe1a99889c12d4e7a2cf3fff1b1cef8d31d049f6b2ee669bcda79456f8a887b4f30094",
            ),
            (
                "d3d1503ee2d0a0774551a2baabfc667713e9e31a77b2126946b10a8237ad0427",
                "49f89250e58dc6446e248a1cf827ca53a44b44ea9f120891095dff4bd25d13b36283b859e1542f90d53c74c15cb94dfc01693e0700471ae9752ec8ee652c6aba",
            ),
            (
                "04d0d34f0a9c453f7fdc10f5d9ea9047916f9dc3a8d5ed80",
                "f488ab988d4e09ebe9c4d3544f5dfe0a3fbc837f4fac0dfb6047fcd859960f0f7d3fb2beca127a3bf71714361a8c04e0fc49840589557b397f97f1a70ffc3628",
            ),
            (
                "92a506914f87382133e365c4acf795ccc1d0ac531caf6eac9655f854132be144e668fd9ae79e8d45be5b2a6a1f47406a544b92b47677744d645b40c3d104",
                "d60a46aa6314ee85e36ba3d5bf916da2bfcc86f95490914761a21e89863eb849c679848c57cddc7bf54cdfca0ca0edaadc65f7ef2c604f28d7f923b8c1a7987a",
            ),
        ];

        for &(input_hex, expected) in test_cases {
            let input = decode_hex(input_hex);
            let mut h = Blake2b::new();
            h.update(&input);
            let digest = h.sum();
            assert_eq!(hex::encode(digest.as_ref()), expected, "mismatch for input \"{}\"", input_hex);
        }
    }
}
