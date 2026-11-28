// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compile-pass / compile-fail harness for properties that cannot be checked
//! inside a single crate (coherence conflicts, `Pin` method availability,
//! macro input rejection).
//!
//! Each case writes a tiny scratch crate whose `pin-project-lite` dependency is
//! a path pointing at the crate under test (`CARGO_MANIFEST_DIR`, or the value
//! of `PIN_PROJECT_LITE_DIR`), then runs `cargo check` in it with an isolated
//! target dir under `/tmp` and `--offline`. `rustc` diagnostics are matched
//! against a required substring. No `trybuild`/`macrotest` dependency is used.
//!
//! Run:
//!   CARGO_BUILD_JOBS=3 cargo test -p pin-project-lite --test audit_compile

#![cfg(all(not(miri), not(target_arch = "wasm32")))] // Miri and wasm cannot spawn processes.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU32, Ordering},
};

/// The exact diagnostic produced by the `#[project_replace]` + `PinnedDrop`
/// guard in `src/lib.rs`.
const PROJECT_REPLACE_PINNED_DROP_ERROR: &str = "`project_replace` is incompatible with `PinnedDrop`: the old value's PinnedDrop::drop would not be called";

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn dep_dir() -> PathBuf {
    PathBuf::from(
        env::var("PIN_PROJECT_LITE_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string()),
    )
}

/// Scratch space and target dir are kept under `/tmp` even when `TMPDIR`
/// points somewhere else, so runs never touch the workspace target dir.
fn audit_tmp() -> PathBuf {
    let tmp = Path::new("/tmp");
    if tmp.is_dir() { tmp.to_path_buf() } else { env::temp_dir() }
}

struct Case {
    name: &'static str,
    /// `true` = must compile, `false` = must be rejected.
    pass: bool,
    /// Substring that must appear in stderr (only when `pass == false`).
    expect: &'static str,
    /// Crate kind: "bin" or "lib".
    kind: &'static str,
    source: &'static str,
}

fn run_case(case: &Case) -> Result<(), String> {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let root = audit_tmp().join(format!("pin-project-lite-audit-{}-{}", std::process::id(), n));
    let src = root.join("src");
    fs::create_dir_all(&src).map_err(|e| format!("mkdir: {e}"))?;
    let manifest = format!(
        r#"[package]
name = "snippet_{n}"
version = "0.0.0"
edition = "2021"

[workspace]

[dependencies]
pin-project-lite = {{ path = {dep:?} }}
"#,
        dep = dep_dir().to_str().unwrap(),
    );
    fs::write(root.join("Cargo.toml"), manifest).map_err(|e| format!("write toml: {e}"))?;
    let file = if case.kind == "lib" { src.join("lib.rs") } else { src.join("main.rs") };
    fs::write(&file, case.source).map_err(|e| format!("write src: {e}"))?;

    let target = audit_tmp().join("pin-project-lite-audit-target");
    let out = Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .arg("check")
        .arg("--offline")
        .arg("--quiet")
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", &target)
        .env("CARGO_BUILD_JOBS", "2")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .output()
        .map_err(|e| format!("spawn cargo: {e}"))?;
    let stderr = String::from_utf8_lossy(&out.stderr);
    let ok = out.status.success();

    let verdict = if ok { "compiled" } else { "rejected" };
    let res = if ok == case.pass {
        if !case.pass && !stderr.contains(case.expect) {
            Err(format!(
                "case `{}`: rejected but stderr lacks {:?}\n--- stderr ---\n{stderr}",
                case.name, case.expect
            ))
        } else {
            Ok(())
        }
    } else {
        Err(format!(
            "case `{}`: expected {}, got {verdict}\n--- stderr ---\n{stderr}",
            case.name,
            if case.pass { "success" } else { "rejection" },
        ))
    };
    let _ = fs::remove_dir_all(&root);
    println!("case {:32} -> {verdict}", case.name);
    res
}

// -- snippets -------------------------------------------------------------

/// Fixed in 0.2.15 (https://github.com/taiki-e/pin-project-lite/pull/84): the
/// bogus overlapping `Unpin` impl must now be rejected with E0119. This is the
/// F1 reproducer; it is not a runtime test.
const NEGATIVE_IMPLS_OVERLAP: &str = r#"
use core::marker::PhantomPinned;
use pin_project_lite::pin_project;

pin_project! {
    struct Foo<Pinned, Unpinned> {
        #[pin]
        pinned: Pinned,
        unpinned: Unpinned,
    }
}

struct MyPhantomPinned(PhantomPinned);
impl Unpin for MyPhantomPinned where for<'cursed> str: Sized {}
impl Unpin for Foo<MyPhantomPinned, ()> {}

fn is_unpin<T: Unpin>() {}

fn main() {
    is_unpin::<Foo<MyPhantomPinned, ()>>();
}
"#;

/// F2 reproducer: `#[project_replace]` + `PinnedDrop` must be rejected by the
/// guard in `__pin_project_expand!`, not silently accepted.
const PROJECT_REPLACE_PINNED_DROP_REJECTED: &str = r#"
use pin_project_lite::pin_project;

pin_project! {
    #[project_replace = SReplace]
    struct S {
        #[pin]
        f: u8,
        u: u16,
    }

    impl PinnedDrop for S {
        fn drop(_this: core::pin::Pin<&mut Self>) {}
    }
}

fn main() {}
"#;

/// Positive half: `project_replace` without `PinnedDrop` must keep compiling.
const PROJECT_REPLACE_ONLY_OK: &str = r#"
use core::pin::Pin;
use pin_project_lite::pin_project;

pin_project! {
    #[project_replace = SReplace]
    struct S {
        #[pin]
        pinned: u8,
        unpinned: u16,
    }
}

fn main() {
    let mut s = S { pinned: 1, unpinned: 2 };
    let mut s = Pin::new(&mut s);
    let old = s.as_mut().project_replace(S { pinned: 3, unpinned: 4 });
    assert_eq!(old.unpinned, 2);
}
"#;

/// Positive half: `PinnedDrop` without `project_replace` must keep compiling.
const PINNED_DROP_ONLY_OK: &str = r#"
use core::pin::Pin;
use pin_project_lite::pin_project;

pin_project! {
    struct S {
        #[pin]
        pinned: u8,
    }

    impl PinnedDrop for S {
        fn drop(_this: Pin<&mut Self>) {}
    }
}

fn main() {
    drop(S { pinned: 1 });
}
"#;

/// `Pin::get_mut`/`into_inner` must be unavailable for a `!Unpin` container.
const PIN_GET_MUT_REJECTED: &str = r#"
use core::marker::PhantomPinned;
use core::pin::Pin;
use pin_project_lite::pin_project;

pin_project! {
    struct P<T> {
        #[pin]
        pinned: T,
    }
}

fn main() {
    let mut v = P { pinned: PhantomPinned };
    let p: Pin<&mut P<PhantomPinned>> = unsafe { Pin::new_unchecked(&mut v) };
    let _ = Pin::into_inner(p); // must be rejected: P<PhantomPinned>: !Unpin
}
"#;

/// Pinned-field structural sharing must be impossible: no `DerefMut` etc.
const PIN_DEREF_MUT_REJECTED: &str = r#"
use core::marker::PhantomPinned;
use core::pin::Pin;
use pin_project_lite::pin_project;

pin_project! {
    struct P<T> {
        #[pin]
        pinned: T,
    }
}

fn main() {
    let mut v = P { pinned: PhantomPinned };
    let mut p: Pin<&mut P<PhantomPinned>> = unsafe { Pin::new_unchecked(&mut v) };
    let x: &mut PhantomPinned = &mut p.pinned; // must be rejected
    let _ = x;
}
"#;

const TUPLE_STRUCT_REJECTED: &str = r#"
use pin_project_lite::pin_project;
pin_project! {
    struct T(#[pin] u8, u8);
}
fn main() {}
"#;

const TUPLE_VARIANT_REJECTED: &str = r#"
use pin_project_lite::pin_project;
pin_project! {
    #[project = EProj]
    enum E {
        V(#[pin] u8),
    }
}
fn main() {}
"#;

const PLAIN_DROP_REJECTED: &str = r#"
use pin_project_lite::pin_project;
pin_project! {
    struct S {
        #[pin]
        f: u8,
    }
}
impl Drop for S {
    fn drop(&mut self) {}
}
fn main() {}
"#;

const REPR_PACKED_REJECTED: &str = r#"
use pin_project_lite::pin_project;
pin_project! {
    #[repr(packed)]
    struct S {
        #[pin]
        f: u16,
    }
}
fn main() {}
"#;

/// The expansion must be usable from a `#![no_std]` crate.
const NO_STD_OK: &str = r#"
#![no_std]
use core::pin::Pin;
use pin_project_lite::pin_project;

pin_project! {
    #[project = PProj]
    #[project_ref = PProjRef]
    #[project_replace = PProjReplace]
    pub struct P<T, U> {
        #[pin]
        pub pinned: T,
        pub unpinned: U,
    }
}

pub fn use_it(mut p: Pin<&mut P<u8, u16>>) -> u16 {
    let x = p.as_mut().project();
    let _: Pin<&mut u8> = x.pinned;
    *x.unpinned
}
"#;

/// The expansion must not be considered "unsafe code" by `forbid(unsafe_code)`.
const FORBID_UNSAFE_OK: &str = r#"
#![forbid(unsafe_code)]
use core::pin::Pin;
use pin_project_lite::pin_project;

pin_project! {
    pub struct S {
        #[pin]
        pub pinned: u8,
        pub unpinned: u8,
    }

    impl PinnedDrop for S {
        fn drop(this: Pin<&mut Self>) {
            let _ = this.project().unpinned;
        }
    }
}

fn main() {}
"#;

/// `deny(unsafe_op_in_unsafe_fn)` must not be violated by the expansion.
const UNSAFE_OP_IN_UNSAFE_FN_OK: &str = r#"
#![deny(unsafe_op_in_unsafe_fn)]
use pin_project_lite::pin_project;

pin_project! {
    pub struct S<T> {
        #[pin]
        pub pinned: T,
        pub unpinned: u8,
    }
}

fn main() {}
"#;

const MOVE_OUT_OF_PIN_REJECTED: &str = r#"
use pin_project_lite::pin_project;

pin_project! {
    struct S {
        #[pin]
        f: u8,
    }
}

fn main() {
    let s = Box::pin(S { f: 1 });
    let _moved: S = *s; // must be rejected: cannot move out of Pin
}
"#;

// LIMITATION: `#[cfg]` on the item *inside* `pin_project!` is not applied to
// the generated impls/impl blocks, and `#[cfg]` on enum variants is dropped
// from the generated projection type (but not from the original enum).
const CFG_ON_TYPE_LIMITATION: &str = r#"
use pin_project_lite::pin_project;

pin_project! {
    #[cfg(any())]
    struct Gone {
        #[pin]
        f: u8,
    }
}

fn main() {}
"#;

const CFG_ON_VARIANT_LIMITATION: &str = r#"
use pin_project_lite::pin_project;

pin_project! {
    #[project = EProj]
    enum E {
        A { #[pin] f: u8 },
        #[cfg(any())]
        B { f: u8 },
    }
}

fn main() {}
"#;

// LIMITATION: const generics and empty structs are not accepted by the
// declarative parser.
const CONST_GENERIC_REJECTED: &str = r#"
use pin_project_lite::pin_project;

pin_project! {
    struct S<const N: usize> {
        #[pin]
        f: [u8; N],
    }
}

fn main() {}
"#;

const EMPTY_STRUCT_REJECTED: &str = r#"
use pin_project_lite::pin_project;

pin_project! {
    struct S {}
}

fn main() {}
"#;

// LIMITATION: `project_replace` needs `Self: Sized`, so it cannot be combined
// with an unsized field.
const UNSIZED_PROJECT_REPLACE_REJECTED: &str = r#"
use pin_project_lite::pin_project;

pin_project! {
    #[project_replace = SProjReplace]
    struct S<T: ?Sized> {
        u: u8,
        #[pin]
        tail: T,
    }
}

fn main() {}
"#;

#[test]
fn compile_matrix() {
    let cases = [
        // F1: overlapping `Unpin` impl must be rejected (0.2.15+).
        Case {
            name: "negative_impls_overlap",
            pass: false,
            expect: "E0119",
            kind: "bin",
            source: NEGATIVE_IMPLS_OVERLAP,
        },
        // F2: `project_replace` + `PinnedDrop` must hit the guard.
        Case {
            name: "project_replace_pinned_drop_rejected",
            pass: false,
            expect: PROJECT_REPLACE_PINNED_DROP_ERROR,
            kind: "bin",
            source: PROJECT_REPLACE_PINNED_DROP_REJECTED,
        },
        // ... while each feature alone keeps compiling.
        Case {
            name: "project_replace_only_ok",
            pass: true,
            expect: "",
            kind: "bin",
            source: PROJECT_REPLACE_ONLY_OK,
        },
        Case {
            name: "pinned_drop_only_ok",
            pass: true,
            expect: "",
            kind: "bin",
            source: PINNED_DROP_ONLY_OK,
        },
        Case {
            name: "pin_get_mut_rejected",
            pass: false,
            expect: "E0277",
            kind: "bin",
            source: PIN_GET_MUT_REJECTED,
        },
        Case {
            name: "pin_deref_mut_rejected",
            pass: false,
            expect: "E0596",
            kind: "bin",
            source: PIN_DEREF_MUT_REJECTED,
        },
        Case {
            name: "tuple_struct_rejected",
            pass: false,
            expect: "no rules expected",
            kind: "bin",
            source: TUPLE_STRUCT_REJECTED,
        },
        Case {
            name: "tuple_variant_rejected",
            pass: false,
            expect: "no rules expected",
            kind: "bin",
            source: TUPLE_VARIANT_REJECTED,
        },
        Case {
            name: "plain_drop_rejected",
            pass: false,
            expect: "E0119",
            kind: "bin",
            source: PLAIN_DROP_REJECTED,
        },
        Case {
            name: "repr_packed_rejected",
            pass: false,
            expect: "E0793",
            kind: "bin",
            source: REPR_PACKED_REJECTED,
        },
        Case { name: "no_std_ok", pass: true, expect: "", kind: "lib", source: NO_STD_OK },
        Case {
            name: "forbid_unsafe_ok",
            pass: true,
            expect: "",
            kind: "bin",
            source: FORBID_UNSAFE_OK,
        },
        Case {
            name: "move_out_of_pin_rejected",
            pass: false,
            expect: "E0507",
            kind: "bin",
            source: MOVE_OUT_OF_PIN_REJECTED,
        },
        // Known compile-time limitations (not soundness): pinned by these
        // probes so a future fix/regression is visible.
        Case {
            name: "limit_cfg_on_type",
            pass: false,
            expect: "E0425",
            kind: "bin",
            source: CFG_ON_TYPE_LIMITATION,
        },
        Case {
            name: "limit_cfg_on_variant",
            pass: false,
            expect: "E0599",
            kind: "bin",
            source: CFG_ON_VARIANT_LIMITATION,
        },
        Case {
            name: "limit_const_generic",
            pass: false,
            expect: "no rules expected",
            kind: "bin",
            source: CONST_GENERIC_REJECTED,
        },
        Case {
            name: "limit_empty_struct",
            pass: false,
            expect: "no rules expected",
            kind: "bin",
            source: EMPTY_STRUCT_REJECTED,
        },
        Case {
            name: "limit_unsized_replace",
            pass: false,
            expect: "E0277",
            kind: "bin",
            source: UNSIZED_PROJECT_REPLACE_REJECTED,
        },
        Case {
            name: "unsafe_op_in_unsafe_fn_ok",
            pass: true,
            expect: "",
            kind: "bin",
            source: UNSAFE_OP_IN_UNSAFE_FN_OK,
        },
    ];

    let mut failures = Vec::new();
    for case in &cases {
        if let Err(e) = run_case(case) {
            failures.push(e);
        }
    }
    if !failures.is_empty() {
        panic!("compile matrix failures:\n\n{}", failures.join("\n\n"));
    }
}

#[test]
fn dependency_dir_is_the_crate_under_test() {
    let dir = dep_dir();
    assert!(
        Path::new(&dir).join("src/lib.rs").is_file(),
        "not a pin-project-lite source dir: {}",
        dir.display()
    );
    let manifest = fs::read_to_string(Path::new(&dir).join("Cargo.toml")).unwrap();
    assert!(manifest.contains("version = \"0.2.18\""), "unexpected crate version");
}
