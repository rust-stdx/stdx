//! Budgets for the generated API surface.
//!
//! The generated type and method trees are large, so their size is a build-cost decision as much as a
//! code-size one: every generated type, `impl` block and doc line is work rustc
//! does on every `cargo check` and every test build. The budgets below are the
//! measured values with a small margin, so a change that balloons the surface
//! (or a generator change that stops pruning) fails here instead of silently
//! making everyone's builds slower.
//!
//! Reachability gating (`#[cfg(any(feature = ...))]` on every response type)
//! adds attribute lines without adding surface, so those are counted separately:
//! `MAX_LINES` covers the lines that are not part of a `#[cfg]` attribute, and
//! `MAX_GATE_FEATURES` bounds the gating text itself.
//!
//! When the specification genuinely grows, raise the numbers deliberately in the
//! same change that explains why.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Types (structs, enums, aliases) across every generated module.
const MAX_ITEMS: usize = 9_800;
/// Lines of generated Rust outside `#[cfg]` attributes.
const MAX_LINES: usize = 270_000;
/// `impl` blocks, including the nested visitors of `Deserialize` impls.
const MAX_IMPLS: usize = 10_400;
/// Bytes of `#[doc]` attribute text.
const MAX_DOC_BYTES: usize = 1_400_000;
/// Feature references across every `#[cfg]` attribute. One per (item, module
/// that can reach it), so it grows with the API surface and with gating.
const MAX_GATE_FEATURES: usize = 140_000;

/// The modules the crate documentation promises the `core` umbrella covers.
///
/// `core` is the umbrella the documentation advertises, so dropping one of
/// these would silently remove checkout, subscriptions, metered billing, the
/// customer portal, or payments from a build that asks for it.
const CORE_ESSENTIALS: &[&str] = &[
    "billing",        // meters, meter events: metered usage
    "billing_portal", // the customer portal
    "charges",
    "checkout", // checkout sessions
    "common",
    "confirmation_tokens",
    "coupons",
    "credit_notes",
    "customers",
    "disputes",
    "invoiceitems",
    "invoices",
    "mandates",
    "payment_intents",
    "payment_methods",
    "prices", // including usage-based prices
    "products",
    "promotion_codes",
    "refunds",
    "setup_intents",
    "subscription_items",
    "subscription_schedules",
    "subscriptions",
    "tax_codes",
    "tax_ids",
    "tax_rates",
    "webhook_endpoints",
];

fn generated_files() -> Vec<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    let api = manifest.join("api");
    for entry in fs::read_dir(&api).expect("the generated api directory") {
        let path = entry.expect("directory entry").path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("rs")
            && path.file_name().and_then(|name| name.to_str()) != Some("lib.rs")
        {
            files.push(path);
        }
    }
    for shard in fs::read_dir(manifest).expect("the generated crate directory") {
        let shard = shard.expect("directory entry").path();
        let Some(name) = shard.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !shard.is_dir() || !name.starts_with("types_") {
            continue;
        }
        for entry in fs::read_dir(&shard).expect("the shard directory") {
            let path = entry.expect("directory entry").path();
            if path.extension().and_then(|extension| extension.to_str()) == Some("rs")
                && path.file_name().and_then(|name| name.to_str()) != Some("lib.rs")
            {
                files.push(path);
            }
        }
    }
    files.sort();
    assert!(files.len() > 50, "expected the generated modules, found {}", files.len());
    files
}

