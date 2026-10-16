//! Reachability gating must never leave a dangling reference.
//!
//! Every response type carries `#[cfg(any(feature = "...", ...))]` naming the
//! modules whose generated methods can reach it (`stripe_codegen` computes the
//! set). The property that makes the scheme sound is monotonicity: if an item is
//! compiled whenever one of its features is enabled, then every type it
//! references must be compiled at least as readily.
//!
//! This test re-derives that property from the generated source instead of
//! trusting the generator: for every gated item that references
//! `crate::module::Type`, the referenced type's own gate must be implied by the
//! referencing item's gate (or the type must be ungated).
//!
//! It runs against the *source*, so it catches a generator change that stops
//! propagating reachability even before the failing build does.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

/// A generated item: the features that gate it plus the text it spans.
struct Item {
    /// Features on the item's `#[cfg]`. Empty means "always compiled".
    gate: BTreeSet<String>,
    /// Whether the item carried a `#[cfg]` at all.
    gated: bool,
    /// The item's source text.
    text: String,
}

fn generated_files() -> Vec<PathBuf> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("api");
    let mut files: Vec<PathBuf> = fs::read_dir(&directory)
        .expect("the generated api directory")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("rs"))
        .collect();
    files.sort();
    files
}

/// The features named by a `#[cfg(...)]` attribute, from its accumulated lines.
fn features_of(attribute: &str) -> BTreeSet<String> {
    let mut features = BTreeSet::new();
    let mut rest = attribute;
    while let Some(start) = rest.find("feature = \"") {
        let after = &rest[start + "feature = \"".len()..];
        let Some(end) = after.find('"') else {
            break;
        };
        features.insert(after[..end].to_string());
        rest = &after[end..];
    }
    features
}

/// Splits a generated module into items, tracking the `#[cfg]` before each one.
fn parse_items(source: &str) -> Vec<Item> {
    let lines: Vec<&str> = source.lines().collect();
    let mut items = Vec::new();
    let mut pending: Option<String> = None;
    let mut index = 0usize;

    while index < lines.len() {
        let line = lines[index].trim_start();

        // A `#[cfg]` attribute may be wrapped over several lines.
        if line.starts_with("#[cfg(") {
            let mut attribute = String::from(line);
            while !line_ends_attribute(&attribute) && index + 1 < lines.len() {
                index += 1;
                attribute.push_str(lines[index].trim_start());
            }
            if pending.is_none() {
                pending = Some(attribute);
            }
            index += 1;
            continue;
        }

        let starts_item = line.starts_with("pub struct ")
            || line.starts_with("pub enum ")
            || line.starts_with("pub type ")
            || line.starts_with("impl ")
            || line.starts_with("impl<");
        if !starts_item {
            index += 1;
            continue;
        }

        // Collect the item: brace-balanced, or up to the terminating `;` for an
        // alias that rustfmt wrapped.
        let mut text = String::new();
        let mut depth = 0i32;
        let mut opened = false;
        loop {
            let current = lines[index];
            text.push_str(current);
            text.push('\n');
            depth += current.matches('{').count() as i32 - current.matches('}').count() as i32;
            if current.contains('{') {
                opened = true;
            }
            index += 1;
            if (opened && depth <= 0) || (!opened && current.trim_end().ends_with(';')) || index >= lines.len() {
                break;
            }
        }

        let (gate, gated) = match pending.take() {
            Some(attribute) => (features_of(&attribute), true),
            None => (BTreeSet::new(), false),
        };
        items.push(Item {
            gate,
            gated,
            text,
        });
    }
    items
}

fn line_ends_attribute(attribute: &str) -> bool {
    attribute.trim_end().ends_with(")]")
}

/// `(module, name)` for every type the modules define.
fn defined_types(files: &[PathBuf]) -> BTreeMap<(String, String), (BTreeSet<String>, bool)> {
    let mut defined = BTreeMap::new();
    for file in files {
        let module = file
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("module name")
            .to_string();
        let source = fs::read_to_string(file).expect("generated module is UTF-8");
        for item in parse_items(&source) {
            let name = item
                .text
                .strip_prefix("pub struct ")
                .or_else(|| item.text.strip_prefix("pub enum "))
                .or_else(|| item.text.strip_prefix("pub type "))
                .map(|rest| {
                    rest.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
                        .next()
                        .unwrap_or("")
                })
                .filter(|name| !name.is_empty());
            if let Some(name) = name {
                defined.insert((module.clone(), name.to_string()), (item.gate, item.gated));
            }
        }
    }
    defined
}

/// Every `crate::module::Type` referenced by the item.
fn referenced_types(text: &str) -> BTreeSet<(String, String)> {
    let mut references = BTreeSet::new();
    let mut rest = text;
    while let Some(start) = rest.find("crate::") {
        let after = &rest[start + "crate::".len()..];
        rest = after;
        let Some((module, tail)) = after.split_once("::") else {
            continue;
        };
        if !module
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
        {
            continue;
        }
        let name: String = tail
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
            .collect();
        if !name.is_empty() && name.chars().next().is_some_and(char::is_uppercase) {
            references.insert((module.to_string(), name));
        }
    }
    references
}

#[test]
fn every_referenced_type_is_compiled_by_its_referrer_gate() {
    let files = generated_files();
    let defined = defined_types(&files);
    let mut problems = Vec::new();

    for file in &files {
        let source = fs::read_to_string(file).expect("generated module is UTF-8");
        for item in parse_items(&source) {
            if !item.gated {
                continue;
            }
            for (module, name) in referenced_types(&item.text) {
                let Some((target_gate, target_gated)) = defined.get(&(module.clone(), name.clone())) else {
                    // Hand-written types (`crate::params::FormBuilder`, `crate::List`)
                    // and the gated-out `events` module are covered elsewhere.
                    continue;
                };
                // An ungated (always compiled) target is always available. A
                // gated one must be enabled whenever its referrer is.
                if !target_gated || item.gate.is_subset(target_gate) {
                    continue;
                }
                let missing: Vec<&String> = item.gate.difference(target_gate).collect();
                problems.push(format!(
                    "{}: an item gated on {:?} references `crate::{}::{}`, which is only compiled with {:?} \
                     (missing {:?})",
                    file.display(),
                    item.gate,
                    module,
                    name,
                    target_gate,
                    missing
                ));
            }
        }
    }

    assert_eq!(
        problems.len(),
        0,
        "reachability gating is unsound:\n{}",
        problems.iter().take(10).cloned().collect::<Vec<_>>().join("\n")
    );
}
