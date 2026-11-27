//! Adversarial tests for the raw (unsafe) libyaml translation used by `yaml`.
//!
//! Every input is driven through both the token scanner and the event parser,
//! with a custom read handler that can return short reads.  The goal is to
//! exercise the buffer/stack/queue code with untrusted lengths, truncations and
//! malformed encodings.

#![allow(unsafe_op_in_unsafe_fn)]

use std::{cmp, ffi::c_void, mem::MaybeUninit, ptr, ptr::addr_of_mut};

use libyaml_unsafe::{
    YAML_NO_ERROR, YAML_STREAM_END_EVENT, YAML_STREAM_END_TOKEN, yaml_event_delete, yaml_event_t, yaml_parser_delete,
    yaml_parser_initialize, yaml_parser_parse, yaml_parser_scan, yaml_parser_set_input, yaml_parser_t,
    yaml_token_delete, yaml_token_t,
};

const ALL_CHUNKS: &[usize] = &[1, 2, 3, 5, 7, 16, 4096, 16_384, usize::MAX];

struct Reader {
    data: Vec<u8>,
    pos: usize,
    chunk: usize,
    // When true, return one zero-length read before signalling EOF.
    stall: bool,
    stalled: bool,
}

unsafe fn read_handler(data: *mut c_void, buffer: *mut u8, size: u64, size_read: *mut u64) -> i32 {
    let reader = &mut *(data as *mut Reader);
    let remaining = reader.data.len() - reader.pos;
    if remaining == 0 {
        if reader.stall && !reader.stalled {
            reader.stalled = true;
            *size_read = 0;
            return 1;
        }
        *size_read = 0;
        return 1;
    }
    let n = cmp::min(remaining, cmp::min(size as usize, reader.chunk));
    ptr::copy_nonoverlapping(reader.data.as_ptr().add(reader.pos), buffer, n);
    reader.pos += n;
    *size_read = n as u64;
    1
}

struct Outcome {
    ok: bool,
    error: u32,
    events: usize,
    #[allow(dead_code)]
    tokens: usize,
}

unsafe fn drive_events(parser: *mut yaml_parser_t) -> (bool, u32, usize) {
    let mut count = 0usize;
    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    loop {
        let success = yaml_parser_parse(parser, event);
        if !success.ok {
            // libyaml zeroes the event before parsing. Free anything a partial
            // event may own; this is what a careful C caller has to do too.
            yaml_event_delete(event);
            break;
        }
        let type_ = (*event).type_;
        yaml_event_delete(event);
        count += 1;
        if type_ == YAML_STREAM_END_EVENT {
            break;
        }
    }
    let error = (&*parser).error as u32;
    (error == YAML_NO_ERROR as u32, error, count)
}

unsafe fn drive_tokens(parser: *mut yaml_parser_t) -> (bool, u32, usize) {
    let mut count = 0usize;
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    loop {
        let success = yaml_parser_scan(parser, token);
        if !success.ok {
            yaml_token_delete(token);
            break;
        }
        let type_ = (*token).type_;
        yaml_token_delete(token);
        count += 1;
        if type_ == YAML_STREAM_END_TOKEN {
            break;
        }
    }
    let error = (&*parser).error as u32;
    (error == YAML_NO_ERROR as u32, error, count)
}

fn run(input: &[u8], chunk: usize, stall: bool, tokens: bool) -> Outcome {
    unsafe {
        let mut reader = Reader {
            data: input.to_vec(),
            pos: 0,
            chunk,
            stall,
            stalled: false,
        };
        let mut parser = MaybeUninit::<yaml_parser_t>::uninit();
        let parser = parser.as_mut_ptr();
        assert!(yaml_parser_initialize(parser).ok);
        yaml_parser_set_input(parser, read_handler, addr_of_mut!(reader).cast());
        let (ok, error, count) = if tokens {
            drive_tokens(parser)
        } else {
            drive_events(parser)
        };
        yaml_parser_delete(parser);
        Outcome {
            ok,
            error,
            events: if tokens { 0 } else { count },
            tokens: if tokens { count } else { 0 },
        }
    }
}

fn parse_all(input: &[u8]) -> Outcome {
    run(input, usize::MAX, false, false)
}

fn exhaustive(input: &[u8]) {
    for &chunk in ALL_CHUNKS {
        for &stall in &[false, true] {
            let _ = run(input, chunk, stall, false);
            let _ = run(input, chunk, stall, true);
        }
    }
}

#[test]
fn empty_and_tiny_inputs() {
    for input in [
        &b""[..],
        b"\0",
        b" ",
        b"\n",
        b"\r",
        b"\r\n",
        b"-",
        b"--",
        b"---",
        b"...",
        b"a",
        b"#",
    ] {
        exhaustive(input);
    }
}

#[test]
fn truncations_of_representative_document() {
    let doc = b"%YAML 1.2\n%TAG !e! tag:example.com,2000:\n---\nbase: &a {b: [1, 2, {c: \"d\\u00e9\"}], e: |\n  literal\n  text\n}\ncopy: *a\n---\n- !!str 123\n- [!e!foo bar]\n...\n";
    for len in 0..=doc.len() {
        let input = &doc[..len];
        exhaustive(input);
        // Every prefix must parse or produce an error, never hang or crash.
        let out = parse_all(input);
        assert!(out.ok || out.error != 0);
    }
}

