//! Audit tests for the `json` crate: number parsing, string decoding,
//! limits/DoS and an independent RFC 8259 reference validator.
//!
//! These tests intentionally avoid new dependencies; the pseudo-random
//! generator is a local SplitMix64 so every run is deterministic.

#![cfg(feature = "alloc")]

use json::{BigNumberPolicy, Document, DuplicateKeys, Number, Options, RawValue, Value};

// ---------------------------------------------------------------------------
// Deterministic RNG (SplitMix64)
// ---------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

// ---------------------------------------------------------------------------
// Independent reference implementation for RFC 8259 (strict, matching the
// crate's documented choices: valid UTF-8, no lone surrogates, BOM allowed at
// the start only, nesting limited to 128).
// ---------------------------------------------------------------------------

const REF_MAX_DEPTH: usize = 128;

struct RefParser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

fn ref_validate(input: &[u8]) -> bool {
    if core::str::from_utf8(input).is_err() {
        return false;
    }
    let mut parser = RefParser {
        bytes: input,
        pos: 0,
    };
    if parser.bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        parser.pos = 3;
    }
    if parser.value(0).is_err() {
        return false;
    }
    parser.ws();
    parser.pos == parser.bytes.len()
}

impl<'a> RefParser<'a> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn eat(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn value(&mut self, depth: usize) -> Result<(), ()> {
        self.ws();
        match self.peek().ok_or(())? {
            b'n' => self.literal(b"null"),
            b't' => self.literal(b"true"),
            b'f' => self.literal(b"false"),
            b'"' => self.string(),
            b'[' => {
                if depth + 1 > REF_MAX_DEPTH {
                    return Err(());
                }
                self.pos += 1;
                self.ws();
                if self.eat(b']') {
                    return Ok(());
                }
                loop {
                    self.value(depth + 1)?;
                    self.ws();
                    if self.eat(b',') {
                        continue;
                    }
                    return if self.eat(b']') { Ok(()) } else { Err(()) };
                }
            }
            b'{' => {
                if depth + 1 > REF_MAX_DEPTH {
                    return Err(());
                }
                self.pos += 1;
                self.ws();
                if self.eat(b'}') {
                    return Ok(());
                }
                loop {
                    self.ws();
                    self.string()?;
                    self.ws();
                    if !self.eat(b':') {
                        return Err(());
                    }
                    self.value(depth + 1)?;
                    self.ws();
                    if self.eat(b',') {
                        continue;
                    }
                    return if self.eat(b'}') { Ok(()) } else { Err(()) };
                }
            }
            b'-' | b'0'..=b'9' => self.number(),
            _ => Err(()),
        }
    }

    fn literal(&mut self, text: &[u8]) -> Result<(), ()> {
        if self.bytes[self.pos..].starts_with(text) {
            self.pos += text.len();
            Ok(())
        } else {
            Err(())
        }
    }

    fn hex4(&mut self) -> Result<u32, ()> {
        let mut value = 0u32;
        for _ in 0..4 {
            let byte = self.peek().ok_or(())?;
            let digit = match byte {
                b'0'..=b'9' => (byte - b'0') as u32,
                b'a'..=b'f' => (byte - b'a' + 10) as u32,
                b'A'..=b'F' => (byte - b'A' + 10) as u32,
                _ => return Err(()),
            };
            value = value * 16 + digit;
            self.pos += 1;
        }
        Ok(value)
    }

    fn string(&mut self) -> Result<(), ()> {
        if !self.eat(b'"') {
            return Err(());
        }
        loop {
            let byte = self.peek().ok_or(())?;
            match byte {
                b'"' => {
                    self.pos += 1;
                    return Ok(());
                }
                b'\\' => {
                    self.pos += 1;
                    match self.peek().ok_or(())? {
                        b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => self.pos += 1,
                        b'u' => {
                            self.pos += 1;
                            let first = self.hex4()?;
                            if (0xD800..=0xDBFF).contains(&first) {
                                if !(self.eat(b'\\') && self.eat(b'u')) {
                                    return Err(());
                                }
                                let second = self.hex4()?;
                                if !(0xDC00..=0xDFFF).contains(&second) {
                                    return Err(());
                                }
                            } else if (0xDC00..=0xDFFF).contains(&first) {
                                return Err(());
                            }
                        }
                        _ => return Err(()),
                    }
                }
                b if b < 0x20 => return Err(()),
                _ => self.pos += 1,
            }
        }
    }

    fn number(&mut self) -> Result<(), ()> {
        self.eat(b'-');
        match self.peek().ok_or(())? {
            b'0' => {
                self.pos += 1;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err(());
                }
            }
            b'1'..=b'9' => {
                self.pos += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return Err(()),
        }
        if self.eat(b'.') {
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(());
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(());
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        Ok(())
    }
}

/// Independent grammar check for one bare JSON number.
fn ref_is_number(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut parser = RefParser {
        bytes,
        pos: 0,
    };
    parser.number().is_ok() && parser.pos == bytes.len()
}

// ---------------------------------------------------------------------------
// Independent string unescape reference
// ---------------------------------------------------------------------------

/// Decodes the body of a JSON string. `None` means the body is invalid.
fn ref_unescape(body: &str) -> Option<String> {
    fn hex(bytes: &[u8], at: usize) -> Option<u32> {
        let mut value = 0;
        for i in 0..4 {
            let byte = *bytes.get(at + i)?;
            let digit = match byte {
                b'0'..=b'9' => (byte - b'0') as u32,
                b'a'..=b'f' => (byte - b'a' + 10) as u32,
                b'A'..=b'F' => (byte - b'A' + 10) as u32,
                _ => return None,
            };
            value = value * 16 + digit;
        }
        Some(value)
    }

    let bytes = body.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'\\' {
            // An unescaped quote terminates the generated string early; the
            // remaining bytes would be trailing data.
            if bytes[i] == b'"' {
                return None;
            }
            let start = i;
            while i < bytes.len() && bytes[i] != b'\\' && bytes[i] != b'"' && bytes[i] >= 0x20 {
                i += 1;
            }
            out.push_str(&body[start..i]);
            if i < bytes.len() && (bytes[i] < 0x20 || bytes[i] == b'"') {
                return None;
            }
            continue;
        }
        i += 1;
        match *bytes.get(i)? {
            b'"' => out.push('"'),
            b'\\' => out.push('\\'),
            b'/' => out.push('/'),
            b'b' => out.push('\u{0008}'),
            b'f' => out.push('\u{000C}'),
            b'n' => out.push('\n'),
            b'r' => out.push('\r'),
            b't' => out.push('\t'),
            b'u' => {
                let first = hex(bytes, i + 1)?;
                i += 5;
                let code = if (0xD800..=0xDBFF).contains(&first) {
                    if bytes.get(i) != Some(&b'\\') || bytes.get(i + 1) != Some(&b'u') {
                        return None;
                    }
                    let second = hex(bytes, i + 2)?;
                    if !(0xDC00..=0xDFFF).contains(&second) {
                        return None;
                    }
                    i += 6;
                    0x1_0000 + ((first - 0xD800) << 10) + (second - 0xDC00)
                } else if (0xDC00..=0xDFFF).contains(&first) {
                    return None;
                } else {
                    first
                };
                out.push(char::from_u32(code)?);
                continue;
            }
            _ => return None,
        }
        i += 1;
    }
    Some(out)
}

