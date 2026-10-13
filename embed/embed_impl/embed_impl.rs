//! Implementation of the `Embed` derive macro for the `embed` crate.
//!
//! This crate is an implementation detail and should not be used directly.
#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
#[macro_use]
extern crate quote;
extern crate proc_macro;

use std::{
    collections::BTreeMap,
    env,
    iter::FromIterator,
    path::{Path, PathBuf},
};

use embed_utils::PathMatcher;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, MetaNameValue, parse_macro_input};

fn embedded(
    ident: &syn::Ident,
    relative_folder_path: Option<&str>,
    absolute_folder_path: &Path,
    prefix: Option<&str>,
    includes: &[String],
    excludes: &[String],
    metadata_only: bool,
    crate_path: &syn::Path,
) -> syn::Result<TokenStream2> {
    extern crate embed_utils;

    let mut match_values = BTreeMap::new();
    let mut list_values = Vec::<String>::new();

    let includes: Vec<&str> = includes.iter().map(AsRef::as_ref).collect();
    let excludes: Vec<&str> = excludes.iter().map(AsRef::as_ref).collect();
    let matcher = PathMatcher::new(&includes, &excludes);
    for embed_utils::FileEntry {
        rel_path,
        full_canonical_path,
    } in embed_utils::get_files(absolute_folder_path, &matcher)
    {
        match_values.insert(
            rel_path.clone(),
            embed_file(
                relative_folder_path,
                ident,
                &rel_path,
                &full_canonical_path,
                metadata_only,
                crate_path,
            )?,
        );

        list_values.push(if let Some(prefix) = prefix {
            format!("{}{}", prefix, rel_path)
        } else {
            rel_path
        });
    }

    let array_len = list_values.len();

    // If debug-embed is on, unconditionally include the code below. Otherwise,
    // make it conditional on cfg(not(debug_assertions)).
    let not_debug_attr = if cfg!(feature = "debug-embed") {
        quote! {}
    } else {
        quote! { #[cfg(not(debug_assertions))]}
    };

    let handle_prefix = if let Some(prefix) = prefix {
        quote! {
          let file_path = match file_path.strip_prefix(#prefix) {
            ::std::option::Option::Some(file_path) => file_path,
            ::std::option::Option::None => return ::std::option::Option::None,
          };
        }
    } else {
        TokenStream2::new()
    };
    let match_values = match_values.into_iter().map(|(path, bytes)| {
        quote! {
            (#path, #bytes),
        }
    });
    let value_type = if cfg!(feature = "compression") {
        quote! { fn() -> #crate_path::EmbeddedFile }
    } else {
        quote! { #crate_path::EmbeddedFile }
    };
    let get_value = if cfg!(feature = "compression") {
        quote! {|idx| (ENTRIES[idx].1)()}
    } else {
        quote! {|idx| ENTRIES[idx].1.clone()}
    };
    Ok(quote! {
        #not_debug_attr
        impl #ident {
            /// Get an embedded file and its metadata.
            pub fn get(file_path: &str) -> ::std::option::Option<#crate_path::EmbeddedFile> {
              #handle_prefix
              const ENTRIES: &'static [(&'static str, #value_type)] = &[
                  #(#match_values)*];
              let position = ENTRIES.binary_search_by_key(&file_path, |entry| entry.0);
              position.ok().map(#get_value)

            }

            fn names() -> ::std::slice::Iter<'static, &'static str> {
                const ITEMS: [&str; #array_len] = [#(#list_values),*];
                ITEMS.iter()
            }

            /// Iterates over the file paths in the folder.
            pub fn iter() -> impl ::std::iter::Iterator<Item = ::std::borrow::Cow<'static, str>> {
                Self::names().map(|x| ::std::borrow::Cow::from(*x))
            }
        }

        #not_debug_attr
        impl #crate_path::RustEmbed for #ident {
          fn get(file_path: &str) -> ::std::option::Option<#crate_path::EmbeddedFile> {
            #ident::get(file_path)
          }
          fn iter() -> #crate_path::Filenames {
            #crate_path::Filenames::Embedded(#ident::names())
          }
        }
    })
}

