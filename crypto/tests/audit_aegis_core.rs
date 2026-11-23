//! Tests for the AEGIS-256 / AEGIS-256X2 / AEGIS-256X4 core, backend dispatch
//! and software fallback (`crypto/src/aegis/{mod,aegis_core,aegis_soft}.rs`).
//!
//! The oracle is a fully independent scalar implementation of RFC 10032
//! written from the RFC text. Its AES round is byte-oriented and the S-box is
//! *computed* from the GF(2^8) inverse plus the FIPS 197 affine transform, so
//! it shares no code with the crate. It is first pinned against the RFC
//! Appendix A vectors (`reference_matches_rfc_vectors`), then used for the
//! differential sweeps.
//!
//! Everything else goes through the public API, which dispatches to the best
//! backend for the target. The portable `Soft` backend is exercised by the
//! in-crate unit tests in `aegis_core.rs`/`tests.rs`; this file compares the
//! selected public backend against the independent RFC reference.

use crypto::{
    Aead, AeadError, StreamCipher,
    aegis::{
        Aegis256, Aegis256Mac, Aegis256Stream, Aegis256X2, Aegis256X2Mac, Aegis256X2Stream, Aegis256X4, Aegis256X4Mac,
        Aegis256X4Stream,
    },
};
use rand::{RngExt, SeedableRng, rngs::StdRng};

// ── Small helpers ─────────────────────────────────────────────────────────────

fn pattern(len: usize, seed: u8) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(u64::from(seed));
    let mut out = vec![0u8; len];
    rng.fill(&mut out);
    out
}

fn xor16(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for i in 0..16 {
        o[i] = a[i] ^ b[i];
    }
    o
}

fn and16(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for i in 0..16 {
        o[i] = a[i] & b[i];
    }
    o
}

fn length_block(a_bits: u64, b_bits: u64) -> [u8; 16] {
    let mut out = [0u8; 16];
    out[..8].copy_from_slice(&a_bits.to_le_bytes());
    out[8..].copy_from_slice(&b_bits.to_le_bytes());
    out
}

// ── Independent RFC 10032 reference ──────────────────────────────────────────

const C0: [u8; 16] = [
    0x00, 0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0d, 0x15, 0x22, 0x37, 0x59, 0x90, 0xe9, 0x79, 0x62,
];
const C1: [u8; 16] = [
    0xdb, 0x3d, 0x18, 0x55, 0x6d, 0xc2, 0x2f, 0xf1, 0x20, 0x11, 0x31, 0x42, 0x73, 0xb5, 0x28, 0xdd,
];

fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

fn gf_inv(a: u8) -> u8 {
    let mut inv = 1u8;
    for _ in 0..254 {
        inv = gf_mul(inv, a);
    }
    inv
}

fn ref_sbox(x: u8) -> u8 {
    let inv = gf_inv(x);
    inv ^ inv.rotate_left(1) ^ inv.rotate_left(2) ^ inv.rotate_left(3) ^ inv.rotate_left(4) ^ 0x63
}

fn xtime(a: u8) -> u8 {
    (a << 1) ^ if a & 0x80 != 0 { 0x1b } else { 0 }
}

struct Reference {
    sbox: [u8; 256],
}

impl Reference {
    fn new() -> Self {
        let mut sbox = [0u8; 256];
        for (i, s) in sbox.iter_mut().enumerate() {
            *s = ref_sbox(i as u8);
        }
        Self {
            sbox,
        }
    }

    /// FIPS 197 round: SubBytes, ShiftRows, MixColumns, AddRoundKey (the
    /// ordering used by RFC 10032's `AESRound(in, rk)`).
    fn aes_round(&self, input: &[u8; 16], rk: &[u8; 16]) -> [u8; 16] {
        let mut sub = [0u8; 16];
        for i in 0..16 {
            sub[i] = self.sbox[input[i] as usize];
        }

        // State is column-major: byte `r + 4*c` is row r, column c.
        let mut shifted = [0u8; 16];
        for c in 0..4 {
            for r in 0..4 {
                shifted[c * 4 + r] = sub[((c + r) & 3) * 4 + r];
            }
        }

        let mut out = [0u8; 16];
        for c in 0..4 {
            let a0 = shifted[c * 4];
            let a1 = shifted[c * 4 + 1];
            let a2 = shifted[c * 4 + 2];
            let a3 = shifted[c * 4 + 3];
            out[c * 4] = xtime(a0) ^ (xtime(a1) ^ a1) ^ a2 ^ a3 ^ rk[c * 4];
            out[c * 4 + 1] = a0 ^ xtime(a1) ^ (xtime(a2) ^ a2) ^ a3 ^ rk[c * 4 + 1];
            out[c * 4 + 2] = a0 ^ a1 ^ xtime(a2) ^ (xtime(a3) ^ a3) ^ rk[c * 4 + 2];
            out[c * 4 + 3] = (xtime(a0) ^ a0) ^ a1 ^ a2 ^ xtime(a3) ^ rk[c * 4 + 3];
        }
        out
    }

    /// `Init(key, nonce)` for D parallel AEGIS-256 instances. The state is
    /// stored block-major, `v[block * d + lane]`, matching the crate's
    /// serialized layout.
    fn init(&self, key: &[u8; 32], nonce: &[u8; 32], d: usize) -> Vec<[u8; 16]> {
        let k0: [u8; 16] = key[..16].try_into().unwrap();
        let k1: [u8; 16] = key[16..].try_into().unwrap();
        let n0: [u8; 16] = nonce[..16].try_into().unwrap();
        let n1: [u8; 16] = nonce[16..].try_into().unwrap();

        let mut v = vec![[0u8; 16]; 6 * d];
        for lane in 0..d {
            v[lane] = xor16(&k0, &n0);
            v[d + lane] = xor16(&k1, &n1);
            v[2 * d + lane] = C1;
            v[3 * d + lane] = C0;
            v[4 * d + lane] = xor16(&k0, &C0);
            v[5 * d + lane] = xor16(&k1, &C1);
        }

        let mut k0v = vec![0u8; 16 * d];
        let mut k1v = vec![0u8; 16 * d];
        let mut kn0v = vec![0u8; 16 * d];
        let mut kn1v = vec![0u8; 16 * d];
        for lane in 0..d {
            k0v[lane * 16..(lane + 1) * 16].copy_from_slice(&k0);
            k1v[lane * 16..(lane + 1) * 16].copy_from_slice(&k1);
            kn0v[lane * 16..(lane + 1) * 16].copy_from_slice(&xor16(&k0, &n0));
            kn1v[lane * 16..(lane + 1) * 16].copy_from_slice(&xor16(&k1, &n1));
        }

        for _ in 0..4 {
            for (msg, _) in [(&k0v, 0), (&k1v, 1), (&kn0v, 2), (&kn1v, 3)] {
                self.xor_ctx(&mut v, d);
                self.update(&mut v, msg, d);
            }
        }
        v
    }

    fn xor_ctx(&self, v: &mut [[u8; 16]], d: usize) {
        for lane in 0..d {
            let mut ctx = [0u8; 16];
            ctx[0] = lane as u8;
            ctx[1] = (d - 1) as u8;
            v[3 * d + lane] = xor16(&v[3 * d + lane], &ctx);
            v[5 * d + lane] = xor16(&v[5 * d + lane], &ctx);
        }
    }