// ---------------------------------------------------------------------------
// Number parsing
// ---------------------------------------------------------------------------

fn valid_number(u: u64, mut rng: Rng) -> (String, Rng) {
    let mut text = String::new();
    let negative = rng.next_u64() & 1 == 0;
    if negative {
        text.push('-');
    }
    let int_digits = 1 + (u % 22) as usize;
    for i in 0..int_digits {
        let digit = if i == 0 && int_digits > 1 {
            1 + (rng.below(9) as u8)
        } else {
            rng.below(10) as u8
        };
        text.push((b'0' + digit) as char);
    }
    if rng.next_u64() & 1 == 0 {
        text.push('.');
        let digits = 1 + rng.below(22) as usize;
        for _ in 0..digits {
            text.push((b'0' + rng.below(10) as u8) as char);
        }
    }
    if rng.next_u64() & 3 == 0 {
        text.push(if rng.next_u64() & 1 == 0 { 'e' } else { 'E' });
        match rng.next_u64() % 3 {
            0 => text.push('+'),
            1 => text.push('-'),
            _ => {}
        }
        let digits = 1 + rng.below(4) as usize;
        for _ in 0..digits {
            text.push((b'0' + rng.below(10) as u8) as char);
        }
    }
    (text, rng)
}

#[test]
fn number_parsing_matches_stdlib() {
    let mut rng = Rng::new(0x5EED_1234_ABCD_0001);
    let mut checked = 0u32;
    for iteration in 0..200_000u64 {
        let (text, next) = valid_number(iteration, rng);
        rng = next;
        assert!(ref_is_number(&text), "generator produced invalid number {text:?}");

        // f64: must byte-for-byte match Rust's correctly rounded parser when
        // the value is finite, and must be an error when it overflows.
        let reference: f64 = text.parse().unwrap();
        let got = json::from_str::<f64>(&text);
        if reference.is_finite() {
            let got = got.unwrap_or_else(|e| panic!("{text:?}: expected {reference:?}, got {e}"));
            assert_eq!(
                got.to_bits(),
                reference.to_bits(),
                "f64 mismatch for {text:?}: {got:?} vs {reference:?}"
            );
        } else {
            assert!(got.is_err(), "expected overflow error for {text:?}, got {got:?}");
        }

        // The same path through the public Decoder and Document APIs.
        let expected_bits = if reference.is_finite() {
            Some(reference.to_bits())
        } else {
            None
        };
        let mut decoder = json::Decoder::from_str_input(&text);
        let via_decoder = decoder.decode_f64();
        assert_eq!(
            via_decoder.map(|v| v.to_bits()).ok(),
            expected_bits,
            "decode_f64 mismatch for {text:?}"
        );
        let document = Document::parse(text.as_bytes()).unwrap();
        assert_eq!(
            document.root().as_f64().ok().flatten().map(f64::to_bits),
            expected_bits,
            "Document::as_f64 mismatch for {text:?}"
        );

        // RawNumber must preserve the text and agree on f64.
        let raw = json::RawNumber::new(&text).unwrap();
        assert_eq!(raw.as_str(), text);
        if reference.is_finite() {
            assert_eq!(raw.as_f64().unwrap().to_bits(), reference.to_bits());
        } else {
            assert!(raw.as_f64().is_err());
        }

        // Number must agree with an integer reference where the value fits.
        // Very large exponents overflow f64, and `Number` then errors.
        if !reference.is_finite() {
            assert!(
                json::from_str::<Number>(&text).is_err(),
                "Number should reject f64-overflowing number {text:?}"
            );
            checked += 1;
            continue;
        }
        let parsed = json::from_str::<Number>(&text).unwrap();
        if !text.contains(['.', 'e', 'E']) {
            if let Some(rest) = text.strip_prefix('-') {
                let magnitude: u128 = rest.parse().unwrap();
                if magnitude == 0 {
                    assert!(matches!(parsed, Number::Float(v) if v.to_bits() == (-0.0f64).to_bits()));
                } else if magnitude <= i64::MAX as u128 {
                    assert_eq!(parsed.as_i64(), Some(-(magnitude as i64)));
                } else if magnitude == (i64::MAX as u128) + 1 {
                    assert_eq!(parsed.as_i64(), Some(i64::MIN));
                } else {
                    assert!(parsed.is_f64(), "big negative integer must be f64: {text}");
                }
            } else {
                let magnitude: u128 = text.parse().unwrap();
                if magnitude <= u64::MAX as u128 {
                    assert_eq!(parsed.as_u64(), Some(magnitude as u64), "{text}");
                } else {
                    assert!(parsed.is_f64(), "big integer must be f64: {text}");
                }
            }
        }

        // BigNumberPolicy::Error rejects integers outside u64/i64.
        let strict = json::from_str_with_options::<Number>(
            &text,
            Options {
                big_numbers: BigNumberPolicy::Error,
                ..Options::default()
            },
        );
        if !text.contains(['.', 'e', 'E']) {
            let fits = if let Some(rest) = text.strip_prefix('-') {
                rest.parse::<u128>().unwrap() <= (i64::MAX as u128) + 1
            } else {
                text.parse::<u128>().unwrap() <= u64::MAX as u128
            };
            assert_eq!(strict.is_ok(), fits, "BigNumberPolicy::Error for {text}");
        }

        checked += 1;
    }
    assert_eq!(checked, 200_000);
}

