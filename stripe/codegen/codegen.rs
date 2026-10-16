//! Generates the `stripe` crate's types and methods from Stripe's official
//! OpenAPI specification.
//!
//! [`generate`] is the whole pipeline: build the model from the vendored
//! specification, emit one file per module, splice the module block into
//! `stripe.rs`, and check that the crate's cargo features and `SPEC_VERSION`
//! still line up. The `stripe_codegen` binary is a thin wrapper around it, and
//! `stripe`'s own tests use [`schema::Builder`] and [`emit::emit`] to check that
//! the committed generated code matches the specification.

pub mod emit;
pub mod ir;
pub mod naming;
pub mod schema;

use std::{collections::BTreeSet, fs, path::Path};

use serde_json::Value;

use crate::schema::Builder;

/// The specification vendored in this package.
pub const DEFAULT_SPEC: &str = "spec/openapi.spec3.sdk.json";
/// Where generated code is written by default.
pub const DEFAULT_OUT: &str = ".";
/// Where `--fetch` downloads the specification from.
pub const LATEST_URL: &str = "https://raw.githubusercontent.com/stripe/openapi/master/latest/openapi.spec3.sdk.json";
/// The crate root whose generated module block is rewritten.
pub const ROOT_FILE: &str = "stripe.rs";

/// What a generation run produced, for the report the binary prints.
#[derive(Debug)]
pub struct Report {
    /// Number of files written, including the module block.
    pub files: usize,
    /// The API version the specification declares.
    pub spec_version: String,
    /// Number of generated types.
    pub types: usize,
    /// Number of schemas dropped because nothing can reach them.
    pub pruned: usize,
    /// Number of generated methods.
    pub methods: usize,
    /// Every module, each of which is also a cargo feature.
    pub features: Vec<String>,
    /// Types per module, largest first.
    pub by_module: Vec<(String, usize)>,
}

/// Regenerates the crate from `spec_path` into `out_dir`.
///
/// `out_dir` must hold the crate's `stripe.rs`, whose generated module block is
/// rewritten in place, and its `Cargo.toml`, which is checked but never edited.
/// Every generated file is formatted with the workspace's `rustfmt`
/// configuration when `rustfmt` is available.
///
/// # Errors
///
/// Returns a description of the first problem encountered: an unreadable or
/// malformed specification, a naming conflict the generator has no rule for, a
/// module without a matching cargo feature, or a crate root that does not
/// mention the specification version.
pub fn generate(spec_path: &Path, out_dir: &Path, fetch: bool) -> Result<Report, String> {
    if fetch {
        fetch_spec(spec_path)?;
    }

    let bytes = fs::read(spec_path).map_err(|error| format!("reading {spec_path:?}: {error}"))?;
    let spec: Value = serde_json::from_slice(&bytes).map_err(|error| format!("parsing {spec_path:?}: {error}"))?;

    let model = Builder::new(spec).build()?;
    let files = emit::emit(&model);

    let mut written: BTreeSet<String> = BTreeSet::new();
    for file in &files {
        if file.path == "modules.rs" {
            rewrite_modules_block(out_dir, &file.contents)?;
            continue;
        }
        let path = out_dir.join(&file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| format!("creating {parent:?}: {error}"))?;
        }
        fs::write(&path, &file.contents).map_err(|error| format!("writing {path:?}: {error}"))?;
        format_file(&path);
        written.insert(file.path.clone());
    }

    prune_stale(out_dir, &written)?;
    check_features(out_dir, &model.modules)?;
    check_spec_version(out_dir, &model.spec_version)?;

    let mut by_module: Vec<(String, usize)> = model
        .modules
        .iter()
        .map(|module| {
            let count = model.types.iter().filter(|ty| &ty.module == module).count();
            (module.clone(), count)
        })
        .collect();
    by_module.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    Ok(Report {
        files: files.len(),
        spec_version: model.spec_version,
        types: model.types.len(),
        pruned: model.pruned,
        methods: model.methods.len(),
        features: model.modules.iter().cloned().collect(),
        by_module,
    })
}