    fn update(&self, v: &mut [[u8; 16]], m: &[u8], d: usize) {
        assert_eq!(m.len(), 16 * d);
        let old = v.to_vec();
        for lane in 0..d {
            let ml: [u8; 16] = m[lane * 16..(lane + 1) * 16].try_into().unwrap();
            v[lane] = self.aes_round(&old[5 * d + lane], &xor16(&old[lane], &ml));
            for block in 1..6 {
                v[block * d + lane] = self.aes_round(&old[(block - 1) * d + lane], &old[block * d + lane]);
            }
        }
    }

    fn keystream(&self, v: &[[u8; 16]], d: usize) -> Vec<u8> {
        let mut z = vec![0u8; 16 * d];
        for lane in 0..d {
            let mut b = xor16(&xor16(&v[d + lane], &v[4 * d + lane]), &v[5 * d + lane]);
            b = xor16(&b, &and16(&v[2 * d + lane], &v[3 * d + lane]));
            z[lane * 16..(lane + 1) * 16].copy_from_slice(&b);
        }
        z
    }

    fn absorb(&self, v: &mut [[u8; 16]], ad: &[u8], d: usize) {
        let rate = 16 * d;
        let mut off = 0;
        while off + rate <= ad.len() {
            self.update(v, &ad[off..off + rate], d);
            off += rate;
        }
        if off < ad.len() {
            let mut buf = vec![0u8; rate];
            buf[..ad.len() - off].copy_from_slice(&ad[off..]);
            self.update(v, &buf, d);
        }
    }

    fn encrypt_msg(&self, v: &mut [[u8; 16]], msg: &[u8], d: usize) -> Vec<u8> {
        let rate = 16 * d;
        let mut out = Vec::with_capacity(msg.len());
        let mut off = 0;
        while off + rate <= msg.len() {
            let z = self.keystream(v, d);
            for i in 0..rate {
                out.push(msg[off + i] ^ z[i]);
            }
            self.update(v, &msg[off..off + rate], d);
            off += rate;
        }
        if off < msg.len() {
            let rem = msg.len() - off;
            let mut padded = vec![0u8; rate];
            padded[..rem].copy_from_slice(&msg[off..]);
            let z = self.keystream(v, d);
            for i in 0..rem {
                out.push(padded[i] ^ z[i]);
            }
            self.update(v, &padded, d);
        }
        out
    }

    /// RFC 10032 §5.5.7 Finalize.
    fn finalize(&self, v: &mut [[u8; 16]], ad_len: usize, msg_len: usize, d: usize, tag_len: usize) -> Vec<u8> {
        let u = length_block((ad_len as u64) * 8, (msg_len as u64) * 8);
        let mut t = vec![0u8; 16 * d];
        for lane in 0..d {
            t[lane * 16..(lane + 1) * 16].copy_from_slice(&xor16(&v[3 * d + lane], &u));
        }
        for _ in 0..7 {
            self.update(v, &t, d);
        }

        let mut a = [0u8; 16];
        let mut b = [0u8; 16];
        for lane in 0..d {
            for block in 0..3 {
                a = xor16(&a, &v[block * d + lane]);
            }
            for block in 3..6 {
                b = xor16(&b, &v[block * d + lane]);
            }
        }
        if tag_len == 16 {
            xor16(&a, &b).to_vec()
        } else {
            let mut tag = a.to_vec();
            tag.extend_from_slice(&b);
            tag
        }
    }

    fn aead(
        &self,
        key: &[u8; 32],
        nonce: &[u8; 32],
        ad: &[u8],
        msg: &[u8],
        d: usize,
        tag_len: usize,
    ) -> (Vec<u8>, Vec<u8>) {
        let mut v = self.init(key, nonce, d);
        self.absorb(&mut v, ad, d);
        let ct = self.encrypt_msg(&mut v, msg, d);
        let tag = self.finalize(&mut v, ad.len(), msg.len(), d, tag_len);
        (ct, tag)
    }

    /// RFC 10032 §7: the stream keystream is `Encrypt(Zeros(len), {})`.
    fn stream(&self, key: &[u8; 32], nonce: &[u8; 32], len: usize, d: usize) -> Vec<u8> {
        let zeros = vec![0u8; len];
        self.aead(key, nonce, &[], &zeros, d, 16).0
    }

    /// RFC 10032 §8.4.2 FinalizeMac.
    fn mac(&self, key: &[u8; 32], nonce: &[u8; 32], data: &[u8], d: usize, tag_len: usize) -> Vec<u8> {
        let mut v = self.init(key, nonce, d);
        self.absorb(&mut v, data, d);

        let tag_bits = (tag_len as u64) * 8;
        let u = length_block((data.len() as u64) * 8, tag_bits);
        let mut t = vec![0u8; 16 * d];
        for lane in 0..d {
            t[lane * 16..(lane + 1) * 16].copy_from_slice(&xor16(&v[3 * d + lane], &u));
        }
        for _ in 0..7 {
            self.update(&mut v, &t, d);
        }

        if d == 1 {
            let mut a = [0u8; 16];
            let mut b = [0u8; 16];
            for block in 0..3 {
                a = xor16(&a, &v[block * d]);
            }
            for block in 3..6 {
                b = xor16(&b, &v[block * d]);
            }
            return if tag_len == 16 {
                xor16(&a, &b).to_vec()
            } else {
                let mut out = a.to_vec();
                out.extend_from_slice(&b);
                out
            };
        }

        let mut tags = Vec::new();
        for lane in 1..d {
            if tag_len == 16 {
                let mut x = [0u8; 16];
                for block in 0..6 {
                    x = xor16(&x, &v[block * d + lane]);
                }
                tags.extend_from_slice(&x);
            } else {
                let mut x0 = [0u8; 16];
                let mut x1 = [0u8; 16];
                for block in 0..3 {
                    x0 = xor16(&x0, &v[block * d + lane]);
                }
                for block in 3..6 {
                    x1 = xor16(&x1, &v[block * d + lane]);
                }
                tags.extend_from_slice(&x0);
                tags.extend_from_slice(&x1);
            }
        }

        let rate = 16 * d;
        for piece in tags.chunks(16) {
            let mut buf = vec![0u8; rate];
            buf[..piece.len()].copy_from_slice(piece);
            self.update(&mut v, &buf, d);
        }

        let u2 = length_block(d as u64, tag_bits);
        let mut buf = vec![0u8; rate];
        buf[..16].copy_from_slice(&xor16(&v[3 * d], &u2));
        for _ in 0..7 {
            self.update(&mut v, &buf, d);
        }

        let mut a = [0u8; 16];
        let mut b = [0u8; 16];
        for block in 0..3 {
            a = xor16(&a, &v[block * d]);
        }
        for block in 3..6 {
            b = xor16(&b, &v[block * d]);
        }
        if tag_len == 16 {
            xor16(&a, &b).to_vec()
        } else {
            let mut out = a.to_vec();
            out.extend_from_slice(&b);
            out
        }
    }
}

// ── Public-API adapters (all three variants, both tag sizes) ──────────────────

trait AeadOps {
    fn enc(&self, msg: &mut [u8], nonce: &[u8; 32], ad: &[u8]) -> Vec<u8>;
    fn dec(&self, msg: &mut [u8], nonce: &[u8; 32], ad: &[u8], tag: &[u8]) -> Result<(), AeadError>;
}

macro_rules! impl_aead_ops {
    ($($t:ident),*) => {$(
        impl<const TAG: usize> AeadOps for $t<TAG> {
            fn enc(&self, msg: &mut [u8], nonce: &[u8; 32], ad: &[u8]) -> Vec<u8> {
                Aead::encrypt_in_place(self, msg, nonce, ad)
                    .expect("valid AEAD inputs")
                    .as_ref()
                    .to_vec()
            }
            fn dec(&self, msg: &mut [u8], nonce: &[u8; 32], ad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
                Aead::decrypt_in_place(self, msg, nonce, ad, tag)
            }
        }
    )*};
}
impl_aead_ops!(Aegis256, Aegis256X2, Aegis256X4);