#[test]
fn number_edge_cases() {
    // Integers around the i64/u64 boundaries.
    let cases: &[(&str, Option<u64>, Option<i64>)] = &[
        ("0", Some(0), Some(0)),
        ("-0", None, None), // stored as Float(-0.0)
        ("18446744073709551615", Some(u64::MAX), None),
        ("18446744073709551616", None, None),
        ("9223372036854775807", Some(i64::MAX as u64), Some(i64::MAX)),
        ("9223372036854775808", Some(i64::MAX as u64 + 1), None),
        ("-9223372036854775808", None, Some(i64::MIN)),
        ("-9223372036854775809", None, None),
        ("10000000000000000000", Some(10_000_000_000_000_000_000), None),
        ("-10000000000000000000", None, None),
    ];
    for &(text, as_u64, as_i64) in cases {
        let number = json::from_str::<Number>(text).unwrap();
        assert_eq!(number.as_u64(), as_u64, "as_u64 {text}");
        assert_eq!(number.as_i64(), as_i64, "as_i64 {text}");
        assert_eq!(json::from_str::<u64>(text).ok(), as_u64, "decode u64 {text}");
        // `decode_i64` uses parse_i64_fast, which can represent -0 as 0 even
        // though `Number` stores it as a float.
        let expected_i64 = if text == "-0" { Some(0) } else { as_i64 };
        assert_eq!(json::from_str::<i64>(text).ok(), expected_i64, "decode i64 {text}");
        // `-0` is a float in this crate, matching serde_json.
        if text == "-0" {
            assert!(number.is_f64());
            assert_eq!(number.as_f64().to_bits(), (-0.0f64).to_bits());
        }
    }

    // Floating-point extremes.
    for &(text, bits) in &[
        ("0.0", 0.0f64.to_bits()),
        ("-0.0", (-0.0f64).to_bits()),
        ("5e-324", 5e-324f64.to_bits()),
        ("2.2250738585072014e-308", 2.2250738585072014e-308f64.to_bits()),
        ("1.7976931348623157e308", 1.7976931348623157e308f64.to_bits()),
        ("9007199254740993", 9007199254740992.0f64.to_bits()),
    ] {
        let got = json::from_str::<f64>(text).unwrap();
        assert_eq!(got.to_bits(), bits, "{text}");
        assert_eq!(got.to_bits(), text.parse::<f64>().unwrap().to_bits(), "{text}");
    }
    // Overflow / underflow to zero.
    assert!(json::from_str::<f64>("1e309").is_err());
    assert!(json::from_str::<f64>("-1e309").is_err());
    assert_eq!(json::from_str::<f64>("1e-400").unwrap(), 0.0);
    assert_eq!(json::from_str::<f64>("-1e-400").unwrap().to_bits(), (-0.0f64).to_bits());
    assert_eq!(json::from_str::<f64>("0e400").unwrap(), 0.0);
    // Long mantissas, with and without exponent.
    let long = format!("0.{}", "1".repeat(1000));
    assert_eq!(json::from_str::<f64>(&long).unwrap(), long.parse::<f64>().unwrap());
    let finite_long: String = core::iter::repeat_n('9', 30).collect();
    assert_eq!(
        json::from_str::<f64>(&finite_long).unwrap(),
        finite_long.parse::<f64>().unwrap()
    );
    let long_int: String = core::iter::repeat_n('9', 1000).collect();
    assert!(json::from_str::<f64>(&long_int).is_err());
    assert!(json::from_str::<f64>(&format!("1e{}", "9".repeat(1000))).is_err());
}

#[test]
fn malformed_numbers_never_panic_and_match_reference() {
    let mut rng = Rng::new(0xC0FF_EE00_1234_5678);
    let pieces = [
        "",
        "-",
        "+",
        "0",
        "00",
        "01",
        "1",
        "9",
        ".",
        "1.",
        ".1",
        "1.0",
        "1e",
        "1e+",
        "1e-",
        "1E5",
        "1e99999",
        "-0",
        "-0.0",
        "1e309",
        "1e-400",
        "18446744073709551615",
        "18446744073709551616",
        "-9223372036854775808",
        "-9223372036854775809",
        "１２３",
    ];
    let mut checked = 0u32;
    for _ in 0..200_000 {
        let mut text = String::new();
        for _ in 0..1 + rng.below(4) {
            text.push_str(pieces[rng.below(pieces.len() as u64) as usize]);
        }
        if text.is_empty() {
            continue;
        }
        let syntactic = ref_is_number(&text);
        // validate accepts a bare number only when the reference grammar does.
        assert_eq!(
            json::validate(text.as_bytes()).is_ok(),
            syntactic,
            "validate disagreement on {text:?}"
        );
        // Typed decoders must not panic; when they accept, the reference must too.
        let as_f64 = json::from_str::<f64>(&text);
        let as_u64 = json::from_str::<u64>(&text);
        let as_i64 = json::from_str::<i64>(&text);
        let as_number = json::from_str::<Number>(&text);
        if as_f64.is_ok() || as_u64.is_ok() || as_i64.is_ok() || as_number.is_ok() {
            assert!(syntactic, "typed decoder accepted malformed {text:?}");
        }
        if syntactic {
            let reference: f64 = text.parse().unwrap();
            if reference.is_finite() {
                assert_eq!(as_f64.as_ref().unwrap().to_bits(), reference.to_bits(), "f64 mismatch {text:?}");
                assert!(as_number.is_ok(), "Number rejected valid number {text:?}");
            } else {
                assert!(as_f64.is_err(), "f64 overflow accepted {text:?}");
                assert!(as_number.is_err(), "Number overflow accepted {text:?}");
            }
        }
        checked += 1;
    }
    assert!(checked > 150_000);
}

// ---------------------------------------------------------------------------
// String decoding
// ---------------------------------------------------------------------------

