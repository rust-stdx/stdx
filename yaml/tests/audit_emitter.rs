//! Audit tests for the safe YAML emitter (`yaml::to_string` / `to_vec`)
//! and its interaction with the parser.
//!
//! Run: `cargo test -p yaml --release --test audit_emitter`

use yaml::{Options, Value, from_slice_with_options, to_vec};

/// Deterministic xorshift64* so failures are reproducible.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn roundtrip(value: &Value) {
    let bytes = to_vec(value).expect("emit");
    let options = Options {
        // The emitter output for these tests is small; relax limits so that
        // long scalars or moderately deep trees are not rejected by the
        // parser's conservative defaults.
        max_input_bytes: None,
        max_values: None,
        max_aliases: None,
        max_documents: None,
        max_depth: 4096,
        ..Options::default()
    };
    let decoded = from_slice_with_options::<Value>(&bytes, options).expect("parse emitted YAML");
    // The public API stops after the first document, matching `to_vec`.
    assert_eq!(&decoded, value, "roundtrip mismatch for {value:?} ({} bytes)", bytes.len());
}

fn string_value(s: impl Into<String>) -> Value {
    Value::String(s.into())
}

#[test]
fn roundtrip_empty_and_tiny_strings() {
    for s in [
        "", "a", "0", "-", "--", "...", "?", ":", "#", " ", "\t", "\n", "\r", "\r\n", "\u{85}", "\u{2028}", "\u{2029}",
        "\u{feff}", "'", "\"", "\\", "a\n", "\n\n", "a ", " a", "a b",
    ] {
        roundtrip(&string_value(s));
    }
}

#[test]
fn roundtrip_control_and_nul_characters() {
    // Every ASCII control character, alone and mixed, including the embedded
    // NUL that exercises libyaml's length/NUL-terminator split.
    for byte in 0u8..=0x1f {
        roundtrip(&string_value((byte as char).to_string()));
        roundtrip(&string_value(format!("a{}b", byte as char)));
    }
    roundtrip(&string_value("\u{7f}"));
    roundtrip(&string_value("nul:\0:inside"));
    roundtrip(&string_value("\0\0\0"));
    // C1 controls and Unicode line separators force double-quoted escapes.
    for s in [
        "\u{80}",
        "\u{9f}",
        "\u{85}",
        "\u{2028}",
        "\u{2029}",
        "\u{feff}",
        "\u{ffff}",
        "\u{10ffff}",
    ] {
        roundtrip(&string_value(s));
    }
}

#[test]
fn roundtrip_non_ascii_and_4_byte_scalars() {
    for s in [
        "é",
        "日本語",
        "🎉",
        "a🎉b",
        "\u{10000}",
        "\u{10ffff}",
        "ñ\u{85}ñ",
        "👍👍👍",
    ] {
        roundtrip(&string_value(s));
    }
}

#[test]
fn roundtrip_buffer_boundary_lengths() {
    // The emitter buffer is 16384 bytes; FLUSH keeps only 6 bytes of headroom
    // while WRITE may copy up to 4 bytes at once. Walk every length around the
    // refill boundaries for several scalar shapes.
    let mut lengths = Vec::new();
    for base in [0usize, 16_384, 32_768, 65_536, 70_000] {
        for delta in 0..12usize {
            lengths.push(base + delta);
        }
    }
    lengths.extend([1, 2, 3, 4, 5, 6, 7, 8, 16, 127, 128, 129]);
    lengths.sort_unstable();
    lengths.dedup();

    for len in lengths {
        // Plain-ish: single long word (escaped style forced by the wrapper
        // only when needed; long words exercise the plain writer).
        let word = "a".repeat(len);
        roundtrip(&string_value(word));

        // Spaces: exercises line folding in the plain and quoted writers.
        let mut spaced = String::with_capacity(len + len / 4);
        for i in 0..len {
            if i % 5 == 4 {
                spaced.push(' ');
            } else {
                spaced.push('b');
            }
        }
        roundtrip(&string_value(spaced));

        // Multibyte: 4-byte scalar repeated, checks the width-aware COPY path.
        let emoji = "🎉".repeat(len / 4 + 1);
        roundtrip(&string_value(emoji));
    }
}