trait StreamOps {
    fn fill(&mut self, out: &mut [u8]);
    fn xor(&mut self, out: &mut [u8]);
}

macro_rules! impl_stream_ops {
    ($($t:ident),*) => {$(
        impl StreamOps for $t {
            fn fill(&mut self, out: &mut [u8]) {
                self.fill_keystream(out)
            }
            fn xor(&mut self, out: &mut [u8]) {
                self.xor_keystream_in_place(out)
            }
        }
    )*};
}
impl_stream_ops!(Aegis256Stream, Aegis256X2Stream, Aegis256X4Stream);

trait MacOps {
    fn feed(&mut self, data: &[u8]);
    fn finish(self) -> Vec<u8>;
    fn verify_tag(self, tag: &[u8]) -> bool;
}

macro_rules! impl_mac_ops {
    ($($t:ident),*) => {$(
        impl<const TAG: usize> MacOps for $t<TAG> {
            fn feed(&mut self, data: &[u8]) {
                Self::update(self, data).expect("valid MAC input")
            }
            fn finish(self) -> Vec<u8> {
                Self::finalize(self).to_vec()
            }
            fn verify_tag(self, tag: &[u8]) -> bool {
                let tag: &[u8; TAG] = tag.try_into().unwrap();
                Self::verify(self, tag).is_ok()
            }
        }
    )*};
}
impl_mac_ops!(Aegis256Mac, Aegis256X2Mac, Aegis256X4Mac);

// ── RFC 10032 constants ───────────────────────────────────────────────────────

fn key_256() -> [u8; 32] {
    hex::decode_array(("1001000000000000000000000000000000000000000000000000000000000000").as_bytes()).unwrap()
}
fn nonce_256() -> [u8; 32] {
    hex::decode_array(("1000020000000000000000000000000000000000000000000000000000000000").as_bytes()).unwrap()
}
fn key_std() -> [u8; 32] {
    hex::decode_array(("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").as_bytes()).unwrap()
}
fn nonce_std() -> [u8; 32] {
    hex::decode_array(("101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f").as_bytes()).unwrap()
}
fn mac_data() -> Vec<u8> {
    hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122").unwrap()
}

// ── Reference self-check ─────────────────────────────────────────────────────

/// Pin the independent reference against RFC 10032 Appendix A.1, A.3, A.6,
/// A.7 and A.8 before trusting it as an oracle.
#[test]
fn reference_matches_rfc_vectors() {
    let r = Reference::new();
    assert_eq!(r.sbox[0x00], 0x63);
    assert_eq!(r.sbox[0x01], 0x7c);
    assert_eq!(r.sbox[0x53], 0xed);

    // A.1 AESRound test vector.
    let round = r.aes_round(
        &hex::decode_array(("000102030405060708090a0b0c0d0e0f").as_bytes()).unwrap(),
        &hex::decode_array(("101112131415161718191a1b1c1d1e1f").as_bytes()).unwrap(),
    );
    assert_eq!(round.to_vec(), hex::decode("7a7b4e5638782546a8c0477a3b813f43").unwrap());

    // A.3.2 (AEGIS-256, one block) and A.3.5 (partial block).
    let (ct, tag) = r.aead(
        &key_256(),
        &nonce_256(),
        &[],
        &hex::decode("00000000000000000000000000000000").unwrap(),
        1,
        16,
    );
    assert_eq!(ct, hex::decode("754fc3d8c973246dcc6d741412a4b236").unwrap());
    assert_eq!(tag, hex::decode("3fe91994768b332ed7f570a19ec5896e").unwrap());

    let (ct, tag) = r.aead(
        &key_256(),
        &nonce_256(),
        &hex::decode("0001020304050607").unwrap(),
        &hex::decode("000102030405060708090a0b0c0d").unwrap(),
        1,
        32,
    );
    assert_eq!(ct, hex::decode("f373079ed84b2709faee37358458").unwrap());
    assert_eq!(
        tag,
        hex::decode("8c1cc703c81281bee3f6d9966e14948b4a175b2efbdc31e61a98b4465235c2d9").unwrap()
    );

    // A.6.3 / A.7.3 (120-byte X2/X4 message, 128-bit tag).
    let ad = hex::decode("0102030401020304").unwrap();
    let msg = hex::decode(
        "040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607",
    ).unwrap();
    assert_eq!(msg.len(), 120);
    let (ct, tag) = r.aead(&key_std(), &nonce_std(), &ad, &msg, 2, 16);
    assert_eq!(
        ct,
        hex::decode(
            "72120c2ea8236180d67859001f4729077b7064c414384fe3a7b52f1571f4f8a7d0f01e18db4f3bc0adb150702e5d147a8d36522132761b994c1bd395589e2ccf0790dfe2a3d12d61cd666b2859827739db4037dd3124c78424459376f6cac08e1a7223a2a43e398ce6385cd654a19f481cba3b8f25910b42"
        ).unwrap()
    );
    assert_eq!(tag, hex::decode("635d391828520bf1512763f0c8f5cdbd").unwrap());

    let (ct, tag) = r.aead(&key_std(), &nonce_std(), &ad, &msg, 4, 16);
    assert_eq!(
        ct,
        hex::decode(
            "bfc2085b7e8017da99b0b6d646ae4d01f4ba8f2e7dfca1d759ae48a135139b9aaac6b4f5db810d426be1fdaff4e1454153a34b11da78ed7e418ee2ee9853042e95536aecbb694cea1b16a478eb0d4d1bf6509b1ce652a45af58e0e46ffccfa2d0426e702391d2ff5813808b81748a490dd656465fed61f09"
        ).unwrap()
    );
    assert_eq!(tag, hex::decode("b63b611b13975e2f3dc3cb6c2397bfcd").unwrap());

    // A.8.5 / A.8.6 AEGISMAC-256X2 / X4, both tag sizes.
    let data = mac_data();
    assert_eq!(
        r.mac(&key_256(), &nonce_256(), &data, 2, 16),
        hex::decode("fb319cb6dd728a764606fb14d37f2a5e").unwrap()
    );
    assert_eq!(
        r.mac(&key_256(), &nonce_256(), &data, 2, 32),
        hex::decode("0844b20ed5147ceae89c7a160263afd4b1382d6b154ecf560ce8a342cb6a8fd1").unwrap()
    );
    assert_eq!(
        r.mac(&key_256(), &nonce_256(), &data, 4, 16),
        hex::decode("a51f9bc5beae60cce77f0dbc60761edd").unwrap()
    );
    assert_eq!(
        r.mac(&key_256(), &nonce_256(), &data, 4, 32),
        hex::decode("b36a16ef07c36d75a91f437502f24f545b8dfa88648ed116943c29fead3bf10c").unwrap()
    );
}

// ── RFC 10032 known-answer vectors through the public API ────────────────────

