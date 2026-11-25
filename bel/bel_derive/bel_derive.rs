//! Derive macro for the `bel` crate.
//!
//! `#[derive(bel::IntoValue)]` generates a `From<Type> for bel::Value`
//! implementation, which in turn makes the type usable with
//! `Context::add_variable` through the blanket
//! `TryIntoValue` implementation.
//!
//! # Representation
//!
//! - A struct with named fields becomes a CEL map keyed by field name.
//! - A tuple struct with one field becomes that field's CEL value.
//! - A tuple struct with several fields becomes a CEL list.
//! - A unit struct becomes `null`.
//! - A unit enum variant becomes a CEL string with the variant name.
//! - Any other enum variant becomes a single-entry map
//!   `{"Variant": payload}`.
//!
//! # Attributes
//!
//! Container attributes:
//!
//! - `#[bel(rename_all = "camelCase")]` — rename all fields or variants.
//!   Supported rules: `lowercase`, `UPPERCASE`, `snake_case`, `kebab-case`,
//!   `camelCase`, `PascalCase`, `SCREAMING_SNAKE_CASE`.
//! - `#[bel(transparent)]` — encode the single field directly.
//!
//! Field and variant attributes:
//!
//! - `#[bel(rename = "name")]` — override the encoded name.
//! - `#[bel(skip)]` — omit a named struct field.
//! - `#[bel(with = "path")]` — call `path(field)` and convert the result into
//!   `bel::Value` instead of using `From` on the field type.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, LitStr, Path, Type, parse_macro_input};

/// Derive `From<Type> for bel::Value`.
#[proc_macro_derive(IntoValue, attributes(bel))]
pub fn derive_into_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[derive(Default)]
struct Container {
    rename_all: Option<RenameRule>,
    transparent: bool,
}

#[derive(Default)]
struct Field {
    rename: Option<String>,
    skip: bool,
    with: Option<Path>,
}

#[derive(Clone, Copy)]
enum RenameRule {
    Lower,
    Upper,
    Snake,
    Kebab,
    Camel,
    Pascal,
    ScreamingSnake,
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let container = parse_container(&input.attrs)?;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) => encode_struct(data, &container)?,
        Data::Enum(data) => encode_enum(data, &container, ident)?,
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(ident, "bel::IntoValue cannot be derived for unions"));
        }
    };

    Ok(quote! {
        impl #impl_generics ::core::convert::From<#ident #ty_generics> for ::bel::Value #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                #body
            }
        }
    })
}

