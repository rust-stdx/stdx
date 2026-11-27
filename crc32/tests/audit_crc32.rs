//! Independent CRC-32/ISO-HDLC tests: bitwise reference, Python zlib vectors,
//! chunking, `new_with_initial`/`new_with_initial_len`/`combine`/`reset`, and
//! the specialized PCLMULQDQ / aarch64-CRC backends against the baseline.

use crc32::Hasher;

#[path = "audit_support/crc32_vectors.rs"]
mod vectors;

const POLY: u32 = 0xEDB8_8320;

/// Bit-by-bit reflected CRC-32/ISO-HDLC with the same "initial state" contract
/// as `crc32::Hasher::new_with_initial`.
fn ref_crc(init: u32, data: &[u8]) -> u32 {
    let mut crc = !init;
    for &b in data {
        crc ^= u32::from(b);
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ POLY } else { crc >> 1 };
        }
    }
    !crc
}

fn pattern(len: usize, salt: u64) -> Vec<u8> {
    let mut x = salt ^ 0x9E37_79B9_7F4A_7C15;
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        out.push((x >> 24) as u8);
    }
    out
}

fn hash_with_init(init: u32, data: &[u8]) -> u32 {
    let mut h = Hasher::new_with_initial(init);
    h.update(data);
    h.finalize()
}

#[test]
fn known_vectors_and_zlib_table() {
    assert_eq!(crc32::hash(b""), 0);
    assert_eq!(crc32::hash(b"123456789"), 0xCBF4_3926);
    assert_eq!(
        crc32::hash(
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
                     \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
        ),
        0x190A_55AD
    );
    assert_eq!(
        crc32::hash(
            b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\
                     \xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"
        ),
        0xFF6C_AB0B
    );

    for &(len, expected) in vectors::CRC32_SMALL {
        let data = pattern(len, 0);
        assert_eq!(crc32::hash(&data), expected, "zlib table len={len}");
        assert_eq!(ref_crc(0, &data), expected, "bitwise reference len={len}");
    }
    for &(len, expected) in vectors::CRC32_LARGE {
        let data = pattern(len, 0);
        assert_eq!(crc32::hash(&data), expected, "zlib table large len={len}");
        assert_eq!(ref_crc(0, &data), expected, "bitwise reference large len={len}");
    }
}

#[test]
fn chunked_updates_match_reference_and_zlib_table() {
    // Every two-way split for small lengths.
    for len in 0..=512usize {
        let data = pattern(len, 0);
        let expected = crc32::hash(&data);
        for split in 0..=len {
            let mut h = Hasher::new();
            h.update(&data[..split]);
            h.update(&data[split..]);
            assert_eq!(h.finalize(), expected, "len={len} split={split}");
        }
    }

    // Deterministic many-way chunking up to 1 MiB.
    for &(len, expected) in vectors::CRC32_LARGE {
        let data = pattern(len, 0);
        let mut h = Hasher::new();
        let mut left = len;
        let mut sizes = [1usize, 3, 15, 16, 17, 63, 64, 65, 127, 128, 255, 256, 257, 4096]
            .iter()
            .cycle();
        while left > 0 {
            let n = (*sizes.next().unwrap()).min(left);
            let start = len - left;
            h.update(&data[start..start + n]);
            left -= n;
        }
        assert_eq!(h.finalize(), expected, "chunked large len={len}");
    }
}

#[test]
fn new_with_initial_and_initial_len() {
    let data = pattern(1024, 0);
    for &init in &[0u32, 1, 0xFFFF_FFFF, 0x1234_5678, 0xDEAD_BEEF] {
        for len in [0usize, 1, 15, 16, 17, 63, 64, 100, 511, 512, 1024] {
            let part = &data[..len];
            assert_eq!(hash_with_init(init, part), ref_crc(init, part), "init={init:#x} len={len}");
        }
    }

    // `new_with_initial_len(a, len_a)` followed by `update(b)` equals hashing a||b.
    for len_a in [0usize, 1, 15, 16, 17, 63, 64, 100, 300, 511] {
        for len_b in [0usize, 1, 15, 16, 17, 63, 64, 100, 300, 511] {
            let a = pattern(len_a, 0);
            let b = pattern(len_b, 1);
            let mut expected_data = a.clone();
            expected_data.extend_from_slice(&b);
            let expected = crc32::hash(&expected_data);

            let mut h = Hasher::new_with_initial_len(crc32::hash(&a), len_a as u64);
            h.update(&b);
            assert_eq!(h.finalize(), expected, "initial_len len_a={len_a} len_b={len_b}");
        }
    }
}