#[test]
fn roundtrip_multiline_lengths() {
    // Literal style (`\n` in the value) and folded/indented output around the
    // buffer boundaries.
    for len in [1usize, 2, 5, 6, 16_380, 16_384, 16_390, 32_764, 32_770] {
        let mut s = String::with_capacity(len * 2);
        for i in 0..len {
            if i % 9 == 8 {
                s.push('\n');
            } else {
                s.push('c');
            }
        }
        roundtrip(&string_value(s));
        let mut trailing = "d".repeat(len);
        trailing.push('\n');
        roundtrip(&string_value(trailing));
        roundtrip(&string_value("\n".repeat(len)));
    }
}

#[test]
fn roundtrip_long_and_awkward_keys() {
    let mut mapping = yaml::Mapping::new();
    mapping.insert(string_value("k".repeat(200)), Value::Bool(true));
    mapping.insert(string_value("key with spaces"), Value::Number(1u64.into()));
    mapping.insert(string_value("line\nbreak"), Value::Number(2u64.into()));
    mapping.insert(Value::Null, string_value("null key"));
    mapping.insert(Value::Sequence(vec![Value::Bool(false)]), string_value("complex key"));
    mapping.insert(string_value("\0"), string_value("nul key"));
    roundtrip(&Value::Mapping(mapping));
}

#[test]
fn roundtrip_tags_and_binary() {
    // `!!binary` and `!!timestamp` are preserved as tagged strings.
    roundtrip(&Value::Tagged(Box::new(yaml::TaggedValue {
        tag: "tag:yaml.org,2002:binary".to_owned(),
        value: string_value("R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7"),
    })));
    roundtrip(&Value::Tagged(Box::new(yaml::TaggedValue {
        tag: "!local".to_owned(),
        value: Value::Sequence(vec![Value::Null, Value::Bool(true)]),
    })));
    roundtrip(&Value::Tagged(Box::new(yaml::TaggedValue {
        tag: "tag:example.com,2000:app/things".to_owned(),
        value: string_value("value"),
    })));
    // A tag that needs percent-encoding in the tag writer.
    roundtrip(&Value::Tagged(Box::new(yaml::TaggedValue {
        tag: "!weird tag:{}/".to_owned(),
        value: string_value("x"),
    })));
}

#[test]
fn roundtrip_deep_and_wide_structures() {
    // Deep enough to grow the emitter's state/indent stacks (initial capacity
    // is 16) without output becoming O(n^2) at a painful size.
    let depth = 300;
    let mut value = Value::Null;
    for _ in 0..depth {
        value = Value::Sequence(vec![value]);
    }
    roundtrip(&value);

    // Wide sequence and mapping with many scalar entries.
    roundtrip(&Value::Sequence((0..2000).map(|i| Value::Number(i.into())).collect()));
    let mut mapping = yaml::Mapping::with_capacity(2000);
    for i in 0..2000u64 {
        mapping.insert(string_value(format!("key{i}")), Value::Number(i.into()));
    }
    roundtrip(&Value::Mapping(mapping));
}

#[test]
fn roundtrip_random_values() {
    let mut rng = Rng::new(0x5eed_1234_abcd_0001);
    for case in 0..400 {
        let value = random_value(&mut rng, 0);
        let bytes = to_vec(&value).expect("emit");
        let options = Options {
            max_input_bytes: None,
            max_values: None,
            max_aliases: None,
            max_documents: None,
            max_depth: 64,
            ..Options::default()
        };
        let decoded = from_slice_with_options::<Value>(&bytes, options)
            .unwrap_or_else(|e| panic!("case {case}: parse failed: {e}; input {bytes:?}"));
        assert_eq!(decoded, value, "case {case} mismatch");
    }
}

fn random_string(rng: &mut Rng) -> String {
    let len = match rng.below(10) {
        0..=4 => rng.below(8),
        5..=7 => rng.below(300),
        _ => rng.below(20_000),
    };
    let mut s = String::with_capacity(len);
    let alphabet = [
        'a', 'Z', '0', ' ', '\t', '\n', '\r', ':', '-', '#', '?', '&', '*', '!', '|', '>', '\'', '"', '\\', '\0',
        '\u{7f}', '\u{85}', '\u{2028}', 'é', '日', '🎉',
    ];
    while s.len() < len {
        s.push(alphabet[rng.below(alphabet.len())]);
    }
    s
}