#[test]
fn rfc10032_known_answers() {
    // AEGIS-256 Appendix A.3.2 .. A.3.6, both tag sizes.
    let vectors: &[(&str, &str, &str, &str, &str)] = &[
        (
            "",
            "00000000000000000000000000000000",
            "754fc3d8c973246dcc6d741412a4b236",
            "3fe91994768b332ed7f570a19ec5896e",
            "1181a1d18091082bf0266f66297d167d2e68b845f61a3b0527d31fc7b7b89f13",
        ),
        (
            "",
            "",
            "",
            "e3def978a0f054afd1e761d7553afba3",
            "6a348c930adbd654896e1666aad67de989ea75ebaa2b82fb588977b1ffec864a",
        ),
        (
            "0001020304050607",
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
            "f373079ed84b2709faee373584585d60accd191db310ef5d8b11833df9dec711",
            "8d86f91ee606e9ff26a01b64ccbdd91d",
            "b7d28d0c3c0ebd409fd22b44160503073a547412da0854bfb9723020dab8da1a",
        ),
        (
            "0001020304050607",
            "000102030405060708090a0b0c0d",
            "f373079ed84b2709faee37358458",
            "c60b9c2d33ceb058f96e6dd03c215652",
            "8c1cc703c81281bee3f6d9966e14948b4a175b2efbdc31e61a98b4465235c2d9",
        ),
        (
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20212223242526272829",
            "101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637",
            "57754a7d09963e7c787583a2e7b859bb24fa1e04d49fd550b2511a358e3bca252a9b1b8b30cc4a67",
            "ab8a7d53fd0e98d727accca94925e128",
            "a3aca270c006094d71c20e6910b5161c0826df233d08919a566ec2c05990f734",
        ),
    ];

    let key = key_256();
    let nonce = nonce_256();
    let c16 = Aegis256::<16>::new(&key);
    let c32 = Aegis256::<32>::new(&key);
    for (ad, msg, ct, tag16, tag32) in vectors {
        let ad = hex::decode(ad).unwrap();
        let msg = hex::decode(msg).unwrap();
        let ct = hex::decode(ct).unwrap();

        let mut buf = msg.clone();
        assert_eq!(c16.enc(&mut buf, &nonce, &ad), hex::decode(tag16).unwrap());
        assert_eq!(buf, ct);
        let mut dec = ct.clone();
        c16.dec(&mut dec, &nonce, &ad, &hex::decode(tag16).unwrap()).unwrap();
        assert_eq!(dec, msg);

        let mut buf = msg.clone();
        assert_eq!(c32.enc(&mut buf, &nonce, &ad), hex::decode(tag32).unwrap());
        assert_eq!(buf, ct);
        let mut dec = ct.clone();
        c32.dec(&mut dec, &nonce, &ad, &hex::decode(tag32).unwrap()).unwrap();
        assert_eq!(dec, msg);
    }

    // AEGIS-256X2 Appendix A.6.2/A.6.3 and AEGIS-256X4 A.7.2/A.7.3.
    let ad = hex::decode("0102030401020304").unwrap();
    let msg120 = hex::decode(
        "040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607040506070405060704050607",
    ).unwrap();
    assert_eq!(msg120.len(), 120);
    let x2 = Aegis256X2::<16>::new(&key_std());
    let x2_32 = Aegis256X2::<32>::new(&key_std());
    let x4 = Aegis256X4::<16>::new(&key_std());
    let x4_32 = Aegis256X4::<32>::new(&key_std());
    let x2_ct = hex::decode(
        "72120c2ea8236180d67859001f4729077b7064c414384fe3a7b52f1571f4f8a7d0f01e18db4f3bc0adb150702e5d147a8d36522132761b994c1bd395589e2ccf0790dfe2a3d12d61cd666b2859827739db4037dd3124c78424459376f6cac08e1a7223a2a43e398ce6385cd654a19f481cba3b8f25910b42",
    ).unwrap();
    let x4_ct = hex::decode(
        "bfc2085b7e8017da99b0b6d646ae4d01f4ba8f2e7dfca1d759ae48a135139b9aaac6b4f5db810d426be1fdaff4e1454153a34b11da78ed7e418ee2ee9853042e95536aecbb694cea1b16a478eb0d4d1bf6509b1ce652a45af58e0e46ffccfa2d0426e702391d2ff5813808b81748a490dd656465fed61f09",
    ).unwrap();

    let nonce = nonce_std();
    let mut buf = msg120.clone();
    assert_eq!(
        x2.enc(&mut buf, &nonce, &ad),
        hex::decode("635d391828520bf1512763f0c8f5cdbd").unwrap()
    );
    assert_eq!(buf, x2_ct);
    let mut buf = msg120.clone();
    assert_eq!(
        x2_32.enc(&mut buf, &nonce, &ad),
        hex::decode("b5668d3317159e9cc5d46e4803c3a76ad63bb42b3f47956d94f30db8cb366ad7").unwrap()
    );
    assert_eq!(buf, x2_ct);
    let mut dec = x2_ct.clone();
    x2.dec(&mut dec, &nonce, &ad, &hex::decode("635d391828520bf1512763f0c8f5cdbd").unwrap())
        .unwrap();
    assert_eq!(dec, msg120);

    let mut buf = msg120.clone();
    assert_eq!(
        x4.enc(&mut buf, &nonce, &ad),
        hex::decode("b63b611b13975e2f3dc3cb6c2397bfcd").unwrap()
    );
    assert_eq!(buf, x4_ct);
    let mut buf = msg120.clone();
    assert_eq!(
        x4_32.enc(&mut buf, &nonce, &ad),
        hex::decode("7847eace74409ee56c8f4cf63a9c2841ce7c8bd567d7c0ca514c879a190b978c").unwrap()
    );
    assert_eq!(buf, x4_ct);
    let mut dec = x4_ct.clone();
    x4.dec(&mut dec, &nonce, &ad, &hex::decode("b63b611b13975e2f3dc3cb6c2397bfcd").unwrap())
        .unwrap();
    assert_eq!(dec, msg120);

    // A.6.2/A.7.2 empty-message tags, both sizes.
    let mut empty = Vec::new();
    assert_eq!(
        x2.enc(&mut empty, &nonce, &[]),
        hex::decode("62cdbab084c83dacdb945bb446f049c8").unwrap()
    );
    let mut empty = Vec::new();
    assert_eq!(
        x2_32.enc(&mut empty, &nonce, &[]),
        hex::decode("25d7e799b49a80354c3f881ac2f1027f471a5d293052bd9997abd3ae84014bb7").unwrap()
    );
    let mut empty = Vec::new();
    assert_eq!(
        x4.enc(&mut empty, &nonce, &[]),
        hex::decode("3b7fee6cee7bf17888ad11ed2397beb4").unwrap()
    );
    let mut empty = Vec::new();
    assert_eq!(
        x4_32.enc(&mut empty, &nonce, &[]),
        hex::decode("6093a1a8aab20ec635dc1ca71745b01b5bec4fc444c9ffbebd710d4a34d20eaf").unwrap()
    );

    // A.8.4/A.8.5/A.8.6 MAC vectors.
    let data = mac_data();
    let comps: &[(&str, &str, &str)] = &[
        (
            "c08e20cfc56f27195a46c9cef5c162d4",
            "a5c906ede3d69545c11e20afa360b221f936e946ed2dba3d7c75ad6dc2784126",
            "256",
        ),
        (
            "fb319cb6dd728a764606fb14d37f2a5e",
            "0844b20ed5147ceae89c7a160263afd4b1382d6b154ecf560ce8a342cb6a8fd1",
            "x2",
        ),
        (
            "a51f9bc5beae60cce77f0dbc60761edd",
            "b36a16ef07c36d75a91f437502f24f545b8dfa88648ed116943c29fead3bf10c",
            "x4",
        ),
    ];
    for (tag16, tag32, which) in comps {
        let (got16, got32): (Vec<u8>, Vec<u8>) = match *which {
            "256" => {
                let mut m = Aegis256Mac::<16>::new_with_nonce(&key, &nonce_256());
                m.update(&data).unwrap();
                let t16 = m.finalize().to_vec();
                let mut m = Aegis256Mac::<32>::new_with_nonce(&key, &nonce_256());
                m.update(&data).unwrap();
                (t16, m.finalize().to_vec())
            }
            "x2" => {
                let mut m = Aegis256X2Mac::<16>::new_with_nonce(&key, &nonce_256());
                m.update(&data).unwrap();
                let t16 = m.finalize().to_vec();
                let mut m = Aegis256X2Mac::<32>::new_with_nonce(&key, &nonce_256());
                m.update(&data).unwrap();
                (t16, m.finalize().to_vec())
            }
            _ => {
                let mut m = Aegis256X4Mac::<16>::new_with_nonce(&key, &nonce_256());
                m.update(&data).unwrap();
                let t16 = m.finalize().to_vec();
                let mut m = Aegis256X4Mac::<32>::new_with_nonce(&key, &nonce_256());
                m.update(&data).unwrap();
                (t16, m.finalize().to_vec())
            }
        };
        assert_eq!(got16, hex::decode(tag16).unwrap(), "MAC-{which} tag128");
        assert_eq!(got32, hex::decode(tag32).unwrap(), "MAC-{which} tag256");
    }
}