fn dynamic(
    ident: &syn::Ident,
    folder_paths: Vec<String>,
    prefix: Option<&str>,
    includes: &[String],
    excludes: &[String],
    metadata_only: bool,
    crate_path: &syn::Path,
) -> TokenStream2 {
    let (handle_prefix, map_iter) = if let ::std::option::Option::Some(prefix) = prefix {
        (
            quote! { let file_path = file_path.strip_prefix(#prefix)?; },
            quote! { ::std::borrow::Cow::Owned(format!("{}{}", #prefix, e.rel_path)) },
        )
    } else {
        (TokenStream2::new(), quote! { ::std::borrow::Cow::from(e.rel_path) })
    };

    let declare_includes = quote! {
      const INCLUDES: &[&str] = &[#(#includes),*];
    };

    let declare_excludes = quote! {
      const EXCLUDES: &[&str] = &[#(#excludes),*];
    };

    // In metadata_only mode, we still need to read file contents to generate the
    // file hash, but then we drop the file data.
    let strip_contents = metadata_only.then_some(quote! {
        .map(|mut file| { file.data = ::std::default::Default::default(); file })
    });

    // The assets folder is resolved at runtime from the given candidates, in
    // order. `folder_paths[0]` is the absolute path captured at build time, so
    // on a machine where the build directory is still present the previous
    // behaviour is unchanged. The remaining candidates are the path as written
    // in `#[folder]`, tried relative to the directory the binary is run from,
    // which keeps debug builds working when the build directory is unavailable
    // (for example inside a WASI sandbox).
    quote! {
        #[cfg(debug_assertions)]
        impl #ident {


          fn matcher() -> &'static #crate_path::utils::PathMatcher {
              #declare_includes
              #declare_excludes
              static PATH_MATCHER: ::std::sync::OnceLock<#crate_path::utils::PathMatcher> = ::std::sync::OnceLock::new();
              PATH_MATCHER.get_or_init(|| #crate_path::utils::PathMatcher::new(INCLUDES, EXCLUDES))
          }

          /// Resolves the assets folder at runtime, caching the result.
          ///
          /// Returns `None` if neither the build time path nor the path relative
          /// to the current directory exists.
          fn folder_root() -> ::std::option::Option<&'static ::std::path::Path> {
              static FOLDER_ROOT: ::std::sync::OnceLock<::std::option::Option<::std::path::PathBuf>> = ::std::sync::OnceLock::new();
              FOLDER_ROOT
                  .get_or_init(|| {
                      [#(::std::path::Path::new(#folder_paths)),*]
                          .into_iter()
                          .find_map(|candidate| candidate.canonicalize().ok())
                  })
                  .as_deref()
          }

            /// Get an embedded file and its metadata.
            ///
            /// Returns `None` when the path is not a valid relative path, when
            /// the folder cannot be resolved, when the file does not exist or
            /// when the requested path would leave the folder (for example
            /// through a symbolic link).
            pub fn get(file_path: &str) -> ::std::option::Option<#crate_path::EmbeddedFile> {
                #handle_prefix

                // Only accept paths that could have been returned by `iter`.
                // This keeps debug builds behaving exactly like release builds,
                // which look the path up in a static table of relative paths.
                if !#crate_path::utils::is_valid_key(file_path) {
                    return ::std::option::Option::None;
                }

                let rel_file_path = file_path;
                let folder_root = Self::folder_root()?;
                let file_path = folder_root.join(rel_file_path);

                // Make sure the path requested does not escape the folder path,
                // even through a symbolic link.
                let canonical_file_path = file_path.canonicalize().ok()?;
                if !canonical_file_path.starts_with(folder_root) {
                    return ::std::option::Option::None;
                }

                let path_matcher = Self::matcher();
                if path_matcher.is_path_included(rel_file_path) {
                  #crate_path::utils::read_file_from_fs(&canonical_file_path).ok() #strip_contents
                } else {
                  ::std::option::Option::None
                }
            }

            /// Iterates over the file paths in the folder.
            pub fn iter() -> impl ::std::iter::Iterator<Item = ::std::borrow::Cow<'static, str>> {
                Self::folder_root()
                    .into_iter()
                    .flat_map(|folder_root| {
                        #crate_path::utils::get_files(folder_root, Self::matcher())
                    })
                    .map(|e| #map_iter)
            }
        }

        #[cfg(debug_assertions)]
        impl #crate_path::RustEmbed for #ident {
          fn get(file_path: &str) -> ::std::option::Option<#crate_path::EmbeddedFile> {
            #ident::get(file_path)
          }
          fn iter() -> #crate_path::Filenames {
            // the return type of iter() is unnamable, so we have to box it
            #crate_path::Filenames::Dynamic(::std::boxed::Box::new(#ident::iter()))
          }
        }
    }
}

fn generate_assets(
    ident: &syn::Ident,
    relative_folder_path: Option<&str>,
    absolute_folder_path: String,
    prefix: Option<String>,
    includes: Vec<String>,
    excludes: Vec<String>,
    metadata_only: bool,
    crate_path: &syn::Path,
) -> syn::Result<TokenStream2> {
    let embedded_impl = embedded(
        ident,
        relative_folder_path,
        Path::new(&absolute_folder_path),
        prefix.as_deref(),
        &includes,
        &excludes,
        metadata_only,
        crate_path,
    );
    if cfg!(feature = "debug-embed") {
        return embedded_impl;
    }
    let embedded_impl = embedded_impl?;
    // Resolve the folder at runtime from the build time absolute path first,
    // then from the path as written in `#[folder]` relative to the directory
    // the binary is run from.
    let fallback_folder_path = relative_folder_path.unwrap_or(absolute_folder_path.as_str()).to_owned();
    let dynamic_impl = dynamic(
        ident,
        vec![absolute_folder_path, fallback_folder_path],
        prefix.as_deref(),
        &includes,
        &excludes,
        metadata_only,
        crate_path,
    );

    Ok(quote! {
        #embedded_impl
        #dynamic_impl
    })
}

fn embed_file(
    folder_path: Option<&str>,
    ident: &syn::Ident,
    rel_path: &str,
    full_canonical_path: &Path,
    metadata_only: bool,
    crate_path: &syn::Path,
) -> syn::Result<TokenStream2> {
    let full_canonical_path = full_canonical_path.to_str().ok_or_else(|| {
        syn::Error::new(
            ident.span(),
            format!("`{}` does not have a UTF-8 representation", full_canonical_path.display()),
        )
    })?;

    let file = embed_utils::read_file_from_fs(Path::new(full_canonical_path)).map_err(|error| {
        syn::Error::new(ident.span(), format!("could not read `{}`: {}", full_canonical_path, error))
    })?;
    let hash = file.metadata.hash();
    let last_modified = match file.metadata.last_modified() {
        Some(last_modified) => quote! { ::std::option::Option::Some(#last_modified) },
        None => quote! { ::std::option::Option::None },
    };
    let created = match file.metadata.created() {
        Some(created) => quote! { ::std::option::Option::Some(#created) },
        None => quote! { ::std::option::Option::None },
    };
    // The mimetype is always emitted. Whether it is kept is decided on the
    // target by `__embed_metadata!`, because this proc-macro is compiled for
    // the host and its features are resolved independently of the target.
    let mimetype = embed_utils::mimetype_of(Path::new(full_canonical_path));
    let mimetype = mimetype.as_str();

    let embedding_code = if metadata_only {
        quote! {
            const BYTES: &'static [u8] = &[];
        }
    } else if cfg!(feature = "compression") {
        let folder_path = folder_path.ok_or(syn::Error::new(
            ident.span(),
            "`folder` must be provided under `compression` feature.",
        ))?;
        // Print some debugging information
        let full_relative_path = PathBuf::from_iter([folder_path, rel_path]);
        let full_relative_path = full_relative_path.to_string_lossy();
        quote! {
          #crate_path::flate!(static BYTES: [u8] from #full_relative_path);
        }
    } else {
        quote! {
          const BYTES: &'static [u8] = include_bytes!(#full_canonical_path);
        }
    };
    let closure_args = if cfg!(feature = "compression") {
        quote! { || }
    } else {
        quote! {}
    };
    Ok(quote! {
         #closure_args {
          #embedding_code

          #crate_path::EmbeddedFile {
              data: ::std::borrow::Cow::Borrowed(&BYTES),
              metadata: #crate_path::utils::__embed_metadata!([#(#hash),*], #last_modified, #created, #mimetype)
          }
        }
    })
}

/// Find all pairs of the `name = "value"` attribute from the derive input
fn find_attribute_values(ast: &syn::DeriveInput, attr_name: &str) -> Vec<String> {
    ast.attrs
        .iter()
        .filter(|value| value.path().is_ident(attr_name))
        .filter_map(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue {
                value: Expr::Lit(ExprLit {
                    lit: Lit::Str(val), ..
                }),
                ..
            }) => Some(val.value()),
            _ => None,
        })
        .collect()
}

fn find_bool_attribute(ast: &syn::DeriveInput, attr_name: &str) -> Option<bool> {
    ast.attrs
        .iter()
        .find(|value| value.path().is_ident(attr_name))
        .and_then(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue {
                value: Expr::Lit(ExprLit {
                    lit: Lit::Bool(val), ..
                }),
                ..
            }) => Some(val.value()),
            _ => None,
        })
}

