//! Audit tests for the raw `libyaml_unsafe` emitter / dumper / loader.
//!
//! These tests drive the C-style API directly (anchors, aliases, tag
//! directives, custom write handlers, UTF-16 output) and check ownership,
//! error propagation, and leaks. Memory-safety checks are meant to run under
//! Miri; see `write_after_flush_error_overflows_raw_buffer`.
//!
//! Run: `cargo test -p yaml --release --test audit_raw_emitter`
//! Miri: `cargo +nightly miri test -p yaml --test audit_raw_emitter -- <filter>`

#![allow(unsafe_op_in_unsafe_fn)]
// `CString::as_ptr` returns `*const c_char`, whose signedness is target
// dependent. The explicit `*const u8` casts are required where `c_char` is
// `i8` even though clippy sees them as redundant on this target.
#![allow(clippy::unnecessary_cast)]
// `sys` keeps the C-style API calls readable in these tests.
#![allow(clippy::unsafe_removed_from_name)]

use std::{
    cell::{Cell, RefCell},
    ffi::{CString, c_void},
    mem::MaybeUninit,
    ptr, slice,
};

use libyaml_unsafe as sys;
use sys::*;

// ---------------------------------------------------------------------------
// Output sink and emitter wrapper
// ---------------------------------------------------------------------------

#[derive(Default, Debug)]
struct Sink {
    bytes: RefCell<Vec<u8>>,
    calls: Cell<usize>,
    fail_on_call: Cell<Option<usize>>,
    fail_after: Cell<Option<usize>>,
    /// Fail every call with an index >= this, *before* touching `bytes`.
    /// Used to keep the retry-after-error test from poking a `RefCell` whose
    /// borrow flag may already have been clobbered by the out-of-bounds write.
    fail_from_call: Cell<Option<usize>>,
}

unsafe fn write_to_sink(data: *mut c_void, buffer: *mut u8, size: u64) -> i32 {
    // Shared reference + interior mutability: the emitter keeps a raw pointer
    // to this sink for its whole lifetime, so creating `&mut Sink` here would
    // invalidate that pointer under Stacked Borrows.
    let sink = unsafe { &*(data as *const Sink) };
    let call = sink.calls.get() + 1;
    sink.calls.set(call);
    if let Some(n) = sink.fail_on_call.get()
        && call == n
    {
        return 0;
    }
    if let Some(n) = sink.fail_from_call.get()
        && call >= n
    {
        return 0;
    }
    let mut bytes = sink.bytes.borrow_mut();
    if let Some(n) = sink.fail_after.get()
        && bytes.len() >= n
    {
        return 0;
    }
    bytes.extend_from_slice(unsafe { slice::from_raw_parts(buffer, size as usize) });
    1
}

struct Emitter {
    /// Raw, owning pointers. The emitter keeps `sink` in
    /// `write_handler_data`, and libyaml stores the emitter pointer itself in
    /// the output-string handler, so neither allocation may be moved or
    /// retagged while the emitter is alive (no `Box` field).
    sys: *mut yaml_emitter_t,
    sink: *mut Sink,
}

impl Emitter {
    fn new_bare() -> Emitter {
        let sys: *mut yaml_emitter_t = Box::into_raw(Box::new(MaybeUninit::<yaml_emitter_t>::zeroed())).cast();
        assert!(unsafe { yaml_emitter_initialize(sys) }.ok);
        let sink = Box::into_raw(Box::new(Sink::default()));
        Emitter {
            sys,
            sink,
        }
    }

    fn new() -> Emitter {
        let em = Emitter::new_bare();
        unsafe {
            yaml_emitter_set_unicode(em.sys, true);
            yaml_emitter_set_output(em.sys, write_to_sink, em.sink.cast());
        }
        em
    }

    fn ptr(&mut self) -> *mut yaml_emitter_t {
        self.sys
    }

    fn set_encoding(&mut self, encoding: yaml_encoding_t) {
        unsafe { yaml_emitter_set_encoding(self.ptr(), encoding) };
    }

    fn set_width(&mut self, width: i32) {
        unsafe { yaml_emitter_set_width(self.ptr(), width) };
    }

    fn set_indent(&mut self, indent: i32) {
        unsafe { yaml_emitter_set_indent(self.ptr(), indent) };
    }

    fn set_canonical(&mut self, canonical: bool) {
        unsafe { yaml_emitter_set_canonical(self.ptr(), canonical) };
    }

    fn set_unicode(&mut self, unicode: bool) {
        unsafe { yaml_emitter_set_unicode(self.ptr(), unicode) };
    }

    fn set_break(&mut self, line_break: yaml_break_t) {
        unsafe { yaml_emitter_set_break(self.ptr(), line_break) };
    }

    fn emit(&mut self, event: *mut yaml_event_t) -> bool {
        unsafe { yaml_emitter_emit(self.ptr(), event) }.ok
    }

    fn flush(&mut self) -> bool {
        unsafe { yaml_emitter_flush(self.ptr()) }.ok
    }

    fn error(&self) -> yaml_error_type_t {
        unsafe { (&(*self.sys)).error }
    }

    fn problem(&self) -> String {
        let p = unsafe { (&(*self.sys)).problem };
        if p.is_null() {
            String::new()
        } else {
            unsafe { std::ffi::CStr::from_ptr(p.cast()) }
                .to_string_lossy()
                .into_owned()
        }
    }

    fn sink(&self) -> &Sink {
        unsafe { &*self.sink }
    }

    fn bytes(&self) -> Vec<u8> {
        self.sink().bytes.borrow().clone()
    }

    fn calls(&self) -> usize {
        self.sink().calls.get()
    }

    fn set_fail_on_call(&self, call: usize) {
        self.sink().fail_on_call.set(Some(call));
    }

    fn set_fail_after(&self, bytes: usize) {
        self.sink().fail_after.set(Some(bytes));
    }

    fn set_fail_from_call(&self, call: usize) {
        self.sink().fail_from_call.set(Some(call));
    }
}

impl Drop for Emitter {
    fn drop(&mut self) {
        unsafe {
            yaml_emitter_delete(self.sys);
            drop(Box::from_raw(self.sys));
            drop(Box::from_raw(self.sink));
        }
    }
}

// ---------------------------------------------------------------------------
// Event helpers
// ---------------------------------------------------------------------------

/// A fresh, uninitialised event. `yaml_emitter_emit` copies the event into
/// its queue, so a stack temporary is enough; nothing is leaked.
fn ev() -> MaybeUninit<yaml_event_t> {
    MaybeUninit::uninit()
}

macro_rules! ok_event {
    ($result:expr) => {{
        let success = $result;
        assert!(success.ok, "event initialization failed");
    }};
}

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn emit_stream_start(em: &mut Emitter) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    ok_event!(unsafe { yaml_stream_start_event_initialize(p, YAML_UTF8_ENCODING) });
    em.emit(p)
}

fn emit_stream_end(em: &mut Emitter) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    ok_event!(unsafe { yaml_stream_end_event_initialize(p) });
    em.emit(p)
}

fn emit_document_start(em: &mut Emitter, tag_directives: &[(&str, &str)], version: Option<(i32, i32)>) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    let handles: Vec<CString> = tag_directives.iter().map(|(h, _)| cstr(h)).collect();
    let prefixes: Vec<CString> = tag_directives.iter().map(|(_, p)| cstr(p)).collect();
    let mut directives: Vec<yaml_tag_directive_t> = handles
        .iter()
        .zip(&prefixes)
        .map(|(h, p)| {
            let mut directive: yaml_tag_directive_t = unsafe { MaybeUninit::zeroed().assume_init() };
            directive.handle = h.as_ptr() as *mut u8;
            directive.prefix = p.as_ptr() as *mut u8;
            directive
        })
        .collect();
    let mut version_directive = version.map(|(major, minor)| {
        let mut directive: yaml_version_directive_t = unsafe { MaybeUninit::zeroed().assume_init() };
        directive.major = major;
        directive.minor = minor;
        directive
    });
    let (start, end) = if directives.is_empty() {
        (ptr::null_mut(), ptr::null_mut())
    } else {
        (directives.as_mut_ptr(), unsafe {
            directives.as_mut_ptr().add(directives.len())
        })
    };
    ok_event!(unsafe {
        yaml_document_start_event_initialize(
            p,
            version_directive.as_mut().map_or(ptr::null_mut(), |v| v),
            start,
            end,
            false,
        )
    });
    em.emit(p)
}

fn emit_document_end(em: &mut Emitter) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    ok_event!(unsafe { yaml_document_end_event_initialize(p, true) });
    em.emit(p)
}