/// A.6.1 / A.7.1 initial states via the first keystream block
/// (`z = S1 ^ S4 ^ S5 ^ (S2 & S3)` per lane).
#[test]
fn parallel_initial_state_lane_layout() {
    let x2_state: [[u8; 16]; 6] = [
        hex::decode_array(("eca2bf4538442e8712d4972595744039").as_bytes()).unwrap(),
        hex::decode_array(("3e536a998799408a97f3479a6f779d48").as_bytes()).unwrap(),
        hex::decode_array(("cf8c63d6f2b4563cdd9231107c85950e").as_bytes()).unwrap(),
        hex::decode_array(("d7e0707e6bfbbad913bc94b6993a9fa0").as_bytes()).unwrap(),
        hex::decode_array(("a373cf6d537dd66bc0ef0f2f9285359f").as_bytes()).unwrap(),
        hex::decode_array(("9f76560dcae1efacabdcce446ae283bc").as_bytes()).unwrap(),
    ];
    let x2_lane1: [[u8; 16]; 6] = [
        hex::decode_array(("201405efa9264f07911db58101903087").as_bytes()).unwrap(),
        hex::decode_array(("0d79a7d822a5d215f78c3bf2feb33ae1").as_bytes()).unwrap(),
        hex::decode_array(("78d17ed7d8d563ff11bd202c76864839").as_bytes()).unwrap(),
        hex::decode_array(("097e4b1bff40d4c19cb29dfd125d62f2").as_bytes()).unwrap(),
        hex::decode_array(("c0d0ae0c48f9df3faaf0e7be7768c326").as_bytes()).unwrap(),
        hex::decode_array(("bd52a6b9c8f976a26ec1409df19e8bfe").as_bytes()).unwrap(),
    ];
    let mut want = Vec::new();
    for state in [&x2_state, &x2_lane1] {
        let z = xor16(&xor16(&state[1], &state[4]), &xor16(&state[5], &and16(&state[2], &state[3])));
        want.extend_from_slice(&z);
    }
    let mut got = vec![0u8; 32];
    Aegis256X2Stream::new(&key_std(), &nonce_std()).fill_keystream(&mut got);
    assert_eq!(got, want, "AEGIS-256X2 initial keystream");

    let x4_states: [[[u8; 16]; 6]; 4] = [
        [
            hex::decode_array(("482a86e8436cd2361063a4b2702769b9").as_bytes()).unwrap(),
            hex::decode_array(("e705e8866267717d96092e58e78b574c").as_bytes()).unwrap(),
            hex::decode_array(("e1aa6af5d9e31dde8d94a48a0810fa89").as_bytes()).unwrap(),
            hex::decode_array(("37f06808410c8f3954525ae44584d3be").as_bytes()).unwrap(),
            hex::decode_array(("d1c9eb37fecbc5ada7b351fa4f501f32").as_bytes()).unwrap(),
            hex::decode_array(("4f6110c2ef0c3c687e90c1e5532ddf8e").as_bytes()).unwrap(),
        ],
        [
            hex::decode_array(("d95a2be81c9245b22996f68eea0122f9").as_bytes()).unwrap(),
            hex::decode_array(("d1dd412142df9806cc267af2fe1d830e").as_bytes()).unwrap(),
            hex::decode_array(("63555cdf0d98f18fb75b029ad80786c0").as_bytes()).unwrap(),
            hex::decode_array(("8fcc23bca2fe2209f93d34e2da35b33d").as_bytes()).unwrap(),
            hex::decode_array(("0b9b803283c1538628b507c8f6432434").as_bytes()).unwrap(),
            hex::decode_array(("031bd85d99f64684d23728a0453c72a1").as_bytes()).unwrap(),
        ],
        [
            hex::decode_array(("0c2a3b348b1a5e256c6751377318c41e").as_bytes()).unwrap(),
            hex::decode_array(("30e7dfd3c9941b8394e95bdf5bac99d9").as_bytes()).unwrap(),
            hex::decode_array(("a3ee0e4a3429a9539e4fcec385475608").as_bytes()).unwrap(),
            hex::decode_array(("33156347df89eaa69ab11096362daccf").as_bytes()).unwrap(),
            hex::decode_array(("bfb8b6d4f87cce28825c7e92f54b8728").as_bytes()).unwrap(),
            hex::decode_array(("10bc7ec34d4119b5bdeb6c7dfc458247").as_bytes()).unwrap(),
        ],
        [
            hex::decode_array(("f64436a21653fe7cf2e0829a177db383").as_bytes()).unwrap(),
            hex::decode_array(("9f27186f8a4fab86820689822c3c74d2").as_bytes()).unwrap(),
            hex::decode_array(("28ea527d31ef61df498dc107fe02df99").as_bytes()).unwrap(),
            hex::decode_array(("bbe58d9dbe8c5b0469be5a87086db5d4").as_bytes()).unwrap(),
            hex::decode_array(("8917bb5b09c32f900c6a5a1d63c46264").as_bytes()).unwrap(),
            hex::decode_array(("591ece530aeaa5c9867220156f5c25e3").as_bytes()).unwrap(),
        ],
    ];
    let mut want = Vec::new();
    for state in &x4_states {
        let z = xor16(&xor16(&state[1], &state[4]), &xor16(&state[5], &and16(&state[2], &state[3])));
        want.extend_from_slice(&z);
    }
    let mut got = vec![0u8; 64];
    Aegis256X4Stream::new(&key_std(), &nonce_std()).fill_keystream(&mut got);
    assert_eq!(got, want, "AEGIS-256X4 initial keystream");
}

// ── Differential sweeps ─────────────────────────────────────────────────────

fn msg_lengths() -> Vec<usize> {
    let mut v: Vec<usize> = (0..=64).collect();
    v.extend([
        65, 66, 95, 96, 97, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 2047, 2048, 2049,
    ]);
    v
}

fn aad_lengths() -> [usize; 8] {
    [0, 1, 15, 16, 17, 127, 128, 129]
}

