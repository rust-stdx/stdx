//! `ensure!` differential and boundary tests for `anyerr`.
//!
//! The macro has two rendering paths:
//!  * comparison operands are formatted into two fixed 40-byte `Buf`s
//!    (`__fancy_ensure!`), falling back to the plain message if a buffer would
//!    overflow or the `Debug` output contains a space/newline;
//!  * everything else goes through `format!` (`__fallback_ensure!`).
//!
//! Every case is cross-checked against `format!`/`stringify!`.

// The parser tests deliberately use unusual-but-valid condition expressions,
// some of which clippy would normally suggest simplifying.
#![allow(
    clippy::const_is_empty,
    clippy::match_like_matches_macro,
    clippy::needless_bool,
    clippy::never_loop,
    clippy::redundant_closure_call,
    clippy::unnecessary_min_or_max
)]

use std::fmt::{self, Debug, Display};

use anyerr::ensure;

fn msg_of(f: impl FnOnce() -> anyerr::Result<()>) -> String {
    f().unwrap_err().to_string()
}

macro_rules! check_format {
    ($($arg:tt)*) => {{
        let expected = format!($($arg)*);
        let got = msg_of(|| {
            ensure!(false, $($arg)*);
            Ok(())
        });
        assert_eq!(got, expected, "ensure! vs format! for {:?}", stringify!($($arg)*));
    }};
}

#[test]
fn differential_against_format() {
    check_format!("literal message");
    check_format!("interpolated {}", 42);
    check_format!("{} and {}", "a", "b");
    check_format!("{:?}", vec![1, 2, 3]);
    check_format!("{:>8}", 42);
    check_format!("{:*>8}", 42);
    check_format!("{:*<8}", 42);
    check_format!("{:*^8}", 42);
    check_format!("{:.3}", 1.23456);
    check_format!("{:.2}", "abcdef");
    check_format!("{:08}", 42);
    check_format!("{:#x}", 255);
    check_format!("{} {} {} {} {} {} {} {} {} {}", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    check_format!("{{}}");
    check_format!("{{ {}{{}} }}", 7);
    check_format!("unicode: é € 🦀");
    check_format!("{:width$}", 1, width = 40);
    check_format!("no args but a long literal that is definitely more than forty bytes long");
}

#[test]
fn huge_width_falls_back_without_panic() {
    let got = msg_of(|| {
        ensure!(false, "{:width$}", 7, width = 65_535usize);
        Ok(())
    });
    let expected = format!("{:width$}", 7, width = 65_535usize);
    assert_eq!(got, expected);
    assert_eq!(got.len(), 65535);
}

#[test]
fn string_width_just_under_and_over_buffer() {
    // Padding produces spaces, so the fancy path must reject and fall back.
    for width in [39usize, 40, 41, 100] {
        let got = msg_of(|| {
            ensure!(false, "{:width$}", 1);
            Ok(())
        });
        assert_eq!(got, format!("{:width$}", 1));
    }
}

// ---------------------------------------------------------------------------
// Debug helpers used to probe the 40-byte comparison buffers
// ---------------------------------------------------------------------------

struct Dbg<'a>(&'a str);

impl Debug for Dbg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl PartialEq for Dbg<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

struct SplitDbg;

impl Debug for SplitDbg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("abc")?;
        f.write_str(" def")
    }
}

impl PartialEq for SplitDbg {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

struct PanicDbg;

impl Debug for PanicDbg {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("debug panic inside ensure");
    }
}