/// Counts `{items, structure lines, impls, doc bytes, gate feature references}`.
///
/// A `#[cfg]` attribute may span many lines (rustfmt wraps long `any(...)`
/// lists), so its continuation lines are attributed to the attribute rather
/// than to the structure.
fn measure() -> (usize, usize, usize, usize, usize) {
    let (mut items, mut lines, mut impls, mut docs, mut gate_features) = (0, 0, 0, 0, 0);
    for file in generated_files() {
        let contents = fs::read_to_string(&file).expect("generated module is UTF-8");
        let mut in_attribute = false;
        for line in contents.lines() {
            let trimmed = line.trim_start();
            if in_attribute {
                gate_features += trimmed.matches("feature = ").count();
                if trimmed.ends_with(")]") || trimmed.ends_with("]\n") || trimmed.ends_with(")]\n") || trimmed == "]" {
                    in_attribute = false;
                }
                continue;
            }
            if trimmed.starts_with("#[cfg(") {
                gate_features += trimmed.matches("feature = ").count();
                if !trimmed.ends_with(")]") {
                    in_attribute = true;
                }
                continue;
            }
            lines += 1;
            if trimmed.starts_with("pub struct ")
                || trimmed.starts_with("pub enum ")
                || trimmed.starts_with("pub type ")
            {
                items += 1;
            } else if trimmed.starts_with("impl ") || trimmed.starts_with("impl<") {
                impls += 1;
            } else if trimmed.starts_with("#[doc =") {
                docs += trimmed.len();
            }
        }
    }
    (items, lines, impls, docs, gate_features)
}

#[test]
fn generated_surface_stays_within_budget() {
    let (items, lines, impls, docs, gate_features) = measure();
    assert!(items <= MAX_ITEMS, "{items} generated types exceed the budget of {MAX_ITEMS}");
    assert!(
        lines <= MAX_LINES,
        "{lines} generated lines (outside `#[cfg]` attributes) exceed the budget of {MAX_LINES}"
    );
    assert!(impls <= MAX_IMPLS, "{impls} impl blocks exceed the budget of {MAX_IMPLS}");
    assert!(
        docs <= MAX_DOC_BYTES,
        "{docs} bytes of generated docs exceed the budget of {MAX_DOC_BYTES}"
    );
    assert!(
        gate_features <= MAX_GATE_FEATURES,
        "{gate_features} gating feature references exceed the budget of {MAX_GATE_FEATURES}"
    );
}

#[test]
fn every_generated_module_is_referenced_by_the_crate_root() {
    // A module file that no `mod` declaration points at would never be compiled
    // (and never noticed).
    let root = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("stripe.rs")).expect("crate root");
    for module in generated_files()
        .iter()
        .filter_map(|file| file.file_stem().and_then(|stem| stem.to_str()))
    {
        assert!(
            root.contains(&format!("pub mod {module}")),
            "`{module}` is generated but not declared in stripe.rs"
        );
    }
}

/// Parses the `core = [...]` feature list out of the manifest.
///
/// Splits on commas instead of lines so the list may be laid out on one line or
/// one entry per line, with or without `#` comments.
fn core_feature_list(manifest: &str) -> Vec<String> {
    let start = manifest.find("core = [").expect("a `core` feature");
    let end = start + manifest[start..].find(']').expect("the end of the `core` list");
    let block = &manifest[start + "core = [".len()..end];
    // Drop `#` comments line by line so a comment on its own line cannot hide
    // the entry that follows it on the next line.
    let block = block
        .lines()
        .map(|line| line.split('#').next().unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n");
    block
        .split(',')
        .filter_map(|entry| {
            let start = entry.find('"')?;
            let rest = &entry[start + 1..];
            let end = rest.find('"')?;
            Some(rest[..end].to_string())
        })
        .collect()
}

#[test]
fn the_core_umbrella_covers_the_documented_essentials() {
    // `core` is the feature set the crate documentation promises and lists; this
    // keeps the two in step.
    let manifest = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")).expect("Cargo.toml");
    let listed = core_feature_list(&manifest);

    for module in CORE_ESSENTIALS {
        assert!(
            listed.iter().any(|feature| feature.as_str() == *module),
            "`core` no longer includes `{module}`; builds that ask for it would lose it"
        );
    }
    assert!(
        listed.len() >= CORE_ESSENTIALS.len(),
        "`core` lists {} modules, fewer than the {} documented essentials",
        listed.len(),
        CORE_ESSENTIALS.len()
    );
}

#[test]
fn the_core_list_parser_accepts_any_layout() {
    let one_line = "[features]\ncore = [\"billing\", \"checkout\"]\n";
    assert_eq!(core_feature_list(one_line), ["billing", "checkout"]);

    let multi_line = "[features]\ncore = [\n    # selling\n    \"billing\",\n    \"checkout\",\n]\n";
    assert_eq!(core_feature_list(multi_line), ["billing", "checkout"]);
}
