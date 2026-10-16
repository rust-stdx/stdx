//! The committed generated code must match the vendored specification.
//!
//! `stripe/api` and the module block in `stripe.rs` are produced by
//! `stripe_codegen` from `spec/openapi.spec3.sdk.json`. This test reruns the
//! generator in memory and compares the result with what is committed, so a
//! hand-edit (or a specification bump without regenerating) fails `cargo test`
//! instead of shipping a client that no longer matches its own generator.
//!
//! Emitted files are formatted with the workspace's `rustfmt` configuration,
//! exactly as `make stripe_codegen` does, and the comparison then ignores
//! layout: only whitespace and trailing commas may differ. Everything inside a
//! string literal (documentation text above all) is compared verbatim.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use stripe_codegen::{ROOT_FILE, emit, schema::Builder};

/// Reduces Rust source to a form where layout cannot matter: whitespace and
/// trailing commas outside string literals are dropped, line comments are
/// dropped, and string contents are kept verbatim.
fn canonical(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut in_string = false;
    while let Some(ch) = chars.next() {
        if in_string {
            out.push(ch);
            if ch == '\\' {
                if let Some(escaped) = chars.next() {
                    out.push(escaped);
                }
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => {
                in_string = true;
                out.push(ch);
            }
            '/' if chars.peek() == Some(&'/') => {
                for next in chars.by_ref() {
                    if next == '\n' {
                        break;
                    }
                }
            }
            _ if ch.is_whitespace() || ch == ',' => {}
            _ => out.push(ch),
        }
    }
    out
}

/// The module block the generator splices into `stripe.rs`.
fn emitted_modules_block(files: &[emit::OutputFile]) -> &str {
    files
        .iter()
        .find(|file| file.path == "modules.rs")
        .map(|file| file.contents.as_str())
        .expect("the generator emits the module block")
}

/// The same block as committed in `stripe.rs`.
fn committed_modules_block(root: &str) -> &str {
    let begin = root.find(emit::MODULES_BEGIN).expect("stripe.rs has the begin marker");
    let end = root.find(emit::MODULES_END).expect("stripe.rs has the end marker") + emit::MODULES_END.len();
    &root[begin..end]
}

/// Formats `paths` in place with the workspace's rustfmt configuration.
///
/// Returns `false` when `rustfmt` is unavailable, in which case the comparison
/// cannot be made (the generator itself treats rustfmt as best effort).
fn rustfmt(paths: &[PathBuf]) -> bool {
    if paths.is_empty() {
        return true;
    }
    let config = Path::new(env!("CARGO_MANIFEST_DIR")).join("../.rustfmt.toml");
    let mut command = Command::new("rustfmt");
    command.args(["--edition", "2024", "--quiet"]);
    if config.exists() {
        command.arg("--config-path").arg(&config);
    }
    command.args(paths);
    command.status().is_ok_and(|status| status.success())
}

#[test]
fn committed_generated_code_matches_the_specification() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_bytes = fs::read(manifest.join("spec/openapi.spec3.sdk.json")).expect("the vendored specification");
    let spec: serde_json::Value = serde_json::from_slice(&spec_bytes).expect("the specification is JSON");
    let model = Builder::new(spec).build().expect("the specification builds a model");
    let files = emit::emit(&model);

    // Write the emitted tree to a scratch directory and format it the way
    // `make stripe_codegen` would.
    let scratch = std::env::temp_dir().join(format!("stripe_codegen_freshness_{}", std::process::id()));
    let _ = fs::remove_dir_all(&scratch);
    let mut written = Vec::new();
    for file in &files {
        if file.path == "modules.rs" {
            continue;
        }
        let path = scratch.join(&file.path);
        fs::create_dir_all(path.parent().expect("parent directory")).expect("creating the scratch directory");
        fs::write(&path, &file.contents).expect("writing the emitted module");
        written.push(path);
    }
    if !rustfmt(&written) {
        eprintln!("skipping: `rustfmt` is not installed, so the emitted tree cannot be compared");
        return;
    }

    let mut stale = Vec::new();
    for file in &files {
        if file.path == "modules.rs" {
            continue;
        }
        let emitted = fs::read_to_string(scratch.join(&file.path)).expect("reading the emitted module");
        let committed = fs::read_to_string(manifest.join(&file.path))
            .unwrap_or_else(|error| panic!("reading {}: {error}", file.path));
        if canonical(&committed) != canonical(&emitted) {
            stale.push(file.path.clone());
        }
    }

    let root = fs::read_to_string(manifest.join(ROOT_FILE)).expect("the crate root");
    if canonical(committed_modules_block(&root)) != canonical(emitted_modules_block(&files)) {
        stale.push(ROOT_FILE.to_string());
    }

    let _ = fs::remove_dir_all(&scratch);

    assert!(
        stale.is_empty(),
        "these files do not match what `stripe_codegen` produces from the vendored specification; \
         run `make stripe_codegen`:\n  {}",
        stale.join("\n  ")
    );
}
