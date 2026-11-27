#![no_std]

use core::panic::PanicInfo;

use crypto::{
    Aead, StreamCipher,
    aegis::{
        Aegis256, Aegis256Stream, Aegis256X2, Aegis256X2Mac, Aegis256X2Stream, Aegis256X4, Aegis256X4Mac,
        Aegis256X4Stream,
    },
    chacha::{ChaCha20Djb, ChaCha20Ietf},
};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn hexval(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 0,
    }
}
fn hex_eq(actual: &[u8], expected_hex: &[u8]) -> bool {
    if actual.len() * 2 != expected_hex.len() {
        return false;
    }
    for (i, b) in actual.iter().enumerate() {
        let hi = hexval(expected_hex[2 * i]);
        let lo = hexval(expected_hex[2 * i + 1]);
        if *b != (hi << 4) | lo {
            return false;
        }
    }
    true
}

static KEY256: [u8; 32] = [
    0x10, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static NONCE256: [u8; 32] = [
    0x10, 0x00, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
fn key_std() -> [u8; 32] {
    let mut k = [0u8; 32];
    for i in 0..32 {
        k[i] = i as u8;
    }
    k
}
fn nonce_std() -> [u8; 32] {
    let mut n = [0u8; 32];
    for i in 0..32 {
        n[i] = (0x10 + i) as u8;
    }
    n
}

fn pattern(buf: &mut [u8], mul: u8, add: u8) {
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(mul).wrapping_add(add);
    }
}

/// Bit 0: ChaCha wasm SIMD != generic for some length.
/// Bit 1: ChaCha known-answer mismatch.
/// Bit 2: AEGIS known-answer mismatch.
/// Bit 3: AEGIS roundtrip/tamper failure.
/// Bit 4: AEGIS MAC known-answer mismatch.
/// Bit 5: AEGIS stream continuity failure.
#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut fail = 0u32;

    // ChaCha: compare the dispatcher's one-shot (SIMD) output against
    // 64-byte-at-a-time calls, which stay on the generic path.
    let key = key_std();
    let n12 = [0x11u8, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc];
    let n8 = [1u8, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    for len in 0..=700usize {
        let mut a = [0u8; 701];
        let mut b = [0u8; 701];
        for i in 0..len {
            a[i] = (i as u8).wrapping_mul(31).wrapping_add(7);
            b[i] = a[i];
        }
        ChaCha20Ietf::new(&key, &n12).xor_keystream(&mut a[..len]).unwrap();
        let mut c = ChaCha20Ietf::new(&key, &n12);
        let mut off = 0;
        while off < len {
            let end = core::cmp::min(off + 64, len);
            c.xor_keystream(&mut b[off..end]).unwrap();
            off = end;
        }
        if a[..len] != b[..len] {
            fail |= 1;
        }

        let mut a = [0u8; 701];
        let mut b = [0u8; 701];
        for i in 0..len {
            a[i] = (i as u8).wrapping_mul(13).wrapping_add(5);
            b[i] = a[i];
        }
        ChaCha20Djb::new(&key, &n8).xor_keystream(&mut a[..len]).unwrap();
        let mut c = ChaCha20Djb::new(&key, &n8);
        let mut off = 0;
        while off < len {
            let end = core::cmp::min(off + 64, len);
            c.xor_keystream(&mut b[off..end]).unwrap();
            off = end;
        }
        if a[..len] != b[..len] {
            fail |= 1;
        }
    }
    // RFC 8439 A.2 #1 (zero key/nonce, counter 0, 64 bytes).
    {
        let z = [0u8; 32];
        let nz = [0u8; 12];
        let mut out = [0u8; 64];
        ChaCha20Ietf::new(&z, &nz).xor_keystream(&mut out).unwrap();
        if !hex_eq(&out, b"76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586") { fail |= 2; }
    }

    // AEGIS-256/ X2 / X4 RFC 10032 known answers.
    {
        let mut buf = [0u8; 16];
        let t = Aegis256::<16>::new(&KEY256)
            .encrypt_in_place(&mut buf, &NONCE256, &[])
            .unwrap();
        if !hex_eq(t.as_ref(), b"3fe91994768b332ed7f570a19ec5896e") {
            fail |= 4;
        }
        buf = [0u8; 16];
        let t = Aegis256::<32>::new(&KEY256)
            .encrypt_in_place(&mut buf, &NONCE256, &[])
            .unwrap();
        if !hex_eq(t.as_ref(), b"1181a1d18091082bf0266f66297d167d2e68b845f61a3b0527d31fc7b7b89f13") {
            fail |= 4;
        }

        let ks = key_std();
        let ns = nonce_std();
        let mut e = [0u8; 0];
        let t2 = Aegis256X2::<16>::new(&ks).encrypt_in_place(&mut e, &ns, &[]).unwrap();
        if !hex_eq(t2.as_ref(), b"62cdbab084c83dacdb945bb446f049c8") {
            fail |= 4;
        }
        let t2 = Aegis256X2::<32>::new(&ks).encrypt_in_place(&mut e, &ns, &[]).unwrap();
        if !hex_eq(t2.as_ref(), b"25d7e799b49a80354c3f881ac2f1027f471a5d293052bd9997abd3ae84014bb7") {
            fail |= 4;
        }
        let t4 = Aegis256X4::<16>::new(&ks).encrypt_in_place(&mut e, &ns, &[]).unwrap();
        if !hex_eq(t4.as_ref(), b"3b7fee6cee7bf17888ad11ed2397beb4") {
            fail |= 4;
        }
        let t4 = Aegis256X4::<32>::new(&ks).encrypt_in_place(&mut e, &ns, &[]).unwrap();
        if !hex_eq(t4.as_ref(), b"6093a1a8aab20ec635dc1ca71745b01b5bec4fc444c9ffbebd710d4a34d20eaf") {
            fail |= 4;
        }

        // X2 120-byte test vector A.6.3 (16-byte tag) and X4 A.7.3.
        let mut msg = [0u8; 120];
        for i in 0..120 {
            msg[i] = 4 + (i as u8 % 3);
        }
        // The RFC input is 0x04050607 repeating; build it exactly.
        for i in 0..120 {
            msg[i] = [4u8, 5, 6, 7][i % 4];
        }
        let ad = [1u8, 2, 3, 4, 1, 2, 3, 4];
        let mut x2 = msg;
        let t = Aegis256X2::<16>::new(&ks).encrypt_in_place(&mut x2, &ns, &ad).unwrap();
        if !hex_eq(&x2, b"72120c2ea8236180d67859001f4729077b7064c414384fe3a7b52f1571f4f8a7d0f01e18db4f3bc0adb150702e5d147a8d36522132761b994c1bd395589e2ccf0790dfe2a3d12d61cd666b2859827739db4037dd3124c78424459376f6cac08e1a7223a2a43e398ce6385cd654a19f481cba3b8f25910b42") { fail |= 4; }
        if !hex_eq(t.as_ref(), b"635d391828520bf1512763f0c8f5cdbd") {
            fail |= 4;
        }
        let mut x2b = [0u8; 120];
        for i in 0..120 {
            x2b[i] = [4u8, 5, 6, 7][i % 4];
        }
        let t = Aegis256X2::<32>::new(&ks).encrypt_in_place(&mut x2b, &ns, &ad).unwrap();
        if !hex_eq(t.as_ref(), b"b5668d3317159e9cc5d46e4803c3a76ad63bb42b3f47956d94f30db8cb366ad7") {
            fail |= 4;
        }
        let mut x4 = msg;
        let t = Aegis256X4::<16>::new(&ks).encrypt_in_place(&mut x4, &ns, &ad).unwrap();
        if !hex_eq(&x4, b"bfc2085b7e8017da99b0b6d646ae4d01f4ba8f2e7dfca1d759ae48a135139b9aaac6b4f5db810d426be1fdaff4e1454153a34b11da78ed7e418ee2ee9853042e95536aecbb694cea1b16a478eb0d4d1bf6509b1ce652a45af58e0e46ffccfa2d0426e702391d2ff5813808b81748a490dd656465fed61f09") { fail |= 4; }
        if !hex_eq(t.as_ref(), b"b63b611b13975e2f3dc3cb6c2397bfcd") {
            fail |= 4;
        }
        let mut x4b = [0u8; 120];
        for i in 0..120 {
            x4b[i] = [4u8, 5, 6, 7][i % 4];
        }
        let t = Aegis256X4::<32>::new(&ks).encrypt_in_place(&mut x4b, &ns, &ad).unwrap();
        if !hex_eq(t.as_ref(), b"7847eace74409ee56c8f4cf63a9c2841ce7c8bd567d7c0ca514c879a190b978c") {
            fail |= 4;
        }
    }

    // AEGIS roundtrip/tamper for all variants, both tag sizes, short lengths.
    {
        let ks = key_std();
        let ns = nonce_std();
        for len in 0..=129usize {
            let mut p = [0u8; 129];
            let mut a = [0u8; 129];
            pattern(&mut p[..len], 17, 5);
            pattern(&mut a[..len.min(34)], 7, 3);
            let now = len.min(34);
            // length-0..129 data in a fixed array; slice exactly.
            let mut b = p;
            let ta = Aegis256::<16>::new(&ks)
                .encrypt_in_place(&mut b[..len], &ns, &a[..now])
                .unwrap();
            if Aegis256::<16>::new(&ks)
                .decrypt_in_place(&mut b[..len], &ns, &a[..now], ta.as_ref())
                .is_err()
            {
                fail |= 8;
            }
            if b[..len] != p[..len] {
                fail |= 8;
            }
            let mut c = p;
            let tb = Aegis256X2::<16>::new(&ks)
                .encrypt_in_place(&mut c[..len], &ns, &a[..now])
                .unwrap();
            if Aegis256X2::<16>::new(&ks)
                .decrypt_in_place(&mut c[..len], &ns, &a[..now], tb.as_ref())
                .is_err()
            {
                fail |= 8;
            }
            if c[..len] != p[..len] {
                fail |= 8;
            }
            let mut d = p;
            let td = Aegis256X4::<16>::new(&ks)
                .encrypt_in_place(&mut d[..len], &ns, &a[..now])
                .unwrap();
            if Aegis256X4::<16>::new(&ks)
                .decrypt_in_place(&mut d[..len], &ns, &a[..now], td.as_ref())
                .is_err()
            {
                fail |= 8;
            }
            if d[..len] != p[..len] {
                fail |= 8;
            }
            let mut d2 = p;
            let td2 = Aegis256X2::<32>::new(&ks)
                .encrypt_in_place(&mut d2[..len], &ns, &a[..now])
                .unwrap();
            if Aegis256X2::<32>::new(&ks)
                .decrypt_in_place(&mut d2[..len], &ns, &a[..now], td2.as_ref())
                .is_err()
            {
                fail |= 8;
            }
            if d2[..len] != p[..len] {
                fail |= 8;
            }
            let mut d4 = p;
            let td4 = Aegis256::<32>::new(&ks)
                .encrypt_in_place(&mut d4[..len], &ns, &a[..now])
                .unwrap();
            if Aegis256::<32>::new(&ks)
                .decrypt_in_place(&mut d4[..len], &ns, &a[..now], td4.as_ref())
                .is_err()
            {
                fail |= 8;
            }
            if d4[..len] != p[..len] {
                fail |= 8;
            }
            if len > 0 {
                // Flip one ciphertext byte; tag must fail and buffer zeroed.
                let mut e = p;
                let te = Aegis256X4::<32>::new(&ks)
                    .encrypt_in_place(&mut e[..len], &ns, &a[..now])
                    .unwrap();
                e[0] ^= 1;
                let mut f = e;
                if Aegis256X4::<32>::new(&ks)
                    .decrypt_in_place(&mut f[..len], &ns, &a[..now], te.as_ref())
                    .is_ok()
                {
                    fail |= 8;
                }
                for i in 0..len {
                    if f[i] != 0 {
                        fail |= 8;
                    }
                }
            }
        }
    }

    // AEGISMAC A.8.5 / A.8.6 (X2/X4, both tag sizes).
    {
        let data: [u8; 35] = {
            let mut d = [0u8; 35];
            for i in 0..35 {
                d[i] = i as u8;
            }
            d
        };
        let mut m = Aegis256X2Mac::<16>::new_with_nonce(&KEY256, &NONCE256);
        m.update(&data).unwrap();
        if !hex_eq(&m.finalize(), b"fb319cb6dd728a764606fb14d37f2a5e") {
            fail |= 16;
        }
        let mut m = Aegis256X2Mac::<32>::new_with_nonce(&KEY256, &NONCE256);
        m.update(&data).unwrap();
        if !hex_eq(
            &m.finalize(),
            b"0844b20ed5147ceae89c7a160263afd4b1382d6b154ecf560ce8a342cb6a8fd1",
        ) {
            fail |= 16;
        }
        let mut m = Aegis256X4Mac::<16>::new_with_nonce(&KEY256, &NONCE256);
        m.update(&data).unwrap();
        if !hex_eq(&m.finalize(), b"a51f9bc5beae60cce77f0dbc60761edd") {
            fail |= 16;
        }
        let mut m = Aegis256X4Mac::<32>::new_with_nonce(&KEY256, &NONCE256);
        m.update(&data).unwrap();
        if !hex_eq(
            &m.finalize(),
            b"b36a16ef07c36d75a91f437502f24f545b8dfa88648ed116943c29fead3bf10c",
        ) {
            fail |= 16;
        }
    }

    // Stream continuity: fill vs chunked fill vs AEAD(zeros) for d=1/2/4.
    {
        let ks = key_std();
        let ns = nonce_std();
        for len in 0..=129usize {
            let mut one = [0u8; 129];
            let mut two = [0u8; 129];
            Aegis256Stream::new(&ks, &ns).fill_keystream(&mut one[..len]);
            let mut s = Aegis256Stream::new(&ks, &ns);
            let mut off = 0;
            while off < len {
                let end = core::cmp::min(off + 7, len);
                s.fill_keystream(&mut two[off..end]);
                off = end;
            }
            if one[..len] != two[..len] {
                fail |= 32;
            }
            let mut z = [0u8; 129];
            let _ = Aegis256::<16>::new(&ks)
                .encrypt_in_place(&mut z[..len], &ns, &[])
                .unwrap();
            if one[..len] != z[..len] {
                fail |= 32;
            }

            let mut one = [0u8; 129];
            let mut two = [0u8; 129];
            Aegis256X2Stream::new(&ks, &ns).fill_keystream(&mut one[..len]);
            let mut s = Aegis256X2Stream::new(&ks, &ns);
            let mut off = 0;
            while off < len {
                let end = core::cmp::min(off + 7, len);
                s.fill_keystream(&mut two[off..end]);
                off = end;
            }
            if one[..len] != two[..len] {
                fail |= 32;
            }
            let mut z = [0u8; 129];
            let _ = Aegis256X2::<16>::new(&ks)
                .encrypt_in_place(&mut z[..len], &ns, &[])
                .unwrap();
            if one[..len] != z[..len] {
                fail |= 32;
            }

            let mut one = [0u8; 129];
            let mut two = [0u8; 129];
            Aegis256X4Stream::new(&ks, &ns).fill_keystream(&mut one[..len]);
            let mut s = Aegis256X4Stream::new(&ks, &ns);
            let mut off = 0;
            while off < len {
                let end = core::cmp::min(off + 7, len);
                s.fill_keystream(&mut two[off..end]);
                off = end;
            }
            if one[..len] != two[..len] {
                fail |= 32;
            }
            let mut z = [0u8; 129];
            let _ = Aegis256X4::<16>::new(&ks)
                .encrypt_in_place(&mut z[..len], &ns, &[])
                .unwrap();
            if one[..len] != z[..len] {
                fail |= 32;
            }
        }
    }

    fail
}

#[unsafe(no_mangle)]
pub extern "C" fn probe_a256_16() -> u32 {
    let mut buf = [0u8; 16];
    let t = Aegis256::<16>::new(&KEY256)
        .encrypt_in_place(&mut buf, &NONCE256, &[])
        .unwrap();
    u32::from_be_bytes([t.as_ref()[0], t.as_ref()[1], t.as_ref()[2], t.as_ref()[3]])
}
#[unsafe(no_mangle)]
pub extern "C" fn probe_a256_32() -> u32 {
    let mut buf = [0u8; 16];
    buf = [0u8; 16];
    let t = Aegis256::<32>::new(&KEY256)
        .encrypt_in_place(&mut buf, &NONCE256, &[])
        .unwrap();
    u32::from_be_bytes([t.as_ref()[0], t.as_ref()[1], t.as_ref()[2], t.as_ref()[3]])
}
#[unsafe(no_mangle)]
pub extern "C" fn probe_x2_16() -> u32 {
    let mut e = [0u8; 0];
    let ks = key_std();
    let ns = nonce_std();
    let t = Aegis256X2::<16>::new(&ks).encrypt_in_place(&mut e, &ns, &[]).unwrap();
    u32::from_be_bytes([t.as_ref()[0], t.as_ref()[1], t.as_ref()[2], t.as_ref()[3]])
}
#[unsafe(no_mangle)]
pub extern "C" fn probe_mac_x2_16() -> u32 {
    let mut d = [0u8; 35];
    for i in 0..35 {
        d[i] = i as u8;
    }
    let mut m = Aegis256X2Mac::<16>::new_with_nonce(&KEY256, &NONCE256);
    m.update(&d).unwrap();
    let t = m.finalize();
    u32::from_be_bytes([t[0], t[1], t[2], t[3]])
}
#[unsafe(no_mangle)]
pub extern "C" fn probe_a256_soft_16() -> u32 {
    // Same vector but force the scalar dispatch is not possible; this is the
    // native result to compare against, hardcoded below in the JS harness.
    let mut buf = [0u8; 16];
    let t = Aegis256::<16>::new(&KEY256)
        .encrypt_in_place(&mut buf, &NONCE256, b"x")
        .unwrap();
    u32::from_be_bytes([t.as_ref()[0], t.as_ref()[1], t.as_ref()[2], t.as_ref()[3]])
}

#[unsafe(no_mangle)]
pub extern "C" fn probe_mac_d1_16() -> u32 {
    let mut d = [0u8; 35];
    for i in 0..35 {
        d[i] = i as u8;
    }
    let mut m = crypto::aegis::Aegis256Mac::<16>::new_with_nonce(&KEY256, &NONCE256);
    m.update(&d).unwrap();
    let t = m.finalize();
    u32::from_be_bytes([t[0], t[1], t[2], t[3]])
}

#[unsafe(no_mangle)]
pub extern "C" fn probe_mac_empty_d1_16() -> u32 {
    let m = crypto::aegis::Aegis256Mac::<16>::new_with_nonce(&KEY256, &NONCE256);
    let t = m.finalize();
    u32::from_be_bytes([t[0], t[1], t[2], t[3]])
}
#[unsafe(no_mangle)]
pub extern "C" fn probe_mac_empty_d1_32() -> u32 {
    let m = crypto::aegis::Aegis256Mac::<32>::new_with_nonce(&KEY256, &NONCE256);
    let t = m.finalize();
    u32::from_be_bytes([t[0], t[1], t[2], t[3]])
}