#[test]
fn every_prefix_of_multibyte_utf8() {
    // Valid 1..4 byte characters, plus overlong/surrogate/out-of-range forms.
    let sequences: &[&[u8]] = &[
        b"a",
        "\u{00e9}".as_bytes(),  // C3 A9
        "\u{0800}".as_bytes(),  // E0 A0 80
        "\u{fffd}".as_bytes(),  // EF BF BD
        "\u{1f600}".as_bytes(), // F0 9F 98 80
        b"\xc0\x80",            // overlong NUL
        b"\xe0\x80\xaf",        // overlong
        b"\xed\xa0\x80",        // surrogate
        b"\xf4\x90\x80\x80",    // > U+10FFFF
        b"\xf5\x80\x80\x80",    // invalid lead
        b"\x80",                // stray continuation
        b"\xfe",
        b"\xff",
    ];
    for seq in sequences {
        let mut doc = Vec::new();
        doc.extend_from_slice(b"a: \"");
        doc.extend_from_slice(seq);
        doc.extend_from_slice(b"\"\n");
        for len in 0..=doc.len() {
            exhaustive(&doc[..len]);
        }
        for prefix_len in 1..seq.len() {
            let mut truncated = Vec::new();
            truncated.extend_from_slice(b"a: ");
            truncated.extend_from_slice(&seq[..prefix_len]);
            exhaustive(&truncated);
        }
    }
}

#[test]
fn invalid_bytes_and_control_characters() {
    let cases: &[&[u8]] = &[
        b"\x00",
        b"a\x00b",
        b"\x01",
        b"a: \x01\n",
        b"a: \x1f\n",
        b"a: \x7f\n",
        b"\xef\xbb\xbfkey: value\n",
        b"\xff\xfek\x00e\x00y\x00",
        b"\xfe\xff\x00k\x00e\x00y\x00",
        b"\xef\xbb",
        b"\xef\xbb\xbf\xef\xbb",
        b"a\r\nb\r\n",
        b"a\rb\r",
        b"a\tb\n",
        b"a: b\xc2\x85c\n",
    ];
    for case in cases {
        exhaustive(case);
    }
}

#[test]
fn exhaustive_short_indicator_inputs() {
    // Every string up to length 3 over YAML indicators, whitespace, breaks,
    // NUL and UTF-8 lead bytes. This is where one-byte lookahead at EOF and
    // invalid/truncated multibyte handling live.
    let alphabet: &[u8] = b" \n\r\t-?:,[]{}#&*!|>'\"%@`.\\\x00\xc2\xe2\xef\xf0a0";
    for len in 0..=3 {
        let mut indices = vec![0usize; len];
        loop {
            let input: Vec<u8> = indices.iter().map(|&i| alphabet[i]).collect();
            let out = parse_all(&input);
            assert!(out.ok || out.error != 0, "{input:?}");
            let out = run(&input, 1, true, true);
            assert!(out.ok || out.error != 0, "{input:?}");

            let mut pos = 0;
            loop {
                if pos == len {
                    break;
                }
                indices[pos] += 1;
                if indices[pos] < alphabet.len() {
                    break;
                }
                indices[pos] = 0;
                pos += 1;
            }
            if pos == len {
                break;
            }
        }
    }
}

#[test]
fn small_deterministic_random_inputs() {
    // Kept small so this test is fast enough to run under Miri.
    let mut state = 0x243f_6a88_85a3_08d3u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let alphabet: &[u8] = b"abc012 :,-[]{}#&*!|>'\"%@`.?\\\n\r\t\x00\x7f\x80\xc2\xe2\xf0";
    for _ in 0..300 {
        let len = (next() % 32) as usize;
        let mut input = Vec::with_capacity(len);
        for _ in 0..len {
            input.push(alphabet[(next() as usize) % alphabet.len()]);
        }
        let out = parse_all(&input);
        assert!(out.ok || out.error != 0, "{input:?}");
        let out = run(&input, 3, true, true);
        assert!(out.ok || out.error != 0, "{input:?}");
    }
}

#[test]
fn scanner_error_after_container_start() {
    // The parser calls PEEK_TOKEN at the entry of a container state and
    // dereferences the result without a null check, exactly like the C
    // original. If a scan error can be reported after the container start
    // token was produced, that is a null dereference.
    let cases: &[&[u8]] = &[
        b"- @",
        b"- @\n",
        b"-\n @",
        b"---\n- @",
        b"key: @",
        b"key: @\n",
        b"a:\n @",
        b"[@",
        b"[ @",
        b"{@",
        b"{a: @",
        b"- !<x",
        b"- *",
        b"- &",
        b"- |0",
    ];
    for case in cases {
        let out = parse_all(case);
        assert!(out.ok || out.error != 0, "{case:?}");
    }
}

#[test]
fn stack_growth_at_moderate_depth() {
    // Force several stack/queue reallocations (>16, >32, ... >256 entries)
    // without going anywhere near the memory exhaustion reported separately.
    for depth in [17usize, 33, 65, 129, 257] {
        let mut flow = "[".repeat(depth);
        flow.push_str(&"]".repeat(depth));
        let out = parse_all(flow.as_bytes());
        assert!(!out.ok || out.events > depth);

        let mut block = String::new();
        for level in 0..depth {
            block.push_str(&" ".repeat(level));
            block.push_str("a:\n");
        }
        let out = parse_all(block.as_bytes());
        assert!(out.ok, "block depth {depth}: error {}", out.error);
    }
}

#[test]
fn utf16_bom_streams() {
    // Raw API only: the safe wrapper forces UTF-8.
    let mut le = vec![0xff, 0xfe];
    for unit in "key: value\n".encode_utf16() {
        le.extend_from_slice(&unit.to_le_bytes());
    }
    let mut be = vec![0xfe, 0xff];
    for unit in "key: value\n".encode_utf16() {
        be.extend_from_slice(&unit.to_be_bytes());
    }
    for input in [&le, &be] {
        for len in 0..=input.len() {
            exhaustive(&input[..len]);
        }
    }
}
