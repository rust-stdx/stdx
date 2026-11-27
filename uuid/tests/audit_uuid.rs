//! Audit tests for `uuid`.
//!
//! `Display`/`to_heapless_string` build a fixed 36-byte buffer and then call
//! `str::from_utf8_unchecked`. These tests check that every buffer slot is
//! written with hex/`-` for all byte patterns and all textual forms, and that
//! parsing never panics for arbitrary input.

use uuid::{Uuid, Version};

fn ref_lower(bytes: [u8; 16]) -> String {
    let mut s = String::with_capacity(36);
    for (i, b) in bytes.iter().enumerate() {
        if matches!(i, 4 | 6 | 8 | 10) {
            s.push('-');
        }
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn patterns() -> Vec<[u8; 16]> {
    let mut out = vec![[0u8; 16], [0xffu8; 16], [0x01u8; 16], [0x80u8; 16]];
    // One byte varied at a time, then a deterministic pseudo-random set.
    for i in 0..16 {
        let mut b = [0u8; 16];
        b[i] = 0xff;
        out.push(b);
        let mut b = [0xa5u8; 16];
        b[i] = 0x5a;
        out.push(b);
    }
    let mut state = 0x0123_4567_89ab_cdefu64;
    for _ in 0..100 {
        let mut b = [0u8; 16];
        for slot in &mut b {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *slot = (state >> 40) as u8;
        }
        out.push(b);
    }
    out
}

#[test]
fn display_and_heapless_are_exact_and_ascii() {
    for bytes in patterns() {
        let u = Uuid::from_bytes(bytes);
        let want = ref_lower(bytes);
        assert_eq!(want.len(), 36);
        assert_eq!(u.to_string(), want, "Display for {bytes:?}");
        assert_eq!(u.to_heapless_string().as_str(), want, "heapless for {bytes:?}");
        assert_eq!(format!("{u:?}"), want);
        // Parse back from both cases.
        assert_eq!(Uuid::parse(&want).unwrap(), u);
        assert_eq!(Uuid::parse(&want.to_uppercase()).unwrap(), u);
    }
}

#[test]
fn shapecase_all_versions() {
    for v in 1..=8u8 {
        let mut b = [0x5au8; 16];
        b[6] = (b[6] & 0x0f) | (v << 4);
        b[8] = (b[8] & 0x3f) | 0x80;
        let u = Uuid::from_bytes(b);
        let want = match v {
            1 => Version::V1,
            2 => Version::V2,
            3 => Version::V3,
            4 => Version::V4,
            5 => Version::V5,
            6 => Version::V6,
            7 => Version::V7,
            8 => Version::V8,
            _ => unreachable!(),
        };
        assert_eq!(u.version(), want);
        assert_eq!(Uuid::parse(&u.to_string()).unwrap(), u);
    }
    assert_eq!(Uuid::nil().version(), Version::Nil);
    assert_eq!(Uuid::max().version(), Version::Max);
    assert_eq!(Uuid::from_bytes([0; 16]).to_string(), "00000000-0000-0000-0000-000000000000");
    assert_eq!(Uuid::max().to_string(), "ffffffff-ffff-ffff-ffff-ffffffffffff");
}

#[test]
fn parse_rejects_bad_lengths_and_shapes_without_panicking() {
    let valid = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
    for len in 0..=60usize {
        if len == valid.len() {
            continue;
        }
        let s: String = valid.chars().cycle().take(len).collect();
        assert_eq!(Uuid::parse(&s), Err(uuid::Error::InvalidUuid), "len {len}");
    }

    for i in 0..36 {
        if matches!(i, 8 | 13 | 18 | 23) {
            continue;
        }
        let mut b = valid.as_bytes().to_vec();
        b[i] = b'-';
        let s = String::from_utf8(b).unwrap();
        assert!(Uuid::parse(&s).is_err(), "hyphen at {i} accepted");
    }

    for i in [0usize, 4, 9, 14, 19, 24, 28, 32, 3, 7, 35] {
        for bad in [b'g', b'G', b'z', b' ', b'/', 0xff] {
            let mut b = valid.as_bytes().to_vec();
            b[i] = bad;
            let s = String::from_utf8_lossy(&b).into_owned();
            if s == valid {
                continue;
            }
            assert!(Uuid::parse(&s).is_err(), "bad byte {bad:#04x} at {i} accepted");
        }
    }

    for s in [
        "",
        " ",
        " f47ac10b-58cc-4372-a567-0e02b2c3d479",
        "f47ac10b-58cc-4372-a567-0e02b2c3d479 ",
        "{f47ac10b-58cc-4372-a567-0e02b2c3d479}",
        "urn:uuid:f47ac10b-58cc-4372-a567-0e02b2c3d479",
        "f47ac10b58cc4372a5670e02b2c3d479",
        "f47ac10b-58cc-4372-a567-0e02b2c3d47",
        "f47ac10b-58cc-4372-a567-0e02b2c3d4799",
        "é47ac10b-58cc-4372-a567-0e02b2c3d479",
    ] {
        assert!(Uuid::parse(s).is_err(), "accepted {s:?}");
    }
}

#[test]
fn parse_bytes_does_not_require_utf8() {
    // `TryFrom<&[u8]>` / `parse_bytes` accepts raw bytes, so invalid UTF-8 in
    // hex positions must be rejected as an error, not panic.
    let mut b = *b"f47ac10b-58cc-4372-a567-0e02b2c3d479";
    for pos in [0usize, 4, 9, 14, 19, 24, 28, 32] {
        let mut bad = b;
        bad[pos] = 0xff;
        assert_eq!(Uuid::try_from(&bad[..]), Err(uuid::Error::InvalidUuid), "pos {pos}");
        bad[pos] = 0x80;
        assert_eq!(Uuid::try_from(&bad[..]), Err(uuid::Error::InvalidUuid), "pos {pos}");
    }
    // Valid bytes parse.
    assert!(Uuid::try_from(&b[..]).is_ok());
    b[8] = 0xff;
    assert!(Uuid::try_from(&b[..]).is_err());

    assert_eq!(Uuid::from_slice(&[0; 16]).unwrap(), Uuid::nil());
    for len in [0usize, 1, 15, 17, 32] {
        assert!(Uuid::from_slice(&vec![0u8; len]).is_err(), "len {len}");
    }
}

#[cfg(feature = "std")]
#[test]
#[cfg_attr(miri, ignore = "generation reads OS entropy, which Miri cannot provide")]
fn generated_uuids_are_well_formed_and_v7_sorted() {
    for _ in 0..1000 {
        let v4 = Uuid::new_v4();
        assert_eq!(v4.version(), Version::V4);
        assert_eq!(v4.as_bytes()[8] & 0xc0, 0x80);
        assert_eq!(Uuid::parse(&v4.to_string()).unwrap(), v4);
        assert_eq!(v4.to_heapless_string().as_str(), v4.to_string());
    }

    let mut last = Uuid::new_v7();
    assert_eq!(last.version(), Version::V7);
    assert!(last.timestamp().is_some());
    for _ in 0..5000 {
        let next = Uuid::new_v7();
        assert_eq!(next.version(), Version::V7);
        assert_eq!(next.as_bytes()[8] & 0xc0, 0x80);
        assert!(next > last, "v7 not strictly increasing: {last} then {next}");
        assert_eq!(next.to_heapless_string().as_str(), next.to_string());
        last = next;
    }
}

#[test]
fn timestamp_only_for_v7() {
    for v in 1..=8u8 {
        let mut b = [0x11u8; 16];
        b[6] = (b[6] & 0x0f) | (v << 4);
        let u = Uuid::from_bytes(b);
        if v == 7 {
            assert!(u.timestamp().is_some());
        } else {
            assert!(u.timestamp().is_none(), "v{v}");
        }
    }
    assert_eq!(Uuid::nil().timestamp(), None);
    assert_eq!(Uuid::max().timestamp(), None);
}
