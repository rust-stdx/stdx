//! Budgets for the generated API surface.
//!
//! `stripe/api` is generated, so its size is a build-cost decision as much as a
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
/// `core` is the default feature set, so dropping one of these would silently
/// remove checkout, subscriptions, metered billing, the customer portal, or
/// payments from a build that asks for nothing in particular.
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
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("api");
    let mut files: Vec<PathBuf> = fs::read_dir(&directory)
        .expect("the generated api directory")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("rs"))
        .collect();
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
    for file in generated_files() {
        let module = file.file_stem().and_then(|stem| stem.to_str()).expect("module name");
        assert!(
            root.contains(&format!("api/{module}.rs")),
            "`{module}` is generated but not declared in stripe.rs"
        );
    }
}

#[test]
fn the_core_umbrella_covers_the_documented_essentials() {
    // `core` is the default feature set and the crate documentation lists what
    // it covers; this keeps the two in step.
    let manifest = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")).expect("Cargo.toml");
    let start = manifest.find("core = [").expect("a `core` feature");
    let end = start + manifest[start..].find(']').expect("the end of the `core` list");
    let listed: Vec<&str> = manifest[start..end]
        .lines()
        .filter_map(|line| line.split('"').nth(1))
        .collect();

    for module in CORE_ESSENTIALS {
        assert!(
            listed.contains(module),
            "`core` no longer includes `{module}`; the default build would lose it"
        );
    }
    assert!(
        listed.len() >= CORE_ESSENTIALS.len(),
        "`core` lists {} modules, fewer than the {} documented essentials",
        listed.len(),
        CORE_ESSENTIALS.len()
    );
}