/// Replaces the generated module block in `<out>/stripe.rs`.
fn rewrite_modules_block(out_dir: &Path, block: &str) -> Result<(), String> {
    let root = out_dir.join(ROOT_FILE);
    let existing = fs::read_to_string(&root).map_err(|error| format!("reading {root:?}: {error}"))?;
    let begin = existing
        .find(emit::MODULES_BEGIN)
        .ok_or_else(|| format!("{root:?} has no `{}` marker", emit::MODULES_BEGIN))?;
    let end = existing
        .find(emit::MODULES_END)
        .ok_or_else(|| format!("{root:?} has no `{}` marker", emit::MODULES_END))?;
    let end = end + emit::MODULES_END.len();
    let mut updated = String::with_capacity(existing.len() + block.len());
    updated.push_str(&existing[..begin]);
    updated.push_str(block.trim_end_matches('\n'));
    updated.push_str(&existing[end..]);
    fs::write(&root, updated).map_err(|error| format!("writing {root:?}: {error}"))
}

/// Removes generated files whose module no longer exists in the spec.
fn prune_stale(out_dir: &Path, written: &BTreeSet<String>) -> Result<(), String> {
    let dir = out_dir.join(emit::GENERATED_DIR);
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(format!("reading {dir:?}: {error}")),
    };
    for entry in entries {
        let entry = entry.map_err(|error| format!("reading {dir:?}: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("");
        let relative = format!("{}/{name}", emit::GENERATED_DIR);
        if !written.contains(&relative) {
            fs::remove_file(&path).map_err(|error| format!("removing {path:?}: {error}"))?;
        }
    }
    Ok(())
}

/// Fails when a generated module has no matching cargo feature.
///
/// Every module gates its request builders and methods behind a feature of the
/// same name; without a feature the module would be generated but unusable.
fn check_features(out_dir: &Path, modules: &BTreeSet<String>) -> Result<(), String> {
    let manifest = out_dir.join("Cargo.toml");
    let text = fs::read_to_string(&manifest).map_err(|error| format!("reading {manifest:?}: {error}"))?;
    let features_section = text
        .split("[features]")
        .nth(1)
        .and_then(|rest| rest.split("\n[").next())
        .ok_or_else(|| format!("{manifest:?} has no [features] section"))?;
    let declared: BTreeSet<&str> = features_section
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(name, _)| name.trim())
        .collect();
    let missing: Vec<&String> = modules
        .iter()
        .filter(|module| !declared.contains(module.as_str()))
        .collect();
    if missing.is_empty() {
        return Ok(());
    }
    let missing: Vec<&str> = missing.into_iter().map(String::as_str).collect();
    Err(format!(
        "{manifest:?} is missing cargo features for: {}\nadd `{} = []` entries (or add them to the `core` umbrella)",
        missing.join(", "),
        missing[0],
    ))
}

/// Fails when the crate root does not declare the specification version the
/// generated code was built from.
fn check_spec_version(out_dir: &Path, spec_version: &str) -> Result<(), String> {
    let root = out_dir.join(ROOT_FILE);
    let text = fs::read_to_string(&root).map_err(|error| format!("reading {root:?}: {error}"))?;
    if text.contains(spec_version) {
        return Ok(());
    }
    Err(format!(
        "{root:?} does not mention the specification version `{spec_version}`; \
         update the `SPEC_VERSION` constant"
    ))
}

/// Formats a generated file in place with `rustfmt`, if it is installed.
///
/// The workspace's `.rustfmt.toml` is passed explicitly so the output is
/// identical no matter which directory the generator writes to. Best effort: a
/// missing or failing `rustfmt` leaves the file as generated, which still
/// compiles.
fn format_file(path: &Path) {
    let config = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.rustfmt.toml");
    let mut command = std::process::Command::new("rustfmt");
    command.args(["--edition", "2024", "--quiet"]);
    if config.exists() {
        command.arg("--config-path").arg(&config);
    }
    let _ = command.arg(path).status();
}

fn fetch_spec(destination: &Path) -> Result<(), String> {
    let response = fetch_bytes(LATEST_URL)?;
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("creating {parent:?}: {error}"))?;
    }
    fs::write(destination, &response).map_err(|error| format!("writing {destination:?}: {error}"))
}

/// Minimal HTTPS fetch using the system `curl`, avoiding an HTTP dependency in
/// the generator (which is a development tool, not part of the shipped crate).
fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    let output = std::process::Command::new("curl")
        .args(["--fail", "--silent", "--show-error", "--location", url])
        .output()
        .map_err(|error| format!("running curl: {error}"))?;
    if !output.status.success() {
        return Err(format!("curl failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(output.stdout)
}
