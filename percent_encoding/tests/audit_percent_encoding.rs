//! Audit tests for `percent_encoding`.
//!
//! The crate has two unchecked `str::from_utf8_unchecked` sites whose safety
//! hinges on one invariant: every byte that may appear in an "unchanged" slice
//! must be ASCII. `should_percent_encode` is the only gate, so these tests
//! exhaustively check every byte value against several sets, including ones
//! built with `add`/`remove`/`union`/`complement`.

use std::{
    borrow::Cow,
    panic::{AssertUnwindSafe, catch_unwind},
};

use percent_encoding::{
    AsciiSet, CONTROLS, NON_ALPHANUMERIC, percent_decode, percent_decode_str, percent_encode, percent_encode_byte,
    utf8_percent_encode,
};

/// Independent reference encoder. Membership is given explicitly so it does not
/// reuse the crate's bitmap logic.
fn ref_encode(input: &[u8], encode: impl Fn(u8) -> bool) -> String {
    let mut out = String::new();
    for &b in input {
        if encode(b) {
            out.push_str(&format!("%{b:02X}"));
        } else {
            assert!(b.is_ascii(), "reference must only leave ASCII unchanged, got {b:#04x}");
            out.push(b as char);
        }
    }
    out
}

fn all_bytes() -> Vec<u8> {
    (0..=255).collect()
}

const ALL_ASCII: AsciiSet = {
    let mut set = AsciiSet::EMPTY;
    let mut b = 0u8;
    while b < 0x80 {
        set = set.add(b);
        b += 1;
    }
    set
};
const COMPLEMENT_EMPTY: AsciiSet = AsciiSet::EMPTY.complement();
const NOT_NON_ALPHANUMERIC: AsciiSet = NON_ALPHANUMERIC.complement();
const NON_ALNUM_MINUS_NUL: AsciiSet = NON_ALPHANUMERIC.remove(0);
const CONTROLS_MINUS_NUL: AsciiSet = CONTROLS.remove(0);
const CONTROLS_PLUS_SPACE: AsciiSet = CONTROLS.add(b' ');

fn check_set(name: &str, set: &'static AsciiSet, encode: impl Fn(u8) -> bool + Copy) {
    // Every byte on its own, so an incorrect branch cannot be masked by a
    // neighbouring byte.
    for b in 0..=255u8 {
        let got = percent_encode(&[b], set).collect::<String>();
        let want = ref_encode(&[b], encode);
        assert_eq!(got, want, "{name}: single byte {b:#04x}");
        assert!(got.is_ascii(), "{name}: non-ASCII output for {b:#04x}");
        assert_eq!(
            percent_decode(got.as_bytes()).collect::<Vec<_>>(),
            vec![b],
            "{name}: decode {b:#04x}"
        );
    }

    // The whole byte range in one input exercises transitions at every value.
    let input = all_bytes();
    let got = percent_encode(&input, set).collect::<String>();
    let want = ref_encode(&input, encode);
    assert_eq!(got, want, "{name}: all bytes");
    assert_eq!(
        percent_decode(got.as_bytes()).collect::<Vec<_>>(),
        input,
        "{name}: all-bytes roundtrip"
    );

    // The Cow conversion and Display paths use the same iterator.
    let cow: Cow<'_, str> = percent_encode(&input, set).into();
    assert_eq!(cow, got, "{name}: Cow");
    assert_eq!(percent_encode(&input, set).to_string(), got, "{name}: Display");
}

#[test]
fn every_byte_every_set() {
    check_set("EMPTY", &AsciiSet::EMPTY, |b| !b.is_ascii());
    check_set("CONTROLS", CONTROLS, |b| b >= 0x80 || !(0x20..0x7f).contains(&b));
    check_set("NON_ALPHANUMERIC", NON_ALPHANUMERIC, |b| {
        b >= 0x80 || !(b.is_ascii_alphanumeric())
    });

    // A set containing every ASCII byte.
    check_set("ALL_ASCII", &ALL_ASCII, |_| true);

    // Complement: everything is in the set, so every byte is encoded.
    check_set("COMPLEMENT_EMPTY", &COMPLEMENT_EMPTY, |_| true);

    // Complement of NON_ALPHANUMERIC: only ASCII alphanumerics are encoded,
    // and non-ASCII must still be encoded.
    check_set("NOT_NON_ALPHANUMERIC", &NOT_NON_ALPHANUMERIC, |b| {
        b >= 0x80 || b.is_ascii_alphanumeric()
    });

    // `remove` only affects ASCII and cannot disable non-ASCII encoding.
    check_set("NON_ALNUM_MINUS_NUL", &NON_ALNUM_MINUS_NUL, |b| {
        b >= 0x80 || (b != 0 && !b.is_ascii_alphanumeric())
    });
    check_set("CONTROLS_MINUS_NUL", &CONTROLS_MINUS_NUL, |b| {
        b >= 0x80 || (b != 0 && !(0x20..0x7f).contains(&b))
    });
    check_set("CONTROLS_PLUS_space", &CONTROLS_PLUS_SPACE, |b| {
        b >= 0x80 || !(0x20..0x7f).contains(&b) || b == b' '
    });

    // Union keeps both halves.
    const UNION_AB: AsciiSet = AsciiSet::EMPTY.add(b'a').union(AsciiSet::EMPTY.add(b'b'));
    check_set("UNION_a_b", &UNION_AB, |b| b >= 0x80 || b == b'a' || b == b'b');
}