impl PartialEq for PanicDbg {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

const PLAIN: &str = "Condition failed: `lhs == rhs`";

/// `ensure!(lhs == rhs)` with the two operands wrapped in [`Dbg`].
fn fancy(a: &str, b: &str) -> String {
    msg_of(|| {
        let lhs = Dbg(a);
        let rhs = Dbg(b);
        ensure!(lhs == rhs);
        Ok(())
    })
}

fn expected_fancy(a: &str, b: &str) -> String {
    format!("Condition failed: `lhs == rhs` ({a} vs {b})")
}

#[test]
fn fancy_comparison_fits() {
    for (a, b) in [("1", "2"), ("abc", "def"), ("a", "b")] {
        assert_eq!(fancy(a, b), expected_fancy(a, b));
    }
}

#[test]
fn fancy_comparison_exact_40_byte_boundaries() {
    let a = "a".repeat(40);
    let b = "b".repeat(40);
    assert_eq!(fancy(&a, &b), expected_fancy(&a, &b));

    // One byte over in either operand: the fancy renderer must fall back to
    // the bare condition message (no truncation).
    let a41 = "a".repeat(41);
    assert_eq!(fancy(&a41, "b"), PLAIN);
    let b41 = "b".repeat(41);
    assert_eq!(fancy("a", &b41), PLAIN);
}

#[test]
fn fancy_comparison_unicode_boundaries() {
    // "é" is 2 bytes: 20 of them are exactly 40 bytes.
    let e40 = "é".repeat(20);
    assert_eq!(e40.len(), 40);
    assert_eq!(fancy(&e40, ""), expected_fancy(&e40, ""));
    let e42 = "é".repeat(21); // 42 bytes
    assert_eq!(fancy(&e42, ""), PLAIN);

    // "€" is 3 bytes: 13 * 3 = 39 bytes, 14 * 3 = 42 bytes.
    let euro39 = "€".repeat(13);
    assert_eq!(fancy(&euro39, "x"), expected_fancy(&euro39, "x"));
    let euro42 = "€".repeat(14);
    assert_eq!(fancy(&euro42, ""), PLAIN);
}

#[test]
fn fancy_comparison_rejects_whitespace_anywhere() {
    assert_eq!(fancy("a b", "c"), PLAIN);
    assert_eq!(fancy("a\nb", "c"), PLAIN);
    // Tab is *not* rejected by `Buf` (only b' ' and b'\n' are), matching
    // upstream anyhow: the tab ends up in the fancy message verbatim.
    assert_eq!(fancy("c", "a\tb"), "Condition failed: `lhs == rhs` (c vs a\tb)");

    // Rejection must discard the partially written buffer: "abc" was already
    // copied before the space was seen.
    let got = msg_of(|| {
        let lhs = SplitDbg;
        let rhs = SplitDbg;
        ensure!(lhs == rhs);
        Ok(())
    });
    assert_eq!(got, PLAIN);
}

#[test]
fn fancy_comparison_all_operators() {
    fn run(op: char) -> (String, i32, i32) {
        let (lhs, rhs) = match op {
            '<' => (2, 1),
            'L' => (2, 1), // <=
            '>' => (1, 2),
            'G' => (1, 2), // >=
            '=' => (1, 2), // ==
            '!' => (1, 1), // !=
            _ => unreachable!(),
        };
        let got = msg_of(move || {
            match op {
                '<' => ensure!(lhs < rhs),
                'L' => ensure!(lhs <= rhs),
                '>' => ensure!(lhs > rhs),
                'G' => ensure!(lhs >= rhs),
                '=' => ensure!(lhs == rhs),
                '!' => ensure!(lhs != rhs),
                _ => unreachable!(),
            }
            Ok(())
        });
        (got, lhs, rhs)
    }
    for (op, tokens) in [
        ('<', "<"),
        ('L', "<="),
        ('>', ">"),
        ('G', ">="),
        ('=', "=="),
        ('!', "!="),
    ] {
        let (got, lhs, rhs) = run(op);
        assert_eq!(got, format!("Condition failed: `lhs {tokens} rhs` ({lhs} vs {rhs})"));
    }
}

/// Only one side implements `Debug`: tagged dispatch must pick
/// `NotBothDebug` and produce the plain message.
#[test]
fn fancy_comparison_requires_both_debug() {
    #[derive(PartialEq)]
    struct OnlyDisplay(u8);
    impl Display for OnlyDisplay {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(PartialEq)]
    struct Plain(u8);

    let got = msg_of(|| {
        let lhs = OnlyDisplay(1);
        let rhs = OnlyDisplay(2);
        ensure!(lhs == rhs);
        Ok(())
    });
    assert_eq!(got, PLAIN);

    let got = msg_of(|| {
        let lhs = Plain(1);
        let rhs = Plain(2);
        ensure!(lhs == rhs);
        Ok(())
    });
    assert_eq!(got, PLAIN);
}

#[cfg(panic = "unwind")]
#[test]
fn fancy_comparison_panic_in_debug_is_not_swallowed() {
    let res = std::panic::catch_unwind(|| -> anyerr::Result<()> {
        let lhs = PanicDbg;
        let rhs = PanicDbg;
        ensure!(lhs == rhs);
        Ok(())
    });
    assert!(res.is_err(), "panic in Debug must propagate");
}

// ---------------------------------------------------------------------------
// Macro parser forms (control flow, closures, paths, literals, patterns)
// ---------------------------------------------------------------------------

#[test]
fn parser_accepts_unusual_conditions() {
    // If / else.
    let got = msg_of(|| {
        let v = 3;
        ensure!(if v > 5 { true } else { false });
        Ok(())
    });
    assert_eq!(
        got,
        format!("Condition failed: `{}`", stringify!(if v > 5 { true } else { false }))
    );

    // Match.
    let got = msg_of(|| {
        let v = 3;
        ensure!(match v {
            0 => true,
            _ => false,
        });
        Ok(())
    });
    assert_eq!(
        got,
        format!(
            "Condition failed: `{}`",
            stringify!(match v {
                0 => true,
                _ => false,
            })
        )
    );

    // Closure call.
    let got = msg_of(|| {
        ensure!((|| false)());
        Ok(())
    });
    assert_eq!(got, format!("Condition failed: `{}`", stringify!((|| false)())));

    // Method call chain (fancy comparison path).
    let got = msg_of(|| {
        let lhs = "";
        let rhs = false;
        ensure!(lhs.is_empty() == rhs);
        Ok(())
    });
    assert_eq!(got, "Condition failed: `lhs.is_empty() == rhs` (true vs false)");

    // Qualified paths and literals (fancy comparison path).
    let got = msg_of(|| {
        let lhs = std::primitive::u8::MAX;
        let rhs = 0u8;
        ensure!(lhs == rhs);
        Ok(())
    });
    assert_eq!(got, "Condition failed: `lhs == rhs` (255 vs 0)");

    // Unsafe block condition.
    let v = 0u8;
    let got = msg_of(|| {
        ensure!(unsafe { std::ptr::read(&v) } == 1);
        Ok(())
    });
    assert_eq!(
        got,
        format!(
            "Condition failed: `{}` (0 vs 1)",
            stringify!(unsafe { std::ptr::read(&v) } == 1)
        )
    );

    // for / while blocks.
    let got = msg_of(|| {
        ensure!({
            let mut n = 0;
            for i in 0..3 {
                n += i;
            }
            n == 4
        });
        Ok(())
    });
    assert_eq!(
        got,
        format!(
            "Condition failed: `{}`",
            stringify!({
                let mut n = 0;
                for i in 0..3 {
                    n += i;
                }
                n == 4
            })
        )
    );

    // Labeled loop.
    let got = msg_of(|| {
        ensure!('a: loop {
            break 'a false;
        });
        Ok(())
    });
    assert_eq!(
        got,
        format!(
            "Condition failed: `{}`",
            stringify!('a: loop {
                break 'a false;
            })
        )
    );

    // if let.
    let got = msg_of(|| {
        let opt = Some(1);
        ensure!(if let Some(x) = opt { x == 2 } else { false });
        Ok(())
    });
    assert_eq!(
        got,
        format!(
            "Condition failed: `{}`",
            stringify!(if let Some(x) = opt { x == 2 } else { false })
        )
    );

    // Casts and references.
    let got = msg_of(|| {
        let n: u8 = 1;
        ensure!(&(n as u16) == &2u16);
        Ok(())
    });
    assert_eq!(
        got,
        format!("Condition failed: `{}` (1 vs 2)", stringify!(&(n as u16) == &2u16))
    );

    // Array indexing and turbofish.
    let got = msg_of(|| {
        let v = [1u8, 2];
        ensure!(v[0].max(0) == 3);
        Ok(())
    });
    assert_eq!(got, format!("Condition failed: `{}` (1 vs 3)", stringify!(v[0].max(0) == 3)));
}

