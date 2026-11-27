//! Audit tests for `dotenv`: parser matrix plus `from_path` behaviour.
//!
//! The parser is exercised through the public `parse`/`from_path` API. The
//! crate-level unit tests already cover most individual cases; this file keeps
//! a consolidated matrix (comments, escapes, BOM, duplicate keys, key
//! validation, non-UTF-8) and adds the file-level cases that are only reachable
//! through `from_path` (atomic failure, precedence, invalid UTF-8 contents,
//! non-UTF-8 process environment).
//!
//! `load`/`from_path` are *safe* functions that call `unsafe { env::set_var }`
//! internally. Their thread-safety requirements are documented on the crate;
//! this file deliberately does not demonstrate calling them while another
//! thread is running.

use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

use dotenv::{ParseErrorKind, parse};

static SEQ: AtomicUsize = AtomicUsize::new(0);

/// Our own tests mutate the process environment; serialize them so they do not
/// race with each other.
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn temp_path(name: &str) -> PathBuf {
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    let p = std::env::temp_dir().join(format!(
        "stdx-dotenv-audit-{}-{}-{n}-{name}",
        std::process::id(),
        std::thread::current().name().unwrap_or("t")
    ));
    let _ = fs::remove_file(&p);
    p
}

fn vars(input: &str) -> Result<BTreeMap<String, String>, ParseErrorKind> {
    parse(input)
        .map(|m| m.into_iter().map(|(k, v)| (k.to_owned(), v.into_owned())).collect())
        .map_err(|e| e.kind)
}

fn one(input: &str) -> (String, String) {
    let m = vars(input).unwrap();
    assert_eq!(m.len(), 1, "expected one pair from {input:?}");
    m.into_iter().next().unwrap()
}