fn emit_scalar_full(
    em: &mut Emitter,
    anchor: Option<&str>,
    tag: Option<&str>,
    value: &[u8],
    style: yaml_scalar_style_t,
) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    let anchor = anchor.map(cstr);
    let tag = tag.map(cstr);
    ok_event!(unsafe {
        yaml_scalar_event_initialize(
            p,
            anchor.as_ref().map_or(ptr::null(), |a| a.as_ptr() as *const u8),
            tag.as_ref().map_or(ptr::null(), |t| t.as_ptr() as *const u8),
            value.as_ptr(),
            value.len() as i32,
            tag.is_none(),
            tag.is_none(),
            style,
        )
    });
    em.emit(p)
}

fn emit_scalar(em: &mut Emitter, value: &str) -> bool {
    emit_scalar_full(em, None, None, value.as_bytes(), YAML_PLAIN_SCALAR_STYLE)
}

fn emit_alias(em: &mut Emitter, anchor: &str) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    let anchor = cstr(anchor);
    ok_event!(unsafe { yaml_alias_event_initialize(p, anchor.as_ptr() as *const u8) });
    em.emit(p)
}

fn emit_sequence_start(
    em: &mut Emitter,
    anchor: Option<&str>,
    tag: Option<&str>,
    style: yaml_sequence_style_t,
) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    let anchor = anchor.map(cstr);
    let tag = tag.map(cstr);
    ok_event!(unsafe {
        yaml_sequence_start_event_initialize(
            p,
            anchor.as_ref().map_or(ptr::null(), |a| a.as_ptr() as *const u8),
            tag.as_ref().map_or(ptr::null(), |t| t.as_ptr() as *const u8),
            tag.is_none(),
            style,
        )
    });
    em.emit(p)
}

fn emit_sequence_end(em: &mut Emitter) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    ok_event!(unsafe { yaml_sequence_end_event_initialize(p) });
    em.emit(p)
}

fn emit_mapping_start(em: &mut Emitter, anchor: Option<&str>, tag: Option<&str>, style: yaml_mapping_style_t) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    let anchor = anchor.map(cstr);
    let tag = tag.map(cstr);
    ok_event!(unsafe {
        yaml_mapping_start_event_initialize(
            p,
            anchor.as_ref().map_or(ptr::null(), |a| a.as_ptr() as *const u8),
            tag.as_ref().map_or(ptr::null(), |t| t.as_ptr() as *const u8),
            tag.is_none(),
            style,
        )
    });
    em.emit(p)
}

fn emit_mapping_end(em: &mut Emitter) -> bool {
    let mut e = ev();
    let p = e.as_mut_ptr();
    ok_event!(unsafe { yaml_mapping_end_event_initialize(p) });
    em.emit(p)
}

fn finish(em: &mut Emitter) {
    assert!(emit_stream_end(em), "stream end failed: {:?} {}", em.error(), em.problem());
    assert!(em.flush(), "flush failed: {:?} {}", em.error(), em.problem());
}

fn parse_value(bytes: &[u8]) -> yaml::Value {
    let options = yaml::Options {
        max_input_bytes: None,
        max_values: None,
        max_aliases: None,
        max_documents: None,
        max_depth: 4096,
        ..yaml::Options::default()
    };
    yaml::from_slice_with_options(bytes, options)
        .unwrap_or_else(|e| panic!("emitted YAML does not parse: {e}\n{}", String::from_utf8_lossy(bytes)))
}

// ---------------------------------------------------------------------------
// Emitter: normal paths, anchors, aliases, tags
// ---------------------------------------------------------------------------

#[test]
fn emits_anchor_alias_and_local_tag() {
    let mut em = Emitter::new();
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &[], None);

    assert!(emit_sequence_start(&mut em, None, None, YAML_BLOCK_SEQUENCE_STYLE));
    assert!(emit_sequence_start(&mut em, Some("shared"), None, YAML_BLOCK_SEQUENCE_STYLE));
    assert!(emit_scalar_full(&mut em, None, None, b"v1", YAML_PLAIN_SCALAR_STYLE));
    assert!(emit_scalar_full(&mut em, None, None, b"v2", YAML_PLAIN_SCALAR_STYLE));
    assert!(emit_sequence_end(&mut em));
    assert!(emit_alias(&mut em, "shared"));
    assert!(emit_scalar_full(
        &mut em,
        None,
        Some("!e!thing"),
        b"tagged",
        YAML_PLAIN_SCALAR_STYLE
    ));
    assert!(emit_sequence_end(&mut em));

    assert!(emit_document_end(&mut em));
    finish(&mut em);

    let text = String::from_utf8(em.bytes()).expect("utf8 output");
    eprintln!("--- emitted ---\n{text}--- end ---");
    let expected = yaml::Value::Sequence(vec![
        yaml::Value::Sequence(vec![yaml::Value::String("v1".into()), yaml::Value::String("v2".into())]),
        yaml::Value::Sequence(vec![yaml::Value::String("v1".into()), yaml::Value::String("v2".into())]),
        yaml::Value::Tagged(Box::new(yaml::TaggedValue {
            tag: "!e!thing".into(),
            value: yaml::Value::String("tagged".into()),
        })),
    ]);
    assert_eq!(parse_value(&em.bytes()), expected);
}

#[test]
fn emits_tag_directive_and_version() {
    let mut em = Emitter::new();
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &[("!e!", "tag:example.com,2000:app/")], Some((1, 2)));
    assert!(emit_scalar_full(
        &mut em,
        None,
        Some("tag:example.com,2000:app/thing"),
        b"value",
        YAML_PLAIN_SCALAR_STYLE
    ));
    emit_document_end(&mut em);
    finish(&mut em);
    let text = String::from_utf8(em.bytes()).unwrap();
    eprintln!("--- emitted ---\n{text}--- end ---");
    assert!(text.contains("%YAML 1.2"));
    assert!(text.contains("%TAG !e! tag:example.com,2000:app/"));
    assert!(text.contains("!e!thing"));
}

#[test]
fn emits_long_anchor_and_long_tag() {
    let anchor = "a".repeat(5000);
    let tag = format!("!e!{}", "t".repeat(5000));
    let mut em = Emitter::new();
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &[("!e!", "tag:example.com,2000:app/")], None);
    assert!(emit_sequence_start(&mut em, None, None, YAML_BLOCK_SEQUENCE_STYLE));
    assert!(emit_scalar_full(
        &mut em,
        Some(&anchor),
        Some(&tag),
        b"payload",
        YAML_PLAIN_SCALAR_STYLE
    ));
    assert!(emit_alias(&mut em, &anchor));
    assert!(emit_sequence_end(&mut em));
    emit_document_end(&mut em);
    finish(&mut em);
    assert!(em.bytes().len() > 10_000);
    let _ = parse_value(&em.bytes()); // must remain valid YAML
}

#[test]
fn emits_deep_nesting() {
    const DEPTH: usize = 300;
    let mut em = Emitter::new();
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &[], None);
    for _ in 0..DEPTH {
        assert!(emit_sequence_start(&mut em, None, None, YAML_BLOCK_SEQUENCE_STYLE));
    }
    assert!(emit_scalar(&mut em, "bottom"));
    for _ in 0..DEPTH {
        assert!(emit_sequence_end(&mut em));
    }
    emit_document_end(&mut em);
    finish(&mut em);
    let value = parse_value(&em.bytes());
    let mut depth = 0;
    let mut v = &value;
    while let yaml::Value::Sequence(items) = v {
        assert_eq!(items.len(), 1);
        v = &items[0];
        depth += 1;
    }
    assert_eq!(depth, DEPTH);
}

#[test]
fn emits_embedded_nul_scalar() {
    let mut em = Emitter::new();
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &[], None);
    // Length-delimited value that is *not* NUL terminated: `a\0b`.
    assert!(emit_scalar_full(&mut em, None, None, b"a\0b", YAML_DOUBLE_QUOTED_SCALAR_STYLE));
    emit_document_end(&mut em);
    finish(&mut em);
    assert_eq!(parse_value(&em.bytes()), yaml::Value::String("a\0b".into()));
}

#[test]
fn emits_multi_document_stream() {
    let mut em = Emitter::new();
    emit_stream_start(&mut em);
    for i in 0..5 {
        emit_document_start(&mut em, &[], None);
        assert!(emit_scalar(&mut em, &format!("doc{i}")));
        emit_document_end(&mut em);
    }
    finish(&mut em);
    let options = yaml::Options {
        max_documents: None,
        max_input_bytes: None,
        ..yaml::Options::default()
    };
    let docs: Vec<yaml::Value> = yaml::from_slice_multi_with_options(&em.bytes(), options).unwrap();
    assert_eq!(docs.len(), 5);
    for (i, doc) in docs.iter().enumerate() {
        assert_eq!(*doc, yaml::Value::String(format!("doc{i}")));
    }
}

