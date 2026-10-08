//! Generates the media type -> extensions reverse lookup.
//!
//! The forward extension -> media type table lives in `media_types.rs`. The reverse lookup is
//! derived from it at build time and emitted as a sorted static slice, so that it can be searched
//! with a binary search at runtime without a build dependency.

#[cfg(feature = "reverse-lookup")]
use std::{
    collections::BTreeMap,
    io::{BufWriter, Write},
};
use std::{env, fs::File, path::Path};

#[cfg(feature = "reverse-lookup")]
#[path = "media_types.rs"]
mod media_types;

#[cfg(feature = "reverse-lookup")]
use media_types::MEDIA_TYPES;

fn main() {
    println!("cargo:rerun-if-changed=media_types.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is set by Cargo");
    let destination = Path::new(&out_dir).join("reverse_lookup_generated.rs");
    println!("cargo:rustc-env=REVERSE_LOOKUP_PATH={}", destination.display());

    #[cfg(feature = "reverse-lookup")]
    {
        let file = File::create(&destination).expect("create generated reverse lookup");
        let mut out = BufWriter::new(file);
        write_reverse_mappings(&mut out);
    }

    // Keep the file around so the crate can always `include!` it if the gating ever changes.
    #[cfg(not(feature = "reverse-lookup"))]
    {
        File::create(&destination).expect("create generated reverse lookup");
    }
}

/// Write the sorted media type -> extensions reverse lookup.
#[cfg(feature = "reverse-lookup")]
fn write_reverse_mappings<W: Write>(out: &mut W) {
    let mappings = reverse_mappings();

    write!(out, "static REV_MAPPINGS: &[(&str, TopLevelExts)] = &[").unwrap();

    let mut extensions: Vec<&str> = Vec::new();

    for (toplevel, sublevels) in mappings {
        let toplevel_start = extensions.len();

        let mut sublevel_map = String::new();
        for (sublevel, sublevel_extensions) in sublevels {
            let sublevel_start = extensions.len();
            extensions.extend(sublevel_extensions);
            let sublevel_end = extensions.len();
            sublevel_map.push_str(&format!("(\"{}\", ({}, {})),", sublevel, sublevel_start, sublevel_end));
        }

        let toplevel_end = extensions.len();

        write!(
            out,
            "(\"{}\", TopLevelExts {{ start: {}, end: {}, subs: &[{}] }}),",
            toplevel, toplevel_start, toplevel_end, sublevel_map
        )
        .unwrap();
    }

    writeln!(out, "];").unwrap();
    writeln!(out, "const EXTS: &[&str] = &{:?};", extensions).unwrap();
}

/// Build the media type -> extensions mapping from the forward table.
#[cfg(feature = "reverse-lookup")]
fn reverse_mappings() -> BTreeMap<&'static str, BTreeMap<&'static str, Vec<&'static str>>> {
    let mut mappings: BTreeMap<&'static str, BTreeMap<&'static str, Vec<&'static str>>> = BTreeMap::new();

    for &(extension, media_types) in MEDIA_TYPES {
        for media_type in media_types {
            let (toplevel, sublevel) = split_media_type(media_type);
            mappings
                .entry(toplevel)
                .or_default()
                .entry(sublevel)
                .or_default()
                .push(extension);
        }
    }

    mappings
}

/// Split a media type into its top-level type and sub-level.
#[cfg(feature = "reverse-lookup")]
fn split_media_type(media_type: &str) -> (&str, &str) {
    let slash = media_type.find('/').expect("static media types always contain a slash");
    (&media_type[..slash], &media_type[slash + 1..])
}
