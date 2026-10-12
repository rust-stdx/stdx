//! Resource behaviour: write-call amplification, writer ownership and
//! concurrency.
//!
//! A counting [`Writer`] makes regressions in the serialization path (per-byte
//! writes, an unbuffered `write_to`) fail the suite instead of silently slowing
//! it down. Everything here is safe Rust.

use std::sync::atomic::{AtomicUsize, Ordering};

use mail_builder::{MessageBuilder, Writer};

/// Counts how many times the serializer calls `write`.
struct CountingWriter {
    calls: usize,
    bytes: usize,
}

impl Writer for CountingWriter {
    fn write(&mut self, bytes: &[u8]) {
        self.calls += 1;
        self.bytes += bytes.len();
    }
}

#[test]
fn a_one_megabyte_attachment_is_not_written_byte_by_byte() {
    let payload = vec![7u8; 1_000_000];
    let mut writer = CountingWriter {
        calls: 0,
        bytes: 0,
    };
    MessageBuilder::new()
        .from(("a", "a@b.c"))
        .to("b@b.c")
        .subject("s")
        .attachment("application/octet-stream", "a.bin", payload)
        .serialize(&mut writer);
    // The Base64 payload alone is ~1.37 MB; anything close to one call per
    // quartet (hundreds of thousands of calls) is the regression we guard
    // against. The run-based encoder issues a handful of calls per line.
    assert!(writer.calls < 5_000, "1 MB attachment took {} write calls", writer.calls);
}

#[test]
fn bodies_are_not_written_byte_by_byte() {
    let mut writer = CountingWriter {
        calls: 0,
        bytes: 0,
    };
    MessageBuilder::new()
        .from(("a", "a@b.c"))
        .to("b@b.c")
        .subject("s")
        .text_body("안녕하세요 세계 ".repeat(20_000))
        .serialize(&mut writer);
    assert!(writer.calls < 5_000, "1 MB body took {} write calls", writer.calls);

    let mut writer = CountingWriter {
        calls: 0,
        bytes: 0,
    };
    MessageBuilder::new()
        .from(("a", "a@b.c"))
        .to("b@b.c")
        .subject("s")
        .text_body("Grüße aus Köln, schöne Grüße! = \t\r\n".repeat(5_000))
        .serialize(&mut writer);
    assert!(writer.calls < 20_000, "mixed body took {} write calls", writer.calls);
}

/// A `Drop` counter proves the serializer hands the writer back and does not
/// keep a reference to it.
#[test]
fn the_writer_is_dropped_after_serialization() {
    static DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DropCounter;

    impl Writer for DropCounter {
        fn write(&mut self, _bytes: &[u8]) {}
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    {
        let mut writer = DropCounter;
        MessageBuilder::new().subject("x").text_body("y").serialize(&mut writer);
    }
    assert_eq!(DROPS.load(Ordering::Relaxed), 1);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn concurrent_builds_do_not_race() {
    let handles: Vec<_> = (0..8)
        .map(|thread| {
            std::thread::spawn(move || {
                (0..1_000)
                    .filter(|index| {
                        MessageBuilder::new()
                            .from(("a", "a@b.c"))
                            .subject(format!("thread {thread} message {index}"))
                            .body(mail_builder::mime::MimePart::new(
                                "multipart/mixed",
                                vec![
                                    mail_builder::mime::MimePart::new("text/plain", "a"),
                                    mail_builder::mime::MimePart::new("text/plain", "b"),
                                ],
                            ))
                            .to_vec()
                            .unwrap()
                            .windows(4)
                            .any(|window| window == b"\r\n\r\n")
                    })
                    .count()
            })
        })
        .collect();
    for handle in handles {
        assert_eq!(handle.join().unwrap(), 1_000);
    }
}