fn check_escaped_body(body: &str) {
    let literal = format!("\"{body}\"");
    let reference = ref_unescape(body);

    let as_string = json::from_str::<String>(&literal);
    let as_slice = json::from_slice::<String>(literal.as_bytes());
    let as_value: json::Result<Value> = json::from_str(&literal);

    match reference {
        Some(expected) => {
            assert_eq!(as_string.as_deref(), Ok(expected.as_str()), "{literal:?}");
            assert_eq!(as_slice.as_deref(), Ok(expected.as_str()), "{literal:?}");
            let value = as_value.unwrap_or_else(|e| panic!("{literal:?}: {e}"));
            assert_eq!(value.as_str(), Some(expected.as_str()));
            assert!(json::validate(literal.as_bytes()).is_ok());
            // Document / Element path (also uses the unchecked UTF-8 branch).
            let document = Document::parse(literal.as_bytes()).unwrap();
            if body.contains('\\') {
                // Escaped strings cannot be borrowed, but can be decoded into
                // a caller buffer.
                assert!(document.root().as_str().is_err(), "{literal:?}");
            } else {
                assert_eq!(document.root().as_str().unwrap(), Some(expected.as_str()));
            }
            let mut buffer = vec![0u8; expected.len()];
            assert_eq!(document.root().string_into(&mut buffer).unwrap(), expected);
        }
        None => {
            assert!(as_string.is_err(), "expected rejection: {literal:?}");
            assert!(as_slice.is_err(), "expected rejection: {literal:?}");
            assert!(as_value.is_err(), "expected rejection: {literal:?}");
            assert!(json::validate(literal.as_bytes()).is_err());
        }
    }
}

#[test]
fn string_escapes_curated() {
    let bodies = [
        r#""#,
        r#"plain"#,
        r#"with space"#,
        "üñïçø∂é",
        "日本語",
        "😀",
        "\u{0000}",
        "a\u{0000}b",
        r#"\"",
        r#"\\"#,
        r#"\/"#,
        r#"\b"#,
        r#"\f"#,
        r#"\n"#,
        r#"\r"#,
        r#"\t"#,
        r#"\u0041"#,
        r#"\u00e9"#,
        r#"\u0000"#,
        r#"\uD83D\uDE00"#,
        r#"\ud83d\ude00"#,
        r#"\uD834\uDD1E"#,
        r#"a\nb\tc"#,
        r#"a\\b\"c"#,
        r#"\u0041\u0042\u0043"#,
        r#"\uD83D"#,
        r#"\uDE00"#,
        r#"\uD83Dx"#,
        r#"\uD83D\u0041"#,
        r#"\uD83D\uD83D"#,
        r#"\u12"#,
        r#"\u12G4"#,
        r#"\u"#,
        r#"\x"#,
        r#"\'"#,
        r#"a\"#,
        r#"\ud83d\ude00\ud83d\ude00"#,
        r#"\\uD83D\\uDE00"#,
        r#"\u0001\u001F"#,
        "\u{7F}",
        "\u{80}\u{7FF}\u{800}\u{FFFF}",
        "é\\",
    ];
    for body in bodies {
        check_escaped_body(body);
    }
}

#[test]
fn raw_control_and_quote_handling() {
    // Raw control characters are rejected.
    for byte in 0..0x20u8 {
        let literal = [b'"', byte, b'"'];
        assert!(json::validate(&literal).is_err(), "raw control 0x{byte:02X} accepted");
        assert!(json::from_slice::<String>(&literal).is_err());
    }
    // 0x7F and 0x80.. are allowed in strings.
    for byte in [0x7Fu8, 0x80, 0xC2, 0xA9] {
        let literal = [b'"', byte, b'"'];
        let _ = json::validate(&literal);
        let _ = json::from_slice::<String>(&literal);
    }
    // Multibyte characters adjacent to quotes/backslashes.
    for body in ["é", "ééé", "\\", "é\\", "\\é", "€\\n", "\u{10FFFF}"] {
        check_escaped_body(body);
    }
    // An escaped multibyte character is an invalid escape but must never be UB
    // (this exercises scan_body skipping one byte into a UTF-8 sequence).
    for text in [r#""\é""#, r#""\€""#, r#""\😀""#] {
        assert!(json::from_str::<String>(text).is_err());
        assert!(json::validate(text.as_bytes()).is_err());
        assert!(json::from_slice::<Value>(text.as_bytes()).is_err());
    }
}

#[test]
fn invalid_utf8_from_slice_is_rejected_without_panic() {
    let cases: &[&[u8]] = &[
        b"\"\xFF\"",
        b"\"\xC0\xAF\"",         // overlong '/'
        b"\"\xE0\x80\x80\"",     // overlong NUL
        b"\"\xED\xA0\x80\"",     // UTF-8 encoded surrogate
        b"\"\xF4\x90\x80\x80\"", // > U+10FFFF
        b"\"\xC2\"",             // truncated
        b"[\"\xFF\"]",
        b"\"a\x00b\"", // embedded NUL is valid UTF-8 but a control char
        b"\xEF\xBB\xBF\"ok\"",
    ];
    for &case in cases {
        let _ = json::validate(case);
        let _ = json::from_slice::<String>(case);
        let _ = json::from_slice::<Value>(case);
        let _ = Document::parse(case);
        let _ = RawValue::new(case);
    }
    assert!(json::from_slice::<String>(b"\"\xFF\"").is_err());
    assert!(json::from_slice::<String>(b"\"\xC0\xAF\"").is_err());
    // The same bytes via a &str cannot exist; from_str is inherently valid UTF-8.
    assert!(json::from_slice::<String>("\"\u{FFFD}\"".as_bytes()).is_ok());
}

// ---------------------------------------------------------------------------
// Whole-document differential fuzz against the reference validator
// ---------------------------------------------------------------------------

const ALPHABET: &[u8] = b"{}[]\",:\\/ \t\n\r0123456789.eE+-truefalsn\x00\x1f\x7f\xc3\xa9\xff\xef";

fn random_document(rng: &mut Rng) -> Vec<u8> {
    let len = rng.below(48) as usize;
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
        let choice = rng.below(100);
        if choice < 92 {
            bytes.push(ALPHABET[rng.below(ALPHABET.len() as u64) as usize]);
        } else {
            bytes.push(rng.below(256) as u8);
        }
    }
    bytes
}

fn exercise_all_entry_points(bytes: &[u8]) {
    let valid = ref_validate(bytes);
    assert_eq!(json::validate(bytes).is_ok(), valid, "validate {bytes:?}");
    assert_eq!(
        json::validate_with_options(bytes, Options::default()).is_ok(),
        valid,
        "validate_with_options {bytes:?}"
    );
    assert_eq!(Document::parse(bytes).is_ok(), valid, "Document::parse {bytes:?}");
    assert_eq!(RawValue::new(bytes).is_ok(), valid, "RawValue::new {bytes:?}");
    assert_eq!(json::RawValueBuf::new(bytes).is_ok(), valid, "RawValueBuf::new {bytes:?}");
    // Untyped decoding must never panic and must agree with validation for
    // RawValue (numbers are captured lexically).
    let value = json::from_slice::<Value>(bytes);
    if value.is_ok() {
        assert!(valid, "Value accepted invalid document {bytes:?}");
    }
    let _ = json::from_slice::<String>(bytes);
    let _ = json::from_slice::<f64>(bytes);
    let _ = json::from_slice::<u64>(bytes);
    let _ = json::from_slice::<i64>(bytes);
    let _ = json::from_slice::<Vec<Value>>(bytes);
    let _ = json::from_slice::<std::collections::BTreeMap<String, Value>>(bytes);
    let _ = json::from_slice::<[i32; 3]>(bytes);
    let _ = json::from_slice::<(i32, String)>(bytes);
}