#[test]
fn parser_accepts_format_message_arm() {
    let got = msg_of(|| {
        let x = 5;
        ensure!(x > 10, "expected {} > 10, got {}", "x", x);
        Ok(())
    });
    assert_eq!(got, "expected x > 10, got 5");

    // Escaped braces through the fallback path.
    let got = msg_of(|| {
        ensure!(false, "{{literal braces}}");
        Ok(())
    });
    assert_eq!(got, "{literal braces}");
}

// ---------------------------------------------------------------------------
// Panic safety of the formatting path
// ---------------------------------------------------------------------------

struct CountedDbg<'a>(&'a std::sync::atomic::AtomicUsize);

impl Debug for CountedDbg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ok")
    }
}

impl PartialEq for CountedDbg<'_> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Drop for CountedDbg<'_> {
    fn drop(&mut self) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(panic = "unwind")]
#[test]
fn formatting_path_drops_operands_once() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let drops = AtomicUsize::new(0);
    {
        let lhs = CountedDbg(&drops);
        let rhs = CountedDbg(&drops);
        let _ = msg_of(|| {
            ensure!(lhs == rhs);
            Ok(())
        });
        assert_eq!(drops.load(Ordering::SeqCst), 0);
    }
    assert_eq!(drops.load(Ordering::SeqCst), 2);

    struct PanicCounted<'a>(&'a AtomicUsize);
    impl Debug for PanicCounted<'_> {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("debug panic");
        }
    }
    impl PartialEq for PanicCounted<'_> {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }
    impl Drop for PanicCounted<'_> {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let drops = AtomicUsize::new(0);
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> anyerr::Result<()> {
        let lhs = PanicCounted(&drops);
        let rhs = PanicCounted(&drops);
        ensure!(lhs == rhs);
        Ok(())
    }));
    assert!(res.is_err());
    assert_eq!(drops.load(Ordering::SeqCst), 2);
}
