use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeMap as Map, BTreeSet as Set};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

#[proc_macro]
pub fn test_emitter(_input: TokenStream) -> TokenStream {
    test("libyaml-emitter", |dir| !dir.join("error").exists())
}

#[proc_macro]
pub fn test_parser(_input: TokenStream) -> TokenStream {
    test("libyaml-parser", |dir| !dir.join("error").exists())
}

#[proc_macro]
pub fn test_parser_error(_input: TokenStream) -> TokenStream {
    test("libyaml-parser-error", |dir| dir.join("error").exists())
}

fn test(ignorelist: &str, check: fn(&Path) -> bool) -> TokenStream {
    let tests_dir = Path::new("tests");

    let mut ignored_ids = Set::new();
    let ignorelist = tests_dir.join("ignorelist").join(ignorelist);
    for line in BufReader::new(File::open(ignorelist).unwrap()).lines() {
        let mut line = line.unwrap();
        line.truncate(4);
        ignored_ids.insert(line);
    }

    let mut ids = Map::new();
    let yaml_test_suite = tests_dir.join("data").join("yaml-test-suite");
    for entry in fs::read_dir(yaml_test_suite).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }

        let path = entry.path();
        let description = path.join("===");
        let slug = if let Ok(description) = fs::read_to_string(description) {
            description_to_slug(description)
        } else {
            continue;
        };

        if !check(&path) {
            continue;
        }

        let file_name = entry.file_name();
        let id = file_name.to_str().unwrap().to_owned();
        ids.insert(id, slug);
    }

    let mut tests = proc_macro2::TokenStream::new();
    let ignore = quote!(#[ignore]);
    for (id, slug) in ids {
        let test_name = format_ident!("_{id}_{slug}");
        let ignore = ignored_ids.contains(&id).then_some(&ignore);

        tests.extend(quote! {
            #[test]
            #ignore
            #[allow(non_snake_case)]
            fn #test_name() {
                test(#id);
            }
        });
    }

    TokenStream::from(tests)
}

fn description_to_slug(mut description: String) -> String {
    description = description.replace(|ch: char| !ch.is_ascii_alphanumeric(), "_");
    while description.contains("__") {
        description = description.replace("__", "_");
    }
    description.trim_matches('_').to_ascii_lowercase()
}