#[test]
fn random_documents_never_panic_and_match_reference() {
    let iterations = if cfg!(miri) { 400 } else { 120_000 };
    let mut rng = Rng::new(0x00C0_FFEE_5EED_0007);
    for iteration in 0..iterations {
        let bytes = random_document(&mut rng);
        // Ensure truncation variants of a valid prefix are also covered.
        exercise_all_entry_points(&bytes);
        if iteration % 16 == 0 {
            exercise_all_entry_points(&bytes[..bytes.len() / 2]);
        }
    }
}

#[test]
fn structured_random_documents_match_reference() {
    // Mostly well-formed documents built recursively, then mutated, so the
    // reference validator and the crate agree on deep paths too.
    fn build(rng: &mut Rng, depth: usize, out: &mut String) {
        let choice = rng.below(if depth > 6 { 5 } else { 7 });
        match choice {
            0 => out.push_str("null"),
            1 => out.push_str(if rng.next_u64() & 1 == 0 { "true" } else { "false" }),
            2 => {
                let number = rng.next_u64() % 1_000_000;
                if rng.next_u64() & 1 == 0 {
                    out.push_str(&number.to_string());
                } else {
                    out.push_str(&format!("{number}.{}", rng.below(1000)));
                }
            }
            3 | 4 => {
                out.push('"');
                let len = rng.below(5);
                for _ in 0..len {
                    match rng.below(6) {
                        0 => out.push('a'),
                        1 => out.push('é'),
                        2 => out.push_str("\\n"),
                        3 => out.push_str("\\u00e9"),
                        4 => out.push_str("\\\""),
                        _ => out.push_str("\\\\"),
                    }
                }
                out.push('"');
            }
            5 => {
                out.push('[');
                let len = rng.below(4);
                for i in 0..len {
                    if i != 0 {
                        out.push(',');
                    }
                    build(rng, depth + 1, out);
                }
                out.push(']');
            }
            _ => {
                out.push('{');
                let len = rng.below(4);
                for i in 0..len {
                    if i != 0 {
                        out.push(',');
                    }
                    out.push_str("\"k");
                    out.push((b'0' + i as u8) as char);
                    out.push_str("\":");
                    build(rng, depth + 1, out);
                }
                out.push('}');
            }
        }
    }

    let mut rng = Rng::new(0xABAD_1DEA_2024_0001);
    for _ in 0..20_000 {
        let mut text = String::new();
        build(&mut rng, 0, &mut text);
        assert!(ref_validate(text.as_bytes()), "reference rejected {text:?}");
        assert!(json::validate(text.as_bytes()).is_ok(), "crate rejected {text:?}");
        assert!(json::from_slice::<Value>(text.as_bytes()).is_ok(), "{text:?}");
        // Mutate one byte; either both accept or both reject.
        if !text.is_empty() {
            let mut bytes = text.clone().into_bytes();
            let at = rng.below(bytes.len() as u64) as usize;
            bytes[at] = ALPHABET[rng.below(ALPHABET.len() as u64) as usize];
            exercise_all_entry_points(&bytes);
        }
    }
}

#[test]
fn random_string_bodies_match_reference() {
    // Bodies exclude raw quotes and raw controls (those have their own test),
    // but are dense in backslashes, valid and invalid escapes, multibyte
    // characters and block-boundary lengths.
    let alphabet: &[&str] = &[
        "a", "b", " ", "é", "€", "😀", "\\", "/", "\\n", "\\t", "\\\"", "\\\\", "\\u", "0041", "D83D", "DE00", "uD834",
        "uDD1E", "bfnrt", "01", "9", "AF", "G", "x",
    ];
    let iterations = if cfg!(miri) { 500 } else { 200_000 };
    let mut rng = Rng::new(0x51A1_1E55_0000_0001);
    for _ in 0..iterations {
        let parts = rng.below(24) as usize;
        let mut body = String::new();
        for _ in 0..parts {
            body.push_str(alphabet[rng.below(alphabet.len() as u64) as usize]);
        }
        // Cap the length so the test stays fast but still crosses boundaries.
        if body.len() > 120 {
            continue;
        }
        let literal = format!("\"{body}\"");
        let expected = ref_unescape(&body);
        let decoded = json::from_slice::<String>(literal.as_bytes());
        match expected {
            Some(expected) => {
                assert_eq!(decoded.as_deref(), Ok(expected.as_str()), "{literal:?}");
                assert!(json::validate(literal.as_bytes()).is_ok(), "{literal:?}");
                let value: Value = json::from_slice(literal.as_bytes()).unwrap();
                assert_eq!(value.as_str(), Some(expected.as_str()), "{literal:?}");
            }
            None => {
                assert!(decoded.is_err(), "accepted invalid {literal:?}");
                assert!(json::validate(literal.as_bytes()).is_err(), "validated invalid {literal:?}");
                assert!(
                    json::from_slice::<Value>(literal.as_bytes()).is_err(),
                    "Value accepted invalid {literal:?}"
                );
            }
        }
    }
}