fn impl_embed(ast: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unit => {}
            _ => return Err(syn::Error::new_spanned(ast, "RustEmbed can only be derived for unit structs")),
        },
        _ => return Err(syn::Error::new_spanned(ast, "RustEmbed can only be derived for unit structs")),
    };

    let crate_path: syn::Path = match find_attribute_values(ast, "crate_path").last() {
        Some(value) => syn::parse_str(value)
            .map_err(|error| syn::Error::new_spanned(ast, format!("invalid `crate_path` value `{value}`: {error}")))?,
        None => syn::parse_str("embed").expect("`embed` is a valid path"),
    };

    let mut folder_paths = find_attribute_values(ast, "folder");
    if folder_paths.len() != 1 {
        return Err(syn::Error::new_spanned(
            ast,
            "#[derive(RustEmbed)] must contain one attribute like this #[folder = \"examples/public/\"]",
        ));
    }
    let folder_path = folder_paths.remove(0);

    let prefix = find_attribute_values(ast, "prefix").into_iter().next();
    let includes = find_attribute_values(ast, "include");
    let excludes = find_attribute_values(ast, "exclude");
    let metadata_only = find_bool_attribute(ast, "metadata_only").unwrap_or(false);

    #[cfg(not(feature = "include-exclude"))]
    if !includes.is_empty() || !excludes.is_empty() {
        return Err(syn::Error::new_spanned(
            ast,
            "Please turn on the `include-exclude` feature to use the `include` and `exclude` attributes",
        ));
    }

    #[cfg(feature = "interpolate-folder-path")]
    let folder_path = shellexpand::full(&folder_path)
        .map_err(|v| syn::Error::new_spanned(ast, v.to_string()))?
        .to_string();

    // Base relative paths on the Cargo.toml location
    let (relative_path, absolute_folder_path) = if Path::new(&folder_path).is_relative() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| syn::Error::new_spanned(ast, "the CARGO_MANIFEST_DIR environment variable is not set"))?;
        let absolute_path = Path::new(&manifest_dir).join(&folder_path);
        let absolute_path = absolute_path
            .to_str()
            .ok_or_else(|| syn::Error::new_spanned(ast, "the `folder` path is not valid UTF-8"))?
            .to_owned();
        (Some(folder_path.clone()), absolute_path)
    } else {
        if cfg!(feature = "compression") {
            return Err(syn::Error::new_spanned(
                ast,
                "`folder` must be a relative path under `compression` feature.",
            ));
        }
        (None, folder_path)
    };

    let absolute_path = Path::new(&absolute_folder_path);
    if !absolute_path.exists() {
        let cwd = env::current_dir()
            .ok()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| "<unknown>".to_owned());
        let mut message = format!(
            "#[derive(RustEmbed)] folder '{}' does not exist. cwd: '{}'",
            absolute_folder_path, cwd
        );

        // Add a message about the interpolate-folder-path feature if the path may
        // include a variable
        if absolute_folder_path.contains('$') && cfg!(not(feature = "interpolate-folder-path")) {
            message += "\nA variable has been detected. RustEmbed can expand variables \
                  when the `interpolate-folder-path` feature is enabled.";
        }

        return Err(syn::Error::new_spanned(ast, message));
    }
    if !absolute_path.is_dir() {
        return Err(syn::Error::new_spanned(
            ast,
            format!("#[derive(RustEmbed)] folder '{}' is not a directory", absolute_folder_path),
        ));
    }

    generate_assets(
        &ast.ident,
        relative_path.as_deref(),
        absolute_folder_path,
        prefix,
        includes,
        excludes,
        metadata_only,
        &crate_path,
    )
}