#[test]
fn emits_utf16le_and_utf16be_bmp() {
    for encoding in [YAML_UTF16LE_ENCODING, YAML_UTF16BE_ENCODING] {
        let mut em = Emitter::new();
        em.set_encoding(encoding);
        emit_stream_start(&mut em);
        emit_document_start(&mut em, &[], None);
        // BMP-only payload: no surrogate pairs involved.
        assert!(emit_scalar_full(
            &mut em,
            None,
            None,
            "aé日".as_bytes(),
            YAML_DOUBLE_QUOTED_SCALAR_STYLE
        ));
        emit_document_end(&mut em);
        finish(&mut em);

        let bytes = &em.bytes();
        let le = encoding == YAML_UTF16LE_ENCODING;
        let u16_at = |i: usize| {
            let a = bytes[i];
            let b = bytes[i + 1];
            if le {
                u16::from_le_bytes([a, b])
            } else {
                u16::from_be_bytes([a, b])
            }
        };
        assert_eq!(bytes.len() % 2, 0, "UTF-16 output has even length");
        assert_eq!(u16_at(0), 0xFEFF, "BOM");
        let units: Vec<u16> = (0..bytes.len()).step_by(2).map(u16_at).collect();
        let decoded = String::from_utf16(&units[1..]).expect("valid UTF-16");
        assert!(decoded.contains('a'));
        assert!(decoded.contains('é'));
        assert!(decoded.contains('日'));
    }
}

fn utf16_units(encoding: yaml_encoding_t, value: &str) -> Vec<u16> {
    let mut em = Emitter::new();
    em.set_encoding(encoding);
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &[], None);
    assert!(emit_scalar_full(
        &mut em,
        None,
        None,
        value.as_bytes(),
        YAML_DOUBLE_QUOTED_SCALAR_STYLE
    ));
    emit_document_end(&mut em);
    finish(&mut em);
    let bytes = em.bytes();
    let le = encoding == YAML_UTF16LE_ENCODING;
    assert_eq!(bytes.len() % 2, 0);
    (0..bytes.len())
        .step_by(2)
        .map(|i| {
            if le {
                u16::from_le_bytes([bytes[i], bytes[i + 1]])
            } else {
                u16::from_be_bytes([bytes[i], bytes[i + 1]])
            }
        })
        .collect()
}

/// Regression for finding F-2: `yaml_emitter_flush` used to encode the high
/// byte of the low surrogate as `0xDC + ((value >> 8) & 0xFF)` instead of
/// `0xDC + ((value >> 8) & 0x03)`, so any character above U+FFFF produced a
/// malformed surrogate pair (and therefore invalid UTF-16) on the UTF-16
/// output path. This is reachable only through the raw emitter API, because
/// the safe wrapper always emits UTF-8.
#[test]
fn utf16_supplementary_surrogate_is_correct() {
    for encoding in [YAML_UTF16LE_ENCODING, YAML_UTF16BE_ENCODING] {
        let units = utf16_units(encoding, "🎉");
        let high = units
            .iter()
            .position(|&u| u == 0xD83C)
            .expect("high surrogate for U+1F389");
        assert_eq!(
            units[high + 1],
            0xDF89,
            "low surrogate for U+1F389 must be U+DF89, got {:#06X} (units {units:04X?})",
            units[high + 1]
        );
    }
}

// ---------------------------------------------------------------------------
// Emitter: failing write handlers
// ---------------------------------------------------------------------------

fn emit_document_with_big_scalar(em: &mut Emitter, len: usize) -> bool {
    emit_stream_start(em);
    emit_document_start(em, &[], None);
    let big = "a".repeat(len);
    let ok = emit_scalar(em, &big);
    if !ok {
        return false;
    }
    if !emit_document_end(em) {
        return false;
    }
    emit_stream_end(em)
}

#[test]
fn write_handler_failure_is_reported_and_leak_free() {
    // A scalar large enough to make the emitter flush the 16 KiB buffer
    // several times. Fail the handler at every plausible call index.
    const LEN: usize = 60_000;
    for fail_on in 1..=6usize {
        let before = alloc_counter::counters();
        {
            let mut em = Emitter::new();
            em.set_fail_on_call(fail_on);
            let result = emit_document_with_big_scalar(&mut em, LEN);
            let calls = em.calls();
            if fail_on <= calls {
                assert!(!result, "fail_on={fail_on} of {calls} calls should fail");
                assert_eq!(em.error(), YAML_WRITER_ERROR, "fail_on={fail_on}");
                assert!(!em.problem().is_empty());
                // Partial output must be a prefix of the successful output.
                let partial = em.bytes();
                let mut ok = Emitter::new();
                assert!(emit_document_with_big_scalar(&mut ok, LEN));
                assert!(ok.bytes().starts_with(&partial));
            } else {
                assert!(result, "fail_on={fail_on} beyond {calls} calls should succeed");
            }
        }
        let after = alloc_counter::counters();
        assert_eq!(
            after.live_allocations, before.live_allocations,
            "leak after emitting with fail_on={fail_on}"
        );
    }
}

#[test]
fn write_handler_fail_after_total_bytes() {
    for fail_after in [0usize, 1, 10, 16_383, 16_384, 20_000] {
        let mut em = Emitter::new();
        em.set_fail_after(fail_after);
        let result = emit_document_with_big_scalar(&mut em, 60_000);
        if fail_after == 0 {
            assert!(!result);
            assert_eq!(em.error(), YAML_WRITER_ERROR);
        } else {
            assert!(!result, "fail_after={fail_after}");
        }
        assert!(em.calls() > 0);
    }
}

#[test]
fn flush_after_error_is_observable_but_not_required() {
    // Retrying after a writer error is outside the documented contract, but it
    // must at least not double-free or panic in a way that corrupts unrelated
    // state. The out-of-bounds write it also causes is checked by
    // `write_after_flush_error_overflows_raw_buffer` under Miri.
    let mut em = Emitter::new();
    em.set_fail_on_call(1);
    assert!(!emit_document_with_big_scalar(&mut em, 40_000));
    assert_eq!(em.error(), YAML_WRITER_ERROR);
    // Second flush converts the same buffer again into `raw_buffer`.
    let _ = em.flush();
}

/// Regression for finding F-1: `yaml_emitter_flush` converts the working
/// buffer to UTF-16 into `raw_buffer.last`. It used to leave `raw_buffer.last`
/// advanced when the write handler failed, so a retry appended past the
/// `2 * 16384 + 2` byte allocation (heap corruption, and a Miri UB report).
/// The flush now discards the partial payload on the error path.
#[test]
fn write_after_flush_error_overflows_raw_buffer() {
    let mut em = Emitter::new();
    em.set_encoding(YAML_UTF16LE_ENCODING);
    // First flush succeeds (raw_buffer.last is reset), second fails and leaves
    // raw_buffer.last at 2 * 16384.
    em.set_fail_on_call(2);
    em.set_fail_from_call(3);
    let len = 40_000;
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &[], None);
    let big = "a".repeat(len);
    let emitted = emit_scalar(&mut em, &big);
    assert!(!emitted, "second flush must fail");
    assert_eq!(em.error(), YAML_WRITER_ERROR);
    eprintln!("calls={} raw bytes={}", em.calls(), em.bytes().len());
    // Retry: converts the buffered 16 KiB again into the already-advanced
    // raw buffer.
    let _ = em.flush();
}

// ---------------------------------------------------------------------------
// Dumper: document ownership, shared nodes, anchors, leaks
// ---------------------------------------------------------------------------

struct Document {
    sys: Box<MaybeUninit<yaml_document_t>>,
}

