//! Structural invariants of the generated messages.
//!
//! These tests do not rely on an external parser: they check the properties a
//! valid message must have directly (RFC 5322 line lengths and line endings,
//! RFC 2046 boundaries, RFC 2045 transfer encodings) and decode the bodies with
//! small reference decoders.

use mail_builder::{MessageBuilder, headers::date::Date, mime::MimePart};

fn header_block(eml: &[u8]) -> &[u8] {
    let end = eml
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .expect("message has no header/body separator");
    &eml[..end]
}

fn body_of(eml: &[u8]) -> &[u8] {
    let at = eml
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .expect("message has no header/body separator");
    &eml[at + 4..]
}

fn field<'a>(headers: &'a str, name: &str) -> Option<&'a str> {
    headers.split("\r\n").find_map(|line| {
        line.strip_prefix(name)
            .and_then(|rest| rest.strip_prefix(':'))
            .map(str::trim)
    })
}

/// Asserts RFC 5322 section 2.1.1: no line longer than 998 bytes, and header
/// lines no longer than the 78 byte recommendation.
fn assert_line_lengths(eml: &[u8]) {
    let headers = header_block(eml);
    for line in headers.split(|&b| b == b'\n') {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        assert!(line.len() <= 998, "header line too long: {} bytes", line.len());
        assert!(line.len() <= 78, "header line over 78 bytes: {line:?}");
    }
    for line in eml.split(|&b| b == b'\n') {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        assert!(line.len() <= 998, "line too long: {} bytes", line.len());
    }
}

/// Asserts RFC 5322 section 2.3: CR and LF only occur as CRLF, and the header
/// block contains no bare line breaks.
fn assert_line_endings(eml: &[u8]) {
    let mut at = 0;
    while at < eml.len() {
        match eml[at] {
            b'\r' => {
                assert_eq!(eml.get(at + 1), Some(&b'\n'), "bare CR at {at}");
                at += 2;
            }
            b'\n' => panic!("bare LF at {at}"),
            _ => at += 1,
        }
    }
    let _ = header_block(eml);
}

fn decode_quoted_printable(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < input.len() {
        match input[at] {
            b'=' if input.get(at + 1) == Some(&b'\r') && input.get(at + 2) == Some(&b'\n') => {
                at += 3;
            }
            b'=' if input.get(at + 1) == Some(&b'\n') => {
                at += 2;
            }
            b'=' => {
                let hex = input.get(at + 1..at + 3).expect("truncated escape");
                let byte = u8::from_str_radix(std::str::from_utf8(hex).unwrap(), 16).expect("bad escape");
                out.push(byte);
                at += 3;
            }
            byte => {
                out.push(byte);
                at += 1;
            }
        }
    }
    out
}

fn decode_base64(input: &[u8]) -> Vec<u8> {
    fn value(byte: u8) -> Option<u8> {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let mut out = Vec::new();
    let mut accumulator = 0u32;
    let mut bits = 0u32;
    for &byte in input {
        let Some(symbol) = value(byte) else {
            continue;
        };
        accumulator = (accumulator << 6) | u32::from(symbol);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((accumulator >> bits) as u8);
        }
    }
    out
}

/// Decodes the body according to its Content-Transfer-Encoding.
fn decoded_body(eml: &[u8]) -> Vec<u8> {
    let headers = String::from_utf8_lossy(header_block(eml)).into_owned();
    let body = body_of(eml);
    match field(&headers, "Content-Transfer-Encoding") {
        Some("base64") => decode_base64(body),
        Some("quoted-printable") => decode_quoted_printable(body),
        _ => body.to_vec(),
    }
}

fn crlf_normalized(input: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < input.len() {
        match input.as_bytes()[at] {
            b'\r' => {
                out.extend_from_slice(b"\r\n");
                at += if input.as_bytes().get(at + 1) == Some(&b'\n') {
                    2
                } else {
                    1
                };
            }
            b'\n' => {
                out.extend_from_slice(b"\r\n");
                at += 1;
            }
            byte => {
                out.push(byte);
                at += 1;
            }
        }
    }
    out
}

#[test]
fn simple_text_message_is_well_formed() {
    let eml = MessageBuilder::new()
        .from(("John Doe", "john@doe.com"))
        .to("jane@doe.com")
        .subject("Hello, world!")
        .text_body("Message contents go here.")
        .to_vec()
        .unwrap();
    assert_line_lengths(&eml);
    assert_line_endings(&eml);
    let headers = String::from_utf8_lossy(header_block(&eml)).into_owned();
    assert_eq!(field(&headers, "Subject"), Some("Hello, world!"));
    assert_eq!(decoded_body(&eml), crlf_normalized("Message contents go here."));
}

#[test]
fn bodies_round_trip_through_every_encoding() {
    for body in [
        "plain ascii\nwith newlines",
        "Grüße aus Köln — schöne Grüße",
        "안녕하세요 세계, こんにちは、你好",
        "a very long line that will definitely need to be split into several pieces because it exceeds the allowed line length by far",
        "trailing spaces   \nand more   ",
        "has = signs = and \t tabs",
        "",
    ] {
        let eml = MessageBuilder::new()
            .from(("a", "a@b.c"))
            .subject("s")
            .text_body(body)
            .to_vec()
            .unwrap();
        assert_line_lengths(&eml);
        assert_line_endings(&eml);
        assert_eq!(decoded_body(&eml), crlf_normalized(body), "body {body:?} did not round-trip");
    }
}