#[test]
fn exhaustive_two_byte_string_bodies() {
    // Every 2-byte body inside a quoted string, compared with the independent
    // reference. This covers all UTF-8 acceptance/rejection, quote/backslash
    // termination and control-byte rejection for short bodies.
    for a in 0..=255u8 {
        for b in 0..=255u8 {
            let literal = [b'"', a, b, b'"'];
            assert_eq!(
                json::validate(&literal).is_ok(),
                ref_validate(&literal),
                "validate 0x{a:02X} 0x{b:02X}"
            );
            let decoded = json::from_slice::<String>(&literal);
            if let Ok(text) = core::str::from_utf8(&[a, b]) {
                let special = [a, b].iter().any(|&byte| byte == b'"' || byte == b'\\' || byte < 0x20);
                if !special {
                    assert_eq!(decoded.as_deref(), Ok(text), "0x{a:02X} 0x{b:02X}");
                }
            } else {
                assert!(decoded.is_err(), "0x{a:02X} 0x{b:02X}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Limits and DoS boundaries
// ---------------------------------------------------------------------------

fn nested(depth: usize) -> String {
    let mut text = String::with_capacity(depth * 2 + 1);
    for _ in 0..depth {
        text.push('[');
    }
    text.push('0');
    for _ in 0..depth {
        text.push(']');
    }
    text
}

#[test]
fn max_depth_boundaries() {
    for depth in [0usize, 1, 2, 127, 128, 129, 130, 200] {
        let text = nested(depth);
        for limit in [0usize, 1, 2, 127, 128, 129, 130, 200, usize::MAX] {
            let options = Options {
                max_depth: limit,
                ..Options::default()
            };
            let expected = depth <= limit;
            assert_eq!(
                json::validate_with_options(text.as_bytes(), options).is_ok(),
                expected,
                "depth={depth} limit={limit}"
            );
            assert_eq!(
                json::from_slice_with_options::<Value>(text.as_bytes(), options).is_ok(),
                expected,
                "Value depth={depth} limit={limit}"
            );
        }
    }
    // Default limit is 128.
    assert!(json::validate(nested(128).as_bytes()).is_ok());
    assert!(json::validate(nested(129).as_bytes()).is_err());
    // A limit of usize::MAX must not itself panic at moderate depth.
    let options = Options {
        max_depth: usize::MAX,
        ..Options::default()
    };
    assert!(json::validate_with_options(nested(200).as_bytes(), options).is_ok());
    assert!(json::from_slice_with_options::<Value>(nested(200).as_bytes(), options).is_ok());
}

#[test]
fn max_string_len_boundaries() {
    for limit in [0usize, 1, 2, 63, 64, 65, 127, 128, 129, usize::MAX] {
        let options = Options {
            max_string_len: Some(limit),
            ..Options::default()
        };
        for len in [0usize, 1, 2, 63, 64, 65, 127, 128, 129] {
            let body = "a".repeat(len);
            let text = format!("\"{body}\"");
            let expected = len <= limit;
            assert_eq!(
                json::validate_with_options(text.as_bytes(), options).is_ok(),
                expected,
                "validate len={len} limit={limit}"
            );
            assert_eq!(
                json::from_slice_with_options::<String>(text.as_bytes(), options).is_ok(),
                expected,
                "String len={len} limit={limit}"
            );
            assert_eq!(
                json::from_slice_with_options::<Value>(text.as_bytes(), options).is_ok(),
                expected,
                "Value len={len} limit={limit}"
            );
        }
        // Escapes count their raw representation against the limit.
        for (body, raw_len) in [(r"\n", 2usize), (r"\u0041", 6), (r"\uD83D\uDE00", 12)] {
            let text = format!("\"{body}\"");
            assert_eq!(
                json::validate_with_options(text.as_bytes(), options).is_ok(),
                raw_len <= limit,
                "escape {body:?} raw_len={raw_len} limit={limit}"
            );
        }
    }
}

#[test]
fn max_container_entries_boundaries() {
    for limit in [0usize, 1, 2, 3, 4, 127, 128, 129, usize::MAX] {
        let options = Options {
            max_container_entries: Some(limit),
            ..Options::default()
        };
        for count in [0usize, 1, 2, 3, 4, 128, 129] {
            let array = format!("[{}]", (0..count).map(|i| i.to_string()).collect::<Vec<_>>().join(","));
            assert_eq!(
                json::validate_with_options(array.as_bytes(), options).is_ok(),
                count <= limit,
                "array count={count} limit={limit}"
            );
            let object = format!(
                "{{{}}}",
                (0..count).map(|i| format!("\"k{i}\":0")).collect::<Vec<_>>().join(",")
            );
            assert_eq!(
                json::validate_with_options(object.as_bytes(), options).is_ok(),
                count <= limit,
                "object count={count} limit={limit}"
            );
        }
    }
    // Entry limits above the fixed 128-entry counter table are rejected for
    // documents deeper than 128, as documented.
    let limit = Options {
        max_container_entries: Some(usize::MAX),
        max_depth: 300,
        ..Options::default()
    };
    let mut text = String::new();
    for _ in 0..140 {
        text.push('[');
    }
    text.push('0');
    for _ in 0..140 {
        text.push(']');
    }
    assert!(json::validate_with_options(text.as_bytes(), limit).is_err());
}

#[test]
fn duplicate_key_policies() {
    let text = r#"{"a":1,"b":2,"a":3}"#;
    let last: Value = json::from_str(text).unwrap();
    assert_eq!(last["a"].as_i64(), Some(3));
    let first: Value = json::from_str_with_options(
        text,
        Options {
            duplicate_keys: DuplicateKeys::FirstWins,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(first["a"].as_i64(), Some(1));
    let rejected: json::Result<Value> = json::from_str_with_options(
        text,
        Options {
            duplicate_keys: DuplicateKeys::Reject,
            ..Options::default()
        },
    );
    assert!(rejected.is_err());
    // `validate` is structural and does not apply the duplicate policy.
    assert!(
        json::validate_with_options(
            text.as_bytes(),
            Options {
                duplicate_keys: DuplicateKeys::Reject,
                ..Options::default()
            }
        )
        .is_ok()
    );
}

#[test]
fn relaxed_depth_limit_stack_overflow_is_reachable() {
    // The child process is this same test binary, filtered to the worker test.
    // A stack overflow aborts the process, so it must run out of process.
    if std::env::var_os("STDX_JSON_DEEP_CHILD").is_some() {
        return;
    }
    let exe = std::env::current_exe().unwrap();
    let status = std::process::Command::new(exe)
        .arg("relaxed_depth_limit_child_worker")
        .arg("--exact")
        .arg("--nocapture")
        .env("STDX_JSON_DEEP_CHILD", "1")
        .status()
        .expect("spawn child test process");
    assert!(
        !status.success(),
        "expected stack overflow/abort with max_depth=usize::MAX, child exited {:?}",
        status
    );
}

#[test]
fn relaxed_depth_limit_child_worker() {
    if std::env::var_os("STDX_JSON_DEEP_CHILD").is_none() {
        return;
    }
    let options = Options {
        max_depth: usize::MAX,
        ..Options::default()
    };
    let depth = std::env::var("STDX_JSON_DEPTH")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(300_000);
    let text = nested(depth);
    let _ = json::from_slice_with_options::<Value>(text.as_bytes(), options);
}

#[test]
fn default_depth_limit_rejects_deep_input_without_recursing() {
    // 1M nesting levels at default options must be rejected by the depth check
    // after at most 128 recursive calls, not overflow the stack.
    let text = nested(1_000_000);
    assert!(json::validate(text.as_bytes()).is_err());
    assert!(json::from_slice::<Value>(text.as_bytes()).is_err());
}

#[test]
fn all_unicode_scalars_round_trip() {
    // Every Unicode scalar value as a raw (unescaped) string body. This
    // exercises the unchecked `input_is_utf8` substring proof at every UTF-8
    // sequence length, including characters adjacent to the quotes.
    let mut checked = 0u32;
    for code in 0..=0x10FFFFu32 {
        let Some(ch) = char::from_u32(code) else {
            continue;
        };
        if ch < '\u{20}' || ch == '"' || ch == '\\' {
            continue;
        }
        let literal = format!("\"{ch}\"");
        assert_eq!(json::from_str::<String>(&literal).unwrap(), ch.to_string(), "U+{code:04X}");
        assert_eq!(
            json::from_slice::<String>(literal.as_bytes()).unwrap(),
            ch.to_string(),
            "U+{code:04X}"
        );
        // The same character next to an escape and another multibyte char.
        let neighbour = format!("\"{ch}\\n{ch}\"");
        assert_eq!(
            json::from_str::<String>(&neighbour).unwrap(),
            format!("{ch}\n{ch}"),
            "U+{code:04X}"
        );
        let value: Value = json::from_slice(literal.as_bytes()).unwrap();
        assert_eq!(value.as_str(), Some(ch.to_string().as_str()));
        checked += 1;
    }
    assert!(checked > 1_000_000, "checked {checked} scalars");
}

#[test]
fn all_unicode_scalars_encode_round_trip() {
    // `escape_into` also slices at `find_string_special` results; every scalar
    // (including quotes, backslashes and controls) must round-trip.
    for code in 0..=0x10FFFFu32 {
        let Some(ch) = char::from_u32(code) else {
            continue;
        };
        let original = ch.to_string();
        let encoded = json::to_string(&original);
        assert!(json::validate(encoded.as_bytes()).is_ok(), "U+{code:04X}: {encoded:?}");
        assert_eq!(
            json::from_str::<String>(&encoded).unwrap(),
            original,
            "U+{code:04X}: {encoded:?}"
        );
    }
    // Long bodies with specials at every 8/16/32/64 boundary round-trip.
    for len in [0usize, 1, 7, 15, 16, 31, 32, 33, 63, 64, 65, 127, 128, 129, 300] {
        for &special in &['"', '\\', '\n', '\u{0}', 'é', '😀', 'a'] {
            for at in [0usize, len / 2, len.saturating_sub(1)] {
                if at > len {
                    continue;
                }
                let mut original = "a".repeat(at);
                original.push(special);
                original.push_str(&"a".repeat(len - at));
                let encoded = json::to_string(&original);
                assert_eq!(json::from_str::<String>(&encoded).unwrap(), original, "len={len}");
            }
        }
    }
}

#[test]
fn document_duplicate_policy_is_structural_only() {
    // `Document::parse_with_options` validates structure; the duplicate-key
    // policy is applied only by typed/Vec/Map decoders, not by `validate`.
    let text = br#"{"a":1,"a":2}"#;
    let document = Document::parse_with_options(
        text,
        Options {
            duplicate_keys: DuplicateKeys::Reject,
            ..Options::default()
        },
    )
    .expect("Reject is not enforced by structural validation");
    assert_eq!(document.get("/a").unwrap().unwrap().as_i64().unwrap(), Some(2));

    let document = Document::parse_with_options(
        text,
        Options {
            duplicate_keys: DuplicateKeys::FirstWins,
            ..Options::default()
        },
    )
    .unwrap();
    // `Element::get` always takes the last occurrence, ignoring FirstWins.
    assert_eq!(document.get("/a").unwrap().unwrap().as_i64().unwrap(), Some(2));

    // Depth and string limits are enforced by the structural pass.
    assert!(
        Document::parse_with_options(
            br#""abcd""#,
            Options {
                max_string_len: Some(1),
                ..Options::default()
            }
        )
        .is_err()
    );
    assert!(
        Document::parse_with_options(
            b"[[1]]",
            Options {
                max_depth: 1,
                ..Options::default()
            }
        )
        .is_err()
    );
}

// ---------------------------------------------------------------------------
// `[T; N]` decoding: initialisation and drop accounting
// ---------------------------------------------------------------------------

use std::cell::Cell;

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

fn drops() -> usize {
    DROPS.with(Cell::get)
}

#[test]
fn fixed_size_array_drop_accounting() {
    // Success: exactly N drops when the value goes out of scope.
    reset_drops();
    {
        // Zero-length arrays have no drop glue, so let it fall out of scope.
        let _value: [Tracked; 0] = json::from_str("[]").unwrap();
    }
    assert_eq!(drops(), 0, "success drops for []");
    reset_drops();
    {
        let value: [Tracked; 1] = json::from_str("[1]").unwrap();
        assert_eq!(value[0].0, 1);
    }
    assert_eq!(drops(), 1, "success drops for [1]");
    reset_drops();
    {
        let value: [Tracked; 2] = json::from_str("[1,2]").unwrap();
        assert_eq!(value[0].0, 1);
        assert_eq!(value[1].0, 2);
    }
    assert_eq!(drops(), 2, "success drops for [1,2]");

    // Error after k successful elements: exactly k drops, no double drop.
    for (input, k) in [("[]", 0usize), ("[1]", 1), ("[1,\"x\"]", 1), ("[1,2,\"x\"]", 2)] {
        reset_drops();
        let result = json::from_str::<[Tracked; 3]>(input);
        assert!(result.is_err(), "{input}");
        assert_eq!(drops(), k, "error drops for {input}");
    }

    // Too many elements: all decoded elements are dropped.
    reset_drops();
    assert!(json::from_str::<[Tracked; 2]>("[1,2,3]").is_err());
    assert_eq!(drops(), 2);

    // Nested arrays: inner arrays that fail drop their initialised prefix and
    // the outer prefix is dropped as well.
    reset_drops();
    assert!(json::from_str::<[[Tracked; 2]; 2]>("[[1,2],[3,\"x\"]]").is_err());
    assert_eq!(drops(), 3);

    // Empty array type accepts only `[]`.
    reset_drops();
    let _empty: [Tracked; 0] = json::from_str("[]").unwrap();
    assert!(json::from_str::<[Tracked; 0]>("[1]").is_err());
    assert_eq!(drops(), 0);
}

#[test]
fn fixed_size_array_zst_and_nested() {
    // ZST element type.
    #[derive(Debug, PartialEq)]
    struct Zst;
    impl<'a> json::Decode<'a> for Zst {
        fn decode(decoder: &mut json::Decoder<'a>) -> json::Result<Self> {
            decoder.decode_null()?;
            Ok(Zst)
        }
    }
    let value: [Zst; 4] = json::from_str("[null,null,null,null]").unwrap();
    assert_eq!(value, [Zst, Zst, Zst, Zst]);
    assert!(json::from_str::<[Zst; 4]>("[null,null,null]").is_err());
    assert!(json::from_str::<[Zst; 4]>("[null,null,null,null,null]").is_err());
    let nested: [[[u8; 2]; 2]; 2] = json::from_str("[[[1,2],[3,4]],[[5,6],[7,8]]]").unwrap();
    assert_eq!(nested, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]);
    // Larger N.
    let mut input = String::from("[");
    for i in 0..64 {
        if i != 0 {
            input.push(',');
        }
        input.push_str(&i.to_string());
    }
    input.push(']');
    let large: [u8; 64] = json::from_str(&input).unwrap();
    assert_eq!(large[63], 63);
}

#[test]
fn fixed_size_array_panic_during_decode_drops_initialised_elements() {
    struct Panic(String);
    impl Drop for Panic {
        fn drop(&mut self) {
            bump_drops();
            // Keep an allocation so a leak is observable under Miri too.
            assert!(!self.0.is_empty());
        }
    }
    impl<'a> json::Decode<'a> for Panic {
        fn decode(decoder: &mut json::Decoder<'a>) -> json::Result<Self> {
            let value: i64 = decoder.decode_i64()?;
            if value == 99 {
                panic!("boom at 99");
            }
            Ok(Panic("x".repeat(value as usize)))
        }
    }

    reset_drops();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: [Panic; 4] = json::from_str("[1,2,99,4]").unwrap();
    }));
    assert!(result.is_err());
    // The two elements decoded before the panic must be dropped while the
    // stack unwinds; the RAII guard in `[T; N]::decode` drops the initialised
    // prefix (and Miri's leak checker confirms no allocation leaks).
    assert_eq!(drops(), 2, "panicking decode leaked the already decoded elements");
}

// ---------------------------------------------------------------------------
// Document / RawValue interactions
// ---------------------------------------------------------------------------

#[test]
fn document_and_raw_value_agree_with_slice_decoding() {
    let documents: &[&[u8]] = &[
        b"null",
        b"true",
        b"-0",
        b"123456789012345678901234567890",
        b"\"hello\\u00e9\\nworld\"",
        b"[\"\\u0000\",\"\\uD83D\\uDE00\"]",
        b"{\"a\":[1,2,{\"b\":null}],\"a\":2}",
        b"  [ \t\r\n 1 ] ",
        b"\xEF\xBB\xBF{\"bom\":true}",
    ];
    for &bytes in documents {
        assert!(json::validate(bytes).is_ok(), "{bytes:?}");
        let raw = RawValue::new(bytes).unwrap();
        assert!(json::validate(raw.get()).is_ok(), "raw slice must validate");
        let document = Document::parse(bytes).unwrap();
        let value: Value = json::from_slice(bytes).unwrap();
        // Round-trip through the encoder and decode again.
        let text = json::to_string(&value);
        let back: Value = json::from_str(&text).unwrap();
        assert_eq!(value, back);
        // Element decoding agrees with the owned value for strings and numbers.
        if let Some(expected) = value.as_str() {
            let mut buffer = vec![0u8; expected.len()];
            assert_eq!(document.root().string_into(&mut buffer).unwrap(), expected);
        }
        if let Some(expected) = value.as_i64() {
            assert_eq!(document.root().as_i64().unwrap(), Some(expected));
        }
        if !value.is_number() {
            assert!(document.root().as_i64().unwrap().is_none());
        }
    }
    // Number outside f64 range is accepted lexically by RawValue/validate but
    // rejected by the numeric decoders.
    assert!(json::from_slice::<f64>(b"1e309").is_err());
    assert!(json::from_slice::<Number>(b"1e309").is_err());
    assert!(json::from_slice::<Value>(b"1e309").is_err());
    assert!(RawValue::new(b"1e309").is_ok());
    assert!(json::validate(b"1e309").is_ok());
}

#[test]
fn decoder_public_api_consistency() {
    // Driving the Decoder manually must agree with the convenience functions.
    let text = "  [1, \"two\", {\"three\": 3.5}, null]";
    let mut decoder = json::Decoder::from_str_input(text);
    decoder.begin_array().unwrap();
    assert!(decoder.array_next(true).unwrap());
    assert_eq!(decoder.decode_inline::<i64>().unwrap(), 1);
    assert!(decoder.array_next(false).unwrap());
    assert_eq!(decoder.decode_inline::<String>().unwrap(), "two");
    assert!(decoder.array_next(false).unwrap());
    decoder.begin_object().unwrap();
    assert!(decoder.object_next(true).unwrap());
    assert_eq!(decoder.key().unwrap().as_str(), "three");
    assert_eq!(decoder.decode_inline::<f64>().unwrap(), 3.5);
    assert!(!decoder.object_next(false).unwrap());
    assert!(decoder.array_next(false).unwrap());
    assert!(decoder.decode_inline::<()>().is_ok());
    assert!(!decoder.array_next(false).unwrap());
    decoder.finish().unwrap();

    // Arbitrary positions inside multibyte input must not create a `&str`
    // from the middle of a character: the string decode requires a quote.
    let mut decoder = json::Decoder::from_str_input("\"é\"");
    decoder.set_position(1); // middle of the two-byte character
    assert!(decoder.decode_str().is_err());
    let mut decoder = json::Decoder::from_str_input("\"é\"");
    decoder.set_position(0);
    assert_eq!(decoder.decode_str().unwrap(), "é");
}