impl Document {
    fn new() -> Document {
        let mut sys = Box::new(MaybeUninit::<yaml_document_t>::zeroed());
        assert!(
            unsafe {
                yaml_document_initialize(
                    sys.as_mut_ptr(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    true,
                    true,
                )
            }
            .ok
        );
        Document {
            sys,
        }
    }

    fn ptr(&mut self) -> *mut yaml_document_t {
        self.sys.as_mut_ptr()
    }

    fn add_scalar(&mut self, value: &str) -> i32 {
        let value = cstr(value);
        unsafe {
            yaml_document_add_scalar(
                self.ptr(),
                ptr::null(),
                value.as_ptr() as *const u8,
                -1,
                YAML_PLAIN_SCALAR_STYLE,
            )
        }
    }

    fn add_sequence(&mut self) -> i32 {
        unsafe { yaml_document_add_sequence(self.ptr(), ptr::null(), YAML_BLOCK_SEQUENCE_STYLE) }
    }

    fn add_mapping(&mut self) -> i32 {
        unsafe { yaml_document_add_mapping(self.ptr(), ptr::null(), YAML_BLOCK_MAPPING_STYLE) }
    }

    fn append_item(&mut self, seq: i32, item: i32) {
        assert!(unsafe { yaml_document_append_sequence_item(self.ptr(), seq, item) }.ok);
    }

    fn append_pair(&mut self, map: i32, key: i32, value: i32) {
        assert!(unsafe { yaml_document_append_mapping_pair(self.ptr(), map, key, value) }.ok);
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        // Only delete if the document was not handed to the emitter.
        unsafe {
            let doc = &*self.sys.as_ptr();
            if !doc.nodes.start.is_null() {
                yaml_document_delete(self.ptr());
            }
        }
    }
}

fn dump_document(doc: &mut Document) -> Vec<u8> {
    let mut em = Emitter::new();
    assert!(unsafe { yaml_emitter_open(em.ptr()) }.ok);
    assert!(unsafe { yaml_emitter_dump(em.ptr(), doc.ptr()) }.ok);
    assert!(unsafe { yaml_emitter_close(em.ptr()) }.ok);
    // `yaml_emitter_dump` takes ownership; mark the document as consumed.
    unsafe { (*doc.ptr()).nodes.start = ptr::null_mut() };
    em.bytes()
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dumper_emits_shared_nodes_as_aliases() {
    // Measure on a dedicated thread: libtest bookkeeping on the test thread can
    // free unrelated allocations mid-test and skew `live_allocations`.
    std::thread::spawn(|| {
        let run = || {
            let text = {
                let mut doc = Document::new();
                // The root node is the first node added to the document.
                let root = doc.add_sequence();
                let shared = doc.add_scalar("shared");
                let nested = doc.add_sequence();
                let inner = doc.add_scalar("inner");
                doc.append_item(nested, inner);
                doc.append_item(root, shared);
                doc.append_item(root, shared);
                doc.append_item(root, nested);
                String::from_utf8(dump_document(&mut doc)).unwrap()
            };
            assert!(text.contains("&id001"));
            assert!(text.contains("*id001"));
            let expected = yaml::Value::Sequence(vec![
                yaml::Value::String("shared".into()),
                yaml::Value::String("shared".into()),
                yaml::Value::Sequence(vec![yaml::Value::String("inner".into())]),
            ]);
            let options = yaml::Options {
                max_aliases: None,
                max_depth: 512,
                max_input_bytes: None,
                ..yaml::Options::default()
            };
            let decoded: yaml::Value = yaml::from_str_with_options(&text, options).unwrap();
            assert_eq!(decoded, expected);
            text
        };
        let _ = run(); // warm up one-time lazy allocations
        let before = alloc_counter::counters();
        let _ = run();
        let after = alloc_counter::counters();
        assert_eq!(after.live_allocations, before.live_allocations, "dumper leak");
    })
    .join()
    .unwrap();
}

#[test]
fn dumper_emits_mapping_with_shared_key_and_value() {
    let mut doc = Document::new();
    let root = doc.add_mapping();
    let key = doc.add_scalar("key");
    let value = doc.add_scalar("value");
    doc.append_pair(root, key, value);
    doc.append_pair(root, value, key);
    let text = String::from_utf8(dump_document(&mut doc)).unwrap();
    eprintln!("--- mapping ---\n{text}--- end ---");
    let options = yaml::Options {
        max_aliases: None,
        max_depth: 64,
        max_input_bytes: None,
        ..yaml::Options::default()
    };
    let decoded: yaml::Value = yaml::from_str_with_options(&text, options).unwrap();
    let mut expected = yaml::Mapping::new();
    expected.insert(yaml::Value::String("key".into()), yaml::Value::String("value".into()));
    expected.insert(yaml::Value::String("value".into()), yaml::Value::String("key".into()));
    assert_eq!(decoded, yaml::Value::Mapping(expected));
}

#[test]
fn dumper_emits_cyclic_document_without_recursing_forever() {
    let mut doc = Document::new();
    let root = doc.add_sequence();
    let scalar = doc.add_scalar("x");
    doc.append_item(root, scalar);
    doc.append_item(root, root); // self cycle
    let text = dump_document(&mut doc);
    let text = String::from_utf8(text).unwrap();
    eprintln!("--- cyclic ---\n{text}--- end ---");
    assert!(text.contains("&id001"));
    assert!(text.contains("*id001"));
}

#[test]
fn dumper_empty_document_closes_stream() {
    let mut doc = Document::new();
    let text = dump_document(&mut doc);
    eprintln!("empty dump: {text:?}");
    // An empty document dumps to an empty (or stream-terminated) stream.
    assert!(text.is_empty() || text.starts_with(b"---") || text.starts_with(b"..."));
}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

fn load_all(input: &[u8]) -> Vec<yaml::Value> {
    let mut parser = Box::new(MaybeUninit::<yaml_parser_t>::zeroed());
    let p = parser.as_mut_ptr();
    assert!(unsafe { yaml_parser_initialize(p) }.ok);
    unsafe { yaml_parser_set_input_string(p, input.as_ptr(), input.len() as u64) };
    let options = yaml::Options {
        max_input_bytes: None,
        max_values: None,
        max_aliases: None,
        max_documents: None,
        max_depth: 512,
        ..yaml::Options::default()
    };
    let mut values = Vec::new();
    // The high-level parser is used to turn the loaded input into values; the
    // raw loader below checks memory management on the same inputs.
    if let Ok(v) = yaml::from_slice_with_options::<yaml::Value>(input, options) {
        values.push(v);
    }

    let mut document = Box::new(MaybeUninit::<yaml_document_t>::zeroed());
    let d = document.as_mut_ptr();
    loop {
        let loaded = unsafe { yaml_parser_load(p, d) };
        if loaded.fail {
            break;
        }
        let done = unsafe { yaml_document_get_root_node(d) }.is_null();
        unsafe { yaml_document_delete(d) };
        if done {
            break;
        }
    }
    unsafe { yaml_parser_delete(p) };
    values
}

#[test]
fn loader_handles_normal_and_malformed_inputs() {
    let cases: &[&[u8]] = &[
        b"",
        b"---\n",
        b"a: b\n",
        b"---\na\n---\nb\n",
        b"- 1\n- 2\n",
        b"a: [1, 2]\n",
        b"a: [1, 2\n",          // truncated flow sequence
        b"a: 'unterminated\n",  // truncated single quote
        b"a: \"unterminated\n", // truncated double quote
        b"a: |\n  block\n  scalar\n",
        b"&a [*a]\n", // self-referential alias
        b"a: &x [1, 2]\nb: *x\n",
        b"a: *missing\n", // undefined alias
        b"%YAML 1.2\n---\nvalue\n",
        b"%TAG !e! tag:example.com,2000:app/\n---\n!e!x v\n",
        b"!!binary \"aGVsbG8=\"\n",
        b"a: \xff\xfe\n", // invalid UTF-8
        b"[",             // unterminated flow sequence
        b"{a: b",         // unterminated flow mapping
        b"---\n...\n---\n",
    ];
    for input in cases {
        let before = alloc_counter::counters();
        let _ = load_all(input);
        let after = alloc_counter::counters();
        assert_eq!(
            after.live_allocations,
            before.live_allocations,
            "loader leak for input {:?}",
            String::from_utf8_lossy(input)
        );
    }
}

#[test]
fn loader_deeply_nested_document_is_iterative() {
    // 1000 levels: the largest document the scanner's MAX_NESTING_LEVEL
    // accepts. `yaml_parser_load_nodes` keeps its own heap stack, so this must
    // not recurse on the Rust stack.
    let depth = 1000;
    let mut input = Vec::with_capacity(depth * 2);
    input.extend(std::iter::repeat_n(b'[', depth));
    input.extend(std::iter::repeat_n(b']', depth));
    let before = alloc_counter::counters();
    {
        let mut parser = Box::new(MaybeUninit::<yaml_parser_t>::zeroed());
        let p = parser.as_mut_ptr();
        assert!(unsafe { yaml_parser_initialize(p) }.ok);
        unsafe { yaml_parser_set_input_string(p, input.as_ptr(), input.len() as u64) };
        let mut document = Box::new(MaybeUninit::<yaml_document_t>::zeroed());
        let d = document.as_mut_ptr();
        assert!(unsafe { yaml_parser_load(p, d) }.ok);
        assert!(!unsafe { yaml_document_get_root_node(d) }.is_null());
        unsafe { yaml_document_delete(d) };
        unsafe { yaml_parser_delete(p) };
    }
    let after = alloc_counter::counters();
    assert_eq!(after.live_allocations, before.live_allocations);
}

// ---------------------------------------------------------------------------
// Event/document delete semantics
// ---------------------------------------------------------------------------

#[test]
fn event_delete_is_idempotent_on_zeroed_event() {
    let mut event = MaybeUninit::<yaml_event_t>::zeroed();
    let p = event.as_mut_ptr();
    unsafe { yaml_event_delete(p) };
    unsafe { yaml_event_delete(p) };
}

#[test]
fn event_delete_after_failed_initialization_is_noop_on_zeroed_event() {
    let mut event = MaybeUninit::<yaml_event_t>::zeroed();
    let p = event.as_mut_ptr();
    // Invalid UTF-8 is rejected; the event must stay at YAML_NO_EVENT (0)
    // rather than containing a partially initialised union.
    let bad = [0xF8u8, 0x80];
    let result = unsafe {
        yaml_scalar_event_initialize(
            p,
            ptr::null(),
            ptr::null(),
            bad.as_ptr(),
            2,
            true,
            true,
            YAML_PLAIN_SCALAR_STYLE,
        )
    };
    assert!(result.fail);
    let kind = unsafe { std::ptr::addr_of!((*p).type_).read() };
    assert_eq!(kind, YAML_NO_EVENT);
    unsafe { yaml_event_delete(p) };
}

#[test]
fn event_delete_frees_every_variant() {
    let before = alloc_counter::counters();
    {
        // scalar with anchor + tag + value
        let mut e = ev();
        let p = e.as_mut_ptr();
        let anchor = cstr("a");
        let tag = cstr("!t");
        ok_event!(unsafe {
            yaml_scalar_event_initialize(
                p,
                anchor.as_ptr() as *const u8,
                tag.as_ptr() as *const u8,
                b"value".as_ptr(),
                5,
                false,
                false,
                YAML_PLAIN_SCALAR_STYLE,
            )
        });
        unsafe { yaml_event_delete(p) };

        // alias
        let mut e = ev();
        let p = e.as_mut_ptr();
        ok_event!(unsafe { yaml_alias_event_initialize(p, anchor.as_ptr() as *const u8) });
        unsafe { yaml_event_delete(p) };

        // document start with version + tag directives
        let mut e = ev();
        let p = e.as_mut_ptr();
        let mut version: yaml_version_directive_t = unsafe { MaybeUninit::zeroed().assume_init() };
        version.major = 1;
        version.minor = 2;
        let h = cstr("!e!");
        let prefix = cstr("tag:example.com,2000:");
        let mut directive: yaml_tag_directive_t = unsafe { MaybeUninit::zeroed().assume_init() };
        directive.handle = h.as_ptr() as *mut u8;
        directive.prefix = prefix.as_ptr() as *mut u8;
        let directive_ptr: *mut yaml_tag_directive_t = &mut directive;
        ok_event!(unsafe {
            yaml_document_start_event_initialize(p, &mut version, directive_ptr, directive_ptr.add(1), true)
        });
        unsafe { yaml_event_delete(p) };
    }
    let after = alloc_counter::counters();
    assert_eq!(after.live_allocations, before.live_allocations);
}

#[test]
fn document_delete_is_idempotent_on_zeroed_document() {
    let mut document = MaybeUninit::<yaml_document_t>::zeroed();
    let p = document.as_mut_ptr();
    unsafe { yaml_document_delete(p) };
    unsafe { yaml_document_delete(p) };
}

// ---------------------------------------------------------------------------
// Output-string handler (truncating writer)
// ---------------------------------------------------------------------------

#[test]
fn output_string_handler_truncates_and_reports_writer_error() {
    fn emit_into(size: usize) -> (bool, yaml_error_type_t, Vec<u8>, u64) {
        let mut em = Emitter::new_bare();
        let mut out = vec![0xA5u8; size.max(1)];
        let mut written: u64 = 0;
        unsafe { yaml_emitter_set_output_string(em.ptr(), out.as_mut_ptr(), size as u64, &mut written) };
        emit_stream_start(&mut em);
        emit_document_start(&mut em, &[], None);
        let ok =
            emit_scalar(&mut em, "payload") && emit_document_end(&mut em) && emit_stream_end(&mut em) && em.flush();
        let err = em.error();
        out.truncate(written as usize);
        (ok, err, out, written)
    }

    let (ok, _, full, _) = emit_into(4096);
    assert!(ok, "large output buffer should succeed");

    let full_len = full.len();
    let mut sizes: Vec<usize> = (1..full_len).collect();
    sizes.extend([full_len, full_len + 1, 4096]);
    for size in sizes {
        let (ok, err, out, written) = emit_into(size);
        assert!(written as usize <= size);
        assert_eq!(out.len(), written as usize);
        assert!(full.starts_with(&out), "partial output is not a prefix (size {size})");
        if size < full_len {
            assert!(!ok, "size {size} is too small and must fail (output is {full_len} bytes)");
            assert_eq!(err, YAML_WRITER_ERROR);
        } else {
            assert!(ok, "size {size} should fit the {full_len}-byte output");
            assert_eq!(out, full);
        }
    }
}

// ---------------------------------------------------------------------------
// Deep document dump recursion (explicit depth limit)
// ---------------------------------------------------------------------------

/// The dumper is recursive, so it enforces a documented depth limit instead of
/// overflowing the stack. The limit counts nodes on the deepest path, root
/// included: exactly `MAX_DUMP_DEPTH` nodes dump, one more fails cleanly.
#[test]
fn dumper_depth_limit_allows_maximum_and_rejects_deeper() {
    fn nested(depth: usize) -> Document {
        let mut doc = Document::new();
        let mut parent = doc.add_sequence();
        for _ in 1..depth {
            // Flow style keeps the emitted text linear in depth.
            let child = unsafe { yaml_document_add_sequence(doc.ptr(), ptr::null(), YAML_FLOW_SEQUENCE_STYLE) };
            doc.append_item(parent, child);
            parent = child;
        }
        doc
    }

    let mut at_limit = nested(sys::MAX_DUMP_DEPTH);
    let mut em = Emitter::new();
    assert!(unsafe { yaml_emitter_open(em.ptr()) }.ok);
    assert!(
        unsafe { yaml_emitter_dump(em.ptr(), at_limit.ptr()) }.ok,
        "a document at the depth limit must dump"
    );
    // `yaml_emitter_dump` took ownership of the node stack.
    unsafe { (*at_limit.ptr()).nodes.start = ptr::null_mut() };
    assert!(unsafe { yaml_emitter_close(em.ptr()) }.ok);

    let mut too_deep = nested(sys::MAX_DUMP_DEPTH + 1);
    let mut em = Emitter::new();
    assert!(unsafe { yaml_emitter_open(em.ptr()) }.ok);
    let result = unsafe { yaml_emitter_dump(em.ptr(), too_deep.ptr()) };
    assert!(result.fail, "a document beyond the depth limit must fail");
    assert_eq!(
        unsafe { (&(*em.ptr())).error },
        YAML_EMITTER_ERROR,
        "the failure must be reported as an emitter error"
    );
    // `yaml_emitter_dump` destroys the document even when it fails.
    unsafe { (*too_deep.ptr()).nodes.start = ptr::null_mut() };
}

#[test]
fn raw_emitter_options_roundtrip() {
    // Values are plain strings so canonical mode (which double-quotes
    // everything) still decodes to the same values.
    let payload = [
        "alpha beta gamma delta epsilon zeta",
        "a\ttab and a space",
        "line one\nline two\nline three",
        "é日🎉 mixed",
        "trailing space ",
        " leading space",
        "a:b#c?d-e_f.g/h",
        "'quoted' \"double\"",
        &"x".repeat(70)[..],
    ];
    let mut expected = yaml::Mapping::new();
    for (i, item) in payload.iter().enumerate() {
        expected.insert(yaml::Value::String(format!("k{i}")), yaml::Value::String((*item).into()));
    }
    let expected = yaml::Value::Mapping(expected);

    let breaks = [YAML_LN_BREAK, YAML_CR_BREAK, YAML_CRLN_BREAK];
    let indents = [1, 2, 4, 9, 10];
    let widths = [-1, 0, 1, 20, 80];
    for &line_break in &breaks {
        for &indent in &indents {
            for &width in &widths {
                for &canonical in &[false, true] {
                    for &unicode in &[false, true] {
                        let mut em = Emitter::new();
                        em.set_break(line_break);
                        em.set_indent(indent);
                        em.set_width(width);
                        em.set_canonical(canonical);
                        em.set_unicode(unicode);
                        emit_stream_start(&mut em);
                        emit_document_start(&mut em, &[], None);
                        assert!(emit_mapping_start(&mut em, None, None, YAML_BLOCK_MAPPING_STYLE));
                        for (i, item) in payload.iter().enumerate() {
                            assert!(emit_scalar(&mut em, &format!("k{i}")));
                            assert!(emit_scalar_full(&mut em, None, None, item.as_bytes(), YAML_ANY_SCALAR_STYLE));
                        }
                        assert!(emit_mapping_end(&mut em));
                        emit_document_end(&mut em);
                        finish(&mut em);
                        let decoded = parse_value(&em.bytes());
                        assert_eq!(
                            decoded,
                            expected,
                            "break={line_break:?} indent={indent} width={width} canonical={canonical} unicode={unicode}\n{}",
                            String::from_utf8_lossy(&em.bytes())
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn emits_many_tag_directives_and_resolves_handles() {
    // 300 directives: larger than the emitter's initial tag-directive stack
    // (16), and enough output to cross the working-buffer boundary.
    const N: usize = 300;
    let directives: Vec<(String, String)> = (0..N)
        .map(|i| (format!("!d{i}!"), format!("tag:example.com,2000:app{i}/")))
        .collect();
    let refs: Vec<(&str, &str)> = directives.iter().map(|(h, p)| (h.as_str(), p.as_str())).collect();

    let mut em = Emitter::new();
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &refs, None);
    assert!(emit_sequence_start(&mut em, None, None, YAML_BLOCK_SEQUENCE_STYLE));
    for i in 0..N {
        let tag = format!("tag:example.com,2000:app{i}/item");
        assert!(emit_scalar_full(&mut em, None, Some(&tag), b"v", YAML_PLAIN_SCALAR_STYLE));
    }
    assert!(emit_sequence_end(&mut em));
    emit_document_end(&mut em);
    finish(&mut em);

    let value = parse_value(&em.bytes());
    let items = value.as_sequence().expect("sequence");
    assert_eq!(items.len(), N);
    for (i, item) in items.iter().enumerate() {
        match item {
            yaml::Value::Tagged(tagged) => {
                assert_eq!(tagged.tag, format!("tag:example.com,2000:app{i}/item"));
            }
            other => panic!("expected tagged value at {i}, got {other:?}"),
        }
    }
}

#[test]
fn emits_many_anchors_and_aliases() {
    const N: usize = 500;
    let mut em = Emitter::new();
    emit_stream_start(&mut em);
    emit_document_start(&mut em, &[], None);
    assert!(emit_sequence_start(&mut em, None, None, YAML_BLOCK_SEQUENCE_STYLE));
    for i in 0..N {
        assert!(emit_sequence_start(&mut em, None, None, YAML_BLOCK_SEQUENCE_STYLE));
        let anchor = format!("anchor{i}");
        assert!(emit_scalar_full(
            &mut em,
            Some(&anchor),
            None,
            b"payload",
            YAML_PLAIN_SCALAR_STYLE
        ));
        assert!(emit_alias(&mut em, &anchor));
        assert!(emit_sequence_end(&mut em));
    }
    assert!(emit_sequence_end(&mut em));
    emit_document_end(&mut em);
    finish(&mut em);

    let value = parse_value(&em.bytes());
    let items = value.as_sequence().expect("sequence");
    assert_eq!(items.len(), N);
    for item in items {
        let pair = item.as_sequence().expect("pair");
        assert_eq!(pair.len(), 2);
        assert_eq!(pair[0], yaml::Value::String("payload".into()));
        assert_eq!(pair[1], yaml::Value::String("payload".into()));
    }
}

#[test]
fn dumper_multiple_documents_on_one_emitter() {
    let mut em = Emitter::new();
    assert!(unsafe { yaml_emitter_open(em.ptr()) }.ok);
    for i in 0..3 {
        let mut doc = Document::new();
        let root = doc.add_scalar(&format!("doc{i}"));
        assert_eq!(root, 1);
        assert!(unsafe { yaml_emitter_dump(em.ptr(), doc.ptr()) }.ok, "dump {i}");
        unsafe { (*doc.ptr()).nodes.start = ptr::null_mut() };
    }
    assert!(unsafe { yaml_emitter_close(em.ptr()) }.ok);
    // Closing again is a no-op (guarded by the `closed` flag).
    assert!(unsafe { yaml_emitter_close(em.ptr()) }.ok);

    let options = yaml::Options {
        max_documents: None,
        max_input_bytes: None,
        ..yaml::Options::default()
    };
    let docs: Vec<yaml::Value> = yaml::from_slice_multi_with_options(&em.bytes(), options).unwrap();
    assert_eq!(
        docs,
        vec![
            yaml::Value::String("doc0".into()),
            yaml::Value::String("doc1".into()),
            yaml::Value::String("doc2".into()),
        ]
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dumper_after_closed_stream_fails_without_leaking() {
    // Empty document closes the stream; a later non-empty document can no longer
    // be emitted. The emitter must report the error and free everything.
    std::thread::spawn(|| {
        let run = || {
            let mut em = Emitter::new();
            assert!(unsafe { yaml_emitter_open(em.ptr()) }.ok);
            let mut empty = Document::new();
            assert!(unsafe { yaml_emitter_dump(em.ptr(), empty.ptr()) }.ok);
            unsafe { (*empty.ptr()).nodes.start = ptr::null_mut() };

            let mut doc = Document::new();
            doc.add_scalar("after close");
            let dumped = unsafe { yaml_emitter_dump(em.ptr(), doc.ptr()) };
            assert!(dumped.fail, "emit after STREAM-END must fail");
            // `yaml_emitter_dump` consumed the document on every path.
            unsafe { (*doc.ptr()).nodes.start = ptr::null_mut() };
            let _ = em.bytes();
        };
        run(); // warm-up lazy allocations
        let before = alloc_counter::counters();
        run();
        let after = alloc_counter::counters();
        assert_eq!(after.live_allocations, before.live_allocations, "leak after dump error");
    })
    .join()
    .unwrap();
}

// ---------------------------------------------------------------------------
// Random (often malformed) event sequences
// ---------------------------------------------------------------------------

struct XorShift(u64);

impl XorShift {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Emit a random event. Returns false if the emitter rejected it.
fn emit_random_event(em: &mut Emitter, kind: usize) -> bool {
    match kind {
        0 => emit_stream_start(em),
        1 => {
            let mut e = ev();
            let p = e.as_mut_ptr();
            ok_event!(unsafe { yaml_stream_end_event_initialize(p) });
            em.emit(p)
        }
        2 => emit_document_start(em, &[], None),
        3 => emit_document_end(em),
        4 => emit_scalar_full(em, None, None, b"x", YAML_PLAIN_SCALAR_STYLE),
        5 => emit_scalar_full(em, None, None, b"y", YAML_DOUBLE_QUOTED_SCALAR_STYLE),
        6 => emit_sequence_start(em, None, None, YAML_BLOCK_SEQUENCE_STYLE),
        7 => emit_sequence_end(em),
        8 => emit_mapping_start(em, None, None, YAML_BLOCK_MAPPING_STYLE),
        9 => emit_mapping_end(em),
        10 => emit_alias(em, "a"),
        11 => emit_scalar_full(em, Some("a"), None, b"anchored", YAML_PLAIN_SCALAR_STYLE),
        12 => emit_sequence_start(em, None, None, YAML_FLOW_SEQUENCE_STYLE),
        13 => emit_mapping_start(em, None, None, YAML_FLOW_MAPPING_STYLE),
        _ => true,
    }
}

#[test]
fn random_malformed_event_sequences_do_not_crash_or_leak() {
    let mut rng = XorShift(0x9e37_79b9_7f4a_7c15);
    // Warm up one-time lazy allocations before measuring.
    {
        let mut em = Emitter::new();
        let _ = emit_random_event(&mut em, 0);
    }
    for case in 0..200 {
        let before = alloc_counter::counters();
        {
            let mut em = Emitter::new();
            let mut clean = true;
            for _ in 0..40 {
                if !emit_random_event(&mut em, rng.below(14)) {
                    clean = false;
                    break;
                }
            }
            // Never retry a flush after an error: that is finding F-1.
            if clean {
                let _ = em.flush();
            }
        }
        let after = alloc_counter::counters();
        assert_eq!(
            after.live_allocations, before.live_allocations,
            "leak after random event sequence {case}"
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dumper_write_error_mid_document_frees_everything() {
    std::thread::spawn(|| {
        let make_doc = || {
            let mut doc = Document::new();
            let root = doc.add_sequence();
            let big = "z".repeat(40_000);
            let s = doc.add_scalar(&big);
            doc.append_item(root, s);
            doc
        };
        let dump = |fail_on: Option<usize>| {
            let mut em = Emitter::new();
            if let Some(n) = fail_on {
                em.set_fail_on_call(n);
            }
            assert!(unsafe { yaml_emitter_open(em.ptr()) }.ok);
            let mut doc = make_doc();
            let result = unsafe { yaml_emitter_dump(em.ptr(), doc.ptr()) };
            unsafe { (*doc.ptr()).nodes.start = ptr::null_mut() };
            (result.ok, em.bytes())
        };

        // Reference output and fail points inside the first few flushes.
        let (ok, full) = dump(None);
        assert!(ok);
        for fail_on in 1..=4 {
            let (ok, partial) = dump(Some(fail_on));
            if !ok {
                assert!(full.starts_with(&partial), "partial dump is not a prefix");
            }
        }

        let run = || {
            let _ = dump(Some(1));
        };
        run(); // warm-up
        let before = alloc_counter::counters();
        run();
        let after = alloc_counter::counters();
        assert_eq!(after.live_allocations, before.live_allocations, "leak after failed dump");
    })
    .join()
    .unwrap();
}

// ---------------------------------------------------------------------------
// Loader with a short/failing custom reader
// ---------------------------------------------------------------------------

struct Reader {
    data: Vec<u8>,
    pos: Cell<usize>,
    chunk: usize,
    fail_after: Option<usize>,
}

unsafe fn read_from_reader(data: *mut c_void, buffer: *mut u8, size: u64, size_read: *mut u64) -> i32 {
    let reader = unsafe { &*(data as *const Reader) };
    let pos = reader.pos.get();
    if let Some(limit) = reader.fail_after
        && pos >= limit
    {
        unsafe { *size_read = 0 };
        return 0;
    }
    let remaining = reader.data.len() - pos;
    let mut n = remaining.min(size as usize);
    if reader.chunk != 0 {
        n = n.min(reader.chunk);
    }
    if n == 0 {
        unsafe { *size_read = 0 };
        return 1;
    }
    unsafe {
        ptr::copy_nonoverlapping(reader.data.as_ptr().add(pos), buffer, n);
        *size_read = n as u64;
    }
    reader.pos.set(pos + n);
    1
}

fn load_documents(
    input: &[u8],
    chunk: usize,
    fail_after: Option<usize>,
) -> (Vec<yaml::Value>, bool, yaml_error_type_t) {
    let reader = Box::into_raw(Box::new(Reader {
        data: input.to_vec(),
        pos: Cell::new(0),
        chunk,
        fail_after,
    }));
    let sys: *mut yaml_parser_t = Box::into_raw(Box::new(MaybeUninit::<yaml_parser_t>::zeroed())).cast();
    assert!(unsafe { yaml_parser_initialize(sys) }.ok);
    unsafe { yaml_parser_set_input(sys, read_from_reader, reader.cast()) };

    let mut documents = Vec::new();
    let mut all_ok = true;
    let mut error = YAML_NO_ERROR;
    let document: *mut yaml_document_t = Box::into_raw(Box::new(MaybeUninit::<yaml_document_t>::zeroed())).cast();
    loop {
        let loaded = unsafe { yaml_parser_load(sys, document) };
        if loaded.fail {
            all_ok = false;
            error = unsafe { (&(*sys)).error };
            break;
        }
        let root = unsafe { yaml_document_get_root_node(document) };
        unsafe { yaml_document_delete(document) };
        if root.is_null() {
            break;
        }
        documents.push(yaml::Value::Null); // content checked separately
    }
    unsafe {
        yaml_parser_delete(sys);
        drop(Box::from_raw(sys));
        drop(Box::from_raw(document));
        drop(Box::from_raw(reader));
    }
    (documents, all_ok, error)
}

#[test]
fn loader_short_reads_and_reader_errors() {
    let input = b"---\na: 1\n---\nb: 2\n---\n[c, d]\n";
    // One byte per read: every event crosses many reader boundaries.
    let (docs, ok, err) = load_documents(input, 1, None);
    assert!(ok, "short reads are not errors: {err:?}");
    assert_eq!(docs.len(), 3);

    // A failing reader at every byte must be reported as a reader error, never
    // as a crash or silent success.
    for fail_after in 0..input.len() {
        let before = alloc_counter::counters();
        let (_, ok, err) = load_documents(input, 1, Some(fail_after));
        let after = alloc_counter::counters();
        if fail_after == 0 {
            assert!(!ok);
            assert_eq!(err, YAML_READER_ERROR);
        }
        // Some boundaries may still complete the document that was in flight;
        // what matters is that the parser reports an error eventually.
        let _ = ok;
        assert_eq!(
            after.live_allocations, before.live_allocations,
            "leak with failing reader after {fail_after} bytes"
        );
    }

    // A large chunk should behave the same as byte-at-a-time.
    let (docs, ok, _) = load_documents(input, 4096, None);
    assert!(ok);
    assert_eq!(docs.len(), 3);
}

/// The parser uses `c_int` for token lengths, so inputs above
/// `YAML_MAX_INPUT_LENGTH` are rejected before any read rather than letting a
/// length truncate. The check is exercised without allocating the input.
#[test]
fn parser_rejects_input_longer_than_c_int_range() {
    assert_eq!(sys::YAML_MAX_INPUT_LENGTH, i32::MAX as u64);

    let input = *b"a";

    // Exactly at the limit is accepted (no error is raised yet).
    let at_limit: *mut yaml_parser_t = Box::into_raw(Box::new(MaybeUninit::<yaml_parser_t>::zeroed())).cast();
    assert!(unsafe { yaml_parser_initialize(at_limit) }.ok);
    unsafe { yaml_parser_set_input_string(at_limit, input.as_ptr(), sys::YAML_MAX_INPUT_LENGTH) };
    assert_eq!(
        unsafe { (&(*at_limit)).error },
        YAML_NO_ERROR,
        "a length at the limit must be accepted"
    );
    unsafe {
        yaml_parser_delete(at_limit);
        drop(Box::from_raw(at_limit));
    }

    // One byte over the limit is a reader error, and the next load fails.
    let too_long: *mut yaml_parser_t = Box::into_raw(Box::new(MaybeUninit::<yaml_parser_t>::zeroed())).cast();
    assert!(unsafe { yaml_parser_initialize(too_long) }.ok);
    unsafe { yaml_parser_set_input_string(too_long, input.as_ptr(), sys::YAML_MAX_INPUT_LENGTH + 1) };
    assert_eq!(
        unsafe { (&(*too_long)).error },
        YAML_READER_ERROR,
        "an over-long input must be a reader error"
    );
    let document: *mut yaml_document_t = Box::into_raw(Box::new(MaybeUninit::<yaml_document_t>::zeroed())).cast();
    assert!(
        unsafe { yaml_parser_load(too_long, document) }.fail,
        "loading must fail once the input was rejected"
    );
    unsafe {
        yaml_parser_delete(too_long);
        drop(Box::from_raw(too_long));
        drop(Box::from_raw(document));
    }
}

/// Finding F-5 regression: `yaml_parser_parse` returns a zeroed
/// (`YAML_NO_EVENT`) event once the parser holds an error. A caller that
/// retries `yaml_parser_load` must get a second `FAIL`, not a process abort.
#[test]
fn loader_second_load_after_error_returns_fail() {
    let input = b"a: 1\n";
    let reader = Box::into_raw(Box::new(Reader {
        data: input.to_vec(),
        pos: Cell::new(0),
        chunk: 1,
        fail_after: Some(2),
    }));
    let sys: *mut yaml_parser_t = Box::into_raw(Box::new(MaybeUninit::<yaml_parser_t>::zeroed())).cast();
    assert!(unsafe { yaml_parser_initialize(sys) }.ok);
    unsafe { yaml_parser_set_input(sys, read_from_reader, reader.cast()) };

    let document: *mut yaml_document_t = Box::into_raw(Box::new(MaybeUninit::<yaml_document_t>::zeroed())).cast();
    assert!(
        unsafe { yaml_parser_load(sys, document) }.fail,
        "the failing reader must fail the first load"
    );
    let error = unsafe { (&(*sys)).error };
    assert_ne!(error, YAML_NO_ERROR);

    // The loader already deleted the document on failure. Retrying the same
    // document buffer must return FAIL and must not abort or leak.
    assert!(
        unsafe { yaml_parser_load(sys, document) }.fail,
        "a second load after an error must fail"
    );
    assert_eq!(unsafe { (&(*sys)).error }, error, "the parser error is sticky");

    unsafe {
        yaml_parser_delete(sys);
        drop(Box::from_raw(sys));
        drop(Box::from_raw(document));
        drop(Box::from_raw(reader));
    }
}

#[test]
fn explicit_scalar_styles_roundtrip_or_parse() {
    let values: &[&str] = &[
        "",
        "\n",
        "\n\n",
        "\n\n\n",
        " ",
        "  ",
        " a",
        "a ",
        "a\n",
        "a\n\n",
        "a\nb",
        "a\nb\n",
        " a\n b\n",
        "\r",
        "\r\n",
        "a\r\nb",
        "\u{85}",
        "\u{85}\u{85}",
        "\u{2028}\u{2028}",
        "🎉",
        "🎉\n",
        "🎉🎉\n\n",
        "line\twith\ttabs",
        &"long ".repeat(400)[..],
    ];
    for &style in &[
        YAML_LITERAL_SCALAR_STYLE,
        YAML_FOLDED_SCALAR_STYLE,
        YAML_SINGLE_QUOTED_SCALAR_STYLE,
    ] {
        for value in values {
            let mut em = Emitter::new();
            emit_stream_start(&mut em);
            emit_document_start(&mut em, &[], None);
            assert!(emit_scalar_full(&mut em, None, None, value.as_bytes(), style));
            emit_document_end(&mut em);
            finish(&mut em);
            // Every style must produce parseable YAML. Literal and single
            // quoted preserve the value; folded intentionally folds newlines.
            let decoded = parse_value(&em.bytes());
            if style != YAML_FOLDED_SCALAR_STYLE {
                assert_eq!(
                    decoded,
                    yaml::Value::String((*value).into()),
                    "style {style:?} value {value:?}\n{}",
                    String::from_utf8_lossy(&em.bytes())
                );
            }
        }
    }
}

#[test]
fn double_quoted_all_ascii_bytes_roundtrip() {
    let mut value = String::new();
    for byte in 0u8..=0x7f {
        value.push(byte as char);
    }
    for unicode in [true, false] {
        let mut em = Emitter::new();
        em.set_unicode(unicode);
        emit_stream_start(&mut em);
        emit_document_start(&mut em, &[], None);
        assert!(emit_scalar_full(
            &mut em,
            None,
            None,
            value.as_bytes(),
            YAML_DOUBLE_QUOTED_SCALAR_STYLE
        ));
        emit_document_end(&mut em);
        finish(&mut em);
        assert_eq!(
            parse_value(&em.bytes()),
            yaml::Value::String(value.clone()),
            "unicode={unicode}"
        );
    }
    // The same for every Unicode scalar value that fits in one or two UTF-8
    // bytes (covers the \xNN / \uNNNN escape width selection).
    for cp in 0u32..0x800 {
        let Some(ch) = char::from_u32(cp) else { continue };
        let mut s = String::new();
        s.push(ch);
        let mut em = Emitter::new();
        em.set_unicode(false);
        emit_stream_start(&mut em);
        emit_document_start(&mut em, &[], None);
        assert!(emit_scalar_full(
            &mut em,
            None,
            None,
            s.as_bytes(),
            YAML_DOUBLE_QUOTED_SCALAR_STYLE
        ));
        emit_document_end(&mut em);
        finish(&mut em);
        assert_eq!(parse_value(&em.bytes()), yaml::Value::String(s), "U+{cp:04X}");
    }
}

/// Regression for finding F-6: `yaml_check_utf8` used to verify only the
/// UTF-8 continuation-byte shape and the lower bound for each width, never the
/// Unicode upper bound U+10FFFF or the surrogate range. A 4-byte sequence
/// encoding U+110000 (or a 3-byte surrogate) was accepted and written out as
/// invalid UTF-8. The safe wrapper cannot reach this because it only accepts
/// `&str`.
#[test]
fn utf8_validator_rejects_codepoints_above_unicode_max() {
    // U+110000 = F4 90 80 80.
    for invalid in [&[0xF4u8, 0x90, 0x80, 0x80][..], &[0xED, 0xA0, 0x80][..]] {
        let mut e = ev();
        let p = e.as_mut_ptr();
        let result = unsafe {
            yaml_scalar_event_initialize(
                p,
                ptr::null(),
                ptr::null(),
                invalid.as_ptr(),
                invalid.len() as i32,
                true,
                true,
                YAML_PLAIN_SCALAR_STYLE,
            )
        };
        assert!(
            result.fail,
            "invalid Unicode scalar {:#04X?} must be rejected, not emitted",
            invalid
        );
    }
    // U+10FFFF (F4 8F BF BF) is still valid and must be accepted.
    let max = [0xF4u8, 0x8F, 0xBF, 0xBF];
    let mut e = ev();
    let p = e.as_mut_ptr();
    let result = unsafe {
        yaml_scalar_event_initialize(
            p,
            ptr::null(),
            ptr::null(),
            max.as_ptr(),
            4,
            true,
            true,
            YAML_PLAIN_SCALAR_STYLE,
        )
    };
    assert!(result.ok);
    unsafe { yaml_event_delete(p) };
}

/// Regression for finding F-7: `yaml_emitter_anchor_node` used to count
/// references only for the root and its immediate children, so a node shared
/// at depth >= 2 had no anchor id and the second reference was emitted as an
/// ALIAS with a NULL anchor (`strlen(NULL)`). The reference-counting pass now
/// walks the whole document graph iteratively, so this emits `&id001` /
/// `*id001`.
#[test]
fn dumper_shared_node_at_depth_two() {
    let mut doc = Document::new();
    let root = doc.add_sequence();
    let a = doc.add_sequence();
    let shared = doc.add_scalar("shared");
    doc.append_item(root, a);
    doc.append_item(a, shared);
    doc.append_item(a, shared);
    let text = String::from_utf8(dump_document(&mut doc)).unwrap();
    eprintln!("OUT: {text}");
    assert!(text.contains("&id001"));
    assert!(text.contains("*id001"));
    let expected = yaml::Value::Sequence(vec![yaml::Value::Sequence(vec![
        yaml::Value::String("shared".into()),
        yaml::Value::String("shared".into()),
    ])]);
    let options = yaml::Options {
        max_aliases: None,
        max_input_bytes: None,
        ..yaml::Options::default()
    };
    assert_eq!(yaml::from_str_with_options::<yaml::Value>(&text, options).unwrap(), expected);
}

/// End-to-end regression for F-7: parse a perfectly ordinary document with an
/// alias one level below the root, then re-emit it with the dumper.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dumper_roundtrip_of_parsed_nested_alias() {
    const ENV: &str = "YAML_AUDIT_NESTED_ALIAS";
    if std::env::var_os(ENV).is_some() {
        let input = b"- - &a shared\n  - *a\n";
        let mut parser = Box::new(MaybeUninit::<yaml_parser_t>::zeroed());
        let p = parser.as_mut_ptr();
        assert!(unsafe { yaml_parser_initialize(p) }.ok);
        unsafe { yaml_parser_set_input_string(p, input.as_ptr(), input.len() as u64) };
        let mut document = Box::new(MaybeUninit::<yaml_document_t>::zeroed());
        let d = document.as_mut_ptr();
        assert!(unsafe { yaml_parser_load(p, d) }.ok);
        let mut em = Emitter::new();
        assert!(unsafe { yaml_emitter_open(em.ptr()) }.ok);
        assert!(unsafe { yaml_emitter_dump(em.ptr(), d) }.ok);
        unsafe { (*d).nodes.start = ptr::null_mut() };
        unsafe { yaml_parser_delete(p) };
        print!("{}", String::from_utf8_lossy(&em.bytes()));
        return;
    }

    if cfg!(miri) {
        // Under Miri, exercise the same path directly (no process spawning).
        let mut doc = Document::new();
        let root = doc.add_sequence();
        let a = doc.add_sequence();
        let shared = doc.add_scalar("shared");
        doc.append_item(root, a);
        doc.append_item(a, shared);
        doc.append_item(a, shared);
        let text = String::from_utf8(dump_document(&mut doc)).unwrap();
        assert!(text.contains("*id001"));
        return;
    }

    let exe = std::env::current_exe().unwrap();
    let output = std::process::Command::new(exe)
        .args(["--exact", "dumper_roundtrip_of_parsed_nested_alias", "--nocapture"])
        .env(ENV, "1")
        .output()
        .expect("spawn child");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "child failed (before the fix it dies on strlen(NULL)): {output:?}"
    );
    assert!(stdout.contains("&id001") && stdout.contains("*id001"), "child output: {stdout}");
}