#[test]
fn empty_and_ascii_only_inputs_borrow() {
    for b in [0u8, b'a', 0x7f] {
        let input = [b];
        let cow: Cow<'_, str> = percent_encode(&input, &AsciiSet::EMPTY).into();
        assert!(matches!(cow, Cow::Borrowed(_)), "byte {b:#04x} should borrow");
    }
    let empty: Cow<'_, str> = percent_encode(b"", &AsciiSet::EMPTY).into();
    assert_eq!(empty, "");
    assert_eq!(percent_encode(b"", NON_ALPHANUMERIC).count(), 0);
    assert_eq!(percent_encode(b"", NON_ALPHANUMERIC).size_hint(), (0, Some(0)));
}

#[test]
fn utf8_input_roundtrips() {
    let samples = [
        "",
        "ascii only",
        "é",
        "aé漢💖",
        "nul\0tab\tnewline\n",
        "reserved :/?#[]@!$&'()*+,;=%",
        "\u{10FFFF}",
    ];
    for s in samples {
        for set in [&AsciiSet::EMPTY, CONTROLS, NON_ALPHANUMERIC] {
            let encoded = utf8_percent_encode(s, set).collect::<String>();
            assert!(encoded.is_ascii(), "encoded form must be ASCII: {encoded:?}");
            let decoded = percent_decode_str(&encoded).decode_utf8().unwrap();
            assert_eq!(decoded, s);
        }
    }
}

#[test]
fn arbitrary_bytes_never_produce_invalid_utf8_fragments() {
    // Deterministic LCG over all byte values, including invalid UTF-8.
    let mut state = 0x1234_5678_9abc_def0u64;
    for _ in 0..200 {
        let len = (state % 33) as usize;
        let mut input = Vec::with_capacity(len);
        for _ in 0..len {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            input.push((state >> 33) as u8);
        }
        for set in [&AsciiSet::EMPTY, CONTROLS, NON_ALPHANUMERIC] {
            let encoded = percent_encode(&input, set).collect::<String>();
            assert!(encoded.is_ascii());
            assert_eq!(percent_decode(encoded.as_bytes()).collect::<Vec<_>>(), input);
        }
    }
}

#[test]
fn percent_encode_byte_table_is_complete() {
    for b in 0..=255u8 {
        let s = percent_encode_byte(b);
        assert_eq!(s.len(), 3);
        assert_eq!(s, format!("%{b:02X}"));
        assert!(s.is_ascii());
    }
}

#[test]
fn decoding_edge_cases_do_not_panic() {
    let cases: &[(&[u8], &[u8])] = &[
        (b"", b""),
        (b"%", b"%"),
        (b"%4", b"%4"),
        (b"%zz", b"%zz"),
        (b"%41", b"A"),
        (b"%4a", b"J"),
        (b"%00", b"\0"),
        (b"%%41", b"%A"),
        (b"%C3%A9", "é".as_bytes()),
    ];
    for (input, want) in cases {
        assert_eq!(percent_decode(input).collect::<Vec<_>>(), *want, "input {input:?}");
    }
    // Truncated escapes must not consume following bytes.
    let got: Vec<u8> = percent_decode(b"%4z%").collect();
    assert_eq!(got, b"%4z%");
}

#[cfg(panic = "unwind")]
#[test]
fn add_non_ascii_panics_instead_of_ub() {
    // The public `add`/`remove` functions document an ASCII-range argument but
    // index the bitmap directly. Non-ASCII input is an out-of-bounds index, so
    // it panics (a safe failure). This is shared with upstream percent-encoding
    // 2.3.x; record it so a future unchecked rewrite keeps the boundary.
    for b in [0x80u8, 0xff] {
        let r = catch_unwind(AssertUnwindSafe(|| AsciiSet::EMPTY.add(b)));
        assert!(r.is_err(), "add({b:#04x}) should panic");
        let r = catch_unwind(AssertUnwindSafe(|| AsciiSet::EMPTY.remove(b)));
        assert!(r.is_err(), "remove({b:#04x}) should panic");
    }
}