#[test]
fn combine_algebra() {
    for len_a in [
        0usize, 1, 2, 15, 16, 17, 63, 64, 65, 127, 128, 255, 256, 300, 1024, 4096,
    ] {
        for len_b in [
            0usize, 1, 2, 15, 16, 17, 63, 64, 65, 127, 128, 255, 256, 300, 1024, 4096,
        ] {
            let a = pattern(len_a, 0);
            let b = pattern(len_b, 1);

            let mut all = Hasher::new();
            all.update(&a);
            all.update(&b);
            let expected = all.finalize();

            // Plain combine: `other` is a normal hasher over b.
            let mut h = Hasher::new();
            h.update(&a);
            let mut other = Hasher::new();
            other.update(&b);
            h.combine(&other);
            assert_eq!(h.finalize(), expected, "combine len_a={len_a} len_b={len_b}");

            // Reconstruct both sides from their lengths (the documented use).
            let mut h = Hasher::new_with_initial_len(crc32::hash(&a), len_a as u64);
            let other = Hasher::new_with_initial_len(crc32::hash(&b), len_b as u64);
            h.combine(&other);
            assert_eq!(h.finalize(), expected, "combine_from_len len_a={len_a} len_b={len_b}");

            // Combining the other way round must not accidentally match (unless
            // a||b == b||a), but it must still match b||a exactly.
            let mut ba = Hasher::new();
            ba.update(&b);
            ba.update(&a);
            let mut h = Hasher::new();
            h.update(&b);
            let mut other = Hasher::new();
            other.update(&a);
            h.combine(&other);
            assert_eq!(h.finalize(), ba.finalize(), "reverse combine len_a={len_a} len_b={len_b}");
        }
    }
}

#[test]
fn reset_and_clone() {
    let data = pattern(777, 0);
    let expected = crc32::hash(&data);

    let mut h = Hasher::new();
    h.update(&data);
    assert_eq!(h.clone().finalize(), expected);
    h.reset();
    assert_eq!(h.clone().finalize(), 0);
    h.update(&data);
    assert_eq!(h.finalize(), expected);

    let mut h = Hasher::new_with_initial(0x1234_5678);
    h.update(&data[..100]);
    h.reset();
    h.update(&data);
    assert_eq!(h.finalize(), expected);

    // std::hash::Hasher must agree with finalize and be clone-safe.
    use std::hash::Hasher as _;
    let mut h = Hasher::new();
    h.write(&data);
    assert_eq!(h.finish(), u64::from(expected));
}

#[test]
fn specialized_matches_baseline_every_length() {
    let mut checked_any = false;
    for len in 0..=4096usize {
        let data = pattern(len, 0x1234 + len as u64);
        for &init in &[0u32, 0xFFFF_FFFF, 0x1234_5678] {
            let mut base = Hasher::internal_new_baseline(init, 0);
            let Some(mut spec) = Hasher::internal_new_specialized(init, 0) else {
                return;
            };
            checked_any = true;
            // Chunk at boundaries that hit the 128-byte entry threshold and
            // the 64/16-byte fold loops.
            let mut left = len;
            let mut off = 0;
            let mut sizes = [1usize, 15, 16, 63, 64, 65, 127, 128, 129, 511, 512].iter().cycle();
            while left > 0 {
                let n = (*sizes.next().unwrap()).min(left);
                base.update(&data[off..off + n]);
                spec.update(&data[off..off + n]);
                off += n;
                left -= n;
            }
            let expected = ref_crc(init, &data);
            assert_eq!(base.finalize(), expected, "baseline len={len} init={init:#x}");
            assert_eq!(spec.finalize(), expected, "specialized len={len} init={init:#x}");
        }
    }
    assert!(checked_any, "no specialized backend was available on this target");
}

#[test]
fn specialized_handles_unaligned_inputs() {
    let backing = pattern(4096 + 32, 0xABCD);
    for len in [
        0usize, 1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 255, 256, 511, 512, 1024, 4096,
    ] {
        for offset in 0..16usize {
            let data = &backing[offset..offset + len];
            let Some(mut spec) = Hasher::internal_new_specialized(0, 0) else {
                return;
            };
            let mut base = Hasher::internal_new_baseline(0, 0);
            spec.update(data);
            base.update(data);
            let spec_sum = spec.finalize();
            assert_eq!(spec_sum, base.finalize(), "len={len} offset={offset}");
            assert_eq!(spec_sum, ref_crc(0, data), "ref len={len} offset={offset}");
        }
    }
}

/// On targets where the specialized backend must exist, make sure the test
/// above is not silently skipping it.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn specialized_backend_is_present_on_x86() {
    assert!(
        Hasher::internal_new_specialized(0, 0).is_some(),
        "PCLMULQDQ/SSE4.1 should be available on this x86 test host"
    );
}

#[cfg(all(target_arch = "aarch64", feature = "nightly"))]
#[test]
fn specialized_backend_is_present_on_aarch64_crc() {
    assert!(
        Hasher::internal_new_specialized(0, 0).is_some(),
        "aarch64 CRC extension should be available with the `nightly` feature"
    );
}
