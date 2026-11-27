//! Miri-focused audit tests: pure-Rust unsafe paths only (no SIMD).
//!
//! Run with:
//! `cargo +nightly miri test -p json --test audit_json_miri`

#![cfg(feature = "alloc")]

use std::cell::Cell;

use json::{Document, Number, Options, RawValue, Value};

// Thread-local so tests running in parallel cannot perturb each other's counts.
thread_local! {
    static DROPS: Cell<usize> = const { Cell::new(0) };
}

fn bump_drops() {
    DROPS.with(|drops| drops.set(drops.get() + 1));
}

fn reset_drops() {
    DROPS.with(|drops| drops.set(0));
}

fn drops() -> usize {
    DROPS.with(Cell::get)
}

struct Tracked(i64);

impl Drop for Tracked {
    fn drop(&mut self) {
        bump_drops();
    }
}

impl<'a> json::Decode<'a> for Tracked {
    fn decode(decoder: &mut json::Decoder<'a>) -> json::Result<Self> {
        Ok(Tracked(decoder.decode_i64()?))
    }
}

#[test]
fn array_maybeuninit_success_and_error_at_each_element() {
    // Success: N initialised values, N drops.
    reset_drops();
    {
        let value: [Tracked; 2] = json::from_str("[10,20]").unwrap();
        assert_eq!(value[0].0, 10);
        assert_eq!(value[1].0, 20);
    }
    assert_eq!(drops(), 2);

    // Error at element k: exactly k drops, no double drop, no leak of an
    // uninitialised slot.
    let inputs = [
        ("[]", 0usize),
        ("[\"bad\"]", 0),
        ("[1,\"bad\"]", 1),
        ("[1,2,\"bad\"]", 2),
    ];
    for (input, initialized) in inputs {
        reset_drops();
        assert!(json::from_str::<[Tracked; 3]>(input).is_err(), "{input}");
        assert_eq!(drops(), initialized, "{input}");
    }

    // Too many elements.
    reset_drops();
    assert!(json::from_str::<[Tracked; 2]>("[1,2,3]").is_err());
    assert_eq!(drops(), 2);

    // Nested arrays: the failing inner array drops its prefix, then the outer
    // array's successful prefix is dropped during unwinding of the error path.
    reset_drops();
    assert!(json::from_str::<[[Tracked; 2]; 2]>("[[1,2],[3,\"bad\"]]").is_err());
    assert_eq!(drops(), 3);

    // N = 0 and N = 1 boundaries. A zero-length array has no drop glue.
    reset_drops();
    let _empty: [Tracked; 0] = json::from_str("[]").unwrap();
    assert!(json::from_str::<[Tracked; 0]>("[1]").is_err());
    assert_eq!(drops(), 0);
    reset_drops();
    let one: [Tracked; 1] = json::from_str("[7]").unwrap();
    assert_eq!(one.len(), 1);
    drop(one);
    assert_eq!(drops(), 1);

    // Larger N.
    reset_drops();
    let mut input = String::from("[");
    for i in 0..32 {
        if i != 0 {
            input.push(',');
        }
        input.push_str(&i.to_string());
    }
    input.push(']');
    let large: [Tracked; 32] = json::from_str(&input).unwrap();
    assert_eq!(large[31].0, 31);
    drop(large);
    assert_eq!(drops(), 32);
}

#[test]
fn array_panic_during_decode_drops_initialised_prefix() {
    // The element type allocates so Miri's leak checker also observes any
    // leaked value, in addition to the drop counter.
    struct Leaky(String);
    impl Drop for Leaky {
        fn drop(&mut self) {
            bump_drops();
            assert!(!self.0.is_empty());
        }
    }
    impl<'a> json::Decode<'a> for Leaky {
        fn decode(decoder: &mut json::Decoder<'a>) -> json::Result<Self> {
            let value = decoder.decode_i64()?;
            if value == 9 {
                panic!("intentional panic inside Decode");
            }
            Ok(Leaky("x".repeat(value as usize)))
        }
    }

    reset_drops();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: [Leaky; 4] = json::from_str("[1,2,9,4]").unwrap();
    }));
    assert!(result.is_err());
    assert_eq!(drops(), 2, "already decoded elements must be dropped exactly once");
}