#[test]
fn parse_matrix() {
    // Basic / whitespace / equals.
    assert_eq!(one("K=v"), ("K".into(), "v".into()));
    assert_eq!(one("  K  =  v  "), ("K".into(), "v".into()));
    assert_eq!(one("K=a=b=c"), ("K".into(), "a=b=c".into()));
    assert_eq!(one("K="), ("K".into(), "".into()));
    assert_eq!(one("K=   "), ("K".into(), "".into()));
    assert_eq!(one("K=hello world"), ("K".into(), "hello world".into()));

    // Comments: only `#` preceded by whitespace starts an inline comment.
    assert_eq!(one("K=v # comment"), ("K".into(), "v".into()));
    assert_eq!(one("K=v\t# comment"), ("K".into(), "v".into()));
    assert_eq!(one("K=v#not-comment"), ("K".into(), "v#not-comment".into()));
    assert_eq!(one("K=#leading-hash"), ("K".into(), "#leading-hash".into()));
    assert_eq!(one("K=a # b # c"), ("K".into(), "a".into()));
    assert_eq!(one("# full line\nK=v"), ("K".into(), "v".into()));

    // Quotes.
    assert_eq!(one("K=\"hello world\""), ("K".into(), "hello world".into()));
    assert_eq!(one("K='hello world'"), ("K".into(), "hello world".into()));
    assert_eq!(one("K=\"a#b\""), ("K".into(), "a#b".into()));
    assert_eq!(one("K='a#b'"), ("K".into(), "a#b".into()));
    assert_eq!(one("K=\"\""), ("K".into(), "".into()));
    assert_eq!(one("K=''"), ("K".into(), "".into()));
    assert_eq!(one("K='\"nested\"'"), ("K".into(), "\"nested\"".into()));
    assert_eq!(one("K=\"it's\""), ("K".into(), "it's".into()));
    assert_eq!(one("K=\"a\" # trailing comment"), ("K".into(), "a".into()));
    assert_eq!(one("K=\"a\"#not-space"), ("K".into(), "a".into()));
    // A '#' in an unquoted value with no preceding whitespace is literal, and
    // quoted values keep backslashes literally in single quotes.
    assert_eq!(one("K='C:\\path\\n'"), ("K".into(), "C:\\path\\n".into()));

    // Double-quoted escapes.
    assert_eq!(one(r#"K="a\\b""#), ("K".into(), "a\\b".into()));
    assert_eq!(one(r#"K="a\"b""#), ("K".into(), "a\"b".into()));
    assert_eq!(one(r#"K="a\'b""#), ("K".into(), "a'b".into()));
    assert_eq!(one(r#"K="a\$b""#), ("K".into(), "a$b".into()));
    assert_eq!(one(r#"K="a\nb\tc\rd""#), ("K".into(), "a\nb\tc\rd".into()));
    // Unquoted backslashes are literal.
    assert_eq!(one(r"K=C:\path\n"), ("K".into(), "C:\\path\\n".into()));

    // Keys.
    for key in ["A", "a1", "_K", ".K", "-K", "A1.b-C_2", "...", "---"] {
        assert_eq!(one(&format!("{key}=v")), (key.into(), "v".into()));
    }

    // Duplicate keys: first wins.
    let m = vars("A=1\nB=2\nA=3\nB=4").unwrap();
    assert_eq!(m["A"], "1");
    assert_eq!(m["B"], "2");

    // Encoding/BOM/line endings.
    let m = vars("\u{FEFF}A=1\r\nB=2\nC=3").unwrap();
    assert_eq!(m.len(), 3);
    assert_eq!(m["A"], "1");
    assert_eq!(m["B"], "2");
    let m = vars("A=1\rB=2").unwrap();
    assert_eq!(m.len(), 1, "lone CR is not a line separator");
    assert_eq!(m["A"], "1\rB=2");

    // UTF-8 values (keys are restricted to ASCII).
    assert_eq!(one("K=é漢💖"), ("K".into(), "é漢💖".into()));

    // Empty input / only comments.
    assert!(vars("").unwrap().is_empty());
    assert!(vars("\n\n# x\n   \n").unwrap().is_empty());

    // Errors.
    assert_eq!(vars("NOVALUE").unwrap_err(), ParseErrorKind::MissingEquals);
    assert_eq!(vars("=v").unwrap_err(), ParseErrorKind::EmptyKey);
    assert_eq!(vars("K%v=1").unwrap_err(), ParseErrorKind::InvalidKey);
    assert_eq!(vars("K V=1").unwrap_err(), ParseErrorKind::InvalidKey);
    assert_eq!(vars("K=\"unterminated").unwrap_err(), ParseErrorKind::UnmatchedQuote);
    assert_eq!(vars("K='unterminated").unwrap_err(), ParseErrorKind::UnmatchedQuote);
    assert_eq!(vars("K=\"a\" junk").unwrap_err(), ParseErrorKind::TrailingContent);
    assert_eq!(vars("K='a' junk").unwrap_err(), ParseErrorKind::TrailingContent);
    assert_eq!(vars(r#"K="\q""#).unwrap_err(), ParseErrorKind::InvalidEscape);
    assert_eq!(vars("K=a\0b").unwrap_err(), ParseErrorKind::InvalidValue);
    assert_eq!(vars("K=\"a\0b\"").unwrap_err(), ParseErrorKind::InvalidValue);
    assert_eq!(vars("K='a\0b'").unwrap_err(), ParseErrorKind::InvalidValue);

    // Line numbers are 1-based and account for skipped comments/blank lines.
    let err = parse("\n# x\nOK=1\nBAD").unwrap_err();
    assert_eq!((err.line, err.kind), (4, ParseErrorKind::MissingEquals));
}

#[test]
#[cfg_attr(miri, ignore = "writes temporary files, which Miri isolation forbids")]
fn parsing_is_atomic_and_does_not_touch_env() {
    let _lock = ENV_LOCK.lock().unwrap();
    let before = std::env::var_os("DOTENV_AUDIT_ATOMIC");
    let p = temp_path("atomic.env");
    fs::write(&p, "DOTENV_AUDIT_ATOMIC=set-it\nBROKEN\n").unwrap();
    let err = dotenv::from_path(&p).unwrap_err();
    assert!(matches!(err, dotenv::Error::Parse(_)));
    assert_eq!(std::env::var_os("DOTENV_AUDIT_ATOMIC"), before);
    let _ = fs::remove_file(&p);
}

#[test]
#[cfg_attr(miri, ignore = "mutates the process environment and writes files")]
fn from_path_precedence_and_values() {
    let _lock = ENV_LOCK.lock().unwrap();
    // These mutations are test-only; the audit finding is about the library's
    // *safe* API, not about this test helper.
    unsafe { std::env::set_var("DOTENV_AUDIT_EXISTING", "original") };
    unsafe { std::env::remove_var("DOTENV_AUDIT_NEW") };
    unsafe { std::env::remove_var("DOTENV_AUDIT_DUP") };

    let p = temp_path("values.env");
    fs::write(
        &p,
        "DOTENV_AUDIT_EXISTING=from-file\nDOTENV_AUDIT_NEW=hello\nDOTENV_AUDIT_DUP=first\nDOTENV_AUDIT_DUP=second\n",
    )
    .unwrap();
    dotenv::from_path(&p).unwrap();
    assert_eq!(std::env::var("DOTENV_AUDIT_EXISTING").unwrap(), "original");
    assert_eq!(std::env::var("DOTENV_AUDIT_NEW").unwrap(), "hello");
    assert_eq!(std::env::var("DOTENV_AUDIT_DUP").unwrap(), "first");

    // NUL values are rejected before any variable is set.
    unsafe { std::env::remove_var("DOTENV_AUDIT_NUL") };
    fs::write(&p, "DOTENV_AUDIT_NUL=a\0b\n").unwrap();
    assert!(dotenv::from_path(&p).is_err());
    assert!(std::env::var_os("DOTENV_AUDIT_NUL").is_none());

    // Invalid UTF-8 file contents are an I/O error (read_to_string).
    fs::write(&p, b"K=\xff\n").unwrap();
    match dotenv::from_path(&p) {
        Err(dotenv::Error::Io(e)) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData),
        other => panic!("expected InvalidData, got {other:?}"),
    }

    for k in ["DOTENV_AUDIT_EXISTING", "DOTENV_AUDIT_NEW", "DOTENV_AUDIT_DUP"] {
        unsafe { std::env::remove_var(k) };
    }
    let _ = fs::remove_file(&p);
}

#[test]
#[cfg(unix)]
#[cfg_attr(miri, ignore = "mutates the process environment and writes files")]
fn non_utf8_existing_env_does_not_break_loading() {
    let _lock = ENV_LOCK.lock().unwrap();
    // `vars_os` is used for the snapshot so a non-UTF-8 key/value elsewhere in
    // the environment does not panic.
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    let key = OsString::from_vec(b"DOTENV_AUDIT_NONUTF8_\xff".to_vec());
    let val = OsString::from_vec(b"\xfe".to_vec());
    unsafe { std::env::set_var(&key, &val) };
    let p = temp_path("nonutf8env.env");
    fs::write(&p, "DOTENV_AUDIT_OK=yes\n").unwrap();
    dotenv::from_path(&p).unwrap();
    assert_eq!(std::env::var("DOTENV_AUDIT_OK").unwrap(), "yes");
    unsafe { std::env::remove_var(&key) };
    unsafe { std::env::remove_var("DOTENV_AUDIT_OK") };
    let _ = fs::remove_file(&p);
}