fn aead_sweep<A: AeadOps>(r: &Reference, cipher: &A, d: usize, tag_len: usize) {
    let key = [0x42u8; 32];
    let nonce = [0x24u8; 32];
    for len in msg_lengths() {
        let plain = pattern(len, 0x11);
        for &ad_len in &aad_lengths() {
            let ad = pattern(ad_len, 0x33);
            let (want_ct, want_tag) = r.aead(&key, &nonce, &ad, &plain, d, tag_len);

            let mut got = plain.clone();
            let tag = cipher.enc(&mut got, &nonce, &ad);
            assert_eq!(got, want_ct, "ct d={d} tag={tag_len} len={len} ad={ad_len}");
            assert_eq!(tag, want_tag, "tag d={d} tag={tag_len} len={len} ad={ad_len}");

            let mut dec = want_ct.clone();
            cipher
                .dec(&mut dec, &nonce, &ad, &want_tag)
                .unwrap_or_else(|e| panic!("decrypt d={d} tag={tag_len} len={len} ad={ad_len}: {e:?}"));
            assert_eq!(dec, plain, "plaintext d={d} tag={tag_len} len={len} ad={ad_len}");
        }
    }
}

#[test]
fn differential_public_vs_reference() {
    let r = Reference::new();
    aead_sweep(&r, &Aegis256::<16>::new(&[0x42; 32]), 1, 16);
    aead_sweep(&r, &Aegis256::<32>::new(&[0x42; 32]), 1, 32);
    aead_sweep(&r, &Aegis256X2::<16>::new(&[0x42; 32]), 2, 16);
    aead_sweep(&r, &Aegis256X2::<32>::new(&[0x42; 32]), 2, 32);
    aead_sweep(&r, &Aegis256X4::<16>::new(&[0x42; 32]), 4, 16);
    aead_sweep(&r, &Aegis256X4::<32>::new(&[0x42; 32]), 4, 32);
}

/// RFC 10032 §5.5.7: the 256-bit tag is
/// `(S0^S1^S2) || (S3^S4^S5)`, while the 128-bit tag is
/// `(S0^S1^S2) ^ (S3^S4^S5)`. The ciphertext must not depend on tag length.
#[test]
fn tag_sizes_share_ciphertext_and_compose() {
    let key = [0x99u8; 32];
    let nonce = [0x17u8; 32];
    let mut lens: Vec<usize> = (0..=40).collect();
    lens.extend([63, 64, 65, 127, 128, 129, 1023, 1024, 1025]);
    for &len in &lens {
        let ad = pattern(31, 0x55);
        let plain = pattern(len, 0x77);
        for d in [1usize, 2, 4] {
            let (ct16, tag16, ct32, tag32) = match d {
                1 => {
                    let c16 = Aegis256::<16>::new(&key);
                    let c32 = Aegis256::<32>::new(&key);
                    let mut a = plain.clone();
                    let t16 = c16.enc(&mut a, &nonce, &ad);
                    let mut b = plain.clone();
                    let t32 = c32.enc(&mut b, &nonce, &ad);
                    (a, t16, b, t32)
                }
                2 => {
                    let c16 = Aegis256X2::<16>::new(&key);
                    let c32 = Aegis256X2::<32>::new(&key);
                    let mut a = plain.clone();
                    let t16 = c16.enc(&mut a, &nonce, &ad);
                    let mut b = plain.clone();
                    let t32 = c32.enc(&mut b, &nonce, &ad);
                    (a, t16, b, t32)
                }
                _ => {
                    let c16 = Aegis256X4::<16>::new(&key);
                    let c32 = Aegis256X4::<32>::new(&key);
                    let mut a = plain.clone();
                    let t16 = c16.enc(&mut a, &nonce, &ad);
                    let mut b = plain.clone();
                    let t32 = c32.enc(&mut b, &nonce, &ad);
                    (a, t16, b, t32)
                }
            };
            assert_eq!(ct16, ct32, "ciphertext depends on tag size (d={d} len={len})");
            let mut folded = [0u8; 16];
            for i in 0..16 {
                folded[i] = tag32[i] ^ tag32[16 + i];
            }
            assert_eq!(folded, tag16[..], "tag128 != tag256[..16] ^ tag256[16..] (d={d})");
        }
    }
}

// ── In-place / canary behaviour ─────────────────────────────────────────────

fn in_place_sweep<A: AeadOps>(r: &Reference, cipher: &A, d: usize, tag_len: usize) {
    let key = [0x42u8; 32];
    let nonce = [0x24u8; 32];
    for len in [0usize, 1, 15, 16, 17, 63, 64, 65, 1023, 1024, 1025] {
        let ad = pattern(17, 0x33);
        let plain = pattern(len, 0x11);
        for off in [0usize, 1, 3, 7, 15, 16, 17] {
            let mut buf = vec![0xA5u8; off + len + 13];
            buf[off..off + len].copy_from_slice(&plain);
            let tag = cipher.enc(&mut buf[off..off + len], &nonce, &ad);
            assert!(buf[..off].iter().all(|&b| b == 0xA5), "prefix canary d={d} off={off}");
            assert!(
                buf[off + len..].iter().all(|&b| b == 0xA5),
                "suffix canary after encrypt d={d} off={off} len={len}"
            );
            let (want, want_tag) = r.aead(&key, &nonce, &ad, &plain, d, tag_len);
            assert_eq!(&buf[off..off + len], &want[..], "ct d={d} off={off} len={len}");
            assert_eq!(tag, want_tag, "tag d={d} off={off} len={len}");

            cipher
                .dec(&mut buf[off..off + len], &nonce, &ad, &tag)
                .unwrap_or_else(|e| panic!("roundtrip d={d} off={off} len={len}: {e:?}"));
            assert_eq!(&buf[off..off + len], &plain[..]);
            assert!(buf[..off].iter().all(|&b| b == 0xA5), "prefix canary after decrypt");
            assert!(
                buf[off + len..].iter().all(|&b| b == 0xA5),
                "suffix canary after decrypt d={d} off={off} len={len}"
            );
        }
    }
}

#[test]
fn in_place_offsets_and_canaries() {
    let r = Reference::new();
    in_place_sweep(&r, &Aegis256::<16>::new(&[0x42; 32]), 1, 16);
    in_place_sweep(&r, &Aegis256::<32>::new(&[0x42; 32]), 1, 32);
    in_place_sweep(&r, &Aegis256X2::<16>::new(&[0x42; 32]), 2, 16);
    in_place_sweep(&r, &Aegis256X2::<32>::new(&[0x42; 32]), 2, 32);
    in_place_sweep(&r, &Aegis256X4::<16>::new(&[0x42; 32]), 4, 16);
    in_place_sweep(&r, &Aegis256X4::<32>::new(&[0x42; 32]), 4, 32);
}

// ── Tamper rejection and zeroization ────────────────────────────────────────

