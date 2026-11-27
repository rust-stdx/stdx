//! Adversarial tests for the safe `yaml` wrapper around the libyaml parser.
//!
//! These tests only use the public, safe API. They cover truncations, invalid
//! encodings, buffer-refill boundaries, deep nesting, huge anchors/tags, alias
//! expansion and deterministic random inputs. A panic or abort is a failure.

use std::panic::{AssertUnwindSafe, catch_unwind};

use yaml::{Options, Value, from_slice, from_slice_multi, from_slice_multi_with_options, from_slice_with_options};

fn parse(input: &[u8]) -> Result<Value, yaml::Error> {
    from_slice::<Value>(input)
}

fn no_panic(input: &[u8]) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = parse(input);
        let _ = from_slice_multi::<Value>(input);
    }));
    assert!(result.is_ok(), "panic for input {:?}", String::from_utf8_lossy(input));
}

#[test]
fn every_truncation_of_representative_document() {
    let doc = b"%YAML 1.2\n%TAG !e! tag:example.com,2000:\n---\nbase: &a {b: [1, 2, {c: \"d\\u00e9\"}], e: |\n  literal\n  text\n}\ncopy: *a\n---\n- !!str 123\n- [!e!foo bar]\n...\n";
    for len in 0..=doc.len() {
        no_panic(&doc[..len]);
    }
}

#[test]
fn invalid_utf8_and_control_bytes() {
    let cases: &[&[u8]] = &[
        b"\x00",
        b"a\x00b",
        b"\x01",
        b"a: \x01\n",
        b"a: \x7f\n",
        b"\xef\xbb\xbfkey: value\n",
        b"\xff\xfek\x00e\x00y\x00",
        b"\xfe\xff\x00k\x00e\x00y\x00",
        b"\xef\xbb",
        b"\xc0\x80",
        b"\xed\xa0\x80",
        b"\xf4\x90\x80\x80",
        b"\xf5\x80\x80\x80",
        b"\x80\x80\x80",
        b"a\r\nb\r\n",
        b"a\rb\r",
        b"a\tb\n",
        b"a: b\xc2\x85c\n",
        b"\"\\\r\n\"",
    ];
    for case in cases {
        no_panic(case);
    }
    // Every prefix of a 4-byte sequence inside a double-quoted scalar.
    for seq in ["\u{00e9}", "\u{0800}", "\u{fffd}", "\u{1f600}"] {
        let mut doc = Vec::new();
        doc.extend_from_slice(b"a: \"");
        doc.extend_from_slice(seq.as_bytes());
        doc.extend_from_slice(b"\"\n");
        for len in 0..=doc.len() {
            no_panic(&doc[..len]);
        }
    }
}

#[test]
fn scalars_across_buffer_refill_boundaries() {
    // The scanner/reader buffers are 16384 and 49152 bytes. Exercise scalars
    // that end just before/at/after each boundary, in every scalar style.
    let sizes = [
        0usize, 1, 2, 3, 4, 5, 16382, 16383, 16384, 16385, 16386, 49150, 49151, 49152, 49153,
    ];
    for size in sizes {
        let text = "x".repeat(size);

        let empty_expectation = if size == 0 {
            Value::Null
        } else {
            Value::from(text.as_str())
        };
        let plain = format!("k: {text}\n");
        let parsed = from_slice::<Value>(plain.as_bytes()).unwrap();
        assert_eq!(parsed["k"], empty_expectation);

        let quoted = format!("k: \"{text}\"\n");
        let parsed = from_slice::<Value>(quoted.as_bytes()).unwrap();
        assert_eq!(parsed["k"], Value::from(text.as_str()));

        let single = format!("k: '{text}'\n");
        let parsed = from_slice::<Value>(single.as_bytes()).unwrap();
        assert_eq!(parsed["k"], Value::from(text.as_str()));

        let literal = format!("k: |\n  {text}\n");
        let parsed = from_slice::<Value>(literal.as_bytes()).unwrap();
        // Literal blocks use clip chomping, so a non-empty last line keeps one
        // trailing newline.
        assert_eq!(parsed["k"].as_str().unwrap().trim_end_matches('\n'), text);
    }

    // A quoted scalar containing four-byte characters ending at many offsets.
    for pad in 0..8 {
        let mut doc = Vec::new();
        doc.extend_from_slice(b"k: \"");
        doc.extend(
            std::iter::repeat_n("\u{1f600}".as_bytes(), 4096 + pad)
                .flatten()
                .copied(),
        );
        doc.extend_from_slice(b"\"\n");
        let parsed = from_slice::<Value>(&doc).unwrap();
        assert_eq!(parsed["k"].as_str().unwrap().chars().count(), 4096 + pad);
    }
}