#[test]
fn string_bodies_from_str_are_valid_utf8() {
    // Exercises the unchecked `input_is_utf8` branch: nulls via escapes,
    // surrogate pairs, multibyte characters adjacent to quotes and
    // backslashes, and an escaped final quote.
    let cases: &[(&str, &str)] = &[
        (r#""""#, ""),
        (r#""plain""#, "plain"),
        (r#""\u0000""#, "\u{0000}"),
        (r#""a\u0000b""#, "a\u{0000}b"),
        (r#""\uD83D\uDE00""#, "😀"),
        (r#""é""#, "é"),
        (r#""é\n€""#, "é\n€"),
        (r#""\é""#, "\\é"), // invalid escape: error, but no UB
        (r#""😀\\""#, "😀\\"),
        (r#""\u0000\u001F\u007F""#, "\u{0000}\u{001F}\u{007F}"),
        (r#""\uD834\uDD1E""#, "\u{1D11E}"),
        (r#""a\"b""#, "a\"b"),
        (r#""\\""#, "\\"),
        (r#""\uDC00""#, ""), // lone low surrogate: error
        (r#""\uD800""#, ""), // lone high surrogate: error
    ];
    for &(literal, expected) in cases {
        let from_str = json::from_str::<String>(literal);
        let from_slice = json::from_slice::<String>(literal.as_bytes());
        let from_value = json::from_str::<Value>(literal);
        let document = Document::parse(literal.as_bytes());
        match expected.chars().next() {
            _ if literal.contains(r"\uDC00") || literal.contains(r"\uD800") || literal == r#""\é""# => {
                assert!(from_str.is_err(), "{literal}");
                assert!(from_slice.is_err(), "{literal}");
                assert!(from_value.is_err(), "{literal}");
                assert!(document.is_err(), "{literal}");
            }
            _ => {
                assert_eq!(from_str.as_deref(), Ok(expected), "{literal}");
                assert_eq!(from_slice.as_deref(), Ok(expected), "{literal}");
                assert_eq!(from_value.unwrap().as_str(), Some(expected));
                let document = document.unwrap();
                if literal.contains('\\') {
                    assert!(document.root().as_str().is_err(), "{literal}");
                } else {
                    assert_eq!(document.root().as_str().unwrap(), Some(expected));
                }
                let mut buffer = vec![0u8; expected.len()];
                assert_eq!(document.root().string_into(&mut buffer).unwrap(), expected);
            }
        }
    }
}

#[test]
fn string_into_and_cow_paths() {
    let input = r#""a\nb\u0000c\uD83D\uDE00é""#;
    let expected = "a\nb\u{0000}c😀é";
    let document = Document::parse(input.as_bytes()).unwrap();
    let mut buffer = vec![0u8; expected.len()];
    assert_eq!(document.root().string_into(&mut buffer).unwrap(), expected);
    let mut decoder = json::Decoder::from_str_input(input);
    assert_eq!(decoder.decode_cow_str().unwrap(), expected);
    let mut decoder = json::Decoder::from_str_input(input);
    let mut out = String::new();
    decoder.decode_string_into(&mut out).unwrap();
    assert_eq!(out, expected);
    // Borrowed (no-escape) path.
    let mut decoder = json::Decoder::from_str_input(r#""borrowed""#);
    assert_eq!(decoder.decode_cow_str().unwrap(), "borrowed");
    let mut decoder = json::Decoder::from_str_input(r#""borrowed""#);
    assert_eq!(decoder.decode_str().unwrap(), "borrowed");
}

#[test]
fn number_paths_do_not_use_invalid_slices() {
    // These hit `scan_fused` (unchecked `from_utf8_unchecked`) and must only
    // ever consume ASCII. Malformed and non-ASCII tails are included.
    let numbers = [
        "0",
        "-0",
        "1",
        "-1",
        "1234567890123456789",
        "12345678901234567890",
        "18446744073709551616",
        "-9223372036854775808",
        "-9223372036854775809",
        "1.5",
        "-1.5e10",
        "1e309",
        "5e-324",
        "0.00000000000000000000000000000001",
        "1e-400",
        "123456789012345678901234567890.5e-7",
    ];
    for text in numbers {
        let _ = json::from_str::<f64>(text);
        let _ = json::from_str::<Number>(text);
        let _ = json::from_str::<u64>(text);
        let _ = json::from_str::<i64>(text);
        let _ = json::from_str::<u128>(text);
        let _ = json::from_str::<i128>(text);
        let _ = json::from_slice_with_options::<Number>(
            text.as_bytes(),
            Options {
                big_numbers: json::BigNumberPolicy::Error,
                ..Options::default()
            },
        );
        if let Ok(raw) = json::RawNumber::new(text) {
            let _ = raw.as_f64();
            let _ = raw.as_u64();
            let _ = raw.as_i64();
        }
    }
    // Non-ASCII directly after a number must never be included in the slice.
    for text in ["1é", "1\u{1F600}", "1.5é", "1e5é", "1,", "1]"] {
        let _ = json::from_str::<f64>(text);
        let _ = json::from_str::<Number>(text);
        let _ = json::validate(text.as_bytes());
    }
    // A Decoder positioned mid-character must not produce a &str from it.
    let mut decoder = json::Decoder::from_str_input("01é");
    decoder.set_position(2);
    assert!(decoder.decode_str().is_err());
}

#[test]
fn document_and_raw_value_lifetimes() {
    let input = br#"{"a":[1,2,{"b":"\u00e9"}],"c":null}"#;
    let _ = json::validate(input);
    let document = Document::parse(input).unwrap();
    let value = document.get("/a/2/b").unwrap().unwrap();
    let mut buffer = [0u8; 2];
    assert_eq!(value.string_into(&mut buffer).unwrap(), "é");
    let raw = RawValue::new(&input[..]).unwrap();
    assert_eq!(raw.as_str().unwrap(), core::str::from_utf8(input).unwrap());
    let owned = raw.into_owned();
    assert_eq!(owned.as_str().unwrap(), core::str::from_utf8(input).unwrap());
    let _: Value = json::from_slice(input).unwrap();
}
