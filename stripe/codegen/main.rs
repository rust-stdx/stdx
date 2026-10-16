//! Generates the `stripe` crate's types and methods from Stripe's official
//! OpenAPI specification.
//!
//! Usage:
//!
//! ```text
//! stripe_codegen [--spec <path>] [--out <dir>] [--fetch]
//! ```
//!
//! With no arguments the vendored specification at `spec/openapi.spec3.sdk.json`
//! is used and the crate's `api/` directory (plus the module block in
//! `stripe.rs`) is rewritten. `--fetch` downloads the latest specification from
//! `stripe/openapi` instead.
//!
//! The pipeline itself lives in the `stripe_codegen` library, which the
//! `stripe` crate's tests also use to check the committed generated code.

use std::{path::PathBuf, process::ExitCode};

use stripe_codegen::{DEFAULT_OUT, DEFAULT_SPEC, generate};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("stripe_codegen: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut spec_path = PathBuf::from(DEFAULT_SPEC);
    let mut out_dir = PathBuf::from(DEFAULT_OUT);
    let mut fetch = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--spec" => {
                spec_path = PathBuf::from(args.next().ok_or("--spec needs a path")?);
            }
            "--out" => {
                out_dir = PathBuf::from(args.next().ok_or("--out needs a path")?);
            }
            "--fetch" => fetch = true,
            other => return Err(format!("unknown argument `{other}`")),
        }
    }

    let report = generate(&spec_path, &out_dir, fetch)?;

    let largest: Vec<String> = report
        .by_module
        .iter()
        .take(12)
        .map(|(module, count)| format!("{module}={count}"))
        .collect();

    println!(
        "stripe_codegen: wrote {} files (spec {}, {} types ({} pruned), {} methods, {} modules)",
        report.files,
        report.spec_version,
        report.types,
        report.pruned,
        report.methods,
        report.features.len()
    );
    println!("stripe_codegen: largest type modules: {}", largest.join(" "));
    println!("stripe_codegen: features = {}", report.features.join(" "));
    Ok(())
}