fn tamper_sweep<A: AeadOps>(cipher: &A, d: usize, tag_len: usize) {
    let nonce = [0x24u8; 32];
    for len in [0usize, 1, 16, 17, 64, 1024] {
        let ad = pattern(17, 0x33);
        let plain = pattern(len, 0x11);
        let mut ct = plain.clone();
        let tag = cipher.enc(&mut ct, &nonce, &ad);
        assert_eq!(ct.len(), len);

        let mut cases: Vec<(Vec<u8>, [u8; 32], Vec<u8>, Vec<u8>)> = Vec::new();
        for i in 0..tag_len {
            let mut t = tag.clone();
            t[i] ^= 1 << (i % 8);
            cases.push((ct.clone(), nonce, ad.clone(), t));
        }
        if len != 0 {
            let mut c = ct.clone();
            c[0] ^= 1;
            cases.push((c, nonce, ad.clone(), tag.clone()));
            let mut c = ct.clone();
            c[len - 1] ^= 1;
            cases.push((c, nonce, ad.clone(), tag.clone()));
        }
        let mut a = ad.clone();
        if let Some(first) = a.first_mut() {
            *first ^= 1;
        } else {
            a.push(0);
        }
        cases.push((ct.clone(), nonce, a, tag.clone()));
        let mut n = nonce;
        n[0] ^= 1;
        cases.push((ct.clone(), n, ad.clone(), tag.clone()));

        for (mut bad_ct, case_nonce, case_ad, case_tag) in cases {
            let mut framed = vec![0xC3u8; 3 + len + 5];
            framed[3..3 + len].copy_from_slice(&bad_ct);
            let res = cipher.dec(&mut framed[3..3 + len], &case_nonce, &case_ad, &case_tag);
            assert_eq!(
                res,
                Err(AeadError::InvalidCiphertext),
                "tampered input accepted d={d} tag={tag_len} len={len}"
            );
            assert!(
                framed[3..3 + len].iter().all(|&b| b == 0),
                "failed decrypt did not zero plaintext d={d} tag={tag_len} len={len}"
            );
            assert!(framed[..3].iter().all(|&b| b == 0xC3), "zeroing crossed the prefix canary");
            assert!(
                framed[3 + len..].iter().all(|&b| b == 0xC3),
                "zeroing crossed the suffix canary d={d} tag={tag_len} len={len}"
            );
            bad_ct.clear();
        }
    }
}

#[test]
fn tamper_rejection_and_zeroization() {
    tamper_sweep(&Aegis256::<16>::new(&[0x42; 32]), 1, 16);
    tamper_sweep(&Aegis256::<32>::new(&[0x42; 32]), 1, 32);
    tamper_sweep(&Aegis256X2::<16>::new(&[0x42; 32]), 2, 16);
    tamper_sweep(&Aegis256X2::<32>::new(&[0x42; 32]), 2, 32);
    tamper_sweep(&Aegis256X4::<16>::new(&[0x42; 32]), 4, 16);
    tamper_sweep(&Aegis256X4::<32>::new(&[0x42; 32]), 4, 32);
}

// ── Wrong nonce / tag lengths are rejected before any write ─────────────────

fn wrong_length_sweep<A: AeadOps + Aead>(cipher: &A, tag_len: usize) {
    let nonce = [0x24u8; 32];
    let ad = b"aad";
    let plain = pattern(37, 0x11);
    let mut ct = plain.clone();
    let tag = cipher.enc(&mut ct, &nonce, ad);

    // Wrong nonce length: rejected before decryption, buffer untouched.
    for bad_nonce_len in [0usize, 1, 16, 31, 33, 64] {
        let bad_nonce = vec![0x5Au8; bad_nonce_len];
        let mut buf = ct.clone();
        let before = buf.clone();
        let res = cipher.decrypt_in_place(&mut buf, &bad_nonce, ad, &tag);
        assert_eq!(res, Err(AeadError::InvalidNonce), "nonce len {bad_nonce_len}");
        assert_eq!(buf, before, "buffer written on bad nonce length {}", bad_nonce_len);
    }

    // Wrong tag length: rejected before decryption, buffer untouched.
    for bad_tag_len in [0usize, 1, 15, 16, 17, 31, 32, 33] {
        if bad_tag_len == tag_len {
            continue;
        }
        let bad_tag = vec![0x77u8; bad_tag_len];
        let mut buf = ct.clone();
        let before = buf.clone();
        let res = cipher.decrypt_in_place(&mut buf, &nonce, ad, &bad_tag);
        assert_eq!(res, Err(AeadError::InvalidCiphertext), "tag len {bad_tag_len}");
        assert_eq!(buf, before, "buffer written on bad tag length {bad_tag_len}");
    }

    // Encrypt with a wrong nonce returns an error before touching the buffer.
    let mut buf = plain.clone();
    let before = buf.clone();
    assert_eq!(
        cipher.encrypt_in_place(&mut buf, &nonce[..31], ad).err(),
        Some(AeadError::InvalidNonce)
    );
    assert_eq!(buf, before, "buffer written on a nonce-length error");

    // `Aead::decrypt` (allocating) rejects a ciphertext shorter than the tag.
    let short = vec![0u8; tag_len.saturating_sub(1)];
    assert!(cipher.decrypt(&short, &nonce, ad).is_err(), "short input accepted");

    // Valid input still works after all the rejected attempts.
    let mut buf = ct.clone();
    cipher
        .decrypt_in_place(&mut buf, &nonce, ad, &tag)
        .expect("valid decrypt after rejected attempts");
    assert_eq!(buf, plain);
}

#[test]
fn wrong_lengths_rejected_before_writes() {
    wrong_length_sweep(&Aegis256::<16>::new(&[0x42; 32]), 16);
    wrong_length_sweep(&Aegis256::<32>::new(&[0x42; 32]), 32);
    wrong_length_sweep(&Aegis256X2::<16>::new(&[0x42; 32]), 16);
    wrong_length_sweep(&Aegis256X2::<32>::new(&[0x42; 32]), 32);
    wrong_length_sweep(&Aegis256X4::<16>::new(&[0x42; 32]), 16);
    wrong_length_sweep(&Aegis256X4::<32>::new(&[0x42; 32]), 32);
}

// ── Stream cipher ───────────────────────────────────────────────────────────

fn stream_sweep<S: StreamOps>(r: &Reference, d: usize, new: impl Fn() -> S) {
    let key = [0x42u8; 32];
    let nonce = [0x24u8; 32];

    let mut lens: Vec<usize> = (0..=80).collect();
    lens.extend([127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 2047, 2048, 2049]);

    for &len in &lens {
        let want = r.stream(&key, &nonce, len, d);

        let mut got = vec![0u8; len];
        new().fill(&mut got);
        assert_eq!(got, want, "fill d={d} len={len}");

        let mut got = vec![0xFFu8; len];
        new().fill(&mut got);
        assert_eq!(got, want, "fill must overwrite d={d} len={len}");

        let mut got = vec![0u8; len];
        new().xor(&mut got);
        assert_eq!(got, want, "xor over zeros d={d} len={len}");

        let mut got = want.clone();
        new().xor(&mut got);
        assert!(got.iter().all(|&b| b == 0), "xor cancels d={d} len={len}");
    }

    // Canary around the output region.
    for &len in &[0usize, 1, 15, 16, 17, 63, 64, 65, 1023, 1024, 1025] {
        let want = r.stream(&key, &nonce, len, d);
        for off in [0usize, 1, 3, 15] {
            let mut buf = vec![0xA5u8; off + len + 7];
            new().fill(&mut buf[off..off + len]);
            assert_eq!(&buf[off..off + len], &want[..]);
            assert!(buf[..off].iter().all(|&b| b == 0xA5));
            assert!(buf[off + len..].iter().all(|&b| b == 0xA5));
        }
    }

    // Chunked calls must produce one continuous keystream.
    for &len in &[
        0usize, 1, 2, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 1023, 1024, 1025,
    ] {
        let want = r.stream(&key, &nonce, len, d);
        for chunk in [1usize, 2, 3, 5, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65] {
            let mut got = vec![0u8; len];
            let mut s = new();
            let mut off = 0;
            while off < len {
                let end = (off + chunk).min(len);
                s.fill(&mut got[off..end]);
                off = end;
            }
            assert_eq!(got, want, "chunked fill d={d} len={len} chunk={chunk}");

            let mut got = want.clone();
            let mut s = new();
            let mut off = 0;
            while off < len {
                let end = (off + chunk).min(len);
                s.xor(&mut got[off..end]);
                off = end;
            }
            assert!(got.iter().all(|&b| b == 0), "chunked xor d={d} len={len} chunk={chunk}");
        }
    }

    // Mixing fill and xor across every split point must stay continuous.
    for &len in &[0usize, 1, 2, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129] {
        let want = r.stream(&key, &nonce, len, d);
        for split in 0..=len {
            let mut got = vec![0u8; len];
            let mut s = new();
            let (a, b) = got.split_at_mut(split);
            s.fill(a);
            s.xor(b);
            assert_eq!(got, want, "mixed fill/xor d={d} len={len} split={split}");
        }
    }
}