#[test]
fn huge_anchors_aliases_tags_and_directives() {
    for size in [0usize, 1, 2, 1023, 1024, 1025, 16383, 16384, 16385] {
        let anchor = "a".repeat(size);
        let doc = if size == 0 {
            "k: & x\n".to_string()
        } else {
            format!("k: &{anchor} {anchor}\n")
        };
        no_panic(doc.as_bytes());

        let tag = "t".repeat(size);
        let doc = format!("k: !{tag} v\n");
        no_panic(doc.as_bytes());

        let handle = "h".repeat(size);
        let doc = format!("%TAG !{handle}! tag:example.com,2000:\n---\nk: !{handle}!s v\n");
        no_panic(doc.as_bytes());

        let prefix = "p".repeat(size);
        let doc = format!("%TAG !e! {prefix}\n---\nk: !e!s v\n");
        no_panic(doc.as_bytes());

        let verbatim = "tag:example.com,2000:".to_string() + &"v".repeat(size);
        let doc = format!("k: !<{verbatim}> v\n");
        no_panic(doc.as_bytes());

        if size != 0 {
            let doc = format!("k: &{anchor} 1\nj: *{anchor}\n");
            let parsed = from_slice::<Value>(doc.as_bytes()).unwrap();
            assert_eq!(parsed["j"], Value::from(1));

            // Simple keys are limited to 1024 characters by the YAML spec, so
            // use the explicit key syntax for the long-key case.
            let doc = format!("? \"{anchor}\"\n: v\n");
            let parsed = from_slice::<Value>(doc.as_bytes()).unwrap();
            assert_eq!(parsed[anchor.as_str()], Value::from("v"));
        }
    }
}

#[test]
fn deeply_nested_flow_and_block() {
    for depth in [1usize, 2, 64, 127, 128, 129, 200, 1000] {
        let mut flow = "[".repeat(depth);
        flow.push_str(&"]".repeat(depth));
        no_panic(flow.as_bytes());

        let mut block = String::new();
        for level in 0..depth {
            block.push_str(&" ".repeat(level));
            block.push_str("- \n");
        }
        no_panic(block.as_bytes());
    }

    // Flow nesting through the default limit must be a clean error, not a hang.
    let deep = "[".repeat(130);
    let error = from_slice::<Value>(deep.as_bytes()).unwrap_err().to_string();
    assert!(error.contains("depth"), "{error}");

    // And unlimited depth must still stop at the input-size limit.
    let options = Options {
        max_depth: usize::MAX,
        max_input_bytes: Some(2048),
        ..Options::default()
    };
    let error = from_slice_with_options::<Value>("[".repeat(4096).as_bytes(), options)
        .unwrap_err()
        .to_string();
    assert!(error.contains("byte limit"), "{error}");
}

#[test]
fn alias_bombs_and_duplicate_anchors() {
    let bomb = "a: &a [x,x,x,x,x,x,x,x,x,x]\n\
                b: &b [*a,*a,*a,*a,*a,*a,*a,*a,*a,*a]\n\
                c: &c [*b,*b,*b,*b,*b,*b,*b,*b,*b,*b]\n\
                d: &d [*c,*c,*c,*c,*c,*c,*c,*c,*c,*c]\n\
                e: &e [*d,*d,*d,*d,*d,*d,*d,*d,*d,*d]\n\
                f: &f [*e,*e,*e,*e,*e,*e,*e,*e,*e,*e]\n\
                g: &g [*f,*f,*f,*f,*f,*f,*f,*f,*f,*f]\n";
    let error = from_slice::<Value>(bomb.as_bytes()).unwrap_err().to_string();
    assert!(error.contains("value limit"), "{error}");

    let duplicates = "a: &x 1\nb: &x 2\nc: *x\n";
    let parsed = from_slice::<Value>(duplicates.as_bytes()).unwrap();
    assert_eq!(parsed["c"], Value::from(2));

    // Unknown aliases and forward aliases are errors.
    assert!(from_slice::<Value>(b"*x").is_err());
    assert!(from_slice::<Value>(b"a: *b\nb: &b 1\n").is_err());
}

#[test]
fn multi_document_limits() {
    let many = "---\n1\n".repeat(2000);
    let options = Options {
        max_documents: Some(3),
        ..Options::default()
    };
    let error = from_slice_multi_with_options::<Value>(many.as_bytes(), options)
        .unwrap_err()
        .to_string();
    assert!(error.contains("document limit"), "{error}");
}

#[test]
fn deterministic_random_inputs_do_not_panic() {
    let mut state = 0x243f_6a88_85a3_08d3u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let alphabet: &[u8] = b"abc012 :,-[]{}#&*!|>'\"%@`.?\\\n\r\t\x00\x7f\x80\xc2\xe2\xf0";
    for _ in 0..20_000 {
        let len = (next() % 64) as usize;
        let mut input = Vec::with_capacity(len);
        for _ in 0..len {
            input.push(alphabet[(next() as usize) % alphabet.len()]);
        }
        no_panic(&input);
    }
}

#[test]
fn length_limits_around_zero_and_one() {
    for input in [
        &b""[..],
        b"\n",
        b"\r\n",
        b"\0",
        b" ",
        b"-",
        b"[",
        b"]",
        b"{",
        b"}",
        b"a:",
        b"a:\n",
        b"---",
        b"...",
        b"%",
        b"%YAML",
        b"%TAG",
        b"&",
        b"*",
        b"!",
        b"!<",
        b"!<>",
        b"|",
        b">",
        b"'",
        b"\"",
        b"\xef\xbb\xbf",
        b"---\n...\n",
        b"...\n---\n",
        b"---\n---\n",
        b"---\n...\n---\na\n...\n",
        b"# only a comment",
        b"# comment\n",
        b"a\r\n...\r\n",
    ] {
        no_panic(input);
    }
}
