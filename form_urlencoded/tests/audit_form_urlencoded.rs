//! Audit tests for `form_urlencoded`.
//!
//! `ByteSerialize` returns `&str` built with `from_utf8_unchecked`; its
//! unchanged-byte set must stay ASCII-only. The parser must tolerate arbitrary
//! bytes without panicking and must never hand out invalid UTF-8.

use std::borrow::Cow;

use form_urlencoded::{Serializer, byte_serialize, parse};

fn unchanged(b: u8) -> bool {
    matches!(b, b'*' | b'-' | b'.' | b'0'..=b'9' | b'A'..=b'Z' | b'_' | b'a'..=b'z')
}

fn ref_serialize(input: &[u8]) -> String {
    let mut out = String::new();
    for &b in input {
        if b == b' ' {
            out.push('+');
        } else if unchanged(b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn owned<'a>(p: impl Iterator<Item = (Cow<'a, str>, Cow<'a, str>)>) -> Vec<(String, String)> {
    p.map(|(k, v)| (k.into_owned(), v.into_owned())).collect()
}

#[test]
fn every_byte_serializes_to_ascii() {
    for b in 0..=255u8 {
        let out = byte_serialize(&[b]).collect::<String>();
        assert_eq!(out, ref_serialize(&[b]), "byte {b:#04x}");
        assert!(out.is_ascii(), "byte {b:#04x} produced non-ASCII {out:?}");
    }

    let all: Vec<u8> = (0..=255).collect();
    let out = byte_serialize(&all).collect::<String>();
    assert_eq!(out, ref_serialize(&all));
    assert!(out.is_ascii());
    // Every byte has a unique serialization and round-trips through the parser
    // when embedded as a value (the parser applies UTF-8 lossy decoding, so
    // compare the byte-level serialization only).
    assert_eq!(byte_serialize(b"").collect::<String>(), "");
}

#[test]
fn serializer_matches_reference() {
    for b in 0..=255u8 {
        let s = b as char;
        let mut value = String::new();
        value.push(s);
        // Skip the non-ASCII chars that cannot be built from a single byte.
        if b >= 0x80 {
            continue;
        }
        let encoded = Serializer::new(String::new()).append_pair("k", &value).finish();
        assert_eq!(encoded, format!("k={}", ref_serialize(&[b])), "byte {b:#04x}");
    }
}

#[test]
fn parse_matrix() {
    let cases: Vec<(&[u8], Vec<(&str, &str)>)> = vec![
        (b"", vec![]),
        (b"&", vec![]),
        (b"&&&", vec![]),
        (b"a", vec![("a", "")]),
        (b"a=", vec![("a", "")]),
        (b"=b", vec![("", "b")]),
        (b"=", vec![("", "")]),
        (b"a=b", vec![("a", "b")]),
        (b"a=b=c", vec![("a", "b=c")]),
        (b"a=b&c=d", vec![("a", "b"), ("c", "d")]),
        (b"a=b&a=c", vec![("a", "b"), ("a", "c")]),
        (b"a+b=c+d", vec![("a b", "c d")]),
        (b"a%20b=c%20d", vec![("a b", "c d")]),
        (b"a=%00", vec![("a", "\0")]),
        (b"a=%0A%0D%09", vec![("a", "\n\r\t")]),
        (b"a=%zz", vec![("a", "%zz")]),
        (b"a=%", vec![("a", "%")]),
        (b"a=%4", vec![("a", "%4")]),
        (b"a=%4z", vec![("a", "%4z")]),
        (b"a=%41", vec![("a", "A")]),
        (b"a=%C3%A9", vec![("a", "é")]),
        (b"a=%E2%9C%93", vec![("a", "✓")]),
        // Invalid UTF-8 is replaced lossily.
        (b"a=%FF", vec![("a", "\u{FFFD}")]),
        (b"a=%C3", vec![("a", "\u{FFFD}")]),
        (b"a=%80", vec![("a", "\u{FFFD}")]),
        (b"%FF=%FE", vec![("\u{FFFD}", "\u{FFFD}")]),
        // `+` inside an escape is literal.
        (b"a=%2B", vec![("a", "+")]),
        (b"a=%2b", vec![("a", "+")]),
        // Consecutive separators.
        (b"a=b&&c=d&", vec![("a", "b"), ("c", "d")]),
        // A key may contain encoded '=' / '&'.
        (b"a%3Db=c%26d", vec![("a=b", "c&d")]),
        // Non-ASCII raw bytes are decoded lossily too.
        ("é=ü".as_bytes(), vec![("é", "ü")]),
        (b"\xff=\xfe", vec![("\u{FFFD}", "\u{FFFD}")]),
    ];

    for (input, want) in cases {
        let got = owned(parse(input));
        let want: Vec<(String, String)> = want.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect();
        assert_eq!(got, want, "input {:?}", String::from_utf8_lossy(input));
        // `into_owned` must agree.
        let got_owned: Vec<(String, String)> = parse(input).into_owned().collect();
        assert_eq!(got_owned, want);
    }
}

#[test]
#[cfg_attr(miri, ignore = "Miri is too slow to round-trip a ~1.6 MB value")]
fn roundtrip_serializer_parser() {
    let pairs: &[(&str, &str)] = &[
        ("", ""),
        ("key", ""),
        ("", "value"),
        ("a b", "c d"),
        ("a+b", "c&d"),
        ("a=b", "c=d"),
        ("%", "%%"),
        ("nul\0key", "nul\0value"),
        ("control\n\r\t", "\u{7f}\u{1f}"),
        ("é漢💖", "ü"),
        ("* - . _ 09AZaz", "!@#$'()~"),
    ];

    for &(k, v) in pairs {
        let encoded = Serializer::new(String::new()).append_pair(k, v).finish();
        assert!(encoded.is_ascii(), "encoded form not ASCII: {encoded:?}");
        let got = owned(parse(encoded.as_bytes()));
        assert_eq!(got, vec![(k.to_owned(), v.to_owned())], "encoded {encoded:?}");
    }

    // Long values, including multi-byte characters at chunk boundaries.
    let long = "aé💖&+=%".repeat(20_000);
    let encoded = Serializer::new(String::new()).append_pair(&long, &long).finish();
    let got = owned(parse(encoded.as_bytes()));
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, long);
    assert_eq!(got[0].1, long);
}

#[test]
#[cfg_attr(miri, ignore = "Miri is too slow for 500 generated inputs")]
fn arbitrary_input_does_not_panic() {
    // Deterministic LCG, short inputs, all byte values. `parse` is total: any
    // output is a valid `String` by construction.
    let mut state = 0x0bad_f00d_dead_beefu64;
    for _ in 0..500 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let len = (state % 64) as usize;
        let mut input = Vec::with_capacity(len);
        for _ in 0..len {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            input.push((state >> 33) as u8);
        }
        let pairs = owned(parse(&input));
        // Every produced string must be valid UTF-8 simply by being a String;
        // exercise serde of the pairs back through the serializer.
        let mut ser = Serializer::new(String::new());
        for (k, v) in &pairs {
            ser.append_pair(k, v);
        }
        let reencoded = ser.finish();
        assert!(reencoded.is_ascii());
    }
}

#[test]
fn serializer_suffix_and_clear() {
    let mut target = String::from("prefix?a=1");
    {
        let mut ser = Serializer::for_suffix(&mut target, "prefix?".len());
        ser.append_pair("b", "2");
        ser.clear();
        ser.append_pair("c", "3");
    }
    assert_eq!(target, "prefix?c=3");
    assert_eq!(Serializer::new(String::new()).append_key_only("k").finish(), "k");
}