/// Arbitrary interleavings of `fill_keystream` and `xor_keystream_in_place`
/// (including `xor` followed by `fill` while a partial block is buffered) must
/// still produce one continuous RFC keystream.
fn stream_alternating_sweep<S: StreamOps>(r: &Reference, d: usize, new: impl Fn() -> S) {
    let key = [0x42u8; 32];
    let nonce = [0x24u8; 32];
    for &len in &[
        0usize, 1, 2, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 1023, 1024, 1025, 2050,
    ] {
        let want = r.stream(&key, &nonce, len, d);
        let mut got = pattern(len, 0x99);
        let mut s = new();
        let mut state = 0x1234_5678u32;
        let mut off = 0;
        while off < len {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            let end = (off + (state as usize % 37) + 1).min(len);
            if state & 0x100 != 0 {
                // `fill` overwrites, so seed the region with the wrong bytes.
                for b in &mut got[off..end] {
                    *b = 0xA5;
                }
                s.fill(&mut got[off..end]);
                assert_eq!(&got[off..end], &want[off..end], "fill d={d} len={len} off={off} end={end}");
            } else {
                // `xor` must add exactly the RFC keystream to the input.
                for i in off..end {
                    got[i] = (state as u8).wrapping_add(i as u8);
                }
                s.xor(&mut got[off..end]);
                for i in off..end {
                    let input = (state as u8).wrapping_add(i as u8);
                    assert_eq!(got[i], input ^ want[i], "xor d={d} len={len} i={i}");
                }
            }
            off = end;
        }
    }
}

#[test]
fn stream_matches_reference() {
    let r = Reference::new();
    stream_sweep(&r, 1, || Aegis256Stream::new(&[0x42; 32], &[0x24; 32]));
    stream_sweep(&r, 2, || Aegis256X2Stream::new(&[0x42; 32], &[0x24; 32]));
    stream_sweep(&r, 4, || Aegis256X4Stream::new(&[0x42; 32], &[0x24; 32]));
    stream_alternating_sweep(&r, 1, || Aegis256Stream::new(&[0x42; 32], &[0x24; 32]));
    stream_alternating_sweep(&r, 2, || Aegis256X2Stream::new(&[0x42; 32], &[0x24; 32]));
    stream_alternating_sweep(&r, 4, || Aegis256X4Stream::new(&[0x42; 32], &[0x24; 32]));
}

#[test]
fn stream_trait_matches_inherent_method() {
    let key = [0x5Au8; 32];
    let nonce = [0xA5u8; 32];
    let mut a = Aegis256Stream::new(&key, &nonce);
    let mut b = Aegis256Stream::new(&key, &nonce);
    let input = pattern(300, 0x19);
    let mut x = input.clone();
    let mut y = input.clone();
    a.xor_keystream_in_place(&mut x);
    StreamCipher::xor_keystream(&mut b, &mut y).unwrap();
    assert_eq!(x, y);
}

// ── AEGISMAC ────────────────────────────────────────────────────────────────

fn mac_sweep<M: MacOps>(r: &Reference, d: usize, tag_len: usize, new: impl Fn() -> M) {
    let key = [0x42u8; 32];
    let nonce = [0x24u8; 32];
    let mut lens: Vec<usize> = (0..=80).collect();
    lens.extend([127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 2047, 2048, 2049, 4096]);

    for &len in &lens {
        let data = pattern(len, 0x51);
        let want = r.mac(&key, &nonce, &data, d, tag_len);
        let mut m = new();
        m.feed(&data);
        assert_eq!(m.finish(), want, "mac d={d} tag={tag_len} len={len}");

        let mut m = new();
        m.feed(&data);
        assert!(m.verify_tag(&want), "verify rejected its own tag d={d} len={len}");
    }

    // Incremental updates must hash identically (the buffer/rate logic).
    for &len in &[
        0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1024, 1025,
    ] {
        let data = pattern(len, 0x51);
        let want = r.mac(&key, &nonce, &data, d, tag_len);
        for chunk in [
            1usize, 2, 3, 5, 7, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 1000, 1024,
        ] {
            let mut m = new();
            for piece in data.chunks(chunk) {
                m.feed(piece);
            }
            assert_eq!(m.finish(), want, "incremental mac d={d} len={len} chunk={chunk}");
        }
    }

    // Empty input and `new()` (all-zero nonce) must work.
    let data = pattern(35, 0x51);
    let want = r.mac(&key, &nonce, &data, d, tag_len);
    let mut m = new();
    m.feed(&data);
    assert!(m.verify_tag(&want));

    // Every tag byte position must be authenticated.
    for i in 0..tag_len {
        let mut bad = want.clone();
        bad[i] ^= 1;
        let mut m = new();
        m.feed(&data);
        assert!(!m.verify_tag(&bad), "MAC ignored tag byte {i} (d={d})");
    }
}

#[test]
fn mac_matches_reference() {
    let r = Reference::new();
    mac_sweep(&r, 1, 16, || Aegis256Mac::<16>::new_with_nonce(&[0x42; 32], &[0x24; 32]));
    mac_sweep(&r, 1, 32, || Aegis256Mac::<32>::new_with_nonce(&[0x42; 32], &[0x24; 32]));
    mac_sweep(&r, 2, 16, || Aegis256X2Mac::<16>::new_with_nonce(&[0x42; 32], &[0x24; 32]));
    mac_sweep(&r, 2, 32, || Aegis256X2Mac::<32>::new_with_nonce(&[0x42; 32], &[0x24; 32]));
    mac_sweep(&r, 4, 16, || Aegis256X4Mac::<16>::new_with_nonce(&[0x42; 32], &[0x24; 32]));
    mac_sweep(&r, 4, 32, || Aegis256X4Mac::<32>::new_with_nonce(&[0x42; 32], &[0x24; 32]));
}

#[test]
fn mac_zero_nonce_constructor_matches_explicit_zero_nonce() {
    let key = [0x11u8; 32];
    let data = pattern(100, 0x41);
    let mut a = Aegis256Mac::<16>::new(&key);
    a.update(&data).unwrap();
    let mut b = Aegis256Mac::<16>::new_with_nonce(&key, &[0u8; 32]);
    b.update(&data).unwrap();
    assert_eq!(a.finalize(), b.finalize());

    // Cloning must fork the state without mutating the original.
    let mut base = Aegis256X2Mac::<32>::new_with_nonce(&key, &[0x24u8; 32]);
    base.update(&data).unwrap();
    let expect = base.clone().finalize();
    let mut other = base.clone();
    other.update(b"x").unwrap();
    assert_ne!(other.finalize(), expect);
    assert_eq!(base.finalize(), expect);
}

// ── Soft-vs-hardware is covered inside the crate ────────────────────────────
//
// `crypto/src/aegis/tests.rs` compares every selectable public backend against
// `aegis_core::Soft<D>` at the same boundaries, and `aegis_core.rs` compares
// the split-state bulk loop with the ordinary core.