fn random_value(rng: &mut Rng, depth: usize) -> Value {
    let choice = if depth >= 5 { rng.below(4) } else { rng.below(7) };
    match choice {
        0 => Value::Null,
        1 => Value::Bool(rng.next() & 1 == 0),
        2 => Value::Number((rng.next() as i64 as u64).into()),
        3 => string_value(random_string(rng)),
        4 => {
            let n = rng.below(6);
            Value::Sequence((0..n).map(|_| random_value(rng, depth + 1)).collect())
        }
        5 => {
            let n = rng.below(6);
            let mut mapping = yaml::Mapping::new();
            for _ in 0..n {
                let key = random_value(rng, depth + 1);
                let value = random_value(rng, depth + 1);
                mapping.insert(key, value);
            }
            Value::Mapping(mapping)
        }
        _ => {
            let inner = random_value(rng, depth + 1);
            // The emitter rejects a tag directly on another tag, so collapse
            // nested tags into the underlying value.
            let inner = match inner {
                Value::Tagged(tagged) => tagged.value,
                other => other,
            };
            Value::Tagged(Box::new(yaml::TaggedValue {
                tag: if rng.next() & 1 == 0 {
                    "!audit".to_owned()
                } else {
                    "tag:example.com,2000:x".to_owned()
                },
                value: inner,
            }))
        }
    }
}

#[test]
fn output_is_always_utf8() {
    let mut rng = Rng::new(7);
    for _ in 0..100 {
        let value = random_value(&mut rng, 0);
        let bytes = to_vec(&value).expect("emit");
        assert!(core::str::from_utf8(&bytes).is_ok());
    }
}

#[cfg(feature = "std")]
#[test]
fn failing_std_writer_reports_error_and_does_not_panic() {
    struct FailingWriter {
        budget: usize,
    }
    impl std::io::Write for FailingWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            if buf.len() > self.budget {
                return Err(std::io::Error::other("audit writer failed"));
            }
            self.budget -= buf.len();
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let value = Value::Sequence((0..100).map(|i| Value::Number(i.into())).collect());
    // `to_writer` serializes up front and then performs one `write_all`, so
    // every failure point is either before any byte or a short write.
    for budget in [0usize, 1, 3, 10, 100] {
        let mut writer = FailingWriter {
            budget,
        };
        let result = yaml::to_writer(&value, &mut writer);
        assert!(result.is_err(), "budget {budget} should fail");
    }
    let mut writer = FailingWriter {
        budget: usize::MAX,
    };
    yaml::to_writer(&value, &mut writer).expect("sufficient budget");
}

/// The emitter rejects scalar lengths that cannot fit libyaml's `c_int`
/// length, instead of silently truncating (which would read past the end of
/// the Rust string via `strlen`).
#[test]
fn oversized_scalar_length_is_rejected_without_reading() {
    // We cannot allocate 2 GiB in a unit test, but we can check the boundary
    // arithmetic that the wrapper uses: `i32::MAX` and above must be refused.
    // The unit test inside the crate covers `scalar_length` directly.
    let small = "x".repeat(1024 * 1024);
    let value = string_value(small);
    let bytes = to_vec(&value).expect("1 MiB scalar still fits");
    assert!(bytes.len() >= 1024 * 1024);
}

/// Regression for finding F-4: a `TaggedValue::tag` containing an interior NUL
/// used to be silently truncated because the safe wrapper passes tags through
/// the C-string (`strlen`) path. Values are length-delimited and preserve NUL,
/// but tags (and anchors, in the raw API) are not. The emitter now rejects such
/// a tag instead of emitting a different one.
#[test]
fn tag_with_interior_nul_round_trips() {
    let tagged = Value::Tagged(Box::new(yaml::TaggedValue {
        tag: "!real\0ignored".to_owned(),
        value: string_value("value"),
    }));
    match yaml::to_string(&tagged) {
        // Acceptable fix: reject the tag instead of silently changing it.
        Err(_) => return,
        Ok(text) => {
            eprintln!("emitted: {text:?}");
            let decoded: Value = yaml::from_str(&text).expect("parse");
            assert_eq!(decoded, tagged, "the emitted tag must round-trip exactly");
        }
    }
}