fn encode_struct(data: &DataStruct, container: &Container) -> syn::Result<TokenStream2> {
    if container.transparent {
        let field = single_field(&data.fields)?;
        let attrs = parse_field(&field.attrs)?;
        let ty = &field.ty;
        let member = field_member(field);
        return Ok(encode_value(&attrs, ty, quote! { value.#member }));
    }

    match &data.fields {
        Fields::Named(fields) => {
            let mut inserts = Vec::new();
            for field in &fields.named {
                let attrs = parse_field(&field.attrs)?;
                if attrs.skip {
                    continue;
                }
                let name = renamed(&attrs, field.ident.as_ref().expect("named field"), container);
                let ty = &field.ty;
                let member = field_member(field);
                let value = encode_value(&attrs, ty, quote! { value.#member });
                inserts.push(quote! {
                    map.insert(#name, #value);
                });
            }
            Ok(quote! {
                let mut map: ::std::collections::HashMap<&'static str, ::bel::Value> =
                    ::std::collections::HashMap::new();
                #(#inserts)*
                ::bel::Value::from(map)
            })
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                let field = fields.unnamed.first().expect("one field");
                let attrs = parse_field(&field.attrs)?;
                let ty = &field.ty;
                let member = field_member(field);
                return Ok(encode_value(&attrs, ty, quote! { value.#member }));
            }
            let mut values = Vec::new();
            for (index, field) in fields.unnamed.iter().enumerate() {
                let attrs = parse_field(&field.attrs)?;
                let ty = &field.ty;
                let member = syn::Member::Unnamed(index.into());
                let value = encode_value(&attrs, ty, quote! { value.#member });
                values.push(value);
            }
            Ok(quote! {
                ::bel::Value::from(::std::vec![#(#values),*])
            })
        }
        Fields::Unit => Ok(quote! { ::bel::Value::Null }),
    }
}

fn encode_enum(data: &DataEnum, container: &Container, ident: &Ident) -> syn::Result<TokenStream2> {
    let mut arms = Vec::new();
    for variant in &data.variants {
        let attrs = parse_field(&variant.attrs)?;
        let encoded_name = renamed(&attrs, &variant.ident, container);
        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Unit => {
                arms.push(quote! {
                    #ident::#variant_ident => ::bel::Value::from(#encoded_name)
                });
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field = fields.unnamed.first().expect("one field");
                let field_attrs = parse_field(&field.attrs)?;
                let value = encode_value(&field_attrs, &field.ty, quote! { inner });
                arms.push(quote! {
                    #ident::#variant_ident(inner) => {
                        let mut map: ::std::collections::HashMap<&'static str, ::bel::Value> =
                            ::std::collections::HashMap::new();
                        map.insert(#encoded_name, #value);
                        ::bel::Value::from(map)
                    }
                });
            }
            Fields::Unnamed(fields) => {
                let bindings = (0..fields.unnamed.len())
                    .map(|index| quote::format_ident!("inner_{index}"))
                    .collect::<Vec<_>>();
                let values = fields
                    .unnamed
                    .iter()
                    .zip(&bindings)
                    .map(|(field, binding)| {
                        let attrs = parse_field(&field.attrs)?;
                        Ok(encode_value(&attrs, &field.ty, quote! { #binding }))
                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                arms.push(quote! {
                    #ident::#variant_ident(#(#bindings),*) => {
                        let mut map: ::std::collections::HashMap<&'static str, ::bel::Value> =
                            ::std::collections::HashMap::new();
                        map.insert(#encoded_name, ::bel::Value::from(::std::vec![#(#values),*]));
                        ::bel::Value::from(map)
                    }
                });
            }
            Fields::Named(fields) => {
                let bindings = fields
                    .named
                    .iter()
                    .map(|field| field.ident.clone().expect("named variant field"))
                    .collect::<Vec<_>>();
                let mut inserts = Vec::new();
                for field in &fields.named {
                    let field_attrs = parse_field(&field.attrs)?;
                    if field_attrs.skip {
                        continue;
                    }
                    let binding = field.ident.as_ref().expect("named field");
                    let field_name = renamed(&field_attrs, binding, container);
                    let value = encode_value(&field_attrs, &field.ty, quote! { #binding });
                    inserts.push(quote! {
                        inner_map.insert(#field_name, #value);
                    });
                }
                arms.push(quote! {
                    #ident::#variant_ident { #(#bindings),* } => {
                        let mut inner_map: ::std::collections::HashMap<&'static str, ::bel::Value> =
                            ::std::collections::HashMap::new();
                        #(#inserts)*
                        let mut map: ::std::collections::HashMap<&'static str, ::bel::Value> =
                            ::std::collections::HashMap::new();
                        map.insert(#encoded_name, ::bel::Value::from(inner_map));
                        ::bel::Value::from(map)
                    }
                });
            }
        }
    }

    Ok(quote! {
        match value {
            #(#arms),*
        }
    })
}

fn single_field(fields: &Fields) -> syn::Result<&syn::Field> {
    match fields {
        Fields::Named(fields) if fields.named.len() == 1 => Ok(fields.named.first().expect("one field")),
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(fields.unnamed.first().expect("one field")),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[bel(transparent)] requires a struct with exactly one field",
        )),
    }
}

fn field_member(field: &syn::Field) -> syn::Member {
    match &field.ident {
        Some(ident) => syn::Member::Named(ident.clone()),
        None => syn::Member::Unnamed(0.into()),
    }
}

fn encode_value(attrs: &Field, _ty: &Type, value: TokenStream2) -> TokenStream2 {
    match &attrs.with {
        Some(path) => quote! { ::bel::Value::from(#path(#value)) },
        None => quote! { ::bel::Value::from(#value) },
    }
}

fn renamed(attrs: &Field, ident: &Ident, container: &Container) -> String {
    if let Some(rename) = &attrs.rename {
        return rename.clone();
    }
    let raw = ident.to_string();
    match container.rename_all {
        Some(rule) => apply_rule(rule, &raw),
        None => raw,
    }
}

fn parse_container(attrs: &[Attribute]) -> syn::Result<Container> {
    let mut container = Container::default();
    for attr in attrs {
        if !attr.path().is_ident("bel") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("transparent") {
                container.transparent = true;
                return Ok(());
            }
            if meta.path.is_ident("rename_all") {
                let value: LitStr = meta.value()?.parse()?;
                container.rename_all = Some(parse_rule(&value)?);
                return Ok(());
            }
            Err(meta.error("unknown bel container attribute"))
        })?;
    }
    Ok(container)
}

fn parse_field(attrs: &[Attribute]) -> syn::Result<Field> {
    let mut field = Field::default();
    for attr in attrs {
        if !attr.path().is_ident("bel") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                field.skip = true;
                return Ok(());
            }
            if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                field.rename = Some(value.value());
                return Ok(());
            }
            if meta.path.is_ident("with") {
                let value: LitStr = meta.value()?.parse()?;
                field.with = Some(syn::parse_str(&value.value())?);
                return Ok(());
            }
            Err(meta.error("unknown bel field attribute"))
        })?;
    }
    Ok(field)
}

fn parse_rule(value: &LitStr) -> syn::Result<RenameRule> {
    match value.value().as_str() {
        "lowercase" => Ok(RenameRule::Lower),
        "UPPERCASE" => Ok(RenameRule::Upper),
        "snake_case" => Ok(RenameRule::Snake),
        "kebab-case" => Ok(RenameRule::Kebab),
        "camelCase" => Ok(RenameRule::Camel),
        "PascalCase" => Ok(RenameRule::Pascal),
        "SCREAMING_SNAKE_CASE" => Ok(RenameRule::ScreamingSnake),
        other => Err(syn::Error::new(value.span(), format!("unknown rename_all rule `{other}`"))),
    }
}

fn apply_rule(rule: RenameRule, name: &str) -> String {
    let words = words(name);
    match rule {
        RenameRule::Lower => name.to_lowercase(),
        RenameRule::Upper => name.to_uppercase(),
        RenameRule::Snake => words.join("_").to_lowercase(),
        RenameRule::Kebab => words.join("-").to_lowercase(),
        RenameRule::ScreamingSnake => words.join("_").to_uppercase(),
        RenameRule::Camel => {
            let mut words = words.into_iter();
            let first = words.next().unwrap_or_default().to_lowercase();
            let rest = words.map(capitalize).collect::<String>();
            first + &rest
        }
        RenameRule::Pascal => words.into_iter().map(capitalize).collect(),
    }
}

fn words(name: &str) -> Vec<&str> {
    let mut words = Vec::new();
    let mut start = 0;
    let bytes = name.as_bytes();
    for index in 1..bytes.len() {
        let previous = bytes[index - 1];
        let current = bytes[index];
        if current == b'_' || current == b'-' {
            if start < index {
                words.push(&name[start..index]);
            }
            start = index + 1;
        } else if current.is_ascii_uppercase() && previous.is_ascii_lowercase() {
            words.push(&name[start..index]);
            start = index;
        }
    }
    if start < name.len() {
        words.push(&name[start..]);
    }
    if words.is_empty() {
        words.push(name);
    }
    words
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}
