//! Regression tests for header injection (CWE-93).
//!
//! Every value that a caller passes in — subjects, addresses, message ids,
//! URLs, file names, custom header names and content-type parameters — must be
//! unable to terminate the field it belongs to and start a new one. The tests
//! here mirror the public advisory for CVE-2026-82661 (Nodemailer), where a
//! CRLF in a builder-produced header produced an attacker-chosen extra header.

use mail_builder::{
    MessageBuilder,
    headers::{address::Address, content_type::ContentType, message_id::MessageId, raw::Raw, text::Text, url::URL},
    mime::MimePart,
};

const EVIL: &str = "\r\nBcc: attacker@evil.com\r\n";

/// Returns the header block of a message, i.e. everything before the first
/// blank line.
fn header_block(eml: &[u8]) -> String {
    let end = eml
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .unwrap_or(eml.len());
    String::from_utf8_lossy(&eml[..end]).into_owned()
}

/// Asserts that the message contains no standalone `Bcc:` field and no bare CR
/// or LF inside its header block.
fn assert_safe(name: &str, eml: &[u8]) {
    let headers = header_block(eml);
    for line in headers.split("\r\n") {
        assert!(
            !line.starts_with("Bcc:") || line == "Bcc: attacker@evil.com",
            "{name}: injected header line {line:?}"
        );
        assert!(!line.contains('\r') && !line.contains('\n'), "{name}: bare CR/LF in {line:?}");
    }
    assert!(
        !headers.split("\r\n").any(|line| line == "Bcc: attacker@evil.com"),
        "{name}: injected Bcc header\n{headers}"
    );
}

fn build(name: &str, f: impl FnOnce() -> Vec<u8>) {
    let eml = f();
    assert_safe(name, &eml);
}

#[test]
fn subject_cannot_inject_headers() {
    for subject in [
        format!("Hello{EVIL}"),
        "Hello\nBcc: attacker@evil.com".to_string(),
        "Hello\rBcc: attacker@evil.com".to_string(),
        format!("{}{}", "a".repeat(120), EVIL),
        format!("{}{}", "a".repeat(5000), EVIL),
        format!("é{}", EVIL),
    ] {
        build("subject", || {
            MessageBuilder::new()
                .from(("John", "john@doe.com"))
                .subject(subject.clone())
                .text_body("body")
                .to_vec()
                .unwrap()
        });
    }
}

#[test]
fn addresses_cannot_inject_headers() {
    build("addr-spec", || {
        MessageBuilder::new()
            .to(format!("victim@x.com{EVIL}"))
            .text_body("body")
            .to_vec()
            .unwrap()
    });
    build("display-name", || {
        MessageBuilder::new()
            .to(Address::new_address(Some(format!("Evil{EVIL}")), "a@b.c"))
            .text_body("body")
            .to_vec()
            .unwrap()
    });
    build("group-name", || {
        MessageBuilder::new()
            .to(Address::new_group(
                Some(format!("G{EVIL}")),
                vec![Address::new_address(Some("a"), "a@b.c")],
            ))
            .text_body("body")
            .to_vec()
            .unwrap()
    });
}

#[test]
fn message_ids_cannot_inject_headers() {
    for value in [
        format!("abc{EVIL}"),
        format!("<a@b>{EVIL}"),
        format!("<a@b>{EVIL}<c@d>"),
    ] {
        build("message-id", || {
            MessageBuilder::new()
                .message_id(MessageId::new(value.clone()))
                .text_body("body")
                .to_vec()
                .unwrap()
        });
        build("in-reply-to", || {
            MessageBuilder::new()
                .in_reply_to(MessageId::new(value.clone()))
                .text_body("body")
                .to_vec()
                .unwrap()
        });
    }
}

#[test]
fn urls_and_custom_headers_cannot_inject_headers() {
    build("url", || {
        MessageBuilder::new()
            .header("List-Archive", URL::new(format!("http://x/{EVIL}")))
            .text_body("body")
            .to_vec()
            .unwrap()
    });
    build("header-name", || {
        MessageBuilder::new()
            .header(format!("X-Evil{EVIL}X-Ok"), Text::new("v"))
            .text_body("body")
            .to_vec()
            .unwrap()
    });
    build("raw-header", || {
        MessageBuilder::new()
            .header("X-Anything", Raw::new(format!("v{EVIL}")))
            .text_body("body")
            .to_vec()
            .unwrap()
    });
}

#[test]
fn content_type_values_cannot_inject_headers() {
    build("filename", || {
        MessageBuilder::new()
            .attachment("text/plain", format!("file{EVIL}.txt"), "data")
            .to_vec()
            .unwrap()
    });
    build("content-type", || {
        let mut part = MimePart::new("multipart/mixed", vec![MimePart::new("text/plain", "a")]);
        part.headers.clear();
        part.headers
            .push(("Content-Type".into(), ContentType::new(format!("text/plain{EVIL}")).into()));
        MessageBuilder::new().body(part).to_vec().unwrap()
    });
    build("attribute-key", || {
        let mut part = MimePart::new("text/plain", "a");
        part.headers.clear();
        part.headers.push((
            "Content-Type".into(),
            ContentType::new("text/plain")
                .attribute(format!("key{EVIL}x"), "v")
                .into(),
        ));
        MessageBuilder::new().body(part).to_vec().unwrap()
    });
    build("part-header-name", || {
        let part = MimePart::new("text/plain", "a").header(format!("X-Evil{EVIL}"), Text::new("v"));
        MessageBuilder::new().body(part).to_vec().unwrap()
    });
}

/// A value that merely looks like an RFC 2047 encoded word must not be
/// presented as one: RFC 2047 section 7 says any such run "MUST" be a valid
/// encoded word, so the builder has to encode or otherwise neutralise it.
#[test]
fn encoded_word_lookalikes_are_neutralised() {
    let eml = MessageBuilder::new()
        .subject("=?utf-8?B?SGVsbG8=?= is my text")
        .text_body("body")
        .to_vec()
        .unwrap();
    let headers = header_block(&eml);
    let subject = headers.split("\r\n").find(|line| line.starts_with("Subject:")).unwrap();
    // The literal text must not survive verbatim (a decoder would turn it into
    // "Hello is my text").
    assert!(
        !subject.contains("=?utf-8?B?SGVsbG8=?="),
        "lookalike encoded word leaked into {subject:?}"
    );
}
