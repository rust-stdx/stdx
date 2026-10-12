//! Deterministic fuzz test.
//!
//! Feeds hostile random values into every public text sink and checks that the
//! builder never panics and that the output keeps its structural invariants.
//! The generator is a seeded LCG so failures are reproducible.

use std::panic;

use mail_builder::{
    MessageBuilder,
    headers::{address::Address, content_type::ContentType, message_id::MessageId, raw::Raw, text::Text, url::URL},
    mime::{BodyPart, MimePart},
};

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }

    fn byte(&mut self) -> u8 {
        const INTERESTING: &[u8] = b"\r\n\t :=?\"\\;<>@,/.\x00\x0b\x07\x7f\xa9\x0d\x0a";
        if self.next().is_multiple_of(2) {
            INTERESTING[self.next() as usize % INTERESTING.len()]
        } else {
            (self.next() % 256) as u8
        }
    }

    fn string(&mut self, max: usize) -> String {
        let len = (self.next() as usize) % max;
        (0..len).map(|_| self.byte() as char).collect()
    }
}

fn random_address(rng: &mut Lcg, depth: u32) -> Address<'static> {
    if depth == 0 {
        return Address::new_address(Some(rng.string(20)), rng.string(20));
    }
    match rng.next() % 3 {
        0 => Address::new_address(Some(rng.string(20)), rng.string(20)),
        1 => Address::new_list((0..rng.next() % 4).map(|_| random_address(rng, depth - 1)).collect()),
        _ => Address::new_group(
            Some(rng.string(10)),
            (0..rng.next() % 4).map(|_| random_address(rng, depth - 1)).collect(),
        ),
    }
}

/// Checks the structural invariants that must hold for every generated message.
fn check_invariants(eml: &[u8]) {
    let separator = eml
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .expect("no header/body separator");
    let headers = &eml[..separator];
    let text = String::from_utf8_lossy(headers);
    let mut last = 0;
    for line in text.split("\r\n") {
        assert!(!line.contains('\r') && !line.contains('\n'), "bare CR/LF: {line:?}");
        assert!(
            line.starts_with(' ') || line.starts_with('\t') || line.contains(": "),
            "malformed header line: {line:?}"
        );
        assert!(line.len() <= 998, "header line too long");
        last += 1;
    }
    assert!(last > 0);
    // The body must contain only CRLF line breaks.
    let body = &eml[separator + 4..];
    let mut at = 0;
    while at < body.len() {
        match body[at] {
            b'\r' => {
                assert_eq!(body.get(at + 1), Some(&b'\n'), "bare CR in body");
                at += 2;
            }
            b'\n' => panic!("bare LF in body"),
            _ => at += 1,
        }
    }
}

#[test]
fn hostile_input_never_panics_or_breaks_structure() {
    let mut rng = Lcg(0x1234_5678_9abc_def0);

    for _ in 0..2_000 {
        let name = format!("X-{}", rng.string(100));
        let subject = rng.string(200);
        let addr_name = rng.string(100);
        let addr_email = rng.string(80);
        let mid = rng.string(80);
        let url = rng.string(80);
        let body = rng.string(300);
        let filename = rng.string(80);
        let content_type = rng.string(40);
        let raw_content_type = rng.string(120);
        let attribute = rng.string(60);

        let outcome = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let mut message = MessageBuilder::new()
                .from((addr_name, addr_email))
                .to(random_address(&mut rng, 3))
                .subject(subject)
                .header(name, Text::new("value"))
                .message_id(MessageId::new(mid))
                .header("List-Archive", URL::new(url))
                .attachment(content_type, filename, body);

            if !raw_content_type.is_empty() {
                let mut part = MimePart::new("multipart/mixed", vec![MimePart::new("text/plain", attribute.clone())]);
                part.headers.clear();
                part.headers
                    .push(("Content-Type".into(), Raw::new(raw_content_type).into()));
                message = message.body(part);
            } else if !attribute.is_empty() {
                let part = MimePart::new("text/plain", "x")
                    .header("Content-Type", ContentType::new("text/plain").attribute("note", attribute));
                message = message.body(part);
            } else {
                message = message.body(MimePart::new(
                    "multipart/mixed",
                    vec![
                        MimePart::new("text/plain", "a"),
                        MimePart::new("text/plain", BodyPart::Binary(b"b".as_slice().into())),
                    ],
                ));
            }

            message.to_vec().unwrap()
        }));

        let eml = outcome.expect("builder panicked on hostile input");
        check_invariants(&eml);
    }
}