#[proc_macro_derive(RustEmbed, attributes(folder, prefix, include, exclude, metadata_only, crate_path))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match impl_embed(&ast) {
        Ok(ok) => ok.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn derive(input: &str) -> syn::Result<TokenStream2> {
        impl_embed(&syn::parse_str::<syn::DeriveInput>(input).expect("valid derive input"))
    }

    #[test]
    fn a_folder_that_is_a_directory_is_accepted() {
        // Resolved relative to this crate's manifest directory.
        assert!(derive(r#"#[folder = "."] struct Assets;"#).is_ok());
    }

    #[test]
    fn a_missing_folder_is_a_clean_error() {
        let error = derive(r#"#[folder = "definitely-not-here"] struct Assets;"#).unwrap_err();
        assert!(error.to_string().contains("does not exist"), "{}", error);
    }

    #[test]
    fn a_folder_that_is_a_file_is_a_clean_error() {
        let error = derive(r#"#[folder = "Cargo.toml"] struct Assets;"#).unwrap_err();
        assert!(error.to_string().contains("not a directory"), "{}", error);
    }

    #[test]
    fn the_folder_attribute_is_required_exactly_once() {
        for input in ["struct Assets;", r#"#[folder = "."] #[folder = "."] struct Assets;"#] {
            let error = derive(input).unwrap_err();
            assert!(error.to_string().contains("one attribute"), "{}", error);
        }
    }

    #[test]
    fn an_invalid_crate_path_is_a_clean_error() {
        let error = derive(r#"#[folder = "."] #[crate_path = "not a path!!"] struct Assets;"#).unwrap_err();
        assert!(error.to_string().contains("crate_path"), "{}", error);
    }

    #[test]
    fn only_unit_structs_are_supported() {
        let error = derive(r#"#[folder = "."] struct Assets { field: u8 }"#).unwrap_err();
        assert!(error.to_string().contains("unit structs"), "{}", error);
    }

    #[test]
    fn a_wrongly_typed_metadata_only_is_ignored() {
        // `metadata_only` without a boolean literal falls back to `false`.
        assert!(derive(r#"#[folder = "."] #[metadata_only = "yes"] struct Assets;"#).is_ok());
    }
}