#[test]
fn boundaries_are_balanced_and_do_not_collide() {
    let mut parts = Vec::new();
    for index in 0..5 {
        parts.push(MimePart::new("text/plain", format!("part number {index} with some content")));
    }
    let eml = MessageBuilder::new()
        .from(("a", "a@b.c"))
        .subject("s")
        .body(MimePart::new("multipart/mixed", parts))
        .to_vec()
        .unwrap();
    assert_line_lengths(&eml);
    assert_line_endings(&eml);

    let headers = String::from_utf8_lossy(header_block(&eml)).into_owned();
    let content_type = field(&headers, "Content-Type").unwrap();
    let boundary = content_type
        .split("boundary=\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("no boundary parameter");
    let text = String::from_utf8_lossy(&eml);
    assert_eq!(text.matches(&format!("\r\n--{boundary}\r\n")).count(), 5);
    assert_eq!(text.matches(&format!("\r\n--{boundary}--\r\n")).count(), 1);
}

#[test]
fn nested_multiparts_use_distinct_boundaries() {
    let eml = MessageBuilder::new()
        .from(("a", "a@b.c"))
        .body(MimePart::new(
            "multipart/mixed",
            vec![
                MimePart::new(
                    "multipart/alternative",
                    vec![
                        MimePart::new("text/plain", "text"),
                        MimePart::new("text/html", "<p>html</p>"),
                    ],
                ),
                MimePart::new("text/plain", "attachment").attachment("f.txt"),
            ],
        ))
        .to_vec()
        .unwrap();
    assert_line_lengths(&eml);
    let text = String::from_utf8_lossy(&eml);
    let boundaries: Vec<&str> = text
        .split("boundary=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap())
        .collect();
    assert!(boundaries.len() >= 2);
    let unique: std::collections::HashSet<&str> = boundaries.iter().copied().collect();
    assert_eq!(unique.len(), boundaries.len(), "duplicate boundary");
}

#[test]
fn non_ascii_filenames_use_rfc_2231() {
    let eml = MessageBuilder::new()
        .from(("a", "a@b.c"))
        .attachment("text/plain", "Jahresabschluß 2024.pdf", "data")
        .to_vec()
        .unwrap();
    assert_line_lengths(&eml);
    let text = String::from_utf8_lossy(&eml);
    assert!(text.contains("filename*=UTF-8''Jahresabschlu%C3%9F%202024.pdf"), "{text}");
    assert!(!text.contains("=?"), "RFC 2047 leaked into a parameter\n{text}");
}

#[test]
fn long_filenames_are_split_into_sections_within_78_columns() {
    let name =
        "Réunion d'équipe: résumé des décisions du trimestre et prochaines étapes du projet (version finale).pdf";
    let eml = MessageBuilder::new()
        .from(("a", "a@b.c"))
        .attachment("text/plain", name, "data")
        .to_vec()
        .unwrap();
    assert_line_lengths(&eml);
    let text = String::from_utf8_lossy(&eml);
    assert!(text.contains("filename*0*=UTF-8''"), "{text}");
    assert!(text.contains("filename*1*="), "{text}");
}

#[test]
fn group_addresses_end_with_a_semicolon() {
    use mail_builder::headers::address::Address;
    let eml = MessageBuilder::new()
        .to(Address::new_group(
            Some("My Group"),
            vec![
                Address::new_address(Some("A"), "a@b.c"),
                Address::new_address(Some("B"), "b@b.c"),
            ],
        ))
        .text_body("x")
        .to_vec()
        .unwrap();
    let headers = String::from_utf8_lossy(header_block(&eml)).into_owned();
    let to = field(&headers, "To").unwrap();
    assert!(to.trim_end().ends_with(';'), "group not terminated: {to:?}");
}

#[test]
fn no_bare_cr_or_lf_survives_in_bodies() {
    for body in ["a\rb", "a\n\rb", "a\r\r\nb\n\nc", "é\rx"] {
        let eml = MessageBuilder::new().text_body(body).to_vec().unwrap();
        assert_line_endings(&eml);
    }
}

#[test]
fn deeply_nested_parts_do_not_overflow_the_stack() {
    let mut part = MimePart::new("text/plain", "leaf");
    for _ in 0..200 {
        part = MimePart::new("multipart/mixed", vec![part]);
    }
    let eml = MessageBuilder::new().body(part).to_vec().unwrap();
    assert!(eml.ends_with(b"--\r\n"));
    assert_line_endings(&eml);
}

#[test]
fn dates_are_valid_for_a_range_of_timestamps() {
    fn is_rfc5322_date(value: &str) -> bool {
        // e.g. "Tue, 14 Nov 2023 22:13:20 +0000"
        let parts: Vec<&str> = value.split(' ').collect();
        parts.len() == 6
            && matches!(parts[0], "Sun," | "Mon," | "Tue," | "Wed," | "Thu," | "Fri," | "Sat,")
            && matches!(
                parts[2],
                "Jan" | "Feb" | "Mar" | "Apr" | "May" | "Jun" | "Jul" | "Aug" | "Sep" | "Oct" | "Nov" | "Dec"
            )
            && parts[3].len() == 4
            && parts[3].parse::<u16>().is_ok()
            && parts[5] == "+0000"
            && parts[4].split(':').count() == 3
            && parts[4]
                .split(':')
                .all(|field| field.len() == 2 && field.bytes().all(|byte| byte.is_ascii_digit()))
    }

    for timestamp in [0i64, 1, -1, -86_400, -2_208_988_800, 1_700_000_000, 2_145_916_800] {
        let value = Date::new(timestamp).to_rfc822();
        assert!(is_rfc5322_date(&value), "invalid date for {timestamp}: {value:?}");
    }
}
